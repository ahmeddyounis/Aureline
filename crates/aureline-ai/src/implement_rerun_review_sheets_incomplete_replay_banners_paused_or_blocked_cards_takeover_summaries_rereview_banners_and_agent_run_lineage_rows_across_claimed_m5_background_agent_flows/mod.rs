//! Three reusable M5 background-agent replay / agent-status primitives — the rerun-review
//! sheet, the incomplete-replay banner, and the agent-status card (paused-or-blocked,
//! takeover summary, re-review banner, and agent-run-lineage row) — so background AI work
//! stays honest about rerun drift and safe takeover rather than appearing alive or
//! reusable by implication alone.
//!
//! Aureline's frozen AI-execution/replay component matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! names the replay-review sheet and the agent-status card as governed component families
//! and freezes their controlled vocabulary — the replay-completeness states, rerun-review
//! reasons, agent lifecycle states, manual-takeover paths, run outcomes, execution modes,
//! approval gates, surface families, deployment lines, consumer surfaces, accessibility
//! routes, qualification classes, and downgrade triggers. This module *implements* the
//! rerun / replay / agent-status side of that matrix as three reusable primitives so a user
//! can tell — from the sheet, the banner, or the card alone — why a rerun needs re-review,
//! what changed since the original run, which parts of a replay were retained versus
//! missing, and how to take over an interrupted branch/worktree agent safely, without
//! reconstructing any of it from raw logs.
//!
//! The module has three resolvers:
//!
//! 1. [`resolve_rerun_review_sheet`] — compares one AI run's original branch/base/provider/
//!    model/policy lineage with the current state and names which dimensions drifted, why a
//!    rerun requires re-review, and whether approval reuse is allowed, producing one
//!    [`M5ResolvedRerunReviewSheet`] that never silently reruns a drifted context nor admits
//!    a rerun on stale approvals.
//! 2. [`resolve_incomplete_replay_banner`] — explains which replay segments were retained
//!    versus missing and why new approvals are required before a rerun, producing one
//!    [`M5ResolvedIncompleteReplayBanner`] that never overstates replay completeness.
//! 3. [`resolve_agent_status_card`] — takes one branch/worktree agent's lifecycle state,
//!    checkpoint, current blast radius, last successful step, and pending writes, and
//!    produces one [`M5ResolvedAgentStatusCard`] carrying whether the agent is really alive,
//!    and the safe continue-manually / restart / abort-with-checkpoint options, so an
//!    interrupted or blocked agent never appears alive or safe to resume by implication.
//!
//! A single parity matrix — [`M5AiBackgroundAgentReplayPrimitivePacket`] — binds one row per
//! claimed M5 background-agent surface (rerun-review, branch-agent console, run-history,
//! support, and CLI) to the shared rerun-review, incomplete-replay, and agent-status anatomy,
//! the same rerun-review reasons, rerun admissions, drift dimensions, replay-completeness
//! states, replay segments, agent lifecycle states, takeover paths, blast radii, continue
//! options, export fields, and non-visual accessibility routes, so the same run lineage,
//! rerun grammar, and safe-takeover vocabulary stay identical across every surface a user
//! reviews, reruns, pauses, resumes, exports, or hands off AI work through.
//!
//! The run outcome ([`M5AiRunOutcome`]), execution mode ([`M5AiExecutionMode`]), approval
//! gate ([`M5AiApprovalGate`]), replay-completeness state ([`M5AiReplayCompleteness`]),
//! rerun-review reason ([`M5AiRerunReviewReason`]), agent lifecycle state
//! ([`M5AiAgentLifecycleState`]), takeover path ([`M5AiTakeoverPath`]), surface family
//! ([`M5AiSurfaceFamily`]), deployment line ([`M5AiDeploymentLine`]), consumer surface
//! ([`M5AiConsumerSurface`]), accessibility route ([`M5AiAccessibilityRoute`]),
//! qualification class ([`M5AiQualificationClass`]), and downgrade trigger
//! ([`M5AiExecutionDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the sheet, the
//! banner, and the card themselves: their background-agent surfaces, their anatomy parts,
//! their rerun drift dimensions, their rerun admissions, their replay segments, their agent
//! blast radii, their safe continue options, and their export fields. No M5 AI surface
//! invents a second rerun-review or agent-status grammar.
//!
//! Raw prompt bodies, raw tool return bodies, raw diffs, raw paths, raw URLs, and credential
//! material stay outside the support boundary; every run id, lineage label, checkpoint
//! label, step label, and packet id is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-rerun-review-sheet-incomplete-replay-banner-and-agent-status-card.schema.json`](../../../../schemas/ai/m5-ai-rerun-review-sheet-incomplete-replay-banner-and-agent-status-card.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows.md`](../../../../docs/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_background_agent_replay_primitive_packet,
    seeded_m5_ai_background_agent_replay_primitive_rerun_blocked_preview_narrowed,
    seeded_m5_ai_background_agent_replay_primitive_support_beta_narrowed,
    M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_PACKET_ID,
};

