//! Two reusable M5 AI tool-governance primitives — the high-friction approval sheet and
//! the tool-call timeline row — so mutating and boundary-crossing AI / tool actions stay
//! review-first and attributable on first-class surfaces.
//!
//! Aureline's frozen AI-execution/replay component matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! names the approval sheet and the tool-call timeline row as two governed component
//! families and freezes their controlled vocabulary — the approval gates, the friction
//! reasons, the tool boundaries, the side-effect classes, the run outcomes, the surface
//! families, the deployment lines, the consumer surfaces, the accessibility routes, the
//! qualification classes, and the downgrade triggers. This module *implements* those two
//! contracts as reusable primitives so a user can tell — from the sheet or the row alone
//! — what action is requested, what scope and side effects it carries, which boundary it
//! crosses, what rollback or checkpoint backs it, and which governed follow-up actions
//! (open output, remove from context, view provenance) apply, before a mutating or
//! boundary-crossing action can slip by as an ordinary status update.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_approval_sheet`] — takes one requested action's opaque label, its scope,
//!    side-effect class, tool boundary, friction reasons, rollback posture, checkpoint
//!    presence, and declared approval gate, and produces one [`M5ResolvedApprovalSheet`]
//!    carrying the effective (friction-escalated) approval gate, whether the action
//!    requires a review sheet, whether it is high-friction, and the explicit
//!    approve-once / deny / open-plan (and rollback / escalation) controls. It never lets
//!    a mutating or boundary-crossing action read as an auto-approved status update.
//! 2. [`resolve_tool_call_timeline_row`] — takes one tool call's opaque time and tool
//!    labels, its boundary, predicted and observed side-effect classes, run outcome,
//!    output availability, and whether it is still loaded in context, and produces one
//!    [`M5ResolvedToolCallTimelineRow`] carrying whether its observed effect escalated
//!    beyond the prediction, whether it mutated or crossed a boundary, and the governed
//!    follow-up actions — provenance and removal controls stay visible instead of buried
//!    inside a raw log.
//!
//! A single parity matrix — [`M5AiApprovalToolCallPrimitivePacket`] — binds one row per
//! claimed M5 tool lane (read-only tool invocations, mutating tool runs,
//! test-generation validations, branch-agent checkpoints, and the CLI / support export)
//! to the shared approval-sheet and tool-call anatomy, the same approval gates, friction
//! reasons, action scopes, side-effect classes, tool boundaries, rollback postures, run
//! outcomes, approval controls, follow-up actions, export fields, and non-visual
//! accessibility routes, so the action-class and rollback vocabulary stays identical
//! across every lane and matches the policy and evidence systems.
//!
//! The approval gate ([`M5AiApprovalGate`]), friction reason ([`M5AiFrictionReason`]),
//! tool boundary ([`M5AiToolBoundary`]), side-effect class ([`M5AiSideEffectClass`]),
//! run outcome ([`M5AiRunOutcome`]), surface family ([`M5AiSurfaceFamily`]), deployment
//! line ([`M5AiDeploymentLine`]), consumer surface ([`M5AiConsumerSurface`]),
//! accessibility route ([`M5AiAccessibilityRoute`]), qualification class
//! ([`M5AiQualificationClass`]), and downgrade trigger
//! ([`M5AiExecutionDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the sheet
//! and the row themselves: their tool lanes, their anatomy parts, their action scopes,
//! their rollback postures, their approval controls, their follow-up actions, and their
//! export fields. No M5 AI surface invents a second approval or tool-call grammar.
//!
//! Raw prompt bodies, raw tool return bodies, raw paths, raw URLs, and credential
//! material stay outside the support boundary; every action label, tool label, and time
//! label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-high-friction-approval-sheet-and-tool-call-timeline-row.schema.json`](../../../../schemas/ai/m5-ai-high-friction-approval-sheet-and-tool-call-timeline-row.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes.md`](../../../../docs/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_approval_tool_call_primitive_branch_agent_checkpoint_beta_narrowed,
    seeded_m5_ai_approval_tool_call_primitive_mutating_tool_run_preview_narrowed,
    seeded_m5_ai_approval_tool_call_primitive_packet,
    M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_PACKET_ID,
};

