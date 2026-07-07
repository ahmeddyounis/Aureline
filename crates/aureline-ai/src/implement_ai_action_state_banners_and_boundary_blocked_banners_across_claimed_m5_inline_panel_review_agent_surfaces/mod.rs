//! One reusable M5 AI action-state-banner / boundary-blocked-banner primitive:
//! execution mode, action state, scope reach, placement, approval posture, and the
//! immediate operator controls a user needs — projected the same way across every
//! claimed M5 inline, panel, review, and background-agent surface.
//!
//! Aureline's frozen AI-execution/replay component matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! names the AI action-state banner as one governed component family and freezes its
//! controlled vocabulary — the action states, the execution modes, the approval
//! gates, the surface families, the deployment lines, the consumer surfaces, the
//! accessibility routes, the qualification classes, and the downgrade triggers. This
//! module *implements* that action-state-banner contract as one reusable primitive so
//! a user can tell — from the banner and its boundary-blocked state alone — what AI
//! execution mode is active, how far it can reach, where it is placed, what approval
//! applies, and what operator controls are available, before a user mistakes an
//! explanation for a mutation or hits a generic model or tool error.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_action_state_banner`] — that takes one banner's label,
//!    execution mode, action state, scope reach, placement, approval gate, any
//!    crossed boundary, and available operator controls, and produces one
//!    [`M5ResolvedActionStateBanner`] carrying the derived banner posture (active
//!    within scope versus awaiting approval versus paused versus boundary-blocked
//!    versus completed versus failed versus idle) and — whenever a request would
//!    cross a reviewed file scope, a connector boundary, or a policy fence — a
//!    self-contained [`M5AiBoundaryBlockedBanner`] that names the exact blocked
//!    boundary and the next safe alternative rather than a generic `model error` or
//!    `tool failed`. The resolver never infers placement or approval from the mode
//!    alone, never shows a boundary crossing as allowed, and never leaves a banner
//!    without at least one operator control.
//! 2. A parity matrix — [`M5AiActionStateBannerPrimitivePacket`] — that binds one row
//!    per claimed M5 banner consumer (the inline explain/fix overlay, the assistant
//!    panel, the patch-review lane, the background branch/worktree agent, and the CLI
//!    / support export) to the shared banner anatomy, the same postures, action
//!    states, scope reaches, placements, approval gates, blocked boundaries, safe
//!    alternatives, operator controls, export fields, and non-visual accessibility
//!    routes, so the mode/scope/approval vocabulary stays identical across inline,
//!    panel, review, and agent surfaces.
//!
//! The execution mode ([`M5AiExecutionMode`]), action state ([`M5AiActionState`]),
//! approval gate ([`M5AiApprovalGate`]), surface family ([`M5AiSurfaceFamily`]),
//! deployment line ([`M5AiDeploymentLine`]), consumer surface
//! ([`M5AiConsumerSurface`]), accessibility route ([`M5AiAccessibilityRoute`]),
//! qualification class ([`M5AiQualificationClass`]), and downgrade trigger
//! ([`M5AiExecutionDowngradeTrigger`]) are reused verbatim from the frozen matrix.
//! This module mints new vocabulary only for what that matrix left implicit about the
//! banner itself: its banner consumer families, its anatomy parts, its scope reaches,
//! its action placements, its operator controls, its derived banner postures, its
//! blocked boundaries, its safe alternatives, and its export fields. No M5 AI surface
//! invents a second banner grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary; every banner label and scope descriptor is carried
//! only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-action-state-banner.schema.json`](../../../../schemas/ai/m5-ai-action-state-banner.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces.md`](../../../../docs/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_action_state_banner_primitive_branch_worktree_agent_preview_narrowed,
    seeded_m5_ai_action_state_banner_primitive_packet,
    seeded_m5_ai_action_state_banner_primitive_patch_review_beta_narrowed,
    M5_AI_ACTION_STATE_BANNER_PRIMITIVE_PACKET_ID,
};