// The run outcome, execution mode, approval gate, replay-completeness state, rerun-review
// reason, agent lifecycle state, takeover path, surface family, deployment line, consumer
// surface, accessibility route, qualification class, and downgrade triggers are frozen once,
// in the AI-execution/replay component matrix. These primitives reuse them verbatim so they
// never invent a parallel rerun-review, replay, or agent-status vocabulary.
pub use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5AiAccessibilityRoute, M5AiAgentLifecycleState, M5AiApprovalGate, M5AiConsumerSurface,
    M5AiDeploymentLine, M5AiExecutionDowngradeTrigger, M5AiExecutionMode, M5AiQualificationClass,
    M5AiReplayCompleteness, M5AiRerunReviewReason, M5AiRunOutcome, M5AiSurfaceFamily,
    M5AiTakeoverPath,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiBackgroundAgentReplayPrimitivePacket`].
pub const M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_ai_rerun_review_sheets_incomplete_replay_banners_and_agent_status_cards_across_claimed_m5_background_agent_flows";

/// Schema version for M5 AI rerun-review / incomplete-replay / agent-status primitive
/// records.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the rerun-review / incomplete-replay / agent-status schema.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-rerun-review-sheet-incomplete-replay-banner-and-agent-status-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_DOC_REF: &str =
    "docs/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows.md";

/// Repo-relative path of the frozen AI-execution/replay component matrix these primitives
/// narrow from.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json";

/// Repo-relative path of the AI rerun-review contract this primitive binds its
/// original-vs-current drift, approval-reuse, and admission truth against.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_RERUN_REVIEW_REF: &str =
    "schemas/ai/ai_rerun_review.schema.json";

/// Repo-relative path of the evidence-replay-packet contract this primitive binds its
/// replay-completeness, retained/missing segment, and re-approval truth against.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_REPLAY_PACKET_REF: &str =
    "schemas/ai/evidence_replay_packet.schema.json";

/// Repo-relative path of the background-branch-agent-run contract this primitive binds its
/// lifecycle-state, checkpoint, blast-radius, and takeover truth against.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_AGENT_RUN_REF: &str =
    "schemas/ai/background-branch-agent-run.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_CSV_REF: &str =
    "artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_BACKGROUND_AGENT_REPLAY_REPORT_REF: &str =
    "artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows.md";

/// One claimed M5 background-agent surface that renders the shared rerun-review sheet,
/// incomplete-replay banner, and agent-status card. These are the surfaces where the same
/// run lineage and safe-takeover truth must stay consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiBackgroundAgentSurface {
    /// The rerun-review / replay surface.
    RerunReview,
    /// The branch/worktree-agent console surface.
    BranchAgentConsole,
    /// The run-history surface.
    RunHistory,
    /// The support-desk surface.
    Support,
    /// The CLI inspect / headless surface.
    Cli,
}

impl M5AiBackgroundAgentSurface {
    /// Every claimed background-agent surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RerunReview,
        Self::BranchAgentConsole,
        Self::RunHistory,
        Self::Support,
        Self::Cli,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunReview => "rerun_review",
            Self::BranchAgentConsole => "branch_agent_console",
            Self::RunHistory => "run_history",
            Self::Support => "support",
            Self::Cli => "cli",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RerunReview => "Rerun-Review",
            Self::BranchAgentConsole => "Branch-Agent Console",
            Self::RunHistory => "Run-History",
            Self::Support => "Support Desk",
            Self::Cli => "CLI Inspect",
        }
    }
}

/// Controlled rerun-review sheet anatomy part. The parts in
/// [`M5AiRerunReviewAnatomyPart::MANDATORY`] are required so the run lineage, the drift
/// dimensions, the rerun reason, and the approval-reuse verdict stay visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRerunReviewAnatomyPart {
    /// The canonical run id anchoring both the original run and the rerun.
    RunIdCue,
    /// The original-vs-current lineage comparison.
    LineageCompareCue,
    /// The set of drifted dimensions.
    DriftDimensionCue,
    /// The rerun-review reason.
    RerunReasonCue,
    /// The approval-reuse verdict.
    ApprovalReuseCue,
    /// The rerun / cancel action row.
    ActionCue,
}

impl M5AiRerunReviewAnatomyPart {
    /// Every rerun-review anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunIdCue,
        Self::LineageCompareCue,
        Self::DriftDimensionCue,
        Self::RerunReasonCue,
        Self::ApprovalReuseCue,
        Self::ActionCue,
    ];

    /// The rerun-review anatomy parts every sheet must render.
    pub const MANDATORY: [Self; 6] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunIdCue => "run_id_cue",
            Self::LineageCompareCue => "lineage_compare_cue",
            Self::DriftDimensionCue => "drift_dimension_cue",
            Self::RerunReasonCue => "rerun_reason_cue",
            Self::ApprovalReuseCue => "approval_reuse_cue",
            Self::ActionCue => "action_cue",
        }
    }
}

/// Controlled incomplete-replay banner anatomy part. The parts in
/// [`M5AiIncompleteReplayAnatomyPart::MANDATORY`] are required so packet id, retained and
/// missing segments, completeness, and the re-approval reason stay visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiIncompleteReplayAnatomyPart {
    /// The replay packet id.
    PacketIdCue,
    /// The run the packet belongs to.
    RunLinkCue,
    /// The retained replay segments.
    RetainedCue,
    /// The missing replay segments.
    MissingCue,
    /// The replay-completeness state.
    CompletenessCue,
    /// The why-new-approvals-required cue.
    ReapprovalCue,
}

impl M5AiIncompleteReplayAnatomyPart {
    /// Every incomplete-replay anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PacketIdCue,
        Self::RunLinkCue,
        Self::RetainedCue,
        Self::MissingCue,
        Self::CompletenessCue,
        Self::ReapprovalCue,
    ];

    /// The incomplete-replay anatomy parts every banner must render.
    pub const MANDATORY: [Self; 5] = [
        Self::PacketIdCue,
        Self::RetainedCue,
        Self::MissingCue,
        Self::CompletenessCue,
        Self::ReapprovalCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketIdCue => "packet_id_cue",
            Self::RunLinkCue => "run_link_cue",
            Self::RetainedCue => "retained_cue",
            Self::MissingCue => "missing_cue",
            Self::CompletenessCue => "completeness_cue",
            Self::ReapprovalCue => "reapproval_cue",
        }
    }
}

/// Controlled agent-status card anatomy part. The parts in
/// [`M5AiAgentStatusAnatomyPart::MANDATORY`] are required so agent id, lifecycle state,
/// checkpoint, blast radius, last successful step, pending writes, and safe continue
/// options stay visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAgentStatusAnatomyPart {
    /// The agent id.
    AgentIdCue,
    /// The lifecycle state.
    LifecycleCue,
    /// The checkpoint state.
    CheckpointCue,
    /// The current blast radius.
    BlastRadiusCue,
    /// The last successful step.
    LastStepCue,
    /// The pending writes.
    PendingWritesCue,
    /// The safe continue / restart / takeover options.
    ContinueOptionCue,
}

impl M5AiAgentStatusAnatomyPart {
    /// Every agent-status anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AgentIdCue,
        Self::LifecycleCue,
        Self::CheckpointCue,
        Self::BlastRadiusCue,
        Self::LastStepCue,
        Self::PendingWritesCue,
        Self::ContinueOptionCue,
    ];

    /// The agent-status anatomy parts every card must render.
    pub const MANDATORY: [Self; 6] = [
        Self::AgentIdCue,
        Self::LifecycleCue,
        Self::CheckpointCue,
        Self::BlastRadiusCue,
        Self::LastStepCue,
        Self::ContinueOptionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AgentIdCue => "agent_id_cue",
            Self::LifecycleCue => "lifecycle_cue",
            Self::CheckpointCue => "checkpoint_cue",
            Self::BlastRadiusCue => "blast_radius_cue",
            Self::LastStepCue => "last_step_cue",
            Self::PendingWritesCue => "pending_writes_cue",
            Self::ContinueOptionCue => "continue_option_cue",
        }
    }
}

/// Controlled rerun drift dimension — one dimension of the original run that may have
/// drifted since, so a rerun-review sheet names exactly what changed rather than rerunning
/// silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRerunDriftDimension {
    /// The original branch changed.
    OriginalBranch,
    /// The base revision changed.
    BaseRevision,
    /// The provider / route changed.
    ProviderRoute,
    /// The model version changed.
    ModelVersion,
    /// The policy epoch changed.
    PolicyEpoch,
    /// A tool contract changed.
    ToolContract,
    /// The input context changed.
    InputContext,
}

impl M5AiRerunDriftDimension {
    /// Every rerun drift dimension, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::OriginalBranch,
        Self::BaseRevision,
        Self::ProviderRoute,
        Self::ModelVersion,
        Self::PolicyEpoch,
        Self::ToolContract,
        Self::InputContext,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginalBranch => "original_branch",
            Self::BaseRevision => "base_revision",
            Self::ProviderRoute => "provider_route",
            Self::ModelVersion => "model_version",
            Self::PolicyEpoch => "policy_epoch",
            Self::ToolContract => "tool_contract",
            Self::InputContext => "input_context",
        }
    }

    /// True when a change to this dimension invalidates prior approval reuse — the route,
    /// model, policy, and tool contract are all approval-relevant.
    pub const fn invalidates_approval_reuse(self) -> bool {
        matches!(
            self,
            Self::ProviderRoute | Self::ModelVersion | Self::PolicyEpoch | Self::ToolContract
        )
    }
}

/// Controlled rerun admission class — whether a rerun may proceed, so a rerun-review sheet
/// never admits a rerun on stale approvals or a drifted provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRerunAdmission {
    /// The rerun may reuse the original approvals.
    AdmitWithApprovalReuse,
    /// The rerun may proceed only after re-review.
    AdmitAfterReReview,
    /// The rerun is blocked pending a fresh approval.
    BlockedPendingApproval,
    /// The rerun is blocked because the provider / model drifted.
    BlockedOnProviderDrift,
}

impl M5AiRerunAdmission {
    /// Every rerun admission, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AdmitWithApprovalReuse,
        Self::AdmitAfterReReview,
        Self::BlockedPendingApproval,
        Self::BlockedOnProviderDrift,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmitWithApprovalReuse => "admit_with_approval_reuse",
            Self::AdmitAfterReReview => "admit_after_re_review",
            Self::BlockedPendingApproval => "blocked_pending_approval",
            Self::BlockedOnProviderDrift => "blocked_on_provider_drift",
        }
    }

    /// True when a rerun in this admission class may proceed (with or without re-review).
    pub const fn is_admitted(self) -> bool {
        matches!(
            self,
            Self::AdmitWithApprovalReuse | Self::AdmitAfterReReview
        )
    }
}

/// Controlled replay segment — one part of a run's replay evidence, so an incomplete-replay
/// banner names exactly which parts were retained and which are missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReplaySegment {
    /// The prompt transcript.
    PromptTranscript,
    /// The tool-call log.
    ToolCallLog,
    /// The route receipt.
    RouteReceipt,
    /// The approval lineage.
    ApprovalLineage,
    /// The diff packet.
    DiffPacket,
    /// The provider response.
    ProviderResponse,
}

impl M5AiReplaySegment {
    /// Every replay segment, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PromptTranscript,
        Self::ToolCallLog,
        Self::RouteReceipt,
        Self::ApprovalLineage,
        Self::DiffPacket,
        Self::ProviderResponse,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptTranscript => "prompt_transcript",
            Self::ToolCallLog => "tool_call_log",
            Self::RouteReceipt => "route_receipt",
            Self::ApprovalLineage => "approval_lineage",
            Self::DiffPacket => "diff_packet",
            Self::ProviderResponse => "provider_response",
        }
    }
}

/// Controlled agent blast radius — how far a branch/worktree agent's current pending changes
/// reach, so an agent-status card discloses the current blast radius instead of implying a
/// contained agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAgentBlastRadius {
    /// No writes yet.
    NoWrites,
    /// Confined to the agent's worktree.
    WorktreeLocal,
    /// Scoped to the agent's branch.
    BranchScoped,
    /// Reaches the whole workspace.
    WorkspaceWide,
    /// Has external side effects.
    ExternalSideEffects,
}

impl M5AiAgentBlastRadius {
    /// Every blast radius, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoWrites,
        Self::WorktreeLocal,
        Self::BranchScoped,
        Self::WorkspaceWide,
        Self::ExternalSideEffects,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWrites => "no_writes",
            Self::WorktreeLocal => "worktree_local",
            Self::BranchScoped => "branch_scoped",
            Self::WorkspaceWide => "workspace_wide",
            Self::ExternalSideEffects => "external_side_effects",
        }
    }

    /// True when the blast radius is contained to the agent's own worktree or branch.
    pub const fn is_contained(self) -> bool {
        matches!(
            self,
            Self::NoWrites | Self::WorktreeLocal | Self::BranchScoped
        )
    }
}

/// Controlled safe continue option — one action a user can take on an interrupted agent, so
/// an agent-status card always offers a safe continue-manually or restart path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAgentContinueOption {
    /// Continue the agent's work manually.
    ContinueManually,
    /// Restart from the last checkpoint.
    RestartFromCheckpoint,
    /// Restart clean from scratch.
    RestartClean,
    /// Abort but preserve a checkpoint.
    AbortWithCheckpoint,
    /// Review the checkpoint before deciding.
    ReviewCheckpoint,
    /// Escalate to the workspace owner.
    EscalateToOwner,
}

impl M5AiAgentContinueOption {
    /// Every continue option, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ContinueManually,
        Self::RestartFromCheckpoint,
        Self::RestartClean,
        Self::AbortWithCheckpoint,
        Self::ReviewCheckpoint,
        Self::EscalateToOwner,
    ];

    /// The always-safe options every agent-status card offers — review the checkpoint and
    /// escalate to the owner never mutate state.
    pub const MANDATORY: [Self; 2] = [Self::ReviewCheckpoint, Self::EscalateToOwner];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueManually => "continue_manually",
            Self::RestartFromCheckpoint => "restart_from_checkpoint",
            Self::RestartClean => "restart_clean",
            Self::AbortWithCheckpoint => "abort_with_checkpoint",
            Self::ReviewCheckpoint => "review_checkpoint",
            Self::EscalateToOwner => "escalate_to_owner",
        }
    }
}

/// A field the rerun-review export carries so sheet truth is reconstructable. The fields in
/// [`M5AiRerunReviewExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRerunReviewExportField {
    /// The rerun-review id.
    RerunReviewId,
    /// The canonical run id.
    CanonicalRunId,
    /// The lineage comparison.
    LineageCompare,
    /// The drifted dimensions.
    DriftDimensions,
    /// The rerun-review reason.
    RerunReason,
    /// Whether approval reuse is allowed.
    ApprovalReuseAllowed,
    /// The rerun admission class.
    Admission,
}

