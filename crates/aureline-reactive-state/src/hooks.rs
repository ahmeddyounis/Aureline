//! Protected-hot-path and observability-only hook counters for
//! the reactive subscription fabric.
//!
//! Hook ids match the names frozen in ADR 0005 § Protected-hot-path
//! hooks. The prototype counts; a production build replaces the
//! struct with a telemetry seam behind the same names so the
//! benchmark lab, the support-export lane, and the eventual
//! timeline / replay lane never have to translate vocabulary.
//!
//! Counts only (no wall-clock latencies) so the emitted
//! invalidation-trace records stay byte-stable across hosts. The
//! benchmark lab layers timing on top of these counters when it
//! scores against the protected-hot-path budgets the ADR freezes.

/// Per-store hook-fire counters. Every counter is incremented at
/// the point the ADR names; the harness asserts expected counts
/// per scenario so drift is caught before artifacts move.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HookCounters {
    /// Protected hot path: a subscribe call is accepted and a
    /// subscription_id is allocated.
    pub subscription_subscribe: u64,
    /// Protected hot path: a producer writes a snapshot frame.
    pub subscription_snapshot_emit: u64,
    /// Protected hot path: a producer writes a delta frame.
    pub subscription_delta_emit: u64,
    /// Protected hot path: a consumer applies a decoded delta to
    /// its local projection.
    pub subscription_delta_apply: u64,
    /// Protected hot path: a producer writes a resync_required
    /// frame.
    pub subscription_resync_required_emit: u64,
    /// Observability only: a consumer observes a freshness
    /// transition toward less-authoritative.
    pub subscription_freshness_downgrade: u64,
    /// Observability only: a consumer observes a completeness
    /// transition.
    pub subscription_completeness_changed: u64,
    /// Protected hot path: a producer coalesces deltas to respect
    /// the consumer's declared backpressure_mode.
    pub subscription_backpressure_coalesce: u64,
    /// Protected hot path: a consumer switches to
    /// backpressure_mode = snapshot_required after falling behind.
    pub subscription_snapshot_required_switch: u64,
    /// Protected hot path: a producer writes a terminal frame.
    pub subscription_terminate: u64,
    /// Observability only: an imported-history frame is attached
    /// to an active projection.
    pub subscription_imported_attach: u64,
    /// Observability only: a replay session starts emitting
    /// replayed frames.
    pub subscription_replay_begin: u64,
    /// Observability only: a replay session ends and live frames
    /// resume.
    pub subscription_replay_end: u64,
}

impl HookCounters {
    /// Ordered `(hook_id, protected_hot_path, count)` rows.
    /// Deterministic iteration order so harness JSON is
    /// byte-stable across hosts.
    pub fn entries(&self) -> [(&'static str, bool, u64); 13] {
        [
            ("subscription_subscribe", true, self.subscription_subscribe),
            (
                "subscription_snapshot_emit",
                true,
                self.subscription_snapshot_emit,
            ),
            (
                "subscription_delta_emit",
                true,
                self.subscription_delta_emit,
            ),
            (
                "subscription_delta_apply",
                true,
                self.subscription_delta_apply,
            ),
            (
                "subscription_resync_required_emit",
                true,
                self.subscription_resync_required_emit,
            ),
            (
                "subscription_freshness_downgrade",
                false,
                self.subscription_freshness_downgrade,
            ),
            (
                "subscription_completeness_changed",
                false,
                self.subscription_completeness_changed,
            ),
            (
                "subscription_backpressure_coalesce",
                true,
                self.subscription_backpressure_coalesce,
            ),
            (
                "subscription_snapshot_required_switch",
                true,
                self.subscription_snapshot_required_switch,
            ),
            ("subscription_terminate", true, self.subscription_terminate),
            (
                "subscription_imported_attach",
                false,
                self.subscription_imported_attach,
            ),
            (
                "subscription_replay_begin",
                false,
                self.subscription_replay_begin,
            ),
            (
                "subscription_replay_end",
                false,
                self.subscription_replay_end,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_are_complete_and_stable() {
        let entries = HookCounters::default().entries();
        assert_eq!(entries.len(), 13);
        let mut labels: Vec<&'static str> = entries.iter().map(|(id, _, _)| *id).collect();
        labels.sort();
        labels.dedup();
        assert_eq!(labels.len(), 13, "hook ids must be unique");
    }

    #[test]
    fn every_protected_hook_is_named() {
        let entries = HookCounters::default().entries();
        let protected: Vec<&'static str> = entries
            .iter()
            .filter(|(_, protected, _)| *protected)
            .map(|(id, _, _)| *id)
            .collect();
        // Eight protected hot-path hooks per ADR 0005 § Protected-
        // hot-path hooks. Drifting this list reopens ADR 0005.
        assert_eq!(protected.len(), 8);
        assert!(protected.contains(&"subscription_subscribe"));
        assert!(protected.contains(&"subscription_snapshot_emit"));
        assert!(protected.contains(&"subscription_delta_emit"));
        assert!(protected.contains(&"subscription_delta_apply"));
        assert!(protected.contains(&"subscription_resync_required_emit"));
        assert!(protected.contains(&"subscription_backpressure_coalesce"));
        assert!(protected.contains(&"subscription_snapshot_required_switch"));
        assert!(protected.contains(&"subscription_terminate"));
    }
}
