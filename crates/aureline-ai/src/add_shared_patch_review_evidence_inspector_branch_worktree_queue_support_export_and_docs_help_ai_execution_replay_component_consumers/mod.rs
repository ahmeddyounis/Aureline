//! Shared consumers for the reusable M5 AI execution/replay components, so the
//! action-state banners, connector / local-model rows, approval sheets, tool-call
//! timeline rows, run-history rows, replay / rerun-review sheets, and agent-status
//! cards keep route, approval, checkpoint-lineage, and replay-completeness truth
//! aligned across every claimed M5 surface where a user reviews, reruns, exports,
//! or hands off AI work.
//!
//! Aureline's frozen AI execution/replay component matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! names the eight governed AI execution/replay component families, and five
//! sibling `implement_*` / `ship_*` lanes narrow those families into working
//! primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the action-state / boundary-blocked banner
//!   ([`crate::implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces`]),
//! * the connector detail row / local-model pack card
//!   ([`crate::implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces`]),
//! * the high-friction approval sheet / tool-call timeline row
//!   ([`crate::implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes`]),
//! * the run-history row / approval-timeline entry / evidence-export summary
//!   ([`crate::ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces`]),
//!   and
//! * the rerun-review sheet / incomplete-replay banner / agent-status card
//!   ([`crate::implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows`]).
//!
//! This module is the *adoption* lane over those primitives. It proves the eight
//! families are reusable components — not one assistant panel plus a few
//! admin-only pages — by binding every claimed M5 execution/replay consumer (patch
//! review, the evidence inspector, the branch/worktree agent queue, the support
//! export, and the docs/help surface) to the same canonical component schemas and
//! the same descriptor vocabulary. Each consumer points at the primitive's
//! canonical schema and support-export artifact rather than re-wording route,
//! approval, checkpoint, or replay-completeness facts in local prose, and each
//! keeps that vocabulary truthful even when route/provider/model drift, a missing
//! connector output, a redaction fence, or a stale approval weakens replayability.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_replay_binding`] — that takes one consumer's adoption
//!    of one component family, the descriptor set it surfaces, the replay-health
//!    mode it renders under, and any export caveats, and produces one
//!    [`M5AiResolvedReplayBinding`] carrying the derived claim-parity state and —
//!    whenever replayability is weakened — a self-contained [`M5AiAutoNarrowBanner`]
//!    that names the exact reason (route/provider/model drift, missing connector
//!    output, redaction fence, or stale approval), the descriptors that stay
//!    preserved, and the recovery action, rather than a generic "degraded" note.
//!    The resolver never lets a narrowed context drop a required descriptor and
//!    never invents a second execution grammar.
//! 2. A parity matrix — [`M5AiExecutionReplayConsumerPacket`] — that binds one row
//!    per claimed M5 execution/replay consumer to the eight canonical component
//!    families, the one shared descriptor vocabulary, the same replay-health modes,
//!    export caveats, parity states, narrowing reasons, recovery actions, export
//!    fields, and non-visual accessibility routes, so route/approval/checkpoint/
//!    replay-completeness facts stop diverging between the product UI, the docs, and
//!    the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, and downgrade triggers are reused verbatim from the
//! frozen AI execution/replay component matrix. This module mints new vocabulary
//! only for what the adoption lane itself needs: its execution/replay consumers,
//! the eight canonical component families and their canonical refs, the shared
//! descriptor vocabulary, the replay-health modes, the export caveats, the
//! claim-parity states, the narrowing reasons and recovery actions, the consumer
//! anatomy parts, and the export fields.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary; every label is carried only as an opaque,
//! export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-execution-replay-component-consumer.schema.json`](../../../../schemas/ai/m5-ai-execution-replay-component-consumer.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers.md`](../../../../docs/ai/m5/add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/m5-ai-execution-replay-component-consumers/`](../../../../fixtures/ai/m5/m5-ai-execution-replay-component-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_execution_replay_consumer_branch_queue_beta_narrowed,
    seeded_m5_ai_execution_replay_consumer_docs_help_preview_narrowed,
    seeded_m5_ai_execution_replay_consumer_packet, M5_AI_EXECUTION_REPLAY_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the AI