impl M5AiRerunReviewExportField {
    /// Every rerun-review export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RerunReviewId,
        Self::CanonicalRunId,
        Self::LineageCompare,
        Self::DriftDimensions,
        Self::RerunReason,
        Self::ApprovalReuseAllowed,
        Self::Admission,
    ];

    /// The rerun-review export fields every sheet must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::RerunReviewId,
        Self::CanonicalRunId,
        Self::DriftDimensions,
        Self::RerunReason,
        Self::ApprovalReuseAllowed,
        Self::Admission,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunReviewId => "rerun_review_id",
            Self::CanonicalRunId => "canonical_run_id",
            Self::LineageCompare => "lineage_compare",
            Self::DriftDimensions => "drift_dimensions",
            Self::RerunReason => "rerun_reason",
            Self::ApprovalReuseAllowed => "approval_reuse_allowed",
            Self::Admission => "admission",
        }
    }
}

/// A field the incomplete-replay export carries so banner truth is reconstructable. The
/// fields in [`M5AiIncompleteReplayExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiIncompleteReplayExportField {
    /// The replay packet id.
    PacketId,
    /// The run the packet belongs to.
    CanonicalRunId,
    /// The retained segments.
    RetainedSegments,
    /// The missing segments.
    MissingSegments,
    /// The replay-completeness state.
    ReplayCompleteness,
    /// Whether new approvals are required.
    RequiresNewApprovals,
    /// Whether the replay is complete.
    IsComplete,
}

impl M5AiIncompleteReplayExportField {
    /// Every incomplete-replay export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PacketId,
        Self::CanonicalRunId,
        Self::RetainedSegments,
        Self::MissingSegments,
        Self::ReplayCompleteness,
        Self::RequiresNewApprovals,
        Self::IsComplete,
    ];

    /// The incomplete-replay export fields every banner must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::PacketId,
        Self::CanonicalRunId,
        Self::RetainedSegments,
        Self::MissingSegments,
        Self::ReplayCompleteness,
        Self::RequiresNewApprovals,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketId => "packet_id",
            Self::CanonicalRunId => "canonical_run_id",
            Self::RetainedSegments => "retained_segments",
            Self::MissingSegments => "missing_segments",
            Self::ReplayCompleteness => "replay_completeness",
            Self::RequiresNewApprovals => "requires_new_approvals",
            Self::IsComplete => "is_complete",
        }
    }
}

/// A field the agent-status export carries so card truth is reconstructable. The fields in
/// [`M5AiAgentStatusExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiAgentStatusExportField {
    /// The agent id.
    AgentId,
    /// The canonical run id.
    CanonicalRunId,
    /// The lifecycle state.
    LifecycleState,
    /// The checkpoint state.
    Checkpoint,
    /// The current blast radius.
    BlastRadius,
    /// The last successful step.
    LastSuccessfulStep,
    /// The pending writes.
    PendingWrites,
    /// The takeover path.
    TakeoverPath,
    /// The safe continue options.
    ContinueOptions,
    /// Whether the agent presents as alive.
    PresentsAsAlive,
}

impl M5AiAgentStatusExportField {
    /// Every agent-status export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::AgentId,
        Self::CanonicalRunId,
        Self::LifecycleState,
        Self::Checkpoint,
        Self::BlastRadius,
        Self::LastSuccessfulStep,
        Self::PendingWrites,
        Self::TakeoverPath,
        Self::ContinueOptions,
        Self::PresentsAsAlive,
    ];

    /// The agent-status export fields every card must carry.
    pub const MANDATORY: [Self; 8] = [
        Self::AgentId,
        Self::CanonicalRunId,
        Self::LifecycleState,
        Self::Checkpoint,
        Self::BlastRadius,
        Self::LastSuccessfulStep,
        Self::ContinueOptions,
        Self::PresentsAsAlive,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AgentId => "agent_id",
            Self::CanonicalRunId => "canonical_run_id",
            Self::LifecycleState => "lifecycle_state",
            Self::Checkpoint => "checkpoint",
            Self::BlastRadius => "blast_radius",
            Self::LastSuccessfulStep => "last_successful_step",
            Self::PendingWrites => "pending_writes",
            Self::TakeoverPath => "takeover_path",
            Self::ContinueOptions => "continue_options",
            Self::PresentsAsAlive => "presents_as_alive",
        }
    }
}

// ---- rerun-review-sheet resolver ----------------------------------------

/// The full input to the rerun-review-sheet resolver for one rerun of a recorded AI run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRerunReviewResolutionInput {
    /// The opaque rerun-review id.
    pub rerun_review_id: String,
    /// The opaque canonical run id of the original run, preserved verbatim on the rerun.
    pub canonical_run_id: String,
    /// The opaque original branch/base lineage label.
    pub original_lineage_label: String,
    /// The opaque current branch/base lineage label.
    pub current_lineage_label: String,
    /// The opaque current provider label.
    pub provider_label: String,
    /// The opaque current model label.
    pub model_label: String,
    /// The dimensions that drifted since the original run.
    pub changed_dimensions: Vec<M5AiRerunDriftDimension>,
    /// True when the original run's approvals are still effective in the current epoch.
    pub original_approvals_effective: bool,
}

/// The resolved rerun-review-sheet truth for one rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRerunReviewSheet {
    /// The opaque rerun-review id.
    pub rerun_review_id: String,
    /// The opaque canonical run id preserved verbatim.
    pub canonical_run_id: String,
    /// The opaque original branch/base lineage label.
    pub original_lineage_label: String,
    /// The opaque current branch/base lineage label.
    pub current_lineage_label: String,
    /// The composed current provider / model route label.
    pub route_label: String,
    /// True when the current provider and model are both named (route is not masked).
    pub route_is_complete: bool,
    /// The dimensions that drifted since the original run.
    pub changed_dimensions: Vec<M5AiRerunDriftDimension>,
    /// The derived rerun-review reason.
    pub rerun_review_reason: M5AiRerunReviewReason,
    /// True when the rerun may reuse the original approvals.
    pub approval_reuse_allowed: bool,
    /// True when the rerun requires re-review before it may proceed.
    pub requires_re_review: bool,
    /// The derived rerun admission class.
    pub admission: M5AiRerunAdmission,
}

/// Errors returned by [`resolve_rerun_review_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiRerunReviewResolutionError {
    /// The rerun-review id was empty.
    EmptyRerunReviewId,
    /// The canonical run id was empty.
    EmptyRunId,
    /// The original or current lineage label was empty.
    EmptyLineage,
    /// The current provider or model route was masked (empty), leaving the route implicit.
    RouteProviderModelMasked,
    /// A rerun-review descriptor carried forbidden material.
    ForbiddenRerunReviewMaterial,
}

impl M5AiRerunReviewResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRerunReviewId => "empty_rerun_review_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyLineage => "empty_lineage",
            Self::RouteProviderModelMasked => "route_provider_model_masked",
            Self::ForbiddenRerunReviewMaterial => "forbidden_rerun_review_material",
        }
    }
}

impl fmt::Display for M5AiRerunReviewResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai rerun review resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiRerunReviewResolutionError {}

