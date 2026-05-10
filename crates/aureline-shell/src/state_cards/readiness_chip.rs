//! Workspace-readiness chip: shell consumer of the shared
//! [`aureline_reactive_state::LiveReactiveStore`].
//!
//! The chip is the M1 minimal slice that wires a derived shell
//! surface to the reactive-state runtime. Two shell consumers (a
//! title/context bar chip and a status-bar mirror) subscribe to
//! the same shared subscription and render the same
//! [`ReadinessLabel`] without inventing private booleans.
//!
//! Why a chip on top of the shared store:
//!
//! - the chip's label, badge, and reason text are derived from
//!   one [`ReadinessProjection`], so screenshot-safe renders,
//!   machine output, and the support/export surface quote the
//!   exact same vocabulary;
//! - the chip subscribes through the live store rather than
//!   reading a local cache, satisfying the "multiple shell
//!   surfaces can subscribe to shared state without creating
//!   private caches that drift" acceptance criterion; and
//! - the readiness-label → degraded-state-token mapping is the
//!   sole place that crosses the reactive-state vocabulary into
//!   the chrome's degraded-state vocabulary, so the boundary
//!   stays auditable.

use std::cell::RefCell;
use std::rc::Rc;

use aureline_reactive_state::{
    open_workspace_readiness, LiveReactiveStore, LiveSubscriptionToken, ReadinessLabel,
    ReadinessProjection, StoreError, WatcherHealthPhase, WorkspaceLifecyclePhase,
    WorkspaceReadinessSnapshot,
};
use aureline_workspace::WorkspaceReadinessInputs;
use serde::{Deserialize, Serialize};

use super::DegradedStateToken;

/// Convert canonical readiness inputs from a workspace lifecycle
/// machine into the reactive-state runtime's input snapshot. Tokens
/// the lifecycle machine emits but the runtime does not recognise
/// fall back to safe values: an unrecognised lifecycle token is
/// treated as `closed` (so the readiness label projects as
/// `unavailable` rather than silently rendering Exact).
pub fn readiness_snapshot_from_lifecycle(
    inputs: &WorkspaceReadinessInputs,
) -> WorkspaceReadinessSnapshot {
    let lifecycle_phase = WorkspaceLifecyclePhase::from_token(inputs.lifecycle_state_token)
        .unwrap_or(WorkspaceLifecyclePhase::Closed);
    let watcher_health = inputs
        .watcher_health_token
        .and_then(WatcherHealthPhase::from_token);
    WorkspaceReadinessSnapshot {
        workspace_id: inputs.workspace_id.clone(),
        lifecycle_phase,
        watcher_health,
        hot_index_ready: inputs.hot_index_ready,
        command_graph_ready: inputs.command_graph_ready,
        observed_at: inputs.observed_at.clone(),
    }
}

/// Map a [`ReadinessLabel`] into the canonical
/// [`DegradedStateToken`] used by chrome and placeholder cards.
///
/// `Exact` returns `None` because an authoritative-and-complete
/// surface does not need a degraded badge. Every other label maps
/// to the closest existing chrome token — no synonyms, no surface-
/// local labels.
pub const fn readiness_label_to_degraded_token(
    label: ReadinessLabel,
) -> Option<DegradedStateToken> {
    match label {
        ReadinessLabel::Exact => None,
        ReadinessLabel::Imported => Some(DegradedStateToken::Cached),
        ReadinessLabel::Heuristic => Some(DegradedStateToken::Limited),
        ReadinessLabel::Stale => Some(DegradedStateToken::Stale),
        ReadinessLabel::Partial => Some(DegradedStateToken::Partial),
        ReadinessLabel::Unavailable => Some(DegradedStateToken::Offline),
        ReadinessLabel::OutOfScope => Some(DegradedStateToken::Limited),
    }
}

/// Serializable workspace-readiness chip record. Chrome surfaces
/// render this verbatim; export/diagnostic surfaces serialize the
/// same record without translation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceReadinessChipRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub query_family: String,
    pub readiness_label: String,
    pub readiness_label_display: String,
    pub freshness: String,
    pub completeness: String,
    pub frame_class: String,
    pub snapshot_epoch: u64,
    pub delta_seq: u64,
    pub producer_id: String,
    pub producer_version: Option<String>,
    pub observed_at: String,
    pub not_ready_reason: Option<String>,
    pub degraded_token: Option<String>,
    pub summary: String,
}

