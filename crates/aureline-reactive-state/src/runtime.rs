//! Live reactive-state runtime: shared subscription envelope and
//! readiness-label projection wired to real shell consumers.
//!
//! The crate's [`store::ReactiveStore`](crate::store::ReactiveStore)
//! is the contract enforcer for the ADR 0005 envelope vocabulary.
//! The [`LiveReactiveStore`] in this module is the small runtime
//! adapter the live shell uses to:
//!
//! 1. publish workspace, derived, and execution-state frames into
//!    one canonical store rather than letting each shell surface
//!    cache its own truth, and
//! 2. project the underlying envelope (freshness × completeness ×
//!    derivation × invalidation) into a single, frozen
//!    [`ReadinessLabel`] vocabulary that surfaces render verbatim.
//!
//! The readiness vocabulary tracks the canonical
//! `semantic_readiness.state` set frozen in
//! `docs/filesystem/semantic_readiness_projection.md`:
//! `exact`, `imported`, `heuristic`, `stale`, `partial`,
//! `unavailable`, `out_of_scope`. Surfaces MUST NOT collapse two
//! labels into one and MUST NOT invent a synonym.
//!
//! # Ownership boundary
//!
//! The runtime adapter is intentionally thin. It owns:
//!
//! - the latest projection per `(query_family, scope_ref)` pair,
//!   so late subscribers see the same value the early ones did;
//! - a list of observer callbacks per pair, so two shell surfaces
//!   that subscribe to the same projection cannot drift; and
//! - the canonical [`ReadinessLabel`] derivation rules.
//!
//! It does NOT own:
//!
//! - workspace lifecycle (lives in `aureline-workspace`),
//! - watcher health (lives in `aureline-vfs`), or
//! - any rendering decisions (live in `aureline-shell`).
//!
//! Producers and consumers pass primitive vocabulary across the
//! boundary so this crate has no upstream dependency on the
//! workspace or shell crates.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::envelope::{
    AuthorityClass, BackpressureMode, Completeness, DerivationClass, FrameClass, Freshness,
    Invalidation, ProducerRef, ScopeClass, ScopeRef, StaleReason, SubscriptionEnvelope, ViewClass,
};
use crate::store::{Producer, ReactiveStore, SamplePayload, StoreError};

/// Frozen readiness-label vocabulary. Mirrors the
/// `semantic_readiness.state` token set used across shell, search,
/// diagnostics, migration, and support/export surfaces. New tokens
/// require a new ADR row; renaming an existing token is forbidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadinessLabel {
    /// Authoritative claim for the producer scope.
    Exact,
    /// Carried in from an external snapshot. Never authoritative.
    Imported,
    /// Best-effort derivation; capped confidence.
    Heuristic,
    /// Inputs changed since last run; refresh/rebuild required.
    Stale,
    /// Coverage is incomplete for the requested scope.
    Partial,
    /// Producer cannot serve claims right now.
    Unavailable,
    /// Subject is outside the current scope (not a failure).
    OutOfScope,
}

impl ReadinessLabel {
    /// Returns the stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// Returns the human-readable label shown in shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Ready",
            Self::Imported => "Imported",
            Self::Heuristic => "Heuristic",
            Self::Stale => "Stale",
            Self::Partial => "Partial",
            Self::Unavailable => "Unavailable",
            Self::OutOfScope => "Out of scope",
        }
    }

    /// True when surfaces SHOULD reach for the readiness inspector
    /// (i.e. anything other than `exact`).
    pub const fn warrants_inspector(self) -> bool {
        !matches!(self, Self::Exact)
    }
}

/// Projection of one subscription frame in the readiness
/// vocabulary. Shell surfaces consume this record; they MUST NOT
/// rebuild a private projection from raw freshness booleans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessProjection {
    pub query_family: String,
    pub scope_ref: ScopeRef,
    pub subscription_id: u64,
    pub snapshot_epoch: u64,
    pub delta_seq: u64,
    pub frame_class: FrameClass,
    pub freshness: Freshness,
    pub completeness: Completeness,
    pub readiness_label: ReadinessLabel,
    /// Optional canonical not-ready reason. When `readiness_label
    /// != Exact`, surfaces MUST surface either this token or a
    /// safe-next-action; they MUST NOT compress to a generic
    /// "loading" state.
    pub not_ready_reason: Option<StaleReason>,
    pub producer_id: String,
    pub producer_version: Option<String>,
    pub observed_at: String,
}