/// Resolves one rerun-review sheet from a rerun's declared state.
///
/// The canonical run id is carried through verbatim so the rerun output can be linked back
/// to its source. The current provider and model must both be named — a masked route is
/// rejected as [`M5AiRerunReviewResolutionError::RouteProviderModelMasked`]. The rerun-review
/// reason is derived from the drifted dimensions with a fixed precedence, approval reuse is
/// allowed only when the original approvals are still effective and no approval-relevant
/// dimension drifted, and the rerun admission is blocked on provider drift or a stale
/// approval rather than admitted silently.
pub fn resolve_rerun_review_sheet(
    input: &M5AiRerunReviewResolutionInput,
) -> Result<M5ResolvedRerunReviewSheet, M5AiRerunReviewResolutionError> {
    if input.rerun_review_id.trim().is_empty() {
        return Err(M5AiRerunReviewResolutionError::EmptyRerunReviewId);
    }
    if input.canonical_run_id.trim().is_empty() {
        return Err(M5AiRerunReviewResolutionError::EmptyRunId);
    }
    if input.original_lineage_label.trim().is_empty()
        || input.current_lineage_label.trim().is_empty()
    {
        return Err(M5AiRerunReviewResolutionError::EmptyLineage);
    }
    if input.provider_label.trim().is_empty() || input.model_label.trim().is_empty() {
        return Err(M5AiRerunReviewResolutionError::RouteProviderModelMasked);
    }
    for value in [
        &input.rerun_review_id,
        &input.canonical_run_id,
        &input.original_lineage_label,
        &input.current_lineage_label,
        &input.provider_label,
        &input.model_label,
    ] {
        if value_repr_is_forbidden(value) {
            return Err(M5AiRerunReviewResolutionError::ForbiddenRerunReviewMaterial);
        }
    }

    let route_label = format!(
        "{} / {}",
        input.provider_label.trim(),
        input.model_label.trim()
    );
    let rerun_review_reason = derive_rerun_review_reason(&input.changed_dimensions);
    let approval_relevant_drift = input
        .changed_dimensions
        .iter()
        .any(|dim| dim.invalidates_approval_reuse());
    let approval_reuse_allowed = input.original_approvals_effective && !approval_relevant_drift;
    let requires_re_review =
        rerun_review_reason != M5AiRerunReviewReason::NoReReviewRequired || !approval_reuse_allowed;
    let provider_or_model_drift = input.changed_dimensions.iter().any(|dim| {
        matches!(
            dim,
            M5AiRerunDriftDimension::ProviderRoute | M5AiRerunDriftDimension::ModelVersion
        )
    });
    let admission = if !approval_reuse_allowed && provider_or_model_drift {
        M5AiRerunAdmission::BlockedOnProviderDrift
    } else if !approval_reuse_allowed {
        M5AiRerunAdmission::BlockedPendingApproval
    } else if requires_re_review {
        M5AiRerunAdmission::AdmitAfterReReview
    } else {
        M5AiRerunAdmission::AdmitWithApprovalReuse
    };

    Ok(M5ResolvedRerunReviewSheet {
        rerun_review_id: input.rerun_review_id.clone(),
        canonical_run_id: input.canonical_run_id.clone(),
        original_lineage_label: input.original_lineage_label.clone(),
        current_lineage_label: input.current_lineage_label.clone(),
        route_label,
        route_is_complete: true,
        changed_dimensions: input.changed_dimensions.clone(),
        rerun_review_reason,
        approval_reuse_allowed,
        requires_re_review,
        admission,
    })
}

/// Derives the rerun-review reason with a fixed precedence so the most approval-relevant
/// drift is named first and an undrifted rerun reads as needing no re-review.
fn derive_rerun_review_reason(changed: &[M5AiRerunDriftDimension]) -> M5AiRerunReviewReason {
    if changed
        .iter()
        .any(|d| *d == M5AiRerunDriftDimension::ModelVersion)
    {
        M5AiRerunReviewReason::ModelVersionChanged
    } else if changed
        .iter()
        .any(|d| *d == M5AiRerunDriftDimension::ToolContract)
    {
        M5AiRerunReviewReason::ToolContractChanged
    } else if changed
        .iter()
        .any(|d| *d == M5AiRerunDriftDimension::ProviderRoute)
    {
        M5AiRerunReviewReason::RouteOrProviderChanged
    } else if changed
        .iter()
        .any(|d| *d == M5AiRerunDriftDimension::PolicyEpoch)
    {
        M5AiRerunReviewReason::PolicyChanged
    } else if changed.is_empty() {
        M5AiRerunReviewReason::NoReReviewRequired
    } else {
        M5AiRerunReviewReason::InputContextChanged
    }
}

// ---- incomplete-replay-banner resolver ----------------------------------

/// The full input to the incomplete-replay-banner resolver for one replay packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiIncompleteReplayResolutionInput {
    /// The opaque replay packet id.
    pub packet_id: String,
    /// The opaque canonical run id this packet belongs to.
    pub canonical_run_id: String,
    /// How completely the run can be replayed.
    pub replay_completeness: M5AiReplayCompleteness,
    /// The replay segments that were retained.
    pub retained_segments: Vec<M5AiReplaySegment>,
    /// The replay segments that are missing.
    pub missing_segments: Vec<M5AiReplaySegment>,
}

/// The resolved incomplete-replay-banner truth for one replay packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedIncompleteReplayBanner {
    /// The opaque replay packet id.
    pub packet_id: String,
    /// The opaque canonical run id this packet belongs to.
    pub canonical_run_id: String,
    /// How completely the run can be replayed.
    pub replay_completeness: M5AiReplayCompleteness,
    /// The replay segments that were retained.
    pub retained_segments: Vec<M5AiReplaySegment>,
    /// The replay segments that are missing.
    pub missing_segments: Vec<M5AiReplaySegment>,
    /// True when the run is fully replayable.
    pub is_complete: bool,
    /// True when the approval lineage is among the missing segments.
    pub approval_lineage_missing: bool,
    /// True when a rerun requires new approvals before it may proceed.
    pub requires_new_approvals: bool,
}

/// Errors returned by [`resolve_incomplete_replay_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiIncompleteReplayResolutionError {
    /// The packet id was empty.
    EmptyPacketId,
    /// The run-id label was empty.
    EmptyRunId,
    /// The banner declared neither retained nor missing segments.
    NoSegmentsDeclared,
    /// The replay is marked fully replayable yet declares missing segments — an overstated
    /// completeness.
    CompleteButSegmentsMissing,
    /// An incomplete-replay descriptor carried forbidden material.
    ForbiddenIncompleteReplayMaterial,
}

impl M5AiIncompleteReplayResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPacketId => "empty_packet_id",
            Self::EmptyRunId => "empty_run_id",
            Self::NoSegmentsDeclared => "no_segments_declared",
            Self::CompleteButSegmentsMissing => "complete_but_segments_missing",
            Self::ForbiddenIncompleteReplayMaterial => "forbidden_incomplete_replay_material",
        }
    }
}

impl fmt::Display for M5AiIncompleteReplayResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai incomplete replay resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiIncompleteReplayResolutionError {}

/// Resolves one incomplete-replay banner from a replay packet's declared state.
///
/// The banner always names the retained and missing segments so a user can tell which parts
/// of the replay survived. A replay marked fully replayable that still declares missing
/// segments is rejected as
/// [`M5AiIncompleteReplayResolutionError::CompleteButSegmentsMissing`] — completeness is never
/// overstated. A rerun requires new approvals whenever the replay is not fully complete or the
/// approval lineage itself is missing.
pub fn resolve_incomplete_replay_banner(
    input: &M5AiIncompleteReplayResolutionInput,
) -> Result<M5ResolvedIncompleteReplayBanner, M5AiIncompleteReplayResolutionError> {
    if input.packet_id.trim().is_empty() {
        return Err(M5AiIncompleteReplayResolutionError::EmptyPacketId);
    }
    if input.canonical_run_id.trim().is_empty() {
        return Err(M5AiIncompleteReplayResolutionError::EmptyRunId);
    }
    if input.retained_segments.is_empty() && input.missing_segments.is_empty() {
        return Err(M5AiIncompleteReplayResolutionError::NoSegmentsDeclared);
    }
    for value in [&input.packet_id, &input.canonical_run_id] {
        if value_repr_is_forbidden(value) {
            return Err(M5AiIncompleteReplayResolutionError::ForbiddenIncompleteReplayMaterial);
        }
    }

    let is_complete = input.replay_completeness == M5AiReplayCompleteness::FullyReplayable;
    if is_complete && !input.missing_segments.is_empty() {
        return Err(M5AiIncompleteReplayResolutionError::CompleteButSegmentsMissing);
    }
    let approval_lineage_missing = input
        .missing_segments
        .contains(&M5AiReplaySegment::ApprovalLineage);
    let requires_new_approvals = !is_complete || approval_lineage_missing;

    Ok(M5ResolvedIncompleteReplayBanner {
        packet_id: input.packet_id.clone(),
        canonical_run_id: input.canonical_run_id.clone(),
        replay_completeness: input.replay_completeness,
        retained_segments: input.retained_segments.clone(),
        missing_segments: input.missing_segments.clone(),
        is_complete,
        approval_lineage_missing,
        requires_new_approvals,
    })
}

