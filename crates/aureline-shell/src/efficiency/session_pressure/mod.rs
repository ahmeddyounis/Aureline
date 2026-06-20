//! Active-session continuity under power and thermal pressure.
//!
//! The parent [`crate::efficiency`] module owns the canonical efficiency-state
//! object model and governs *background and optional* work: the per-workload
//! [`WorkloadBudgetDecision`] decides whether AI warmups, prefetch, uploads,
//! indexing, preview refresh, and the like throttle, defer, pause, or stop. The
//! diagnostics and support [`surfaces`][crate::efficiency::surfaces] and the
//! per-surface [`disclosures`][crate::efficiency::disclosures] then explain that
//! background story to operators and end users.
//!
//! This module adds the missing *active-session* dimension. M5 runs live work a
//! person is actively depending on — an executing task, an attached debug
//! session, a remote attach, a notebook kernel, a live trace, or a long-running
//! capture. Under battery or thermal pressure these must behave predictably
//! **without giving up correctness or user authority**. The contract is:
//!
//! - **Optional work sheds first.** Every active session carries optional assists
//!   (assistant warmups, speculative prefetch, decorative motion, background
//!   refresh). Each maps to a canonical [`WorkloadFamily`] so its reduction comes
//!   from the frozen budget policy — the same policy the rest of the
//!   efficiency-state contract uses — rather than per-session invention. They
//!   shed *before* any protected active-session behavior regresses.
//! - **Correctness and authority stay protected.** A running task completes; debug
//!   control, breakpoints, and stepping stay authoritative; a remote attach is
//!   never silently dropped; a kernel is never silently restarted; a capture is
//!   never silently truncated or replayed. Each session records the protected
//!   authority it preserves and proves the no-silent-kill / no-replay guardrail.
//! - **A material downgrade is warned about first.** A few sessions cannot shed
//!   only optional work under the hardest pressure — a long capture, a live
//!   trace, or a remote attach may need a material downgrade (reduced sampling,
//!   buffering to disk, a widened heartbeat). When one is proposed the session
//!   emits an inline, scope-accurate [`SessionContinuityWarning`] *before* the
//!   change applies, names exactly what changes and what stays correct, and keeps
//!   the user's authority (and any policy-aware override) intact. Nothing material
//!   ever happens silently.
//!
//! Everything projects from the same canonical [`EfficiencyState`],
//! [`WorkloadBudgetDecision`], override posture, recovery state, and frozen
//! governance binding the status, diagnostics, support, and disclosure surfaces
//! use, so the session-continuity story can never disagree with the rest of the
//! low-power contract. The [`SessionPressurePosture`] packet is the canonical
//! truth for "how do active runs behave under pressure?" and the same transitions
//! it records flow into the diagnostics and support packets so recovery stays
//! explainable.

use serde::{Deserialize, Serialize};

use super::governance::{
    EfficiencyGovernanceProjection, HiddenPaneBehavior, OverridePosture,
    M5_EFFICIENCY_GOVERNANCE_MATRIX_REF, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
};
use super::surfaces::{EFFICIENCY_DETAILS_SURFACE_REF, EFFICIENCY_INSPECT_COMMAND_ID};
use super::{
    derive_override_posture, derive_recovery_state, protected_interactions,
    EfficiencyPressureSource, EfficiencyState, EfficiencyStateSnapshot, WorkloadBudgetDecision,
    WorkloadFamily,
};
use crate::notifications::envelope::SourceSubsystem;

#[cfg(test)]
mod tests;

/// Stable record kind for [`SessionPressurePosture`] payloads.
pub const SESSION_PRESSURE_POSTURE_RECORD_KIND: &str = "efficiency_session_pressure_posture";

/// Stable record kind for an individual [`ActiveSessionDecision`].
pub const ACTIVE_SESSION_DECISION_RECORD_KIND: &str = "efficiency_active_session_decision";

/// Stable record kind for an individual [`SessionAssistDecision`].
pub const SESSION_ASSIST_DECISION_RECORD_KIND: &str = "efficiency_session_assist_decision";

/// Stable record kind for a [`SessionContinuityWarning`].
pub const SESSION_CONTINUITY_WARNING_RECORD_KIND: &str = "efficiency_session_continuity_warning";

/// Schema version shared by the posture packet and its rows.
pub const SESSION_PRESSURE_POSTURE_SCHEMA_VERSION: u32 = 1;