// The execution mode, action state, approval gate, surface family, deployment line,
// consumer surface, accessibility route, qualification class, and downgrade triggers
// are frozen once, in the AI-execution/replay component matrix. This primitive reuses
// them verbatim so it never invents a parallel banner vocabulary.
pub use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5AiAccessibilityRoute, M5AiActionState, M5AiApprovalGate, M5AiConsumerSurface,
    M5AiDeploymentLine, M5AiExecutionDowngradeTrigger, M5AiExecutionMode, M5AiQualificationClass,
    M5AiSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiActionStateBannerPrimitivePacket`].
pub const M5_AI_ACTION_STATE_BANNER_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces";

/// Schema version for M5 AI action-state-banner-primitive records.
pub const M5_AI_ACTION_STATE_BANNER_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the action-state-banner boundary schema.
pub const M5_AI_ACTION_STATE_BANNER_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-action-state-banner.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_ACTION_STATE_BANNER_DOC_REF: &str =
    "docs/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces.md";

/// Repo-relative path of the frozen AI-execution/replay component matrix this
/// primitive narrows from.
pub const M5_AI_ACTION_STATE_BANNER_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json";

/// Repo-relative path of the tool-call-timeline-entry contract this primitive binds
/// its tool-boundary truth against.
pub const M5_AI_ACTION_STATE_BANNER_TOOL_REF: &str =
    "schemas/ai/tool_call_timeline_entry.schema.json";

/// Repo-relative path of the branch-agent-session contract this primitive binds its
/// background-agent placement truth against.
pub const M5_AI_ACTION_STATE_BANNER_AGENT_REF: &str = "schemas/ai/branch_agent_session.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_ACTION_STATE_BANNER_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_ACTION_STATE_BANNER_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_ACTION_STATE_BANNER_CSV_REF: &str =
    "artifacts/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_ACTION_STATE_BANNER_REPORT_REF: &str =
    "artifacts/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces.md";

/// One claimed M5 banner consumer that renders the shared action-state banner and its
/// boundary-blocked state. These are the consumers the acceptance criteria name — the
/// inline explain/fix overlay, the assistant panel, the patch-review lane, the
/// background branch/worktree agent, and the CLI / support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiBannerConsumerSurface {
    /// The inline explain/fix overlay.
    InlineExplainFix,
    /// The assistant side panel.
    AssistantPanel,
    /// The patch-review lane.
    PatchReview,
    /// The background branch / worktree agent surface.
    BranchWorktreeAgent,
    /// The CLI inspect / support export.
    CliSupportExport,
}

impl M5AiBannerConsumerSurface {
    /// Every claimed banner consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InlineExplainFix,
        Self::AssistantPanel,
        Self::PatchReview,
        Self::BranchWorktreeAgent,
        Self::CliSupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineExplainFix => "inline_explain_fix",
            Self::AssistantPanel => "assistant_panel",
            Self::PatchReview => "patch_review",
            Self::BranchWorktreeAgent => "branch_worktree_agent",
            Self::CliSupportExport => "cli_support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InlineExplainFix => "Inline Explain/Fix",
            Self::AssistantPanel => "Assistant Panel",
            Self::PatchReview => "Patch-Review Lane",
            Self::BranchWorktreeAgent => "Branch / Worktree Agent",
            Self::CliSupportExport => "CLI / Support Export",
        }
    }
}

/// One anatomy part the shared banner / boundary-blocked banner surfaces. The parts in
/// [`M5AiBannerAnatomyPart::MANDATORY`] are required on every banner so a user can
/// orient before mistaking explanation for mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiBannerAnatomyPart {
    /// The execution-mode badge.
    ExecutionModeBadge,
    /// The scope-reach cue: how far the AI can reach.
    ScopeReachCue,
    /// The placement cue: where the action is placed.
    PlacementCue,
    /// The approval-posture cue.
    ApprovalPostureCue,
    /// The action-state cue.
    ActionStateCue,
    /// The immediate operator controls.
    OperatorControls,
    /// The boundary-blocked banner (shown when a boundary is crossed).
    BoundaryBlockedBanner,
    /// The next-safe-action cue.
    NextSafeActionCue,
}

impl M5AiBannerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ExecutionModeBadge,
        Self::ScopeReachCue,
        Self::PlacementCue,
        Self::ApprovalPostureCue,
        Self::ActionStateCue,
        Self::OperatorControls,
        Self::BoundaryBlockedBanner,
        Self::NextSafeActionCue,
    ];

    /// The anatomy parts every banner must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ExecutionModeBadge,
        Self::ScopeReachCue,
        Self::ActionStateCue,
        Self::OperatorControls,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionModeBadge => "execution_mode_badge",
            Self::ScopeReachCue => "scope_reach_cue",
            Self::PlacementCue => "placement_cue",
            Self::ApprovalPostureCue => "approval_posture_cue",
            Self::ActionStateCue => "action_state_cue",
            Self::OperatorControls => "operator_controls",
            Self::BoundaryBlockedBanner => "boundary_blocked_banner",
            Self::NextSafeActionCue => "next_safe_action_cue",
        }
    }
}

/// Controlled scope reach — how far the active AI action can reach, so a banner never
/// leaves its blast radius implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionScopeReach {
    /// A single selection.
    SingleSelection,
    /// The current file.
    CurrentFile,
    /// The reviewed file set.
    ReviewedFileSet,
    /// The whole workspace.
    WorkspaceScoped,
    /// A connector's scope.
    ConnectorScoped,
    /// Across workspaces / tenants.
    CrossWorkspaceScoped,
}

impl M5AiExecutionScopeReach {
    /// Every scope reach, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleSelection,
        Self::CurrentFile,
        Self::ReviewedFileSet,
        Self::WorkspaceScoped,
        Self::ConnectorScoped,
        Self::CrossWorkspaceScoped,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSelection => "single_selection",
            Self::CurrentFile => "current_file",
            Self::ReviewedFileSet => "reviewed_file_set",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::ConnectorScoped => "connector_scoped",
            Self::CrossWorkspaceScoped => "cross_workspace_scoped",
        }
    }
}

/// Controlled action placement — where an AI action is placed, so a banner never
/// leaves its placement to be inferred from the mode alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiActionPlacement {
    /// An inline overlay.
    InlineOverlay,
    /// The assistant side panel.
    AssistantSidePanel,
    /// The review lane.
    ReviewLane,
    /// A background branch / worktree.
    BackgroundBranchWorktree,
    /// The tool-run timeline.
    ToolRunTimeline,
}

impl M5AiActionPlacement {
    /// Every action placement, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InlineOverlay,
        Self::AssistantSidePanel,
        Self::ReviewLane,
        Self::BackgroundBranchWorktree,
        Self::ToolRunTimeline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineOverlay => "inline_overlay",
            Self::AssistantSidePanel => "assistant_side_panel",
            Self::ReviewLane => "review_lane",
            Self::BackgroundBranchWorktree => "background_branch_worktree",
            Self::ToolRunTimeline => "tool_run_timeline",
        }
    }
}

/// Controlled operator control — an immediate control a banner offers so a user is
/// never stuck watching an AI action with no way to steer it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiOperatorControl {
    /// Open the plan.
    OpenPlan,
    /// Pause the run.
    Pause,
    /// Cancel the run.
    Cancel,
    /// Resume the run.
    Resume,
    /// Take over manually.
    TakeOver,
    /// Narrow the scope.
    NarrowScope,
}

impl M5AiOperatorControl {
    /// Every operator control, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenPlan,
        Self::Pause,
        Self::Cancel,
        Self::Resume,
        Self::TakeOver,
        Self::NarrowScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenPlan => "open_plan",
            Self::Pause => "pause",
            Self::Cancel => "cancel",
            Self::Resume => "resume",
            Self::TakeOver => "take_over",
            Self::NarrowScope => "narrow_scope",
        }
    }

    /// True when this is an immediate steering control the acceptance criteria call
    /// out (open plan, pause, or cancel).
    pub const fn is_immediate_steering_control(self) -> bool {
        matches!(self, Self::OpenPlan | Self::Pause | Self::Cancel)
    }
}

/// Controlled blocked boundary — the exact fence a request would cross, so a
/// boundary-blocked banner never reads like a generic model or tool error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiBlockedBoundary {
    /// The request would leave the reviewed file scope.
    ReviewedFileScope,
    /// The request would cross a connector boundary.
    ConnectorBoundary,
    /// The request would cross a policy fence.
    PolicyFence,
    /// The request would cross a credential boundary.
    CredentialBoundary,
    /// The request would cross a network egress fence.
    NetworkEgressFence,
    /// The request would cross into another workspace / tenant.
    CrossWorkspaceScope,
}

impl M5AiBlockedBoundary {
    /// Every blocked boundary, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewedFileScope,
        Self::ConnectorBoundary,
        Self::PolicyFence,
        Self::CredentialBoundary,
        Self::NetworkEgressFence,
        Self::CrossWorkspaceScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewedFileScope => "reviewed_file_scope",
            Self::ConnectorBoundary => "connector_boundary",
            Self::PolicyFence => "policy_fence",
            Self::CredentialBoundary => "credential_boundary",
            Self::NetworkEgressFence => "network_egress_fence",
            Self::CrossWorkspaceScope => "cross_workspace_scope",
        }
    }

    /// Review-safe phrase naming the boundary for a banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ReviewedFileScope => "the reviewed file scope",
            Self::ConnectorBoundary => "a connector boundary",
            Self::PolicyFence => "a policy fence",
            Self::CredentialBoundary => "a credential boundary",
            Self::NetworkEgressFence => "a network egress fence",
            Self::CrossWorkspaceScope => "the cross-workspace boundary",
        }
    }

    /// The narrower safe alternative a user can take when this boundary blocks.
    pub const fn safe_alternative(self) -> M5AiSafeAlternative {
        match self {
            Self::ReviewedFileScope => M5AiSafeAlternative::NarrowToReviewedScope,
            Self::ConnectorBoundary => M5AiSafeAlternative::RequestConnectorApproval,
            Self::PolicyFence => M5AiSafeAlternative::SplitIntoApprovedSteps,
            Self::CredentialBoundary => M5AiSafeAlternative::RequestScopedCredential,
            Self::NetworkEgressFence => M5AiSafeAlternative::RunReadOnlyPreview,
            Self::CrossWorkspaceScope => M5AiSafeAlternative::StayWithinCurrentWorkspace,
        }
    }
}

/// Controlled safe alternative — the narrower, safe next action named on a
/// boundary-blocked banner, so a blocked state is always actionable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSafeAlternative {
    /// Narrow the request to the reviewed file scope.
    NarrowToReviewedScope,
    /// Request connector approval for this capability.
    RequestConnectorApproval,
    /// Request a scoped credential grant.
    RequestScopedCredential,
    /// Split the request into individually approved steps.
    SplitIntoApprovedSteps,
    /// Stay within the current workspace.
    StayWithinCurrentWorkspace,
    /// Run a read-only preview first.
    RunReadOnlyPreview,
}

impl M5AiSafeAlternative {
    /// Every safe alternative, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NarrowToReviewedScope,
        Self::RequestConnectorApproval,
        Self::RequestScopedCredential,
        Self::SplitIntoApprovedSteps,
        Self::StayWithinCurrentWorkspace,
        Self::RunReadOnlyPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowToReviewedScope => "narrow_to_reviewed_scope",
            Self::RequestConnectorApproval => "request_connector_approval",
            Self::RequestScopedCredential => "request_scoped_credential",
            Self::SplitIntoApprovedSteps => "split_into_approved_steps",
            Self::StayWithinCurrentWorkspace => "stay_within_current_workspace",
            Self::RunReadOnlyPreview => "run_read_only_preview",
        }
    }

    /// Review-safe phrase naming the safe next action for a banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::NarrowToReviewedScope => "narrow the request to the reviewed file scope",
            Self::RequestConnectorApproval => "request connector approval for this capability",
            Self::RequestScopedCredential => "request a scoped credential grant",
            Self::SplitIntoApprovedSteps => "split the request into individually approved steps",
            Self::StayWithinCurrentWorkspace => "stay within the current workspace",
            Self::RunReadOnlyPreview => "run a read-only preview first",
        }
    }
}

/// The derived headline posture of a banner — the resolver's verdict about what a user
/// is looking at: an active-within-scope action, one awaiting approval, a paused run,
/// a boundary-blocked request, a completed run, a failed run, or an idle banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiBannerPosture {
    /// Active and within its declared scope.
    ActiveWithinScope,
    /// Active but awaiting a human approval.
    ActiveAwaitingApproval,
    /// Paused mid-run.
    PausedMidRun,
    /// Blocked at an execution boundary.
    BoundaryBlocked,
    /// Completed cleanly.
    CompletedClear,
    /// Failed and needs attention.
    FailedNeedsAttention,
    /// Idle and ready.
    IdleReady,
}

impl M5AiBannerPosture {
    /// Every banner posture, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ActiveWithinScope,
        Self::ActiveAwaitingApproval,
        Self::PausedMidRun,
        Self::BoundaryBlocked,
        Self::CompletedClear,
        Self::FailedNeedsAttention,
        Self::IdleReady,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveWithinScope => "active_within_scope",
            Self::ActiveAwaitingApproval => "active_awaiting_approval",
            Self::PausedMidRun => "paused_mid_run",
            Self::BoundaryBlocked => "boundary_blocked",
            Self::CompletedClear => "completed_clear",
            Self::FailedNeedsAttention => "failed_needs_attention",
            Self::IdleReady => "idle_ready",
        }
    }

    /// True when the AI action is actively running (with or without a pending
    /// approval).
    pub const fn is_active(self) -> bool {
        matches!(self, Self::ActiveWithinScope | Self::ActiveAwaitingApproval)
    }

    /// True when the request is blocked at an execution boundary.
    pub const fn is_boundary_blocked(self) -> bool {
        matches!(self, Self::BoundaryBlocked)
    }

    /// True when the posture needs immediate operator attention.
    pub const fn needs_operator_attention(self) -> bool {
        matches!(
            self,
            Self::ActiveAwaitingApproval
                | Self::PausedMidRun
                | Self::BoundaryBlocked
                | Self::FailedNeedsAttention
        )
    }
}

/// A field the support / export packet carries so banner truth is reconstructable from
/// the shared model. The fields in [`M5AiBannerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiBannerExportField {
    /// The execution mode.
    ExecutionMode,
    /// The action state.
    ActionState,
    /// The scope reach.
    ScopeReach,
    /// The placement.
    Placement,
    /// The approval gate.
    ApprovalGate,
    /// The derived banner posture.
    BannerPosture,
    /// The blocked boundary (when boundary-blocked).
    BlockedBoundary,
    /// The safe alternative (when boundary-blocked).
    SafeAlternative,
    /// The operator controls.
    OperatorControls,
    /// The opaque banner label.
    BannerLabel,
}