// ---- agent-status-card resolver -----------------------------------------

/// The full input to the agent-status-card resolver for one branch/worktree agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiAgentStatusResolutionInput {
    /// The opaque agent id.
    pub agent_id: String,
    /// The opaque canonical run id this agent produced.
    pub canonical_run_id: String,
    /// The agent's lifecycle state.
    pub lifecycle_state: M5AiAgentLifecycleState,
    /// The opaque checkpoint label (empty when the agent has no checkpoint).
    pub checkpoint_label: String,
    /// True when the agent carries a durable checkpoint.
    pub has_checkpoint: bool,
    /// The current blast radius of the agent's pending changes.
    pub blast_radius: M5AiAgentBlastRadius,
    /// The opaque last-successful-step label.
    pub last_successful_step_label: String,
    /// The number of pending (uncommitted) writes the agent holds.
    pub pending_writes_count: u32,
    /// The declared manual-takeover path.
    pub takeover_path: M5AiTakeoverPath,
}

/// The resolved agent-status-card truth for one branch/worktree agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAgentStatusCard {
    /// The opaque agent id.
    pub agent_id: String,
    /// The opaque canonical run id this agent produced.
    pub canonical_run_id: String,
    /// The agent's lifecycle state.
    pub lifecycle_state: M5AiAgentLifecycleState,
    /// The opaque checkpoint label.
    pub checkpoint_label: String,
    /// True when the agent carries a durable checkpoint.
    pub has_checkpoint: bool,
    /// The current blast radius of the agent's pending changes.
    pub blast_radius: M5AiAgentBlastRadius,
    /// The opaque last-successful-step label.
    pub last_successful_step_label: String,
    /// The number of pending writes.
    pub pending_writes_count: u32,
    /// The declared manual-takeover path.
    pub takeover_path: M5AiTakeoverPath,
    /// True only when the agent is really running (never true for a paused, blocked,
    /// awaiting-takeover, handed-off, completed, or abandoned agent).
    pub presents_as_alive: bool,
    /// True when the agent is interrupted (paused, blocked, or awaiting takeover).
    pub is_interrupted: bool,
    /// True when the agent holds pending writes.
    pub has_pending_writes: bool,
    /// The safe continue / restart / takeover options for this agent.
    pub continue_options: Vec<M5AiAgentContinueOption>,
}

/// Errors returned by [`resolve_agent_status_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiAgentStatusResolutionError {
    /// The agent id was empty.
    EmptyAgentId,
    /// The run-id label was empty.
    EmptyRunId,
    /// The last-successful-step label was empty.
    EmptyLastStep,
    /// The checkpoint was claimed but carries no label.
    CheckpointClaimedWithoutLabel,
    /// The agent is interrupted with pending writes but carries no checkpoint — its work
    /// could be lost on takeover.
    InterruptedWithPendingWritesButNoCheckpoint,
    /// An agent-status descriptor carried forbidden material.
    ForbiddenAgentStatusMaterial,
}

impl M5AiAgentStatusResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAgentId => "empty_agent_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyLastStep => "empty_last_step",
            Self::CheckpointClaimedWithoutLabel => "checkpoint_claimed_without_label",
            Self::InterruptedWithPendingWritesButNoCheckpoint => {
                "interrupted_with_pending_writes_but_no_checkpoint"
            }
            Self::ForbiddenAgentStatusMaterial => "forbidden_agent_status_material",
        }
    }
}

impl fmt::Display for M5AiAgentStatusResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai agent status resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiAgentStatusResolutionError {}

/// Resolves one agent-status card from a branch/worktree agent's declared state.
///
/// The agent presents as alive only when it is really running — a paused, blocked,
/// awaiting-takeover, handed-off, completed, or abandoned agent never reads as alive by
/// implication. An interrupted agent that holds pending writes but carries no checkpoint is
/// rejected as
/// [`M5AiAgentStatusResolutionError::InterruptedWithPendingWritesButNoCheckpoint`] so no work
/// is silently at risk on takeover. The safe continue options are derived from the checkpoint,
/// the pending-writes state, and whether the agent is interrupted, and always include the
/// non-mutating review-checkpoint and escalate-to-owner paths.
pub fn resolve_agent_status_card(
    input: &M5AiAgentStatusResolutionInput,
) -> Result<M5ResolvedAgentStatusCard, M5AiAgentStatusResolutionError> {
    if input.agent_id.trim().is_empty() {
        return Err(M5AiAgentStatusResolutionError::EmptyAgentId);
    }
    if input.canonical_run_id.trim().is_empty() {
        return Err(M5AiAgentStatusResolutionError::EmptyRunId);
    }
    if input.last_successful_step_label.trim().is_empty() {
        return Err(M5AiAgentStatusResolutionError::EmptyLastStep);
    }
    if input.has_checkpoint && input.checkpoint_label.trim().is_empty() {
        return Err(M5AiAgentStatusResolutionError::CheckpointClaimedWithoutLabel);
    }
    for value in [
        &input.agent_id,
        &input.canonical_run_id,
        &input.last_successful_step_label,
        &input.checkpoint_label,
    ] {
        if value_repr_is_forbidden(value) {
            return Err(M5AiAgentStatusResolutionError::ForbiddenAgentStatusMaterial);
        }
    }

    let presents_as_alive = input.lifecycle_state == M5AiAgentLifecycleState::Running;
    let is_interrupted = matches!(
        input.lifecycle_state,
        M5AiAgentLifecycleState::Paused
            | M5AiAgentLifecycleState::BlockedOnApproval
            | M5AiAgentLifecycleState::AwaitingTakeover
    );
    let has_pending_writes = input.pending_writes_count > 0;

    if is_interrupted && has_pending_writes && !input.has_checkpoint {
        return Err(M5AiAgentStatusResolutionError::InterruptedWithPendingWritesButNoCheckpoint);
    }

    let continue_options = derive_continue_options(input, is_interrupted, has_pending_writes);

    Ok(M5ResolvedAgentStatusCard {
        agent_id: input.agent_id.clone(),
        canonical_run_id: input.canonical_run_id.clone(),
        lifecycle_state: input.lifecycle_state,
        checkpoint_label: input.checkpoint_label.clone(),
        has_checkpoint: input.has_checkpoint,
        blast_radius: input.blast_radius,
        last_successful_step_label: input.last_successful_step_label.clone(),
        pending_writes_count: input.pending_writes_count,
        takeover_path: input.takeover_path,
        presents_as_alive,
        is_interrupted,
        has_pending_writes,
        continue_options,
    })
}

/// Derives the safe continue options in enum-declaration order so no card invents a parallel
/// action grammar. Review-checkpoint and escalate-to-owner are always offered; restart-clean
/// is only offered when there are no pending writes to lose, and abort-with-checkpoint only
/// when there are.
fn derive_continue_options(
    input: &M5AiAgentStatusResolutionInput,
    is_interrupted: bool,
    has_pending_writes: bool,
) -> Vec<M5AiAgentContinueOption> {
    let can_continue_manually = is_interrupted
        || matches!(
            input.lifecycle_state,
            M5AiAgentLifecycleState::HandedOff | M5AiAgentLifecycleState::AwaitingTakeover
        );
    M5AiAgentContinueOption::ALL
        .into_iter()
        .filter(|option| match option {
            M5AiAgentContinueOption::ContinueManually => can_continue_manually,
            M5AiAgentContinueOption::RestartFromCheckpoint => input.has_checkpoint,
            M5AiAgentContinueOption::RestartClean => !has_pending_writes,
            M5AiAgentContinueOption::AbortWithCheckpoint => has_pending_writes,
            M5AiAgentContinueOption::ReviewCheckpoint => true,
            M5AiAgentContinueOption::EscalateToOwner => true,
        })
        .collect()
}

// ---- worked cases -------------------------------------------------------

/// One worked rerun-review resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRerunReviewResolutionCase {
    /// The resolver input.
    pub input: M5AiRerunReviewResolutionInput,
    /// The resolved truth. Must equal `resolve_rerun_review_sheet(&input)`.
    pub resolved: M5ResolvedRerunReviewSheet,
}

