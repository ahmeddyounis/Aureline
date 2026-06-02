//! Stabilized review-side AI evidence attachment and safe suggestion apply
//! without broadening authority.
//!
//! This module owns the bounded beta contract that keeps AI-generated review
//! evidence and suggestion application previewable, attributable, and reversible.
//! Every claimed AI evidence attachment is modeled with explicit source
//! (`ai_model_local`, `ai_model_provider`, `ai_model_hybrid`,
//! `human_curated_ai`), freshness, acting identity, and target row. Safe
//! suggestion apply never broadens the user's authority: suggestions remain
//! advisory until explicitly previewed, checkpointed, and applied with a
//! recoverable undo path.
//!
//! The record family includes:
//!
//! - [`AiReviewEvidenceRecord`] — stable identity binding workspace, AI evidence
//!   attachments, and safe suggestion apply rows.
//! - [`AiEvidenceAttachmentRecord`] — one AI evidence item bound to a review
//!   row with explicit source, freshness, actor, and return path.
//! - [`SafeSuggestionApplyRecord`] — safe suggestion apply operation with
//!   explicit authority class, preview state, and reversibility.
//! - [`SuggestionApplyCheckpointRecord`] — checkpoint before/after apply
//!   preserving exact base/head identity and worktree state.
//! - [`AiEvidenceCommandRecord`] — command-graph operations surfaced to the
//!   inspector.
//! - [`AiEvidenceSupportExportPacket`] — redaction-safe support export that
//!   preserves evidence lineage and suggestion authority.
//! - [`AiEvidenceInspectionRecord`] — compact boolean projection for CLI and
//!   inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/ai_review_evidence.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/stabilize-review-side-ai-evidence-attachment-and-safe/`.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every AI review evidence record.
pub const AI_REVIEW_EVIDENCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`AiReviewEvidencePacket`].
pub const AI_REVIEW_EVIDENCE_PACKET_RECORD_KIND: &str = "ai_review_evidence_packet";

/// Stable record-kind tag for [`AiReviewEvidenceRecord`].
pub const AI_REVIEW_EVIDENCE_RECORD_KIND: &str = "ai_review_evidence_record";

/// Stable record-kind tag for [`AiEvidenceAttachmentRecord`].
pub const AI_EVIDENCE_ATTACHMENT_RECORD_KIND: &str = "ai_evidence_attachment_record";

/// Stable record-kind tag for [`SafeSuggestionApplyRecord`].
pub const SAFE_SUGGESTION_APPLY_RECORD_KIND: &str = "safe_suggestion_apply_record";

/// Stable record-kind tag for [`SuggestionApplyCheckpointRecord`].
pub const SUGGESTION_APPLY_CHECKPOINT_RECORD_KIND: &str = "suggestion_apply_checkpoint_record";

/// Stable record-kind tag for [`AiEvidenceCommandRecord`].
pub const AI_EVIDENCE_COMMAND_RECORD_KIND: &str = "ai_evidence_command_record";

/// Stable record-kind tag for [`AiEvidenceSupportExportPacket`].
pub const AI_EVIDENCE_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str = "ai_evidence_support_export_packet";

/// Stable record-kind tag for [`AiEvidenceInspectionRecord`].
pub const AI_EVIDENCE_INSPECTION_RECORD_KIND: &str = "ai_evidence_inspection_record";

/// Closed set of AI evidence states.
pub const AI_EVIDENCE_STATES: &[&str] = &[
    "attached_current",
    "attached_stale",
    "detached_invalidated",
    "pending_verification",
];

/// Closed set of AI evidence source classes.
pub const AI_EVIDENCE_SOURCE_CLASSES: &[&str] = &[
    "ai_model_local",
    "ai_model_provider",
    "ai_model_hybrid",
    "human_curated_ai",
];

/// Closed set of suggestion apply states.
pub const SUGGESTION_APPLY_STATES: &[&str] = &[
    "preview_ready",
    "applied_with_checkpoint",
    "reverted",
    "blocked_pending_review",
    "blocked_scope_exceeded",
];

/// Closed set of suggestion authority classes.
pub const SUGGESTION_AUTHORITY_CLASSES: &[&str] = &[
    "advisory_only",
    "previewable_local_apply",
    "checkpointed_reversible",
    "blocked_requires_human_approval",
];

/// Closed set of command classes for the AI evidence lane.
pub const AI_EVIDENCE_COMMAND_CLASSES: &[&str] = &[
    "preview_suggestion",
    "apply_suggestion",
    "revert_suggestion",
    "detach_evidence",
    "refresh_evidence",
    "export_evidence",
];

/// Closed set of consumer surfaces for AI evidence packets.
pub const AI_EVIDENCE_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
];

/// Closed set of invalidation reasons that mark AI evidence stale.
pub const AI_EVIDENCE_INVALIDATION_REASONS: &[&str] = &[
    "evidence_stale",
    "source_model_changed",
    "actor_scope_changed",
    "target_row_changed",
    "suggestion_conflicts_with_local",
    "authority_exceeded",
    "human_review_required",
];