// The approval gate, friction reason, tool boundary, side-effect class, run outcome,
// surface family, deployment line, consumer surface, accessibility route, qualification
// class, and downgrade triggers are frozen once, in the AI-execution/replay component
// matrix. These primitives reuse them verbatim so they never invent a parallel approval
// or tool-call vocabulary.
pub use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5AiAccessibilityRoute, M5AiApprovalGate, M5AiConsumerSurface, M5AiDeploymentLine,
    M5AiExecutionDowngradeTrigger, M5AiFrictionReason, M5AiQualificationClass, M5AiRunOutcome,
    M5AiSideEffectClass, M5AiSurfaceFamily, M5AiToolBoundary,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiApprovalToolCallPrimitivePacket`].
pub const M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_ai_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes";

/// Schema version for M5 AI approval-sheet / tool-call-timeline-row primitive records.
pub const M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the approval-sheet / tool-call-timeline-row schema.
pub const M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-high-friction-approval-sheet-and-tool-call-timeline-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_APPROVAL_TOOL_CALL_DOC_REF: &str =
    "docs/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes.md";

/// Repo-relative path of the frozen AI-execution/replay component matrix these
/// primitives narrow from.
pub const M5_AI_APPROVAL_TOOL_CALL_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json";

/// Repo-relative path of the approval-action-class contract this primitive binds its
/// action-class and rollback vocabulary against, so an approval sheet preserves the same
/// action classes and rollback vocabulary as the policy and evidence systems.
pub const M5_AI_APPROVAL_TOOL_CALL_APPROVAL_ACTION_REF: &str =
    "schemas/ai/approval_action_class.schema.json";

/// Repo-relative path of the tool-call-timeline-entry contract this primitive binds its
/// tool-call, side-effect, and follow-up-action truth against.
pub const M5_AI_APPROVAL_TOOL_CALL_TIMELINE_ENTRY_REF: &str =
    "schemas/ai/tool_call_timeline_entry.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_APPROVAL_TOOL_CALL_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_APPROVAL_TOOL_CALL_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_APPROVAL_TOOL_CALL_CSV_REF: &str =
    "artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_APPROVAL_TOOL_CALL_REPORT_REF: &str =
    "artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes.md";

/// One claimed M5 tool lane that renders the shared approval sheet and the tool-call
/// timeline row. These are the lanes the acceptance criteria name — read-only tool
/// invocations, mutating tool runs, test-generation validations, branch-agent
/// checkpoints, and the CLI / support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiToolLaneSurface {
    /// A read-only tool invocation.
    ReadOnlyToolInvocation,
    /// A mutating tool run.
    MutatingToolRun,
    /// A test-generation validation.
    TestGenerationValidation,
    /// A branch / worktree agent checkpoint.
    BranchAgentCheckpoint,
    /// The CLI inspect / support export.
    CliSupportExport,
}

impl M5AiToolLaneSurface {
    /// Every claimed tool lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReadOnlyToolInvocation,
        Self::MutatingToolRun,
        Self::TestGenerationValidation,
        Self::BranchAgentCheckpoint,
        Self::CliSupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyToolInvocation => "read_only_tool_invocation",
            Self::MutatingToolRun => "mutating_tool_run",
            Self::TestGenerationValidation => "test_generation_validation",
            Self::BranchAgentCheckpoint => "branch_agent_checkpoint",
            Self::CliSupportExport => "cli_support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReadOnlyToolInvocation => "Read-Only Tool Invocation",
            Self::MutatingToolRun => "Mutating Tool Run",
            Self::TestGenerationValidation => "Test-Generation Validation",
            Self::BranchAgentCheckpoint => "Branch-Agent Checkpoint",
            Self::CliSupportExport => "CLI / Support Export",
        }
    }
}

/// Controlled action scope — the blast radius a requested action reaches, so an approval
/// sheet never leaves the acceptance-criterion "scope" implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiActionScope {
    /// A single file.
    SingleFile,
    /// A subtree of the workspace.
    WorkspaceSubtree,
    /// The whole workspace.
    WholeWorkspace,
    /// An external resource outside the workspace.
    ExternalResource,
    /// A cross-tenant scope.
    CrossTenant,
    /// The host system.
    HostSystem,
}

impl M5AiActionScope {
    /// Every action scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleFile,
        Self::WorkspaceSubtree,
        Self::WholeWorkspace,
        Self::ExternalResource,
        Self::CrossTenant,
        Self::HostSystem,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::WorkspaceSubtree => "workspace_subtree",
            Self::WholeWorkspace => "whole_workspace",
            Self::ExternalResource => "external_resource",
            Self::CrossTenant => "cross_tenant",
            Self::HostSystem => "host_system",
        }
    }

    /// Review-safe phrase naming the blast radius.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::SingleFile => "a single file",
            Self::WorkspaceSubtree => "a subtree of this workspace",
            Self::WholeWorkspace => "this whole workspace",
            Self::ExternalResource => "an external resource outside this workspace",
            Self::CrossTenant => "a cross-tenant scope",
            Self::HostSystem => "the host system",
        }
    }

    /// True when the scope reaches beyond this workspace's boundary.
    pub const fn is_boundary_crossing(self) -> bool {
        matches!(
            self,
            Self::ExternalResource | Self::CrossTenant | Self::HostSystem
        )
    }
}

/// Controlled rollback posture — how a requested action can be undone, so an approval
/// sheet never drops the acceptance-criterion rollback / checkpoint vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRollbackPosture {
    /// Backed by a restorable checkpoint.
    CheckpointBacked,
    /// Reversible in place without a checkpoint.
    ReversibleInPlace,
    /// Only a forward fix is possible.
    ForwardFixOnly,
    /// Irreversible external side effect.
    IrreversibleExternal,
    /// No rollback is possible.
    NoRollback,
}

impl M5AiRollbackPosture {
    /// Every rollback posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CheckpointBacked,
        Self::ReversibleInPlace,
        Self::ForwardFixOnly,
        Self::IrreversibleExternal,
        Self::NoRollback,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointBacked => "checkpoint_backed",
            Self::ReversibleInPlace => "reversible_in_place",
            Self::ForwardFixOnly => "forward_fix_only",
            Self::IrreversibleExternal => "irreversible_external",
            Self::NoRollback => "no_rollback",
        }
    }

    /// True when the action can be cleanly reversed.
    pub const fn is_reversible(self) -> bool {
        matches!(self, Self::CheckpointBacked | Self::ReversibleInPlace)
    }

    /// True when the posture claims a restorable checkpoint.
    pub const fn is_checkpoint_backed(self) -> bool {
        matches!(self, Self::CheckpointBacked)
    }
}

/// Controlled approval sheet anatomy part. The parts in
/// [`M5AiApprovalSheetAnatomyPart::MANDATORY`] are required on every approval sheet so a
/// user can tell the requested action, scope, side effects, boundary, rollback, and
/// controls before approving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalSheetAnatomyPart {
    /// The requested action.
    RequestedActionCue,
    /// The action scope / blast radius.
    ScopeCue,
    /// The side-effect class.
    SideEffectCue,
    /// The tool boundary.
    BoundaryCue,
    /// The rollback / checkpoint posture.
    RollbackCheckpointCue,
    /// The friction reason.
    FrictionReasonCue,
    /// The effective approval gate.
    ApprovalGateCue,
    /// The explicit control row (approve-once / deny / open-plan).
    ControlRowCue,
}

impl M5AiApprovalSheetAnatomyPart {
    /// Every approval-sheet anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RequestedActionCue,
        Self::ScopeCue,
        Self::SideEffectCue,
        Self::BoundaryCue,
        Self::RollbackCheckpointCue,
        Self::FrictionReasonCue,
        Self::ApprovalGateCue,
        Self::ControlRowCue,
    ];

    /// The approval-sheet anatomy parts every sheet must render.
    pub const MANDATORY: [Self; 6] = [
        Self::RequestedActionCue,
        Self::ScopeCue,
        Self::SideEffectCue,
        Self::BoundaryCue,
        Self::RollbackCheckpointCue,
        Self::ControlRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedActionCue => "requested_action_cue",
            Self::ScopeCue => "scope_cue",
            Self::SideEffectCue => "side_effect_cue",
            Self::BoundaryCue => "boundary_cue",
            Self::RollbackCheckpointCue => "rollback_checkpoint_cue",
            Self::FrictionReasonCue => "friction_reason_cue",
            Self::ApprovalGateCue => "approval_gate_cue",
            Self::ControlRowCue => "control_row_cue",
        }
    }
}

/// Controlled tool-call timeline row anatomy part. The parts in
/// [`M5AiToolCallAnatomyPart::MANDATORY`] are required on every row so time, tool,
/// side-effect class, boundary, outcome, and governed follow-up actions stay visible
/// instead of buried in a raw log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiToolCallAnatomyPart {
    /// The time the tool call occurred.
    TimeCue,
    /// The tool identity.
    ToolCue,
    /// The side-effect class.
    SideEffectClassCue,
    /// The tool boundary.
    BoundaryCue,
    /// The run outcome.
    OutcomeCue,
    /// The governed follow-up action row.
    FollowUpActionCue,
    /// The provenance cue.
    ProvenanceCue,
    /// The predicted-versus-observed effect comparison.
    EffectComparisonCue,
}

impl M5AiToolCallAnatomyPart {
    /// Every tool-call anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TimeCue,
        Self::ToolCue,
        Self::SideEffectClassCue,
        Self::BoundaryCue,
        Self::OutcomeCue,
        Self::FollowUpActionCue,
        Self::ProvenanceCue,
        Self::EffectComparisonCue,
    ];

    /// The tool-call anatomy parts every row must render.
    pub const MANDATORY: [Self; 6] = [
        Self::TimeCue,
        Self::ToolCue,
        Self::SideEffectClassCue,
        Self::BoundaryCue,
        Self::OutcomeCue,
        Self::FollowUpActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimeCue => "time_cue",
            Self::ToolCue => "tool_cue",
            Self::SideEffectClassCue => "side_effect_class_cue",
            Self::BoundaryCue => "boundary_cue",
            Self::OutcomeCue => "outcome_cue",
            Self::FollowUpActionCue => "follow_up_action_cue",
            Self::ProvenanceCue => "provenance_cue",
            Self::EffectComparisonCue => "effect_comparison_cue",
        }
    }
}

/// One explicit control an approval sheet offers, so a sheet never hides the
/// approve-once / deny / open-plan (and rollback / escalation) affordances behind a
/// generic confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalControl {
    /// Approve this action once.
    ApproveOnce,
    /// Deny this action.
    Deny,
    /// Open the plan / review the full context.
    OpenPlan,
    /// Review the rollback checkpoint before approving.
    ReviewRollbackCheckpoint,
    /// Escalate to a second reviewer.
    EscalateSecondReviewer,
}

impl M5AiApprovalControl {
    /// Every approval control, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ApproveOnce,
        Self::Deny,
        Self::OpenPlan,
        Self::ReviewRollbackCheckpoint,
        Self::EscalateSecondReviewer,
    ];

    /// The three controls every approvable sheet must offer.
    pub const MANDATORY_TRIAD: [Self; 3] = [Self::ApproveOnce, Self::Deny, Self::OpenPlan];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApproveOnce => "approve_once",
            Self::Deny => "deny",
            Self::OpenPlan => "open_plan",
            Self::ReviewRollbackCheckpoint => "review_rollback_checkpoint",
            Self::EscalateSecondReviewer => "escalate_second_reviewer",
        }
    }
}

/// One governed follow-up action a tool-call timeline row offers, so provenance and
/// removal controls stay visible instead of buried inside raw logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiToolCallFollowUp {
    /// Open the tool output.
    OpenOutput,
    /// Remove the tool result from the active context.
    RemoveFromContext,
    /// View the provenance of the tool call.
    ViewProvenance,
    /// Replay the tool call in a sandbox.
    ReplayInSandbox,
    /// Renew the approval for a mutating / boundary-crossing call.
    RenewApproval,
}

impl M5AiToolCallFollowUp {
    /// Every follow-up action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenOutput,
        Self::RemoveFromContext,
        Self::ViewProvenance,
        Self::ReplayInSandbox,
        Self::RenewApproval,
    ];

    /// The follow-up controls every row must keep visible (provenance / removal).
    pub const MANDATORY: [Self; 2] = [Self::ViewProvenance, Self::RemoveFromContext];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenOutput => "open_output",
            Self::RemoveFromContext => "remove_from_context",
            Self::ViewProvenance => "view_provenance",
            Self::ReplayInSandbox => "replay_in_sandbox",
            Self::RenewApproval => "renew_approval",
        }
    }
}

/// A field the approval-sheet export carries so sheet truth is reconstructable. The
/// fields in [`M5AiApprovalSheetExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalSheetExportField {
    /// The requested action.
    RequestedAction,
    /// The action scope.
    Scope,
    /// The side-effect class.
    SideEffect,
    /// The tool boundary.
    Boundary,
    /// The rollback posture.
    RollbackPosture,
    /// The effective approval gate.
    EffectiveGate,
    /// The friction reasons.
    FrictionReasons,
    /// Whether a review sheet is required.
    RequiresReviewSheet,
}

