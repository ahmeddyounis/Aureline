//! Three reusable M5 AI history / export primitives — the AI run-history row, the
//! approval-timeline entry, and the evidence / export summary — so prior AI work stays
//! auditable and shareable without reconstructing it from generic logs.
//!
//! Aureline's frozen AI-execution/replay component matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! names the run-history row and the replay/rerun-review sheet as governed component
//! families and freezes their controlled vocabulary — the run outcomes, execution modes,
//! approval gates, replay-completeness states, surface families, deployment lines,
//! consumer surfaces, accessibility routes, qualification classes, and downgrade
//! triggers. This module *implements* the history / export side of that matrix as three
//! reusable primitives so a user can tell — from the row, the entry, or the summary alone
//! — which AI run they are looking at, which approvals influenced it, and what an export
//! actually contains, without reconstructing any of it from raw logs.
//!
//! The module has three resolvers:
//!
//! 1. [`resolve_run_history_row`] — takes one AI run's canonical run id, task label,
//!    time, provider / model route, execution mode, and outcome, and produces one
//!    [`M5ResolvedRunHistoryRow`] carrying the same canonical run identity, the composed
//!    provider/model route, the run outcome, and the stable open / replay / export entry
//!    points. It never masks the provider/model route or drops the run identity.
//! 2. [`resolve_approval_timeline_entry`] — takes one approval that influenced the run —
//!    its opaque id, actor, actor class, grant scope, policy epoch, satisfied gate, and
//!    expiry state — and produces one [`M5ResolvedApprovalTimelineEntry`] preserving the
//!    distinct actor, scope, policy epoch, and expiry state, deriving whether the grant is
//!    still effective, so approval history is inspectable and never collapses multiple
//!    distinct grants into one vague "approved" badge.
//! 3. [`resolve_evidence_export_summary`] — takes one evidence packet's id, the run it
//!    belongs to, its included artifact classes, redaction posture, support-packet
//!    linkage, and supported export formats, and produces one
//!    [`M5ResolvedEvidenceExportSummary`] carrying whether the packet is safe to share and
//!    whether it preserves redaction and support-continuity state, so an export summary is
//!    never reduced to a bare raw-file download link.
//!
//! A single parity matrix — [`M5AiRunHistoryExportPrimitivePacket`] — binds one row per
//! claimed M5 replay surface (run-history, evidence-packet, export, support, and replay)
//! to the shared run-history, approval-timeline, and evidence-summary anatomy, the same
//! run outcomes, execution modes, approval gates, actor classes, grant scopes, expiry
//! states, artifact classes, redaction postures, support linkages, export formats, entry
//! points, export fields, and non-visual accessibility routes, so the same AI run
//! identity, approval grammar, and export vocabulary stay identical across every surface a
//! user reviews, reruns, exports, or hands off AI work through.
//!
//! The run outcome ([`M5AiRunOutcome`]), execution mode ([`M5AiExecutionMode`]), approval
//! gate ([`M5AiApprovalGate`]), surface family ([`M5AiSurfaceFamily`]), deployment line
//! ([`M5AiDeploymentLine`]), consumer surface ([`M5AiConsumerSurface`]), accessibility
//! route ([`M5AiAccessibilityRoute`]), qualification class ([`M5AiQualificationClass`]),
//! and downgrade trigger ([`M5AiExecutionDowngradeTrigger`]) are reused verbatim from the
//! frozen matrix. This module mints new vocabulary only for what that matrix left implicit
//! about the row, the entry, and the summary themselves: their replay surfaces, their
//! anatomy parts, their entry points, their approval actor classes, grant scopes, expiry
//! states, their evidence artifact classes, redaction postures, support linkages, export
//! formats, and their export fields. No M5 AI surface invents a second run-history,
//! approval, or export grammar.
//!
//! Raw prompt bodies, raw tool return bodies, raw paths, raw URLs, and credential material
//! stay outside the support boundary; every run id, task label, time label, actor label,
//! policy epoch label, and packet id is carried only as an opaque, export-safe
//! representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-run-history-row-approval-timeline-entry-and-evidence-export-summary.schema.json`](../../../../schemas/ai/m5-ai-run-history-row-approval-timeline-entry-and-evidence-export-summary.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces.md`](../../../../docs/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed,
    seeded_m5_ai_run_history_export_primitive_packet,
    seeded_m5_ai_run_history_export_primitive_support_beta_narrowed,
    M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_PACKET_ID,
};

