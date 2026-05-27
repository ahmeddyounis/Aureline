//! Finalized migration rollback checkpoints, diff review, and retained diagnostics
//! for failed imports.
//!
//! This module hardens the migration lane by requiring a diff review gate and an
//! explicit rollback checkpoint before any destructive import is applied. It
//! consumes the workspace migration-wizard import-fidelity beta record family
//! (by reference) and projects it into review surfaces as a
//! [`MigrationRollbackDiffReviewPacket`] that carries:
//!
//! - [`MigrationRollbackDiffReviewRecord`] — stable identity, operation provenance,
//!   source editor, target families, and flow state.
//! - [`MigrationDiffReviewRecord`] — the review gate that blocks apply until the
//!   diff is approved and checkpoints are captured.
//! - [`MigrationRollbackCheckpointRecord`] — summary of the checkpoint captured
//!   before the destructive step, or the disclosure acknowledged.
//! - [`RetainedDiagnosticRecord`] — diagnostics retained for failed imports with
//!   reason class, suggested action, and fallback posture.
//! - [`MigrationCommandRecord`] — command-graph operations surfaced to the
//!   inspector (preview diff, approve, capture checkpoint, apply, rollback, abort,
//!   review diagnostics, etc.).
//! - [`MigrationSupportExportPacket`] — redaction-safe support export that
//!   can reopen the same structured migration truth after restart.
//! - [`MigrationInspectionRecord`] — compact boolean projection for CLI and
//!   inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/migration_rollback_diff_review.schema.json`. Canonical fixtures
//! live under
//! `fixtures/review/m4/finalize-migration-rollback-checkpoints-diff-review-and-retained/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every migration rollback diff-review record.
pub const MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`MigrationRollbackDiffReviewPacket`].
pub const MIGRATION_ROLLBACK_DIFF_REVIEW_PACKET_RECORD_KIND: &str =
    "review_migration_rollback_diff_review_packet";

/// Stable record-kind tag for [`MigrationRollbackDiffReviewRecord`].
pub const MIGRATION_ROLLBACK_DIFF_REVIEW_RECORD_KIND: &str =
    "review_migration_rollback_diff_review_record";

/// Stable record-kind tag for [`MigrationDiffReviewRecord`].
pub const MIGRATION_DIFF_REVIEW_RECORD_KIND: &str = "review_migration_diff_review_record";

/// Stable record-kind tag for [`MigrationRollbackCheckpointRecord`].
pub const MIGRATION_ROLLBACK_CHECKPOINT_RECORD_KIND: &str =
    "review_migration_rollback_checkpoint_record";

/// Stable record-kind tag for [`RetainedDiagnosticRecord`].
pub const RETAINED_DIAGNOSTIC_RECORD_KIND: &str = "review_retained_diagnostic_record";

/// Stable record-kind tag for [`MigrationCommandRecord`].
pub const MIGRATION_COMMAND_RECORD_KIND: &str = "review_migration_command_record";

/// Stable record-kind tag for [`MigrationSupportExportPacket`].
pub const MIGRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_migration_support_export_packet";

/// Stable record-kind tag for [`MigrationInspectionRecord`].
pub const MIGRATION_INSPECTION_RECORD_KIND: &str = "review_migration_inspection_record";

/// Closed set of migration operation kinds.
pub const MIGRATION_OPERATION_KINDS: &[&str] = &[
    "settings_import",
    "keymap_import",
    "snippet_import",
    "theme_import",
    "task_import",
    "launch_config_import",
    "workspace_metadata_import",
    "extension_manifest_import",
];

/// Closed set of migration flow lifecycle states.
pub const MIGRATION_FLOW_STATES: &[&str] = &[
    "preview_pending",
    "diff_pending_review",
    "diff_review_approved",
    "checkpoint_pending",
    "checkpoint_captured",
    "applying",
    "applied",
    "validation_failed",
    "rolled_back",
    "aborted",
];

/// Closed set of migration diff-first review states.
pub const MIGRATION_DIFF_REVIEW_STATES: &[&str] = &[
    "pending",
    "approved_with_checkpoints",
    "rejected",
    "requires_manual_review",
];

/// Closed set of migration checkpoint-summary states.
pub const MIGRATION_CHECKPOINT_STATES: &[&str] = &[
    "none_required",
    "captured_ready",
    "captured_pending",
    "restored",
    "expired",
    "missing_blocks_apply",
];

/// Closed set of diagnostic reason classes.
pub const MIGRATION_DIAGNOSTIC_REASON_CLASSES: &[&str] = &[
    "no_semantic_equivalent",
    "ambiguous_mapping",
    "secret_material_excluded",
    "policy_locked",
    "capability_missing",
    "version_mismatch",
    "corrupted_source",
    "partial_schema_match",
];

/// Closed set of suggested action classes for diagnostics.
pub const MIGRATION_DIAGNOSTIC_ACTION_CLASSES: &[&str] = &[
    "manual_review",
    "use_bridge",
    "use_native_alternative",
    "skip_and_continue",
    "rollback_and_repair",
    "contact_support",
];

/// Closed set of command classes for the migration lane.
pub const MIGRATION_COMMAND_CLASSES: &[&str] = &[
    "preview_diff",
    "approve_diff",
    "reject_diff",
    "capture_checkpoint",
    "restore_checkpoint",
    "apply_migration",
    "rollback_migration",
    "abort_flow",
    "review_diagnostics",
    "continue_after_resolve",
];