impl M5AiBannerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ExecutionMode,
        Self::ActionState,
        Self::ScopeReach,
        Self::Placement,
        Self::ApprovalGate,
        Self::BannerPosture,
        Self::BlockedBoundary,
        Self::SafeAlternative,
        Self::OperatorControls,
        Self::BannerLabel,
    ];

    /// The export fields every banner export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ExecutionMode,
        Self::ActionState,
        Self::ScopeReach,
        Self::BannerPosture,
        Self::ApprovalGate,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionMode => "execution_mode",
            Self::ActionState => "action_state",
            Self::ScopeReach => "scope_reach",
            Self::Placement => "placement",
            Self::ApprovalGate => "approval_gate",
            Self::BannerPosture => "banner_posture",
            Self::BlockedBoundary => "blocked_boundary",
            Self::SafeAlternative => "safe_alternative",
            Self::OperatorControls => "operator_controls",
            Self::BannerLabel => "banner_label",
        }
    }
}

/// A self-contained boundary-blocked banner: the exact blocked boundary, the safe
/// alternative, the scope reach, and the placement, so a blocked AI request is
/// understood from the banner alone rather than from a generic model or tool error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBoundaryBlockedBanner {
    /// The exact blocked boundary.
    pub blocked_boundary: M5AiBlockedBoundary,
    /// The narrower safe alternative a user can take.
    pub safe_alternative: M5AiSafeAlternative,
    /// The scope reach the blocked request would have exceeded.
    pub scope_reach: M5AiExecutionScopeReach,
    /// The placement the block applies to.
    pub placement: M5AiActionPlacement,
    /// A deterministic, self-contained headline naming the boundary, the reach, and
    /// the safe next action — never a generic `model error` or `tool failed`.
    pub headline: String,
}