const WORKSPACE_READINESS_CHIP_SCHEMA_VERSION: u32 = 1;

/// Materialize a chip record from a [`ReadinessProjection`]. The
/// projection is the shared truth; this function never invents
/// fields the projection cannot prove.
pub fn materialize_workspace_readiness_chip(
    projection: &ReadinessProjection,
) -> WorkspaceReadinessChipRecord {
    let degraded = readiness_label_to_degraded_token(projection.readiness_label);
    let summary = chip_summary_line(projection);
    WorkspaceReadinessChipRecord {
        record_kind: "workspace_readiness_chip_record".to_owned(),
        schema_version: WORKSPACE_READINESS_CHIP_SCHEMA_VERSION,
        workspace_id: projection.scope_ref.id.clone(),
        query_family: projection.query_family.clone(),
        readiness_label: projection.readiness_label.as_str().to_owned(),
        readiness_label_display: projection.readiness_label.label().to_owned(),
        freshness: projection.freshness.as_str().to_owned(),
        completeness: projection.completeness.as_str().to_owned(),
        frame_class: projection.frame_class.as_str().to_owned(),
        snapshot_epoch: projection.snapshot_epoch,
        delta_seq: projection.delta_seq,
        producer_id: projection.producer_id.clone(),
        producer_version: projection.producer_version.clone(),
        observed_at: projection.observed_at.clone(),
        not_ready_reason: projection.not_ready_reason.map(|r| r.as_str().to_owned()),
        degraded_token: degraded.map(|t| t.token().to_owned()),
        summary,
    }
}

fn chip_summary_line(projection: &ReadinessProjection) -> String {
    match projection.not_ready_reason {
        Some(reason) => format!(
            "{label} — {reason} (epoch={epoch}, seq={seq})",
            label = projection.readiness_label.label(),
            reason = reason.as_str(),
            epoch = projection.snapshot_epoch,
            seq = projection.delta_seq,
        ),
        None => format!(
            "{label} (epoch={epoch}, seq={seq})",
            label = projection.readiness_label.label(),
            epoch = projection.snapshot_epoch,
            seq = projection.delta_seq,
        ),
    }
}

/// Live workspace-readiness chip mounted on a shell zone. Holds a
/// shared subscription token plus a callback-driven cache of the
/// most recent rendered chip record so the host can re-render
/// without polling.
///
/// Multiple instances mounted on the same `(workspace_id,
/// LiveReactiveStore)` pair MUST observe identical chip records;
/// the regression test in this module exercises that invariant.
pub struct WorkspaceReadinessChipMount {
    workspace_id: String,
    token: LiveSubscriptionToken,
    latest: Rc<RefCell<Option<WorkspaceReadinessChipRecord>>>,
}

impl WorkspaceReadinessChipMount {
    /// Mount a workspace-readiness chip onto an existing shared
    /// subscription. The chip starts populated with the latest
    /// projection cached on the store, if one is present.
    pub fn mount_existing(
        store: &LiveReactiveStore,
        subscription_id: u64,
        workspace_id: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let latest: Rc<RefCell<Option<WorkspaceReadinessChipRecord>>> = Rc::new(RefCell::new(None));
        let latest_inner = Rc::clone(&latest);
        let token = store.subscribe(
            subscription_id,
            Rc::new(move |projection: &ReadinessProjection| {
                *latest_inner.borrow_mut() = Some(materialize_workspace_readiness_chip(projection));
            }),
        )?;
        Ok(Self {
            workspace_id: workspace_id.into(),
            token,
            latest,
        })
    }

    /// Convenience constructor: open a shared workspace-readiness
    /// subscription on the live store and mount one chip on it.
    /// Returns the mount and the subscription id so additional
    /// shell surfaces can attach via [`Self::mount_existing`].
    pub fn open_and_mount(
        store: &LiveReactiveStore,
        snapshot: &WorkspaceReadinessSnapshot,
    ) -> Result<(Self, u64), StoreError> {
        let (sid, _initial) = open_workspace_readiness(store, snapshot)?;
        let mount = Self::mount_existing(store, sid, snapshot.workspace_id.clone())?;
        Ok((mount, sid))
    }

    /// Returns the latest rendered chip record, if any frame has
    /// been published.
    pub fn latest_record(&self) -> Option<WorkspaceReadinessChipRecord> {
        self.latest.borrow().clone()
    }

    /// Returns the workspace id this chip was mounted for.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Returns the underlying shared subscription token.
    pub const fn token(&self) -> LiveSubscriptionToken {
        self.token
    }