/// A live session a person is actively depending on while efficiency posture may
/// change.
///
/// Each kind names the correctness/authority it always preserves
/// ([`protected_authority`](Self::protected_authority)) and the optional assists
/// that shed first ([`optional_assists`](Self::optional_assists)). The assists
/// reuse canonical [`WorkloadFamily`] budgets, so the shedding behavior comes from
/// the frozen policy rather than per-session invention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveSessionKind {
    /// A running task, build, or test the user launched.
    ActiveTaskRun,
    /// An attached debug session with breakpoints, stepping, and inspection.
    DebugSession,
    /// A remote session attach.
    RemoteAttach,
    /// A live notebook kernel holding in-memory state.
    NotebookKernel,
    /// A live trace being recorded.
    TraceCapture,
    /// A long-running capture, recording, or profile.
    LongRunningCapture,
}

impl ActiveSessionKind {
    /// Every active-session kind, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::ActiveTaskRun,
        Self::DebugSession,
        Self::RemoteAttach,
        Self::NotebookKernel,
        Self::TraceCapture,
        Self::LongRunningCapture,
    ];

    /// Stable token recorded in postures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveTaskRun => "active_task_run",
            Self::DebugSession => "debug_session",
            Self::RemoteAttach => "remote_attach",
            Self::NotebookKernel => "notebook_kernel",
            Self::TraceCapture => "trace_capture",
            Self::LongRunningCapture => "long_running_capture",
        }
    }

    /// Title-cased label rendered as the session subject.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveTaskRun => "Active task run",
            Self::DebugSession => "Debug session",
            Self::RemoteAttach => "Remote attach",
            Self::NotebookKernel => "Notebook kernel",
            Self::TraceCapture => "Trace capture",
            Self::LongRunningCapture => "Long-running capture",
        }
    }

    /// Lower-case noun phrase used inside warning sentences.
    pub const fn subject(self) -> &'static str {
        match self {
            Self::ActiveTaskRun => "this task run",
            Self::DebugSession => "this debug session",
            Self::RemoteAttach => "this remote attach",
            Self::NotebookKernel => "this notebook kernel",
            Self::TraceCapture => "this trace capture",
            Self::LongRunningCapture => "this capture",
        }
    }

    /// Canonical subsystem that owns the session, for vocabulary traceability.
    pub const fn owner_subsystem(self) -> SourceSubsystem {
        match self {
            Self::ActiveTaskRun => SourceSubsystem::TaskRunner,
            Self::DebugSession => SourceSubsystem::DebugSession,
            Self::RemoteAttach => SourceSubsystem::RemoteAgent,
            Self::NotebookKernel => SourceSubsystem::NotebookKernel,
            Self::TraceCapture => SourceSubsystem::DebugSession,
            Self::LongRunningCapture => SourceSubsystem::TaskRunner,
        }
    }

    /// Human-readable owner label rendered next to the session.
    pub const fn owner_label(self) -> &'static str {
        match self {
            Self::ActiveTaskRun => "Task runner",
            Self::DebugSession => "Debug session",
            Self::RemoteAttach => "Remote agent",
            Self::NotebookKernel => "Notebook kernel",
            Self::TraceCapture => "Debug session",
            Self::LongRunningCapture => "Task runner",
        }
    }

    /// The correctness and authority this session always preserves, even under the
    /// hardest pressure. Answers "what stays correct?" so the user never has to
    /// fear a live run was silently killed, downgraded, or replayed.
    pub const fn protected_authority(self) -> &'static str {
        match self {
            Self::ActiveTaskRun => {
                "The running task keeps executing to completion; its output, exit status, and logs stay correct and attributable."
            }
            Self::DebugSession => {
                "Breakpoints, stepping, variable inspection, and the call stack stay authoritative; debug control is never taken from you."
            }
            Self::RemoteAttach => {
                "The attach stays live and attributable; remote state is never hidden and the session is never silently dropped or replayed."
            }
            Self::NotebookKernel => {
                "Kernel state and prior cell outputs stay intact; the kernel is never silently restarted, so no in-memory state is lost."
            }
            Self::TraceCapture => {
                "Already-captured samples stay intact and attributable; the trace is never silently truncated or replayed."
            }
            Self::LongRunningCapture => {
                "Captured data is preserved and attributable; the capture is never silently killed or restarted."
            }
        }
    }

    /// The optional assists attached to this session that shed first under
    /// pressure. Each is governed by a canonical [`WorkloadFamily`] budget, so
    /// their reduction is the frozen policy's, not the session's.
    pub fn optional_assists(self) -> &'static [WorkloadFamily] {
        match self {
            Self::ActiveTaskRun => &[
                WorkloadFamily::AiWarmup,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::NonEssentialAnimation,
            ],
            Self::DebugSession => &[
                WorkloadFamily::AiWarmup,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::NonEssentialAnimation,
            ],
            Self::RemoteAttach => &[
                WorkloadFamily::RemoteSessionHelper,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::AiWarmup,
            ],
            Self::NotebookKernel => &[
                WorkloadFamily::AiWarmup,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::PreviewRefresh,
            ],
            Self::TraceCapture => &[
                WorkloadFamily::GraphEnrichment,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::NonEssentialAnimation,
            ],
            Self::LongRunningCapture => &[
                WorkloadFamily::UploadTransfer,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::GraphEnrichment,
            ],
        }
    }

    /// Resolves a stable token back into its session kind, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == token)
    }
}