// The run outcome, execution mode, approval gate, surface family, deployment line,
// consumer surface, accessibility route, qualification class, and downgrade triggers are
// frozen once, in the AI-execution/replay component matrix. These primitives reuse them
// verbatim so they never invent a parallel run-history, approval, or export vocabulary.
pub use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5AiAccessibilityRoute, M5AiApprovalGate, M5AiConsumerSurface, M5AiDeploymentLine,
    M5AiExecutionDowngradeTrigger, M5AiExecutionMode, M5AiQualificationClass, M5AiRunOutcome,
    M5AiSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiRunHistoryExportPrimitivePacket`].
pub const M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_RECORD_KIND: &str =
    "ship_m5_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces";

/// Schema version for M5 AI run-history / approval-timeline / evidence-export primitive
/// records.
pub const M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the run-history / approval-timeline / evidence-export schema.
pub const M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-run-history-row-approval-timeline-entry-and-evidence-export-summary.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_RUN_HISTORY_EXPORT_DOC_REF: &str =
    "docs/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces.md";

/// Repo-relative path of the frozen AI-execution/replay component matrix these primitives
/// narrow from.
pub const M5_AI_RUN_HISTORY_EXPORT_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json";

/// Repo-relative path of the AI-run-history-entry contract this primitive binds its
/// run-identity, route, and outcome truth against.
pub const M5_AI_RUN_HISTORY_EXPORT_RUN_HISTORY_REF: &str =
    "schemas/ai/ai_run_history_entry.schema.json";

/// Repo-relative path of the approval-action-class contract this primitive binds its
/// actor, scope, and gate vocabulary against, so an approval-timeline entry preserves the
/// same approval grammar as the policy and evidence systems.
pub const M5_AI_RUN_HISTORY_EXPORT_APPROVAL_REF: &str =
    "schemas/ai/approval_action_class.schema.json";

/// Repo-relative path of the evidence-replay-packet contract this primitive binds its
/// packet-id, artifact-class, redaction, and export-format truth against.
pub const M5_AI_RUN_HISTORY_EXPORT_EVIDENCE_REF: &str =
    "schemas/ai/evidence_replay_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_RUN_HISTORY_EXPORT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_RUN_HISTORY_EXPORT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_RUN_HISTORY_EXPORT_CSV_REF: &str =
    "artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_RUN_HISTORY_EXPORT_REPORT_REF: &str =
    "artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces.md";

/// One claimed M5 replay surface that renders the shared run-history row, approval-timeline
/// entry, and evidence / export summary. These are the surfaces the acceptance criteria
/// name — history, evidence, export, support, and replay — where the same AI run identity
/// must stay consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReplaySurface {
    /// The run-history surface.
    RunHistory,
    /// The evidence-packet surface.
    EvidencePacket,
    /// The export surface.
    Export,
    /// The support-desk surface.
    Support,
    /// The replay / rerun-review surface.
    Replay,
}

impl M5AiReplaySurface {
    /// Every claimed replay surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunHistory,
        Self::EvidencePacket,
        Self::Export,
        Self::Support,
        Self::Replay,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunHistory => "run_history",
            Self::EvidencePacket => "evidence_packet",
            Self::Export => "export",
            Self::Support => "support",
            Self::Replay => "replay",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RunHistory => "Run-History",
            Self::EvidencePacket => "Evidence Packet",
            Self::Export => "Export",
            Self::Support => "Support Desk",
            Self::Replay => "Replay / Rerun-Review",
        }
    }
}

/// A stable entry point an AI run-history row offers, so prior work stays openable,
/// replayable, and exportable rather than a dead log line. The entry points in
/// [`M5AiRunHistoryEntryPoint::MANDATORY`] are on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRunHistoryEntryPoint {
    /// Open the run.
    OpenRun,
    /// Replay the run.
    ReplayRun,
    /// Export the run's evidence.
    ExportEvidence,
    /// View the linked support packet.
    ViewSupportPacket,
    /// Inspect the approvals that influenced the run.
    InspectApprovals,
}

impl M5AiRunHistoryEntryPoint {
    /// Every entry point, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenRun,
        Self::ReplayRun,
        Self::ExportEvidence,
        Self::ViewSupportPacket,
        Self::InspectApprovals,
    ];

    /// The stable open / replay / export entry points every run-history row must offer.
    pub const MANDATORY: [Self; 3] = [Self::OpenRun, Self::ReplayRun, Self::ExportEvidence];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRun => "open_run",
            Self::ReplayRun => "replay_run",
            Self::ExportEvidence => "export_evidence",
            Self::ViewSupportPacket => "view_support_packet",
            Self::InspectApprovals => "inspect_approvals",
        }
    }
}

/// Controlled run-history row anatomy part. The parts in
/// [`M5AiRunHistoryAnatomyPart::MANDATORY`] are required so the canonical run id, task
/// label, time, provider/model route, outcome, and entry points stay visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRunHistoryAnatomyPart {
    /// The canonical run id.
    RunIdCue,
    /// The task label.
    TaskLabelCue,
    /// The time the run occurred.
    TimeCue,
    /// The provider / model route.
    RouteCue,
    /// The run outcome.
    OutcomeCue,
    /// The stable entry-point row.
    EntryPointCue,
}

impl M5AiRunHistoryAnatomyPart {
    /// Every run-history anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunIdCue,
        Self::TaskLabelCue,
        Self::TimeCue,
        Self::RouteCue,
        Self::OutcomeCue,
        Self::EntryPointCue,
    ];

    /// The run-history anatomy parts every row must render.
    pub const MANDATORY: [Self; 6] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunIdCue => "run_id_cue",
            Self::TaskLabelCue => "task_label_cue",
            Self::TimeCue => "time_cue",
            Self::RouteCue => "route_cue",
            Self::OutcomeCue => "outcome_cue",
            Self::EntryPointCue => "entry_point_cue",
        }
    }
}

/// Controlled approval-timeline entry anatomy part. The parts in
/// [`M5AiApprovalTimelineAnatomyPart::MANDATORY`] are required so actor, scope, policy
/// epoch, expiry, and inspectability stay visible and never collapse into one badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalTimelineAnatomyPart {
    /// The granting actor.
    ActorCue,
    /// The grant scope.
    ScopeCue,
    /// The policy epoch the grant was made under.
    PolicyEpochCue,
    /// The gate the grant satisfied.
    GateCue,
    /// The expiry state.
    ExpiryCue,
    /// The inspect / open-detail affordance.
    InspectCue,
}

impl M5AiApprovalTimelineAnatomyPart {
    /// Every approval-timeline anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActorCue,
        Self::ScopeCue,
        Self::PolicyEpochCue,
        Self::GateCue,
        Self::ExpiryCue,
        Self::InspectCue,
    ];

    /// The approval-timeline anatomy parts every entry must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ActorCue,
        Self::ScopeCue,
        Self::PolicyEpochCue,
        Self::ExpiryCue,
        Self::InspectCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorCue => "actor_cue",
            Self::ScopeCue => "scope_cue",
            Self::PolicyEpochCue => "policy_epoch_cue",
            Self::GateCue => "gate_cue",
            Self::ExpiryCue => "expiry_cue",
            Self::InspectCue => "inspect_cue",
        }
    }
}

/// Controlled evidence / export summary anatomy part. The parts in
/// [`M5AiEvidenceSummaryAnatomyPart::MANDATORY`] are required so packet id, artifact
/// classes, redaction posture, support linkage, and export formats stay visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiEvidenceSummaryAnatomyPart {
    /// The evidence packet id.
    PacketIdCue,
    /// The run the packet belongs to.
    RunLinkCue,
    /// The included artifact classes.
    ArtifactClassCue,
    /// The redaction posture.
    RedactionCue,
    /// The support-packet linkage.
    SupportLinkageCue,
    /// The supported export formats.
    ExportFormatCue,
}

impl M5AiEvidenceSummaryAnatomyPart {
    /// Every evidence-summary anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PacketIdCue,
        Self::RunLinkCue,
        Self::ArtifactClassCue,
        Self::RedactionCue,
        Self::SupportLinkageCue,
        Self::ExportFormatCue,
    ];

    /// The evidence-summary anatomy parts every summary must render.
    pub const MANDATORY: [Self; 5] = [
        Self::PacketIdCue,
        Self::ArtifactClassCue,
        Self::RedactionCue,
        Self::SupportLinkageCue,
        Self::ExportFormatCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketIdCue => "packet_id_cue",
            Self::RunLinkCue => "run_link_cue",
            Self::ArtifactClassCue => "artifact_class_cue",
            Self::RedactionCue => "redaction_cue",
            Self::SupportLinkageCue => "support_linkage_cue",
            Self::ExportFormatCue => "export_format_cue",
        }
    }
}

/// Controlled approval actor class — who granted an approval that influenced the run, so
/// an approval-timeline entry never leaves the granting authority implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalActorClass {
    /// The workspace owner.
    WorkspaceOwner,
    /// A delegated reviewer.
    DelegatedReviewer,
    /// A security reviewer.
    SecurityReviewer,
    /// The policy engine (automated policy grant).
    PolicyEngine,
    /// An automation agent acting under delegated authority.
    AutomationAgent,
}

impl M5AiApprovalActorClass {
    /// Every actor class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WorkspaceOwner,
        Self::DelegatedReviewer,
        Self::SecurityReviewer,
        Self::PolicyEngine,
        Self::AutomationAgent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceOwner => "workspace_owner",
            Self::DelegatedReviewer => "delegated_reviewer",
            Self::SecurityReviewer => "security_reviewer",
            Self::PolicyEngine => "policy_engine",
            Self::AutomationAgent => "automation_agent",
        }
    }
}

/// Controlled approval grant scope — how far a grant reaches, so an approval-timeline
/// entry never collapses a broad standing grant into the same badge as a single action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalGrantScope {
    /// A single action.
    SingleAction,
    /// The current session.
    Session,
    /// A whole task.
    Task,
    /// The whole workspace.
    Workspace,
    /// A whole tenant.
    Tenant,
    /// A global standing grant.
    Global,
}

impl M5AiApprovalGrantScope {
    /// Every grant scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleAction,
        Self::Session,
        Self::Task,
        Self::Workspace,
        Self::Tenant,
        Self::Global,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleAction => "single_action",
            Self::Session => "session",
            Self::Task => "task",
            Self::Workspace => "workspace",
            Self::Tenant => "tenant",
            Self::Global => "global",
        }
    }

    /// True when the grant reaches beyond a single task (a broad standing grant).
    pub const fn is_standing_grant(self) -> bool {
        matches!(self, Self::Workspace | Self::Tenant | Self::Global)
    }
}

/// Controlled approval expiry state — whether a grant is still in force, so an
/// approval-timeline entry never shows an expired, revoked, or consumed grant as if it
/// still applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalExpiryState {
    /// Active, not near expiry.
    Active,
    /// Active but expiring soon.
    ExpiringSoon,
    /// Expired.
    Expired,
    /// Revoked before expiry.
    Revoked,
    /// A single-use grant that has been consumed.
    SingleUseConsumed,
    /// A standing grant with no expiry.
    NoExpiry,
}

impl M5AiApprovalExpiryState {
    /// Every expiry state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Active,
        Self::ExpiringSoon,
        Self::Expired,
        Self::Revoked,
        Self::SingleUseConsumed,
        Self::NoExpiry,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::SingleUseConsumed => "single_use_consumed",
            Self::NoExpiry => "no_expiry",
        }
    }

    /// True when a grant in this state is still in force.
    pub const fn is_effective(self) -> bool {
        matches!(self, Self::Active | Self::ExpiringSoon | Self::NoExpiry)
    }
}

/// Controlled evidence artifact class — one class of artifact an evidence packet includes,
/// so an export summary discloses what it actually carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiEvidenceArtifactClass {
    /// The prompt transcript.
    PromptTranscript,
    /// The tool-call log.
    ToolCallLog,
    /// The diff packet.
    DiffPacket,
    /// The route receipt.
    RouteReceipt,
    /// The spend receipt.
    SpendReceipt,
    /// The approval lineage.
    ApprovalLineage,
    /// The validation receipt.
    ValidationReceipt,
    /// The redaction manifest.
    RedactionManifest,
}

impl M5AiEvidenceArtifactClass {
    /// Every artifact class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PromptTranscript,
        Self::ToolCallLog,
        Self::DiffPacket,
        Self::RouteReceipt,
        Self::SpendReceipt,
        Self::ApprovalLineage,
        Self::ValidationReceipt,
        Self::RedactionManifest,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptTranscript => "prompt_transcript",
            Self::ToolCallLog => "tool_call_log",
            Self::DiffPacket => "diff_packet",
            Self::RouteReceipt => "route_receipt",
            Self::SpendReceipt => "spend_receipt",
            Self::ApprovalLineage => "approval_lineage",
            Self::ValidationReceipt => "validation_receipt",
            Self::RedactionManifest => "redaction_manifest",
        }
    }
}

/// Controlled redaction posture — how far an evidence packet has been redacted, so an
/// export summary never presents an unredacted or redaction-failed packet as safe to
/// share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRedactionPosture {
    /// Fully redacted.
    FullyRedacted,
    /// Secrets redacted.
    CredentialsRedacted,
    /// PII redacted.
    PiiRedacted,
    /// Redaction pending.
    RedactionPending,
    /// Unredacted.
    Unredacted,
    /// Redaction failed.
    RedactionFailed,
}

impl M5AiRedactionPosture {
    /// Every redaction posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyRedacted,
        Self::CredentialsRedacted,
        Self::PiiRedacted,
        Self::RedactionPending,
        Self::Unredacted,
        Self::RedactionFailed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyRedacted => "fully_redacted",
            Self::CredentialsRedacted => "credentials_redacted",
            Self::PiiRedacted => "pii_redacted",
            Self::RedactionPending => "redaction_pending",
            Self::Unredacted => "unredacted",
            Self::RedactionFailed => "redaction_failed",
        }
    }

    /// True when a packet in this posture has had at least its secrets removed and can be
    /// shared outside the trust boundary.
    pub const fn is_share_safe(self) -> bool {
        matches!(
            self,
            Self::FullyRedacted | Self::CredentialsRedacted | Self::PiiRedacted
        )
    }
}

/// Controlled support-packet linkage — how an evidence packet is tied to a support case,
/// so an export summary preserves support-continuity state instead of a bare download.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSupportLinkage {
    /// Linked to an open support ticket.
    LinkedOpenTicket,
    /// Linked to a resolved support ticket.
    LinkedResolvedTicket,
    /// Linked to an internal case.
    LinkedInternalCase,
    /// Linkage pending consent.
    LinkagePendingConsent,
    /// Not linked to any case.
    NotLinked,
}

impl M5AiSupportLinkage {
    /// Every support linkage, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LinkedOpenTicket,
        Self::LinkedResolvedTicket,
        Self::LinkedInternalCase,
        Self::LinkagePendingConsent,
        Self::NotLinked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinkedOpenTicket => "linked_open_ticket",
            Self::LinkedResolvedTicket => "linked_resolved_ticket",
            Self::LinkedInternalCase => "linked_internal_case",
            Self::LinkagePendingConsent => "linkage_pending_consent",
            Self::NotLinked => "not_linked",
        }
    }

    /// True when the packet carries an active support-continuity link.
    pub const fn is_linked(self) -> bool {
        matches!(
            self,
            Self::LinkedOpenTicket | Self::LinkedResolvedTicket | Self::LinkedInternalCase
        )
    }
}

/// Controlled export format — one format an export summary can produce, so a summary is
/// never reduced to a single opaque file type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExportFormat {
    /// A JSON bundle.
    JsonBundle,
    /// A Markdown report.
    MarkdownReport,
    /// A CSV table.
    CsvTable,
    /// A signed archive.
    SignedArchive,
    /// A redacted PDF.
    RedactedPdf,
}

impl M5AiExportFormat {
    /// Every export format, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::JsonBundle,
        Self::MarkdownReport,
        Self::CsvTable,
        Self::SignedArchive,
        Self::RedactedPdf,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonBundle => "json_bundle",
            Self::MarkdownReport => "markdown_report",
            Self::CsvTable => "csv_table",
            Self::SignedArchive => "signed_archive",
            Self::RedactedPdf => "redacted_pdf",
        }
    }
}

/// A field the run-history export carries so row truth is reconstructable. The fields in
/// [`M5AiRunHistoryExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRunHistoryExportField {
    /// The canonical run id.
    RunId,
    /// The task label.
    TaskLabel,
    /// The time the run occurred.
    OccurredAt,
    /// The provider / model route.
    ProviderModelRoute,
    /// The execution mode.
    ExecutionMode,
    /// The run outcome.
    Outcome,
    /// The stable entry points.
    EntryPoints,
}