impl M5AiRerunReviewResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiRerunReviewResolutionInput) -> Self {
        let resolved = resolve_rerun_review_sheet(&input).expect("seed rerun-review case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_rerun_review_sheet(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked incomplete-replay resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiIncompleteReplayResolutionCase {
    /// The resolver input.
    pub input: M5AiIncompleteReplayResolutionInput,
    /// The resolved truth. Must equal `resolve_incomplete_replay_banner(&input)`.
    pub resolved: M5ResolvedIncompleteReplayBanner,
}

impl M5AiIncompleteReplayResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiIncompleteReplayResolutionInput) -> Self {
        let resolved =
            resolve_incomplete_replay_banner(&input).expect("seed incomplete-replay case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_incomplete_replay_banner(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked agent-status resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiAgentStatusResolutionCase {
    /// The resolver input.
    pub input: M5AiAgentStatusResolutionInput,
    /// The resolved truth. Must equal `resolve_agent_status_card(&input)`.
    pub resolved: M5ResolvedAgentStatusCard,
}

impl M5AiAgentStatusResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiAgentStatusResolutionInput) -> Self {
        let resolved = resolve_agent_status_card(&input).expect("seed agent-status case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_agent_status_card(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one claimed M5 background-agent surface bound to the
/// shared rerun-review, incomplete-replay, and agent-status anatomy, vocabularies, continue
/// options, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBackgroundAgentReplayRow {
    /// Background-agent surface family.
    pub background_agent_surface: M5AiBackgroundAgentSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5AiQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 AI surface families that render / consume these components.
    pub surface_families: Vec<M5AiSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5AiDeploymentLine>,
    /// Rerun-review anatomy parts this sheet renders (must include the mandatory parts).
    pub rerun_review_anatomy_parts: Vec<M5AiRerunReviewAnatomyPart>,
    /// Incomplete-replay anatomy parts this banner renders (must include the mandatory
    /// parts).
    pub incomplete_replay_anatomy_parts: Vec<M5AiIncompleteReplayAnatomyPart>,
    /// Agent-status anatomy parts this card renders (must include the mandatory parts).
    pub agent_status_anatomy_parts: Vec<M5AiAgentStatusAnatomyPart>,
    /// Safe continue options this surface offers (must include the mandatory options).
    pub continue_options: Vec<M5AiAgentContinueOption>,
    /// Execution modes this surface distinguishes.
    pub execution_modes: Vec<M5AiExecutionMode>,
    /// Run outcomes this surface distinguishes.
    pub run_outcomes: Vec<M5AiRunOutcome>,
    /// Approval gates this surface distinguishes.
    pub approval_gates: Vec<M5AiApprovalGate>,
    /// Rerun-review reasons this surface names.
    pub rerun_review_reasons: Vec<M5AiRerunReviewReason>,
    /// Rerun admissions this surface distinguishes.
    pub rerun_admissions: Vec<M5AiRerunAdmission>,
    /// Rerun drift dimensions this surface distinguishes.
    pub rerun_drift_dimensions: Vec<M5AiRerunDriftDimension>,
    /// Replay-completeness states this surface distinguishes.
    pub replay_completeness_states: Vec<M5AiReplayCompleteness>,
    /// Replay segments this surface names.
    pub replay_segments: Vec<M5AiReplaySegment>,
    /// Agent lifecycle states this surface distinguishes.
    pub agent_lifecycle_states: Vec<M5AiAgentLifecycleState>,
    /// Takeover paths this surface distinguishes.
    pub takeover_paths: Vec<M5AiTakeoverPath>,
    /// Agent blast radii this surface distinguishes.
    pub agent_blast_radii: Vec<M5AiAgentBlastRadius>,
    /// Rerun-review export fields this sheet carries (must include the mandatory fields).
    pub rerun_review_export_fields: Vec<M5AiRerunReviewExportField>,
    /// Incomplete-replay export fields this banner carries (must include the mandatory
    /// fields).
    pub incomplete_replay_export_fields: Vec<M5AiIncompleteReplayExportField>,
    /// Agent-status export fields this card carries (must include the mandatory fields).
    pub agent_status_export_fields: Vec<M5AiAgentStatusExportField>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5AiAccessibilityRoute>,
    /// AI subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5AiConsumerSurface>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5AiExecutionDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked rerun-review resolutions proving the rerun-review resolver on this surface.
    pub rerun_review_examples: Vec<M5AiRerunReviewResolutionCase>,
    /// Worked incomplete-replay resolutions proving the replay resolver on this surface.
    pub incomplete_replay_examples: Vec<M5AiIncompleteReplayResolutionCase>,
    /// Worked agent-status resolutions proving the agent-status resolver on this surface.
    pub agent_status_examples: Vec<M5AiAgentStatusResolutionCase>,
    /// Hard invariant: this surface never masks the AI run lineage across surfaces. MUST be
    /// `false`.
    pub masks_run_lineage_across_surfaces: bool,
    /// Hard invariant: this surface never presents an interrupted agent as alive. MUST be
    /// `false`.
    pub presents_interrupted_agent_as_alive: bool,
    /// Hard invariant: this surface never overstates replay completeness. MUST be `false`.
    pub overstates_replay_completeness: bool,
    /// Hard invariant: this surface never invents a parallel rerun-review or agent-status
    /// grammar. MUST be `false`.
    pub invents_parallel_rerun_or_agent_grammar: bool,
}

impl M5AiBackgroundAgentReplayRow {
    /// True when the row declares every mandatory rerun-review anatomy part.
    fn declares_mandatory_rerun_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiRerunReviewAnatomyPart> =
            self.rerun_review_anatomy_parts.iter().copied().collect();
        M5AiRerunReviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory incomplete-replay anatomy part.
    fn declares_mandatory_replay_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiIncompleteReplayAnatomyPart> = self
            .incomplete_replay_anatomy_parts
            .iter()
            .copied()
            .collect();
        M5AiIncompleteReplayAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory agent-status anatomy part.
    fn declares_mandatory_agent_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiAgentStatusAnatomyPart> =
            self.agent_status_anatomy_parts.iter().copied().collect();
        M5AiAgentStatusAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row keeps the mandatory always-safe continue options.
    fn declares_mandatory_continue_options(&self) -> bool {
        let present: BTreeSet<M5AiAgentContinueOption> =
            self.continue_options.iter().copied().collect();
        M5AiAgentContinueOption::MANDATORY
            .iter()
            .all(|option| present.contains(option))
    }