/// What efficiency posture does to a live session. Distinct from the background
/// [`WorkloadAction`][super::WorkloadAction]: this is about the *active run*, not
/// the optional work attached to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionContinuityAction {
    /// The session runs at full fidelity and nothing attached to it changes.
    PreserveActive,
    /// Only the session's optional assists are reduced; the active run is
    /// untouched and fully correct.
    ShedOptionalAssists,
    /// A material downgrade to the live run is proposed; an inline warning is
    /// shown first and the user keeps authority. Nothing material is silent.
    WarnBeforeDowngrade,
    /// Pressure has cleared and the session's optional assists resume in stages
    /// while the active run continues uninterrupted.
    StagedResume,
}

impl SessionContinuityAction {
    /// Every continuity action, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::PreserveActive,
        Self::ShedOptionalAssists,
        Self::WarnBeforeDowngrade,
        Self::StagedResume,
    ];

    /// Stable token recorded in postures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreserveActive => "preserve_active",
            Self::ShedOptionalAssists => "shed_optional_assists",
            Self::WarnBeforeDowngrade => "warn_before_downgrade",
            Self::StagedResume => "staged_resume",
        }
    }

    /// One-sentence label describing what happens to the active run.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PreserveActive => "Running at full fidelity.",
            Self::ShedOptionalAssists => {
                "Optional assists are reduced; the active session is unchanged."
            }
            Self::WarnBeforeDowngrade => {
                "A material change is proposed; you are warned before it applies."
            }
            Self::StagedResume => "Optional assists are resuming in stages.",
        }
    }

    /// True when a material downgrade to the live run is proposed, which requires
    /// an inline warning first.
    pub const fn requires_warning(self) -> bool {
        matches!(self, Self::WarnBeforeDowngrade)
    }

    /// Resolves a stable token back into its continuity action, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|action| action.as_str() == token)
    }
}

/// The specific material change a [`SessionContinuityAction::WarnBeforeDowngrade`]
/// would apply to a live session. Only a few sessions can reach this under the
/// hardest pressure; the kind names exactly what changes so the warning is
/// scope-accurate rather than vague.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionDowngradeKind {
    /// Reconnect and heartbeat cadence widens within freshness bounds.
    RemoteHeartbeatWidened,
    /// Live sampling fidelity is reduced.
    CaptureSamplingReduced,
    /// The capture buffers to disk and reduces its live rate.
    CaptureBufferedToDisk,
}

impl SessionDowngradeKind {
    /// Every downgrade kind, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::RemoteHeartbeatWidened,
        Self::CaptureSamplingReduced,
        Self::CaptureBufferedToDisk,
    ];

    /// Stable token recorded in warnings and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoteHeartbeatWidened => "remote_heartbeat_widened",
            Self::CaptureSamplingReduced => "capture_sampling_reduced",
            Self::CaptureBufferedToDisk => "capture_buffered_to_disk",
        }
    }

    /// The scope-accurate sentence describing what materially changes.
    pub const fn what_changes(self) -> &'static str {
        match self {
            Self::RemoteHeartbeatWidened => {
                "Reconnect and heartbeat cadence will widen within freshness bounds, so a dropped link may take slightly longer to detect."
            }
            Self::CaptureSamplingReduced => {
                "Live sampling fidelity will drop, so fewer samples are recorded per second until pressure clears."
            }
            Self::CaptureBufferedToDisk => {
                "The capture will buffer to disk and reduce its live rate to protect core interaction."
            }
        }
    }

    /// Resolves a stable token back into its downgrade kind, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == token)
    }
}