impl M5AiRunHistoryExportField {
    /// Every run-history export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RunId,
        Self::TaskLabel,
        Self::OccurredAt,
        Self::ProviderModelRoute,
        Self::ExecutionMode,
        Self::Outcome,
        Self::EntryPoints,
    ];

    /// The run-history export fields every row must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::RunId,
        Self::TaskLabel,
        Self::OccurredAt,
        Self::ProviderModelRoute,
        Self::Outcome,
        Self::EntryPoints,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunId => "run_id",
            Self::TaskLabel => "task_label",
            Self::OccurredAt => "occurred_at",
            Self::ProviderModelRoute => "provider_model_route",
            Self::ExecutionMode => "execution_mode",
            Self::Outcome => "outcome",
            Self::EntryPoints => "entry_points",
        }
    }
}

/// A field the approval-timeline export carries so entry truth is reconstructable. The
/// fields in [`M5AiApprovalTimelineExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiApprovalTimelineExportField {
    /// The approval id.
    ApprovalId,
    /// The granting actor.
    Actor,
    /// The grant scope.
    GrantScope,
    /// The policy epoch.
    PolicyEpoch,
    /// The satisfied gate.
    Gate,
    /// The expiry state.
    ExpiryState,
    /// Whether the grant is still effective.
    Effective,
}

impl M5AiApprovalTimelineExportField {
    /// Every approval-timeline export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ApprovalId,
        Self::Actor,
        Self::GrantScope,
        Self::PolicyEpoch,
        Self::Gate,
        Self::ExpiryState,
        Self::Effective,
    ];

    /// The approval-timeline export fields every entry must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ApprovalId,
        Self::Actor,
        Self::GrantScope,
        Self::PolicyEpoch,
        Self::ExpiryState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApprovalId => "approval_id",
            Self::Actor => "actor",
            Self::GrantScope => "grant_scope",
            Self::PolicyEpoch => "policy_epoch",
            Self::Gate => "gate",
            Self::ExpiryState => "expiry_state",
            Self::Effective => "effective",
        }
    }
}

/// A field the evidence / export summary carries so summary truth is reconstructable. The
/// fields in [`M5AiEvidenceSummaryExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiEvidenceSummaryExportField {
    /// The evidence packet id.
    PacketId,
    /// The run the packet belongs to.
    RunId,
    /// The included artifact classes.
    ArtifactClasses,
    /// The redaction posture.
    RedactionPosture,
    /// The support linkage.
    SupportLinkage,
    /// The supported export formats.
    ExportFormats,
    /// Whether the packet is safe to share.
    Shareable,
}

impl M5AiEvidenceSummaryExportField {
    /// Every evidence-summary export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PacketId,
        Self::RunId,
        Self::ArtifactClasses,
        Self::RedactionPosture,
        Self::SupportLinkage,
        Self::ExportFormats,
        Self::Shareable,
    ];

    /// The evidence-summary export fields every summary must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::PacketId,
        Self::RunId,
        Self::ArtifactClasses,
        Self::RedactionPosture,
        Self::SupportLinkage,
        Self::ExportFormats,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketId => "packet_id",
            Self::RunId => "run_id",
            Self::ArtifactClasses => "artifact_classes",
            Self::RedactionPosture => "redaction_posture",
            Self::SupportLinkage => "support_linkage",
            Self::ExportFormats => "export_formats",
            Self::Shareable => "shareable",
        }
    }
}

// ---- run-history-row resolver -------------------------------------------

/// The full input to the run-history-row resolver for one AI run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryResolutionInput {
    /// The opaque canonical run id.
    pub canonical_run_id: String,
    /// The opaque task label.
    pub task_label: String,
    /// The opaque time-occurred label.
    pub occurred_at_label: String,
    /// The opaque provider label.
    pub provider_label: String,
    /// The opaque model label.
    pub model_label: String,
    /// The execution mode the run ran in.
    pub execution_mode: M5AiExecutionMode,
    /// How the run ended.
    pub run_outcome: M5AiRunOutcome,
    /// True when the run is linked to a support packet.
    pub support_linked: bool,
    /// True when approvals influenced the run.
    pub has_approvals: bool,
}

/// The resolved run-history-row truth for one AI run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRunHistoryRow {
    /// The opaque canonical run id.
    pub canonical_run_id: String,
    /// The opaque task label.
    pub task_label: String,
    /// The opaque time-occurred label.
    pub occurred_at_label: String,
    /// The opaque provider label.
    pub provider_label: String,
    /// The opaque model label.
    pub model_label: String,
    /// The composed provider / model route label.
    pub route_label: String,
    /// The execution mode the run ran in.
    pub execution_mode: M5AiExecutionMode,
    /// How the run ended.
    pub run_outcome: M5AiRunOutcome,
    /// True when the provider and model are both named (route is not masked).
    pub route_is_complete: bool,
    /// The stable entry points this row offers.
    pub entry_points: Vec<M5AiRunHistoryEntryPoint>,
}

