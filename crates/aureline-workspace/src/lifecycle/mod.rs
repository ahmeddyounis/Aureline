//! Workspace lifecycle state machine and readiness projections.
//!
//! The workspace lifecycle vocabulary is a shared truth surface used by the
//! shell and exportable diagnostics. Consumers may cache projections but must
//! not invent their own notion of "partially ready" vs "ready" when the state
//! machine can express the running truth.

use aureline_vfs::WatcherHealth;
use serde::{Deserialize, Serialize};

use crate::TrustState;

/// Canonical lifecycle state for a workspace instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceLifecycleState {
    Discovered,
    TrustEvaluating,
    Opening,
    PartiallyReady,
    Ready,
    Degraded,
    Closing,
    Closed,
}

impl WorkspaceLifecycleState {
    /// Returns the stable string vocabulary for this lifecycle state.
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

    /// Returns true when the workspace must remain interactive for editing.
    pub const fn is_interactive(self) -> bool {
        matches!(self, Self::PartiallyReady | Self::Ready | Self::Degraded)
    }
}

/// Identifies the record kind for a [`WorkspaceLifecycleSnapshot`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceLifecycleSnapshotRecordKind {
    /// `workspace_lifecycle_snapshot_record`
    WorkspaceLifecycleSnapshotRecord,
}

/// One exportable snapshot of the workspace lifecycle state machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceLifecycleSnapshot {
    pub record_kind: WorkspaceLifecycleSnapshotRecordKind,
    pub workspace_lifecycle_snapshot_schema_version: u32,
    pub workspace_id: String,
    pub lifecycle_state: WorkspaceLifecycleState,
    pub trust_state: TrustState,
    pub watcher_health: Option<String>,
    pub hot_index_ready: bool,
    pub command_graph_ready: bool,
    pub last_transition_reason_code: Option<String>,
    pub observed_at: String,
}

/// Canonical readiness-input projection emitted by
/// [`WorkspaceLifecycleMachine::readiness_inputs`]. Exposed as a
/// stable string vocabulary so consumers (notably the reactive
/// state runtime adaptor) can build their own typed snapshot
/// without taking a hard dependency on this crate's enum types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceReadinessInputs {
    pub workspace_id: String,
    pub lifecycle_state_token: &'static str,
    pub watcher_health_token: Option<&'static str>,
    pub hot_index_ready: bool,
    pub command_graph_ready: bool,
    pub observed_at: String,
}

/// One transition frame emitted by the lifecycle state machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceLifecycleTransitionFrame {
    pub workspace_id: String,
    pub from_state: WorkspaceLifecycleState,
    pub to_state: WorkspaceLifecycleState,
    pub observed_at: String,
    pub reason_code: Option<String>,
}

/// Mutable workspace lifecycle state machine.
#[derive(Debug, Clone)]
pub struct WorkspaceLifecycleMachine {
    workspace_id: String,
    state: WorkspaceLifecycleState,
    trust_state: TrustState,
    watcher_health: Option<WatcherHealth>,
    hot_index_ready: bool,
    command_graph_ready: bool,
    observed_at: String,
    last_transition_reason_code: Option<String>,
    frames: Vec<WorkspaceLifecycleTransitionFrame>,
}