/// The full input to the action-state-banner resolver for one banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBannerResolutionInput {
    /// The opaque, export-safe banner label.
    pub banner_label: String,
    /// The opaque, export-safe scope descriptor.
    pub scope_repr: String,
    /// The execution mode behind the banner (never inferred from placement).
    pub execution_mode: M5AiExecutionMode,
    /// The current action state.
    pub action_state: M5AiActionState,
    /// The scope reach of the active action.
    pub scope_reach: M5AiExecutionScopeReach,
    /// Where the action is placed.
    pub placement: M5AiActionPlacement,
    /// The approval gate behind the action.
    pub approval_gate: M5AiApprovalGate,
    /// The boundary a request would cross, when one is blocked.
    pub blocked_boundary: Option<M5AiBlockedBoundary>,
    /// The immediate operator controls the banner offers. Must be non-empty so a user
    /// is never stuck.
    pub operator_controls: Vec<M5AiOperatorControl>,
}

/// The resolved banner / boundary-blocked truth for one AI action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedActionStateBanner {
    /// The opaque banner label.
    pub banner_label: String,
    /// The opaque scope descriptor.
    pub scope_repr: String,
    /// The execution mode.
    pub execution_mode: M5AiExecutionMode,
    /// The action state.
    pub action_state: M5AiActionState,
    /// The scope reach.
    pub scope_reach: M5AiExecutionScopeReach,
    /// The placement.
    pub placement: M5AiActionPlacement,
    /// The approval gate.
    pub approval_gate: M5AiApprovalGate,
    /// The blocked boundary, when one is crossed.
    pub blocked_boundary: Option<M5AiBlockedBoundary>,
    /// The immediate operator controls.
    pub operator_controls: Vec<M5AiOperatorControl>,
    /// The derived banner posture.
    pub banner_posture: M5AiBannerPosture,
    /// True when the AI action is actively running.
    pub is_active: bool,
    /// True when the request is boundary-blocked.
    pub is_boundary_blocked: bool,
    /// True when the posture needs immediate operator attention.
    pub needs_operator_attention: bool,
    /// The boundary-blocked banner, present when boundary-blocked.
    pub boundary_banner: Option<M5AiBoundaryBlockedBanner>,
}