/// Errors returned by [`resolve_run_history_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiRunHistoryResolutionError {
    /// The canonical run id was empty.
    EmptyRunId,
    /// The task label was empty.
    EmptyTaskLabel,
    /// The time-occurred label was empty.
    EmptyOccurredAt,
    /// The provider or model route was masked (empty), leaving the route implicit.
    RouteProviderModelMasked,
    /// A run-history descriptor carried forbidden material.
    ForbiddenRunHistoryMaterial,
}

impl M5AiRunHistoryResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyTaskLabel => "empty_task_label",
            Self::EmptyOccurredAt => "empty_occurred_at",
            Self::RouteProviderModelMasked => "route_provider_model_masked",
            Self::ForbiddenRunHistoryMaterial => "forbidden_run_history_material",
        }
    }
}

impl fmt::Display for M5AiRunHistoryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "ai run history resolution error: {}", self.as_str())
    }
}

impl Error for M5AiRunHistoryResolutionError {}

/// Resolves one AI run-history row from a run's declared state.
///
/// The canonical run id is carried through verbatim so the same AI run identity appears
/// consistently across history, evidence, export, support, and replay surfaces. The
/// provider and model must both be named — a masked route is rejected as
/// [`M5AiRunHistoryResolutionError::RouteProviderModelMasked`] rather than shown as an
/// anonymous run. The row always offers the stable open / replay / export entry points,
/// adds the support-packet entry point when the run is support-linked, and adds the
/// approvals entry point when approvals influenced the run.
pub fn resolve_run_history_row(
    input: &M5AiRunHistoryResolutionInput,
) -> Result<M5ResolvedRunHistoryRow, M5AiRunHistoryResolutionError> {
    if input.canonical_run_id.trim().is_empty() {
        return Err(M5AiRunHistoryResolutionError::EmptyRunId);
    }
    if input.task_label.trim().is_empty() {
        return Err(M5AiRunHistoryResolutionError::EmptyTaskLabel);
    }
    if input.occurred_at_label.trim().is_empty() {
        return Err(M5AiRunHistoryResolutionError::EmptyOccurredAt);
    }
    if input.provider_label.trim().is_empty() || input.model_label.trim().is_empty() {
        return Err(M5AiRunHistoryResolutionError::RouteProviderModelMasked);
    }
    for value in [
        &input.canonical_run_id,
        &input.task_label,
        &input.occurred_at_label,
        &input.provider_label,
        &input.model_label,
    ] {
        if value_repr_is_forbidden(value) {
            return Err(M5AiRunHistoryResolutionError::ForbiddenRunHistoryMaterial);
        }
    }

    let route_label = format!(
        "{} / {}",
        input.provider_label.trim(),
        input.model_label.trim()
    );
    let mut entry_points = vec![
        M5AiRunHistoryEntryPoint::OpenRun,
        M5AiRunHistoryEntryPoint::ReplayRun,
        M5AiRunHistoryEntryPoint::ExportEvidence,
    ];
    if input.support_linked {
        entry_points.push(M5AiRunHistoryEntryPoint::ViewSupportPacket);
    }
    if input.has_approvals {
        entry_points.push(M5AiRunHistoryEntryPoint::InspectApprovals);
    }

    Ok(M5ResolvedRunHistoryRow {
        canonical_run_id: input.canonical_run_id.clone(),
        task_label: input.task_label.clone(),
        occurred_at_label: input.occurred_at_label.clone(),
        provider_label: input.provider_label.clone(),
        model_label: input.model_label.clone(),
        route_label,
        execution_mode: input.execution_mode,
        run_outcome: input.run_outcome,
        route_is_complete: true,
        entry_points,
    })
}

// ---- approval-timeline-entry resolver -----------------------------------

/// The full input to the approval-timeline-entry resolver for one approval that influenced
/// a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalTimelineResolutionInput {
    /// The opaque approval id.
    pub approval_id: String,
    /// The canonical run id this approval influenced.
    pub run_id_label: String,
    /// The opaque granting-actor label.
    pub actor_label: String,
    /// The class of the granting actor.
    pub actor_class: M5AiApprovalActorClass,
    /// The scope the grant reaches.
    pub grant_scope: M5AiApprovalGrantScope,
    /// The opaque policy-epoch label the grant was made under.
    pub policy_epoch_label: String,
    /// The gate the grant satisfied.
    pub gate: M5AiApprovalGate,
    /// The opaque expiry-timestamp label (empty when the grant has no expiry).
    pub expiry_label: String,
    /// True when the grant carries an expiry timestamp.
    pub has_expiry: bool,
    /// True when the grant was revoked before expiry.
    pub is_revoked: bool,
    /// True when the grant is single-use.
    pub is_single_use: bool,
    /// True when a single-use grant has already been consumed.
    pub single_use_consumed: bool,
    /// True when the grant has passed its expiry.
    pub is_expired: bool,
    /// True when the grant is active but expiring soon.
    pub expiring_soon: bool,
    /// True when this entry can be inspected for its full grant detail.
    pub inspectable: bool,
}

/// The resolved approval-timeline-entry truth for one approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedApprovalTimelineEntry {
    /// The opaque approval id.
    pub approval_id: String,
    /// The canonical run id this approval influenced.
    pub run_id_label: String,
    /// The opaque granting-actor label.
    pub actor_label: String,
    /// The class of the granting actor.
    pub actor_class: M5AiApprovalActorClass,
    /// The scope the grant reaches.
    pub grant_scope: M5AiApprovalGrantScope,
    /// The opaque policy-epoch label the grant was made under.
    pub policy_epoch_label: String,
    /// The gate the grant satisfied.
    pub gate: M5AiApprovalGate,
    /// The derived expiry state.
    pub expiry_state: M5AiApprovalExpiryState,
    /// True when the grant is still in force.
    pub is_effective: bool,
    /// True when a rerun would require a fresh approval.
    pub requires_reapproval: bool,
    /// True when this entry can be inspected for its full grant detail.
    pub inspectable: bool,
}

/// Errors returned by [`resolve_approval_timeline_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiApprovalTimelineResolutionError {
    /// The approval id was empty.
    EmptyApprovalId,
    /// The run-id label was empty.
    EmptyRunId,
    /// The actor label was empty.
    EmptyActor,
    /// The policy-epoch label was empty.
    EmptyPolicyEpoch,
    /// The grant claims an expiry (has_expiry / expired / expiring-soon) but carries no
    /// expiry timestamp.
    ExpiryClaimedWithoutTimestamp,
    /// The approval is not inspectable, so its grant detail would collapse into a vague
    /// badge.
    ApprovalNotInspectable,
    /// An approval descriptor carried forbidden material.
    ForbiddenApprovalMaterial,
}

impl M5AiApprovalTimelineResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyApprovalId => "empty_approval_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyActor => "empty_actor",
            Self::EmptyPolicyEpoch => "empty_policy_epoch",
            Self::ExpiryClaimedWithoutTimestamp => "expiry_claimed_without_timestamp",
            Self::ApprovalNotInspectable => "approval_not_inspectable",
            Self::ForbiddenApprovalMaterial => "forbidden_approval_material",
        }
    }
}

impl fmt::Display for M5AiApprovalTimelineResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai approval timeline resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiApprovalTimelineResolutionError {}

/// Resolves one approval-timeline entry from an approval's declared state.
///
/// The actor, scope, policy epoch, and expiry state are preserved distinctly so approval
/// history can be inspected and never collapses multiple distinct grants into one vague
/// "approved" badge. The expiry state is derived with a fixed precedence — revoked, then
/// single-use-consumed, then expired, then expiring-soon, then active (with or without an
/// expiry) — and the grant is effective only while active, expiring-soon, or non-expiring.
/// An approval that influenced the run must be inspectable; a non-inspectable approval is
/// rejected as [`M5AiApprovalTimelineResolutionError::ApprovalNotInspectable`].
pub fn resolve_approval_timeline_entry(
    input: &M5AiApprovalTimelineResolutionInput,
) -> Result<M5ResolvedApprovalTimelineEntry, M5AiApprovalTimelineResolutionError> {
    if input.approval_id.trim().is_empty() {
        return Err(M5AiApprovalTimelineResolutionError::EmptyApprovalId);
    }
    if input.run_id_label.trim().is_empty() {
        return Err(M5AiApprovalTimelineResolutionError::EmptyRunId);
    }
    if input.actor_label.trim().is_empty() {
        return Err(M5AiApprovalTimelineResolutionError::EmptyActor);
    }
    if input.policy_epoch_label.trim().is_empty() {
        return Err(M5AiApprovalTimelineResolutionError::EmptyPolicyEpoch);
    }
    let claims_expiry = input.has_expiry || input.is_expired || input.expiring_soon;
    if claims_expiry && input.expiry_label.trim().is_empty() {
        return Err(M5AiApprovalTimelineResolutionError::ExpiryClaimedWithoutTimestamp);
    }
    if !input.inspectable {
        return Err(M5AiApprovalTimelineResolutionError::ApprovalNotInspectable);
    }
    for value in [
        &input.approval_id,
        &input.run_id_label,
        &input.actor_label,
        &input.policy_epoch_label,
        &input.expiry_label,
    ] {
        if value_repr_is_forbidden(value) {
            return Err(M5AiApprovalTimelineResolutionError::ForbiddenApprovalMaterial);
        }
    }

    let expiry_state = derive_expiry_state(input);
    let is_effective = expiry_state.is_effective();

    Ok(M5ResolvedApprovalTimelineEntry {
        approval_id: input.approval_id.clone(),
        run_id_label: input.run_id_label.clone(),
        actor_label: input.actor_label.clone(),
        actor_class: input.actor_class,
        grant_scope: input.grant_scope,
        policy_epoch_label: input.policy_epoch_label.clone(),
        gate: input.gate,
        expiry_state,
        is_effective,
        requires_reapproval: !is_effective,
        inspectable: input.inspectable,
    })
}

/// Derives the expiry state with a fixed precedence so a revoked or consumed grant never
/// reads as active.
fn derive_expiry_state(input: &M5AiApprovalTimelineResolutionInput) -> M5AiApprovalExpiryState {
    if input.is_revoked {
        M5AiApprovalExpiryState::Revoked
    } else if input.is_single_use && input.single_use_consumed {
        M5AiApprovalExpiryState::SingleUseConsumed
    } else if input.is_expired {
        M5AiApprovalExpiryState::Expired
    } else if input.expiring_soon {
        M5AiApprovalExpiryState::ExpiringSoon
    } else if input.has_expiry {
        M5AiApprovalExpiryState::Active
    } else {
        M5AiApprovalExpiryState::NoExpiry
    }
}

// ---- evidence / export summary resolver ---------------------------------

/// The full input to the evidence / export summary resolver for one evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceSummaryResolutionInput {
    /// The opaque evidence packet id.
    pub packet_id: String,
    /// The canonical run id this packet belongs to.
    pub run_id_label: String,
    /// The artifact classes the packet includes.
    pub artifact_classes: Vec<M5AiEvidenceArtifactClass>,
    /// The packet's redaction posture.
    pub redaction_posture: M5AiRedactionPosture,
    /// The packet's support linkage.
    pub support_linkage: M5AiSupportLinkage,
    /// The export formats the summary can produce.
    pub export_formats: Vec<M5AiExportFormat>,
    /// True when the summary offers structured redaction / support / artifact state rather
    /// than only a raw-file download link.
    pub offers_structured_summary: bool,
}

/// The resolved evidence / export summary truth for one evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEvidenceExportSummary {
    /// The opaque evidence packet id.
    pub packet_id: String,
    /// The canonical run id this packet belongs to.
    pub run_id_label: String,
    /// The artifact classes the packet includes.
    pub artifact_classes: Vec<M5AiEvidenceArtifactClass>,
    /// The packet's redaction posture.
    pub redaction_posture: M5AiRedactionPosture,
    /// The packet's support linkage.
    pub support_linkage: M5AiSupportLinkage,
    /// The export formats the summary can produce.
    pub export_formats: Vec<M5AiExportFormat>,
    /// True when the packet's redaction posture makes it safe to share.
    pub is_shareable: bool,
    /// True when the summary preserves redaction and support-continuity state rather than
    /// only a raw-file download link.
    pub preserves_redaction_and_support_continuity: bool,
    /// True when the packet carries an active support-continuity link.
    pub support_continuity_linked: bool,
}

/// Errors returned by [`resolve_evidence_export_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiEvidenceSummaryResolutionError {
    /// The packet id was empty.
    EmptyPacketId,
    /// The run-id label was empty.
    EmptyRunId,
    /// The packet declared no artifact classes.
    NoArtifactClasses,
    /// The summary declared no export formats.
    NoExportFormats,
    /// The summary offers only a raw-file download link with no redaction / support state.
    RawDownloadOnly,
    /// An evidence descriptor carried forbidden material.
    ForbiddenEvidenceMaterial,
}

impl M5AiEvidenceSummaryResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPacketId => "empty_packet_id",
            Self::EmptyRunId => "empty_run_id",
            Self::NoArtifactClasses => "no_artifact_classes",
            Self::NoExportFormats => "no_export_formats",
            Self::RawDownloadOnly => "raw_download_only",
            Self::ForbiddenEvidenceMaterial => "forbidden_evidence_material",
        }
    }
}

impl fmt::Display for M5AiEvidenceSummaryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai evidence summary resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiEvidenceSummaryResolutionError {}

/// Resolves one evidence / export summary from an evidence packet's declared state.
///
/// The summary always carries the packet id, the run it belongs to, its artifact classes,
/// its redaction posture, its support linkage, and its export formats — so it preserves
/// redaction and support-continuity state instead of collapsing to a raw-file download
/// link, which is rejected as [`M5AiEvidenceSummaryResolutionError::RawDownloadOnly`]. A
/// packet is safe to share only when its redaction posture has at least removed its
/// secrets; an unredacted, pending, or redaction-failed packet resolves to
/// `is_shareable = false`.
pub fn resolve_evidence_export_summary(
    input: &M5AiEvidenceSummaryResolutionInput,
) -> Result<M5ResolvedEvidenceExportSummary, M5AiEvidenceSummaryResolutionError> {
    if input.packet_id.trim().is_empty() {
        return Err(M5AiEvidenceSummaryResolutionError::EmptyPacketId);
    }
    if input.run_id_label.trim().is_empty() {
        return Err(M5AiEvidenceSummaryResolutionError::EmptyRunId);
    }
    if input.artifact_classes.is_empty() {
        return Err(M5AiEvidenceSummaryResolutionError::NoArtifactClasses);
    }
    if input.export_formats.is_empty() {
        return Err(M5AiEvidenceSummaryResolutionError::NoExportFormats);
    }
    if !input.offers_structured_summary {
        return Err(M5AiEvidenceSummaryResolutionError::RawDownloadOnly);
    }
    for value in [&input.packet_id, &input.run_id_label] {
        if value_repr_is_forbidden(value) {
            return Err(M5AiEvidenceSummaryResolutionError::ForbiddenEvidenceMaterial);
        }
    }

    let is_shareable = input.redaction_posture.is_share_safe();
    let support_continuity_linked = input.support_linkage.is_linked();

    Ok(M5ResolvedEvidenceExportSummary {
        packet_id: input.packet_id.clone(),
        run_id_label: input.run_id_label.clone(),
        artifact_classes: input.artifact_classes.clone(),
        redaction_posture: input.redaction_posture,
        support_linkage: input.support_linkage,
        export_formats: input.export_formats.clone(),
        is_shareable,
        preserves_redaction_and_support_continuity: input.offers_structured_summary,
        support_continuity_linked,
    })
}

// ---- worked cases -------------------------------------------------------

/// One worked run-history resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryResolutionCase {
    /// The resolver input.
    pub input: M5AiRunHistoryResolutionInput,
    /// The resolved truth. Must equal `resolve_run_history_row(&input)`.
    pub resolved: M5ResolvedRunHistoryRow,
}