impl M5AiApprovalSheetExportField {
    /// Every approval-sheet export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RequestedAction,
        Self::Scope,
        Self::SideEffect,
        Self::Boundary,
        Self::RollbackPosture,
        Self::EffectiveGate,
        Self::FrictionReasons,
        Self::RequiresReviewSheet,
    ];

    /// The approval-sheet export fields every sheet must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::RequestedAction,
        Self::Scope,
        Self::SideEffect,
        Self::Boundary,
        Self::RollbackPosture,
        Self::EffectiveGate,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedAction => "requested_action",
            Self::Scope => "scope",
            Self::SideEffect => "side_effect",
            Self::Boundary => "boundary",
            Self::RollbackPosture => "rollback_posture",
            Self::EffectiveGate => "effective_gate",
            Self::FrictionReasons => "friction_reasons",
            Self::RequiresReviewSheet => "requires_review_sheet",
        }
    }
}

/// A field the tool-call export carries so timeline-row truth is reconstructable. The
/// fields in [`M5AiToolCallExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiToolCallExportField {
    /// The time the tool call occurred.
    OccurredAt,
    /// The tool identity.
    Tool,
    /// The tool boundary.
    Boundary,
    /// The predicted side-effect class.
    PredictedSideEffect,
    /// The observed side-effect class.
    ObservedSideEffect,
    /// The run outcome.
    Outcome,
    /// The governed follow-up actions.
    FollowUpActions,
    /// Whether the observed effect escalated beyond the prediction.
    EffectEscalated,
}

impl M5AiToolCallExportField {
    /// Every tool-call export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::OccurredAt,
        Self::Tool,
        Self::Boundary,
        Self::PredictedSideEffect,
        Self::ObservedSideEffect,
        Self::Outcome,
        Self::FollowUpActions,
        Self::EffectEscalated,
    ];

    /// The tool-call export fields every row must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::OccurredAt,
        Self::Tool,
        Self::Boundary,
        Self::ObservedSideEffect,
        Self::Outcome,
        Self::FollowUpActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OccurredAt => "occurred_at",
            Self::Tool => "tool",
            Self::Boundary => "boundary",
            Self::PredictedSideEffect => "predicted_side_effect",
            Self::ObservedSideEffect => "observed_side_effect",
            Self::Outcome => "outcome",
            Self::FollowUpActions => "follow_up_actions",
            Self::EffectEscalated => "effect_escalated",
        }
    }
}

/// True when a side-effect class mutates state beyond a read-only call and so must be
/// kept review-first.
pub const fn side_effect_is_mutating(class: M5AiSideEffectClass) -> bool {
    !matches!(class, M5AiSideEffectClass::ReadOnly)
}

/// Ordered severity of a side-effect class, so an observed effect that escalates beyond
/// the prediction can be detected. Read-only is lowest; destructive is highest.
pub const fn side_effect_severity(class: M5AiSideEffectClass) -> u8 {
    match class {
        M5AiSideEffectClass::ReadOnly => 0,
        M5AiSideEffectClass::FileWrite => 1,
        M5AiSideEffectClass::NetworkCall => 2,
        M5AiSideEffectClass::ProcessSpawn => 3,
        M5AiSideEffectClass::StateMutation => 4,
        M5AiSideEffectClass::Destructive => 5,
    }
}

/// True when a tool boundary leaves the local process (a boundary-crossing call).
pub const fn tool_boundary_is_crossing(boundary: M5AiToolBoundary) -> bool {
    matches!(
        boundary,
        M5AiToolBoundary::RemoteConnector
            | M5AiToolBoundary::ExternalService
            | M5AiToolBoundary::HostDelegated
    )
}

/// Friction rank for an approval gate, so the effective gate never falls below the
/// friction floor. Higher is more friction; `PolicyBlocked` is highest.
const fn approval_gate_rank(gate: M5AiApprovalGate) -> u8 {
    match gate {
        M5AiApprovalGate::AutoApproved => 0,
        M5AiApprovalGate::NotifyOnly => 1,
        M5AiApprovalGate::OneClickConfirm => 2,
        M5AiApprovalGate::HighFrictionTyped => 3,
        M5AiApprovalGate::TwoPersonReview => 4,
        M5AiApprovalGate::PolicyBlocked => 5,
    }
}

// ---- approval-sheet resolver --------------------------------------------