impl WorkspaceLifecycleMachine {
    /// Creates a new workspace lifecycle machine in the `discovered` state.
    pub fn discovered(workspace_id: impl Into<String>, observed_at: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            state: WorkspaceLifecycleState::Discovered,
            trust_state: TrustState::PendingEvaluation,
            watcher_health: None,
            hot_index_ready: false,
            command_graph_ready: false,
            observed_at: observed_at.into(),
            last_transition_reason_code: None,
            frames: Vec::new(),
        }
    }

    /// Returns the current workspace id.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Returns the current lifecycle state.
    pub const fn state(&self) -> WorkspaceLifecycleState {
        self.state
    }

    /// Returns the current workspace trust posture.
    pub const fn trust_state(&self) -> TrustState {
        self.trust_state
    }

    /// Returns the latest watcher health observed for the workspace roots.
    pub const fn watcher_health(&self) -> Option<WatcherHealth> {
        self.watcher_health
    }

    /// Returns whether the hot index gate is satisfied.
    pub const fn hot_index_ready(&self) -> bool {
        self.hot_index_ready
    }

    /// Returns whether the command-graph gate is satisfied.
    pub const fn command_graph_ready(&self) -> bool {
        self.command_graph_ready
    }

    /// Returns the canonical readiness inputs the reactive-state
    /// runtime adaptor consumes. Surfaces wiring the lifecycle to
    /// the [`aureline_reactive_state::LiveReactiveStore`] should
    /// convert these inputs into a
    /// [`aureline_reactive_state::WorkspaceReadinessSnapshot`]
    /// rather than re-deriving the readiness vocabulary locally.
    pub fn readiness_inputs(&self) -> WorkspaceReadinessInputs {
        WorkspaceReadinessInputs {
            workspace_id: self.workspace_id.clone(),
            lifecycle_state_token: self.state.as_str(),
            watcher_health_token: self.watcher_health.map(|h| h.as_str()),
            hot_index_ready: self.hot_index_ready,
            command_graph_ready: self.command_graph_ready,
            observed_at: self.observed_at.clone(),
        }
    }

    /// Returns an exportable snapshot of the current lifecycle state.
    pub fn snapshot(&self) -> WorkspaceLifecycleSnapshot {
        WorkspaceLifecycleSnapshot {
            record_kind: WorkspaceLifecycleSnapshotRecordKind::WorkspaceLifecycleSnapshotRecord,
            workspace_lifecycle_snapshot_schema_version: 1,
            workspace_id: self.workspace_id.clone(),
            lifecycle_state: self.state,
            trust_state: self.trust_state,
            watcher_health: self.watcher_health.map(|v| v.as_str().to_owned()),
            hot_index_ready: self.hot_index_ready,
            command_graph_ready: self.command_graph_ready,
            last_transition_reason_code: self.last_transition_reason_code.clone(),
            observed_at: self.observed_at.clone(),
        }
    }

    /// Drains any transition frames emitted since the last call.
    pub fn drain_transition_frames(&mut self) -> Vec<WorkspaceLifecycleTransitionFrame> {
        std::mem::take(&mut self.frames)
    }

    /// Handles the "open workspace" intent, transitioning `discovered → trust_evaluating`.
    pub fn open_workspace(&mut self, observed_at: impl Into<String>) {
        self.transition(
            WorkspaceLifecycleState::TrustEvaluating,
            observed_at,
            Some("open_workspace".to_string()),
        );
    }

    /// Updates trust posture and, when appropriate, transitions `trust_evaluating → opening`.
    pub fn resolve_trust(&mut self, trust_state: TrustState, observed_at: impl Into<String>) {
        self.trust_state = trust_state;
        let observed_at = observed_at.into();
        match self.state {
            WorkspaceLifecycleState::TrustEvaluating => self.transition(
                WorkspaceLifecycleState::Opening,
                observed_at,
                Some("trust_resolved".to_string()),
            ),
            WorkspaceLifecycleState::Ready => {
                if trust_state != TrustState::Trusted {
                    self.transition(
                        WorkspaceLifecycleState::Degraded,
                        observed_at,
                        Some("trust_downgraded".to_string()),
                    );
                } else {
                    self.observed_at = observed_at;
                }
            }
            _ => {
                self.observed_at = observed_at;
            }
        }
    }

    /// Marks the shell as interactive for the workspace, transitioning `opening → partially_ready`.
    pub fn mark_shell_interactive(&mut self, observed_at: impl Into<String>) {
        if self.state == WorkspaceLifecycleState::Opening {
            self.transition(
                WorkspaceLifecycleState::PartiallyReady,
                observed_at,
                Some("shell_interactive".to_string()),
            );
            return;
        }
        self.observed_at = observed_at.into();
    }

    /// Updates readiness gate signals and applies any derived lifecycle transitions.
    ///
    /// This method never blocks: it records the latest gate signals and applies
    /// the minimal state transitions required to stay honest about readiness.
    pub fn update_readiness_gates(
        &mut self,
        watcher_health: Option<WatcherHealth>,
        hot_index_ready: Option<bool>,
        command_graph_ready: Option<bool>,
        observed_at: impl Into<String>,
        reason_code: Option<String>,
    ) {
        if let Some(health) = watcher_health {
            self.watcher_health = Some(health);
        }
        if let Some(ready) = hot_index_ready {
            self.hot_index_ready = ready;
        }
        if let Some(ready) = command_graph_ready {
            self.command_graph_ready = ready;
        }

        let observed_at = observed_at.into();
        self.observed_at = observed_at.clone();

        let degrade = self.should_degrade();
        match self.state {
            WorkspaceLifecycleState::PartiallyReady => {
                if degrade {
                    self.transition(
                        WorkspaceLifecycleState::Degraded,
                        observed_at,
                        reason_code.or_else(|| Some("readiness_fault".to_string())),
                    );
                    return;
                }
                if self.should_be_ready() {
                    self.transition(
                        WorkspaceLifecycleState::Ready,
                        observed_at,
                        Some("readiness_full".to_string()),
                    );
                }
            }
            WorkspaceLifecycleState::Ready => {
                if degrade {
                    self.transition(
                        WorkspaceLifecycleState::Degraded,
                        observed_at,
                        reason_code.or_else(|| Some("readiness_fault".to_string())),
                    );
                }
            }
            WorkspaceLifecycleState::Degraded => {
                if !degrade {
                    self.transition(
                        WorkspaceLifecycleState::PartiallyReady,
                        observed_at,
                        Some("recovered".to_string()),
                    );
                    if self.should_be_ready() {
                        self.transition(
                            WorkspaceLifecycleState::Ready,
                            self.observed_at.clone(),
                            Some("readiness_full".to_string()),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    /// Transitions the lifecycle machine into `closing`.
    pub fn close(&mut self, observed_at: impl Into<String>) {
        match self.state {
            WorkspaceLifecycleState::Closed | WorkspaceLifecycleState::Closing => {
                self.observed_at = observed_at.into();
            }
            _ => self.transition(
                WorkspaceLifecycleState::Closing,
                observed_at,
                Some("close_requested".to_string()),
            ),
        }
    }

    /// Marks the workspace closed, transitioning `closing → closed`.
    pub fn mark_closed(&mut self, observed_at: impl Into<String>) {
        if self.state == WorkspaceLifecycleState::Closing {
            self.transition(
                WorkspaceLifecycleState::Closed,
                observed_at,
                Some("closed".to_string()),
            );
            return;
        }
        self.observed_at = observed_at.into();
    }

    fn transition(
        &mut self,
        to: WorkspaceLifecycleState,
        observed_at: impl Into<String>,
        reason_code: Option<String>,
    ) {
        if self.state == to {
            self.observed_at = observed_at.into();
            return;
        }
        let from = self.state;
        self.state = to;
        self.observed_at = observed_at.into();
        self.last_transition_reason_code = reason_code.clone();
        self.frames.push(WorkspaceLifecycleTransitionFrame {
            workspace_id: self.workspace_id.clone(),
            from_state: from,
            to_state: to,
            observed_at: self.observed_at.clone(),
            reason_code,
        });
    }

    fn should_be_ready(&self) -> bool {
        self.watcher_health == Some(WatcherHealth::Healthy)
            && self.hot_index_ready
            && self.command_graph_ready
    }

    fn should_degrade(&self) -> bool {
        if self.trust_state != TrustState::Trusted {
            return true;
        }
        match self.watcher_health {
            None => false,
            Some(WatcherHealth::Healthy) => false,
            Some(WatcherHealth::Warming) => false,
            Some(WatcherHealth::Degraded)
            | Some(WatcherHealth::FallbackPolling)
            | Some(WatcherHealth::Unavailable) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use std::path::Path;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum WatcherHealthFixture {
        Healthy,
        Warming,
        Degraded,
        FallbackPolling,
        Unavailable,
    }

    impl WatcherHealthFixture {
        const fn to_health(self) -> WatcherHealth {
            match self {
                Self::Healthy => WatcherHealth::Healthy,
                Self::Warming => WatcherHealth::Warming,
                Self::Degraded => WatcherHealth::Degraded,
                Self::FallbackPolling => WatcherHealth::FallbackPolling,
                Self::Unavailable => WatcherHealth::Unavailable,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    struct RequiredTransition {
        from: WorkspaceLifecycleState,
        to: WorkspaceLifecycleState,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    struct LifecycleCaseExpect {
        final_state: WorkspaceLifecycleState,
        final_trust_state: TrustState,
        final_watcher_health: Option<WatcherHealthFixture>,
        final_hot_index_ready: bool,
        final_command_graph_ready: bool,
        required_transitions: Vec<RequiredTransition>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    #[serde(tag = "action", rename_all = "snake_case")]
    enum LifecycleCaseStep {
        OpenWorkspace {
            observed_at: String,
        },
        ResolveTrust {
            trust_state: TrustState,
            observed_at: String,
        },
        MarkShellInteractive {
            observed_at: String,
        },
        UpdateReadinessGates {
            watcher_health: Option<WatcherHealthFixture>,
            hot_index_ready: Option<bool>,
            command_graph_ready: Option<bool>,
            observed_at: String,
            #[serde(default)]
            reason_code: Option<String>,
        },
        Close {
            observed_at: String,
        },
        MarkClosed {
            observed_at: String,
        },
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    struct LifecycleCaseFixture {
        record_kind: String,
        schema_version: u32,
        workspace_id: String,
        initial_observed_at: String,
        steps: Vec<LifecycleCaseStep>,
        expect: LifecycleCaseExpect,
    }

    #[test]
    fn transitions_through_partially_ready_before_ready() {
        let mut machine = WorkspaceLifecycleMachine::discovered("ws-test", "mono:0");
        assert_eq!(machine.state(), WorkspaceLifecycleState::Discovered);

        machine.open_workspace("mono:1");
        assert_eq!(machine.state(), WorkspaceLifecycleState::TrustEvaluating);

        machine.resolve_trust(TrustState::Trusted, "mono:2");
        assert_eq!(machine.state(), WorkspaceLifecycleState::Opening);

        machine.mark_shell_interactive("mono:3");
        assert_eq!(machine.state(), WorkspaceLifecycleState::PartiallyReady);

        machine.update_readiness_gates(
            Some(WatcherHealth::Warming),
            Some(false),
            Some(true),
            "mono:4",
            None,
        );
        assert_eq!(machine.state(), WorkspaceLifecycleState::PartiallyReady);

        machine.update_readiness_gates(
            Some(WatcherHealth::Healthy),
            Some(true),
            Some(true),
            "mono:5",
            None,
        );
        assert_eq!(machine.state(), WorkspaceLifecycleState::Ready);
    }

    #[test]
    fn degrades_on_watcher_fault_and_recovers_to_partially_ready() {
        let mut machine = WorkspaceLifecycleMachine::discovered("ws-test", "mono:0");
        machine.open_workspace("mono:1");
        machine.resolve_trust(TrustState::Trusted, "mono:2");
        machine.mark_shell_interactive("mono:3");
        machine.update_readiness_gates(
            Some(WatcherHealth::Healthy),
            Some(true),
            Some(true),
            "mono:4",
            None,
        );
        assert_eq!(machine.state(), WorkspaceLifecycleState::Ready);

        machine.update_readiness_gates(
            Some(WatcherHealth::FallbackPolling),
            None,
            None,
            "mono:5",
            Some("watcher_fault".to_string()),
        );
        assert_eq!(machine.state(), WorkspaceLifecycleState::Degraded);

        machine.update_readiness_gates(Some(WatcherHealth::Healthy), None, None, "mono:6", None);
        assert_eq!(machine.state(), WorkspaceLifecycleState::Ready);
    }

    #[test]
    fn trust_downgrade_degrades_ready_until_trust_recovers() {
        let mut machine = WorkspaceLifecycleMachine::discovered("ws-test", "mono:0");
        machine.open_workspace("mono:1");
        machine.resolve_trust(TrustState::Trusted, "mono:2");
        machine.mark_shell_interactive("mono:3");
        machine.update_readiness_gates(
            Some(WatcherHealth::Healthy),
            Some(true),
            Some(true),
            "mono:4",
            None,
        );
        assert_eq!(machine.state(), WorkspaceLifecycleState::Ready);

        machine.resolve_trust(TrustState::Restricted, "mono:5");
        assert_eq!(machine.state(), WorkspaceLifecycleState::Degraded);

        machine.resolve_trust(TrustState::Trusted, "mono:6");
        machine.update_readiness_gates(None, None, None, "mono:7", None);
        assert_eq!(machine.state(), WorkspaceLifecycleState::Ready);
    }

    #[test]
    fn lifecycle_case_fixtures_match_expected_transitions() {
        let fixtures_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/lifecycle_cases");
        let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
            .expect("fixtures dir must exist")
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        fixtures.sort();

        for path in fixtures {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let fixture: LifecycleCaseFixture =
                serde_json::from_str(&payload).expect("fixture must parse");
            assert_eq!(
                fixture.record_kind, "workspace_lifecycle_case",
                "unexpected record_kind in {path:?}"
            );
            assert_eq!(
                fixture.schema_version, 1,
                "unexpected schema_version in {path:?}"
            );

            let mut machine = WorkspaceLifecycleMachine::discovered(
                fixture.workspace_id.clone(),
                fixture.initial_observed_at.clone(),
            );
            let mut transitions: Vec<(WorkspaceLifecycleState, WorkspaceLifecycleState)> =
                Vec::new();

            for step in fixture.steps {
                match step {
                    LifecycleCaseStep::OpenWorkspace { observed_at } => {
                        machine.open_workspace(observed_at);
                    }
                    LifecycleCaseStep::ResolveTrust {
                        trust_state,
                        observed_at,
                    } => {
                        machine.resolve_trust(trust_state, observed_at);
                    }
                    LifecycleCaseStep::MarkShellInteractive { observed_at } => {
                        machine.mark_shell_interactive(observed_at);
                    }
                    LifecycleCaseStep::UpdateReadinessGates {
                        watcher_health,
                        hot_index_ready,
                        command_graph_ready,
                        observed_at,
                        reason_code,
                    } => {
                        machine.update_readiness_gates(
                            watcher_health.map(WatcherHealthFixture::to_health),
                            hot_index_ready,
                            command_graph_ready,
                            observed_at,
                            reason_code,
                        );
                    }
                    LifecycleCaseStep::Close { observed_at } => {
                        machine.close(observed_at);
                    }
                    LifecycleCaseStep::MarkClosed { observed_at } => {
                        machine.mark_closed(observed_at);
                    }
                }

                for frame in machine.drain_transition_frames() {
                    transitions.push((frame.from_state, frame.to_state));
                }
            }

            let snapshot = machine.snapshot();
            assert_eq!(
                snapshot.lifecycle_state, fixture.expect.final_state,
                "unexpected final lifecycle_state in {path:?}"
            );
            assert_eq!(
                snapshot.trust_state, fixture.expect.final_trust_state,
                "unexpected final trust_state in {path:?}"
            );
            assert_eq!(
                machine.watcher_health(),
                fixture
                    .expect
                    .final_watcher_health
                    .map(WatcherHealthFixture::to_health),
                "unexpected final watcher_health in {path:?}"
            );
            assert_eq!(
                snapshot.hot_index_ready, fixture.expect.final_hot_index_ready,
                "unexpected final hot_index_ready in {path:?}"
            );
            assert_eq!(
                snapshot.command_graph_ready, fixture.expect.final_command_graph_ready,
                "unexpected final command_graph_ready in {path:?}"
            );

            for required in fixture.expect.required_transitions {
                assert!(
                    transitions.contains(&(required.from, required.to)),
                    "missing transition {:?}->{:?} in {path:?}",
                    required.from,
                    required.to
                );
            }
        }
    }
}