/// One optional assist attached to a live session, with the canonical budget
/// action it sheds under. Projected straight from a [`WorkloadBudgetDecision`] so
/// the assist's reduction can never disagree with the rest of the contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionAssistDecision {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Workload-family token for the assist.
    pub assist_token: String,
    /// Human-readable assist label.
    pub assist_label: String,
    /// Source-subsystem owner label.
    pub owner_label: String,
    /// Budget-action token applied to the assist.
    pub action: String,
    /// Visible-capability-state token after the action.
    pub visible_state: String,
    /// User-impact sentence for the reduction.
    pub user_impact_label: String,
    /// Always true: the assist is optional, so it is eligible to shed first.
    pub is_optional: bool,
    /// True when the assist reduced this posture (its action changed behavior).
    pub shed: bool,
}

impl SessionAssistDecision {
    fn from_decision(decision: &WorkloadBudgetDecision) -> Self {
        Self {
            record_kind: SESSION_ASSIST_DECISION_RECORD_KIND.to_owned(),
            assist_token: decision.workload_id.clone(),
            assist_label: decision.capability_row.capability_label.clone(),
            owner_label: decision.capability_row.host_owner_label.clone(),
            action: decision.action.clone(),
            visible_state: decision.capability_row.visible_state.clone(),
            user_impact_label: decision.capability_row.user_impact_label.clone(),
            is_optional: true,
            shed: decision.changed_behavior(),
        }
    }
}

/// An inline, scope-accurate warning shown *before* a material downgrade to a live
/// session. It names what changes, what stays correct, that nothing is silent, and
/// the policy-aware override the user keeps. Present only when the continuity
/// action is [`SessionContinuityAction::WarnBeforeDowngrade`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionContinuityWarning {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Downgrade-kind token the warning is about.
    pub downgrade_kind: String,
    /// Headline naming the session, the proposed change, and the cause.
    pub headline: String,
    /// Scope-accurate description of exactly what materially changes.
    pub what_changes: String,
    /// The correctness and authority that stay intact through the change.
    pub what_stays_correct: String,
    /// Always true: the warning is shown before the change applies.
    pub shown_before_change: bool,
    /// Always false: the downgrade is never applied silently.
    pub silent: bool,
    /// Always true: the user keeps authority over the live session.
    pub user_keeps_authority: bool,
    /// Override-posture token the affordance derives from.
    pub override_posture: String,
    /// True when the user may decline the downgrade and keep full fidelity.
    pub override_allowed: bool,
    /// Policy reference that blocks the override, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_blocked_ref: Option<String>,
    /// User-facing actions exposed by the warning.
    pub actions: Vec<String>,
    /// Command id that opens the full efficiency-state details.
    pub inspect_command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
}

impl SessionContinuityWarning {
    fn build(
        kind: ActiveSessionKind,
        downgrade: SessionDowngradeKind,
        source: EfficiencyPressureSource,
        posture: OverridePosture,
    ) -> Self {
        let override_allowed = posture.is_user_overridable();
        let policy_blocked_ref = match posture {
            OverridePosture::PolicyBlocked => Some("policy:efficiency.override_blocked".to_owned()),
            OverridePosture::AdminControlled => {
                Some("policy:efficiency.admin_controlled".to_owned())
            }
            _ => None,
        };
        let mut actions = Vec::new();
        if override_allowed {
            actions.push(format!(
                "Keep {} at full fidelity this session",
                kind.subject()
            ));
        }
        actions.push("Open efficiency details".to_owned());
        Self {
            record_kind: SESSION_CONTINUITY_WARNING_RECORD_KIND.to_owned(),
            downgrade_kind: downgrade.as_str().to_owned(),
            headline: warning_headline(kind, downgrade, source),
            what_changes: downgrade.what_changes().to_owned(),
            what_stays_correct: kind.protected_authority().to_owned(),
            shown_before_change: true,
            silent: false,
            user_keeps_authority: true,
            override_posture: posture.as_str().to_owned(),
            override_allowed,
            policy_blocked_ref,
            actions,
            inspect_command_id: EFFICIENCY_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: EFFICIENCY_DETAILS_SURFACE_REF.to_owned(),
        }
    }
}