impl ReadinessProjection {
    /// Build a projection from a [`SubscriptionEnvelope`] using the
    /// canonical mapping rules. Falls back to the producer's first
    /// `producer_refs` row for attribution.
    pub fn from_envelope(envelope: &SubscriptionEnvelope, observed_at: impl Into<String>) -> Self {
        let label = readiness_from_envelope(envelope);
        let not_ready_reason = envelope
            .invalidation
            .as_ref()
            .map(|inv: &Invalidation| inv.stale_reason);
        let attribution = envelope.producer_refs.first();
        Self {
            query_family: envelope.query_family.clone(),
            scope_ref: envelope.scope_ref.clone(),
            subscription_id: envelope.subscription_id,
            snapshot_epoch: envelope.snapshot_epoch,
            delta_seq: envelope.delta_seq,
            frame_class: envelope.frame_class,
            freshness: envelope.freshness,
            completeness: envelope.completeness,
            readiness_label: label,
            not_ready_reason,
            producer_id: attribution
                .map(|r| r.producer_id.clone())
                .unwrap_or_else(|| "unknown".to_owned()),
            producer_version: attribution.and_then(|r| r.producer_version.clone()),
            observed_at: observed_at.into(),
        }
    }
}

/// Canonical mapping from the envelope vocabulary to the
/// readiness label vocabulary. The mapping is total and does not
/// invent synonyms; any future authority/derivation pair MUST
/// resolve to one of the seven labels above.
pub fn readiness_from_envelope(envelope: &SubscriptionEnvelope) -> ReadinessLabel {
    // Terminal frames always project as Unavailable; the stream
    // is closed.
    if matches!(envelope.frame_class, FrameClass::Terminal) {
        return ReadinessLabel::Unavailable;
    }

    // Completeness = unavailable always wins; the producer is
    // stating it cannot serve the requested scope.
    if matches!(envelope.completeness, Completeness::Unavailable) {
        return ReadinessLabel::Unavailable;
    }

    // Freshness drives the rest of the projection.
    match envelope.freshness {
        Freshness::Imported => ReadinessLabel::Imported,
        // Replayed bundles are not authoritative for the live
        // surface; treat them as imported for projection
        // purposes (they are a separate snapshot lineage).
        Freshness::Replayed => ReadinessLabel::Imported,
        Freshness::Stale => ReadinessLabel::Stale,
        Freshness::Cached => match envelope.completeness {
            Completeness::Full => ReadinessLabel::Heuristic,
            _ => ReadinessLabel::Stale,
        },
        Freshness::Warming => match envelope.completeness {
            Completeness::Full => ReadinessLabel::Heuristic,
            _ => ReadinessLabel::Partial,
        },
        Freshness::Authoritative => {
            // Authoritative + derivation matters: a derived
            // producer can never claim authoritative freshness
            // (the store rejects it), so we only need to handle
            // authoritative-class producers here. Completeness
            // decides Exact vs Partial.
            let _ = envelope.derivation_class;
            match envelope.completeness {
                Completeness::Full => ReadinessLabel::Exact,
                Completeness::Partial => ReadinessLabel::Partial,
                Completeness::Unloaded => ReadinessLabel::Partial,
                Completeness::Unavailable => ReadinessLabel::Unavailable,
            }
        }
    }
}

/// Observer callback invoked when the projection for a
/// `(query_family, scope_ref)` pair changes.
pub type ReadinessObserver = Rc<dyn Fn(&ReadinessProjection)>;

/// Internal registration record for a live observer.
struct ObserverRegistration {
    id: u64,
    observer: ReadinessObserver,
}

/// Per-subscription state held by the live runtime.
struct LiveSubscription {
    query_family: String,
    scope_ref: ScopeRef,
    subscription_id: u64,
    latest: Option<ReadinessProjection>,
    observers: Vec<ObserverRegistration>,
}

/// Handle returned by [`LiveReactiveStore::subscribe`]. Keeping
/// the handle alive is not required to receive callbacks; dropping
/// it does NOT auto-unsubscribe (use [`LiveReactiveStore::unsubscribe`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LiveSubscriptionToken {
    pub subscription_id: u64,
    pub observer_id: u64,
}

/// Live reactive store. Wraps the contract-enforcing
/// [`ReactiveStore`] and adds observer fan-out + readiness
/// projection so multiple shell surfaces can subscribe without
/// inventing their own caches.
pub struct LiveReactiveStore {
    inner: RefCell<ReactiveStore>,
    subscriptions: RefCell<Vec<LiveSubscription>>,
    next_observer_id: RefCell<u64>,
}