/// Closed set of consumer surfaces for migration packets.
pub const MIGRATION_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "migration_center",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
];

/// Closed set of invalidation reasons that mark a migration flow stale.
pub const MIGRATION_INVALIDATION_REASONS: &[&str] = &[
    "source_profile_changed",
    "checkpoint_expired",
    "validation_failed",
    "user_aborted",
    "policy_blocked",
    "downgrade_required",
    "provider_overlay_stale",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a migration rollback diff-review flow to materialize on top
/// of a review-workspace packet and an optional migration-wizard import-fidelity
/// record reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRollbackDiffReviewInput {
    /// Stable migration-flow identity.
    pub migration_flow_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Operation kind from the closed vocabulary.
    pub operation_kind: String,
    /// Current flow lifecycle state.
    pub flow_state: String,
    /// Source editor ecosystem (e.g. `vs_code_code_oss`).
    pub source_editor: String,
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Target family refs selected for this migration.
    pub target_family_refs: Vec<String>,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Stable session ref used to reopen this flow after restart.
    pub restart_session_ref: String,
    /// Diff-first review input.
    pub diff_review: MigrationDiffReviewInput,
    /// Rollback checkpoint summary input.
    pub rollback_checkpoint: MigrationRollbackCheckpointInput,
    /// Retained diagnostics for failed imports.
    #[serde(default)]
    pub retained_diagnostics: Vec<RetainedDiagnosticInput>,
    /// Command-graph operations defined for this flow.
    pub commands: Vec<MigrationCommandInput>,
    /// Support/export envelope input.
    pub support_export: MigrationSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing the migration diff-first review gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationDiffReviewInput {
    /// Diff review state from the closed vocabulary.
    pub diff_review_state: String,
    /// Opaque ref to the diff preview packet reviewed by the user.
    pub diff_preview_ref: String,
    /// True when suspicious content was flagged by the safety layer.
    pub suspicious_content_flagged: bool,
    /// True when the user explicitly reviewed flagged content.
    pub suspicious_content_reviewed: bool,
    /// True when a checkpoint is required before apply.
    pub checkpoint_required_before_apply: bool,
    /// Optional reason when the review requires manual inspection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_review_reason: Option<String>,
}

/// Input describing the migration rollback-checkpoint summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRollbackCheckpointInput {
    /// Checkpoint summary state from the closed vocabulary.
    pub checkpoint_state: String,
    /// Opaque checkpoint ref when captured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Stable command id that restores the checkpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_command_id: Option<String>,
    /// Redaction-safe disclosure label.
    pub disclosure_label: String,
    /// True when the checkpoint is restorable without network egress.
    pub restorable_offline: bool,
}

/// Input describing one retained diagnostic for a failed import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedDiagnosticInput {
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Import target family.
    pub import_target_family: String,
    /// Source item identifier.
    pub source_item_id: String,
    /// Diagnostic reason class from the closed vocabulary.
    pub reason_class: String,
    /// Human-readable, redaction-aware diagnostic message.
    pub message: String,
    /// Suggested action class from the closed vocabulary.
    pub suggested_action: String,
    /// Whether a fallback or bridge path exists.
    pub fallback_available: bool,
}

/// Input describing one command-graph operation for a migration flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCommandInput {
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

/// Input row for the migration support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the flow.
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

/// Migration rollback diff-review record materialized from input plus workspace
/// truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRollbackDiffReviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable migration-flow identity.
    pub migration_flow_id: String,
    /// Review workspace this flow belongs to.
    pub review_workspace_id_ref: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Current flow lifecycle state.
    pub flow_state: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Target family refs.
    pub target_family_refs: Vec<String>,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Stable session ref used to reopen this flow after restart.
    pub restart_session_ref: String,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocking reasons preventing apply.
    pub blocked_reasons: Vec<String>,
    /// True when the flow is actionable from the current state.
    pub actionable: bool,
    /// Timestamp the flow was frozen.
    pub generated_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Migration diff-first review record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationDiffReviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Migration flow this review gate belongs to.
    pub migration_flow_id_ref: String,
    /// Diff review state.
    pub diff_review_state: String,
    /// Opaque ref to the diff preview packet.
    pub diff_preview_ref: String,
    /// True when suspicious content was flagged.
    pub suspicious_content_flagged: bool,
    /// True when the user explicitly reviewed flagged content.
    pub suspicious_content_reviewed: bool,
    /// True when a checkpoint is required before apply.
    pub checkpoint_required_before_apply: bool,
    /// Optional manual-review reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_review_reason: Option<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Migration rollback-checkpoint summary record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRollbackCheckpointRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Migration flow this summary belongs to.
    pub migration_flow_id_ref: String,
    /// Checkpoint summary state.
    pub checkpoint_state: String,
    /// Opaque checkpoint ref when captured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Stable command id that restores the checkpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_command_id: Option<String>,
    /// Redaction-safe disclosure label.
    pub disclosure_label: String,
    /// True when the checkpoint is restorable without network egress.
    pub restorable_offline: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Retained diagnostic record for failed imports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedDiagnosticRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Migration flow this diagnostic belongs to.
    pub migration_flow_id_ref: String,
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Source editor ecosystem.
    pub source_editor: String,
    /// Import target family.
    pub import_target_family: String,
    /// Source item identifier.
    pub source_item_id: String,
    /// Diagnostic reason class.
    pub reason_class: String,
    /// Human-readable, redaction-aware diagnostic message.
    pub message: String,
    /// Suggested action class.
    pub suggested_action: String,
    /// Whether a fallback or bridge path exists.
    pub fallback_available: bool,
    /// Timestamp when the diagnostic was retained.
    pub retained_at: String,
}