/// Closed set of checkpoint states.
pub const CHECKPOINT_STATES: &[&str] = &[
    "checkpoint_created",
    "checkpoint_applied",
    "checkpoint_reverted",
    "checkpoint_failed",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing an AI review evidence packet to materialize on top of a
/// beta review-workspace packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewEvidenceInput {
    /// Stable evidence packet identity.
    pub evidence_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// AI evidence state from the closed vocabulary.
    pub evidence_state: String,
    /// AI evidence attachment inputs.
    pub evidence_attachments: Vec<AiEvidenceAttachmentInput>,
    /// Safe suggestion apply inputs.
    pub suggestion_applies: Vec<SafeSuggestionApplyInput>,
    /// Suggestion apply checkpoint inputs.
    pub checkpoints: Vec<SuggestionApplyCheckpointInput>,
    /// Command-graph operations.
    pub commands: Vec<AiEvidenceCommandInput>,
    /// Support/export envelope input.
    pub support_export: AiEvidenceSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing one AI evidence attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceAttachmentInput {
    /// Stable attachment identity.
    pub attachment_id: String,
    /// Opaque ref to the review row this evidence is attached to.
    pub review_row_ref: String,
    /// AI evidence source class from the closed vocabulary.
    pub evidence_source_class: String,
    /// Opaque model run ref.
    pub model_run_ref: String,
    /// Opaque actor ref who triggered the AI generation.
    pub actor_ref: String,
    /// Timestamp when the evidence was generated.
    pub generated_at: String,
    /// Freshness class at attachment time.
    pub freshness_class: String,
    /// True when the evidence has been verified by a human reviewer.
    pub human_verified: bool,
    /// True when the evidence may be replayed without live AI service.
    pub replayable_offline: bool,
    /// Opaque return path ref for reversible detachment.
    pub return_path_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one safe suggestion apply operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeSuggestionApplyInput {
    /// Stable suggestion apply identity.
    pub suggestion_apply_id: String,
    /// Opaque ref to the review row this suggestion targets.
    pub review_row_ref: String,
    /// Opaque ref to the attached evidence that produced this suggestion.
    pub source_evidence_ref: String,
    /// Suggestion apply state from the closed vocabulary.
    pub apply_state: String,
    /// Suggestion authority class from the closed vocabulary.
    pub suggestion_authority_class: String,
    /// True when the suggestion supports preview before apply.
    pub preview_supported: bool,
    /// True when the suggestion creates a checkpoint before apply.
    pub checkpoint_before_apply: bool,
    /// True when the suggestion is reversible after apply.
    pub reversible_after_apply: bool,
    /// True when applying the suggestion would broaden the user's authority.
    pub would_broaden_authority: bool,
    /// Opaque checkpoint ref when a checkpoint exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one suggestion apply checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionApplyCheckpointInput {
    /// Stable checkpoint identity.
    pub checkpoint_id: String,
    /// Opaque ref to the suggestion apply row this checkpoint belongs to.
    pub suggestion_apply_ref: String,
    /// Checkpoint state from the closed vocabulary.
    pub checkpoint_state: String,
    /// Base revision ref at checkpoint time.
    pub base_revision_ref: String,
    /// Head revision ref at checkpoint time.
    pub head_revision_ref: String,
    /// Worktree state hash at checkpoint time.
    pub worktree_state_hash: String,
    /// Timestamp when the checkpoint was created.
    pub created_at: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input describing one command-graph operation for AI evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceCommandInput {
    /// Stable command identity.
    pub command_id: String,
    /// Command class from the closed vocabulary.
    pub command_class: String,
    /// Target object ref the command would mutate.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when the command supports preview/dry-run.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Active blocked reasons preventing execution; empty when actionable.
    #[serde(default)]
    pub blocked_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row for the AI evidence support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the evidence packet.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// AI review evidence record materialized from input plus workspace truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewEvidenceRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable evidence packet identity.
    pub evidence_id: String,
    /// Review workspace this evidence belongs to.
    pub review_workspace_id_ref: String,
    /// AI evidence state.
    pub evidence_state: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing mutation.
    pub blocked_reasons: Vec<String>,
    /// True when the evidence packet is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the evidence packet was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// AI evidence attachment record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceAttachmentRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Evidence packet this attachment belongs to.
    pub evidence_id_ref: String,
    /// Stable attachment identity.
    pub attachment_id: String,
    /// Opaque ref to the review row this evidence is attached to.
    pub review_row_ref: String,
    /// AI evidence source class.
    pub evidence_source_class: String,
    /// Opaque model run ref.
    pub model_run_ref: String,
    /// Opaque actor ref who triggered the AI generation.
    pub actor_ref: String,
    /// Timestamp when the evidence was generated.
    pub generated_at: String,
    /// Freshness class at attachment time.
    pub freshness_class: String,
    /// True when the evidence has been verified by a human reviewer.
    pub human_verified: bool,
    /// True when the evidence may be replayed without live AI service.
    pub replayable_offline: bool,
    /// Opaque return path ref for reversible detachment.
    pub return_path_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Safe suggestion apply record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeSuggestionApplyRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Evidence packet this suggestion belongs to.
    pub evidence_id_ref: String,
    /// Stable suggestion apply identity.
    pub suggestion_apply_id: String,
    /// Opaque ref to the review row this suggestion targets.
    pub review_row_ref: String,
    /// Opaque ref to the attached evidence that produced this suggestion.
    pub source_evidence_ref: String,
    /// Suggestion apply state.
    pub apply_state: String,
    /// Suggestion authority class.
    pub suggestion_authority_class: String,
    /// True when the suggestion supports preview before apply.
    pub preview_supported: bool,
    /// True when the suggestion creates a checkpoint before apply.
    pub checkpoint_before_apply: bool,
    /// True when the suggestion is reversible after apply.
    pub reversible_after_apply: bool,
    /// True when applying the suggestion would broaden the user's authority.
    pub would_broaden_authority: bool,
    /// Optional checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Suggestion apply checkpoint record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionApplyCheckpointRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Evidence packet this checkpoint belongs to.
    pub evidence_id_ref: String,
    /// Stable checkpoint identity.
    pub checkpoint_id: String,
    /// Opaque ref to the suggestion apply row this checkpoint belongs to.
    pub suggestion_apply_ref: String,
    /// Checkpoint state.
    pub checkpoint_state: String,
    /// Base revision ref at checkpoint time.
    pub base_revision_ref: String,
    /// Head revision ref at checkpoint time.
    pub head_revision_ref: String,
    /// Worktree state hash at checkpoint time.
    pub worktree_state_hash: String,
    /// Timestamp when the checkpoint was created.
    pub created_at: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Command-graph operation record for AI evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Evidence packet this command belongs to.
    pub evidence_id_ref: String,
    /// Command class.
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// True when preview/dry-run is supported.
    pub preview_supported: bool,
    /// True when the command emits an audit event when executed.
    pub emits_audit_event: bool,
    /// Active blocked reasons preventing execution.
    pub blocked_reasons: Vec<String>,
    /// True when the command is actionable from the current evidence state.
    pub actionable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the AI evidence lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Evidence packet this packet exports.
    pub evidence_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stable context ref used to reopen the evidence packet.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Command ids exported in this packet.
    pub command_id_refs: Vec<String>,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the export cites.
    pub source_schema_refs: Vec<String>,
    /// False so raw URLs cannot cross the support boundary.
    pub raw_url_export_allowed: bool,
    /// False so raw provider payloads cannot cross the support boundary.
    pub raw_provider_payload_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Evidence packet inspected by this row.
    pub evidence_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the evidence state is attached_current.
    pub evidence_state_attached_current: bool,
    /// True when the evidence state is attached_stale.
    pub evidence_state_attached_stale: bool,
    /// True when the evidence state is detached_invalidated.
    pub evidence_state_detached_invalidated: bool,
    /// True when the evidence state is pending_verification.
    pub evidence_state_pending_verification: bool,
    /// True when at least one attachment is from an AI model.
    pub ai_model_source_present: bool,
    /// True when at least one attachment is human curated.
    pub human_curated_source_present: bool,
    /// True when at least one suggestion is preview ready.
    pub suggestion_preview_ready: bool,
    /// True when at least one suggestion is applied with checkpoint.
    pub suggestion_applied_with_checkpoint: bool,
    /// True when at least one suggestion is reverted.
    pub suggestion_reverted: bool,
    /// True when at least one suggestion is blocked pending review.
    pub suggestion_blocked_pending_review: bool,
    /// True when at least one suggestion is blocked due to scope exceeded.
    pub suggestion_blocked_scope_exceeded: bool,
    /// True when at least one suggestion would broaden authority.
    pub suggestion_would_broaden_authority: bool,
    /// True when every suggestion that is applied has a checkpoint.
    pub all_applied_have_checkpoints: bool,
    /// True when all checkpoints are in a recoverable state.
    pub all_checkpoints_recoverable: bool,
    /// True when the evidence packet is actionable.
    pub actionable: bool,
    /// True when the evidence packet is invalidated by any reason.
    pub invalidated: bool,
    /// Number of evidence attachment records.
    pub evidence_attachment_count: usize,
    /// Number of safe suggestion apply records.
    pub suggestion_apply_count: usize,
    /// Number of checkpoint records.
    pub checkpoint_count: usize,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the evidence context.
    pub support_export_reopenable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// AI review evidence packet consumed by review surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewEvidencePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the packet.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Review workspace summary copied from the beta packet.
    pub review_workspace: crate::workspace::ReviewWorkspaceRecord,
    /// AI review evidence record.
    pub evidence: AiReviewEvidenceRecord,
    /// AI evidence attachment records.
    pub evidence_attachments: Vec<AiEvidenceAttachmentRecord>,
    /// Safe suggestion apply records.
    pub suggestion_applies: Vec<SafeSuggestionApplyRecord>,
    /// Suggestion apply checkpoint records.
    pub checkpoints: Vec<SuggestionApplyCheckpointRecord>,
    /// Command-graph operation records.
    pub commands: Vec<AiEvidenceCommandRecord>,
    /// Support/export packet.
    pub support_export: AiEvidenceSupportExportPacket,
    /// Inspection row.
    pub inspection: AiEvidenceInspectionRecord,
}

impl AiReviewEvidencePacket {
    /// Builds an AI review evidence packet from a beta review-workspace packet
    /// and evidence input.
    ///
    /// # Errors
    ///
    /// Returns [`AiReviewEvidenceValidationError`] when the input violates an
    /// AI evidence invariant.
    pub fn from_workspace_packet(
        input: AiReviewEvidenceInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, AiReviewEvidenceValidationError> {
        validate_input(&input, workspace_packet)?;

        let evidence = evidence_record(&input, workspace_packet);
        let evidence_attachments = input
            .evidence_attachments
            .iter()
            .map(|a| attachment_record(a, &evidence))
            .collect::<Vec<_>>();
        let suggestion_applies = input
            .suggestion_applies
            .iter()
            .map(|s| suggestion_apply_record(s, &evidence))
            .collect::<Vec<_>>();
        let checkpoints = input
            .checkpoints
            .iter()
            .map(|c| checkpoint_record(c, &evidence))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|c| command_record(c, &evidence))
            .collect::<Vec<_>>();
        let support_export = support_export_packet(
            &input.support_export,
            &evidence,
            workspace_packet,
            &commands,
        );
        let inspection = inspection_record(
            &evidence,
            &evidence_attachments,
            &suggestion_applies,
            &checkpoints,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: AI_REVIEW_EVIDENCE_PACKET_RECORD_KIND.to_string(),
            schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            evidence,
            evidence_attachments,
            suggestion_applies,
            checkpoints,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the AI review evidence invariants.
    ///
    /// # Errors
    ///
    /// Returns [`AiReviewEvidenceValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), AiReviewEvidenceValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            AI_REVIEW_EVIDENCE_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_evidence_record(&self.evidence, &self.review_workspace.review_workspace_id)?;
        for attachment in &self.evidence_attachments {
            validate_attachment_record(attachment, &self.evidence.evidence_id)?;
        }
        for suggestion in &self.suggestion_applies {
            validate_suggestion_apply_record(suggestion, &self.evidence.evidence_id)?;
        }
        for checkpoint in &self.checkpoints {
            validate_checkpoint_record(checkpoint, &self.evidence.evidence_id)?;
        }
        for command in &self.commands {
            validate_command_record(command, &self.evidence.evidence_id)?;
        }
        validate_support_export(&self.support_export, &self.evidence, &self.commands)?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        let attachment_ids: std::collections::BTreeSet<&str> = self
            .evidence_attachments
            .iter()
            .map(|a| a.attachment_id.as_str())
            .collect();
        let checkpoint_ids: std::collections::BTreeSet<&str> = self
            .checkpoints
            .iter()
            .map(|c| c.checkpoint_id.as_str())
            .collect();

        for suggestion in &self.suggestion_applies {
            if let Some(ref checkpoint_ref) = suggestion.checkpoint_ref {
                if !checkpoint_ids.contains(checkpoint_ref.as_str()) {
                    return Err(ai_evidence_validation_error(format!(
                        "suggestion_apply {checkpoint_ref} cites unknown checkpoint_ref"
                    )));
                }
            }
            if !attachment_ids.contains(suggestion.source_evidence_ref.as_str()) {
                return Err(ai_evidence_validation_error(format!(
                    "suggestion_apply {} cites unknown source_evidence_ref",
                    suggestion.suggestion_apply_id
                )));
            }
        }

        // Authority invariant: no suggestion may claim applied if it would broaden authority
        for suggestion in &self.suggestion_applies {
            if suggestion.would_broaden_authority
                && suggestion.apply_state == "applied_with_checkpoint"
            {
                return Err(ai_evidence_validation_error(format!(
                    "suggestion_apply {} would_broaden_authority but is applied",
                    suggestion.suggestion_apply_id
                )));
            }
        }

        // Checkpoint coherence: applied suggestions must have checkpoints
        for suggestion in &self.suggestion_applies {
            if suggestion.apply_state == "applied_with_checkpoint"
                && suggestion.checkpoint_ref.is_none()
            {
                return Err(ai_evidence_validation_error(format!(
                    "suggestion_apply {} is applied but has no checkpoint_ref",
                    suggestion.suggestion_apply_id
                )));
            }
        }

        Ok(())
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
    }

    /// Returns true when every AI evidence source is from the closed vocabulary.
    pub fn evidence_sources_disclosed(&self) -> bool {
        self.evidence_attachments
            .iter()
            .all(|a| contains_token(AI_EVIDENCE_SOURCE_CLASSES, &a.evidence_source_class))
    }

    /// Returns true when no suggestion would broaden authority.
    pub fn no_authority_broadening(&self) -> bool {
        !self
            .suggestion_applies
            .iter()
            .any(|s| s.would_broaden_authority)
    }

    /// Returns true when every applied suggestion has a checkpoint.
    pub fn applied_suggestions_have_checkpoints(&self) -> bool {
        self.suggestion_applies.iter().all(|s| {
            if s.apply_state == "applied_with_checkpoint" {
                s.checkpoint_ref.is_some()
            } else {
                true
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Projection type
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiReviewEvidenceProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Evidence packet identity.
    pub evidence_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// AI evidence state.
    pub evidence_state: String,
    /// True when the packet is actionable.
    pub actionable: bool,
    /// True when at least one suggestion is preview ready.
    pub suggestion_preview_ready: bool,
    /// True when at least one suggestion is applied with checkpoint.
    pub suggestion_applied_with_checkpoint: bool,
    /// True when at least one suggestion is blocked pending review.
    pub suggestion_blocked_pending_review: bool,
    /// True when at least one suggestion would broaden authority.
    pub suggestion_would_broaden_authority: bool,
    /// True when all applied suggestions have checkpoints.
    pub all_applied_have_checkpoints: bool,
    /// Number of evidence attachments.
    pub evidence_attachment_count: usize,
    /// Number of suggestion applies.
    pub suggestion_apply_count: usize,
    /// Number of checkpoints.
    pub checkpoint_count: usize,
    /// Command count.
    pub command_count: usize,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for AI review evidence operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiReviewEvidenceError {
    /// Validation failed.
    Validation(AiReviewEvidenceValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for AiReviewEvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for AiReviewEvidenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for AI review evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiReviewEvidenceValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for AiReviewEvidenceValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AiReviewEvidenceValidationError {}

impl AiReviewEvidenceValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Parses and validates a materialized AI review evidence packet.
///
/// # Errors
///
/// Returns [`AiReviewEvidenceError`] when the payload fails to parse or
/// violates the AI review evidence invariants.
pub fn project_ai_review_evidence_packet(
    payload: &str,
) -> Result<AiReviewEvidenceProjection, AiReviewEvidenceError> {
    let packet: AiReviewEvidencePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(AiReviewEvidenceProjection::from(packet))
}

impl From<AiReviewEvidencePacket> for AiReviewEvidenceProjection {
    fn from(packet: AiReviewEvidencePacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            evidence_id: packet.evidence.evidence_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            evidence_state: packet.evidence.evidence_state,
            actionable: packet.evidence.actionable,
            suggestion_preview_ready: packet.inspection.suggestion_preview_ready,
            suggestion_applied_with_checkpoint: packet
                .inspection
                .suggestion_applied_with_checkpoint,
            suggestion_blocked_pending_review: packet.inspection.suggestion_blocked_pending_review,
            suggestion_would_broaden_authority: packet
                .inspection
                .suggestion_would_broaden_authority,
            all_applied_have_checkpoints: packet.inspection.all_applied_have_checkpoints,
            evidence_attachment_count: packet.evidence_attachments.len(),
            suggestion_apply_count: packet.suggestion_applies.len(),
            checkpoint_count: packet.checkpoints.len(),
            command_count: packet.commands.len(),
            invalidation_reasons: packet.evidence.invalidation_reasons.clone(),
            blocked_reasons: packet.evidence.blocked_reasons.clone(),
        }
    }
}

impl From<serde_json::Error> for AiReviewEvidenceError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(AiReviewEvidenceValidationError {
            message: err.to_string(),
        })
    }
}

impl From<AiReviewEvidenceValidationError> for AiReviewEvidenceError {
    fn from(err: AiReviewEvidenceValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn evidence_record(
    input: &AiReviewEvidenceInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> AiReviewEvidenceRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    for attachment in &input.evidence_attachments {
        if attachment.freshness_class == "stale" {
            if !invalidation_reasons.contains(&"evidence_stale".to_string()) {
                invalidation_reasons.push("evidence_stale".to_string());
            }
        }
    }
    for suggestion in &input.suggestion_applies {
        if suggestion.would_broaden_authority {
            if !invalidation_reasons.contains(&"authority_exceeded".to_string()) {
                invalidation_reasons.push("authority_exceeded".to_string());
            }
        }
        if suggestion.apply_state == "blocked_pending_review" {
            if !invalidation_reasons.contains(&"human_review_required".to_string()) {
                invalidation_reasons.push("human_review_required".to_string());
            }
        }
    }
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let mut blocked_reasons = Vec::new();
    if input.evidence_state == "detached_invalidated" {
        blocked_reasons.push("evidence_detached".to_string());
    }
    if input.evidence_state == "pending_verification" {
        blocked_reasons.push("pending_human_verification".to_string());
    }
    for suggestion in &input.suggestion_applies {
        if suggestion.would_broaden_authority {
            blocked_reasons.push("authority_exceeded".to_string());
        }
        if suggestion.apply_state == "blocked_scope_exceeded" {
            blocked_reasons.push("scope_exceeded".to_string());
        }
    }
    for reason in &invalidation_reasons {
        if reason == "source_model_changed" {
            blocked_reasons.push("source_model_changed".to_string());
        }
        if reason == "suggestion_conflicts_with_local" {
            blocked_reasons.push("suggestion_conflicts_with_local".to_string());
        }
    }
    blocked_reasons.sort();
    blocked_reasons.dedup();

    AiReviewEvidenceRecord {
        record_kind: AI_REVIEW_EVIDENCE_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        evidence_id: input.evidence_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        evidence_state: input.evidence_state.clone(),
        invalidation_reasons,
        blocked_reasons,
        actionable: input.commands.iter().any(|c| c.blocked_reasons.is_empty()),
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn attachment_record(
    input: &AiEvidenceAttachmentInput,
    evidence: &AiReviewEvidenceRecord,
) -> AiEvidenceAttachmentRecord {
    AiEvidenceAttachmentRecord {
        record_kind: AI_EVIDENCE_ATTACHMENT_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        evidence_id_ref: evidence.evidence_id.clone(),
        attachment_id: input.attachment_id.clone(),
        review_row_ref: input.review_row_ref.clone(),
        evidence_source_class: input.evidence_source_class.clone(),
        model_run_ref: input.model_run_ref.clone(),
        actor_ref: input.actor_ref.clone(),
        generated_at: input.generated_at.clone(),
        freshness_class: input.freshness_class.clone(),
        human_verified: input.human_verified,
        replayable_offline: input.replayable_offline,
        return_path_ref: input.return_path_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn suggestion_apply_record(
    input: &SafeSuggestionApplyInput,
    evidence: &AiReviewEvidenceRecord,
) -> SafeSuggestionApplyRecord {
    SafeSuggestionApplyRecord {
        record_kind: SAFE_SUGGESTION_APPLY_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        evidence_id_ref: evidence.evidence_id.clone(),
        suggestion_apply_id: input.suggestion_apply_id.clone(),
        review_row_ref: input.review_row_ref.clone(),
        source_evidence_ref: input.source_evidence_ref.clone(),
        apply_state: input.apply_state.clone(),
        suggestion_authority_class: input.suggestion_authority_class.clone(),
        preview_supported: input.preview_supported,
        checkpoint_before_apply: input.checkpoint_before_apply,
        reversible_after_apply: input.reversible_after_apply,
        would_broaden_authority: input.would_broaden_authority,
        checkpoint_ref: input.checkpoint_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn checkpoint_record(
    input: &SuggestionApplyCheckpointInput,
    evidence: &AiReviewEvidenceRecord,
) -> SuggestionApplyCheckpointRecord {
    SuggestionApplyCheckpointRecord {
        record_kind: SUGGESTION_APPLY_CHECKPOINT_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        evidence_id_ref: evidence.evidence_id.clone(),
        checkpoint_id: input.checkpoint_id.clone(),
        suggestion_apply_ref: input.suggestion_apply_ref.clone(),
        checkpoint_state: input.checkpoint_state.clone(),
        base_revision_ref: input.base_revision_ref.clone(),
        head_revision_ref: input.head_revision_ref.clone(),
        worktree_state_hash: input.worktree_state_hash.clone(),
        created_at: input.created_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn command_record(
    input: &AiEvidenceCommandInput,
    evidence: &AiReviewEvidenceRecord,
) -> AiEvidenceCommandRecord {
    AiEvidenceCommandRecord {
        record_kind: AI_EVIDENCE_COMMAND_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        evidence_id_ref: evidence.evidence_id.clone(),
        command_class: input.command_class.clone(),
        target_object_ref: input.target_object_ref.clone(),
        target_object_kind: input.target_object_kind.clone(),
        preview_supported: input.preview_supported,
        emits_audit_event: input.emits_audit_event,
        blocked_reasons: input.blocked_reasons.clone(),
        actionable: input.blocked_reasons.is_empty(),
        summary_label: input.summary_label.clone(),
    }
}

fn support_export_packet(
    input: &AiEvidenceSupportExportInput,
    evidence: &AiReviewEvidenceRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    commands: &[AiEvidenceCommandRecord],
) -> AiEvidenceSupportExportPacket {
    AiEvidenceSupportExportPacket {
        record_kind: AI_EVIDENCE_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        evidence_id_ref: evidence.evidence_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec!["schemas/review/ai_review_evidence.schema.json".to_string()],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn inspection_record(
    evidence: &AiReviewEvidenceRecord,
    attachments: &[AiEvidenceAttachmentRecord],
    suggestions: &[SafeSuggestionApplyRecord],
    checkpoints: &[SuggestionApplyCheckpointRecord],
    commands: &[AiEvidenceCommandRecord],
    support_export: &AiEvidenceSupportExportPacket,
) -> AiEvidenceInspectionRecord {
    AiEvidenceInspectionRecord {
        record_kind: AI_EVIDENCE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        evidence_id_ref: evidence.evidence_id.clone(),
        review_workspace_id_ref: evidence.review_workspace_id_ref.clone(),
        evidence_state_attached_current: evidence.evidence_state == "attached_current",
        evidence_state_attached_stale: evidence.evidence_state == "attached_stale",
        evidence_state_detached_invalidated: evidence.evidence_state == "detached_invalidated",
        evidence_state_pending_verification: evidence.evidence_state == "pending_verification",
        ai_model_source_present: attachments.iter().any(|a| {
            a.evidence_source_class == "ai_model_local"
                || a.evidence_source_class == "ai_model_provider"
                || a.evidence_source_class == "ai_model_hybrid"
        }),
        human_curated_source_present: attachments
            .iter()
            .any(|a| a.evidence_source_class == "human_curated_ai"),
        suggestion_preview_ready: suggestions.iter().any(|s| s.apply_state == "preview_ready"),
        suggestion_applied_with_checkpoint: suggestions
            .iter()
            .any(|s| s.apply_state == "applied_with_checkpoint"),
        suggestion_reverted: suggestions.iter().any(|s| s.apply_state == "reverted"),
        suggestion_blocked_pending_review: suggestions
            .iter()
            .any(|s| s.apply_state == "blocked_pending_review"),
        suggestion_blocked_scope_exceeded: suggestions
            .iter()
            .any(|s| s.apply_state == "blocked_scope_exceeded"),
        suggestion_would_broaden_authority: suggestions.iter().any(|s| s.would_broaden_authority),
        all_applied_have_checkpoints: suggestions.iter().all(|s| {
            if s.apply_state == "applied_with_checkpoint" {
                s.checkpoint_ref.is_some()
            } else {
                true
            }
        }),
        all_checkpoints_recoverable: checkpoints.iter().all(|c| {
            c.checkpoint_state == "checkpoint_created" || c.checkpoint_state == "checkpoint_applied"
        }),
        actionable: evidence.actionable,
        invalidated: !evidence.invalidation_reasons.is_empty(),
        evidence_attachment_count: attachments.len(),
        suggestion_apply_count: suggestions.len(),
        checkpoint_count: checkpoints.len(),
        command_count: commands.len(),
        preview_capable: commands.iter().any(|c| c.preview_supported),
        support_export_reopenable: !support_export.reopen_context_ref.is_empty(),
        summary_label: evidence.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &AiReviewEvidenceInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_nonempty(&input.evidence_id, "evidence_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.evidence_state, "evidence_state")?;
    ensure_token(AI_EVIDENCE_STATES, &input.evidence_state, "evidence_state")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    for reason in &input.invalidation_reasons {
        ensure_token(
            AI_EVIDENCE_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }

    if input.evidence_attachments.is_empty() {
        return Err(ai_evidence_validation_error(
            "input must contain at least one evidence_attachment".to_string(),
        ));
    }

    let mut attachment_ids = std::collections::BTreeSet::new();
    for attachment in &input.evidence_attachments {
        ensure_nonempty(
            &attachment.attachment_id,
            "evidence_attachment.attachment_id",
        )?;
        if !attachment_ids.insert(&attachment.attachment_id) {
            return Err(ai_evidence_validation_error(format!(
                "duplicate attachment_id: {}",
                attachment.attachment_id
            )));
        }
        ensure_token(
            AI_EVIDENCE_SOURCE_CLASSES,
            &attachment.evidence_source_class,
            "evidence_attachment.evidence_source_class",
        )?;
    }

    let mut suggestion_ids = std::collections::BTreeSet::new();
    for suggestion in &input.suggestion_applies {
        ensure_nonempty(
            &suggestion.suggestion_apply_id,
            "suggestion_apply.suggestion_apply_id",
        )?;
        if !suggestion_ids.insert(&suggestion.suggestion_apply_id) {
            return Err(ai_evidence_validation_error(format!(
                "duplicate suggestion_apply_id: {}",
                suggestion.suggestion_apply_id
            )));
        }
        ensure_token(
            SUGGESTION_APPLY_STATES,
            &suggestion.apply_state,
            "suggestion_apply.apply_state",
        )?;
        ensure_token(
            SUGGESTION_AUTHORITY_CLASSES,
            &suggestion.suggestion_authority_class,
            "suggestion_apply.suggestion_authority_class",
        )?;
    }

    let mut checkpoint_ids = std::collections::BTreeSet::new();
    for checkpoint in &input.checkpoints {
        ensure_nonempty(&checkpoint.checkpoint_id, "checkpoint.checkpoint_id")?;
        if !checkpoint_ids.insert(&checkpoint.checkpoint_id) {
            return Err(ai_evidence_validation_error(format!(
                "duplicate checkpoint_id: {}",
                checkpoint.checkpoint_id
            )));
        }
        ensure_token(
            CHECKPOINT_STATES,
            &checkpoint.checkpoint_state,
            "checkpoint.checkpoint_state",
        )?;
    }

    let mut command_ids = std::collections::BTreeSet::new();
    for command in &input.commands {
        ensure_nonempty(&command.command_id, "command.command_id")?;
        if !command_ids.insert(&command.command_id) {
            return Err(ai_evidence_validation_error(format!(
                "duplicate command_id: {}",
                command.command_id
            )));
        }
        ensure_token(
            AI_EVIDENCE_COMMAND_CLASSES,
            &command.command_class,
            "command.command_class",
        )?;
    }

    ensure_nonempty(
        &input.support_export.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &input.support_export.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(
        &input.support_export.reopen_command_id_ref,
        "support_export.reopen_command_id_ref",
    )?;
    for surface in &input.support_export.consumer_surfaces {
        ensure_token(
            AI_EVIDENCE_CONSUMER_SURFACES,
            surface,
            "support_export.consumer_surfaces",
        )?;
    }

    // Workspace packet must have a valid workspace id
    ensure_nonempty(
        &workspace_packet.review_workspace.review_workspace_id,
        "workspace_packet.review_workspace.review_workspace_id",
    )?;

    Ok(())
}

fn validate_evidence_record(
    record: &AiReviewEvidenceRecord,
    workspace_id: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        AI_REVIEW_EVIDENCE_RECORD_KIND,
        "evidence record_kind",
    )?;
    ensure_eq_u32(
        record.schema_version,
        AI_REVIEW_EVIDENCE_SCHEMA_VERSION,
        "evidence schema_version",
    )?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        workspace_id,
        "evidence review_workspace_id_ref",
    )?;
    ensure_token(
        AI_EVIDENCE_STATES,
        &record.evidence_state,
        "evidence evidence_state",
    )?;
    for reason in &record.invalidation_reasons {
        ensure_token(
            AI_EVIDENCE_INVALIDATION_REASONS,
            reason,
            "evidence invalidation_reason",
        )?;
    }
    Ok(())
}

fn validate_attachment_record(
    record: &AiEvidenceAttachmentRecord,
    evidence_id: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        AI_EVIDENCE_ATTACHMENT_RECORD_KIND,
        "attachment record_kind",
    )?;
    ensure_eq(
        record.evidence_id_ref.as_str(),
        evidence_id,
        "attachment evidence_id_ref",
    )?;
    ensure_token(
        AI_EVIDENCE_SOURCE_CLASSES,
        &record.evidence_source_class,
        "attachment evidence_source_class",
    )?;
    Ok(())
}

fn validate_suggestion_apply_record(
    record: &SafeSuggestionApplyRecord,
    evidence_id: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        SAFE_SUGGESTION_APPLY_RECORD_KIND,
        "suggestion_apply record_kind",
    )?;
    ensure_eq(
        record.evidence_id_ref.as_str(),
        evidence_id,
        "suggestion_apply evidence_id_ref",
    )?;
    ensure_token(
        SUGGESTION_APPLY_STATES,
        &record.apply_state,
        "suggestion_apply apply_state",
    )?;
    ensure_token(
        SUGGESTION_AUTHORITY_CLASSES,
        &record.suggestion_authority_class,
        "suggestion_apply suggestion_authority_class",
    )?;
    Ok(())
}

fn validate_checkpoint_record(
    record: &SuggestionApplyCheckpointRecord,
    evidence_id: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        SUGGESTION_APPLY_CHECKPOINT_RECORD_KIND,
        "checkpoint record_kind",
    )?;
    ensure_eq(
        record.evidence_id_ref.as_str(),
        evidence_id,
        "checkpoint evidence_id_ref",
    )?;
    ensure_token(
        CHECKPOINT_STATES,
        &record.checkpoint_state,
        "checkpoint checkpoint_state",
    )?;
    Ok(())
}

fn validate_command_record(
    record: &AiEvidenceCommandRecord,
    evidence_id: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        AI_EVIDENCE_COMMAND_RECORD_KIND,
        "command record_kind",
    )?;
    ensure_eq(
        record.evidence_id_ref.as_str(),
        evidence_id,
        "command evidence_id_ref",
    )?;
    ensure_token(
        AI_EVIDENCE_COMMAND_CLASSES,
        &record.command_class,
        "command command_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &AiEvidenceSupportExportPacket,
    evidence: &AiReviewEvidenceRecord,
    commands: &[AiEvidenceCommandRecord],
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        AI_EVIDENCE_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export record_kind",
    )?;
    ensure_eq(
        export.evidence_id_ref.as_str(),
        evidence.evidence_id.as_str(),
        "support_export evidence_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        evidence.review_workspace_id_ref.as_str(),
        "support_export review_workspace_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(ai_evidence_validation_error(
            "support_export raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(ai_evidence_validation_error(
            "support_export raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.command_id_refs.len() != commands.len() {
        return Err(ai_evidence_validation_error(
            "support_export command_id_refs length must match commands length",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &AiEvidenceInspectionRecord,
    packet: &AiReviewEvidencePacket,
) -> Result<(), AiReviewEvidenceValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        AI_EVIDENCE_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.evidence_id_ref.as_str(),
        packet.evidence.evidence_id.as_str(),
        "inspection evidence_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection review_workspace_id_ref",
    )?;
    if inspection.command_count != packet.commands.len() {
        return Err(ai_evidence_validation_error(
            "inspection command_count must match commands length",
        ));
    }
    if inspection.evidence_attachment_count != packet.evidence_attachments.len() {
        return Err(ai_evidence_validation_error(
            "inspection evidence_attachment_count must match evidence_attachments length",
        ));
    }
    if inspection.suggestion_apply_count != packet.suggestion_applies.len() {
        return Err(ai_evidence_validation_error(
            "inspection suggestion_apply_count must match suggestion_applies length",
        ));
    }
    if inspection.checkpoint_count != packet.checkpoints.len() {
        return Err(ai_evidence_validation_error(
            "inspection checkpoint_count must match checkpoints length",
        ));
    }
    if inspection.suggestion_would_broaden_authority
        != packet
            .suggestion_applies
            .iter()
            .any(|s| s.would_broaden_authority)
    {
        return Err(ai_evidence_validation_error(
            "inspection suggestion_would_broaden_authority must match suggestion_applies",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn ai_evidence_validation_error(message: impl Into<String>) -> AiReviewEvidenceValidationError {
    AiReviewEvidenceValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), AiReviewEvidenceValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(ai_evidence_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    if left != right {
        return Err(ai_evidence_validation_error(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), AiReviewEvidenceValidationError> {
    if value.trim().is_empty() {
        return Err(ai_evidence_validation_error(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), AiReviewEvidenceValidationError> {
    if !tokens.contains(&value) {
        return Err(ai_evidence_validation_error(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

fn contains_token(tokens: &[&str], value: &str) -> bool {
    tokens.contains(&value)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_nonempty() {
        assert!(!AI_EVIDENCE_STATES.is_empty());
        assert!(!AI_EVIDENCE_SOURCE_CLASSES.is_empty());
        assert!(!SUGGESTION_APPLY_STATES.is_empty());
        assert!(!SUGGESTION_AUTHORITY_CLASSES.is_empty());
        assert!(!AI_EVIDENCE_COMMAND_CLASSES.is_empty());
        assert!(!AI_EVIDENCE_CONSUMER_SURFACES.is_empty());
        assert!(!AI_EVIDENCE_INVALIDATION_REASONS.is_empty());
        assert!(!CHECKPOINT_STATES.is_empty());
    }

    #[test]
    fn evidence_validation_rejects_empty_evidence_id() {
        let input = AiReviewEvidenceInput {
            evidence_id: "".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            evidence_state: "attached_current".to_string(),
            evidence_attachments: vec![AiEvidenceAttachmentInput {
                attachment_id: "a1".to_string(),
                review_row_ref: "r1".to_string(),
                evidence_source_class: "ai_model_local".to_string(),
                model_run_ref: "m1".to_string(),
                actor_ref: "actor1".to_string(),
                generated_at: "2026-05-27T10:00:00Z".to_string(),
                freshness_class: "current".to_string(),
                human_verified: false,
                replayable_offline: true,
                return_path_ref: "ret1".to_string(),
                summary_label: "Attachment".to_string(),
            }],
            suggestion_applies: vec![],
            checkpoints: vec![],
            commands: vec![],
            support_export: AiEvidenceSupportExportInput {
                support_export_id: "se1".to_string(),
                reopen_context_ref: "rc1".to_string(),
                reopen_command_id_ref: "cmd1".to_string(),
                consumer_surfaces: vec!["support_export".to_string()],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Export".to_string(),
            },
            invalidation_reasons: vec![],
            summary_label: "Summary".to_string(),
        };

        let workspace = ReviewWorkspaceBetaPacket {
            record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND.to_string(),
            schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            packet_id: "wp1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            review_workspace: crate::workspace::ReviewWorkspaceRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_RECORD_KIND.to_string(),
                review_workspace_schema_version:
                    crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id: "ws1".to_string(),
                review_workspace_source_class: "local_git".to_string(),
                provider_authority_class: "local_only".to_string(),
                review_workspace_lifecycle_state: "active".to_string(),
                local_locator: Some(crate::workspace::ReviewLocalLocator {
                    workspace_id_ref: "ws1".to_string(),
                    branch_or_worktree_ref: "main".to_string(),
                    base_revision_ref: None,
                    head_revision_ref: None,
                }),
                provider_overlay: None,
                imported_bundle_envelope: None,
                browser_handoff_envelope: None,
                policy_context: crate::workspace::ReviewPolicyContext {
                    policy_epoch: "1".to_string(),
                    trust_state: "trusted".to_string(),
                    execution_context_id: None,
                    workspace_trust_state_class: "trusted_local".to_string(),
                },
                client_scopes: vec![],
                redaction_class: "metadata_safe_default".to_string(),
                freshness_class: "current".to_string(),
                summary_label: "Workspace".to_string(),
                created_at: "2026-05-27T10:00:00Z".to_string(),
                updated_at: "2026-05-27T10:00:00Z".to_string(),
                archived_at: None,
                hosted_review_inbox_record_id_ref: None,
                merge_policy_record_id_ref: None,
            },
            diff_entries: vec![],
            durable_comment_anchors: vec![],
            object_lineage: vec![],
            check_freshness: vec![],
            browser_handoff: None,
            inspection: crate::workspace::ReviewWorkspaceBetaInspectionRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id_ref: "ws1".to_string(),
                durable_comment_anchor_count: 0,
                object_lineage_count: 0,
                check_freshness_count: 0,
                anchor_identity_preserved: true,
                object_lineage_preserved: true,
                check_freshness_browser_independent: true,
                typed_reversible_browser_handoff_present: false,
                support_export_reopenable: true,
                raw_escape_hatches_absent: true,
                operator_truth_current: true,
                stale_check_blocks_operator_truth: false,
                summary_label: "Inspection".to_string(),
            },
            support_export: crate::workspace::ReviewWorkspaceSupportExportPacket {
                record_kind: crate::workspace::REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                support_export_id: "wse1".to_string(),
                review_workspace_id_ref: "ws1".to_string(),
                reopen_context_ref: "wrc1".to_string(),
                reopen_command_id_ref: "wcmd1".to_string(),
                durable_comment_anchor_refs: vec![],
                check_freshness_refs: vec![],
                object_lineage_refs: vec![],
                browser_handoff_ref: None,
                consumer_surfaces: vec!["support_export".to_string()],
                source_schema_refs: vec![],
                raw_comment_body_export_allowed: false,
                raw_url_export_allowed: false,
                raw_source_body_export_allowed: false,
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Export".to_string(),
            },
        };

        let result = AiReviewEvidencePacket::from_workspace_packet(input, &workspace);
        assert!(result.is_err());
    }

    #[test]
    fn suggestion_would_broaden_authority_blocks_apply() {
        let input = AiReviewEvidenceInput {
            evidence_id: "e1".to_string(),
            packet_id: "p1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            evidence_state: "attached_current".to_string(),
            evidence_attachments: vec![AiEvidenceAttachmentInput {
                attachment_id: "a1".to_string(),
                review_row_ref: "r1".to_string(),
                evidence_source_class: "ai_model_local".to_string(),
                model_run_ref: "m1".to_string(),
                actor_ref: "actor1".to_string(),
                generated_at: "2026-05-27T10:00:00Z".to_string(),
                freshness_class: "current".to_string(),
                human_verified: false,
                replayable_offline: true,
                return_path_ref: "ret1".to_string(),
                summary_label: "Attachment".to_string(),
            }],
            suggestion_applies: vec![SafeSuggestionApplyInput {
                suggestion_apply_id: "s1".to_string(),
                review_row_ref: "r1".to_string(),
                source_evidence_ref: "a1".to_string(),
                apply_state: "applied_with_checkpoint".to_string(),
                suggestion_authority_class: "advisory_only".to_string(),
                preview_supported: true,
                checkpoint_before_apply: true,
                reversible_after_apply: true,
                would_broaden_authority: true,
                checkpoint_ref: Some("c1".to_string()),
                summary_label: "Suggestion".to_string(),
            }],
            checkpoints: vec![SuggestionApplyCheckpointInput {
                checkpoint_id: "c1".to_string(),
                suggestion_apply_ref: "s1".to_string(),
                checkpoint_state: "checkpoint_applied".to_string(),
                base_revision_ref: "base1".to_string(),
                head_revision_ref: "head1".to_string(),
                worktree_state_hash: "hash1".to_string(),
                created_at: "2026-05-27T10:00:00Z".to_string(),
                summary_label: "Checkpoint".to_string(),
            }],
            commands: vec![],
            support_export: AiEvidenceSupportExportInput {
                support_export_id: "se1".to_string(),
                reopen_context_ref: "rc1".to_string(),
                reopen_command_id_ref: "cmd1".to_string(),
                consumer_surfaces: vec!["support_export".to_string()],
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Export".to_string(),
            },
            invalidation_reasons: vec![],
            summary_label: "Summary".to_string(),
        };

        let workspace = ReviewWorkspaceBetaPacket {
            record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND.to_string(),
            schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            packet_id: "wp1".to_string(),
            generated_at: "2026-05-27T10:00:00Z".to_string(),
            review_workspace: crate::workspace::ReviewWorkspaceRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_RECORD_KIND.to_string(),
                review_workspace_schema_version:
                    crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id: "ws1".to_string(),
                review_workspace_source_class: "local_git".to_string(),
                provider_authority_class: "local_only".to_string(),
                review_workspace_lifecycle_state: "active".to_string(),
                local_locator: Some(crate::workspace::ReviewLocalLocator {
                    workspace_id_ref: "ws1".to_string(),
                    branch_or_worktree_ref: "main".to_string(),
                    base_revision_ref: None,
                    head_revision_ref: None,
                }),
                provider_overlay: None,
                imported_bundle_envelope: None,
                browser_handoff_envelope: None,
                policy_context: crate::workspace::ReviewPolicyContext {
                    policy_epoch: "1".to_string(),
                    trust_state: "trusted".to_string(),
                    execution_context_id: None,
                    workspace_trust_state_class: "trusted_local".to_string(),
                },
                client_scopes: vec![],
                redaction_class: "metadata_safe_default".to_string(),
                freshness_class: "current".to_string(),
                summary_label: "Workspace".to_string(),
                created_at: "2026-05-27T10:00:00Z".to_string(),
                updated_at: "2026-05-27T10:00:00Z".to_string(),
                archived_at: None,
                hosted_review_inbox_record_id_ref: None,
                merge_policy_record_id_ref: None,
            },
            diff_entries: vec![],
            durable_comment_anchors: vec![],
            object_lineage: vec![],
            check_freshness: vec![],
            browser_handoff: None,
            inspection: crate::workspace::ReviewWorkspaceBetaInspectionRecord {
                record_kind: crate::workspace::REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                review_workspace_id_ref: "ws1".to_string(),
                durable_comment_anchor_count: 0,
                object_lineage_count: 0,
                check_freshness_count: 0,
                anchor_identity_preserved: true,
                object_lineage_preserved: true,
                check_freshness_browser_independent: true,
                typed_reversible_browser_handoff_present: false,
                support_export_reopenable: true,
                raw_escape_hatches_absent: true,
                operator_truth_current: true,
                stale_check_blocks_operator_truth: false,
                summary_label: "Inspection".to_string(),
            },
            support_export: crate::workspace::ReviewWorkspaceSupportExportPacket {
                record_kind: crate::workspace::REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND
                    .to_string(),
                schema_version: crate::workspace::REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
                support_export_id: "wse1".to_string(),
                review_workspace_id_ref: "ws1".to_string(),
                reopen_context_ref: "wrc1".to_string(),
                reopen_command_id_ref: "wcmd1".to_string(),
                durable_comment_anchor_refs: vec![],
                check_freshness_refs: vec![],
                object_lineage_refs: vec![],
                browser_handoff_ref: None,
                consumer_surfaces: vec!["support_export".to_string()],
                source_schema_refs: vec![],
                raw_comment_body_export_allowed: false,
                raw_url_export_allowed: false,
                raw_source_body_export_allowed: false,
                redaction_class: "metadata_safe_default".to_string(),
                summary_label: "Export".to_string(),
            },
        };

        let result = AiReviewEvidencePacket::from_workspace_packet(input, &workspace);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("would_broaden_authority"));
    }
}