impl Default for LiveReactiveStore {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for LiveReactiveStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiveReactiveStore")
            .field("subscription_count", &self.subscriptions.borrow().len())
            .finish()
    }
}

impl LiveReactiveStore {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(ReactiveStore::new()),
            subscriptions: RefCell::new(Vec::new()),
            next_observer_id: RefCell::new(1),
        }
    }

    /// Register a producer + open a single shared subscription on
    /// it. Subsequent observers attach to the same subscription
    /// rather than allocating their own caches. Returns the
    /// subscription id allocated by the underlying
    /// [`ReactiveStore`].
    pub fn open_shared_subscription(
        &self,
        producer: Producer,
        backpressure_mode: BackpressureMode,
    ) -> Result<u64, StoreError> {
        let query_family = producer.query_family.clone();
        let scope_ref = producer.scope_ref.clone();
        {
            let mut store = self.inner.borrow_mut();
            store.register_producer(producer);
        }
        let subscription_id = self
            .inner
            .borrow_mut()
            .subscribe(&query_family, &scope_ref, backpressure_mode)?;
        self.subscriptions.borrow_mut().push(LiveSubscription {
            query_family,
            scope_ref,
            subscription_id,
            latest: None,
            observers: Vec::new(),
        });
        Ok(subscription_id)
    }

    /// Subscribe an observer to a previously opened shared
    /// subscription. If a projection has already been published,
    /// the observer is invoked synchronously with the latest value
    /// so late subscribers do not see an empty cache.
    pub fn subscribe(
        &self,
        subscription_id: u64,
        observer: ReadinessObserver,
    ) -> Result<LiveSubscriptionToken, StoreError> {
        let observer_id = {
            let mut next = self.next_observer_id.borrow_mut();
            let id = *next;
            *next += 1;
            id
        };
        let mut subs = self.subscriptions.borrow_mut();
        let sub = subs
            .iter_mut()
            .find(|s| s.subscription_id == subscription_id)
            .ok_or(StoreError::SubscriptionNotFound)?;
        if let Some(latest) = &sub.latest {
            observer(latest);
        }
        sub.observers.push(ObserverRegistration {
            id: observer_id,
            observer,
        });
        Ok(LiveSubscriptionToken {
            subscription_id,
            observer_id,
        })
    }

    /// Remove a previously-registered observer.
    pub fn unsubscribe(&self, token: LiveSubscriptionToken) -> Result<(), StoreError> {
        let mut subs = self.subscriptions.borrow_mut();
        let sub = subs
            .iter_mut()
            .find(|s| s.subscription_id == token.subscription_id)
            .ok_or(StoreError::SubscriptionNotFound)?;
        sub.observers.retain(|obs| obs.id != token.observer_id);
        Ok(())
    }

    /// Publish a frame on the shared subscription. Routes the
    /// envelope through the contract-enforcing store (so derived ≠
    /// authoritative, delta-before-snapshot, and the other
    /// invariants stay live), projects it into a
    /// [`ReadinessProjection`], caches it, and fans out to every
    /// observer registered for the subscription.
    pub fn publish_snapshot(
        &self,
        subscription_id: u64,
        freshness: Freshness,
        completeness: Completeness,
        payload: SamplePayload,
        invalidation: Option<Invalidation>,
        observed_at: impl Into<String>,
    ) -> Result<ReadinessProjection, StoreError> {
        let emission = self.inner.borrow_mut().emit_snapshot(
            subscription_id,
            freshness,
            completeness,
            payload,
            invalidation,
        )?;
        Ok(self.fan_out(subscription_id, &emission.envelope, observed_at))
    }

    /// Publish a delta frame on the shared subscription.
    pub fn publish_delta(
        &self,
        subscription_id: u64,
        freshness: Freshness,
        completeness: Completeness,
        payload: SamplePayload,
        invalidation: Option<Invalidation>,
        observed_at: impl Into<String>,
    ) -> Result<ReadinessProjection, StoreError> {
        let emission = self.inner.borrow_mut().emit_delta(
            subscription_id,
            freshness,
            completeness,
            payload,
            invalidation,
        )?;
        Ok(self.fan_out(subscription_id, &emission.envelope, observed_at))
    }

    /// Returns the latest projection for a `(query_family,
    /// scope_ref)` pair, if any has been published.
    pub fn latest(
        &self,
        query_family: &str,
        scope_ref: &ScopeRef,
    ) -> Option<ReadinessProjection> {
        self.subscriptions
            .borrow()
            .iter()
            .find(|s| s.query_family == query_family && &s.scope_ref == scope_ref)
            .and_then(|s| s.latest.clone())
    }

    /// Number of shared subscriptions currently open. Useful for
    /// tests and diagnostics; not part of the surface contract.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions.borrow().len()
    }

    /// Number of observers attached to a given shared subscription.
    pub fn observer_count(&self, subscription_id: u64) -> usize {
        self.subscriptions
            .borrow()
            .iter()
            .find(|s| s.subscription_id == subscription_id)
            .map(|s| s.observers.len())
            .unwrap_or(0)
    }

    fn fan_out(
        &self,
        subscription_id: u64,
        envelope: &SubscriptionEnvelope,
        observed_at: impl Into<String>,
    ) -> ReadinessProjection {
        let projection = ReadinessProjection::from_envelope(envelope, observed_at);
        let mut subs = self.subscriptions.borrow_mut();
        if let Some(sub) = subs
            .iter_mut()
            .find(|s| s.subscription_id == subscription_id)
        {
            sub.latest = Some(projection.clone());
            for obs in &sub.observers {
                (obs.observer)(&projection);
            }
        }
        projection
    }
}

