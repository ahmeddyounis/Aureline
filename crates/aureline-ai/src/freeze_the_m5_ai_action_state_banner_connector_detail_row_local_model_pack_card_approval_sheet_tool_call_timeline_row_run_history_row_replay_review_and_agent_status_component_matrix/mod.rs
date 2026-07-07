//! Frozen M5 AI action-state-banner, connector-detail-row, local-model-pack-card,
//! high-friction approval-sheet, tool-call-timeline-row, AI run-history-row,
//! replay/rerun-review, and agent-status component matrix.
//!
//! This module locks Aureline's reusable AI execution and replay components into
//! one export-safe packet. Every AI component family M5 claims that still drifts
//! too easily by assistant panel, patch-review sheet, branch-agent console,
//! connector admin, model manager, run-history list, or replay-review surface —
//! the action-state banner, the connector/tool-server detail row, the local model
//! pack card, the high-friction approval sheet, the tool-call timeline row, the AI
//! run-history row, the rerun/replay-review sheet, and the agent-status card (paused,
//! blocked, awaiting-takeover, handed-off, or lineage) — is named once here and
//! constrained by the same execution-mode, route/provider, tool-boundary,
//! auth-posture, approval-gate, checkpoint-lineage, replay-completeness,
//! rerun-review-reason, and manual-takeover-path rules regardless of the surface
//! family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the AI action states, the execution modes,
//! the connector capability classes and auth postures, the local-model pack states,
//! the approval gates and friction reasons, the tool boundaries and side-effect
//! classes, the run outcomes, the replay-completeness states and rerun-review
//! reasons, the agent lifecycle states and manual-takeover paths, the deployment
//! lines every component must survive, the non-visual accessibility routes, and the
//! mandatory labels every component must be able to show. It does not re-architect
//! AI execution policy, evidence storage, or route selection that already own those
//! records — it is the shared component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 AI, review,
//! branch-agent, connector, model, run-history, or replay surface may publish a
//! mode, route, boundary, approval, replay-completeness, drift, or takeover claim.
//! Assistant, patch-review, branch-agent-console, connector-admin, model-manager,
//! run-history, replay-review, and support surfaces all consume this packet so one
//! action-state banner names the mode it is running, one connector row names its
//! capability class and auth posture, one local-model card names its pack state and
//! provenance, one approval sheet names its gate and friction, one tool-call row
//! names where the tool ran and its side effect, one run-history row names its
//! outcome and route, one replay-review sheet names how complete the replay is and
//! why a rerun requires re-review, and one agent-status card names its lifecycle
//! state and manual-takeover path. No M5 lane invents a second AI-status grammar,
//! masks an execution mode or route, overstates replay completeness, or hides an
//! approval gate or manual-takeover path.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5AiExecutionComponentVocabularySet`] rather than minted per surface. Raw URLs,
//! raw tokens, credentials, private endpoints, and user text bodies stay outside the
//! support boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_execution_replay_component_matrix,
    seeded_m5_ai_execution_replay_component_matrix_agent_status_preview_narrowed,
    seeded_m5_ai_execution_replay_component_matrix_replay_review_beta_narrowed,
    M5_AI_EXECUTION_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiExecutionComponentMatrixPacket`].
pub const M5_AI_EXECUTION_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_high_friction_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix";

/// Schema version for M5 AI-execution/replay-component-matrix records.
pub const M5_AI_EXECUTION_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI-execution/replay-components boundary schema.
pub const M5_AI_EXECUTION_COMPONENT_SCHEMA_REF: &str =
    "schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_EXECUTION_COMPONENT_DOC_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md";

/// Repo-relative path of the tool-call-timeline-entry contract this matrix binds
/// against.
pub const M5_AI_EXECUTION_COMPONENT_TOOL_REF: &str =
    "schemas/ai/tool_call_timeline_entry.schema.json";

/// Repo-relative path of the AI-run-history-entry contract this matrix binds
/// against.
pub const M5_AI_EXECUTION_COMPONENT_RUN_HISTORY_REF: &str =
    "schemas/ai/ai_run_history_entry.schema.json";

/// Repo-relative path of the evidence-replay-packet contract this matrix binds
/// against.
pub const M5_AI_EXECUTION_COMPONENT_REPLAY_REF: &str =
    "schemas/ai/evidence_replay_packet.schema.json";

/// Repo-relative path of the branch-agent-session contract this matrix binds
/// against.
pub const M5_AI_EXECUTION_COMPONENT_AGENT_REF: &str = "schemas/ai/branch_agent_session.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_EXECUTION_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_EXECUTION_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_EXECUTION_COMPONENT_CSV_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_EXECUTION_COMPONENT_REPORT_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md";

/// One of the eight governed AI-execution/replay component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionComponentFamily {
    /// An AI action-state banner carrying the active mode and action state.
    AiActionStateBanner,
    /// A connector / tool-server detail row carrying capability class and auth
    /// posture.
    ConnectorDetailRow,
    /// A local model pack card carrying its pack state and provenance.
    LocalModelPackCard,
    /// A high-friction approval sheet carrying its approval gate and friction
    /// reason.
    ApprovalSheet,
    /// A tool-call timeline row carrying where the tool ran and its side effect.
    ToolCallTimelineRow,
    /// An AI run-history row carrying its outcome and route.
    RunHistoryRow,
    /// A replay / rerun-review sheet carrying replay completeness and rerun-review
    /// reason.
    ReplayReview,
    /// An agent-status card carrying its lifecycle state and manual-takeover path.
    AgentStatus,
}

impl M5AiExecutionComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AiActionStateBanner,
        Self::ConnectorDetailRow,
        Self::LocalModelPackCard,
        Self::ApprovalSheet,
        Self::ToolCallTimelineRow,
        Self::RunHistoryRow,
        Self::ReplayReview,
        Self::AgentStatus,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiActionStateBanner => "ai_action_state_banner",
            Self::ConnectorDetailRow => "connector_detail_row",
            Self::LocalModelPackCard => "local_model_pack_card",
            Self::ApprovalSheet => "approval_sheet",
            Self::ToolCallTimelineRow => "tool_call_timeline_row",
            Self::RunHistoryRow => "run_history_row",
            Self::ReplayReview => "replay_review",
            Self::AgentStatus => "agent_status",
        }
    }

    /// `true` when this family is an action-state banner and must therefore declare
    /// its action states and execution modes.
    pub const fn is_action_state_banner(self) -> bool {
        matches!(self, Self::AiActionStateBanner)
    }

    /// `true` when this family is a connector detail row and must therefore declare
    /// its connector capabilities and auth postures.
    pub const fn is_connector_row(self) -> bool {
        matches!(self, Self::ConnectorDetailRow)
    }

    /// `true` when this family is a local model pack card and must therefore declare
    /// its pack states.
    pub const fn is_local_model_card(self) -> bool {
        matches!(self, Self::LocalModelPackCard)
    }

    /// `true` when this family is an approval sheet and must therefore declare its
    /// approval gates and friction reasons.
    pub const fn is_approval_sheet(self) -> bool {
        matches!(self, Self::ApprovalSheet)
    }

    /// `true` when this family is a tool-call timeline row and must therefore
    /// declare its tool boundaries and side-effect classes.
    pub const fn is_tool_call_row(self) -> bool {
        matches!(self, Self::ToolCallTimelineRow)
    }

    /// `true` when this family is a run-history row and must therefore declare its
    /// run outcomes.
    pub const fn is_run_history_row(self) -> bool {
        matches!(self, Self::RunHistoryRow)
    }

    /// `true` when this family is a replay/rerun-review sheet and must therefore
    /// declare its replay-completeness states and rerun-review reasons.
    pub const fn is_replay_review(self) -> bool {
        matches!(self, Self::ReplayReview)
    }

    /// `true` when this family is an agent-status card and must therefore declare
    /// its agent lifecycle states and manual-takeover paths.
    pub const fn is_agent_status(self) -> bool {
        matches!(self, Self::AgentStatus)
    }
}

/// Controlled AI action state — what an AI action-state banner is currently showing,
/// so a banner never leaves the live execution state implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiActionState {
    /// Idle, no action in flight.
    Idle,
    /// Composing a request or plan.
    Composing,
    /// Streaming a response.
    Streaming,
    /// Running a tool call.
    ToolRunning,
    /// Awaiting a human approval.
    AwaitingApproval,
    /// Paused mid-run.
    Paused,
    /// Blocked at an execution boundary.
    BoundaryBlocked,
    /// Completed.
    Completed,
    /// Failed.
    Failed,
}

impl M5AiActionState {
    /// Every action state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Idle,
        Self::Composing,
        Self::Streaming,
        Self::ToolRunning,
        Self::AwaitingApproval,
        Self::Paused,
        Self::BoundaryBlocked,
        Self::Completed,
        Self::Failed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Composing => "composing",
            Self::Streaming => "streaming",
            Self::ToolRunning => "tool_running",
            Self::AwaitingApproval => "awaiting_approval",
            Self::Paused => "paused",
            Self::BoundaryBlocked => "boundary_blocked",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

/// Controlled AI execution mode — which mode an AI action is running in, so a banner
/// or history row never leaves the mode ambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionMode {
    /// Foreground assistant mode.
    ForegroundAssistant,
    /// Guided-patch mode.
    GuidedPatch,
    /// Background branch / worktree agent mode.
    BackgroundBranchAgent,
    /// Review-first placement mode.
    ReviewFirstPlacement,
    /// Headless / automation mode.
    HeadlessAutomation,
}

impl M5AiExecutionMode {
    /// Every execution mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ForegroundAssistant,
        Self::GuidedPatch,
        Self::BackgroundBranchAgent,
        Self::ReviewFirstPlacement,
        Self::HeadlessAutomation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForegroundAssistant => "foreground_assistant",
            Self::GuidedPatch => "guided_patch",
            Self::BackgroundBranchAgent => "background_branch_agent",
            Self::ReviewFirstPlacement => "review_first_placement",
            Self::HeadlessAutomation => "headless_automation",
        }
    }
}

/// Controlled connector capability class — what an external connector / tool server
/// can do, so a connector detail row never leaves its capability class implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConnectorCapability {
    /// Read-only query.
    ReadOnlyQuery,
    /// File mutation.
    FileMutation,
    /// Network egress.
    NetworkEgress,
    /// Shell execution.
    ShellExecution,
    /// External service call.
    ExternalServiceCall,
    /// Credential-scoped access.
    CredentialScoped,
}

impl M5AiConnectorCapability {
    /// Every connector capability, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadOnlyQuery,
        Self::FileMutation,
        Self::NetworkEgress,
        Self::ShellExecution,
        Self::ExternalServiceCall,
        Self::CredentialScoped,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyQuery => "read_only_query",
            Self::FileMutation => "file_mutation",
            Self::NetworkEgress => "network_egress",
            Self::ShellExecution => "shell_execution",
            Self::ExternalServiceCall => "external_service_call",
            Self::CredentialScoped => "credential_scoped",
        }
    }
}

/// Controlled auth posture — how a connector authenticates, so a connector detail
/// row never masks whether the connector is delegated, managed, byok, or
/// unauthenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAuthPosture {
    /// OAuth-delegated.
    OauthDelegated,
    /// Managed credential.
    ManagedCredential,
    /// Bring-your-own-key, scoped.
    ByokScoped,
    /// Service account.
    ServiceAccount,
    /// Token-scoped access.
    TokenScoped,
    /// Unauthenticated.
    Unauthenticated,
}

impl M5AiAuthPosture {
    /// Every auth posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OauthDelegated,
        Self::ManagedCredential,
        Self::ByokScoped,
        Self::ServiceAccount,
        Self::TokenScoped,
        Self::Unauthenticated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OauthDelegated => "oauth_delegated",
            Self::ManagedCredential => "managed_credential",
            Self::ByokScoped => "byok_scoped",
            Self::ServiceAccount => "service_account",
            Self::TokenScoped => "token_scoped",
            Self::Unauthenticated => "unauthenticated",
        }
    }
}

/// Controlled local-model pack state — the lifecycle posture of a local model pack,
/// so a local model pack card never shows a quarantined, hardware-unfit, or
/// provenance-unverified pack as freely ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelPackState {
    /// Installed and ready.
    Installed,
    /// Served from a mirror.
    Mirrored,
    /// Available offline only.
    OfflineOnly,
    /// Quarantined pending review.
    Quarantined,
    /// Hardware fit check failed.
    HardwareUnfit,
    /// An update is available.
    UpdateAvailable,
    /// Provenance is unverified.
    ProvenanceUnverified,
}

impl M5AiModelPackState {
    /// Every model pack state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Installed,
        Self::Mirrored,
        Self::OfflineOnly,
        Self::Quarantined,
        Self::HardwareUnfit,
        Self::UpdateAvailable,
        Self::ProvenanceUnverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Mirrored => "mirrored",
            Self::OfflineOnly => "offline_only",
            Self::Quarantined => "quarantined",
            Self::HardwareUnfit => "hardware_unfit",
            Self::UpdateAvailable => "update_available",
            Self::ProvenanceUnverified => "provenance_unverified",
        }
    }
}

/// Controlled approval gate — the friction class of an approval sheet, so an
/// approval sheet never presents a high-friction or blocked action as auto-approved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalGate {
    /// Auto-approved under policy.
    AutoApproved,
    /// Notify-only.
    NotifyOnly,
    /// One-click confirm.
    OneClickConfirm,
    /// High-friction typed confirmation.
    HighFrictionTyped,
    /// Two-person review.
    TwoPersonReview,
    /// Blocked by policy.
    PolicyBlocked,
}

impl M5AiApprovalGate {
    /// Every approval gate, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AutoApproved,
        Self::NotifyOnly,
        Self::OneClickConfirm,
        Self::HighFrictionTyped,
        Self::TwoPersonReview,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoApproved => "auto_approved",
            Self::NotifyOnly => "notify_only",
            Self::OneClickConfirm => "one_click_confirm",
            Self::HighFrictionTyped => "high_friction_typed",
            Self::TwoPersonReview => "two_person_review",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Controlled friction reason — why an approval carries friction, so an approval
/// sheet never hides why a confirmation is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiFrictionReason {
    /// An irreversible side effect.
    IrreversibleSideEffect,
    /// External network egress.
    ExternalNetworkEgress,
    /// Credential access.
    CredentialAccess,
    /// Cross-tenant scope.
    CrossTenantScope,
    /// A destructive file change.
    DestructiveFileChange,
    /// A policy-mandated review.
    PolicyMandatedReview,
}

impl M5AiFrictionReason {
    /// Every friction reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IrreversibleSideEffect,
        Self::ExternalNetworkEgress,
        Self::CredentialAccess,
        Self::CrossTenantScope,
        Self::DestructiveFileChange,
        Self::PolicyMandatedReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IrreversibleSideEffect => "irreversible_side_effect",
            Self::ExternalNetworkEgress => "external_network_egress",
            Self::CredentialAccess => "credential_access",
            Self::CrossTenantScope => "cross_tenant_scope",
            Self::DestructiveFileChange => "destructive_file_change",
            Self::PolicyMandatedReview => "policy_mandated_review",
        }
    }
}

/// Controlled tool boundary — where a tool call actually ran, so a tool-call
/// timeline row never leaves the execution boundary implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiToolBoundary {
    /// In-process.
    InProcess,
    /// A local sandbox.
    LocalSandbox,
    /// A local shell.
    LocalShell,
    /// A remote connector.
    RemoteConnector,
    /// An external service.
    ExternalService,
    /// Host-delegated.
    HostDelegated,
}

impl M5AiToolBoundary {
    /// Every tool boundary, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InProcess,
        Self::LocalSandbox,
        Self::LocalShell,
        Self::RemoteConnector,
        Self::ExternalService,
        Self::HostDelegated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcess => "in_process",
            Self::LocalSandbox => "local_sandbox",
            Self::LocalShell => "local_shell",
            Self::RemoteConnector => "remote_connector",
            Self::ExternalService => "external_service",
            Self::HostDelegated => "host_delegated",
        }
    }
}

/// Controlled side-effect class — what a tool call changed, so a tool-call timeline
/// row never shows a destructive or state-mutating call as read-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSideEffectClass {
    /// Read-only.
    ReadOnly,
    /// A file write.
    FileWrite,
    /// A network call.
    NetworkCall,
    /// A process spawn.
    ProcessSpawn,
    /// A state mutation.
    StateMutation,
    /// A destructive change.
    Destructive,
}

impl M5AiSideEffectClass {
    /// Every side-effect class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadOnly,
        Self::FileWrite,
        Self::NetworkCall,
        Self::ProcessSpawn,
        Self::StateMutation,
        Self::Destructive,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::FileWrite => "file_write",
            Self::NetworkCall => "network_call",
            Self::ProcessSpawn => "process_spawn",
            Self::StateMutation => "state_mutation",
            Self::Destructive => "destructive",
        }
    }
}

/// Controlled run outcome — how a recorded AI run ended, so a run-history row never
/// leaves the outcome ambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRunOutcome {
    /// Succeeded.
    Succeeded,
    /// Failed.
    Failed,
    /// Cancelled.
    Cancelled,
    /// Superseded by a newer run.
    Superseded,
    /// Partially applied.
    PartiallyApplied,
    /// Awaiting review.
    AwaitingReview,
}

impl M5AiRunOutcome {
    /// Every run outcome, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Succeeded,
        Self::Failed,
        Self::Cancelled,
        Self::Superseded,
        Self::PartiallyApplied,
        Self::AwaitingReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Superseded => "superseded",
            Self::PartiallyApplied => "partially_applied",
            Self::AwaitingReview => "awaiting_review",
        }
    }
}

/// Controlled replay-completeness state — how completely an AI run can be replayed,
/// so a replay-review sheet never shows a partial or non-deterministic replay as
/// fully replayable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReplayCompleteness {
    /// Fully replayable.
    FullyReplayable,
    /// Partially replayable.
    PartiallyReplayable,
    /// Incomplete replay.
    IncompleteReplay,
    /// Non-deterministic.
    NonDeterministic,
    /// Missing inputs.
    MissingInputs,
    /// The provider drifted since the run.
    ProviderDrifted,
}

impl M5AiReplayCompleteness {
    /// Every replay-completeness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyReplayable,
        Self::PartiallyReplayable,
        Self::IncompleteReplay,
        Self::NonDeterministic,
        Self::MissingInputs,
        Self::ProviderDrifted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReplayable => "fully_replayable",
            Self::PartiallyReplayable => "partially_replayable",
            Self::IncompleteReplay => "incomplete_replay",
            Self::NonDeterministic => "non_deterministic",
            Self::MissingInputs => "missing_inputs",
            Self::ProviderDrifted => "provider_drifted",
        }
    }
}

/// Controlled rerun-review reason — why a rerun requires re-review, so a
/// replay-review sheet never silently reruns a drifted context as if nothing
/// changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRerunReviewReason {
    /// The model version changed.
    ModelVersionChanged,
    /// A tool contract changed.
    ToolContractChanged,
    /// The input context changed.
    InputContextChanged,
    /// The route or provider changed.
    RouteOrProviderChanged,
    /// Policy changed.
    PolicyChanged,
    /// No re-review is required.
    NoReReviewRequired,
}

impl M5AiRerunReviewReason {
    /// Every rerun-review reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ModelVersionChanged,
        Self::ToolContractChanged,
        Self::InputContextChanged,
        Self::RouteOrProviderChanged,
        Self::PolicyChanged,
        Self::NoReReviewRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelVersionChanged => "model_version_changed",
            Self::ToolContractChanged => "tool_contract_changed",
            Self::InputContextChanged => "input_context_changed",
            Self::RouteOrProviderChanged => "route_or_provider_changed",
            Self::PolicyChanged => "policy_changed",
            Self::NoReReviewRequired => "no_re_review_required",
        }
    }
}

/// Controlled agent lifecycle state — the state of a branch / worktree agent, so an
/// agent-status card never shows a blocked or awaiting-takeover agent as running
/// clean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAgentLifecycleState {
    /// Running.
    Running,
    /// Paused.
    Paused,
    /// Blocked on an approval.
    BlockedOnApproval,
    /// Awaiting a manual takeover.
    AwaitingTakeover,
    /// Handed off to a human.
    HandedOff,
    /// Completed.
    Completed,
    /// Abandoned.
    Abandoned,
}

impl M5AiAgentLifecycleState {
    /// Every agent lifecycle state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Running,
        Self::Paused,
        Self::BlockedOnApproval,
        Self::AwaitingTakeover,
        Self::HandedOff,
        Self::Completed,
        Self::Abandoned,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Paused => "paused",
            Self::BlockedOnApproval => "blocked_on_approval",
            Self::AwaitingTakeover => "awaiting_takeover",
            Self::HandedOff => "handed_off",
            Self::Completed => "completed",
            Self::Abandoned => "abandoned",
        }
    }
}

/// Controlled manual-takeover path — how a user can take over an interrupted agent,
/// so an agent-status card never leaves an interrupted agent without a safe
/// takeover path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiTakeoverPath {
    /// Resume in place.
    ResumeInPlace,
    /// Take over locally.
    TakeOverLocally,
    /// Hand off via branch review.
    BranchReviewHandoff,
    /// Abort with a checkpoint.
    AbortWithCheckpoint,
    /// Escalate to the owner.
    EscalateToOwner,
    /// No takeover is possible.
    NoTakeoverPossible,
}

impl M5AiTakeoverPath {
    /// Every takeover path, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResumeInPlace,
        Self::TakeOverLocally,
        Self::BranchReviewHandoff,
        Self::AbortWithCheckpoint,
        Self::EscalateToOwner,
        Self::NoTakeoverPossible,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumeInPlace => "resume_in_place",
            Self::TakeOverLocally => "take_over_locally",
            Self::BranchReviewHandoff => "branch_review_handoff",
            Self::AbortWithCheckpoint => "abort_with_checkpoint",
            Self::EscalateToOwner => "escalate_to_owner",
            Self::NoTakeoverPossible => "no_takeover_possible",
        }
    }
}

/// Claimed M5 AI surface family that renders / consumes an AI-execution/replay
/// component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSurfaceFamily {
    /// The assistant panel surface.
    AssistantPanel,
    /// The patch-review surface.
    PatchReview,
    /// The branch-agent console surface.
    BranchAgentConsole,
    /// The connector-admin surface.
    ConnectorAdmin,
    /// The model-manager surface.
    ModelManager,
    /// The run-history surface.
    RunHistory,
    /// The replay-review surface.
    ReplayReview,
    /// The support-desk surface.
    SupportDesk,
}

impl M5AiSurfaceFamily {
    /// Every AI surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AssistantPanel,
        Self::PatchReview,
        Self::BranchAgentConsole,
        Self::ConnectorAdmin,
        Self::ModelManager,
        Self::RunHistory,
        Self::ReplayReview,
        Self::SupportDesk,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssistantPanel => "assistant_panel",
            Self::PatchReview => "patch_review",
            Self::BranchAgentConsole => "branch_agent_console",
            Self::ConnectorAdmin => "connector_admin",
            Self::ModelManager => "model_manager",
            Self::RunHistory => "run_history",
            Self::ReplayReview => "replay_review",
            Self::SupportDesk => "support_desk",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// mode, route, or boundary never silently narrows or widens between deployment
/// shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5AiDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// AI subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConsumerSurface {
    /// The assistant-panel UI.
    AssistantPanelUi,
    /// The patch-review UI.
    PatchReviewUi,
    /// The branch-agent console UI.
    BranchAgentConsoleUi,
    /// The connector-admin console.
    ConnectorAdminConsole,
    /// The model-manager UI.
    ModelManagerUi,
    /// The run-history UI.
    RunHistoryUi,
    /// The replay-review UI.
    ReplayReviewUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5AiConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::AssistantPanelUi,
        Self::PatchReviewUi,
        Self::BranchAgentConsoleUi,
        Self::ConnectorAdminConsole,
        Self::ModelManagerUi,
        Self::RunHistoryUi,
        Self::ReplayReviewUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssistantPanelUi => "assistant_panel_ui",
            Self::PatchReviewUi => "patch_review_ui",
            Self::BranchAgentConsoleUi => "branch_agent_console_ui",
            Self::ConnectorAdminConsole => "connector_admin_console",
            Self::ModelManagerUi => "model_manager_ui",
            Self::RunHistoryUi => "run_history_ui",
            Self::ReplayReviewUi => "replay_review_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no AI truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5AiAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed AI-execution/replay component must be able to show. The
/// first three are hard requirements on every component; the remaining three close
/// the acceptance-criteria ambiguity about execution mode, route, and approval gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRequiredLabel {
    /// The component's stable identity / what AI object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The execution mode behind the component.
    ExecutionMode,
    /// The route / provider behind the component.
    Route,
    /// The approval gate behind the component's action.
    ApprovalGate,
}

impl M5AiRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ExecutionMode,
        Self::Route,
        Self::ApprovalGate,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ExecutionMode => "execution_mode",
            Self::Route => "route",
            Self::ApprovalGate => "approval_gate",
        }
    }
}

/// Qualification class for an M5 AI-execution/replay-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5AiQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an AI-execution/replay component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionDowngradeTrigger {
    /// A component left its execution mode unstated.
    ExecutionModeUnstated,
    /// A component masked its route or provider.
    RouteOrProviderMasked,
    /// A tool-call row left its tool boundary unstated.
    ToolBoundaryUnstated,
    /// A connector row masked its auth posture.
    AuthPostureMasked,
    /// An approval sheet hid its approval gate.
    ApprovalGateHidden,
    /// A component broke its checkpoint lineage.
    CheckpointLineageBroken,
    /// A replay-review sheet overstated replay completeness.
    ReplayCompletenessOverstated,
    /// A replay-review sheet left the rerun-review reason unstated.
    RerunReviewReasonUnstated,
    /// An agent-status card hid the manual-takeover path.
    TakeoverPathHidden,
    /// A connector row failed to disclose a side effect.
    ConnectorSideEffectUndisclosed,
    /// A local model pack card masked its provenance.
    LocalModelProvenanceMasked,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5AiExecutionDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ExecutionModeUnstated,
        Self::RouteOrProviderMasked,
        Self::ToolBoundaryUnstated,
        Self::AuthPostureMasked,
        Self::ApprovalGateHidden,
        Self::CheckpointLineageBroken,
        Self::ReplayCompletenessOverstated,
        Self::RerunReviewReasonUnstated,
        Self::TakeoverPathHidden,
        Self::ConnectorSideEffectUndisclosed,
        Self::LocalModelProvenanceMasked,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionModeUnstated => "execution_mode_unstated",
            Self::RouteOrProviderMasked => "route_or_provider_masked",
            Self::ToolBoundaryUnstated => "tool_boundary_unstated",
            Self::AuthPostureMasked => "auth_posture_masked",
            Self::ApprovalGateHidden => "approval_gate_hidden",
            Self::CheckpointLineageBroken => "checkpoint_lineage_broken",
            Self::ReplayCompletenessOverstated => "replay_completeness_overstated",
            Self::RerunReviewReasonUnstated => "rerun_review_reason_unstated",
            Self::TakeoverPathHidden => "takeover_path_hidden",
            Self::ConnectorSideEffectUndisclosed => "connector_side_effect_undisclosed",
            Self::LocalModelProvenanceMasked => "local_model_provenance_masked",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed AI-execution/replay component family bound to
/// the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentRow {
    /// Governed component family.
    pub component_family: M5AiExecutionComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5AiQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 AI surface families that render / consume this component.
    pub surface_families: Vec<M5AiSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5AiDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5AiRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5AiRequiredLabel>,
    /// Action states this component distinguishes (action-state-banner only).
    pub action_states: Vec<M5AiActionState>,
    /// Execution modes this component names (action-state-banner only).
    pub execution_modes: Vec<M5AiExecutionMode>,
    /// Connector capabilities this component names (connector-row only).
    pub connector_capabilities: Vec<M5AiConnectorCapability>,
    /// Auth postures this component names (connector-row only).
    pub auth_postures: Vec<M5AiAuthPosture>,
    /// Model pack states this component distinguishes (local-model-card only).
    pub model_pack_states: Vec<M5AiModelPackState>,
    /// Approval gates this component distinguishes (approval-sheet only).
    pub approval_gates: Vec<M5AiApprovalGate>,
    /// Friction reasons this component discloses (approval-sheet only).
    pub friction_reasons: Vec<M5AiFrictionReason>,
    /// Tool boundaries this component names (tool-call-row only).
    pub tool_boundaries: Vec<M5AiToolBoundary>,
    /// Side-effect classes this component distinguishes (tool-call-row only).
    pub side_effect_classes: Vec<M5AiSideEffectClass>,
    /// Run outcomes this component distinguishes (run-history-row only).
    pub run_outcomes: Vec<M5AiRunOutcome>,
    /// Replay-completeness states this component distinguishes (replay-review only).
    pub replay_completeness: Vec<M5AiReplayCompleteness>,
    /// Rerun-review reasons this component discloses (replay-review only).
    pub rerun_review_reasons: Vec<M5AiRerunReviewReason>,
    /// Agent lifecycle states this component distinguishes (agent-status only).
    pub agent_lifecycle_states: Vec<M5AiAgentLifecycleState>,
    /// Manual-takeover paths this component offers (agent-status only).
    pub takeover_paths: Vec<M5AiTakeoverPath>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5AiAccessibilityRoute>,
    /// AI subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5AiConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5AiExecutionDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its execution mode or route /
    /// provider. MUST be `false`.
    pub masks_execution_mode_or_route: bool,
    /// Hard invariant: this component never overstates replay completeness. MUST be
    /// `false`.
    pub overstates_replay_completeness: bool,
    /// Hard invariant: this component never invents a private AI-status grammar.
    /// MUST be `false`.
    pub invents_private_ai_status_grammar: bool,
    /// Hard invariant: this component never hides an approval gate or a manual-
    /// takeover path. MUST be `false`.
    pub hides_approval_gate_or_takeover_path: bool,
}

impl M5AiExecutionComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5AiRequiredLabel> = self.required_labels.iter().copied().collect();
        M5AiRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_execution_mode_or_route
            && !self.overstates_replay_completeness
            && !self.invents_private_ai_status_grammar
            && !self.hides_approval_gate_or_takeover_path
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Action-state tokens.
    pub action_states: Vec<String>,
    /// Execution-mode tokens.
    pub execution_modes: Vec<String>,
    /// Connector-capability tokens.
    pub connector_capabilities: Vec<String>,
    /// Auth-posture tokens.
    pub auth_postures: Vec<String>,
    /// Model-pack-state tokens.
    pub model_pack_states: Vec<String>,
    /// Approval-gate tokens.
    pub approval_gates: Vec<String>,
    /// Friction-reason tokens.
    pub friction_reasons: Vec<String>,
    /// Tool-boundary tokens.
    pub tool_boundaries: Vec<String>,
    /// Side-effect-class tokens.
    pub side_effect_classes: Vec<String>,
    /// Run-outcome tokens.
    pub run_outcomes: Vec<String>,
    /// Replay-completeness tokens.
    pub replay_completeness: Vec<String>,
    /// Rerun-review-reason tokens.
    pub rerun_review_reasons: Vec<String>,
    /// Agent-lifecycle-state tokens.
    pub agent_lifecycle_states: Vec<String>,
    /// Takeover-path tokens.
    pub takeover_paths: Vec<String>,
    /// AI-surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5AiExecutionComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5AiExecutionComponentFamily::ALL, |v| v.as_str()),
            action_states: tokens(&M5AiActionState::ALL, |v| v.as_str()),
            execution_modes: tokens(&M5AiExecutionMode::ALL, |v| v.as_str()),
            connector_capabilities: tokens(&M5AiConnectorCapability::ALL, |v| v.as_str()),
            auth_postures: tokens(&M5AiAuthPosture::ALL, |v| v.as_str()),
            model_pack_states: tokens(&M5AiModelPackState::ALL, |v| v.as_str()),
            approval_gates: tokens(&M5AiApprovalGate::ALL, |v| v.as_str()),
            friction_reasons: tokens(&M5AiFrictionReason::ALL, |v| v.as_str()),
            tool_boundaries: tokens(&M5AiToolBoundary::ALL, |v| v.as_str()),
            side_effect_classes: tokens(&M5AiSideEffectClass::ALL, |v| v.as_str()),
            run_outcomes: tokens(&M5AiRunOutcome::ALL, |v| v.as_str()),
            replay_completeness: tokens(&M5AiReplayCompleteness::ALL, |v| v.as_str()),
            rerun_review_reasons: tokens(&M5AiRerunReviewReason::ALL, |v| v.as_str()),
            agent_lifecycle_states: tokens(&M5AiAgentLifecycleState::ALL, |v| v.as_str()),
            takeover_paths: tokens(&M5AiTakeoverPath::ALL, |v| v.as_str()),
            surface_families: tokens(&M5AiSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5AiDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5AiConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AiAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5AiRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentGovernanceReview {
    /// The action-state banner shows its execution mode and action state.
    pub action_state_banner_shows_mode_and_state: bool,
    /// The connector row shows its capability class and auth posture.
    pub connector_row_shows_capability_and_auth: bool,
    /// The local model pack card shows its pack state and provenance.
    pub local_model_card_shows_pack_state_and_provenance: bool,
    /// The approval sheet shows its gate and friction reason.
    pub approval_sheet_shows_gate_and_friction: bool,
    /// The tool-call row shows its boundary and side effect.
    pub tool_call_row_shows_boundary_and_side_effect: bool,
    /// The run-history row shows its outcome and route.
    pub run_history_row_shows_outcome_and_route: bool,
    /// The replay-review sheet shows its completeness and rerun-review reason.
    pub replay_review_shows_completeness_and_rerun_reason: bool,
    /// The agent-status card shows its lifecycle and manual-takeover path.
    pub agent_status_shows_lifecycle_and_takeover_path: bool,
    /// Live and replayed / rerun execution are never conflated.
    pub live_versus_replayed_never_conflated: bool,
    /// No component invents a second AI-status grammar.
    pub no_component_invents_second_status_grammar: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel AI-execution vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentConsumerProjection {
    /// Assistant and run surfaces consume the shared mode / action vocabulary.
    pub assistant_and_run_surfaces_consume_mode_vocabulary: bool,
    /// Connector and tool surfaces consume the boundary / side-effect vocabulary.
    pub connector_and_tool_surfaces_consume_boundary_vocabulary: bool,
    /// Model surfaces consume the pack-state vocabulary.
    pub model_surfaces_consume_pack_state_vocabulary: bool,
    /// Approval surfaces consume the approval-gate vocabulary.
    pub approval_surfaces_consume_gate_vocabulary: bool,
    /// Support / export reads a single canonical AI-execution source.
    pub support_export_reads_single_source: bool,
    /// Replay and agent surfaces read a single canonical AI-execution source.
    pub replay_and_agent_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the AI-execution/replay lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting AI-execution audit for the lane.
    pub ai_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiExecutionComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiExecutionComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5AiExecutionComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiExecutionComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiExecutionComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiExecutionComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiExecutionComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiExecutionComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 AI-execution/replay-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionComponentMatrixPacket {
    /// Record kind; must equal [`M5_AI_EXECUTION_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_EXECUTION_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5AiExecutionComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiExecutionComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiExecutionComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiExecutionComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiExecutionComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiExecutionComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiExecutionComponentMatrixPacket {
    /// Builds an M5 AI-execution/replay-component matrix packet from stable-lane
    /// input.
    pub fn new(input: M5AiExecutionComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_AI_EXECUTION_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_AI_EXECUTION_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 AI-execution/replay-component matrix invariants.
    pub fn validate(&self) -> Vec<M5AiExecutionComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_EXECUTION_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5AiExecutionComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_EXECUTION_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5AiExecutionComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiExecutionComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai execution component matrix packet serializes"),
        ) {
            violations.push(M5AiExecutionComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 ai execution component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 AI Action-State-Banner, Connector-Detail-Row, Local-Model-Pack-Card, Approval-Sheet, Tool-Call-Timeline-Row, Run-History-Row, Replay-Review, and Agent-Status Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Execution modes: {}\n",
            self.vocabulary_set.execution_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Replay completeness: {}\n",
            self.vocabulary_set.replay_completeness.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 AI-execution matrix export.
#[derive(Debug)]
pub enum M5AiExecutionComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiExecutionComponentMatrixViolation>),
}

impl fmt::Display for M5AiExecutionComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai execution component matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 ai execution component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiExecutionComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5AiExecutionComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiExecutionComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// An action-state-banner component declares no action states.
    ActionStateMissing,
    /// An action-state-banner component declares no execution modes.
    ExecutionModeMissing,
    /// A connector-row component declares no connector capabilities.
    ConnectorCapabilityMissing,
    /// A connector-row component declares no auth postures.
    AuthPostureMissing,
    /// A local-model-card component declares no pack states.
    ModelPackStateMissing,
    /// An approval-sheet component declares no approval gates.
    ApprovalGateMissing,
    /// An approval-sheet component declares no friction reasons.
    FrictionReasonMissing,
    /// A tool-call-row component declares no tool boundaries.
    ToolBoundaryMissing,
    /// A tool-call-row component declares no side-effect classes.
    SideEffectClassMissing,
    /// A run-history-row component declares no run outcomes.
    RunOutcomeMissing,
    /// A replay-review component declares no replay-completeness states.
    ReplayCompletenessMissing,
    /// A replay-review component declares no rerun-review reasons.
    RerunReviewReasonMissing,
    /// An agent-status component declares no agent lifecycle states.
    AgentLifecycleStateMissing,
    /// An agent-status component declares no manual-takeover paths.
    TakeoverPathMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked mode/route, overstated replay,
    /// private status grammar, or hidden approval/takeover path).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AiExecutionComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ActionStateMissing => "action_state_missing",
            Self::ExecutionModeMissing => "execution_mode_missing",
            Self::ConnectorCapabilityMissing => "connector_capability_missing",
            Self::AuthPostureMissing => "auth_posture_missing",
            Self::ModelPackStateMissing => "model_pack_state_missing",
            Self::ApprovalGateMissing => "approval_gate_missing",
            Self::FrictionReasonMissing => "friction_reason_missing",
            Self::ToolBoundaryMissing => "tool_boundary_missing",
            Self::SideEffectClassMissing => "side_effect_class_missing",
            Self::RunOutcomeMissing => "run_outcome_missing",
            Self::ReplayCompletenessMissing => "replay_completeness_missing",
            Self::RerunReviewReasonMissing => "rerun_review_reason_missing",
            Self::AgentLifecycleStateMissing => "agent_lifecycle_state_missing",
            Self::TakeoverPathMissing => "takeover_path_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 AI-execution matrix export.
pub fn current_stable_m5_ai_execution_replay_component_matrix_export(
) -> Result<M5AiExecutionComponentMatrixPacket, M5AiExecutionComponentMatrixArtifactError> {
    let packet: M5AiExecutionComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/support_export.json"
    )))
    .map_err(M5AiExecutionComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiExecutionComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_EXECUTION_COMPONENT_SCHEMA_REF,
        M5_AI_EXECUTION_COMPONENT_DOC_REF,
        M5_AI_EXECUTION_COMPONENT_TOOL_REF,
        M5_AI_EXECUTION_COMPONENT_RUN_HISTORY_REF,
        M5_AI_EXECUTION_COMPONENT_REPLAY_REF,
        M5_AI_EXECUTION_COMPONENT_AGENT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiExecutionComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiExecutionComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    let present: BTreeSet<M5AiExecutionComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5AiExecutionComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5AiExecutionComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5AiExecutionComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5AiExecutionComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_action_state_banner() && row.action_states.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ActionStateMissing);
        }
        if family.is_action_state_banner() && row.execution_modes.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ExecutionModeMissing);
        }
        if family.is_connector_row() && row.connector_capabilities.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ConnectorCapabilityMissing);
        }
        if family.is_connector_row() && row.auth_postures.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::AuthPostureMissing);
        }
        if family.is_local_model_card() && row.model_pack_states.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ModelPackStateMissing);
        }
        if family.is_approval_sheet() && row.approval_gates.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ApprovalGateMissing);
        }
        if family.is_approval_sheet() && row.friction_reasons.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::FrictionReasonMissing);
        }
        if family.is_tool_call_row() && row.tool_boundaries.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ToolBoundaryMissing);
        }
        if family.is_tool_call_row() && row.side_effect_classes.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::SideEffectClassMissing);
        }
        if family.is_run_history_row() && row.run_outcomes.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::RunOutcomeMissing);
        }
        if family.is_replay_review() && row.replay_completeness.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ReplayCompletenessMissing);
        }
        if family.is_replay_review() && row.rerun_review_reasons.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::RerunReviewReasonMissing);
        }
        if family.is_agent_status() && row.agent_lifecycle_states.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::AgentLifecycleStateMissing);
        }
        if family.is_agent_status() && row.takeover_paths.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::TakeoverPathMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiExecutionComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiExecutionComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.action_state_banner_shows_mode_and_state,
        review.connector_row_shows_capability_and_auth,
        review.local_model_card_shows_pack_state_and_provenance,
        review.approval_sheet_shows_gate_and_friction,
        review.tool_call_row_shows_boundary_and_side_effect,
        review.run_history_row_shows_outcome_and_route,
        review.replay_review_shows_completeness_and_rerun_reason,
        review.agent_status_shows_lifecycle_and_takeover_path,
        review.live_versus_replayed_never_conflated,
        review.no_component_invents_second_status_grammar,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5AiExecutionComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.assistant_and_run_surfaces_consume_mode_vocabulary,
        projection.connector_and_tool_surfaces_consume_boundary_vocabulary,
        projection.model_surfaces_consume_pack_state_vocabulary,
        projection.approval_surfaces_consume_gate_vocabulary,
        projection.support_export_reads_single_source,
        projection.replay_and_agent_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(M5AiExecutionComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiExecutionComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiExecutionComponentMatrixPacket,
    violations: &mut Vec<M5AiExecutionComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiExecutionComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
