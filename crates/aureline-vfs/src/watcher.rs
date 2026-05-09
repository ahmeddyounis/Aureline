//! Watcher source and watcher-health state machine.
//!
//! Watcher behaviour is normalised through the VFS; no surface MAY
//! wire its own filesystem watcher (ADR 0006 § Watcher-source and
//! watcher-health contract). The prototype tracks the source and
//! health for every attached root and emits
//! [`WatcherHealthFrame`] records whenever health transitions.
//!
//! Save and external-change honesty do NOT depend on watcher
//! perfection — `compare_before_write` is the correctness floor
//! (see [`crate::save`]). The watcher is a latency optimisation,
//! not a correctness guarantee.

use std::collections::BTreeMap;

/// Frozen watcher-source vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatcherSource {
    OsNativeWatcher,
    RemoteAgentWatcherStream,
    ScalableWatcherIntegration,
    PollingFallback,
}

impl WatcherSource {
    /// Returns the frozen snake-case string for this watcher source.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OsNativeWatcher => "os_native_watcher",
            Self::RemoteAgentWatcherStream => "remote_agent_watcher_stream",
            Self::ScalableWatcherIntegration => "scalable_watcher_integration",
            Self::PollingFallback => "polling_fallback",
        }
    }
}

/// Frozen watcher-health taxonomy. Every surface that renders
/// watcher-derived truth reads `WatcherHealth` and surfaces the
/// degraded states visibly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatcherHealth {
    Healthy,
    Warming,
    Degraded,
    FallbackPolling,
    Unavailable,
}

impl WatcherHealth {
    /// Returns the frozen snake-case string for this watcher health state.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Warming => "warming",
            Self::Degraded => "degraded",
            Self::FallbackPolling => "fallback_polling",
            Self::Unavailable => "unavailable",
        }
    }

    /// Health transition toward less healthy emits a freshness
    /// downgrade on the VFS subscription frame (ADR 0005).
    pub fn is_degraded(self) -> bool {
        !matches!(self, Self::Healthy | Self::Warming)
    }
}

/// Frozen watcher-health-transition frame. Pairs with the
/// ADR-0005 `subscription_freshness_downgrade` frame (with
/// `stale_reason = watcher_dropped`) when the transition is
/// toward a less healthy state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatcherHealthFrame {
    pub root_id: String,
    pub watcher_source: WatcherSource,
    pub watcher_health: WatcherHealth,
    pub reason_code: Option<String>,
    pub observed_at: String,
}

/// Per-root watcher registration state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatcherRegistration {
    pub root_id: String,
    pub source: WatcherSource,
    pub health: WatcherHealth,
    /// Cumulative count of primary/fallback watcher events fired
    /// against the root since attach.
    pub events_fired: u64,
}

/// Registry of watcher registrations for every attached root.
/// Emits [`WatcherHealthFrame`] records whenever health transitions
/// on a registered root.
#[derive(Debug, Clone, Default)]
pub struct WatcherRegistry {
    registrations: BTreeMap<String, WatcherRegistration>,
    frames: Vec<WatcherHealthFrame>,
}

impl WatcherRegistry {
    /// Creates an empty watcher registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a watcher for `root_id`. Records a `warming`
    /// frame so downstream surfaces can label the initial attach.
    pub fn register(
        &mut self,
        root_id: String,
        source: WatcherSource,
        observed_at: String,
        reason_code: Option<String>,
    ) {
        let registration = WatcherRegistration {
            root_id: root_id.clone(),
            source,
            health: WatcherHealth::Warming,
            events_fired: 0,
        };
        self.registrations.insert(root_id.clone(), registration);
        self.frames.push(WatcherHealthFrame {
            root_id,
            watcher_source: source,
            watcher_health: WatcherHealth::Warming,
            reason_code,
            observed_at,
        });
    }