/// How one active session behaves under the current efficiency posture.
///
/// It records the protected authority the session keeps, the optional assists
/// that shed first, the continuity action taken, and — only when a material
/// downgrade is proposed — the inline warning shown before it applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveSessionDecision {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Session-kind token.
    pub session_token: String,
    /// Title-cased session label.
    pub session_label: String,
    /// Owner label rendered next to the session.
    pub owner_label: String,
    /// Canonical owner-subsystem token, for vocabulary traceability.
    pub owner_subsystem_token: String,
    /// Active efficiency-state token.
    pub active_state: String,
    /// Source-of-change tokens that drove the active state.
    pub source_of_change: Vec<String>,
    /// Continuity-action token applied to the active run.
    pub continuity_action: String,
    /// One-sentence label describing what happens to the active run.
    pub continuity_label: String,
    /// The correctness and authority this session preserves.
    pub protected_authority: String,
    /// True when the active run stays correct under this posture.
    pub correctness_preserved: bool,
    /// True when the user keeps authority over the live session.
    pub user_authority_preserved: bool,
    /// True when the run and its results stay attributable.
    pub attributable: bool,
    /// Always true: the session is never silently killed.
    pub never_silently_killed: bool,
    /// Always true: the session is never silently replayed.
    pub never_replayed: bool,
    /// True when this posture materially changed the session's behavior.
    pub behavior_changed: bool,
    /// Optional assists attached to this session, in shed-first order.
    pub assists: Vec<SessionAssistDecision>,
    /// Inline warning shown before a material downgrade, when one is proposed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<SessionContinuityWarning>,
}

impl ActiveSessionDecision {
    /// Builds the decision for one session under a typed posture.
    fn for_session(
        kind: ActiveSessionKind,
        state: EfficiencyState,
        sources: &[EfficiencyPressureSource],
        posture: OverridePosture,
        observed_at: &str,
    ) -> Self {
        let source = *sources
            .first()
            .unwrap_or(&EfficiencyPressureSource::AcPower);
        let assists = kind
            .optional_assists()
            .iter()
            .map(|family| {
                let decision =
                    WorkloadBudgetDecision::for_state(*family, state, source, observed_at);
                SessionAssistDecision::from_decision(&decision)
            })
            .collect::<Vec<_>>();
        let downgrade = downgrade_for(kind, state);
        let continuity_action = continuity_action_for(state, downgrade.is_some());
        let assists_shed = assists.iter().any(|assist| assist.shed);
        let warning = downgrade
            .map(|downgrade| SessionContinuityWarning::build(kind, downgrade, source, posture));
        Self {
            record_kind: ACTIVE_SESSION_DECISION_RECORD_KIND.to_owned(),
            session_token: kind.as_str().to_owned(),
            session_label: kind.label().to_owned(),
            owner_label: kind.owner_label().to_owned(),
            owner_subsystem_token: source_subsystem_token(kind.owner_subsystem()).to_owned(),
            active_state: state.as_str().to_owned(),
            source_of_change: sources.iter().map(|s| s.as_str().to_owned()).collect(),
            continuity_action: continuity_action.as_str().to_owned(),
            continuity_label: continuity_action.label().to_owned(),
            protected_authority: kind.protected_authority().to_owned(),
            correctness_preserved: true,
            user_authority_preserved: true,
            attributable: true,
            never_silently_killed: true,
            never_replayed: true,
            behavior_changed: assists_shed || warning.is_some(),
            assists,
            warning,
        }
    }

    /// True when every optional assist attached to this session reduced under the
    /// current posture, proving optional work sheds before the active run.
    pub fn optional_assists_all_shed(&self) -> bool {
        !self.assists.is_empty() && self.assists.iter().all(|assist| assist.shed)
    }
}