    /// True when the row declares every mandatory rerun-review export field.
    fn declares_mandatory_rerun_export(&self) -> bool {
        let present: BTreeSet<M5AiRerunReviewExportField> =
            self.rerun_review_export_fields.iter().copied().collect();
        M5AiRerunReviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory incomplete-replay export field.
    fn declares_mandatory_replay_export(&self) -> bool {
        let present: BTreeSet<M5AiIncompleteReplayExportField> = self
            .incomplete_replay_export_fields
            .iter()
            .copied()
            .collect();
        M5AiIncompleteReplayExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory agent-status export field.
    fn declares_mandatory_agent_export(&self) -> bool {
        let present: BTreeSet<M5AiAgentStatusExportField> =
            self.agent_status_export_fields.iter().copied().collect();
        M5AiAgentStatusExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row proves a blocked rerun with named drift so a rerun's re-review
    /// reason never collapses into an implicit "changed" badge.
    fn proves_blocked_rerun_with_drift(&self) -> bool {
        self.rerun_review_examples.iter().any(|case| {
            !case.resolved.approval_reuse_allowed
                && !case.resolved.changed_dimensions.is_empty()
                && case.resolved.requires_re_review
        })
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_run_lineage_across_surfaces
            && !self.presents_interrupted_agent_as_alive
            && !self.overstates_replay_completeness
            && !self.invents_parallel_rerun_or_agent_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBackgroundAgentReplayVocabularySet {
    /// Background-agent-surface tokens.
    pub background_agent_surfaces: Vec<String>,
    /// Rerun-review-anatomy-part tokens.
    pub rerun_review_anatomy_parts: Vec<String>,
    /// Incomplete-replay-anatomy-part tokens.
    pub incomplete_replay_anatomy_parts: Vec<String>,
    /// Agent-status-anatomy-part tokens.
    pub agent_status_anatomy_parts: Vec<String>,
    /// Continue-option tokens.
    pub continue_options: Vec<String>,
    /// Rerun-drift-dimension tokens.
    pub rerun_drift_dimensions: Vec<String>,
    /// Rerun-admission tokens.
    pub rerun_admissions: Vec<String>,
    /// Replay-segment tokens.
    pub replay_segments: Vec<String>,
    /// Agent-blast-radius tokens.
    pub agent_blast_radii: Vec<String>,
    /// Rerun-review-export-field tokens.
    pub rerun_review_export_fields: Vec<String>,
    /// Incomplete-replay-export-field tokens.
    pub incomplete_replay_export_fields: Vec<String>,
    /// Agent-status-export-field tokens.
    pub agent_status_export_fields: Vec<String>,
    /// Execution-mode tokens (reused from the frozen matrix).
    pub execution_modes: Vec<String>,
    /// Run-outcome tokens (reused from the frozen matrix).
    pub run_outcomes: Vec<String>,
    /// Approval-gate tokens (reused from the frozen matrix).
    pub approval_gates: Vec<String>,
    /// Replay-completeness tokens (reused from the frozen matrix).
    pub replay_completeness_states: Vec<String>,
    /// Rerun-review-reason tokens (reused from the frozen matrix).
    pub rerun_review_reasons: Vec<String>,
    /// Agent-lifecycle-state tokens (reused from the frozen matrix).
    pub agent_lifecycle_states: Vec<String>,
    /// Takeover-path tokens (reused from the frozen matrix).
    pub takeover_paths: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AiBackgroundAgentReplayVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            background_agent_surfaces: tokens(&M5AiBackgroundAgentSurface::ALL, |v| v.as_str()),
            rerun_review_anatomy_parts: tokens(&M5AiRerunReviewAnatomyPart::ALL, |v| v.as_str()),
            incomplete_replay_anatomy_parts: tokens(&M5AiIncompleteReplayAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            agent_status_anatomy_parts: tokens(&M5AiAgentStatusAnatomyPart::ALL, |v| v.as_str()),
            continue_options: tokens(&M5AiAgentContinueOption::ALL, |v| v.as_str()),
            rerun_drift_dimensions: tokens(&M5AiRerunDriftDimension::ALL, |v| v.as_str()),
            rerun_admissions: tokens(&M5AiRerunAdmission::ALL, |v| v.as_str()),
            replay_segments: tokens(&M5AiReplaySegment::ALL, |v| v.as_str()),
            agent_blast_radii: tokens(&M5AiAgentBlastRadius::ALL, |v| v.as_str()),
            rerun_review_export_fields: tokens(&M5AiRerunReviewExportField::ALL, |v| v.as_str()),
            incomplete_replay_export_fields: tokens(&M5AiIncompleteReplayExportField::ALL, |v| {
                v.as_str()
            }),
            agent_status_export_fields: tokens(&M5AiAgentStatusExportField::ALL, |v| v.as_str()),
            execution_modes: tokens(&M5AiExecutionMode::ALL, |v| v.as_str()),
            run_outcomes: tokens(&M5AiRunOutcome::ALL, |v| v.as_str()),
            approval_gates: tokens(&M5AiApprovalGate::ALL, |v| v.as_str()),
            replay_completeness_states: tokens(&M5AiReplayCompleteness::ALL, |v| v.as_str()),
            rerun_review_reasons: tokens(&M5AiRerunReviewReason::ALL, |v| v.as_str()),
            agent_lifecycle_states: tokens(&M5AiAgentLifecycleState::ALL, |v| v.as_str()),
            takeover_paths: tokens(&M5AiTakeoverPath::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AiAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5AiBackgroundAgentReplayGovernanceReview {
    /// One primitive trio carries rerun-review, incomplete-replay, and agent-status truth on
    /// every surface.
    pub one_primitive_carries_rerun_replay_and_agent_truth: bool,
    /// The canonical run lineage stays consistent across rerun-review, replay, and
    /// agent-status.
    pub run_lineage_consistent_across_surfaces: bool,
    /// A rerun-review sheet names why re-review is required and what drifted.
    pub rerun_review_names_reason_and_drift: bool,
    /// Approval reuse is admitted only when no approval-relevant dimension drifted.
    pub approval_reuse_only_when_no_relevant_drift: bool,
    /// An incomplete-replay banner names retained versus missing segments.
    pub incomplete_replay_names_retained_and_missing: bool,
    /// Replay completeness is never overstated.
    pub replay_completeness_never_overstated: bool,
    /// An interrupted agent never presents as alive.
    pub interrupted_agent_never_presents_as_alive: bool,
    /// Every interrupted agent offers a safe continue-manually or restart option.
    pub interrupted_agent_offers_safe_continue: bool,
    /// The support / export packet reconstructs sheet, banner, and card truth.
    pub support_export_reconstructs_rerun_replay_and_agent_truth: bool,
    /// No surface invents a second rerun-review or agent-status grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBackgroundAgentReplayConsumerProjection {
    /// Rerun-review, branch-agent console, run-history, support, and CLI surfaces all consume
    /// the shared primitive trio.
    pub background_agent_surfaces_consume_shared_primitive: bool,
    /// The run-lineage projection reads a single canonical source.
    pub run_lineage_reads_single_source: bool,
    /// The rerun-admission projection reads a single canonical source.
    pub rerun_admission_reads_single_source: bool,
    /// The replay-completeness projection reads a single canonical source.
    pub replay_completeness_reads_single_source: bool,
    /// The agent-liveness / takeover projection reads a single canonical source.
    pub agent_liveness_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBackgroundAgentReplayProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive trio.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBackgroundAgentReplayReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiBackgroundAgentReplayPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiBackgroundAgentReplayPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Background-agent-surface rows.
    pub rows: Vec<M5AiBackgroundAgentReplayRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiBackgroundAgentReplayVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiBackgroundAgentReplayGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiBackgroundAgentReplayConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiBackgroundAgentReplayProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiBackgroundAgentReplayReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 AI rerun-review / incomplete-replay / agent-status primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBackgroundAgentReplayPrimitivePacket {
    /// Record kind; must equal [`M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Background-agent-surface rows.
    pub rows: Vec<M5AiBackgroundAgentReplayRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiBackgroundAgentReplayVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiBackgroundAgentReplayGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiBackgroundAgentReplayConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiBackgroundAgentReplayProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiBackgroundAgentReplayReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiBackgroundAgentReplayPrimitivePacket {
    /// Builds an M5 AI rerun-review / incomplete-replay / agent-status primitive packet.
    pub fn new(input: M5AiBackgroundAgentReplayPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 rerun-review / incomplete-replay / agent-status invariants.
    pub fn validate(&self) -> Vec<M5AiBackgroundAgentReplayPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_RECORD_KIND {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_BACKGROUND_AGENT_REPLAY_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_run_lineage_consistency(self, &mut violations);
        validate_drift_disclosure(self, &mut violations);
        validate_interrupted_agent_honesty(self, &mut violations);
        validate_incomplete_replay_reapproval(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai background-agent replay primitive packet serializes"),
        ) {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 ai background-agent replay primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per background-agent surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "background_agent_surface,qualification,owner,rerun_anatomy,replay_anatomy,agent_anatomy,continue_options,rerun_admissions,replay_completeness,lifecycle_states,takeover_paths,rerun_examples,replay_examples,agent_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.background_agent_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.rerun_review_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.incomplete_replay_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.agent_status_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.continue_options, |v| v.as_str()),
                join_tokens(&row.rerun_admissions, |v| v.as_str()),
                join_tokens(&row.replay_completeness_states, |v| v.as_str()),
                join_tokens(&row.agent_lifecycle_states, |v| v.as_str()),
                join_tokens(&row.takeover_paths, |v| v.as_str()),
                row.rerun_review_examples.len(),
                row.incomplete_replay_examples.len(),
                row.agent_status_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 AI Rerun-Review Sheet, Incomplete-Replay Banner, and Agent-Status Card Primitive\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Background-agent surfaces: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Continue options: {}\n",
            self.vocabulary_set.continue_options.join(", ")
        ));
        out.push_str(&format!(
            "- Rerun admissions: {}\n",
            self.vocabulary_set.rerun_admissions.join(", ")
        ));
        out.push_str(&format!(
            "- Replay completeness: {}\n",
            self.vocabulary_set.replay_completeness_states.join(", ")
        ));
        out.push_str(&format!(
            "- Agent lifecycle states: {}\n",
            self.vocabulary_set.agent_lifecycle_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Background-agent surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.background_agent_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked rerun-review sheets: {}\n",
                row.rerun_review_examples.len()
            ));
            for case in &row.rerun_review_examples {
                out.push_str(&format!(
                    "    - `{}` → reason `{}` admission `{}` (reuse `{}`)\n",
                    case.resolved.canonical_run_id,
                    case.resolved.rerun_review_reason.as_str(),
                    case.resolved.admission.as_str(),
                    case.resolved.approval_reuse_allowed,
                ));
            }
            out.push_str(&format!(
                "  - Worked incomplete-replay banners: {}\n",
                row.incomplete_replay_examples.len()
            ));
            for case in &row.incomplete_replay_examples {
                out.push_str(&format!(
                    "    - `{}` → completeness `{}` (missing {}, reapprove `{}`)\n",
                    case.resolved.packet_id,
                    case.resolved.replay_completeness.as_str(),
                    case.resolved.missing_segments.len(),
                    case.resolved.requires_new_approvals,
                ));
            }
            out.push_str(&format!(
                "  - Worked agent-status cards: {}\n",
                row.agent_status_examples.len()
            ));
            for case in &row.agent_status_examples {
                out.push_str(&format!(
                    "    - `{}` → lifecycle `{}` blast `{}` (alive `{}`, options {})\n",
                    case.resolved.agent_id,
                    case.resolved.lifecycle_state.as_str(),
                    case.resolved.blast_radius.as_str(),
                    case.resolved.presents_as_alive,
                    case.resolved.continue_options.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 background-agent-replay-primitive export.
#[derive(Debug)]
pub enum M5AiBackgroundAgentReplayPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiBackgroundAgentReplayPrimitiveViolation>),
}

impl fmt::Display for M5AiBackgroundAgentReplayPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai background-agent replay primitive export parse failed: {error}"
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
                    "m5 ai background-agent replay primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiBackgroundAgentReplayPrimitiveArtifactError {}

/// Validation failures emitted by [`M5AiBackgroundAgentReplayPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiBackgroundAgentReplayPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required background-agent surface is missing from the matrix.
    RequiredSurfaceMissing,
    /// A background-agent-surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory rerun-review anatomy parts.
    MandatoryRerunAnatomyMissing,
    /// A row omits one of the mandatory incomplete-replay anatomy parts.
    MandatoryReplayAnatomyMissing,
    /// A row omits one of the mandatory agent-status anatomy parts.
    MandatoryAgentAnatomyMissing,
    /// A row omits one of the mandatory always-safe continue options.
    MandatoryContinueOptionMissing,
    /// A row omits one of the mandatory rerun-review export fields.
    MandatoryRerunExportMissing,
    /// A row omits one of the mandatory incomplete-replay export fields.
    MandatoryReplayExportMissing,
    /// A row omits one of the mandatory agent-status export fields.
    MandatoryAgentExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked rerun-review resolutions.
    RerunExampleMissing,
    /// A row declares no worked incomplete-replay resolutions.
    ReplayExampleMissing,
    /// A row declares no worked agent-status resolutions.
    AgentExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No canonical run lineage is shared across a rerun-review, a replay, and an
    /// agent-status example.
    RunLineageConsistencyUnproven,
    /// No rerun-review example proves a blocked rerun with named drift so why-re-review stays
    /// explicit.
    DriftDisclosureUnproven,
    /// No agent-status example proves an interrupted agent shown as not alive with a safe
    /// continue option.
    InterruptedAgentHonestyUnproven,
    /// No incomplete-replay example proves an incomplete replay requiring new approvals with
    /// named retained and missing segments.
    IncompleteReplayReapprovalUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AiBackgroundAgentReplayPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryRerunAnatomyMissing => "mandatory_rerun_anatomy_missing",
            Self::MandatoryReplayAnatomyMissing => "mandatory_replay_anatomy_missing",
            Self::MandatoryAgentAnatomyMissing => "mandatory_agent_anatomy_missing",
            Self::MandatoryContinueOptionMissing => "mandatory_continue_option_missing",
            Self::MandatoryRerunExportMissing => "mandatory_rerun_export_missing",
            Self::MandatoryReplayExportMissing => "mandatory_replay_export_missing",
            Self::MandatoryAgentExportMissing => "mandatory_agent_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RerunExampleMissing => "rerun_example_missing",
            Self::ReplayExampleMissing => "replay_example_missing",
            Self::AgentExampleMissing => "agent_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::RunLineageConsistencyUnproven => "run_lineage_consistency_unproven",
            Self::DriftDisclosureUnproven => "drift_disclosure_unproven",
            Self::InterruptedAgentHonestyUnproven => "interrupted_agent_honesty_unproven",
            Self::IncompleteReplayReapprovalUnproven => "incomplete_replay_reapproval_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 background-agent-replay-primitive export.
pub fn current_stable_m5_ai_background_agent_replay_primitive_export(
) -> Result<M5AiBackgroundAgentReplayPrimitivePacket, M5AiBackgroundAgentReplayPrimitiveArtifactError>
{
    let packet: M5AiBackgroundAgentReplayPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/support_export.json"
    )))
    .map_err(M5AiBackgroundAgentReplayPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiBackgroundAgentReplayPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_DOC_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_COMPONENT_MATRIX_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_RERUN_REVIEW_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_REPLAY_PACKET_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_AGENT_RUN_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let present: BTreeSet<M5AiBackgroundAgentSurface> = packet
        .rows
        .iter()
        .map(|row| row.background_agent_surface)
        .collect();
    for required in M5AiBackgroundAgentSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.rerun_review_anatomy_parts.is_empty()
            || row.incomplete_replay_anatomy_parts.is_empty()
            || row.agent_status_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.continue_options.is_empty()
            || row.execution_modes.is_empty()
            || row.run_outcomes.is_empty()
            || row.approval_gates.is_empty()
            || row.rerun_review_reasons.is_empty()
            || row.rerun_admissions.is_empty()
            || row.rerun_drift_dimensions.is_empty()
            || row.replay_completeness_states.is_empty()
            || row.replay_segments.is_empty()
            || row.agent_lifecycle_states.is_empty()
            || row.takeover_paths.is_empty()
            || row.agent_blast_radii.is_empty()
        {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::RowIncomplete);
        }
        if !row.declares_mandatory_rerun_anatomy() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryRerunAnatomyMissing);
        }
        if !row.declares_mandatory_replay_anatomy() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryReplayAnatomyMissing);
        }
        if !row.declares_mandatory_agent_anatomy() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryAgentAnatomyMissing);
        }
        if !row.declares_mandatory_continue_options() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryContinueOptionMissing);
        }
        if !row.declares_mandatory_rerun_export() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryRerunExportMissing);
        }
        if !row.declares_mandatory_replay_export() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryReplayExportMissing);
        }
        if !row.declares_mandatory_agent_export() {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::MandatoryAgentExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AiAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.rerun_review_examples.is_empty() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::RerunExampleMissing);
        }
        if row.incomplete_replay_examples.is_empty() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::ReplayExampleMissing);
        }
        if row.agent_status_examples.is_empty() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::AgentExampleMissing);
        }
        if row
            .rerun_review_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .incomplete_replay_examples
                .iter()
                .any(|case| !case.is_self_consistent())
            || row
                .agent_status_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::RowInvariantViolated);
        }
    }
}