/// The full input to the approval-sheet resolver for one requested action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalSheetResolutionInput {
    /// The opaque requested-action label.
    pub requested_action_label: String,
    /// The blast radius the action reaches.
    pub action_scope: M5AiActionScope,
    /// The side-effect class of the action.
    pub side_effect_class: M5AiSideEffectClass,
    /// The tool boundary the action crosses.
    pub tool_boundary: M5AiToolBoundary,
    /// The friction reasons that apply (may be empty).
    pub friction_reasons: Vec<M5AiFrictionReason>,
    /// How the action can be rolled back.
    pub rollback_posture: M5AiRollbackPosture,
    /// True when a restorable checkpoint ref is present.
    pub checkpoint_ref_present: bool,
    /// The declared approval gate for the action.
    pub declared_approval_gate: M5AiApprovalGate,
}

/// The resolved approval-sheet truth for one requested action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedApprovalSheet {
    /// The opaque requested-action label.
    pub requested_action_label: String,
    /// The blast radius the action reaches.
    pub action_scope: M5AiActionScope,
    /// The side-effect class of the action.
    pub side_effect_class: M5AiSideEffectClass,
    /// The tool boundary the action crosses.
    pub tool_boundary: M5AiToolBoundary,
    /// The friction reasons that apply.
    pub friction_reasons: Vec<M5AiFrictionReason>,
    /// How the action can be rolled back.
    pub rollback_posture: M5AiRollbackPosture,
    /// The effective (friction-escalated) approval gate.
    pub effective_approval_gate: M5AiApprovalGate,
    /// True when the action mutates state or crosses a boundary.
    pub is_mutating_or_boundary_crossing: bool,
    /// True when the action requires a review sheet rather than a status update.
    pub requires_review_sheet: bool,
    /// True when the effective gate is high-friction (typed or two-person).
    pub is_high_friction: bool,
    /// True when the action can be cleanly reversed.
    pub is_reversible: bool,
    /// The explicit controls this sheet offers.
    pub available_controls: Vec<M5AiApprovalControl>,
}

/// Errors returned by [`resolve_approval_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiApprovalSheetResolutionError {
    /// The requested-action label was empty.
    EmptyActionLabel,
    /// A mutating or boundary-crossing action was declared as an auto-approved or
    /// notify-only status update, masking it as an ordinary status.
    MutatingActionMaskedAsStatus,
    /// The rollback posture claimed a checkpoint but no checkpoint ref was present.
    CheckpointClaimedWithoutRef,
    /// An approval descriptor carried forbidden material.
    ForbiddenApprovalMaterial,
}

impl M5AiApprovalSheetResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyActionLabel => "empty_action_label",
            Self::MutatingActionMaskedAsStatus => "mutating_action_masked_as_status",
            Self::CheckpointClaimedWithoutRef => "checkpoint_claimed_without_ref",
            Self::ForbiddenApprovalMaterial => "forbidden_approval_material",
        }
    }
}

impl fmt::Display for M5AiApprovalSheetResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai approval sheet resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiApprovalSheetResolutionError {}

/// Resolves one high-friction approval sheet from a requested action's declared state.
///
/// A mutating or boundary-crossing action can never read as an auto-approved or
/// notify-only status update: the resolver rejects that as
/// [`M5AiApprovalSheetResolutionError::MutatingActionMaskedAsStatus`]. Otherwise the
/// effective approval gate is the friction floor the action's side effect, boundary, and
/// friction reasons imply, escalated no lower than the declared gate — a policy-blocked
/// gate stays blocked. The sheet always carries the requested action, scope, side
/// effect, boundary, rollback posture, and the explicit approve-once / deny / open-plan
/// controls (with rollback-review and escalation added when they apply), so a mutating
/// action is always review-first and attributable.
pub fn resolve_approval_sheet(
    input: &M5AiApprovalSheetResolutionInput,
) -> Result<M5ResolvedApprovalSheet, M5AiApprovalSheetResolutionError> {
    if input.requested_action_label.trim().is_empty() {
        return Err(M5AiApprovalSheetResolutionError::EmptyActionLabel);
    }
    if value_repr_is_forbidden(&input.requested_action_label) {
        return Err(M5AiApprovalSheetResolutionError::ForbiddenApprovalMaterial);
    }
    if input.rollback_posture.is_checkpoint_backed() && !input.checkpoint_ref_present {
        return Err(M5AiApprovalSheetResolutionError::CheckpointClaimedWithoutRef);
    }

    let is_mutating = side_effect_is_mutating(input.side_effect_class);
    let boundary_crossing = input.action_scope.is_boundary_crossing()
        || tool_boundary_is_crossing(input.tool_boundary);
    let is_mutating_or_boundary_crossing = is_mutating || boundary_crossing;

    // A mutating or boundary-crossing action must never be declared as an ordinary
    // status update (auto-approved or notify-only) with no review gate.
    if is_mutating_or_boundary_crossing
        && matches!(
            input.declared_approval_gate,
            M5AiApprovalGate::AutoApproved | M5AiApprovalGate::NotifyOnly
        )
    {
        return Err(M5AiApprovalSheetResolutionError::MutatingActionMaskedAsStatus);
    }

    let effective_approval_gate = derive_effective_gate(
        input.declared_approval_gate,
        is_mutating,
        boundary_crossing,
        &input.friction_reasons,
    );
    let requires_review_sheet = is_mutating_or_boundary_crossing
        || !input.friction_reasons.is_empty()
        || approval_gate_rank(effective_approval_gate)
            >= approval_gate_rank(M5AiApprovalGate::OneClickConfirm);
    let is_high_friction = matches!(
        effective_approval_gate,
        M5AiApprovalGate::HighFrictionTyped | M5AiApprovalGate::TwoPersonReview
    );
    let available_controls = derive_controls(
        effective_approval_gate,
        input.rollback_posture,
        input.checkpoint_ref_present,
    );

    Ok(M5ResolvedApprovalSheet {
        requested_action_label: input.requested_action_label.clone(),
        action_scope: input.action_scope,
        side_effect_class: input.side_effect_class,
        tool_boundary: input.tool_boundary,
        friction_reasons: input.friction_reasons.clone(),
        rollback_posture: input.rollback_posture,
        effective_approval_gate,
        is_mutating_or_boundary_crossing,
        requires_review_sheet,
        is_high_friction,
        is_reversible: input.rollback_posture.is_reversible(),
        available_controls,
    })
}

/// The fixed friction-first gate ladder: the effective gate is the declared gate raised
/// to the friction floor implied by the action, never lowered.
fn derive_effective_gate(
    declared: M5AiApprovalGate,
    is_mutating: bool,
    boundary_crossing: bool,
    friction_reasons: &[M5AiFrictionReason],
) -> M5AiApprovalGate {
    // A policy-blocked action stays blocked regardless of friction.
    if matches!(declared, M5AiApprovalGate::PolicyBlocked) {
        return M5AiApprovalGate::PolicyBlocked;
    }
    let floor = friction_floor(is_mutating, boundary_crossing, friction_reasons);
    if approval_gate_rank(declared) >= approval_gate_rank(floor) {
        declared
    } else {
        floor
    }
}