    /// Transition the watcher health for `root_id`. Emits a frame
    /// only when the health actually changes. Returns `true` if a
    /// frame was emitted.
    pub fn transition(
        &mut self,
        root_id: &str,
        to: WatcherHealth,
        observed_at: String,
        reason_code: Option<String>,
    ) -> bool {
        let Some(reg) = self.registrations.get_mut(root_id) else {
            return false;
        };
        if reg.health == to {
            return false;
        }
        reg.health = to;
        let source = reg.source;
        self.frames.push(WatcherHealthFrame {
            root_id: root_id.to_owned(),
            watcher_source: source,
            watcher_health: to,
            reason_code,
            observed_at,
        });
        true
    }

    /// Rebind the watcher `source` for `root_id` and update its `health`.
    ///
    /// This emits a new [`WatcherHealthFrame`] when either the watcher source or
    /// watcher health changes. Consumers should treat the latest frame as the
    /// current watcher truth for the root.
    pub fn rebind(
        &mut self,
        root_id: &str,
        source: WatcherSource,
        health: WatcherHealth,
        observed_at: String,
        reason_code: Option<String>,
    ) -> bool {
        let Some(reg) = self.registrations.get_mut(root_id) else {
            return false;
        };
        if reg.source == source && reg.health == health {
            return false;
        }
        reg.source = source;
        reg.health = health;
        self.frames.push(WatcherHealthFrame {
            root_id: root_id.to_owned(),
            watcher_source: source,
            watcher_health: health,
            reason_code,
            observed_at,
        });
        true
    }

    /// Record a watcher event against `root_id`. Returns the new
    /// cumulative event count, or `None` if the root is not
    /// registered.
    pub fn record_event(&mut self, root_id: &str) -> Option<u64> {
        let reg = self.registrations.get_mut(root_id)?;
        reg.events_fired += 1;
        Some(reg.events_fired)
    }

    /// Returns the current watcher registration for `root_id`, if any.
    pub fn registration(&self, root_id: &str) -> Option<&WatcherRegistration> {
        self.registrations.get(root_id)
    }

    /// Returns all watcher-health frames emitted so far.
    pub fn frames(&self) -> &[WatcherHealthFrame] {
        &self.frames
    }

    /// Iterates over all registered roots.
    pub fn registrations(&self) -> impl Iterator<Item = (&String, &WatcherRegistration)> {
        self.registrations.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_emits_one_frame_per_change() {
        let mut reg = WatcherRegistry::new();
        reg.register(
            "root-1".to_owned(),
            WatcherSource::OsNativeWatcher,
            "mono:0".to_owned(),
            None,
        );
        assert!(reg.transition("root-1", WatcherHealth::Healthy, "mono:1".to_owned(), None,));
        // Same health — no frame.
        assert!(!reg.transition("root-1", WatcherHealth::Healthy, "mono:2".to_owned(), None,));
        assert!(reg.transition(
            "root-1",
            WatcherHealth::Degraded,
            "mono:3".to_owned(),
            Some("os_native_buffer_overflow".to_owned()),
        ));
        assert_eq!(reg.frames().len(), 3);
        assert_eq!(reg.frames()[0].watcher_health, WatcherHealth::Warming);
        assert_eq!(reg.frames()[1].watcher_health, WatcherHealth::Healthy);
        assert_eq!(reg.frames()[2].watcher_health, WatcherHealth::Degraded);
    }

    #[test]
    fn unknown_root_transition_is_a_noop() {
        let mut reg = WatcherRegistry::new();
        assert!(!reg.transition("ghost", WatcherHealth::Degraded, "mono:0".to_owned(), None,));
        assert!(reg.record_event("ghost").is_none());
    }

    #[test]
    fn rebind_updates_source_and_health_together() {
        let mut reg = WatcherRegistry::new();
        reg.register(
            "root-1".to_owned(),
            WatcherSource::OsNativeWatcher,
            "mono:0".to_owned(),
            None,
        );

        assert!(reg.rebind(
            "root-1",
            WatcherSource::PollingFallback,
            WatcherHealth::FallbackPolling,
            "mono:1".to_owned(),
            Some("polling_fallback_active".to_owned()),
        ));
        let frame = reg.frames().last().expect("frame");
        assert_eq!(frame.watcher_source, WatcherSource::PollingFallback);
        assert_eq!(frame.watcher_health, WatcherHealth::FallbackPolling);
    }
}