// ---------------------------------------------------------------------------
// Workspace-readiness adaptor.
//
// The shell needs a typed entry point that takes the workspace
// lifecycle's primitive vocabulary (state name + watcher health
// token + readiness gate booleans) and publishes a frame. The
// adaptor lives in this crate so the workspace and shell crates
// do not have to re-encode the projection rules.
// ---------------------------------------------------------------------------

/// Primitive workspace-lifecycle state token used by the readiness
/// adaptor. Mirrors `aureline_workspace::WorkspaceLifecycleState`
/// so the adaptor does not take a hard dependency on that crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkspaceLifecyclePhase {
    Discovered,
    TrustEvaluating,
    Opening,
    PartiallyReady,
    Ready,
    Degraded,
    Closing,
    Closed,
}

impl WorkspaceLifecyclePhase {
    /// Parse the snake-case token emitted by
    /// `aureline_workspace::WorkspaceLifecycleState::as_str`.
    pub fn from_token(token: &str) -> Option<Self> {
        Some(match token {
            "discovered" => Self::Discovered,
            "trust_evaluating" => Self::TrustEvaluating,
            "opening" => Self::Opening,
            "partially_ready" => Self::PartiallyReady,
            "ready" => Self::Ready,
            "degraded" => Self::Degraded,
            "closing" => Self::Closing,
            "closed" => Self::Closed,
            _ => return None,
        })
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Discovered => "discovered",
            Self::TrustEvaluating => "trust_evaluating",
            Self::Opening => "opening",
            Self::PartiallyReady => "partially_ready",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Closing => "closing",
            Self::Closed => "closed",
        }
    }
}

/// Primitive watcher-health token. Mirrors
/// `aureline_vfs::WatcherHealth` so the adaptor does not take a
/// hard dependency on that crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatcherHealthPhase {
    Healthy,
    Warming,
    Degraded,
    FallbackPolling,
    Unavailable,
}