/// The friction floor an action's side effect, boundary, and friction reasons imply.
fn friction_floor(
    is_mutating: bool,
    boundary_crossing: bool,
    friction_reasons: &[M5AiFrictionReason],
) -> M5AiApprovalGate {
    use M5AiFrictionReason as Reason;
    // A policy-mandated review forces a two-person review.
    if friction_reasons.contains(&Reason::PolicyMandatedReview) {
        return M5AiApprovalGate::TwoPersonReview;
    }
    // Irreversible, destructive, credential, or cross-tenant friction forces a
    // high-friction typed confirmation.
    if friction_reasons.iter().any(|reason| {
        matches!(
            reason,
            Reason::IrreversibleSideEffect
                | Reason::DestructiveFileChange
                | Reason::CredentialAccess
                | Reason::CrossTenantScope
        )
    }) {
        return M5AiApprovalGate::HighFrictionTyped;
    }
    // Any other mutation, external egress, or boundary crossing forces at least a
    // one-click confirm.
    if is_mutating
        || boundary_crossing
        || friction_reasons.contains(&Reason::ExternalNetworkEgress)
    {
        return M5AiApprovalGate::OneClickConfirm;
    }
    M5AiApprovalGate::AutoApproved
}

/// Derives the explicit control set from the effective gate and rollback posture.
fn derive_controls(
    gate: M5AiApprovalGate,
    rollback_posture: M5AiRollbackPosture,
    checkpoint_ref_present: bool,
) -> Vec<M5AiApprovalControl> {
    use M5AiApprovalControl as Control;
    let mut controls = Vec::new();
    // A policy-blocked action offers no approve-once affordance; deny and open-plan stay.
    if !matches!(gate, M5AiApprovalGate::PolicyBlocked) {
        controls.push(Control::ApproveOnce);
    }
    controls.push(Control::Deny);
    controls.push(Control::OpenPlan);
    if checkpoint_ref_present || rollback_posture.is_checkpoint_backed() {
        controls.push(Control::ReviewRollbackCheckpoint);
    }
    if matches!(gate, M5AiApprovalGate::TwoPersonReview) {
        controls.push(Control::EscalateSecondReviewer);
    }
    controls
}

// ---- tool-call timeline row resolver ------------------------------------

/// The full input to the tool-call-timeline-row resolver for one tool call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiToolCallResolutionInput {
    /// The opaque time-occurred label.
    pub occurred_at_label: String,
    /// The opaque tool label.
    pub tool_label: String,
    /// The tool boundary the call ran on.
    pub tool_boundary: M5AiToolBoundary,
    /// The predicted side-effect class before the call.
    pub predicted_side_effect: M5AiSideEffectClass,
    /// The observed side-effect class after the call.
    pub observed_side_effect: M5AiSideEffectClass,
    /// How the run ended.
    pub run_outcome: M5AiRunOutcome,
    /// True when the tool output is available to open.
    pub output_available: bool,
    /// True when the tool result is still loaded in the active context.
    pub in_active_context: bool,
}

/// The resolved tool-call-timeline-row truth for one tool call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedToolCallTimelineRow {
    /// The opaque time-occurred label.
    pub occurred_at_label: String,
    /// The opaque tool label.
    pub tool_label: String,
    /// The tool boundary the call ran on.
    pub tool_boundary: M5AiToolBoundary,
    /// The predicted side-effect class.
    pub predicted_side_effect: M5AiSideEffectClass,
    /// The observed side-effect class.
    pub observed_side_effect: M5AiSideEffectClass,
    /// How the run ended.
    pub run_outcome: M5AiRunOutcome,
    /// True when the observed effect escalated beyond the prediction.
    pub effect_escalated: bool,
    /// True when the observed effect mutated state.
    pub is_mutating: bool,
    /// True when the call crossed a boundary.
    pub boundary_crossing: bool,
    /// The governed follow-up actions this row offers.
    pub follow_up_actions: Vec<M5AiToolCallFollowUp>,
}

/// Errors returned by [`resolve_tool_call_timeline_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiToolCallResolutionError {
    /// The time-occurred label was empty.
    EmptyOccurredAt,
    /// The tool label was empty.
    EmptyToolLabel,
    /// A tool-call descriptor carried forbidden material.
    ForbiddenToolCallMaterial,
}

impl M5AiToolCallResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyOccurredAt => "empty_occurred_at",
            Self::EmptyToolLabel => "empty_tool_label",
            Self::ForbiddenToolCallMaterial => "forbidden_tool_call_material",
        }
    }
}

impl fmt::Display for M5AiToolCallResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai tool call resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiToolCallResolutionError {}

/// Resolves one tool-call timeline row from a tool call's declared state.
///
/// The observed side-effect class is carried explicitly and compared against the
/// prediction, so a call that escalated (for example, a predicted read-only call that
/// observed a state mutation) is flagged rather than shown as read-only. The governed
/// follow-up actions always keep the provenance and removal controls visible — the row
/// offers view-provenance always, remove-from-context whenever the result is still in
/// the active context, open-output when output is available, and replay-in-sandbox and
/// renew-approval for mutating or boundary-crossing calls — instead of burying them in a
/// raw log.
pub fn resolve_tool_call_timeline_row(
    input: &M5AiToolCallResolutionInput,
) -> Result<M5ResolvedToolCallTimelineRow, M5AiToolCallResolutionError> {
    if input.occurred_at_label.trim().is_empty() {
        return Err(M5AiToolCallResolutionError::EmptyOccurredAt);
    }
    if input.tool_label.trim().is_empty() {
        return Err(M5AiToolCallResolutionError::EmptyToolLabel);
    }
    if value_repr_is_forbidden(&input.occurred_at_label)
        || value_repr_is_forbidden(&input.tool_label)
    {
        return Err(M5AiToolCallResolutionError::ForbiddenToolCallMaterial);
    }

    let effect_escalated = side_effect_severity(input.observed_side_effect)
        > side_effect_severity(input.predicted_side_effect);
    let is_mutating = side_effect_is_mutating(input.observed_side_effect);
    let boundary_crossing = tool_boundary_is_crossing(input.tool_boundary);
    let follow_up_actions = derive_follow_ups(
        is_mutating,
        boundary_crossing,
        input.output_available,
        input.in_active_context,
    );

    Ok(M5ResolvedToolCallTimelineRow {
        occurred_at_label: input.occurred_at_label.clone(),
        tool_label: input.tool_label.clone(),
        tool_boundary: input.tool_boundary,
        predicted_side_effect: input.predicted_side_effect,
        observed_side_effect: input.observed_side_effect,
        run_outcome: input.run_outcome,
        effect_escalated,
        is_mutating,
        boundary_crossing,
        follow_up_actions,
    })
}