// execution/replay component matrix. This adoption lane reuses them verbatim so it
// never invents a parallel AI execution vocabulary.
pub use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5AiAccessibilityRoute, M5AiConsumerSurface, M5AiDeploymentLine, M5AiExecutionDowngradeTrigger,
    M5AiQualificationClass, M5AiSurfaceFamily,
};

// The canonical primitive schema / doc / artifact refs this adoption lane points
// every consumer at, rather than re-wording their facts in local prose.
use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5_AI_EXECUTION_COMPONENT_DOC_REF, M5_AI_EXECUTION_COMPONENT_SCHEMA_REF,
};
use crate::implement_ai_action_state_banners_and_boundary_blocked_banners_across_claimed_m5_inline_panel_review_agent_surfaces::{
    M5_AI_ACTION_STATE_BANNER_ARTIFACT_REF, M5_AI_ACTION_STATE_BANNER_DOC_REF,
    M5_AI_ACTION_STATE_BANNER_SCHEMA_REF,
};
use crate::implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces::{
    M5_AI_CONNECTOR_MODEL_ARTIFACT_REF, M5_AI_CONNECTOR_MODEL_DOC_REF,
    M5_AI_CONNECTOR_MODEL_SCHEMA_REF,
};
use crate::implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes::{
    M5_AI_APPROVAL_TOOL_CALL_ARTIFACT_REF, M5_AI_APPROVAL_TOOL_CALL_DOC_REF,
    M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF,
};
use crate::implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows::{
    M5_AI_BACKGROUND_AGENT_REPLAY_ARTIFACT_REF, M5_AI_BACKGROUND_AGENT_REPLAY_DOC_REF,
    M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF,
};
use crate::ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces::{
    M5_AI_RUN_HISTORY_EXPORT_ARTIFACT_REF, M5_AI_RUN_HISTORY_EXPORT_DOC_REF,
    M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiExecutionReplayConsumerPacket`].
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_RECORD_KIND: &str =
    "add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers";

/// Schema version for M5 AI execution/replay-component-consumer records.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the execution/replay-component-consumer boundary schema.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-execution-replay-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_DOC_REF: &str =
    "docs/ai/m5/add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers.md";

/// Repo-relative path of the frozen AI execution/replay component matrix this lane
/// adopts from.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_AI_EXECUTION_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_OBJECT_MODEL_REF: &str =
    M5_AI_EXECUTION_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ai/m5/m5-ai-execution-replay-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/ai/m5/m5-ai-execution-replay-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_CSV_REF: &str =
    "artifacts/ai/m5/m5-ai-execution-replay-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_EXECUTION_REPLAY_CONSUMER_REPORT_REF: &str =
    "artifacts/ai/m5/m5-ai-execution-replay-component-consumer-proof/report.md";

/// One claimed M5 AI execution/replay-component consumer that adopts the shared
/// components. These are the consumers the acceptance criteria name — patch review,
/// the evidence inspector, the branch/worktree agent queue, the support export, and
/// the docs/help surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionReplayConsumer {
    /// The patch-review surface.
    PatchReview,
    /// The evidence-inspector surface.
    EvidenceInspector,
    /// The branch/worktree agent queue.
    BranchWorktreeQueue,
    /// The support export.
    SupportExport,
    /// The docs/help surface.
    DocsHelp,
}

impl M5AiExecutionReplayConsumer {
    /// Every claimed execution/replay-component consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PatchReview,
        Self::EvidenceInspector,
        Self::BranchWorktreeQueue,
        Self::SupportExport,
        Self::DocsHelp,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PatchReview => "patch_review",
            Self::EvidenceInspector => "evidence_inspector",
            Self::BranchWorktreeQueue => "branch_worktree_queue",
            Self::SupportExport => "support_export",
            Self::DocsHelp => "docs_help",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PatchReview => "Patch Review",
            Self::EvidenceInspector => "Evidence Inspector",
            Self::BranchWorktreeQueue => "Branch / Worktree Queue",
            Self::SupportExport => "Support Export",
            Self::DocsHelp => "Docs / Help",
        }
    }

    /// True when this consumer is a docs/help surface — the surface the acceptance
    /// criteria single out for a canonical-schema reference so its prose can never
    /// drift from the product truth.
    pub const fn is_docs_or_help(self) -> bool {
        matches!(self, Self::DocsHelp)
    }
}

/// One canonical M5 AI execution/replay component family this lane adopts. Each
/// maps to exactly one narrowed primitive's canonical schema, doc, and
/// support-export artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSharedComponent {
    /// The action-state / boundary-blocked banner primitive.
    AiActionStateBanner,
    /// The connector detail row primitive.
    ConnectorDetailRow,
    /// The local-model pack card primitive.
    LocalModelPackCard,
    /// The high-friction approval sheet primitive.
    ApprovalSheet,
    /// The tool-call timeline row primitive.
    ToolCallTimelineRow,
    /// The run-history row primitive.
    RunHistoryRow,
    /// The replay / rerun-review sheet primitive.
    ReplayReview,
    /// The agent-status card primitive.
    AgentStatus,
}

impl M5AiSharedComponent {
    /// Every canonical component family, in declaration order.
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

    /// Review-safe label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AiActionStateBanner => "AI Action-State Banner",
            Self::ConnectorDetailRow => "Connector Detail Row",
            Self::LocalModelPackCard => "Local-Model Pack Card",
            Self::ApprovalSheet => "High-Friction Approval Sheet",
            Self::ToolCallTimelineRow => "Tool-Call Timeline Row",
            Self::RunHistoryRow => "Run-History Row",
            Self::ReplayReview => "Replay / Rerun-Review Sheet",
            Self::AgentStatus => "Agent-Status Card",
        }
    }

    /// The canonical boundary schema ref of the narrowed primitive that owns this
    /// family. A consumer that adopts this family must point at this schema, not at
    /// a local re-description.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::AiActionStateBanner => M5_AI_ACTION_STATE_BANNER_SCHEMA_REF,
            Self::ConnectorDetailRow | Self::LocalModelPackCard => M5_AI_CONNECTOR_MODEL_SCHEMA_REF,
            Self::ApprovalSheet | Self::ToolCallTimelineRow => M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF,
            Self::RunHistoryRow => M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF,
            Self::ReplayReview | Self::AgentStatus => M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF,
        }
    }

    /// The canonical contract-doc ref of the narrowed primitive that owns this
    /// family.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::AiActionStateBanner => M5_AI_ACTION_STATE_BANNER_DOC_REF,
            Self::ConnectorDetailRow | Self::LocalModelPackCard => M5_AI_CONNECTOR_MODEL_DOC_REF,
            Self::ApprovalSheet | Self::ToolCallTimelineRow => M5_AI_APPROVAL_TOOL_CALL_DOC_REF,
            Self::RunHistoryRow => M5_AI_RUN_HISTORY_EXPORT_DOC_REF,
            Self::ReplayReview | Self::AgentStatus => M5_AI_BACKGROUND_AGENT_REPLAY_DOC_REF,
        }
    }

    /// The canonical support-export artifact ref of the narrowed primitive that owns
    /// this family.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::AiActionStateBanner => M5_AI_ACTION_STATE_BANNER_ARTIFACT_REF,
            Self::ConnectorDetailRow | Self::LocalModelPackCard => {
                M5_AI_CONNECTOR_MODEL_ARTIFACT_REF
            }
            Self::ApprovalSheet | Self::ToolCallTimelineRow => {
                M5_AI_APPROVAL_TOOL_CALL_ARTIFACT_REF
            }
            Self::RunHistoryRow => M5_AI_RUN_HISTORY_EXPORT_ARTIFACT_REF,
            Self::ReplayReview | Self::AgentStatus => M5_AI_BACKGROUND_AGENT_REPLAY_ARTIFACT_REF,
        }
    }
}

/// The one shared descriptor vocabulary every execution/replay component keeps
/// aligned across surfaces, so no consumer invents a new grammar or stale wording.
/// The descriptors in [`M5AiReplayDescriptor::REQUIRED`] must be present on every
/// binding — the track invariant that route, approval, checkpoint lineage, and
/// replay completeness stay explicit everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReplayDescriptor {
    /// The route / provider / model descriptor.
    Route,
    /// The approval-gate descriptor.
    ApprovalGate,
    /// The checkpoint-lineage descriptor.
    CheckpointLineage,
    /// The replay-completeness descriptor.
    ReplayCompleteness,
}

impl M5AiReplayDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Route,
        Self::ApprovalGate,
        Self::CheckpointLineage,
        Self::ReplayCompleteness,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Route => "route",
            Self::ApprovalGate => "approval_gate",
            Self::CheckpointLineage => "checkpoint_lineage",
            Self::ReplayCompleteness => "replay_completeness",
        }
    }
}

/// The replay-health mode a consumer renders a component under. A weakened mode
/// still keeps the descriptor vocabulary — it only discloses that replayability is
/// narrowed relative to the authoritative in-product run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReplayHealth {
    /// Full replay: the authoritative in-product rendering.
    FullReplay,
    /// Route / provider / model drift weakens replayability.
    RouteProviderModelDrift,
    /// A missing connector output weakens replayability.
    MissingConnectorOutput,
    /// A redaction fence weakens replayability.
    RedactionFenced,
    /// A stale approval weakens replayability.
    StaleApproval,
}

impl M5AiReplayHealth {
    /// Every replay-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullReplay,
        Self::RouteProviderModelDrift,
        Self::MissingConnectorOutput,
        Self::RedactionFenced,
        Self::StaleApproval,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullReplay => "full_replay",
            Self::RouteProviderModelDrift => "route_provider_model_drift",
            Self::MissingConnectorOutput => "missing_connector_output",
            Self::RedactionFenced => "redaction_fenced",
            Self::StaleApproval => "stale_approval",
        }
    }

    /// True when the mode renders below the authoritative full replay and so must
    /// disclose a self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullReplay)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5AiNarrowingReason> {
        Some(match self {
            Self::RouteProviderModelDrift => M5AiNarrowingReason::RouteProviderModelDrift,
            Self::MissingConnectorOutput => M5AiNarrowingReason::MissingConnectorOutput,
            Self::RedactionFenced => M5AiNarrowingReason::RedactionFence,
            Self::StaleApproval => M5AiNarrowingReason::StaleApproval,
            Self::FullReplay => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its replay/resume claim language, so an
/// auto-narrow banner never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiNarrowingReason {
    /// The route / provider / model drifted from what the run recorded.
    RouteProviderModelDrift,
    /// A connector output the run depended on is missing.
    MissingConnectorOutput,
    /// A redaction fence withholds fields the claim would otherwise cover.
    RedactionFence,
    /// The approval that authorised the run has gone stale.
    StaleApproval,
}

impl M5AiNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RouteProviderModelDrift,
        Self::MissingConnectorOutput,
        Self::RedactionFence,
        Self::StaleApproval,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteProviderModelDrift => "route_provider_model_drift",
            Self::MissingConnectorOutput => "missing_connector_output",
            Self::RedactionFence => "redaction_fence",
            Self::StaleApproval => "stale_approval",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::RouteProviderModelDrift => {
                "the route, provider, or model drifted from what the run recorded"
            }
            Self::MissingConnectorOutput => "a connector output the run depended on is missing",
            Self::RedactionFence => "a redaction fence withholds fields this claim would cover",
            Self::StaleApproval => "the approval that authorised the run has gone stale",
        }
    }

    /// The recovery action a reader should take before trusting a full replay.
    pub const fn recovery_action(self) -> M5AiRecoveryAction {
        match self {
            Self::RouteProviderModelDrift => M5AiRecoveryAction::RerouteToDeclaredProvider,
            Self::MissingConnectorOutput => M5AiRecoveryAction::ReattachConnectorEvidence,
            Self::RedactionFence => M5AiRecoveryAction::ReplayWithinRedactionScope,
            Self::StaleApproval => M5AiRecoveryAction::RenewApprovalThenRerun,
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is
/// actionable from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRecoveryAction {
    /// Reroute to the provider / model the run declared.
    RerouteToDeclaredProvider,
    /// Reattach the missing connector evidence before replay.
    ReattachConnectorEvidence,
    /// Replay only within the redaction scope, not beyond it.
    ReplayWithinRedactionScope,
    /// Renew the approval, then rerun with re-review.
    RenewApprovalThenRerun,
}

impl M5AiRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RerouteToDeclaredProvider,
        Self::ReattachConnectorEvidence,
        Self::ReplayWithinRedactionScope,
        Self::RenewApprovalThenRerun,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerouteToDeclaredProvider => "reroute_to_declared_provider",
            Self::ReattachConnectorEvidence => "reattach_connector_evidence",
            Self::ReplayWithinRedactionScope => "replay_within_redaction_scope",
            Self::RenewApprovalThenRerun => "renew_approval_then_rerun",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the live
/// in-product run (a mirrored route, a partial replay, a redaction fence, or an
/// approval that must be re-verified).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExportCaveat {
    /// The route is shown from a mirror / cache, not the live provider.
    RouteMirroredNotLive,
    /// Only a partial replay is available.
    PartialReplayOnly,
    /// Redacted fields are withheld from this rendering.
    RedactedFieldsWithheld,
    /// The approval must be re-verified before it applies again.
    ApprovalReverificationRequired,
}

impl M5AiExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RouteMirroredNotLive,
        Self::PartialReplayOnly,
        Self::RedactedFieldsWithheld,
        Self::ApprovalReverificationRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteMirroredNotLive => "route_mirrored_not_live",
            Self::PartialReplayOnly => "partial_replay_only",
            Self::RedactedFieldsWithheld => "redacted_fields_withheld",
            Self::ApprovalReverificationRequired => "approval_reverification_required",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor
/// vocabulary is preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiClaimParityState {
    /// The descriptor vocabulary is preserved at full replay.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5AiClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5AiConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The replay-health cue.
    ReplayHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5AiConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ReplayHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ReplayHealthCue => "replay_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is
/// reconstructable from the shared model. The fields in
/// [`M5AiConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The replay-health mode.
    ReplayHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5AiConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ReplayHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ReplayHealth => "replay_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay
/// preserved, the export caveats, and the recovery action, so a narrowed rendering
/// is understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5AiNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5AiRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5AiExecutionReplayConsumer,
    /// The component family the banner applies to.
    pub component_family: M5AiSharedComponent,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5AiReplayDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5AiExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the replay-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReplayBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5AiExecutionReplayConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5AiSharedComponent,
    /// The descriptor set the binding surfaces. Must cover every required descriptor
    /// so route, approval, checkpoint lineage, and replay completeness stay explicit.
    pub descriptor_families: Vec<M5AiReplayDescriptor>,
    /// The replay-health mode the binding renders under.
    pub replay_health: M5AiReplayHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5AiExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiResolvedReplayBinding {
    /// The consumer.
    pub consumer: M5AiExecutionReplayConsumer,
    /// The component family.
    pub component_family: M5AiSharedComponent,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5AiReplayDescriptor>,
    /// The replay-health mode.
    pub replay_health: M5AiReplayHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5AiExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5AiClaimParityState,
    /// True when the binding renders under a weakened replay-health mode.
    pub is_narrowed: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5AiAutoNarrowBanner>,
}

/// Errors returned by [`resolve_replay_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiReplayBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5AiReplayBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5AiReplayBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "replay binding error: {}", self.as_str())
    }
}