/// Errors returned by [`resolve_action_state_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiBannerResolutionError {
    /// The banner label was empty.
    EmptyBannerLabel,
    /// The scope descriptor was empty.
    EmptyScopeRepr,
    /// The operator controls were empty (a user must never be stuck).
    EmptyOperatorControls,
    /// The action is at a boundary (blocked action state or policy-blocked gate) but
    /// no blocked boundary was named.
    BoundaryBlockedWithoutBoundary,
    /// A banner label or scope descriptor carried forbidden material.
    ForbiddenBannerMaterial,
}

impl M5AiBannerResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBannerLabel => "empty_banner_label",
            Self::EmptyScopeRepr => "empty_scope_repr",
            Self::EmptyOperatorControls => "empty_operator_controls",
            Self::BoundaryBlockedWithoutBoundary => "boundary_blocked_without_boundary",
            Self::ForbiddenBannerMaterial => "forbidden_banner_material",
        }
    }
}

impl fmt::Display for M5AiBannerResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "ai banner resolution error: {}", self.as_str())
    }
}

impl Error for M5AiBannerResolutionError {}

/// Resolves one AI action-state banner from its declared state.
///
/// The derived banner posture is the headline verdict, computed in a fixed
/// blocking-first order: a crossed boundary blocks first (and the banner names the
/// exact boundary and safe alternative), then a failed action needs attention, then a
/// paused run, then an action awaiting approval (either explicitly awaiting or behind a
/// high-friction / two-person gate), then an actively-running action within scope, then
/// a completed action, then an idle banner. Placement and approval are carried
/// explicitly — never inferred from the mode — and a boundary-blocked request always
/// produces a self-contained banner rather than a generic model or tool error.
pub fn resolve_action_state_banner(
    input: &M5AiBannerResolutionInput,
) -> Result<M5ResolvedActionStateBanner, M5AiBannerResolutionError> {
    if input.banner_label.trim().is_empty() {
        return Err(M5AiBannerResolutionError::EmptyBannerLabel);
    }
    if input.scope_repr.trim().is_empty() {
        return Err(M5AiBannerResolutionError::EmptyScopeRepr);
    }
    if input.operator_controls.is_empty() {
        return Err(M5AiBannerResolutionError::EmptyOperatorControls);
    }
    if value_repr_is_forbidden(&input.banner_label) || value_repr_is_forbidden(&input.scope_repr) {
        return Err(M5AiBannerResolutionError::ForbiddenBannerMaterial);
    }
    // A blocked action state or a policy-blocked approval gate is an execution
    // boundary — the banner must name which boundary rather than fail generically.
    let at_boundary = matches!(input.action_state, M5AiActionState::BoundaryBlocked)
        || matches!(input.approval_gate, M5AiApprovalGate::PolicyBlocked);
    if at_boundary && input.blocked_boundary.is_none() {
        return Err(M5AiBannerResolutionError::BoundaryBlockedWithoutBoundary);
    }

    let banner_posture = derive_banner_posture(
        input.action_state,
        input.approval_gate,
        input.blocked_boundary,
    );

    let is_active = banner_posture.is_active();
    let is_boundary_blocked = banner_posture.is_boundary_blocked();
    let needs_operator_attention = banner_posture.needs_operator_attention();

    let boundary_banner = if is_boundary_blocked {
        let blocked_boundary = input
            .blocked_boundary
            .expect("boundary-blocked posture always carries a blocked boundary");
        let safe_alternative = blocked_boundary.safe_alternative();
        let headline = format!(
            "AI blocked at {}: request would exceed {} reach; safe next: {}",
            blocked_boundary.phrase(),
            input.scope_reach.as_str(),
            safe_alternative.phrase()
        );
        Some(M5AiBoundaryBlockedBanner {
            blocked_boundary,
            safe_alternative,
            scope_reach: input.scope_reach,
            placement: input.placement,
            headline,
        })
    } else {
        None
    };

    Ok(M5ResolvedActionStateBanner {
        banner_label: input.banner_label.clone(),
        scope_repr: input.scope_repr.clone(),
        execution_mode: input.execution_mode,
        action_state: input.action_state,
        scope_reach: input.scope_reach,
        placement: input.placement,
        approval_gate: input.approval_gate,
        blocked_boundary: input.blocked_boundary,
        operator_controls: input.operator_controls.clone(),
        banner_posture,
        is_active,
        is_boundary_blocked,
        needs_operator_attention,
        boundary_banner,
    })
}