/// Derives the governed follow-up action set, keeping provenance and removal controls
/// visible on every row.
fn derive_follow_ups(
    is_mutating: bool,
    boundary_crossing: bool,
    output_available: bool,
    in_active_context: bool,
) -> Vec<M5AiToolCallFollowUp> {
    use M5AiToolCallFollowUp as FollowUp;
    let mut actions = Vec::new();
    if output_available {
        actions.push(FollowUp::OpenOutput);
    }
    // Provenance is always inspectable; removal is offered whenever the result is still
    // loaded in the active context.
    if in_active_context {
        actions.push(FollowUp::RemoveFromContext);
    }
    actions.push(FollowUp::ViewProvenance);
    if is_mutating || boundary_crossing {
        actions.push(FollowUp::ReplayInSandbox);
        actions.push(FollowUp::RenewApproval);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked approval-sheet resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalSheetResolutionCase {
    /// The resolver input.
    pub input: M5AiApprovalSheetResolutionInput,
    /// The resolved truth. Must equal `resolve_approval_sheet(&input)`.
    pub resolved: M5ResolvedApprovalSheet,
}

impl M5AiApprovalSheetResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiApprovalSheetResolutionInput) -> Self {
        let resolved = resolve_approval_sheet(&input).expect("seed approval case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_approval_sheet(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked tool-call resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiToolCallResolutionCase {
    /// The resolver input.
    pub input: M5AiToolCallResolutionInput,
    /// The resolved truth. Must equal `resolve_tool_call_timeline_row(&input)`.
    pub resolved: M5ResolvedToolCallTimelineRow,
}

impl M5AiToolCallResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiToolCallResolutionInput) -> Self {
        let resolved =
            resolve_tool_call_timeline_row(&input).expect("seed tool call case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_tool_call_timeline_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one tool lane bound to the shared approval-sheet and
/// tool-call anatomy, action scopes, side-effect classes, tool boundaries, rollback
/// postures, approval gates, friction reasons, run outcomes, approval controls, follow-up
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalToolCallRow {
    /// Tool lane family.
    pub tool_lane: M5AiToolLaneSurface,
    /// Qualification class earned by this lane.
    pub qualification: M5AiQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 AI surface families that render / consume these components.
    pub surface_families: Vec<M5AiSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5AiDeploymentLine>,
    /// Approval-sheet anatomy parts this sheet renders (must include the mandatory parts).
    pub approval_anatomy_parts: Vec<M5AiApprovalSheetAnatomyPart>,
    /// Tool-call anatomy parts this row renders (must include the mandatory parts).
    pub tool_call_anatomy_parts: Vec<M5AiToolCallAnatomyPart>,
    /// Action scopes this lane distinguishes.
    pub action_scopes: Vec<M5AiActionScope>,
    /// Side-effect classes this lane names.
    pub side_effect_classes: Vec<M5AiSideEffectClass>,
    /// Tool boundaries this lane distinguishes.
    pub tool_boundaries: Vec<M5AiToolBoundary>,
    /// Rollback postures this lane distinguishes.
    pub rollback_postures: Vec<M5AiRollbackPosture>,
    /// Approval gates this lane distinguishes.
    pub approval_gates: Vec<M5AiApprovalGate>,
    /// Friction reasons this lane names.
    pub friction_reasons: Vec<M5AiFrictionReason>,
    /// Run outcomes this lane distinguishes.
    pub run_outcomes: Vec<M5AiRunOutcome>,
    /// Approval controls this lane offers.
    pub approval_controls: Vec<M5AiApprovalControl>,
    /// Governed follow-up actions this lane offers.
    pub follow_up_actions: Vec<M5AiToolCallFollowUp>,
    /// Approval-sheet export fields this sheet carries (must include the mandatory fields).
    pub approval_export_fields: Vec<M5AiApprovalSheetExportField>,
    /// Tool-call export fields this row carries (must include the mandatory fields).
    pub tool_call_export_fields: Vec<M5AiToolCallExportField>,
    /// Non-visual accessibility routes this lane offers.
    pub accessibility_routes: Vec<M5AiAccessibilityRoute>,
    /// AI subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5AiConsumerSurface>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5AiExecutionDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked approval-sheet resolutions proving the approval resolver on this lane.
    pub approval_examples: Vec<M5AiApprovalSheetResolutionCase>,
    /// Worked tool-call resolutions proving the tool-call resolver on this lane.
    pub tool_call_examples: Vec<M5AiToolCallResolutionCase>,
    /// Hard invariant: this lane never lets a mutating or boundary-crossing action read
    /// as an ordinary status update. MUST be `false`.
    pub masks_mutation_or_boundary_as_status: bool,
    /// Hard invariant: this lane never buries provenance or removal controls inside raw
    /// logs. MUST be `false`.
    pub buries_provenance_or_removal_in_logs: bool,
    /// Hard invariant: this lane never drops the rollback / checkpoint vocabulary. MUST
    /// be `false`.
    pub drops_rollback_or_checkpoint_vocabulary: bool,
    /// Hard invariant: this lane never invents a parallel approval or tool-call grammar.
    /// MUST be `false`.
    pub invents_parallel_approval_or_tool_grammar: bool,
}

impl M5AiApprovalToolCallRow {
    /// True when the row declares every mandatory approval-sheet anatomy part.
    fn declares_mandatory_approval_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiApprovalSheetAnatomyPart> =
            self.approval_anatomy_parts.iter().copied().collect();
        M5AiApprovalSheetAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory tool-call anatomy part.
    fn declares_mandatory_tool_call_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiToolCallAnatomyPart> =
            self.tool_call_anatomy_parts.iter().copied().collect();
        M5AiToolCallAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory approval-sheet export field.
    fn declares_mandatory_approval_export(&self) -> bool {
        let present: BTreeSet<M5AiApprovalSheetExportField> =
            self.approval_export_fields.iter().copied().collect();
        M5AiApprovalSheetExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory tool-call export field.
    fn declares_mandatory_tool_call_export(&self) -> bool {
        let present: BTreeSet<M5AiToolCallExportField> =
            self.tool_call_export_fields.iter().copied().collect();
        M5AiToolCallExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps the mandatory provenance / removal follow-up controls.
    fn declares_mandatory_follow_ups(&self) -> bool {
        let present: BTreeSet<M5AiToolCallFollowUp> =
            self.follow_up_actions.iter().copied().collect();
        M5AiToolCallFollowUp::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_mutation_or_boundary_as_status
            && !self.buries_provenance_or_removal_in_logs
            && !self.drops_rollback_or_checkpoint_vocabulary
            && !self.invents_parallel_approval_or_tool_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalToolCallVocabularySet {
    /// Tool-lane tokens.
    pub tool_lanes: Vec<String>,
    /// Approval-sheet-anatomy-part tokens.
    pub approval_anatomy_parts: Vec<String>,
    /// Tool-call-anatomy-part tokens.
    pub tool_call_anatomy_parts: Vec<String>,
    /// Action-scope tokens.
    pub action_scopes: Vec<String>,
    /// Rollback-posture tokens.
    pub rollback_postures: Vec<String>,
    /// Approval-control tokens.
    pub approval_controls: Vec<String>,
    /// Follow-up-action tokens.
    pub follow_up_actions: Vec<String>,
    /// Approval-sheet-export-field tokens.
    pub approval_export_fields: Vec<String>,
    /// Tool-call-export-field tokens.
    pub tool_call_export_fields: Vec<String>,
    /// Approval-gate tokens (reused from the frozen matrix).
    pub approval_gates: Vec<String>,
    /// Friction-reason tokens (reused from the frozen matrix).
    pub friction_reasons: Vec<String>,
    /// Tool-boundary tokens (reused from the frozen matrix).
    pub tool_boundaries: Vec<String>,
    /// Side-effect-class tokens (reused from the frozen matrix).
    pub side_effect_classes: Vec<String>,
    /// Run-outcome tokens (reused from the frozen matrix).
    pub run_outcomes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AiApprovalToolCallVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            tool_lanes: tokens(&M5AiToolLaneSurface::ALL, |v| v.as_str()),
            approval_anatomy_parts: tokens(&M5AiApprovalSheetAnatomyPart::ALL, |v| v.as_str()),
            tool_call_anatomy_parts: tokens(&M5AiToolCallAnatomyPart::ALL, |v| v.as_str()),
            action_scopes: tokens(&M5AiActionScope::ALL, |v| v.as_str()),
            rollback_postures: tokens(&M5AiRollbackPosture::ALL, |v| v.as_str()),
            approval_controls: tokens(&M5AiApprovalControl::ALL, |v| v.as_str()),
            follow_up_actions: tokens(&M5AiToolCallFollowUp::ALL, |v| v.as_str()),
            approval_export_fields: tokens(&M5AiApprovalSheetExportField::ALL, |v| v.as_str()),
            tool_call_export_fields: tokens(&M5AiToolCallExportField::ALL, |v| v.as_str()),
            approval_gates: tokens(&M5AiApprovalGate::ALL, |v| v.as_str()),
            friction_reasons: tokens(&M5AiFrictionReason::ALL, |v| v.as_str()),
            tool_boundaries: tokens(&M5AiToolBoundary::ALL, |v| v.as_str()),
            side_effect_classes: tokens(&M5AiSideEffectClass::ALL, |v| v.as_str()),
            run_outcomes: tokens(&M5AiRunOutcome::ALL, |v| v.as_str()),
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
pub struct M5AiApprovalToolCallGovernanceReview {
    /// One primitive pair carries approval and tool-call truth on every lane.
    pub one_primitive_carries_approval_and_tool_call_truth: bool,
    /// The requested action, scope, and side effect are always shown.
    pub requested_action_scope_side_effect_always_shown: bool,
    /// A mutating or boundary-crossing action never reads as an ordinary status update.
    pub mutating_action_never_ordinary_status: bool,
    /// The tool boundary and rollback posture are always named.
    pub boundary_and_rollback_always_named: bool,
    /// Provenance and removal controls are always visible, never buried in logs.
    pub provenance_and_removal_always_visible: bool,
    /// The approve-once / deny / open-plan controls are always explicit.
    pub approval_controls_always_explicit: bool,
    /// The action classes and rollback vocabulary match policy and evidence systems.
    pub action_classes_match_policy_and_evidence: bool,
    /// The support / export packet reconstructs sheet and row truth.
    pub support_export_reconstructs_sheet_and_row_truth: bool,
    /// No lane invents a second approval or tool-call grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalToolCallConsumerProjection {
    /// Read-only, mutating, test-generation, branch-agent, and CLI/support lanes all
    /// consume the shared primitive pair.
    pub tool_lanes_consume_shared_primitive: bool,
    /// The effective-approval-gate resolver reads a single canonical source.
    pub approval_gate_reads_single_source: bool,
    /// The side-effect-class comparison reads a single canonical source.
    pub side_effect_class_reads_single_source: bool,
    /// The follow-up-action derivation reads a single canonical source.
    pub follow_up_actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalToolCallProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalToolCallReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiApprovalToolCallPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiApprovalToolCallPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Tool-lane rows.
    pub rows: Vec<M5AiApprovalToolCallRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiApprovalToolCallVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiApprovalToolCallGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiApprovalToolCallConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiApprovalToolCallProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiApprovalToolCallReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 approval-sheet / tool-call-timeline-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalToolCallPrimitivePacket {
    /// Record kind; must equal [`M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Tool-lane rows.
    pub rows: Vec<M5AiApprovalToolCallRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiApprovalToolCallVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiApprovalToolCallGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiApprovalToolCallConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiApprovalToolCallProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiApprovalToolCallReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiApprovalToolCallPrimitivePacket {
    /// Builds an M5 approval-sheet / tool-call-timeline-row primitive packet.
    pub fn new(input: M5AiApprovalToolCallPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_SCHEMA_VERSION,
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

    /// Validates the M5 approval-sheet / tool-call-timeline-row invariants.
    pub fn validate(&self) -> Vec<M5AiApprovalToolCallPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_RECORD_KIND {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_APPROVAL_TOOL_CALL_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_mutating_review_first(self, &mut violations);
        validate_approval_control_triad(self, &mut violations);
        validate_provenance_removal(self, &mut violations);
        validate_effect_honesty(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai approval/tool-call primitive packet serializes"),
        ) {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 ai approval/tool-call primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per tool lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "tool_lane,qualification,owner,approval_anatomy,tool_call_anatomy,action_scopes,side_effect_classes,rollback_postures,approval_controls,follow_up_actions,approval_examples,tool_call_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.tool_lane.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.approval_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.tool_call_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.action_scopes, |v| v.as_str()),
                join_tokens(&row.side_effect_classes, |v| v.as_str()),
                join_tokens(&row.rollback_postures, |v| v.as_str()),
                join_tokens(&row.approval_controls, |v| v.as_str()),
                join_tokens(&row.follow_up_actions, |v| v.as_str()),
                row.approval_examples.len(),
                row.tool_call_examples.len(),
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
        out.push_str("# M5 AI High-Friction Approval-Sheet and Tool-Call-Timeline-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Tool lanes: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Action scopes: {}\n",
            self.vocabulary_set.action_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Rollback postures: {}\n",
            self.vocabulary_set.rollback_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Approval controls: {}\n",
            self.vocabulary_set.approval_controls.join(", ")
        ));
        out.push_str(&format!(
            "- Follow-up actions: {}\n",
            self.vocabulary_set.follow_up_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Tool lanes\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.tool_lane.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked approval sheets: {}\n",
                row.approval_examples.len()
            ));
            for case in &row.approval_examples {
                out.push_str(&format!(
                    "    - `{}` over `{}` → gate `{}` (review-first `{}`, reversible `{}`)\n",
                    case.resolved.requested_action_label,
                    case.resolved.action_scope.as_str(),
                    case.resolved.effective_approval_gate.as_str(),
                    case.resolved.requires_review_sheet,
                    case.resolved.is_reversible,
                ));
            }
            out.push_str(&format!(
                "  - Worked tool-call rows: {}\n",
                row.tool_call_examples.len()
            ));
            for case in &row.tool_call_examples {
                out.push_str(&format!(
                    "    - `{}` at `{}` → observed `{}` (escalated `{}`, follow-ups {})\n",
                    case.resolved.tool_label,
                    case.resolved.tool_boundary.as_str(),
                    case.resolved.observed_side_effect.as_str(),
                    case.resolved.effect_escalated,
                    case.resolved.follow_up_actions.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 approval/tool-call-primitive export.
#[derive(Debug)]
pub enum M5AiApprovalToolCallPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiApprovalToolCallPrimitiveViolation>),
}

impl fmt::Display for M5AiApprovalToolCallPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai approval/tool-call primitive export parse failed: {error}"
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
                    "m5 ai approval/tool-call primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiApprovalToolCallPrimitiveArtifactError {}

/// Validation failures emitted by [`M5AiApprovalToolCallPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiApprovalToolCallPrimitiveViolation {
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
    /// A required tool lane is missing from the matrix.
    RequiredLaneMissing,
    /// A tool-lane row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory approval-sheet anatomy parts.
    MandatoryApprovalAnatomyMissing,
    /// A row omits one of the mandatory tool-call anatomy parts.
    MandatoryToolCallAnatomyMissing,
    /// A row omits one of the mandatory approval-sheet export fields.
    MandatoryApprovalExportMissing,
    /// A row omits one of the mandatory tool-call export fields.
    MandatoryToolCallExportMissing,
    /// A row buries the mandatory provenance / removal follow-up controls.
    MandatoryFollowUpMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked approval-sheet resolutions.
    ApprovalExampleMissing,
    /// A row declares no worked tool-call resolutions.
    ToolCallExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableLaneMissingProof,
    /// No worked approval resolution proves a mutating / boundary-crossing action held
    /// review-first at a high-friction gate.
    MutatingActionReviewFirstUnproven,
    /// No worked approval resolution proves the explicit approve-once / deny / open-plan
    /// control triad.
    ApprovalControlTriadUnproven,
    /// No worked tool-call resolution proves both provenance and removal controls stay
    /// visible.
    ToolCallProvenanceRemovalUnproven,
    /// No worked tool-call resolution proves an escalated (observed worse than predicted)
    /// effect.
    ToolCallEffectHonestyUnproven,
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

impl M5AiApprovalToolCallPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryApprovalAnatomyMissing => "mandatory_approval_anatomy_missing",
            Self::MandatoryToolCallAnatomyMissing => "mandatory_tool_call_anatomy_missing",
            Self::MandatoryApprovalExportMissing => "mandatory_approval_export_missing",
            Self::MandatoryToolCallExportMissing => "mandatory_tool_call_export_missing",
            Self::MandatoryFollowUpMissing => "mandatory_follow_up_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ApprovalExampleMissing => "approval_example_missing",
            Self::ToolCallExampleMissing => "tool_call_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableLaneMissingProof => "stable_lane_missing_proof",
            Self::MutatingActionReviewFirstUnproven => "mutating_action_review_first_unproven",
            Self::ApprovalControlTriadUnproven => "approval_control_triad_unproven",
            Self::ToolCallProvenanceRemovalUnproven => "tool_call_provenance_removal_unproven",
            Self::ToolCallEffectHonestyUnproven => "tool_call_effect_honesty_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 approval/tool-call-primitive export.
pub fn current_stable_m5_ai_approval_tool_call_primitive_export(
) -> Result<M5AiApprovalToolCallPrimitivePacket, M5AiApprovalToolCallPrimitiveArtifactError> {
    let packet: M5AiApprovalToolCallPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/support_export.json"
    )))
    .map_err(M5AiApprovalToolCallPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiApprovalToolCallPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF,
        M5_AI_APPROVAL_TOOL_CALL_DOC_REF,
        M5_AI_APPROVAL_TOOL_CALL_COMPONENT_MATRIX_REF,
        M5_AI_APPROVAL_TOOL_CALL_APPROVAL_ACTION_REF,
        M5_AI_APPROVAL_TOOL_CALL_TIMELINE_ENTRY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let present: BTreeSet<M5AiToolLaneSurface> =
        packet.rows.iter().map(|row| row.tool_lane).collect();
    for required in M5AiToolLaneSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.approval_anatomy_parts.is_empty()
            || row.tool_call_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.action_scopes.is_empty()
            || row.side_effect_classes.is_empty()
            || row.tool_boundaries.is_empty()
            || row.rollback_postures.is_empty()
            || row.approval_gates.is_empty()
            || row.friction_reasons.is_empty()
            || row.run_outcomes.is_empty()
            || row.approval_controls.is_empty()
            || row.follow_up_actions.is_empty()
        {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::RowIncomplete);
        }
        if !row.declares_mandatory_approval_anatomy() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MandatoryApprovalAnatomyMissing);
        }
        if !row.declares_mandatory_tool_call_anatomy() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MandatoryToolCallAnatomyMissing);
        }
        if !row.declares_mandatory_approval_export() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MandatoryApprovalExportMissing);
        }
        if !row.declares_mandatory_tool_call_export() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MandatoryToolCallExportMissing);
        }
        if !row.declares_mandatory_follow_ups() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::MandatoryFollowUpMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AiAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.approval_examples.is_empty() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::ApprovalExampleMissing);
        }
        if row.tool_call_examples.is_empty() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::ToolCallExampleMissing);
        }
        if row
            .approval_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .tool_call_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::StableLaneMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked approval resolution across the matrix must prove a mutating or