impl M5AiRunHistoryResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiRunHistoryResolutionInput) -> Self {
        let resolved = resolve_run_history_row(&input).expect("seed run-history case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_run_history_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked approval-timeline resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiApprovalTimelineResolutionCase {
    /// The resolver input.
    pub input: M5AiApprovalTimelineResolutionInput,
    /// The resolved truth. Must equal `resolve_approval_timeline_entry(&input)`.
    pub resolved: M5ResolvedApprovalTimelineEntry,
}

impl M5AiApprovalTimelineResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiApprovalTimelineResolutionInput) -> Self {
        let resolved =
            resolve_approval_timeline_entry(&input).expect("seed approval-timeline case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_approval_timeline_entry(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked evidence / export summary resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceSummaryResolutionCase {
    /// The resolver input.
    pub input: M5AiEvidenceSummaryResolutionInput,
    /// The resolved truth. Must equal `resolve_evidence_export_summary(&input)`.
    pub resolved: M5ResolvedEvidenceExportSummary,
}

impl M5AiEvidenceSummaryResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiEvidenceSummaryResolutionInput) -> Self {
        let resolved =
            resolve_evidence_export_summary(&input).expect("seed evidence-summary case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_evidence_export_summary(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one claimed M5 replay surface bound to the shared
/// run-history, approval-timeline, and evidence-summary anatomy, vocabularies, entry
/// points, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryExportRow {
    /// Replay surface family.
    pub replay_surface: M5AiReplaySurface,
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
    /// Run-history anatomy parts this row renders (must include the mandatory parts).
    pub run_history_anatomy_parts: Vec<M5AiRunHistoryAnatomyPart>,
    /// Approval-timeline anatomy parts this entry renders (must include the mandatory
    /// parts).
    pub approval_timeline_anatomy_parts: Vec<M5AiApprovalTimelineAnatomyPart>,
    /// Evidence-summary anatomy parts this summary renders (must include the mandatory
    /// parts).
    pub evidence_summary_anatomy_parts: Vec<M5AiEvidenceSummaryAnatomyPart>,
    /// Stable entry points this surface offers.
    pub entry_points: Vec<M5AiRunHistoryEntryPoint>,
    /// Execution modes this surface distinguishes.
    pub execution_modes: Vec<M5AiExecutionMode>,
    /// Run outcomes this surface distinguishes.
    pub run_outcomes: Vec<M5AiRunOutcome>,
    /// Approval actor classes this surface names.
    pub approval_actor_classes: Vec<M5AiApprovalActorClass>,
    /// Approval grant scopes this surface distinguishes.
    pub approval_grant_scopes: Vec<M5AiApprovalGrantScope>,
    /// Approval expiry states this surface distinguishes.
    pub approval_expiry_states: Vec<M5AiApprovalExpiryState>,
    /// Approval gates this surface distinguishes.
    pub approval_gates: Vec<M5AiApprovalGate>,
    /// Evidence artifact classes this surface names.
    pub artifact_classes: Vec<M5AiEvidenceArtifactClass>,
    /// Redaction postures this surface distinguishes.
    pub redaction_postures: Vec<M5AiRedactionPosture>,
    /// Support linkages this surface distinguishes.
    pub support_linkages: Vec<M5AiSupportLinkage>,
    /// Export formats this surface produces.
    pub export_formats: Vec<M5AiExportFormat>,
    /// Run-history export fields this row carries (must include the mandatory fields).
    pub run_history_export_fields: Vec<M5AiRunHistoryExportField>,
    /// Approval-timeline export fields this entry carries (must include the mandatory
    /// fields).
    pub approval_timeline_export_fields: Vec<M5AiApprovalTimelineExportField>,
    /// Evidence-summary export fields this summary carries (must include the mandatory
    /// fields).
    pub evidence_summary_export_fields: Vec<M5AiEvidenceSummaryExportField>,
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
    /// Worked run-history resolutions proving the run-history resolver on this surface.
    pub run_history_examples: Vec<M5AiRunHistoryResolutionCase>,
    /// Worked approval-timeline resolutions proving the approval resolver on this surface.
    pub approval_timeline_examples: Vec<M5AiApprovalTimelineResolutionCase>,
    /// Worked evidence-summary resolutions proving the evidence resolver on this surface.
    pub evidence_summary_examples: Vec<M5AiEvidenceSummaryResolutionCase>,
    /// Hard invariant: this surface never masks the AI run identity across surfaces. MUST
    /// be `false`.
    pub masks_run_identity_across_surfaces: bool,
    /// Hard invariant: this surface never collapses multiple distinct grants into one
    /// vague "approved" badge. MUST be `false`.
    pub collapses_multiple_grants_into_one_badge: bool,
    /// Hard invariant: this surface never reduces an export summary to a raw-file download
    /// link. MUST be `false`.
    pub offers_raw_download_links_only: bool,
    /// Hard invariant: this surface never invents a parallel run-history, approval, or
    /// export grammar. MUST be `false`.
    pub invents_parallel_history_or_export_grammar: bool,
}

impl M5AiRunHistoryExportRow {
    /// True when the row declares every mandatory run-history anatomy part.
    fn declares_mandatory_run_history_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiRunHistoryAnatomyPart> =
            self.run_history_anatomy_parts.iter().copied().collect();
        M5AiRunHistoryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory approval-timeline anatomy part.
    fn declares_mandatory_approval_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiApprovalTimelineAnatomyPart> = self
            .approval_timeline_anatomy_parts
            .iter()
            .copied()
            .collect();
        M5AiApprovalTimelineAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory evidence-summary anatomy part.
    fn declares_mandatory_evidence_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiEvidenceSummaryAnatomyPart> =
            self.evidence_summary_anatomy_parts.iter().copied().collect();
        M5AiEvidenceSummaryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row keeps the mandatory open / replay / export entry points.
    fn declares_mandatory_entry_points(&self) -> bool {
        let present: BTreeSet<M5AiRunHistoryEntryPoint> =
            self.entry_points.iter().copied().collect();
        M5AiRunHistoryEntryPoint::MANDATORY
            .iter()
            .all(|point| present.contains(point))
    }