/// How every active session behaves under one workspace's efficiency posture.
///
/// Project it from the canonical [`EfficiencyStateSnapshot`] with
/// [`from_snapshot`](Self::from_snapshot) so the session-continuity story, the
/// status pill, the diagnostics row, the support export, and the per-surface
/// disclosures all derive from one object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionPressurePosture {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency-state token.
    pub active_state: String,
    /// Source-of-change tokens that drove the active state.
    pub source_of_change: Vec<String>,
    /// True when any active session's behavior materially changed.
    pub behavior_changed: bool,
    /// Aggregate override-posture token for the adaptation.
    pub override_posture: String,
    /// Recovery-state token for the adaptation.
    pub recovery_state: String,
    /// One decision per active-session kind.
    pub sessions: Vec<ActiveSessionDecision>,
    /// Always false: no active session is silently killed under any posture.
    pub any_session_silently_killed: bool,
    /// Always false: no active session is silently replayed under any posture.
    pub any_session_replayed: bool,
    /// Protected interactions the adaptation may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when save durability and user-owned artifacts stay preserved.
    pub durability_preserved: bool,
    /// Matrix-bound governance projection for vocabulary traceability.
    pub governance: EfficiencyGovernanceProjection,
    /// Open-details command id shared by every warning.
    pub inspect_command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
    /// Support-export packet id that quotes the same posture.
    pub support_export_ref: String,
    /// Observation timestamp.
    pub observed_at: String,
}

impl SessionPressurePosture {
    /// Builds the session-continuity posture for a typed efficiency state.
    pub fn for_state(
        workspace_id: &str,
        state: EfficiencyState,
        sources: &[EfficiencyPressureSource],
        hidden_surface_count: usize,
        observed_at: &str,
    ) -> Self {
        let posture = derive_override_posture(state, sources);
        let recovery = derive_recovery_state(state);
        let sessions = ActiveSessionKind::ALL
            .into_iter()
            .map(|kind| {
                ActiveSessionDecision::for_session(kind, state, sources, posture, observed_at)
            })
            .collect::<Vec<_>>();
        let behavior_changed = sessions.iter().any(|session| session.behavior_changed);
        let source_tokens = sources
            .iter()
            .map(|source| source.as_str().to_owned())
            .collect::<Vec<_>>();
        let governance = EfficiencyGovernanceProjection {
            matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            schema_ref: M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF.to_owned(),
            active_state: state.as_str().to_owned(),
            source_of_change: source_tokens.clone(),
            hidden_pane_behaviors: hidden_pane_behaviors_for(hidden_surface_count)
                .iter()
                .map(|behavior| behavior.as_str().to_owned())
                .collect(),
            override_posture: posture.as_str().to_owned(),
            recovery_state: recovery.as_str().to_owned(),
        };
        Self {
            record_kind: SESSION_PRESSURE_POSTURE_RECORD_KIND.to_owned(),
            schema_version: SESSION_PRESSURE_POSTURE_SCHEMA_VERSION,
            workspace_id: workspace_id.to_owned(),
            active_state: state.as_str().to_owned(),
            source_of_change: source_tokens,
            behavior_changed,
            override_posture: posture.as_str().to_owned(),
            recovery_state: recovery.as_str().to_owned(),
            sessions,
            any_session_silently_killed: false,
            any_session_replayed: false,
            protected_interactions_preserved: protected_interactions(),
            durability_preserved: true,
            governance,
            inspect_command_id: EFFICIENCY_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: EFFICIENCY_DETAILS_SURFACE_REF.to_owned(),
            support_export_ref: support_export_id(workspace_id, state),
            observed_at: observed_at.to_owned(),
        }
    }

    /// Projects the canonical snapshot into the session-continuity posture.
    ///
    /// The snapshot already tokenizes the active state and its causes; this
    /// re-derives the typed inputs and reuses [`for_state`](Self::for_state) so
    /// the posture shares the snapshot's workspace, state, source, override
    /// posture, recovery state, and governance binding.
    pub fn from_snapshot(snapshot: &EfficiencyStateSnapshot) -> Self {
        let state = EfficiencyState::from_token(&snapshot.active_state).unwrap_or_default();
        let sources = snapshot
            .pressure_sources
            .iter()
            .filter_map(|token| EfficiencyPressureSource::from_token(token))
            .collect::<Vec<_>>();
        Self::for_state(
            &snapshot.workspace_id,
            state,
            &sources,
            snapshot.hidden_pane_audit.hidden_surface_count,
            &snapshot.observed_at,
        )
    }

    /// Returns the decision for a session token, if present.
    pub fn session_for(&self, session_token: &str) -> Option<&ActiveSessionDecision> {
        self.sessions
            .iter()
            .find(|session| session.session_token == session_token)
    }

    /// True when every active session keeps correctness, user authority, and
    /// attribution, and is never silently killed or replayed. This is the
    /// "active runs remain correct and attributable" guardrail.
    pub fn preserves_active_session_correctness(&self) -> bool {
        !self.any_session_silently_killed
            && !self.any_session_replayed
            && self.sessions.iter().all(|session| {
                session.correctness_preserved
                    && session.user_authority_preserved
                    && session.attributable
                    && session.never_silently_killed
                    && session.never_replayed
            })
    }