impl Error for M5AiReplayBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the track invariant that route,
/// approval, checkpoint lineage, and replay completeness stay explicit on every
/// surface. The claim-parity state is preserved at full replay and auto-narrowed
/// under any weakened replay-health mode, and a weakened mode always produces a
/// self-contained banner naming the exact reason and recovery action while keeping
/// the descriptor vocabulary intact.
pub fn resolve_replay_binding(
    input: &M5AiReplayBindingInput,
) -> Result<M5AiResolvedReplayBinding, M5AiReplayBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5AiReplayBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5AiReplayDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5AiReplayDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5AiReplayBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5AiReplayBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future
        // free-text extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5AiReplayBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.replay_health.is_narrowed();
    let claim_parity_state = if is_narrowed {
        M5AiClaimParityState::ClaimsAutoNarrowed
    } else {
        M5AiClaimParityState::ClaimsPreserved
    };

    let auto_narrow_banner = input.replay_health.narrowing_reason().map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5AiAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5AiResolvedReplayBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: input.component_family.canonical_schema_ref().to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        replay_health: input.replay_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReplayBindingCase {
    /// The resolver input.
    pub input: M5AiReplayBindingInput,
    /// The resolved truth. Must equal `resolve_replay_binding(&input)`.
    pub resolved: M5AiResolvedReplayBinding,
}