impl WatcherHealthPhase {
    /// Parse the snake-case token emitted by
    /// `aureline_vfs::WatcherHealth::as_str`.
    pub fn from_token(token: &str) -> Option<Self> {
        Some(match token {
            "healthy" => Self::Healthy,
            "warming" => Self::Warming,
            "degraded" => Self::Degraded,
            "fallback_polling" => Self::FallbackPolling,
            "unavailable" => Self::Unavailable,
            _ => return None,
        })
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Warming => "warming",
            Self::Degraded => "degraded",
            Self::FallbackPolling => "fallback_polling",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Snapshot of the workspace-readiness inputs. Constructed by the
/// shell after pulling the latest values from the workspace
/// lifecycle machine and the VFS watcher service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceReadinessSnapshot {
    pub workspace_id: String,
    pub lifecycle_phase: WorkspaceLifecyclePhase,
    pub watcher_health: Option<WatcherHealthPhase>,
    pub hot_index_ready: bool,
    pub command_graph_ready: bool,
    pub observed_at: String,
}

impl WorkspaceReadinessSnapshot {
    /// Map the readiness inputs into the envelope's
    /// `(freshness, completeness)` posture. Returns the
    /// `(freshness, completeness, invalidation)` tuple to publish.
    pub fn to_envelope_inputs(
        &self,
    ) -> (Freshness, Completeness, Option<Invalidation>) {
        match self.lifecycle_phase {
            WorkspaceLifecyclePhase::Closed | WorkspaceLifecyclePhase::Closing => (
                Freshness::Stale,
                Completeness::Unavailable,
                Some(Invalidation {
                    stale_reason: StaleReason::ScopeRemoved,
                    caused_by: None,
                }),
            ),
            WorkspaceLifecyclePhase::Discovered
            | WorkspaceLifecyclePhase::TrustEvaluating
            | WorkspaceLifecyclePhase::Opening => (
                Freshness::Warming,
                Completeness::Unloaded,
                None,
            ),
            WorkspaceLifecyclePhase::PartiallyReady => (
                Freshness::Warming,
                Completeness::Partial,
                None,
            ),
            WorkspaceLifecyclePhase::Ready => {
                let healthy = matches!(self.watcher_health, Some(WatcherHealthPhase::Healthy));
                if healthy && self.hot_index_ready && self.command_graph_ready {
                    (Freshness::Authoritative, Completeness::Full, None)
                } else if matches!(self.watcher_health, Some(WatcherHealthPhase::Warming)) {
                    (Freshness::Warming, Completeness::Partial, None)
                } else {
                    (
                        Freshness::Authoritative,
                        Completeness::Partial,
                        None,
                    )
                }
            }
            WorkspaceLifecyclePhase::Degraded => match self.watcher_health {
                Some(WatcherHealthPhase::Unavailable) => (
                    Freshness::Stale,
                    Completeness::Unavailable,
                    Some(Invalidation {
                        stale_reason: StaleReason::WatcherDropped,
                        caused_by: None,
                    }),
                ),
                Some(WatcherHealthPhase::Degraded)
                | Some(WatcherHealthPhase::FallbackPolling) => (
                    Freshness::Stale,
                    Completeness::Partial,
                    Some(Invalidation {
                        stale_reason: StaleReason::WatcherDropped,
                        caused_by: None,
                    }),
                ),
                _ => (
                    Freshness::Cached,
                    Completeness::Partial,
                    Some(Invalidation {
                        stale_reason: StaleReason::CacheServed,
                        caused_by: None,
                    }),
                ),
            },
        }
    }
}

/// Build a workspace-readiness producer for the live runtime.
/// Authority class = `workspace_vfs`, derivation class =
/// `authoritative`, view class = `durable_local_materialization`.
pub fn workspace_readiness_producer(workspace_id: &str) -> Producer {
    let producer_id = "aureline.workspace.readiness";
    Producer {
        query_family: "workspace.readiness".to_owned(),
        scope_ref: ScopeRef {
            class: ScopeClass::Workspace,
            id: workspace_id.to_owned(),
        },
        authority_class: AuthorityClass::WorkspaceVfs,
        derivation_class: DerivationClass::Authoritative,
        view_class: ViewClass::DurableLocalMaterialization,
        backpressure_mode: BackpressureMode::Coalesced,
        producer_refs: vec![ProducerRef {
            producer_id: producer_id.to_owned(),
            producer_instance: format!("aureline-shell/{workspace_id}"),
            producer_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
            input_digests: vec![],
            derivation_epoch: None,
            source: None,
        }],
    }
}

/// Convenience helper: register the workspace-readiness producer
/// on the live store, open a shared subscription, and publish the
/// initial snapshot. Returns the subscription id and the projection
/// observers will see for the initial frame.
pub fn open_workspace_readiness(
    store: &LiveReactiveStore,
    snapshot: &WorkspaceReadinessSnapshot,
) -> Result<(u64, ReadinessProjection), StoreError> {
    let producer = workspace_readiness_producer(&snapshot.workspace_id);
    let sid = store.open_shared_subscription(producer, BackpressureMode::Coalesced)?;
    let (freshness, completeness, invalidation) = snapshot.to_envelope_inputs();
    let projection = store.publish_snapshot(
        sid,
        freshness,
        completeness,
        SamplePayload {
            summary: format!(
                "workspace_lifecycle={} watcher={}",
                snapshot.lifecycle_phase.as_str(),
                snapshot
                    .watcher_health
                    .map(WatcherHealthPhase::as_str)
                    .unwrap_or("unobserved"),
            ),
            entry_count: 0,
            coverage_ready: u64::from(snapshot.hot_index_ready)
                + u64::from(snapshot.command_graph_ready),
            coverage_total: 2,
            detail_lines: vec![],
        },
        invalidation,
        snapshot.observed_at.clone(),
    )?;
    Ok((sid, projection))
}

/// Republish the workspace-readiness frame after the inputs have
/// changed. Emits a delta unless the freshness or completeness
/// would jump in a way that requires a snapshot resync; the
/// adaptor takes the safe path and emits a snapshot in that case.
pub fn republish_workspace_readiness(
    store: &LiveReactiveStore,
    subscription_id: u64,
    snapshot: &WorkspaceReadinessSnapshot,
) -> Result<ReadinessProjection, StoreError> {
    let (freshness, completeness, invalidation) = snapshot.to_envelope_inputs();
    let payload = SamplePayload {
        summary: format!(
            "workspace_lifecycle={} watcher={}",
            snapshot.lifecycle_phase.as_str(),
            snapshot
                .watcher_health
                .map(WatcherHealthPhase::as_str)
                .unwrap_or("unobserved"),
        ),
        entry_count: 0,
        coverage_ready: u64::from(snapshot.hot_index_ready)
            + u64::from(snapshot.command_graph_ready),
        coverage_total: 2,
        detail_lines: vec![],
    };
    // Snapshot when the producer would otherwise be unable to
    // emit a delta (e.g. completeness moved to unavailable). This
    // is a conservative choice: a real producer would consult the
    // backpressure mode and the prior projection.
    let needs_snapshot = matches!(completeness, Completeness::Unavailable)
        || matches!(freshness, Freshness::Stale | Freshness::Cached);
    if needs_snapshot {
        store.publish_snapshot(
            subscription_id,
            freshness,
            completeness,
            payload,
            invalidation,
            snapshot.observed_at.clone(),
        )
    } else {
        store.publish_delta(
            subscription_id,
            freshness,
            completeness,
            payload,
            invalidation,
            snapshot.observed_at.clone(),
        )
    }
}

/// Render the readiness label as a one-line chip suitable for the
/// shell title bar / status surfaces. Surfaces MAY add their own
/// chrome but MUST NOT change the label or the not-ready reason.
pub fn render_chip_line(projection: &ReadinessProjection) -> String {
    let mut out = format!(
        "[{label}] {family}",
        label = projection.readiness_label.label(),
        family = projection.query_family,
    );
    if let Some(reason) = projection.not_ready_reason {
        out.push_str(&format!(" — {}", reason.as_str()));
    }
    out
}

/// Returns the count map of readiness labels currently held in the
/// live store. Used by tests and exported diagnostics.
pub fn readiness_label_counts(store: &LiveReactiveStore) -> BTreeMap<&'static str, u64> {
    let mut counts: BTreeMap<&'static str, u64> = BTreeMap::new();
    for sub in store.subscriptions.borrow().iter() {
        if let Some(projection) = &sub.latest {
            *counts.entry(projection.readiness_label.as_str()).or_insert(0) += 1;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn snapshot(
        phase: WorkspaceLifecyclePhase,
        watcher: Option<WatcherHealthPhase>,
        hot_index: bool,
        command_graph: bool,
    ) -> WorkspaceReadinessSnapshot {
        WorkspaceReadinessSnapshot {
            workspace_id: "ws-test".to_owned(),
            lifecycle_phase: phase,
            watcher_health: watcher,
            hot_index_ready: hot_index,
            command_graph_ready: command_graph,
            observed_at: "mono:1".to_owned(),
        }
    }

    #[test]
    fn ready_with_healthy_watcher_projects_exact() {
        let store = LiveReactiveStore::new();
        let (_sid, projection) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
            ),
        )
        .unwrap();
        assert_eq!(projection.readiness_label, ReadinessLabel::Exact);
        assert!(projection.not_ready_reason.is_none());
    }