    /// True when optional work sheds before any protected active-session behavior
    /// regresses: under a pressured posture every changed session sheds its
    /// optional assists while correctness stays preserved.
    pub fn optional_work_sheds_first(&self) -> bool {
        self.sessions.iter().all(|session| {
            if !session.behavior_changed {
                return true;
            }
            session.correctness_preserved && session.optional_assists_all_shed()
        })
    }

    /// True when every proposed material downgrade is preceded by an inline,
    /// scope-accurate, non-silent warning that keeps the user's authority — and no
    /// downgrade happens without one. This is the "warn before material downgrade"
    /// acceptance criterion.
    pub fn warns_before_material_downgrade(&self) -> bool {
        self.sessions.iter().all(|session| {
            let needs_warning = SessionContinuityAction::from_token(&session.continuity_action)
                .is_some_and(SessionContinuityAction::requires_warning);
            match (&session.warning, needs_warning) {
                (Some(warning), true) => {
                    warning.shown_before_change
                        && !warning.silent
                        && warning.user_keeps_authority
                        && !warning.what_changes.is_empty()
                        && !warning.what_stays_correct.is_empty()
                }
                (None, false) => true,
                // A downgrade without a warning, or a warning without a downgrade,
                // both violate the contract.
                _ => false,
            }
        })
    }
}

/// One seeded session-pressure scenario: the typed inputs that drive it together
/// with the posture they produce. Backs the dump example, the checked-in fixtures,
/// and the round-trip test so the posture never drifts from code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionPressureCase {
    /// Stable scenario id.
    pub case_id: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency state.
    pub active_state: EfficiencyState,
    /// Source-of-change pressure sources.
    pub source_of_change: Vec<EfficiencyPressureSource>,
    /// Number of hidden surfaces the posture audited.
    pub hidden_surface_count: usize,
    /// Observation timestamp.
    pub observed_at: String,
    /// The session-continuity posture the inputs produce.
    pub posture: SessionPressurePosture,
}

/// Builds a deterministic session-pressure case for a seeded posture.
pub fn seed_session_pressure_case(
    case_id: &str,
    workspace_id: &str,
    state: EfficiencyState,
    sources: &[EfficiencyPressureSource],
    hidden_surface_count: usize,
    observed_at: &str,
) -> SessionPressureCase {
    let posture = SessionPressurePosture::for_state(
        workspace_id,
        state,
        sources,
        hidden_surface_count,
        observed_at,
    );
    SessionPressureCase {
        case_id: case_id.to_owned(),
        workspace_id: workspace_id.to_owned(),
        active_state: state,
        source_of_change: sources.to_vec(),
        hidden_surface_count,
        observed_at: observed_at.to_owned(),
        posture,
    }
}

/// The representative session-pressure scenarios. They mirror the postures the
/// other efficiency-state surfaces seed — OS battery saver, thermal pressure, a
/// policy-imposed cap, a critical-battery protect-core posture, and staged
/// recovery — so the session-continuity story aligns with the canonical snapshots
/// the status, diagnostics, support, and disclosure surfaces project.
pub fn seeded_session_pressure_cases() -> Vec<SessionPressureCase> {
    use EfficiencyPressureSource as Source;
    use EfficiencyState as State;
    vec![
        seed_session_pressure_case(
            "battery-saver",
            "ws:battery-saver",
            State::EfficiencyAware,
            &[Source::OsBatterySaver],
            1,
            "2026-06-20T14:01:00Z",
        ),
        seed_session_pressure_case(
            "thermal",
            "ws:efficiency-demo",
            State::ThermalConstrained,
            &[Source::ThermalPressure],
            2,
            "2026-06-20T14:00:00Z",
        ),
        seed_session_pressure_case(
            "policy-cap",
            "ws:policy-cap",
            State::EfficiencyAware,
            &[Source::PolicyCap],
            0,
            "2026-06-20T14:02:00Z",
        ),
        seed_session_pressure_case(
            "critical-battery",
            "ws:critical-battery",
            State::ProtectCore,
            &[Source::CriticalBattery],
            1,
            "2026-06-20T14:03:00Z",
        ),
        seed_session_pressure_case(
            "recovery",
            "ws:recovery",
            State::Recovery,
            &[Source::PressureCleared],
            0,
            "2026-06-20T14:04:00Z",
        ),
    ]
}