/// The fixed blocking-first banner-posture ladder.
fn derive_banner_posture(
    action_state: M5AiActionState,
    approval_gate: M5AiApprovalGate,
    blocked_boundary: Option<M5AiBlockedBoundary>,
) -> M5AiBannerPosture {
    let at_boundary = blocked_boundary.is_some()
        || matches!(action_state, M5AiActionState::BoundaryBlocked)
        || matches!(approval_gate, M5AiApprovalGate::PolicyBlocked);
    if at_boundary {
        M5AiBannerPosture::BoundaryBlocked
    } else if matches!(action_state, M5AiActionState::Failed) {
        M5AiBannerPosture::FailedNeedsAttention
    } else if matches!(action_state, M5AiActionState::Paused) {
        M5AiBannerPosture::PausedMidRun
    } else if matches!(action_state, M5AiActionState::AwaitingApproval)
        || matches!(
            approval_gate,
            M5AiApprovalGate::HighFrictionTyped | M5AiApprovalGate::TwoPersonReview
        )
    {
        M5AiBannerPosture::ActiveAwaitingApproval
    } else if matches!(
        action_state,
        M5AiActionState::Composing | M5AiActionState::Streaming | M5AiActionState::ToolRunning
    ) {
        M5AiBannerPosture::ActiveWithinScope
    } else if matches!(action_state, M5AiActionState::Completed) {
        M5AiBannerPosture::CompletedClear
    } else {
        M5AiBannerPosture::IdleReady
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs banner truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiBannerResolutionCase {
    /// The resolver input.
    pub input: M5AiBannerResolutionInput,
    /// The resolved truth. Must equal `resolve_action_state_banner(&input)`.
    pub resolved: M5ResolvedActionStateBanner,
}

impl M5AiBannerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiBannerResolutionInput) -> Self {
        let resolved = resolve_action_state_banner(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_action_state_banner(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one banner consumer bound to the shared banner
/// anatomy, postures, action states, scope reaches, placements, approval gates,
/// blocked boundaries, safe alternatives, operator controls, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiActionStateBannerRow {
    /// Banner consumer family.
    pub consumer_surface: M5AiBannerConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5AiQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 AI surface families that render / consume this banner.
    pub surface_families: Vec<M5AiSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5AiDeploymentLine>,
    /// Anatomy parts this banner renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5AiBannerAnatomyPart>,
    /// Execution modes this banner names.
    pub execution_modes: Vec<M5AiExecutionMode>,
    /// Action states this banner distinguishes.
    pub action_states: Vec<M5AiActionState>,
    /// Scope reaches this banner distinguishes.
    pub scope_reaches: Vec<M5AiExecutionScopeReach>,
    /// Action placements this banner distinguishes.
    pub placements: Vec<M5AiActionPlacement>,
    /// Approval gates this banner distinguishes.
    pub approval_gates: Vec<M5AiApprovalGate>,
    /// Banner postures this banner distinguishes.
    pub banner_postures: Vec<M5AiBannerPosture>,
    /// Blocked boundaries this banner names.
    pub blocked_boundaries: Vec<M5AiBlockedBoundary>,
    /// Safe alternatives this banner names.
    pub safe_alternatives: Vec<M5AiSafeAlternative>,
    /// Operator controls this banner offers.
    pub operator_controls: Vec<M5AiOperatorControl>,
    /// Export fields this banner carries (must include the mandatory fields).
    pub export_fields: Vec<M5AiBannerExportField>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5AiAccessibilityRoute>,
    /// AI subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5AiConsumerSurface>,
    /// Downgrade triggers that apply to this banner.
    pub downgrade_triggers: Vec<M5AiExecutionDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5AiBannerResolutionCase>,
    /// Hard invariant: this banner never masks its execution mode or scope reach.
    /// MUST be `false`.
    pub masks_execution_mode_or_reach: bool,
    /// Hard invariant: this banner never shows a boundary crossing as allowed. MUST be
    /// `false`.
    pub shows_boundary_crossing_as_allowed: bool,
    /// Hard invariant: this banner never emits a generic model or tool error in place
    /// of a named boundary. MUST be `false`.
    pub emits_generic_model_or_tool_error: bool,
    /// Hard invariant: this banner never hides its operator controls or takeover path.
    /// MUST be `false`.
    pub hides_operator_controls_or_takeover: bool,
}

impl M5AiActionStateBannerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiBannerAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5AiBannerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AiBannerExportField> = self.export_fields.iter().copied().collect();
        M5AiBannerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_execution_mode_or_reach
            && !self.shows_boundary_crossing_as_allowed
            && !self.emits_generic_model_or_tool_error
            && !self.hides_operator_controls_or_takeover
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiActionStateBannerVocabularySet {
    /// Banner-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Scope-reach tokens.
    pub scope_reaches: Vec<String>,
    /// Action-placement tokens.
    pub placements: Vec<String>,
    /// Operator-control tokens.
    pub operator_controls: Vec<String>,
    /// Banner-posture tokens.
    pub banner_postures: Vec<String>,
    /// Blocked-boundary tokens.
    pub blocked_boundaries: Vec<String>,
    /// Safe-alternative tokens.
    pub safe_alternatives: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Execution-mode tokens (reused from the frozen matrix).
    pub execution_modes: Vec<String>,
    /// Action-state tokens (reused from the frozen matrix).
    pub action_states: Vec<String>,
    /// Approval-gate tokens (reused from the frozen matrix).
    pub approval_gates: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AiActionStateBannerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5AiBannerConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5AiBannerAnatomyPart::ALL, |v| v.as_str()),
            scope_reaches: tokens(&M5AiExecutionScopeReach::ALL, |v| v.as_str()),
            placements: tokens(&M5AiActionPlacement::ALL, |v| v.as_str()),
            operator_controls: tokens(&M5AiOperatorControl::ALL, |v| v.as_str()),
            banner_postures: tokens(&M5AiBannerPosture::ALL, |v| v.as_str()),
            blocked_boundaries: tokens(&M5AiBlockedBoundary::ALL, |v| v.as_str()),
            safe_alternatives: tokens(&M5AiSafeAlternative::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AiBannerExportField::ALL, |v| v.as_str()),
            execution_modes: tokens(&M5AiExecutionMode::ALL, |v| v.as_str()),
            action_states: tokens(&M5AiActionState::ALL, |v| v.as_str()),
            approval_gates: tokens(&M5AiApprovalGate::ALL, |v| v.as_str()),
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
pub struct M5AiActionStateBannerGovernanceReview {
    /// One banner primitive carries mode, scope, placement, approval, and control
    /// truth on every consumer.
    pub one_primitive_carries_banner_truth: bool,
    /// The execution mode and scope reach are shown without a secondary inspector.
    pub mode_and_reach_always_shown: bool,
    /// Placement and approval are explicit, never inferred from the mode.
    pub placement_and_approval_never_inferred: bool,
    /// A boundary crossing is never shown as allowed.
    pub boundary_crossing_never_shown_as_allowed: bool,
    /// Immediate operator controls are always present.
    pub operator_controls_always_present: bool,
    /// A boundary-blocked request always shows a self-contained banner.
    pub boundary_blocked_always_shows_self_contained_banner: bool,
    /// The banner names an exact boundary and safe action, never a generic error.
    pub banner_names_boundary_and_safe_action: bool,
    /// The support / export packet reconstructs banner truth.
    pub support_export_reconstructs_banner_truth: bool,
    /// No consumer invents a second banner grammar.
    pub no_surface_invents_second_banner_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel banner vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiActionStateBannerConsumerProjection {
    /// Inline, panel, review, agent, and CLI/support consumers all consume the shared
    /// primitive.
    pub banner_surfaces_consume_shared_primitive: bool,
    /// The posture resolver reads a single canonical source.
    pub posture_resolver_reads_single_source: bool,
    /// The scope-reach cue reads a single canonical source.
    pub scope_reach_reads_single_source: bool,
    /// The boundary-blocked banner reads a single canonical source.
    pub boundary_banner_reads_single_source: bool,
    /// Support / export reads a single canonical banner source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiActionStateBannerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the banner primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiActionStateBannerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiActionStateBannerPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiActionStateBannerPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Banner rows.
    pub banner_rows: Vec<M5AiActionStateBannerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiActionStateBannerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiActionStateBannerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiActionStateBannerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiActionStateBannerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiActionStateBannerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 action-state-banner-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiActionStateBannerPrimitivePacket {
    /// Record kind; must equal [`M5_AI_ACTION_STATE_BANNER_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_ACTION_STATE_BANNER_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Banner rows.
    pub banner_rows: Vec<M5AiActionStateBannerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiActionStateBannerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiActionStateBannerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiActionStateBannerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiActionStateBannerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiActionStateBannerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiActionStateBannerPrimitivePacket {
    /// Builds an M5 action-state-banner-primitive packet from stable-lane input.
    pub fn new(input: M5AiActionStateBannerPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_AI_ACTION_STATE_BANNER_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_AI_ACTION_STATE_BANNER_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            banner_rows: input.banner_rows,
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

    /// Validates the M5 action-state-banner-primitive invariants.
    pub fn validate(&self) -> Vec<M5AiActionStateBannerPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_ACTION_STATE_BANNER_PRIMITIVE_RECORD_KIND {
            violations.push(M5AiActionStateBannerPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_ACTION_STATE_BANNER_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AiActionStateBannerPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiActionStateBannerPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_banner_rows(self, &mut violations);
        validate_posture_coverage(self, &mut violations);
        validate_mode_and_reach_explicit(self, &mut violations);
        validate_boundary_blocked_self_contained(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai action-state-banner primitive packet serializes"),
        ) {
            violations.push(M5AiActionStateBannerPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 ai action-state-banner primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per banner consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,banner_postures,scope_reaches,blocked_boundaries,safe_alternatives,export_fields,example_count\n",
        );
        for row in &self.banner_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.banner_postures, |v| v.as_str()),
                join_tokens(&row.scope_reaches, |v| v.as_str()),
                join_tokens(&row.blocked_boundaries, |v| v.as_str()),
                join_tokens(&row.safe_alternatives, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .banner_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 AI Action-State-Banner and Boundary-Blocked-Banner Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Banner consumers: {} ({} stable)\n",
            self.banner_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Banner postures: {}\n",
            self.vocabulary_set.banner_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Scope reaches: {}\n",
            self.vocabulary_set.scope_reaches.join(", ")
        ));
        out.push_str(&format!(
            "- Blocked boundaries: {}\n",
            self.vocabulary_set.blocked_boundaries.join(", ")
        ));
        out.push_str(&format!(
            "- Safe alternatives: {}\n",
            self.vocabulary_set.safe_alternatives.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Banner consumers\n\n");
        for row in &self.banner_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let banner = match &case.resolved.boundary_banner {
                    Some(banner) => banner.blocked_boundary.as_str(),
                    None => "clear",
                };
                out.push_str(&format!(
                    "    - `{}` in `{}` → `{}` (reach `{}`, gate `{}`, boundary `{}`)\n",
                    case.resolved.action_state.as_str(),
                    case.resolved.execution_mode.as_str(),
                    case.resolved.banner_posture.as_str(),
                    case.resolved.scope_reach.as_str(),
                    case.resolved.approval_gate.as_str(),
                    banner
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 action-state-banner-primitive export.
#[derive(Debug)]
pub enum M5AiActionStateBannerPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiActionStateBannerPrimitiveViolation>),
}

impl fmt::Display for M5AiActionStateBannerPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai action-state-banner primitive export parse failed: {error}"
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
                    "m5 ai action-state-banner primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiActionStateBannerPrimitiveArtifactError {}

/// Validation failures emitted by [`M5AiActionStateBannerPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiActionStateBannerPrimitiveViolation {
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
    /// A required banner consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A banner row is incomplete.
    BannerRowIncomplete,
    /// A banner row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A banner row declares no execution modes.
    ExecutionModeMissing,
    /// A banner row declares no banner postures.
    BannerPostureMissing,
    /// A banner row declares no scope reaches.
    ScopeReachMissing,
    /// A banner row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A banner row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A banner row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A banner row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A banner row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A banner claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both an active and a boundary-blocked banner.
    PostureCoverageUnproven,
    /// No worked resolution proves an active banner showing mode, reach, and an
    /// immediate operator control.
    ModeAndReachExplicitUnproven,
    /// No worked resolution proves a boundary-blocked banner with a self-contained
    /// boundary and safe alternative.
    BoundaryBlockedSelfContainedUnproven,
    /// A banner row violates a hard invariant.
    BannerInvariantViolated,
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

impl M5AiActionStateBannerPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::BannerRowIncomplete => "banner_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::ExecutionModeMissing => "execution_mode_missing",
            Self::BannerPostureMissing => "banner_posture_missing",
            Self::ScopeReachMissing => "scope_reach_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::PostureCoverageUnproven => "posture_coverage_unproven",
            Self::ModeAndReachExplicitUnproven => "mode_and_reach_explicit_unproven",
            Self::BoundaryBlockedSelfContainedUnproven => {
                "boundary_blocked_self_contained_unproven"
            }
            Self::BannerInvariantViolated => "banner_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 action-state-banner-primitive export.
pub fn current_stable_m5_ai_action_state_banner_primitive_export(
) -> Result<M5AiActionStateBannerPrimitivePacket, M5AiActionStateBannerPrimitiveArtifactError> {
    let packet: M5AiActionStateBannerPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces/support_export.json"
    )))
    .map_err(M5AiActionStateBannerPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiActionStateBannerPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_ACTION_STATE_BANNER_SCHEMA_REF,
        M5_AI_ACTION_STATE_BANNER_DOC_REF,
        M5_AI_ACTION_STATE_BANNER_COMPONENT_MATRIX_REF,
        M5_AI_ACTION_STATE_BANNER_TOOL_REF,
        M5_AI_ACTION_STATE_BANNER_AGENT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiActionStateBannerPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiActionStateBannerPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_banner_rows(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let present: BTreeSet<M5AiBannerConsumerSurface> = packet
        .banner_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5AiBannerConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AiActionStateBannerPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.banner_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.placements.is_empty()
            || row.approval_gates.is_empty()
            || row.action_states.is_empty()
            || row.blocked_boundaries.is_empty()
            || row.safe_alternatives.is_empty()
            || row.operator_controls.is_empty()
        {
            violations.push(M5AiActionStateBannerPrimitiveViolation::BannerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.execution_modes.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::ExecutionModeMissing);
        }
        if row.banner_postures.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::BannerPostureMissing);
        }
        if row.scope_reaches.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::ScopeReachMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AiAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AiActionStateBannerPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AiActionStateBannerPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiActionStateBannerPrimitiveViolation::BannerInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove an active banner and at
/// least one must prove a boundary-blocked banner — the acceptance-criterion example
/// that a user can tell an active action from a blocked one.
fn validate_posture_coverage(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let has_active = packet.banner_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_active)
    });
    let has_boundary_blocked = packet.banner_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_boundary_blocked)
    });
    if !(has_active && has_boundary_blocked) {
        violations.push(M5AiActionStateBannerPrimitiveViolation::PostureCoverageUnproven);
    }
}

/// At least one worked resolution across the matrix must prove an active banner that
/// shows its execution mode, its scope reach, and an immediate operator control (open
/// plan, pause, or cancel) — the acceptance-criterion example that a user can tell the
/// active mode and reach without opening a secondary inspector.
fn validate_mode_and_reach_explicit(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let proven = packet.banner_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_active
                && case
                    .resolved
                    .operator_controls
                    .iter()
                    .any(|control| control.is_immediate_steering_control())
        })
    });
    if !proven {
        violations.push(M5AiActionStateBannerPrimitiveViolation::ModeAndReachExplicitUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a boundary-blocked
/// banner that names its boundary, its safe alternative, and a non-empty headline — the
/// acceptance-criterion example that a blocked case names the boundary and the next
/// safe action rather than failing with a generic model or tool error.
fn validate_boundary_blocked_self_contained(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let proven = packet.banner_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_boundary_blocked
                && case
                    .resolved
                    .boundary_banner
                    .as_ref()
                    .is_some_and(|banner| {
                        !banner.headline.trim().is_empty()
                            && banner.safe_alternative == banner.blocked_boundary.safe_alternative()
                    })
        })
    });
    if !proven {
        violations
            .push(M5AiActionStateBannerPrimitiveViolation::BoundaryBlockedSelfContainedUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_banner_truth,
        review.mode_and_reach_always_shown,
        review.placement_and_approval_never_inferred,
        review.boundary_crossing_never_shown_as_allowed,
        review.operator_controls_always_present,
        review.boundary_blocked_always_shows_self_contained_banner,
        review.banner_names_boundary_and_safe_action,
        review.support_export_reconstructs_banner_truth,
        review.no_surface_invents_second_banner_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5AiActionStateBannerPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.banner_surfaces_consume_shared_primitive,
        projection.posture_resolver_reads_single_source,
        projection.scope_reach_reads_single_source,
        projection.boundary_banner_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5AiActionStateBannerPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiActionStateBannerPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiActionStateBannerPrimitivePacket,
    violations: &mut Vec<M5AiActionStateBannerPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiActionStateBannerPrimitiveViolation::ReleasePostureIncomplete);
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