/// The same canonical AI run lineage must appear in a rerun-review example, an
/// incomplete-replay example, and an agent-status example — the acceptance-criterion that
/// manual takeover and replay/export paths preserve run lineage across UI and support
/// exports.
fn validate_run_lineage_consistency(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let rerun_ids: BTreeSet<&str> = packet
        .rows
        .iter()
        .flat_map(|row| row.rerun_review_examples.iter())
        .map(|case| case.resolved.canonical_run_id.as_str())
        .collect();
    let replay_ids: BTreeSet<&str> = packet
        .rows
        .iter()
        .flat_map(|row| row.incomplete_replay_examples.iter())
        .map(|case| case.resolved.canonical_run_id.as_str())
        .collect();
    let agent_ids: BTreeSet<&str> = packet
        .rows
        .iter()
        .flat_map(|row| row.agent_status_examples.iter())
        .map(|case| case.resolved.canonical_run_id.as_str())
        .collect();
    let shared = rerun_ids
        .iter()
        .any(|id| replay_ids.contains(id) && agent_ids.contains(id));
    if !shared {
        violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::RunLineageConsistencyUnproven);
    }
}

/// At least one rerun-review example must prove a blocked rerun with named drift — the
/// acceptance-criterion that a user can tell why rerun needs re-review and what changed since
/// the original run.
fn validate_drift_disclosure(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    if !packet
        .rows
        .iter()
        .any(|row| row.proves_blocked_rerun_with_drift())
    {
        violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::DriftDisclosureUnproven);
    }
}

/// At least one agent-status example must show an interrupted agent that is not alive and
/// still offers a safe continue option — the acceptance-criterion that interrupted or drifted
/// agents no longer appear alive or safe to resume by implication alone.
fn validate_interrupted_agent_honesty(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.agent_status_examples.iter().any(|case| {
            case.resolved.is_interrupted
                && !case.resolved.presents_as_alive
                && !case.resolved.continue_options.is_empty()
        })
    });
    if !proven {
        violations
            .push(M5AiBackgroundAgentReplayPrimitiveViolation::InterruptedAgentHonestyUnproven);
    }
}

/// At least one incomplete-replay example must prove an incomplete replay requiring new
/// approvals with named retained and missing segments — the acceptance-criterion that
/// incomplete-replay banners explain which parts were retained versus missing and why new
/// approvals are required.
fn validate_incomplete_replay_reapproval(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.incomplete_replay_examples.iter().any(|case| {
            !case.resolved.is_complete
                && case.resolved.requires_new_approvals
                && !case.resolved.retained_segments.is_empty()
                && !case.resolved.missing_segments.is_empty()
        })
    });
    if !proven {
        violations
            .push(M5AiBackgroundAgentReplayPrimitiveViolation::IncompleteReplayReapprovalUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_rerun_replay_and_agent_truth,
        review.run_lineage_consistent_across_surfaces,
        review.rerun_review_names_reason_and_drift,
        review.approval_reuse_only_when_no_relevant_drift,
        review.incomplete_replay_names_retained_and_missing,
        review.replay_completeness_never_overstated,
        review.interrupted_agent_never_presents_as_alive,
        review.interrupted_agent_offers_safe_continue,
        review.support_export_reconstructs_rerun_replay_and_agent_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.background_agent_surfaces_consume_shared_primitive,
        projection.run_lineage_reads_single_source,
        projection.rerun_admission_reads_single_source,
        projection.replay_completeness_reads_single_source,
        projection.agent_liveness_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5AiBackgroundAgentReplayPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiBackgroundAgentReplayPrimitivePacket,
    violations: &mut Vec<M5AiBackgroundAgentReplayPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiBackgroundAgentReplayPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