    /// Detach the chip from the live store. Subsequent publishes
    /// will not refresh `latest_record`.
    pub fn unmount(self, store: &LiveReactiveStore) -> Result<(), StoreError> {
        store.unsubscribe(self.token)
    }
}

#[cfg(test)]
mod tests {
    use aureline_reactive_state::{
        republish_workspace_readiness, WatcherHealthPhase, WorkspaceLifecyclePhase,
    };

    use super::*;

    fn snapshot(
        phase: WorkspaceLifecyclePhase,
        watcher: Option<WatcherHealthPhase>,
        hot_index: bool,
        command_graph: bool,
        observed_at: &str,
    ) -> WorkspaceReadinessSnapshot {
        WorkspaceReadinessSnapshot {
            workspace_id: "ws-protected".to_owned(),
            lifecycle_phase: phase,
            watcher_health: watcher,
            hot_index_ready: hot_index,
            command_graph_ready: command_graph,
            observed_at: observed_at.to_owned(),
        }
    }

    #[test]
    fn chip_label_matches_readiness_projection() {
        let store = LiveReactiveStore::new();
        let (mount, _sid) = WorkspaceReadinessChipMount::open_and_mount(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
                "mono:1",
            ),
        )
        .unwrap();
        let record = mount.latest_record().expect("initial frame");
        assert_eq!(record.readiness_label, "exact");
        assert_eq!(record.readiness_label_display, "Ready");
        assert_eq!(record.degraded_token, None);
        assert_eq!(record.workspace_id, "ws-protected");
        assert!(record.not_ready_reason.is_none());
    }

    #[test]
    fn partial_state_renders_degraded_partial_badge() {
        let store = LiveReactiveStore::new();
        let (mount, _sid) = WorkspaceReadinessChipMount::open_and_mount(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::PartiallyReady,
                Some(WatcherHealthPhase::Warming),
                false,
                true,
                "mono:1",
            ),
        )
        .unwrap();
        let record = mount.latest_record().expect("initial frame");
        assert_eq!(record.readiness_label, "partial");
        assert_eq!(record.degraded_token.as_deref(), Some("Partial"));
    }

    #[test]
    fn degraded_state_renders_stale_badge_with_reason() {
        let store = LiveReactiveStore::new();
        let (mount, _sid) = WorkspaceReadinessChipMount::open_and_mount(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Degraded,
                Some(WatcherHealthPhase::FallbackPolling),
                true,
                true,
                "mono:1",
            ),
        )
        .unwrap();
        let record = mount.latest_record().expect("initial frame");
        assert_eq!(record.readiness_label, "stale");
        assert_eq!(record.degraded_token.as_deref(), Some("Stale"));
        assert_eq!(record.not_ready_reason.as_deref(), Some("watcher_dropped"));
        assert!(
            record.summary.contains("watcher_dropped"),
            "summary missing reason: {}",
            record.summary
        );
    }

    #[test]
    fn closed_workspace_renders_unavailable_badge() {
        let store = LiveReactiveStore::new();
        let (mount, _sid) = WorkspaceReadinessChipMount::open_and_mount(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Closed,
                Some(WatcherHealthPhase::Unavailable),
                false,
                false,
                "mono:1",
            ),
        )
        .unwrap();
        let record = mount.latest_record().expect("initial frame");
        assert_eq!(record.readiness_label, "unavailable");
        assert_eq!(record.degraded_token.as_deref(), Some("Offline"));
    }

    #[test]
    fn two_chips_share_one_projection_without_drift() {
        let store = LiveReactiveStore::new();
        let (chip_a, sid) = WorkspaceReadinessChipMount::open_and_mount(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::PartiallyReady,
                Some(WatcherHealthPhase::Warming),
                false,
                true,
                "mono:1",
            ),
        )
        .unwrap();
        let chip_b = WorkspaceReadinessChipMount::mount_existing(&store, sid, "ws-protected")
            .expect("second mount");

        // Both chips replayed the same cached projection on
        // mount; they cannot drift.
        let record_a = chip_a.latest_record().expect("chip_a initial frame");
        let record_b = chip_b.latest_record().expect("chip_b initial frame");
        assert_eq!(record_a, record_b);
        assert_eq!(store.observer_count(sid), 2);

        // Republish: both chips must observe the same updated
        // record, with no second-cache divergence.
        republish_workspace_readiness(
            &store,
            sid,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
                "mono:2",
            ),
        )
        .unwrap();
        let record_a = chip_a.latest_record().expect("chip_a updated frame");
        let record_b = chip_b.latest_record().expect("chip_b updated frame");
        assert_eq!(record_a, record_b);
        assert_eq!(record_a.readiness_label, "exact");
    }

    #[test]
    fn unmount_stops_further_updates() {
        let store = LiveReactiveStore::new();
        let (chip, sid) = WorkspaceReadinessChipMount::open_and_mount(
            &store,
            &snapshot(
                WorkspaceLifecyclePhase::Ready,
                Some(WatcherHealthPhase::Healthy),
                true,
                true,
                "mono:1",
            ),
        )
        .unwrap();
        let initial = chip.latest_record().expect("initial frame");
        chip.unmount(&store).unwrap();
        assert_eq!(store.observer_count(sid), 0);

        // Republishing after unmount must not refresh the
        // already-detached chip cache.
        republish_workspace_readiness(
            &store,
            sid,
            &snapshot(
                WorkspaceLifecyclePhase::Degraded,
                Some(WatcherHealthPhase::FallbackPolling),
                true,
                true,
                "mono:2",
            ),
        )
        .unwrap();

        // The store's latest projection moved to Stale, but the
        // detached chip is still on the initial Exact value.
        let updated_in_store = store
            .latest("workspace.readiness", &initial_scope())
            .expect("store latest");
        assert_eq!(updated_in_store.readiness_label, ReadinessLabel::Stale);
    }

    fn initial_scope() -> aureline_reactive_state::ScopeRef {
        aureline_reactive_state::ScopeRef {
            class: aureline_reactive_state::ScopeClass::Workspace,
            id: "ws-protected".to_owned(),
        }
    }

    #[test]
    fn lifecycle_machine_drives_chip_through_partially_ready_to_ready() {
        use aureline_vfs::WatcherHealth;
        use aureline_workspace::{TrustState, WorkspaceLifecycleMachine};

        // Drive a workspace lifecycle machine through the
        // protected `discovered → trust_evaluating → opening →
        // partially_ready → ready` walk and confirm the chip
        // mounted on the live store renders the matching
        // readiness label at every stage rather than inventing
        // private readiness booleans.
        let store = LiveReactiveStore::new();
        let mut machine = WorkspaceLifecycleMachine::discovered("ws-protected", "mono:0");
        machine.open_workspace("mono:1");
        machine.resolve_trust(TrustState::Trusted, "mono:2");
        machine.mark_shell_interactive("mono:3");
        machine.update_readiness_gates(
            Some(WatcherHealth::Warming),
            Some(false),
            Some(false),
            "mono:4",
            None,
        );

        let snapshot = readiness_snapshot_from_lifecycle(&machine.readiness_inputs());
        let (chip, sid) = WorkspaceReadinessChipMount::open_and_mount(&store, &snapshot).unwrap();
        let initial = chip.latest_record().expect("initial chip");
        assert_eq!(initial.readiness_label, "partial");
        assert_eq!(initial.degraded_token.as_deref(), Some("Partial"));

        // Move the workspace to fully ready and republish.
        machine.update_readiness_gates(
            Some(WatcherHealth::Healthy),
            Some(true),
            Some(true),
            "mono:5",
            None,
        );
        let updated_snapshot = readiness_snapshot_from_lifecycle(&machine.readiness_inputs());
        republish_workspace_readiness(&store, sid, &updated_snapshot).unwrap();
        let updated = chip.latest_record().expect("updated chip");
        assert_eq!(updated.readiness_label, "exact");
        assert_eq!(updated.degraded_token, None);

        // Force a watcher fault: the chip must render Stale, not
        // a generic loading label or stale Exact.
        machine.update_readiness_gates(
            Some(WatcherHealth::FallbackPolling),
            None,
            None,
            "mono:6",
            Some("watcher_fault".to_owned()),
        );
        let degraded_snapshot = readiness_snapshot_from_lifecycle(&machine.readiness_inputs());
        republish_workspace_readiness(&store, sid, &degraded_snapshot).unwrap();
        let degraded_chip = chip.latest_record().expect("degraded chip");
        assert_eq!(degraded_chip.readiness_label, "stale");
        assert_eq!(degraded_chip.degraded_token.as_deref(), Some("Stale"));
        assert_eq!(
            degraded_chip.not_ready_reason.as_deref(),
            Some("watcher_dropped")
        );
    }
}