impl M5AiReplayBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiReplayBindingInput) -> Self {
        let resolved = resolve_replay_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_replay_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs
/// the consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5AiSharedComponent,
    /// The canonical schema ref the consumer points at. Must equal the family's
    /// canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal
    /// the family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5AiReplayBindingCase>,
}

impl M5AiComponentBinding {
    /// True when the binding points at the family's canonical refs and references
    /// the canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == self.component_family.canonical_schema_ref()
            && self.canonical_artifact_ref == self.component_family.canonical_artifact_ref()
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one execution/replay-component consumer bound to
/// the canonical component families, the shared descriptor vocabulary, the
/// replay-health modes, export caveats, parity states, narrowing reasons, recovery
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionReplayConsumerRow {
    /// Execution/replay-component consumer.
    pub consumer: M5AiExecutionReplayConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5AiQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 AI surface families that render / consume this projection.
    pub surface_families: Vec<M5AiSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5AiDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5AiConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5AiReplayDescriptor>,
    /// Replay-health modes this consumer distinguishes.
    pub replay_health_modes: Vec<M5AiReplayHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5AiExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5AiClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5AiNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5AiRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5AiConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5AiAccessibilityRoute>,
    /// AI subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5AiConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5AiExecutionDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5AiComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be
    /// `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new execution grammar. MUST be
    /// `false`.
    pub invents_new_execution_grammar: bool,
    /// Hard invariant: this consumer never drops route or approval truth when
    /// narrowed. MUST be `false`.
    pub drops_route_or_approval_when_narrowed: bool,
    /// Hard invariant: this consumer never hides the drift reason or the
    /// manual-takeover path. MUST be `false`.
    pub hides_drift_reason_or_takeover_path: bool,
}

impl M5AiExecutionReplayConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AiConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AiConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5AiConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5AiReplayDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5AiReplayDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5AiComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5AiSharedComponent> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_execution_grammar
            && !self.drops_route_or_approval_when_narrowed
            && !self.hides_drift_reason_or_takeover_path
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionReplayConsumerVocabularySet {
    /// Execution/replay-component-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Replay-health-mode tokens.
    pub replay_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AiExecutionReplayConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5AiExecutionReplayConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5AiSharedComponent::ALL, |v| v.as_str()),
            descriptors: tokens(&M5AiReplayDescriptor::ALL, |v| v.as_str()),
            replay_health_modes: tokens(&M5AiReplayHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5AiExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5AiNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5AiRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5AiClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5AiConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AiConsumerExportField::ALL, |v| v.as_str()),
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
pub struct M5AiExecutionReplayConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new execution grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Route, approval, checkpoint lineage, and replay completeness stay explicit
    /// everywhere.
    pub descriptors_explicit_on_every_surface: bool,
    /// Drift, missing evidence, redaction, and stale approvals auto-narrow the claim.
    pub weakened_replay_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// Help / support / export consumers present the same run IDs, route truth, and
    /// drift reasons shown in-product.
    pub help_support_export_present_same_run_and_route_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionReplayConsumerProjection {
    /// Patch review, evidence inspector, branch/worktree queue, support export, and
    /// docs/help all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The route descriptor reads a single canonical source.
    pub route_reads_single_source: bool,
    /// The approval descriptor reads a single canonical source.
    pub approval_reads_single_source: bool,
    /// The checkpoint-lineage descriptor reads a single canonical source.
    pub checkpoint_reads_single_source: bool,
    /// The replay-completeness descriptor reads a single canonical source.
    pub replay_completeness_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionReplayConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionReplayConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI consumer audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiExecutionReplayConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiExecutionReplayConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5AiExecutionReplayConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiExecutionReplayConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiExecutionReplayConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiExecutionReplayConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiExecutionReplayConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiExecutionReplayConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 AI execution/replay-component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiExecutionReplayConsumerPacket {
    /// Record kind; must equal [`M5_AI_EXECUTION_REPLAY_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5AiExecutionReplayConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiExecutionReplayConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiExecutionReplayConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiExecutionReplayConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiExecutionReplayConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiExecutionReplayConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiExecutionReplayConsumerPacket {
    /// Builds an M5 AI execution/replay-component-consumer packet from stable-lane
    /// input.
    pub fn new(input: M5AiExecutionReplayConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_AI_EXECUTION_REPLAY_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 AI execution/replay-component-consumer invariants.
    pub fn validate(&self) -> Vec<M5AiExecutionReplayConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_EXECUTION_REPLAY_CONSUMER_RECORD_KIND {
            violations.push(M5AiExecutionReplayConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_VERSION {
            violations.push(M5AiExecutionReplayConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiExecutionReplayConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_docs_help_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai execution/replay component consumer packet serializes"),
        ) {
            violations.push(M5AiExecutionReplayConsumerViolation::RawMaterialInExport);
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
            .expect("m5 ai execution/replay component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,replay_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.replay_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 AI Execution/Replay-Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Execution/replay-component consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Replay-health modes: {}\n",
            self.vocabulary_set.replay_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Execution/replay-component consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.replay_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 AI execution/replay-component-consumer
/// export.
#[derive(Debug)]
pub enum M5AiExecutionReplayConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiExecutionReplayConsumerViolation>),
}

impl fmt::Display for M5AiExecutionReplayConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai execution/replay component consumer export parse failed: {error}"
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
                    "m5 ai execution/replay component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiExecutionReplayConsumerArtifactError {}

/// Validation failures emitted by [`M5AiExecutionReplayConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiExecutionReplayConsumerViolation {
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
    /// A required execution/replay-component consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one
    /// consumer (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-replay rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// A docs/help consumer does not reference the canonical component schema.
    DocsHelpReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5AiExecutionReplayConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::DocsHelpReferenceMissing => "docs_help_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 AI execution/replay-component-consumer
/// export.
pub fn current_stable_m5_ai_execution_replay_consumer_export(
) -> Result<M5AiExecutionReplayConsumerPacket, M5AiExecutionReplayConsumerArtifactError> {
    let packet: M5AiExecutionReplayConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-ai-execution-replay-component-consumer-proof/support_export.json"
    )))
    .map_err(M5AiExecutionReplayConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiExecutionReplayConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_EXECUTION_REPLAY_CONSUMER_SCHEMA_REF,
        M5_AI_EXECUTION_REPLAY_CONSUMER_DOC_REF,
        M5_AI_EXECUTION_REPLAY_CONSUMER_COMPONENT_MATRIX_REF,
        M5_AI_ACTION_STATE_BANNER_SCHEMA_REF,
        M5_AI_CONNECTOR_MODEL_SCHEMA_REF,
        M5_AI_APPROVAL_TOOL_CALL_SCHEMA_REF,
        M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF,
        M5_AI_BACKGROUND_AGENT_REPLAY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiExecutionReplayConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiExecutionReplayConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let present: BTreeSet<M5AiExecutionReplayConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5AiExecutionReplayConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5AiExecutionReplayConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.replay_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5AiExecutionReplayConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AiExecutionReplayConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5AiExecutionReplayConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AiExecutionReplayConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AiAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AiExecutionReplayConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiExecutionReplayConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiExecutionReplayConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5AiExecutionReplayConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5AiExecutionReplayConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5AiExecutionReplayConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5AiExecutionReplayConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiExecutionReplayConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiExecutionReplayConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct
/// consumers — the acceptance-criterion proof that the families are reusable
/// components rather than one assistant panel plus a few admin-only pages.
fn validate_family_reuse(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    for family in M5AiSharedComponent::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5AiExecutionReplayConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering
/// whose banner carries a specific reason, a recovery action, and a non-empty set of
/// preserved descriptors — the acceptance-criterion example that execution/replay
/// components stay truthful when route drift, missing evidence, redaction, or stale
/// approvals weaken replayability.
fn validate_narrowing_disclosure(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5AiExecutionReplayConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-replay rendering
/// with preserved parity and no banner — the acceptance-criterion example that
/// full-replay consumers keep the descriptor vocabulary without a spurious narrowing
/// note.
fn validate_scope_preserved(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5AiClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5AiExecutionReplayConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every docs/help consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that docs/help prose can never drift
/// from the product truth.
fn validate_docs_help_reference(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_docs_or_help() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5AiComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5AiExecutionReplayConsumerViolation::DocsHelpReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.descriptors_explicit_on_every_surface,
        review.weakened_replay_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.help_support_export_present_same_run_and_route_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5AiExecutionReplayConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.route_reads_single_source,
        projection.approval_reads_single_source,
        projection.checkpoint_reads_single_source,
        projection.replay_completeness_reads_single_source,
    ] {
        if !ok {
            violations.push(M5AiExecutionReplayConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiExecutionReplayConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiExecutionReplayConsumerPacket,
    violations: &mut Vec<M5AiExecutionReplayConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiExecutionReplayConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5AiExecutionReplayConsumerPacket,
) -> impl Iterator<Item = &M5AiReplayBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
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