    #[test]
    fn partially_ready_projects_partial() {
        let store = LiveReactiveStore::new();
        let (_sid, projection) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::PartiallyReady,
                Some(WatcherHealthPhase::Warming),
                false,
                false,
            ),
        )
        .unwrap();
        assert_eq!(projection.readiness_label, ReadinessLabel::Partial);
    }

    #[test]
    fn degraded_with_fallback_watcher_projects_stale() {
        let store = LiveReactiveStore::new();
        let (_sid, projection) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Degraded,
                Some(WatcherHealthPhase::FallbackPolling),
                true,
                true,
            ),
        )
        .unwrap();
        assert_eq!(projection.readiness_label, ReadinessLabel::Stale);
        assert_eq!(
            projection.not_ready_reason,
            Some(StaleReason::WatcherDropped)
        );
    }

    #[test]
    fn closed_workspace_projects_unavailable() {
        let store = LiveReactiveStore::new();
        let (_sid, projection) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Closed,
                Some(WatcherHealthPhase::Unavailable),
                false,
                false,
            ),
        )
        .unwrap();
        assert_eq!(projection.readiness_label, ReadinessLabel::Unavailable);
        assert_eq!(projection.not_ready_reason, Some(StaleReason::ScopeRemoved));
    }

    #[test]
    fn multiple_observers_share_one_cache_without_drift() {
        let store = LiveReactiveStore::new();
        let (sid, _initial) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::PartiallyReady,
                Some(WatcherHealthPhase::Warming),
                false,
                true,
            ),
        )
        .unwrap();

        let chrome_log: Rc<RefCell<Vec<ReadinessLabel>>> = Rc::new(RefCell::new(Vec::new()));
        let inspector_log: Rc<RefCell<Vec<ReadinessLabel>>> = Rc::new(RefCell::new(Vec::new()));

        let chrome_log_inner = Rc::clone(&chrome_log);
        let _chrome_token = store
            .subscribe(
                sid,
                Rc::new(move |p: &ReadinessProjection| {
                    chrome_log_inner.borrow_mut().push(p.readiness_label);
                }),
            )
            .unwrap();
        let inspector_log_inner = Rc::clone(&inspector_log);
        let _inspector_token = store
            .subscribe(
                sid,
                Rc::new(move |p: &ReadinessProjection| {
                    inspector_log_inner.borrow_mut().push(p.readiness_label);
                }),
            )
            .unwrap();

        // Both observers must have replayed the latest projection
        // when they subscribed, so they start from the same value.
        assert_eq!(chrome_log.borrow().as_slice(), &[ReadinessLabel::Partial]);
        assert_eq!(inspector_log.borrow().as_slice(), &[ReadinessLabel::Partial]);

        // Republish: both observers must see the same change in
        // the same order, with no private cache drift.
        republish_workspace_readiness(
            &store,
            sid,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
            ),
        )
        .unwrap();
        assert_eq!(
            chrome_log.borrow().as_slice(),
            &[ReadinessLabel::Partial, ReadinessLabel::Exact]
        );
        assert_eq!(
            inspector_log.borrow().as_slice(),
            &[ReadinessLabel::Partial, ReadinessLabel::Exact]
        );
    }

    #[test]
    fn late_subscriber_sees_latest_projection() {
        let store = LiveReactiveStore::new();
        let (sid, _initial) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Degraded,
                Some(WatcherHealthPhase::Degraded),
                true,
                true,
            ),
        )
        .unwrap();
        let observed: Rc<RefCell<Vec<ReadinessLabel>>> = Rc::new(RefCell::new(Vec::new()));
        let observed_inner = Rc::clone(&observed);
        store
            .subscribe(
                sid,
                Rc::new(move |p: &ReadinessProjection| {
                    observed_inner.borrow_mut().push(p.readiness_label);
                }),
            )
            .unwrap();
        // The late subscriber must have been replayed the cached
        // Stale projection rather than starting from an empty cache.
        assert_eq!(observed.borrow().as_slice(), &[ReadinessLabel::Stale]);
    }

    #[test]
    fn unsubscribe_stops_callbacks() {
        let store = LiveReactiveStore::new();
        let (sid, _initial) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
            ),
        )
        .unwrap();
        let observed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
        let observed_inner = Rc::clone(&observed);
        let token = store
            .subscribe(
                sid,
                Rc::new(move |_p: &ReadinessProjection| {
                    *observed_inner.borrow_mut() += 1;
                }),
            )
            .unwrap();
        // Replay on subscribe.
        assert_eq!(*observed.borrow(), 1);
        store.unsubscribe(token).unwrap();
        republish_workspace_readiness(
            &store,
            sid,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
            ),
        )
        .unwrap();
        // No further callbacks after unsubscribe.
        assert_eq!(*observed.borrow(), 1);
    }

    #[test]
    fn render_chip_line_quotes_label_and_reason() {
        let store = LiveReactiveStore::new();
        let (_sid, projection) = open_workspace_readiness(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Degraded,
                Some(WatcherHealthPhase::FallbackPolling),
                true,
                true,
            ),
        )
        .unwrap();
        let line = render_chip_line(&projection);
        assert!(line.contains("Stale"), "line: {line}");
        assert!(line.contains("workspace.readiness"), "line: {line}");
        assert!(line.contains("watcher_dropped"), "line: {line}");
    }

    #[test]
    fn readiness_case_fixtures_match_expected_projection() {
        use serde::Deserialize;
        use std::path::Path;

        #[derive(Debug, Deserialize)]
        struct ReadinessCaseFixture {
            record_kind: String,
            schema_version: u32,
            #[serde(default)]
            #[allow(dead_code)]
            case_id: String,
            #[serde(default)]
            #[allow(dead_code)]
            title: String,
            input: ReadinessCaseInput,
            expect: ReadinessCaseExpect,
        }

        #[derive(Debug, Deserialize)]
        struct ReadinessCaseInput {
            workspace_id: String,
            lifecycle_phase: String,
            watcher_health: Option<String>,
            hot_index_ready: bool,
            command_graph_ready: bool,
            observed_at: String,
        }

        #[derive(Debug, Deserialize)]
        struct ReadinessCaseExpect {
            freshness: String,
            completeness: String,
            readiness_label: String,
            not_ready_reason: Option<String>,
        }

        let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/state/readiness_cases");
        let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
            .expect("readiness_cases directory must exist")
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        fixtures.sort();
        assert!(
            fixtures.len() >= 4,
            "expected at least 4 readiness_case fixtures, found {}",
            fixtures.len()
        );

        for path in fixtures {
            let payload = std::fs::read_to_string(&path).expect("readiness_case fixture must read");
            let fixture: ReadinessCaseFixture =
                serde_json::from_str(&payload).expect("readiness_case fixture must parse");
            assert_eq!(
                fixture.record_kind, "readiness_case",
                "unexpected record_kind in {path:?}"
            );
            assert_eq!(
                fixture.schema_version, 1,
                "unexpected schema_version in {path:?}"
            );

            let lifecycle_phase = WorkspaceLifecyclePhase::from_token(&fixture.input.lifecycle_phase)
                .unwrap_or_else(|| panic!("unknown lifecycle_phase in {path:?}"));
            let watcher_health = fixture
                .input
                .watcher_health
                .as_deref()
                .map(|token| {
                    WatcherHealthPhase::from_token(token)
                        .unwrap_or_else(|| panic!("unknown watcher_health in {path:?}"))
                });

            let snapshot = WorkspaceReadinessSnapshot {
                workspace_id: fixture.input.workspace_id,
                lifecycle_phase,
                watcher_health,
                hot_index_ready: fixture.input.hot_index_ready,
                command_graph_ready: fixture.input.command_graph_ready,
                observed_at: fixture.input.observed_at,
            };

            let store = LiveReactiveStore::new();
            let (_sid, projection) = open_workspace_readiness(&store, &snapshot).unwrap();
            assert_eq!(
                projection.freshness.as_str(),
                fixture.expect.freshness,
                "freshness mismatch in {path:?}"
            );
            assert_eq!(
                projection.completeness.as_str(),
                fixture.expect.completeness,
                "completeness mismatch in {path:?}"
            );
            assert_eq!(
                projection.readiness_label.as_str(),
                fixture.expect.readiness_label,
                "readiness_label mismatch in {path:?}"
            );
            assert_eq!(
                projection.not_ready_reason.map(|r| r.as_str().to_owned()),
                fixture.expect.not_ready_reason,
                "not_ready_reason mismatch in {path:?}"
            );
        }
    }

    #[test]
    fn lifecycle_phase_round_trip_via_token_string() {
        let cases = [
            WorkspaceLifecyclePhase::Discovered,
            WorkspaceLifecyclePhase::TrustEvaluating,
            WorkspaceLifecyclePhase::Opening,
            WorkspaceLifecyclePhase::PartiallyReady,
            WorkspaceLifecyclePhase::Ready,
            WorkspaceLifecyclePhase::Degraded,
            WorkspaceLifecyclePhase::Closing,
            WorkspaceLifecyclePhase::Closed,
        ];
        for phase in cases {
            assert_eq!(WorkspaceLifecyclePhase::from_token(phase.as_str()), Some(phase));
        }
        assert_eq!(WorkspaceLifecyclePhase::from_token("nope"), None);
    }
}