/// Command-graph operation record for a migration flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Migration flow this command belongs to.
    pub migration_flow_id_ref: String,
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
    /// True when the command is actionable from the current flow state.
    pub actionable: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Inspection row used by support/export and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Migration flow inspected by this row.
    pub migration_flow_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the diff has been approved with checkpoints.
    pub diff_approved: bool,
    /// True when the diff review is still pending.
    pub diff_pending: bool,
    /// True when the diff was rejected.
    pub diff_rejected: bool,
    /// True when a checkpoint is captured and ready.
    pub checkpoint_ready: bool,
    /// True when the flow is currently applying.
    pub applying: bool,
    /// True when the flow completed successfully.
    pub completed: bool,
    /// True when the flow failed validation.
    pub validation_failed: bool,
    /// True when the flow was rolled back.
    pub rolled_back: bool,
    /// True when the flow was aborted.
    pub aborted: bool,
    /// True when approval is invalidated.
    pub approval_invalidated: bool,
    /// True when checks are stale enough to block apply.
    pub checks_stale_blocks_apply: bool,
    /// True when the flow is actionable from the current state.
    pub actionable: bool,
    /// True when the flow can be reopened after restart.
    pub restartable: bool,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the flow context.
    pub support_export_reopenable: bool,
    /// True when suspicious content was flagged but not reviewed.
    pub suspicious_content_unreviewed: bool,
    /// Number of retained diagnostics.
    pub retained_diagnostic_count: usize,
    /// True when at least one retained diagnostic has a fallback path.
    pub retained_diagnostic_fallback_available: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the migration lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Migration flow this packet exports.
    pub migration_flow_id_ref: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stable context ref used to reopen the flow.
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
    /// False so raw paths cannot cross the support boundary.
    pub raw_path_export_allowed: bool,
    /// False so raw branch names cannot cross the support boundary.
    pub raw_branch_name_export_allowed: bool,
    /// False so raw patch bodies cannot cross the support boundary.
    pub raw_patch_body_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Restart snapshot used by support to reconstruct session truth.
    pub restart_snapshot: MigrationRestartSnapshot,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Restart snapshot embedded in the migration support-export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRestartSnapshot {
    /// Flow state at export time.
    pub flow_state: String,
    /// Diff review state at export time.
    pub diff_review_state: String,
    /// Checkpoint state at export time.
    pub checkpoint_state: String,
    /// Operation kind at export time.
    pub operation_kind: String,
    /// Restart session ref.
    pub restart_session_ref: String,
    /// Active invalidation reasons at export time.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons at export time.
    pub blocked_reasons: Vec<String>,
}


// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Migration rollback diff-review packet consumed by review surfaces and support
/// exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRollbackDiffReviewPacket {
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
    /// Migration rollback diff-review record.
    pub migration_flow: MigrationRollbackDiffReviewRecord,
    /// Diff-first review record.
    pub diff_review: MigrationDiffReviewRecord,
    /// Recovery checkpoint summary record.
    pub rollback_checkpoint: MigrationRollbackCheckpointRecord,
    /// Retained diagnostic records.
    #[serde(default)]
    pub retained_diagnostics: Vec<RetainedDiagnosticRecord>,
    /// Command-graph operation records.
    pub commands: Vec<MigrationCommandRecord>,
    /// Support/export packet.
    pub support_export: MigrationSupportExportPacket,
    /// Inspection row.
    pub inspection: MigrationInspectionRecord,
}