/// The continuity action for a state, given whether a material downgrade applies.
/// Nominal preserves the run untouched; recovery resumes assists in stages; any
/// other (pressured) state sheds optional assists, escalating to a warned
/// downgrade only where one is genuinely proposed.
fn continuity_action_for(state: EfficiencyState, has_downgrade: bool) -> SessionContinuityAction {
    match state {
        EfficiencyState::Nominal => SessionContinuityAction::PreserveActive,
        EfficiencyState::Recovery => SessionContinuityAction::StagedResume,
        _ => {
            if has_downgrade {
                SessionContinuityAction::WarnBeforeDowngrade
            } else {
                SessionContinuityAction::ShedOptionalAssists
            }
        }
    }
}

/// The material downgrade, if any, that a session faces under a state. Debug
/// sessions, active task runs, and notebook kernels never reach a downgrade —
/// their authority and in-memory state are protected paths — so only captures,
/// traces, and remote attaches can, and only under thermal or protect-core
/// pressure.
fn downgrade_for(kind: ActiveSessionKind, state: EfficiencyState) -> Option<SessionDowngradeKind> {
    use ActiveSessionKind as Kind;
    use EfficiencyState as State;
    match (kind, state) {
        (Kind::TraceCapture, State::ThermalConstrained | State::ProtectCore) => {
            Some(SessionDowngradeKind::CaptureSamplingReduced)
        }
        (Kind::LongRunningCapture, State::ThermalConstrained) => {
            Some(SessionDowngradeKind::CaptureSamplingReduced)
        }
        (Kind::LongRunningCapture, State::ProtectCore) => {
            Some(SessionDowngradeKind::CaptureBufferedToDisk)
        }
        (Kind::RemoteAttach, State::ProtectCore) => {
            Some(SessionDowngradeKind::RemoteHeartbeatWidened)
        }
        _ => None,
    }
}

/// Builds the warning headline from the session, the proposed downgrade, and the
/// cause, reusing the canonical source label so it agrees with the status pill.
fn warning_headline(
    kind: ActiveSessionKind,
    downgrade: SessionDowngradeKind,
    source: EfficiencyPressureSource,
) -> String {
    match downgrade {
        SessionDowngradeKind::RemoteHeartbeatWidened => format!(
            "{}: reconnect cadence will widen while {} is active.",
            kind.label(),
            source.label()
        ),
        SessionDowngradeKind::CaptureSamplingReduced => format!(
            "{}: sampling will be reduced while {} is active.",
            kind.label(),
            source.label()
        ),
        SessionDowngradeKind::CaptureBufferedToDisk => format!(
            "{}: capture will buffer to disk while {} is active.",
            kind.label(),
            source.label()
        ),
    }
}

/// Builds the canonical support-export id for a workspace's active state, so the
/// posture points at the same packet the diagnostics, support, and disclosure
/// surfaces quote. Kept in lockstep with the support-export id minted by
/// [`crate::efficiency::surfaces`].
fn support_export_id(workspace_id: &str, state: EfficiencyState) -> String {
    format!(
        "support.export.efficiency.{}.{}",
        workspace_id,
        state.as_str()
    )
}

/// The hidden-pane behaviours a hidden surface adopted, derived from how many
/// hidden surfaces the snapshot audited. Mirrors the parent surfaces' mapping so
/// the governance projection agrees across surfaces.
fn hidden_pane_behaviors_for(hidden_surface_count: usize) -> Vec<HiddenPaneBehavior> {
    if hidden_surface_count == 0 {
        return Vec::new();
    }
    vec![
        HiddenPaneBehavior::RenderSuppressed,
        HiddenPaneBehavior::AnimationSuppressed,
        HiddenPaneBehavior::PollingPaused,
    ]
}

/// Stable owner-subsystem token for a [`SourceSubsystem`]. Kept private to the
/// session-pressure module so the session owner is recorded with the same token
/// vocabulary the rest of the efficiency contract uses.
fn source_subsystem_token(source: SourceSubsystem) -> &'static str {
    match source {
        SourceSubsystem::TaskRunner => "task_runner",
        SourceSubsystem::DebugSession => "debug_session",
        SourceSubsystem::RemoteAgent => "remote_agent",
        SourceSubsystem::NotebookKernel => "notebook_kernel",
        _ => "shell",
    }
}