    /// True when the row declares every mandatory run-history export field.
    fn declares_mandatory_run_history_export(&self) -> bool {
        let present: BTreeSet<M5AiRunHistoryExportField> =
            self.run_history_export_fields.iter().copied().collect();
        M5AiRunHistoryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory approval-timeline export field.
    fn declares_mandatory_approval_export(&self) -> bool {
        let present: BTreeSet<M5AiApprovalTimelineExportField> = self
            .approval_timeline_export_fields
            .iter()
            .copied()
            .collect();
        M5AiApprovalTimelineExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory evidence-summary export field.
    fn declares_mandatory_evidence_export(&self) -> bool {
        let present: BTreeSet<M5AiEvidenceSummaryExportField> =
            self.evidence_summary_export_fields.iter().copied().collect();
        M5AiEvidenceSummaryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row proves at least two distinct grants (distinct actor class and
    /// scope) so approval history never collapses into one vague badge.
    fn proves_multiple_distinct_grants(&self) -> bool {
        let pairs: BTreeSet<(M5AiApprovalActorClass, M5AiApprovalGrantScope)> = self
            .approval_timeline_examples
            .iter()
            .map(|case| (case.resolved.actor_class, case.resolved.grant_scope))
            .collect();
        pairs.len() >= 2
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_run_identity_across_surfaces
            && !self.collapses_multiple_grants_into_one_badge
            && !self.offers_raw_download_links_only
            && !self.invents_parallel_history_or_export_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryExportVocabularySet {
    /// Replay-surface tokens.
    pub replay_surfaces: Vec<String>,
    /// Run-history-anatomy-part tokens.
    pub run_history_anatomy_parts: Vec<String>,
    /// Approval-timeline-anatomy-part tokens.
    pub approval_timeline_anatomy_parts: Vec<String>,
    /// Evidence-summary-anatomy-part tokens.
    pub evidence_summary_anatomy_parts: Vec<String>,
    /// Entry-point tokens.
    pub entry_points: Vec<String>,
    /// Approval-actor-class tokens.
    pub approval_actor_classes: Vec<String>,
    /// Approval-grant-scope tokens.
    pub approval_grant_scopes: Vec<String>,
    /// Approval-expiry-state tokens.
    pub approval_expiry_states: Vec<String>,
    /// Evidence-artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Redaction-posture tokens.
    pub redaction_postures: Vec<String>,
    /// Support-linkage tokens.
    pub support_linkages: Vec<String>,
    /// Export-format tokens.
    pub export_formats: Vec<String>,
    /// Run-history-export-field tokens.
    pub run_history_export_fields: Vec<String>,
    /// Approval-timeline-export-field tokens.
    pub approval_timeline_export_fields: Vec<String>,
    /// Evidence-summary-export-field tokens.
    pub evidence_summary_export_fields: Vec<String>,
    /// Execution-mode tokens (reused from the frozen matrix).
    pub execution_modes: Vec<String>,
    /// Run-outcome tokens (reused from the frozen matrix).
    pub run_outcomes: Vec<String>,
    /// Approval-gate tokens (reused from the frozen matrix).
    pub approval_gates: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AiRunHistoryExportVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            replay_surfaces: tokens(&M5AiReplaySurface::ALL, |v| v.as_str()),
            run_history_anatomy_parts: tokens(&M5AiRunHistoryAnatomyPart::ALL, |v| v.as_str()),
            approval_timeline_anatomy_parts: tokens(&M5AiApprovalTimelineAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            evidence_summary_anatomy_parts: tokens(&M5AiEvidenceSummaryAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            entry_points: tokens(&M5AiRunHistoryEntryPoint::ALL, |v| v.as_str()),
            approval_actor_classes: tokens(&M5AiApprovalActorClass::ALL, |v| v.as_str()),
            approval_grant_scopes: tokens(&M5AiApprovalGrantScope::ALL, |v| v.as_str()),
            approval_expiry_states: tokens(&M5AiApprovalExpiryState::ALL, |v| v.as_str()),
            artifact_classes: tokens(&M5AiEvidenceArtifactClass::ALL, |v| v.as_str()),
            redaction_postures: tokens(&M5AiRedactionPosture::ALL, |v| v.as_str()),
            support_linkages: tokens(&M5AiSupportLinkage::ALL, |v| v.as_str()),
            export_formats: tokens(&M5AiExportFormat::ALL, |v| v.as_str()),
            run_history_export_fields: tokens(&M5AiRunHistoryExportField::ALL, |v| v.as_str()),
            approval_timeline_export_fields: tokens(&M5AiApprovalTimelineExportField::ALL, |v| {
                v.as_str()
            }),
            evidence_summary_export_fields: tokens(&M5AiEvidenceSummaryExportField::ALL, |v| {
                v.as_str()
            }),
            execution_modes: tokens(&M5AiExecutionMode::ALL, |v| v.as_str()),
            run_outcomes: tokens(&M5AiRunOutcome::ALL, |v| v.as_str()),
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
pub struct M5AiRunHistoryExportGovernanceReview {
    /// One primitive trio carries run-history, approval, and evidence truth on every
    /// surface.
    pub one_primitive_carries_history_approval_and_evidence_truth: bool,
    /// The canonical run identity stays consistent across history, evidence, export,
    /// support, and replay.
    pub run_identity_consistent_across_surfaces: bool,
    /// The provider / model route is always named, never masked.
    pub provider_model_route_always_named: bool,
    /// Every run-history row offers stable open / replay / export entry points.
    pub stable_open_replay_export_entry_points: bool,
    /// Approval history preserves actor, scope, policy epoch, and expiry distinctly.
    pub approval_history_preserves_distinct_grants: bool,
    /// Approval history never collapses multiple grants into one vague badge.
    pub approval_history_never_collapsed_into_one_badge: bool,
    /// Export summaries preserve redaction and support-continuity state.
    pub export_summaries_preserve_redaction_and_support: bool,
    /// Export summaries are never reduced to raw-file download links.
    pub export_summaries_never_raw_download_only: bool,
    /// The support / export packet reconstructs row, entry, and summary truth.
    pub support_export_reconstructs_history_and_export_truth: bool,
    /// No surface invents a second history, approval, or export grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryExportConsumerProjection {
    /// History, evidence, export, support, and replay surfaces all consume the shared
    /// primitive trio.
    pub replay_surfaces_consume_shared_primitive: bool,
    /// The run-identity projection reads a single canonical source.
    pub run_identity_reads_single_source: bool,
    /// The approval-expiry projection reads a single canonical source.
    pub approval_expiry_reads_single_source: bool,
    /// The redaction / shareability projection reads a single canonical source.
    pub redaction_shareability_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryExportProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive trio.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryExportReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiRunHistoryExportPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiRunHistoryExportPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Replay-surface rows.
    pub rows: Vec<M5AiRunHistoryExportRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiRunHistoryExportVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiRunHistoryExportGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiRunHistoryExportConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiRunHistoryExportProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiRunHistoryExportReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 run-history / approval-timeline / evidence-export primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRunHistoryExportPrimitivePacket {
    /// Record kind; must equal [`M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Replay-surface rows.
    pub rows: Vec<M5AiRunHistoryExportRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiRunHistoryExportVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiRunHistoryExportGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiRunHistoryExportConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiRunHistoryExportProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiRunHistoryExportReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiRunHistoryExportPrimitivePacket {
    /// Builds an M5 run-history / approval-timeline / evidence-export primitive packet.
    pub fn new(input: M5AiRunHistoryExportPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_SCHEMA_VERSION,
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

    /// Validates the M5 run-history / approval-timeline / evidence-export invariants.
    pub fn validate(&self) -> Vec<M5AiRunHistoryExportPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_RECORD_KIND {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_RUN_HISTORY_EXPORT_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_run_identity_consistency(self, &mut violations);
        validate_distinct_grants(self, &mut violations);
        validate_expiry_honesty(self, &mut violations);
        validate_redaction_support_continuity(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai run-history/export primitive packet serializes"),
        ) {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 ai run-history/export primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per replay surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "replay_surface,qualification,owner,run_history_anatomy,approval_anatomy,evidence_anatomy,entry_points,artifact_classes,redaction_postures,support_linkages,export_formats,run_history_examples,approval_examples,evidence_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.replay_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.run_history_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.approval_timeline_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.evidence_summary_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.entry_points, |v| v.as_str()),
                join_tokens(&row.artifact_classes, |v| v.as_str()),
                join_tokens(&row.redaction_postures, |v| v.as_str()),
                join_tokens(&row.support_linkages, |v| v.as_str()),
                join_tokens(&row.export_formats, |v| v.as_str()),
                row.run_history_examples.len(),
                row.approval_timeline_examples.len(),
                row.evidence_summary_examples.len(),
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
            "# M5 AI Run-History Row, Approval-Timeline Entry, and Evidence-Export Summary Primitive\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Replay surfaces: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Entry points: {}\n",
            self.vocabulary_set.entry_points.join(", ")
        ));
        out.push_str(&format!(
            "- Approval expiry states: {}\n",
            self.vocabulary_set.approval_expiry_states.join(", ")
        ));
        out.push_str(&format!(
            "- Redaction postures: {}\n",
            self.vocabulary_set.redaction_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Export formats: {}\n",
            self.vocabulary_set.export_formats.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Replay surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.replay_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked run-history rows: {}\n",
                row.run_history_examples.len()
            ));
            for case in &row.run_history_examples {
                out.push_str(&format!(
                    "    - `{}` → route `{}` outcome `{}` (entry points {})\n",
                    case.resolved.canonical_run_id,
                    case.resolved.route_label,
                    case.resolved.run_outcome.as_str(),
                    case.resolved.entry_points.len(),
                ));
            }
            out.push_str(&format!(
                "  - Worked approval-timeline entries: {}\n",
                row.approval_timeline_examples.len()
            ));
            for case in &row.approval_timeline_examples {
                out.push_str(&format!(
                    "    - `{}` by `{}` over `{}` → expiry `{}` (effective `{}`)\n",
                    case.resolved.approval_id,
                    case.resolved.actor_class.as_str(),
                    case.resolved.grant_scope.as_str(),
                    case.resolved.expiry_state.as_str(),
                    case.resolved.is_effective,
                ));
            }
            out.push_str(&format!(
                "  - Worked evidence-export summaries: {}\n",
                row.evidence_summary_examples.len()
            ));
            for case in &row.evidence_summary_examples {
                out.push_str(&format!(
                    "    - `{}` → redaction `{}` linkage `{}` (shareable `{}`)\n",
                    case.resolved.packet_id,
                    case.resolved.redaction_posture.as_str(),
                    case.resolved.support_linkage.as_str(),
                    case.resolved.is_shareable,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 run-history/export-primitive export.
#[derive(Debug)]
pub enum M5AiRunHistoryExportPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiRunHistoryExportPrimitiveViolation>),
}

impl fmt::Display for M5AiRunHistoryExportPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai run-history/export primitive export parse failed: {error}"
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
                    "m5 ai run-history/export primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiRunHistoryExportPrimitiveArtifactError {}

/// Validation failures emitted by [`M5AiRunHistoryExportPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiRunHistoryExportPrimitiveViolation {
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
    /// A required replay surface is missing from the matrix.
    RequiredSurfaceMissing,
    /// A replay-surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory run-history anatomy parts.
    MandatoryRunHistoryAnatomyMissing,
    /// A row omits one of the mandatory approval-timeline anatomy parts.
    MandatoryApprovalAnatomyMissing,
    /// A row omits one of the mandatory evidence-summary anatomy parts.
    MandatoryEvidenceAnatomyMissing,
    /// A row omits one of the stable open / replay / export entry points.
    MandatoryEntryPointMissing,
    /// A row omits one of the mandatory run-history export fields.
    MandatoryRunHistoryExportMissing,
    /// A row omits one of the mandatory approval-timeline export fields.
    MandatoryApprovalExportMissing,
    /// A row omits one of the mandatory evidence-summary export fields.
    MandatoryEvidenceExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked run-history resolutions.
    RunHistoryExampleMissing,
    /// A row declares no worked approval-timeline resolutions.
    ApprovalExampleMissing,
    /// A row declares no worked evidence-summary resolutions.
    EvidenceExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No canonical run identity is shared across a run-history, an evidence, and an
    /// approval example.
    RunIdentityConsistencyUnproven,
    /// No row proves two distinct grants, so approval history could collapse into one
    /// badge.
    MultipleDistinctGrantsUnproven,
    /// No worked approval resolution proves an expired / revoked / consumed grant is shown
    /// as no longer effective.
    ExpiryHonestyUnproven,
    /// No worked evidence resolution proves a shareable, support-linked summary that
    /// preserves redaction and support-continuity state.
    RedactionSupportContinuityUnproven,
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

impl M5AiRunHistoryExportPrimitiveViolation {
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
            Self::MandatoryRunHistoryAnatomyMissing => "mandatory_run_history_anatomy_missing",
            Self::MandatoryApprovalAnatomyMissing => "mandatory_approval_anatomy_missing",
            Self::MandatoryEvidenceAnatomyMissing => "mandatory_evidence_anatomy_missing",
            Self::MandatoryEntryPointMissing => "mandatory_entry_point_missing",
            Self::MandatoryRunHistoryExportMissing => "mandatory_run_history_export_missing",
            Self::MandatoryApprovalExportMissing => "mandatory_approval_export_missing",
            Self::MandatoryEvidenceExportMissing => "mandatory_evidence_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RunHistoryExampleMissing => "run_history_example_missing",
            Self::ApprovalExampleMissing => "approval_example_missing",
            Self::EvidenceExampleMissing => "evidence_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::RunIdentityConsistencyUnproven => "run_identity_consistency_unproven",
            Self::MultipleDistinctGrantsUnproven => "multiple_distinct_grants_unproven",
            Self::ExpiryHonestyUnproven => "expiry_honesty_unproven",
            Self::RedactionSupportContinuityUnproven => "redaction_support_continuity_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 run-history/export-primitive export.
pub fn current_stable_m5_ai_run_history_export_primitive_export(
) -> Result<M5AiRunHistoryExportPrimitivePacket, M5AiRunHistoryExportPrimitiveArtifactError> {
    let packet: M5AiRunHistoryExportPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/support_export.json"
    )))
    .map_err(M5AiRunHistoryExportPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiRunHistoryExportPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_RUN_HISTORY_EXPORT_SCHEMA_REF,
        M5_AI_RUN_HISTORY_EXPORT_DOC_REF,
        M5_AI_RUN_HISTORY_EXPORT_COMPONENT_MATRIX_REF,
        M5_AI_RUN_HISTORY_EXPORT_RUN_HISTORY_REF,
        M5_AI_RUN_HISTORY_EXPORT_APPROVAL_REF,
        M5_AI_RUN_HISTORY_EXPORT_EVIDENCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let present: BTreeSet<M5AiReplaySurface> =
        packet.rows.iter().map(|row| row.replay_surface).collect();
    for required in M5AiReplaySurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.run_history_anatomy_parts.is_empty()
            || row.approval_timeline_anatomy_parts.is_empty()
            || row.evidence_summary_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.entry_points.is_empty()
            || row.execution_modes.is_empty()
            || row.run_outcomes.is_empty()
            || row.approval_actor_classes.is_empty()
            || row.approval_grant_scopes.is_empty()
            || row.approval_expiry_states.is_empty()
            || row.approval_gates.is_empty()
            || row.artifact_classes.is_empty()
            || row.redaction_postures.is_empty()
            || row.support_linkages.is_empty()
            || row.export_formats.is_empty()
        {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::RowIncomplete);
        }
        if !row.declares_mandatory_run_history_anatomy() {
            violations
                .push(M5AiRunHistoryExportPrimitiveViolation::MandatoryRunHistoryAnatomyMissing);
        }
        if !row.declares_mandatory_approval_anatomy() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MandatoryApprovalAnatomyMissing);
        }
        if !row.declares_mandatory_evidence_anatomy() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MandatoryEvidenceAnatomyMissing);
        }
        if !row.declares_mandatory_entry_points() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MandatoryEntryPointMissing);
        }
        if !row.declares_mandatory_run_history_export() {
            violations
                .push(M5AiRunHistoryExportPrimitiveViolation::MandatoryRunHistoryExportMissing);
        }
        if !row.declares_mandatory_approval_export() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MandatoryApprovalExportMissing);
        }
        if !row.declares_mandatory_evidence_export() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::MandatoryEvidenceExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AiAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.run_history_examples.is_empty() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::RunHistoryExampleMissing);
        }
        if row.approval_timeline_examples.is_empty() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::ApprovalExampleMissing);
        }
        if row.evidence_summary_examples.is_empty() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::EvidenceExampleMissing);
        }
        if row
            .run_history_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .approval_timeline_examples
                .iter()
                .any(|case| !case.is_self_consistent())
            || row
                .evidence_summary_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::RowInvariantViolated);
        }
    }
}