impl MigrationRollbackDiffReviewPacket {
    /// Builds a migration rollback diff-review packet from a beta review-workspace
    /// packet and migration flow input.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationRollbackDiffReviewValidationError`] when the input
    /// violates a migration flow invariant.
    pub fn from_workspace_packet(
        input: MigrationRollbackDiffReviewInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, MigrationRollbackDiffReviewValidationError> {
        validate_input(&input, workspace_packet)?;

        let migration_flow = migration_flow_record(&input, workspace_packet);
        let diff_review = diff_review_record(&input.diff_review, &migration_flow);
        let rollback_checkpoint =
            rollback_checkpoint_record(&input.rollback_checkpoint, &migration_flow);
        let retained_diagnostics = input
            .retained_diagnostics
            .iter()
            .map(|diag| retained_diagnostic_record(diag, &migration_flow, &input.generated_at))
            .collect::<Vec<_>>();
        let commands = input
            .commands
            .iter()
            .map(|command| migration_command_record(command, &migration_flow))
            .collect::<Vec<_>>();
        let support_export = migration_support_export_packet(
            &input.support_export,
            &migration_flow,
            workspace_packet,
            &commands,
            &diff_review,
            &rollback_checkpoint,
        );
        let inspection = migration_inspection_record(
            &migration_flow,
            &diff_review,
            &rollback_checkpoint,
            &retained_diagnostics,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: MIGRATION_ROLLBACK_DIFF_REVIEW_PACKET_RECORD_KIND.to_string(),
            schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            migration_flow,
            diff_review,
            rollback_checkpoint,
            retained_diagnostics,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the migration rollback diff-review invariants.
    ///
    /// # Errors
    ///
    /// Returns [`MigrationRollbackDiffReviewValidationError`] when an invariant
    /// is violated.
    pub fn validate(&self) -> Result<(), MigrationRollbackDiffReviewValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            MIGRATION_ROLLBACK_DIFF_REVIEW_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_migration_flow_record(&self.migration_flow, &self.review_workspace.review_workspace_id)?;
        validate_diff_review_record(&self.diff_review, &self.migration_flow.migration_flow_id)?;
        validate_rollback_checkpoint_record(
            &self.rollback_checkpoint,
            self.migration_flow.migration_flow_id.as_str(),
        )?;
        for diagnostic in &self.retained_diagnostics {
            validate_retained_diagnostic_record(diagnostic, &self.migration_flow.migration_flow_id)?;
        }
        for command in &self.commands {
            validate_migration_command_record(command, &self.migration_flow.migration_flow_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.migration_flow,
            &self.commands,
            &self.diff_review,
            &self.rollback_checkpoint,
        )?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        if self.diff_review.diff_review_state == "approved_with_checkpoints"
            && self.diff_review.checkpoint_required_before_apply
            && !matches!(
                self.rollback_checkpoint.checkpoint_state.as_str(),
                "captured_ready" | "captured_pending" | "restored"
            )
        {
            return Err(migration_validation_error(
                "diff approved_with_checkpoints requires a captured or restored checkpoint",
            ));
        }
        if matches!(
            self.migration_flow.flow_state.as_str(),
            "applying" | "applied" | "validation_failed" | "rolled_back"
        ) && self.diff_review.diff_review_state != "approved_with_checkpoints"
        {
            return Err(migration_validation_error(
                "flow_state applying/applied/validation_failed/rolled_back requires diff_review_state approved_with_checkpoints",
            ));
        }
        if self.migration_flow.flow_state == "applied" && self.inspection.validation_failed {
            return Err(migration_validation_error(
                "applied flow cannot be marked validation_failed",
            ));
        }
        if self.migration_flow.flow_state == "rolled_back"
            && self.rollback_checkpoint.checkpoint_state == "missing_blocks_apply"
        {
            return Err(migration_validation_error(
                "rolled_back flow cannot have missing_blocks_apply checkpoint",
            ));
        }
        if self.migration_flow.flow_state == "validation_failed" && self.retained_diagnostics.is_empty() {
            return Err(migration_validation_error(
                "validation_failed flow must retain at least one diagnostic",
            ));
        }
        Ok(())
    }

    /// Returns true when migration truth axes are surfaced as separable
    /// inspectable truths.
    pub fn truths_are_separable(&self) -> bool {
        let flow = &self.migration_flow;
        contains_token(MIGRATION_OPERATION_KINDS, &flow.operation_kind)
            && contains_token(MIGRATION_FLOW_STATES, &flow.flow_state)
            && contains_token(MIGRATION_DIFF_REVIEW_STATES, &self.diff_review.diff_review_state)
            && contains_token(
                MIGRATION_CHECKPOINT_STATES,
                &self.rollback_checkpoint.checkpoint_state,
            )
    }

    /// Returns true when no raw escape hatch crosses the support boundary.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        !self.support_export.raw_url_export_allowed
            && !self.support_export.raw_provider_payload_export_allowed
            && !self.support_export.raw_path_export_allowed
            && !self.support_export.raw_branch_name_export_allowed
            && !self.support_export.raw_patch_body_export_allowed
    }

    /// Returns true when the flow can be reopened after restart from the
    /// support export.
    pub fn restartable_from_support_export(&self) -> bool {
        self.inspection.restartable && self.inspection.support_export_reopenable
    }
}

// ---------------------------------------------------------------------------
// Projection type
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationRollbackDiffReviewProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Migration flow identity.
    pub migration_flow_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Flow state.
    pub flow_state: String,
    /// Diff review state.
    pub diff_review_state: String,
    /// Checkpoint state.
    pub checkpoint_state: String,
    /// True when the diff is approved.
    pub diff_approved: bool,
    /// True when a checkpoint is ready.
    pub checkpoint_ready: bool,
    /// True when the flow is actionable.
    pub actionable: bool,
    /// True when the flow is restartable.
    pub restartable: bool,
    /// True when approval is invalidated.
    pub approval_invalidated: bool,
    /// Active invalidation reasons.
    pub invalidation_reasons: Vec<String>,
    /// Active blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Command count.
    pub command_count: usize,
    /// True when at least one command supports preview.
    pub preview_capable: bool,
    /// True when support/export can reopen the flow context.
    pub support_export_reopenable: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
    /// Number of retained diagnostics.
    pub retained_diagnostic_count: usize,
}

/// Parses and validates a materialized migration rollback diff-review packet.
///
/// # Errors
///
/// Returns [`MigrationRollbackDiffReviewError`] when the payload fails to parse
/// or violates the migration flow invariants.
pub fn project_migration_rollback_diff_review_packet(
    payload: &str,
) -> Result<MigrationRollbackDiffReviewProjection, MigrationRollbackDiffReviewError> {
    let packet: MigrationRollbackDiffReviewPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(MigrationRollbackDiffReviewProjection::from(packet))
}

impl From<MigrationRollbackDiffReviewPacket> for MigrationRollbackDiffReviewProjection {
    fn from(packet: MigrationRollbackDiffReviewPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            migration_flow_id: packet.migration_flow.migration_flow_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            operation_kind: packet.migration_flow.operation_kind,
            flow_state: packet.migration_flow.flow_state,
            diff_review_state: packet.diff_review.diff_review_state,
            checkpoint_state: packet.rollback_checkpoint.checkpoint_state,
            diff_approved: packet.inspection.diff_approved,
            checkpoint_ready: packet.inspection.checkpoint_ready,
            actionable: packet.inspection.actionable,
            restartable: packet.inspection.restartable,
            approval_invalidated: packet.inspection.approval_invalidated,
            invalidation_reasons: packet.migration_flow.invalidation_reasons,
            blocked_reasons: packet.migration_flow.blocked_reasons,
            command_count: packet.commands.len(),
            preview_capable: packet.inspection.preview_capable,
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
            retained_diagnostic_count: packet.inspection.retained_diagnostic_count,
        }
    }
}


// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error returned when a migration rollback diff-review payload cannot be projected.
#[derive(Debug)]
pub enum MigrationRollbackDiffReviewError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the migration flow invariants.
    Validation(MigrationRollbackDiffReviewValidationError),
}

impl fmt::Display for MigrationRollbackDiffReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "migration rollback diff-review parse error: {err}"),
            Self::Validation(err) => write!(formatter, "migration rollback diff-review validation error: {err}"),
        }
    }
}

impl std::error::Error for MigrationRollbackDiffReviewError {}

impl From<serde_json::Error> for MigrationRollbackDiffReviewError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<MigrationRollbackDiffReviewValidationError> for MigrationRollbackDiffReviewError {
    fn from(err: MigrationRollbackDiffReviewValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for migration rollback diff-review packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationRollbackDiffReviewValidationError {
    message: String,
}

impl MigrationRollbackDiffReviewValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for MigrationRollbackDiffReviewValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for MigrationRollbackDiffReviewValidationError {}

fn migration_validation_error(message: impl Into<String>) -> MigrationRollbackDiffReviewValidationError {
    MigrationRollbackDiffReviewValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn ensure_eq<T: PartialEq + std::fmt::Debug>(
    actual: T,
    expected: T,
    field: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    if actual != expected {
        return Err(migration_validation_error(format!(
            "{field} mismatch: expected {expected:?}, got {actual:?}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    if value.trim().is_empty() {
        return Err(migration_validation_error(format!("{field} must be non-empty")));
    }
    Ok(())
}

fn contains_token(haystack: &[&str], needle: &str) -> bool {
    haystack.iter().any(|token| *token == needle)
}

fn validate_input(
    input: &MigrationRollbackDiffReviewInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_nonempty(&input.migration_flow_id, "migration_flow_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    if !contains_token(MIGRATION_OPERATION_KINDS, &input.operation_kind) {
        return Err(migration_validation_error(format!(
            "unsupported operation_kind {}", input.operation_kind
        )));
    }
    if !contains_token(MIGRATION_FLOW_STATES, &input.flow_state) {
        return Err(migration_validation_error(format!(
            "unsupported flow_state {}", input.flow_state
        )));
    }
    if !contains_token(MIGRATION_DIFF_REVIEW_STATES, &input.diff_review.diff_review_state) {
        return Err(migration_validation_error(format!(
            "unsupported diff_review_state {}", input.diff_review.diff_review_state
        )));
    }
    if !contains_token(MIGRATION_CHECKPOINT_STATES, &input.rollback_checkpoint.checkpoint_state) {
        return Err(migration_validation_error(format!(
            "unsupported checkpoint_state {}", input.rollback_checkpoint.checkpoint_state
        )));
    }
    for reason in &input.invalidation_reasons {
        if !contains_token(MIGRATION_INVALIDATION_REASONS, reason) {
            return Err(migration_validation_error(format!(
                "unsupported invalidation_reason {reason}"
            )));
        }
    }
    for diag in &input.retained_diagnostics {
        if !contains_token(MIGRATION_DIAGNOSTIC_REASON_CLASSES, &diag.reason_class) {
            return Err(migration_validation_error(format!(
                "unsupported diagnostic reason_class {}", diag.reason_class
            )));
        }
        if !contains_token(MIGRATION_DIAGNOSTIC_ACTION_CLASSES, &diag.suggested_action) {
            return Err(migration_validation_error(format!(
                "unsupported diagnostic suggested_action {}", diag.suggested_action
            )));
        }
    }
    for command in &input.commands {
        if !contains_token(MIGRATION_COMMAND_CLASSES, &command.command_class) {
            return Err(migration_validation_error(format!(
                "unsupported command_class {}", command.command_class
            )));
        }
    }
    if workspace_packet.review_workspace.review_workspace_id.trim().is_empty() {
        return Err(migration_validation_error("workspace_packet review_workspace_id must be non-empty"));
    }
    Ok(())
}

fn validate_migration_flow_record(
    record: &MigrationRollbackDiffReviewRecord,
    expected_workspace_id: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        MIGRATION_ROLLBACK_DIFF_REVIEW_RECORD_KIND,
        "migration_flow.record_kind",
    )?;
    ensure_eq(record.schema_version, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION, "migration_flow.schema_version")?;
    ensure_nonempty(&record.migration_flow_id, "migration_flow.migration_flow_id")?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        expected_workspace_id,
        "migration_flow.review_workspace_id_ref",
    )?;
    ensure_nonempty(&record.operation_kind, "migration_flow.operation_kind")?;
    ensure_nonempty(&record.flow_state, "migration_flow.flow_state")?;
    ensure_nonempty(&record.source_editor, "migration_flow.source_editor")?;
    ensure_nonempty(&record.migration_session_ref, "migration_flow.migration_session_ref")?;
    Ok(())
}

fn validate_diff_review_record(
    record: &MigrationDiffReviewRecord,
    expected_flow_id: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(record.record_kind.as_str(), MIGRATION_DIFF_REVIEW_RECORD_KIND, "diff_review.record_kind")?;
    ensure_eq(record.schema_version, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION, "diff_review.schema_version")?;
    ensure_eq(
        record.migration_flow_id_ref.as_str(),
        expected_flow_id,
        "diff_review.migration_flow_id_ref",
    )?;
    ensure_nonempty(&record.diff_review_state, "diff_review.diff_review_state")?;
    Ok(())
}

fn validate_rollback_checkpoint_record(
    record: &MigrationRollbackCheckpointRecord,
    expected_flow_id: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        MIGRATION_ROLLBACK_CHECKPOINT_RECORD_KIND,
        "rollback_checkpoint.record_kind",
    )?;
    ensure_eq(record.schema_version, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION, "rollback_checkpoint.schema_version")?;
    ensure_eq(
        record.migration_flow_id_ref.as_str(),
        expected_flow_id,
        "rollback_checkpoint.migration_flow_id_ref",
    )?;
    ensure_nonempty(&record.checkpoint_state, "rollback_checkpoint.checkpoint_state")?;
    Ok(())
}

fn validate_retained_diagnostic_record(
    record: &RetainedDiagnosticRecord,
    expected_flow_id: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(record.record_kind.as_str(), RETAINED_DIAGNOSTIC_RECORD_KIND, "retained_diagnostic.record_kind")?;
    ensure_eq(record.schema_version, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION, "retained_diagnostic.schema_version")?;
    ensure_eq(
        record.migration_flow_id_ref.as_str(),
        expected_flow_id,
        "retained_diagnostic.migration_flow_id_ref",
    )?;
    ensure_nonempty(&record.diagnostic_id, "retained_diagnostic.diagnostic_id")?;
    ensure_nonempty(&record.reason_class, "retained_diagnostic.reason_class")?;
    ensure_nonempty(&record.message, "retained_diagnostic.message")?;
    Ok(())
}

fn validate_migration_command_record(
    record: &MigrationCommandRecord,
    expected_flow_id: &str,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(record.record_kind.as_str(), MIGRATION_COMMAND_RECORD_KIND, "command.record_kind")?;
    ensure_eq(record.schema_version, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION, "command.schema_version")?;
    ensure_eq(
        record.migration_flow_id_ref.as_str(),
        expected_flow_id,
        "command.migration_flow_id_ref",
    )?;
    ensure_nonempty(&record.command_id, "command.command_id")?;
    ensure_nonempty(&record.command_class, "command.command_class")?;
    Ok(())
}

fn validate_support_export(
    export: &MigrationSupportExportPacket,
    flow: &MigrationRollbackDiffReviewRecord,
    commands: &[MigrationCommandRecord],
    diff_review: &MigrationDiffReviewRecord,
    checkpoint: &MigrationRollbackCheckpointRecord,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        MIGRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq(export.schema_version, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION, "support_export.schema_version")?;
    ensure_eq(
        export.migration_flow_id_ref.as_str(),
        flow.migration_flow_id.as_str(),
        "support_export.migration_flow_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        flow.review_workspace_id_ref.as_str(),
        "support_export.review_workspace_id_ref",
    )?;
    ensure_nonempty(&export.support_export_id, "support_export.support_export_id")?;
    ensure_nonempty(&export.reopen_context_ref, "support_export.reopen_context_ref")?;
    ensure_nonempty(&export.reopen_command_id_ref, "support_export.reopen_command_id_ref")?;
    if export.consumer_surfaces.is_empty() {
        return Err(migration_validation_error(
            "support_export.consumer_surfaces must not be empty",
        ));
    }
    for surface in &export.consumer_surfaces {
        if !contains_token(MIGRATION_CONSUMER_SURFACES, surface) {
            return Err(migration_validation_error(format!(
                "unsupported consumer_surface {surface}"
            )));
        }
    }
    let command_ids: BTreeSet<_> = commands.iter().map(|c| c.command_id.as_str()).collect();
    for cmd_id in &export.command_id_refs {
        if !command_ids.contains(cmd_id.as_str()) {
            return Err(migration_validation_error(format!(
                "support_export references unknown command_id {cmd_id}"
            )));
        }
    }
    // Restart snapshot consistency
    if export.restart_snapshot.flow_state != flow.flow_state {
        return Err(migration_validation_error(
            "support_export restart_snapshot.flow_state must match migration_flow.flow_state",
        ));
    }
    if export.restart_snapshot.diff_review_state != diff_review.diff_review_state {
        return Err(migration_validation_error(
            "support_export restart_snapshot.diff_review_state must match diff_review.diff_review_state",
        ));
    }
    if export.restart_snapshot.checkpoint_state != checkpoint.checkpoint_state {
        return Err(migration_validation_error(
            "support_export restart_snapshot.checkpoint_state must match rollback_checkpoint.checkpoint_state",
        ));
    }
    if export.restart_snapshot.operation_kind != flow.operation_kind {
        return Err(migration_validation_error(
            "support_export restart_snapshot.operation_kind must match migration_flow.operation_kind",
        ));
    }
    if export.restart_snapshot.restart_session_ref != flow.restart_session_ref {
        return Err(migration_validation_error(
            "support_export restart_snapshot.restart_session_ref must match migration_flow.restart_session_ref",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &MigrationInspectionRecord,
    packet: &MigrationRollbackDiffReviewPacket,
) -> Result<(), MigrationRollbackDiffReviewValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        MIGRATION_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq(
        inspection.schema_version,
        MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        "inspection.schema_version",
    )?;
    ensure_eq(
        inspection.migration_flow_id_ref.as_str(),
        packet.migration_flow.migration_flow_id.as_str(),
        "inspection.migration_flow_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "inspection.review_workspace_id_ref",
    )?;
    if inspection.command_count != packet.commands.len() {
        return Err(migration_validation_error(
            "inspection.command_count must match number of commands",
        ));
    }
    if inspection.retained_diagnostic_count != packet.retained_diagnostics.len() {
        return Err(migration_validation_error(
            "inspection.retained_diagnostic_count must match number of retained_diagnostics",
        ));
    }
    if inspection.diff_approved != (packet.diff_review.diff_review_state == "approved_with_checkpoints") {
        return Err(migration_validation_error(
            "inspection.diff_approved must match diff_review_state",
        ));
    }
    if inspection.diff_pending != (packet.diff_review.diff_review_state == "pending") {
        return Err(migration_validation_error(
            "inspection.diff_pending must match diff_review_state pending",
        ));
    }
    if inspection.diff_rejected != (packet.diff_review.diff_review_state == "rejected") {
        return Err(migration_validation_error(
            "inspection.diff_rejected must match diff_review_state rejected",
        ));
    }
    if inspection.checkpoint_ready != matches!(packet.rollback_checkpoint.checkpoint_state.as_str(), "captured_ready" | "restored") {
        return Err(migration_validation_error(
            "inspection.checkpoint_ready must match captured_ready or restored checkpoint_state",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Record builders
// ---------------------------------------------------------------------------

fn migration_flow_record(
    input: &MigrationRollbackDiffReviewInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> MigrationRollbackDiffReviewRecord {
    let mut blocked_reasons = Vec::new();
    if !contains_token(MIGRATION_DIFF_REVIEW_STATES, &input.diff_review.diff_review_state) {
        blocked_reasons.push("diff_review_state_invalid".to_string());
    }
    if input.diff_review.diff_review_state != "approved_with_checkpoints" {
        blocked_reasons.push("diff_not_approved".to_string());
    }
    if input.rollback_checkpoint.checkpoint_state == "missing_blocks_apply" {
        blocked_reasons.push("checkpoint_missing".to_string());
    }
    if !input.invalidation_reasons.is_empty() {
        blocked_reasons.push("invalidated".to_string());
    }
    let actionable = blocked_reasons.is_empty()
        && input.flow_state != "aborted"
        && input.flow_state != "rolled_back"
        && input.flow_state != "validation_failed";

    MigrationRollbackDiffReviewRecord {
        record_kind: MIGRATION_ROLLBACK_DIFF_REVIEW_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        migration_flow_id: input.migration_flow_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        operation_kind: input.operation_kind.clone(),
        flow_state: input.flow_state.clone(),
        source_editor: input.source_editor.clone(),
        migration_session_ref: input.migration_session_ref.clone(),
        target_family_refs: input.target_family_refs.clone(),
        review_pack_digest_ref: input.review_pack_digest_ref.clone(),
        environment_capsule_digest_ref: input.environment_capsule_digest_ref.clone(),
        restart_session_ref: input.restart_session_ref.clone(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        blocked_reasons,
        actionable,
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn diff_review_record(
    input: &MigrationDiffReviewInput,
    flow: &MigrationRollbackDiffReviewRecord,
) -> MigrationDiffReviewRecord {
    MigrationDiffReviewRecord {
        record_kind: MIGRATION_DIFF_REVIEW_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        migration_flow_id_ref: flow.migration_flow_id.clone(),
        diff_review_state: input.diff_review_state.clone(),
        diff_preview_ref: input.diff_preview_ref.clone(),
        suspicious_content_flagged: input.suspicious_content_flagged,
        suspicious_content_reviewed: input.suspicious_content_reviewed,
        checkpoint_required_before_apply: input.checkpoint_required_before_apply,
        manual_review_reason: input.manual_review_reason.clone(),
        summary_label: "Diff review gate for migration flow".to_string(),
    }
}

fn rollback_checkpoint_record(
    input: &MigrationRollbackCheckpointInput,
    flow: &MigrationRollbackDiffReviewRecord,
) -> MigrationRollbackCheckpointRecord {
    MigrationRollbackCheckpointRecord {
        record_kind: MIGRATION_ROLLBACK_CHECKPOINT_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        migration_flow_id_ref: flow.migration_flow_id.clone(),
        checkpoint_state: input.checkpoint_state.clone(),
        checkpoint_ref: input.checkpoint_ref.clone(),
        restore_command_id: input.restore_command_id.clone(),
        disclosure_label: input.disclosure_label.clone(),
        restorable_offline: input.restorable_offline,
        summary_label: "Rollback checkpoint for migration flow".to_string(),
    }
}

fn retained_diagnostic_record(
    input: &RetainedDiagnosticInput,
    flow: &MigrationRollbackDiffReviewRecord,
    generated_at: &str,
) -> RetainedDiagnosticRecord {
    RetainedDiagnosticRecord {
        record_kind: RETAINED_DIAGNOSTIC_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        migration_flow_id_ref: flow.migration_flow_id.clone(),
        diagnostic_id: input.diagnostic_id.clone(),
        source_editor: input.source_editor.clone(),
        import_target_family: input.import_target_family.clone(),
        source_item_id: input.source_item_id.clone(),
        reason_class: input.reason_class.clone(),
        message: input.message.clone(),
        suggested_action: input.suggested_action.clone(),
        fallback_available: input.fallback_available,
        retained_at: generated_at.to_string(),
    }
}

fn migration_command_record(
    input: &MigrationCommandInput,
    flow: &MigrationRollbackDiffReviewRecord,
) -> MigrationCommandRecord {
    let actionable = input.blocked_reasons.is_empty()
        && flow.actionable
        && input.command_class != "abort_flow";
    MigrationCommandRecord {
        record_kind: MIGRATION_COMMAND_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        migration_flow_id_ref: flow.migration_flow_id.clone(),
        command_class: input.command_class.clone(),
        target_object_ref: input.target_object_ref.clone(),
        target_object_kind: input.target_object_kind.clone(),
        preview_supported: input.preview_supported,
        emits_audit_event: input.emits_audit_event,
        blocked_reasons: input.blocked_reasons.clone(),
        actionable,
        summary_label: input.summary_label.clone(),
    }
}

fn migration_support_export_packet(
    input: &MigrationSupportExportInput,
    flow: &MigrationRollbackDiffReviewRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    commands: &[MigrationCommandRecord],
    diff_review: &MigrationDiffReviewRecord,
    checkpoint: &MigrationRollbackCheckpointRecord,
) -> MigrationSupportExportPacket {
    MigrationSupportExportPacket {
        record_kind: MIGRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        migration_flow_id_ref: flow.migration_flow_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/migration_rollback_diff_review.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        raw_path_export_allowed: false,
        raw_branch_name_export_allowed: false,
        raw_patch_body_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        restart_snapshot: MigrationRestartSnapshot {
            flow_state: flow.flow_state.clone(),
            diff_review_state: diff_review.diff_review_state.clone(),
            checkpoint_state: checkpoint.checkpoint_state.clone(),
            operation_kind: flow.operation_kind.clone(),
            restart_session_ref: flow.restart_session_ref.clone(),
            invalidation_reasons: flow.invalidation_reasons.clone(),
            blocked_reasons: flow.blocked_reasons.clone(),
        },
        summary_label: input.summary_label.clone(),
    }
}

fn migration_inspection_record(
    flow: &MigrationRollbackDiffReviewRecord,
    diff_review: &MigrationDiffReviewRecord,
    checkpoint: &MigrationRollbackCheckpointRecord,
    diagnostics: &[RetainedDiagnosticRecord],
    commands: &[MigrationCommandRecord],
    support_export: &MigrationSupportExportPacket,
) -> MigrationInspectionRecord {
    let diff_approved = diff_review.diff_review_state == "approved_with_checkpoints";
    let diff_pending = diff_review.diff_review_state == "pending";
    let diff_rejected = diff_review.diff_review_state == "rejected";
    let checkpoint_ready = matches!(checkpoint.checkpoint_state.as_str(), "captured_ready" | "restored");
    let applying = flow.flow_state == "applying";
    let completed = flow.flow_state == "applied";
    let validation_failed = flow.flow_state == "validation_failed";
    let rolled_back = flow.flow_state == "rolled_back";
    let aborted = flow.flow_state == "aborted";
    let approval_invalidated = flow.invalidation_reasons.iter().any(|r| r == "approval_invalidated");
    let checks_stale_blocks_apply = flow.invalidation_reasons.iter().any(|r| r == "checks_stale");
    let restartable = !flow.restart_session_ref.trim().is_empty()
        && !aborted
        && !support_export.reopen_context_ref.trim().is_empty();
    let preview_capable = commands.iter().any(|c| c.preview_supported);
    let support_export_reopenable = !support_export.reopen_context_ref.trim().is_empty()
        && !support_export.reopen_command_id_ref.trim().is_empty();
    let suspicious_content_unreviewed =
        diff_review.suspicious_content_flagged && !diff_review.suspicious_content_reviewed;
    let retained_diagnostic_fallback_available = diagnostics.iter().any(|d| d.fallback_available);

    MigrationInspectionRecord {
        record_kind: MIGRATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
        migration_flow_id_ref: flow.migration_flow_id.clone(),
        review_workspace_id_ref: flow.review_workspace_id_ref.clone(),
        diff_approved,
        diff_pending,
        diff_rejected,
        checkpoint_ready,
        applying,
        completed,
        validation_failed,
        rolled_back,
        aborted,
        approval_invalidated,
        checks_stale_blocks_apply,
        actionable: flow.actionable,
        restartable,
        command_count: commands.len(),
        preview_capable,
        support_export_reopenable,
        suspicious_content_unreviewed,
        retained_diagnostic_count: diagnostics.len(),
        retained_diagnostic_fallback_available,
        summary_label: "Migration rollback diff-review inspection".to_string(),
    }
}