/// boundary-crossing action held review-first at a high-friction gate — the
/// acceptance-criterion example that a mutating action never reads as an ordinary status
/// update.
fn validate_mutating_review_first(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.approval_examples.iter().any(|case| {
            case.resolved.is_mutating_or_boundary_crossing
                && case.resolved.requires_review_sheet
                && case.resolved.is_high_friction
        })
    });
    if !proven {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::MutatingActionReviewFirstUnproven);
    }
}

/// At least one worked approval resolution must offer the explicit approve-once / deny /
/// open-plan control triad — the acceptance-criterion example that approval controls stay
/// explicit.
fn validate_approval_control_triad(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.approval_examples.iter().any(|case| {
            M5AiApprovalControl::MANDATORY_TRIAD
                .iter()
                .all(|control| case.resolved.available_controls.contains(control))
        })
    });
    if !proven {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::ApprovalControlTriadUnproven);
    }
}

/// At least one worked tool-call resolution must keep both the provenance and the removal
/// controls visible — the acceptance-criterion example that tool-call history keeps
/// provenance / removal controls visible instead of burying them in raw logs.
fn validate_provenance_removal(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.tool_call_examples.iter().any(|case| {
            case.resolved
                .follow_up_actions
                .contains(&M5AiToolCallFollowUp::ViewProvenance)
                && case
                    .resolved
                    .follow_up_actions
                    .contains(&M5AiToolCallFollowUp::RemoveFromContext)
        })
    });
    if !proven {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::ToolCallProvenanceRemovalUnproven);
    }
}

/// At least one worked tool-call resolution must prove an escalated effect (observed
/// worse than predicted) — the acceptance-criterion example that a destructive or
/// state-mutating call is never shown as read-only.
fn validate_effect_honesty(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.tool_call_examples
            .iter()
            .any(|case| case.resolved.effect_escalated)
    });
    if !proven {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::ToolCallEffectHonestyUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_approval_and_tool_call_truth,
        review.requested_action_scope_side_effect_always_shown,
        review.mutating_action_never_ordinary_status,
        review.boundary_and_rollback_always_named,
        review.provenance_and_removal_always_visible,
        review.approval_controls_always_explicit,
        review.action_classes_match_policy_and_evidence,
        review.support_export_reconstructs_sheet_and_row_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.tool_lanes_consume_shared_primitive,
        projection.approval_gate_reads_single_source,
        projection.side_effect_class_reads_single_source,
        projection.follow_up_actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5AiApprovalToolCallPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiApprovalToolCallPrimitivePacket,
    violations: &mut Vec<M5AiApprovalToolCallPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiApprovalToolCallPrimitiveViolation::ReleasePostureIncomplete);
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