/// The same canonical AI run identity must appear in a run-history example, an evidence
/// example, and an approval example — the acceptance-criterion that one run identity stays
/// consistent across history, evidence, export, support, and replay surfaces.
fn validate_run_identity_consistency(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let run_ids: BTreeSet<&str> = packet
        .rows
        .iter()
        .flat_map(|row| row.run_history_examples.iter())
        .map(|case| case.resolved.canonical_run_id.as_str())
        .collect();
    let evidence_ids: BTreeSet<&str> = packet
        .rows
        .iter()
        .flat_map(|row| row.evidence_summary_examples.iter())
        .map(|case| case.resolved.run_id_label.as_str())
        .collect();
    let approval_ids: BTreeSet<&str> = packet
        .rows
        .iter()
        .flat_map(|row| row.approval_timeline_examples.iter())
        .map(|case| case.resolved.run_id_label.as_str())
        .collect();
    let shared = run_ids
        .iter()
        .any(|id| evidence_ids.contains(id) && approval_ids.contains(id));
    if !shared {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::RunIdentityConsistencyUnproven);
    }
}

/// At least one row must prove two distinct grants (distinct actor class and scope) — the
/// acceptance-criterion that approval history never collapses multiple grants into one
/// vague badge.
fn validate_distinct_grants(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    if !packet
        .rows
        .iter()
        .any(|row| row.proves_multiple_distinct_grants())
    {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::MultipleDistinctGrantsUnproven);
    }
}

/// At least one worked approval resolution must show an expired, revoked, or consumed grant
/// as no longer effective — the acceptance-criterion that approval history stays honest
/// about which grants still apply.
fn validate_expiry_honesty(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.approval_timeline_examples.iter().any(|case| {
            !case.resolved.is_effective
                && matches!(
                    case.resolved.expiry_state,
                    M5AiApprovalExpiryState::Expired
                        | M5AiApprovalExpiryState::Revoked
                        | M5AiApprovalExpiryState::SingleUseConsumed
                )
        })
    });
    if !proven {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::ExpiryHonestyUnproven);
    }
}

/// At least one worked evidence resolution must prove a shareable, support-linked summary
/// that preserves redaction and support-continuity state — the acceptance-criterion that
/// export summaries preserve redaction and support continuity rather than only raw file
/// download links.
fn validate_redaction_support_continuity(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.evidence_summary_examples.iter().any(|case| {
            case.resolved.is_shareable
                && case.resolved.support_continuity_linked
                && case.resolved.preserves_redaction_and_support_continuity
        })
    });
    if !proven {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::RedactionSupportContinuityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_history_approval_and_evidence_truth,
        review.run_identity_consistent_across_surfaces,
        review.provider_model_route_always_named,
        review.stable_open_replay_export_entry_points,
        review.approval_history_preserves_distinct_grants,
        review.approval_history_never_collapsed_into_one_badge,
        review.export_summaries_preserve_redaction_and_support,
        review.export_summaries_never_raw_download_only,
        review.support_export_reconstructs_history_and_export_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.replay_surfaces_consume_shared_primitive,
        projection.run_identity_reads_single_source,
        projection.approval_expiry_reads_single_source,
        projection.redaction_shareability_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5AiRunHistoryExportPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiRunHistoryExportPrimitivePacket,
    violations: &mut Vec<M5AiRunHistoryExportPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiRunHistoryExportPrimitiveViolation::ReleasePostureIncomplete);
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
