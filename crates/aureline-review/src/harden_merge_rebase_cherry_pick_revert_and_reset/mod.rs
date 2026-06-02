//! Diff-first review and recovery-checkpoint contracts for merge, rebase,
//! cherry-pick, revert, and reset flows.
//!
//! This module hardens the daily-driver Git rewrite lane by requiring a diff
//! review gate and an explicit recovery checkpoint before any destructive ref
//! update is applied. It consumes the [`aureline_git::history_rewrite`] beta
//! record family and projects it into review surfaces as a
//! [`DiffFirstRewriteFlowPacket`] that carries:
//!
//! - [`RewriteFlowRecord`] — stable identity, operation provenance, divergence
//!   summary, protected-branch posture, and restart-resilient session truth.
//! - [`DiffFirstReviewRecord`] — the review gate that blocks apply until the
//!   diff is approved and checkpoints are captured.
//! - [`SequenceEditProposalRecord`] — durable ordered operations for rebase,
//!   interactive rebase, and cherry-pick sequences with protected-branch blocks
//!   and invalidation state.
//! - [`RecoveryCheckpointSummaryRecord`] — summary of the checkpoint captured
//!   before the destructive step, or the reflog-only disclosure acknowledged.
//! - [`RewriteFlowCommandRecord`] — command-graph operations surfaced to the
//!   inspector (preview diff, approve, capture checkpoint, apply, abort, etc.).
//! - [`RewriteFlowSupportExportPacket`] — redaction-safe support export that
//!   can reopen the same structured session truth after restart.
//! - [`RewriteFlowInspectionRecord`] — compact boolean projection for CLI and
//!   inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/diff_first_rewrite_flow.schema.json`. Canonical fixtures
//! live under `fixtures/review/m4/harden_merge_rebase_cherry_pick_revert_and_reset/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every diff-first rewrite-flow record.
pub const DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`DiffFirstRewriteFlowPacket`].
pub const DIFF_FIRST_REWRITE_FLOW_PACKET_RECORD_KIND: &str =
    "review_diff_first_rewrite_flow_packet";

/// Stable record-kind tag for [`RewriteFlowRecord`].
pub const REWRITE_FLOW_RECORD_KIND: &str = "review_rewrite_flow_record";

/// Stable record-kind tag for [`DiffFirstReviewRecord`].
pub const DIFF_FIRST_REVIEW_RECORD_KIND: &str = "review_diff_first_review_record";

/// Stable record-kind tag for [`SequenceEditProposalRecord`].
pub const SEQUENCE_EDIT_PROPOSAL_RECORD_KIND: &str = "review_sequence_edit_proposal_record";

/// Stable record-kind tag for [`RecoveryCheckpointSummaryRecord`].
pub const RECOVERY_CHECKPOINT_SUMMARY_RECORD_KIND: &str =
    "review_recovery_checkpoint_summary_record";

/// Stable record-kind tag for [`RewriteFlowCommandRecord`].
pub const REWRITE_FLOW_COMMAND_RECORD_KIND: &str = "review_rewrite_flow_command_record";

/// Stable record-kind tag for [`RewriteFlowSupportExportPacket`].
pub const REWRITE_FLOW_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_rewrite_flow_support_export_packet";

/// Stable record-kind tag for [`RewriteFlowInspectionRecord`].
pub const REWRITE_FLOW_INSPECTION_RECORD_KIND: &str = "review_rewrite_flow_inspection_record";

/// Closed set of rewrite-flow operation kinds.
pub const REWRITE_FLOW_OPERATION_KINDS: &[&str] = &[
    "merge",
    "rebase",
    "interactive_rebase",
    "cherry_pick",
    "revert",
    "reset",
];

/// Closed set of rewrite-flow lifecycle states.
pub const REWRITE_FLOW_STATES: &[&str] = &[
    "diff_pending_review",
    "diff_review_approved",
    "diff_review_rejected",
    "checkpoint_pending",
    "checkpoint_captured",
    "executing",
    "paused_conflict",
    "completed",
    "aborted_rolled_back",
    "failed_no_changes_made",
];

/// Closed set of diff-first review states.
pub const DIFF_FIRST_REVIEW_STATES: &[&str] = &[
    "pending",
    "approved_with_checkpoints",
    "rejected",
    "requires_manual_review",
];

/// Closed set of checkpoint-summary states.
pub const CHECKPOINT_SUMMARY_STATES: &[&str] = &[
    "none_required",
    "captured_ready",
    "captured_pending",
    "restored",
    "expired",
    "reflog_only_acknowledged",
    "missing_blocks_apply",
];

/// Closed set of local/remote divergence classes.
pub const DIVERGENCE_CLASSES: &[&str] = &[
    "no_divergence",
    "local_ahead",
    "remote_ahead",
    "diverged_requires_rebase",
    "diverged_requires_merge",
];

/// Closed set of protected-branch postures.
pub const PROTECTED_BRANCH_POSTURES: &[&str] = &[
    "no_protected_refs",
    "protected_branch_readonly",
    "protected_branch_blocked",
    "policy_lock_active",
];

/// Closed set of approval states surfaced on rewrite flows.
pub const REWRITE_FLOW_APPROVAL_STATES: &[&str] = &[
    "approval_not_required_by_policy",
    "approval_required_outstanding",
    "approved_current",
    "approval_invalidated_by_changes",
];

/// Closed set of checks-freshness states.
pub const REWRITE_FLOW_CHECKS_FRESHNESS_STATES: &[&str] = &[
    "checks_current",
    "checks_stale_within_grace",
    "checks_stale_blocks_apply",
];

/// Closed set of command classes for the rewrite-flow lane.
pub const REWRITE_FLOW_COMMAND_CLASSES: &[&str] = &[
    "preview_diff",
    "approve_diff",
    "reject_diff",
    "capture_checkpoint",
    "restore_checkpoint",
    "apply_proposal",
    "abort_flow",
    "continue_after_resolve",
    "skip_conflicted_step",
    "open_sequence_editor",
    "request_external_handoff",
];

/// Closed set of consumer surfaces for rewrite-flow packets.
pub const REWRITE_FLOW_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "desktop_conflict_resolver",
    "desktop_sequence_editor",
];

/// Closed set of invalidation reasons that mark a rewrite flow stale.
pub const REWRITE_FLOW_INVALIDATION_REASONS: &[&str] = &[
    "stale_base",
    "checks_stale",
    "approval_invalidated",
    "policy_blocked",
    "worktree_scope_changed",
    "environment_capsule_changed",
    "provider_overlay_stale",
    "sequence_edit_modified",
    "conflict_session_aborted",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a rewrite flow to materialize on top of a review-workspace
/// packet and an optional history-rewrite record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowInput {
    /// Stable rewrite-flow identity.
    pub rewrite_flow_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Operation kind from the closed vocabulary.
    pub operation_kind: String,
    /// Current flow lifecycle state.
    pub flow_state: String,
    /// Source ref the operation replays from.
    pub source_ref: String,
    /// Target ref the operation replays onto.
    pub target_ref: String,
    /// Base ref used for divergence calculation.
    pub base_ref: String,
    /// Worktree-identity ref backing the flow.
    pub worktree_identity_ref: String,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Protected-branch posture observed at flow start.
    pub protected_branch_posture: String,
    /// Local/remote divergence class.
    pub divergence_class: String,
    /// Approval state observed at flow start.
    pub approval_state: String,
    /// Checks-freshness state observed at flow start.
    pub checks_freshness_state: String,
    /// Stable session ref used to reopen this flow after restart.
    pub restart_session_ref: String,
    /// Diff-first review input.
    pub diff_review: DiffFirstReviewInput,
    /// Optional sequence-edit proposal input (required for interactive rebase
    /// and cherry-pick sequences).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_edit_proposal: Option<SequenceEditProposalInput>,
    /// Recovery checkpoint summary input.
    pub recovery_checkpoint_summary: RecoveryCheckpointSummaryInput,
    /// Command-graph operations defined for this flow.
    pub commands: Vec<RewriteFlowCommandInput>,
    /// Support/export envelope input.
    pub support_export: RewriteFlowSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing the diff-first review gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffFirstReviewInput {
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

/// Input describing a durable sequence-edit proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditProposalInput {
    /// Stable proposal identity.
    pub proposal_id: String,
    /// Ordered operations in the sequence.
    pub ordered_operations: Vec<SequenceEditOperationInput>,
    /// True when the proposal is blocked by a protected-branch policy.
    pub protected_branch_blocked: bool,
    /// Invalidation state for the proposal.
    pub invalidation_state: String,
}

/// One ordered operation inside a sequence-edit proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditOperationInput {
    /// Zero-based ordinal.
    pub ordinal: u32,
    /// Verb class (pick, reword, edit, squash, fixup, drop, exec, etc.).
    pub verb: String,
    /// Opaque target ref (commit or label).
    pub target_ref: String,
    /// Redaction-safe display label.
    pub display_label: String,
    /// True when this step is currently selected.
    pub current_step: bool,
    /// True when this step has already executed.
    pub completed: bool,
    /// Optional conflict-session ref if this step paused for conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_session_ref: Option<String>,
}

/// Input describing the recovery-checkpoint summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCheckpointSummaryInput {
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

/// Input describing one command-graph operation for a rewrite flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowCommandInput {
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

/// Input row for the rewrite-flow support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowSupportExportInput {
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

/// Rewrite-flow record materialized from input plus workspace truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable rewrite-flow identity.
    pub rewrite_flow_id: String,
    /// Review workspace this flow belongs to.
    pub review_workspace_id_ref: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Current flow lifecycle state.
    pub flow_state: String,
    /// Source ref.
    pub source_ref: String,
    /// Target ref.
    pub target_ref: String,
    /// Base ref.
    pub base_ref: String,
    /// Worktree-identity ref.
    pub worktree_identity_ref: String,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Protected-branch posture.
    pub protected_branch_posture: String,
    /// Local/remote divergence class.
    pub divergence_class: String,
    /// Approval state.
    pub approval_state: String,
    /// Checks-freshness state.
    pub checks_freshness_state: String,
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

/// Diff-first review record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffFirstReviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Rewrite flow this review gate belongs to.
    pub rewrite_flow_id_ref: String,
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

/// Sequence-edit proposal record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditProposalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Rewrite flow this proposal belongs to.
    pub rewrite_flow_id_ref: String,
    /// Stable proposal identity.
    pub proposal_id: String,
    /// Ordered operations.
    pub ordered_operations: Vec<SequenceEditOperationRecord>,
    /// True when the proposal is blocked by a protected-branch policy.
    pub protected_branch_blocked: bool,
    /// Invalidation state.
    pub invalidation_state: String,
    /// Number of remaining (not completed) steps.
    pub remaining_step_count: u32,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// One ordered operation inside a sequence-edit proposal record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditOperationRecord {
    /// Zero-based ordinal.
    pub ordinal: u32,
    /// Verb class.
    pub verb: String,
    /// Opaque target ref.
    pub target_ref: String,
    /// Redaction-safe display label.
    pub display_label: String,
    /// True when this step is currently selected.
    pub current_step: bool,
    /// True when this step has already executed.
    pub completed: bool,
    /// Optional conflict-session ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_session_ref: Option<String>,
}

/// Recovery-checkpoint summary record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCheckpointSummaryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Rewrite flow this summary belongs to.
    pub rewrite_flow_id_ref: String,
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

/// Command-graph operation record for a rewrite flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Rewrite flow this command belongs to.
    pub rewrite_flow_id_ref: String,
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
pub struct RewriteFlowInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Rewrite flow inspected by this row.
    pub rewrite_flow_id_ref: String,
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
    /// True when the flow is currently executing.
    pub executing: bool,
    /// True when the flow is paused for conflict resolution.
    pub paused_conflict: bool,
    /// True when the flow completed successfully.
    pub completed: bool,
    /// True when the flow was aborted and rolled back.
    pub aborted: bool,
    /// True when a protected-branch block is active.
    pub protected_branch_blocked: bool,
    /// True when policy blocks the flow.
    pub policy_blocks_apply: bool,
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
    /// Number of remaining sequence-edit steps.
    pub remaining_step_count: u32,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the rewrite-flow lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Rewrite flow this packet exports.
    pub rewrite_flow_id_ref: String,
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
    pub restart_snapshot: RewriteFlowRestartSnapshot,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Restart snapshot embedded in the rewrite-flow support-export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteFlowRestartSnapshot {
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

/// Diff-first rewrite-flow packet consumed by review surfaces and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffFirstRewriteFlowPacket {
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
    /// Rewrite flow record.
    pub rewrite_flow: RewriteFlowRecord,
    /// Diff-first review record.
    pub diff_first_review: DiffFirstReviewRecord,
    /// Optional sequence-edit proposal record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_edit_proposal: Option<SequenceEditProposalRecord>,
    /// Recovery checkpoint summary record.
    pub recovery_checkpoint_summary: RecoveryCheckpointSummaryRecord,
    /// Command-graph operation records.
    pub commands: Vec<RewriteFlowCommandRecord>,
    /// Support/export packet.
    pub support_export: RewriteFlowSupportExportPacket,
    /// Inspection row.
    pub inspection: RewriteFlowInspectionRecord,
}

impl DiffFirstRewriteFlowPacket {
    /// Builds a diff-first rewrite-flow packet from a beta review-workspace
    /// packet and rewrite-flow input.
    ///
    /// # Errors
    ///
    /// Returns [`RewriteFlowValidationError`] when the input violates a
    /// rewrite-flow invariant.
    pub fn from_workspace_packet(
        input: RewriteFlowInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, RewriteFlowValidationError> {
        validate_input(&input, workspace_packet)?;

        let rewrite_flow = rewrite_flow_record(&input, workspace_packet);
        let diff_first_review = diff_first_review_record(&input.diff_review, &rewrite_flow);
        let sequence_edit_proposal = input
            .sequence_edit_proposal
            .as_ref()
            .map(|prop| sequence_edit_proposal_record(prop, &rewrite_flow));
        let recovery_checkpoint_summary =
            recovery_checkpoint_summary_record(&input.recovery_checkpoint_summary, &rewrite_flow);
        let commands = input
            .commands
            .iter()
            .map(|command| rewrite_flow_command_record(command, &rewrite_flow))
            .collect::<Vec<_>>();
        let support_export = rewrite_flow_support_export_packet(
            &input.support_export,
            &rewrite_flow,
            workspace_packet,
            &commands,
            &diff_first_review,
            &recovery_checkpoint_summary,
        );
        let inspection = rewrite_flow_inspection_record(
            &rewrite_flow,
            &diff_first_review,
            sequence_edit_proposal.as_ref(),
            &recovery_checkpoint_summary,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: DIFF_FIRST_REWRITE_FLOW_PACKET_RECORD_KIND.to_string(),
            schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            rewrite_flow,
            diff_first_review,
            sequence_edit_proposal,
            recovery_checkpoint_summary,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the rewrite-flow invariants.
    ///
    /// # Errors
    ///
    /// Returns [`RewriteFlowValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), RewriteFlowValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            DIFF_FIRST_REWRITE_FLOW_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_rewrite_flow_record(
            &self.rewrite_flow,
            &self.review_workspace.review_workspace_id,
        )?;
        validate_diff_first_review_record(
            &self.diff_first_review,
            &self.rewrite_flow.rewrite_flow_id,
        )?;
        if let Some(proposal) = &self.sequence_edit_proposal {
            validate_sequence_edit_proposal_record(proposal, &self.rewrite_flow.rewrite_flow_id)?;
        }
        validate_recovery_checkpoint_summary_record(
            &self.recovery_checkpoint_summary,
            &self.rewrite_flow.rewrite_flow_id,
        )?;
        for command in &self.commands {
            validate_rewrite_flow_command_record(command, &self.rewrite_flow.rewrite_flow_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.rewrite_flow,
            &self.commands,
            &self.diff_first_review,
            &self.recovery_checkpoint_summary,
        )?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        if self.diff_first_review.diff_review_state == "approved_with_checkpoints" {
            if self.diff_first_review.checkpoint_required_before_apply {
                if !matches!(
                    self.recovery_checkpoint_summary.checkpoint_state.as_str(),
                    "captured_ready" | "captured_pending" | "restored"
                ) {
                    return Err(rewrite_flow_validation_error(
                        "diff approved_with_checkpoints requires a captured or restored checkpoint",
                    ));
                }
            }
        }
        if matches!(
            self.rewrite_flow.flow_state.as_str(),
            "executing" | "completed" | "paused_conflict"
        ) {
            if self.diff_first_review.diff_review_state != "approved_with_checkpoints" {
                return Err(rewrite_flow_validation_error(
                    "flow_state executing/completed/paused_conflict requires diff_review_state approved_with_checkpoints",
                ));
            }
        }
        if self.rewrite_flow.flow_state == "completed" && self.inspection.paused_conflict {
            return Err(rewrite_flow_validation_error(
                "completed flow cannot be paused for conflict",
            ));
        }
        if self.rewrite_flow.protected_branch_posture == "protected_branch_blocked"
            && self.rewrite_flow.actionable
        {
            return Err(rewrite_flow_validation_error(
                "protected_branch_blocked posture must make the flow not actionable",
            ));
        }
        if self.rewrite_flow.flow_state == "aborted_rolled_back"
            && self.recovery_checkpoint_summary.checkpoint_state == "missing_blocks_apply"
        {
            return Err(rewrite_flow_validation_error(
                "aborted_rolled_back flow cannot have missing_blocks_apply checkpoint",
            ));
        }
        Ok(())
    }

    /// Returns true when rewrite-flow truth axes are surfaced as separable
    /// inspectable truths.
    pub fn truths_are_separable(&self) -> bool {
        let flow = &self.rewrite_flow;
        contains_token(REWRITE_FLOW_OPERATION_KINDS, &flow.operation_kind)
            && contains_token(REWRITE_FLOW_STATES, &flow.flow_state)
            && contains_token(PROTECTED_BRANCH_POSTURES, &flow.protected_branch_posture)
            && contains_token(DIVERGENCE_CLASSES, &flow.divergence_class)
            && contains_token(REWRITE_FLOW_APPROVAL_STATES, &flow.approval_state)
            && contains_token(
                REWRITE_FLOW_CHECKS_FRESHNESS_STATES,
                &flow.checks_freshness_state,
            )
            && contains_token(
                DIFF_FIRST_REVIEW_STATES,
                &self.diff_first_review.diff_review_state,
            )
            && contains_token(
                CHECKPOINT_SUMMARY_STATES,
                &self.recovery_checkpoint_summary.checkpoint_state,
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
pub struct DiffFirstRewriteFlowProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Rewrite flow identity.
    pub rewrite_flow_id: String,
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
    /// True when protected-branch block is active.
    pub protected_branch_blocked: bool,
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
    /// Remaining sequence-edit steps.
    pub remaining_step_count: u32,
}

/// Parses and validates a materialized diff-first rewrite-flow packet.
///
/// # Errors
///
/// Returns [`RewriteFlowError`] when the payload fails to parse or violates
/// the rewrite-flow invariants.
pub fn project_diff_first_rewrite_flow_packet(
    payload: &str,
) -> Result<DiffFirstRewriteFlowProjection, RewriteFlowError> {
    let packet: DiffFirstRewriteFlowPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(DiffFirstRewriteFlowProjection::from(packet))
}

impl From<DiffFirstRewriteFlowPacket> for DiffFirstRewriteFlowProjection {
    fn from(packet: DiffFirstRewriteFlowPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            rewrite_flow_id: packet.rewrite_flow.rewrite_flow_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            operation_kind: packet.rewrite_flow.operation_kind,
            flow_state: packet.rewrite_flow.flow_state,
            diff_review_state: packet.diff_first_review.diff_review_state,
            checkpoint_state: packet.recovery_checkpoint_summary.checkpoint_state,
            diff_approved: packet.inspection.diff_approved,
            checkpoint_ready: packet.inspection.checkpoint_ready,
            actionable: packet.inspection.actionable,
            restartable: packet.inspection.restartable,
            protected_branch_blocked: packet.inspection.protected_branch_blocked,
            invalidation_reasons: packet.rewrite_flow.invalidation_reasons,
            blocked_reasons: packet.rewrite_flow.blocked_reasons,
            command_count: packet.commands.len(),
            preview_capable: packet.inspection.preview_capable,
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
            remaining_step_count: packet.inspection.remaining_step_count,
        }
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error returned when a rewrite-flow payload cannot be projected.
#[derive(Debug)]
pub enum RewriteFlowError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the rewrite-flow invariants.
    Validation(RewriteFlowValidationError),
}

impl fmt::Display for RewriteFlowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "rewrite flow parse error: {err}"),
            Self::Validation(err) => write!(formatter, "rewrite flow validation error: {err}"),
        }
    }
}

impl std::error::Error for RewriteFlowError {}

impl From<serde_json::Error> for RewriteFlowError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<RewriteFlowValidationError> for RewriteFlowError {
    fn from(err: RewriteFlowValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for rewrite-flow packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteFlowValidationError {
    message: String,
}

impl RewriteFlowValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RewriteFlowValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RewriteFlowValidationError {}

fn rewrite_flow_validation_error(message: impl Into<String>) -> RewriteFlowValidationError {
    RewriteFlowValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Record builders
// ---------------------------------------------------------------------------

fn rewrite_flow_record(
    input: &RewriteFlowInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> RewriteFlowRecord {
    let mut invalidation_reasons = input.invalidation_reasons.clone();
    invalidation_reasons.extend(derive_invalidation_reasons(
        &input.approval_state,
        &input.checks_freshness_state,
        &input.protected_branch_posture,
    ));
    invalidation_reasons.sort();
    invalidation_reasons.dedup();

    let blocked_reasons = derive_blocked_reasons(
        &input.diff_review.diff_review_state,
        &input.recovery_checkpoint_summary.checkpoint_state,
        &input.protected_branch_posture,
        &input.approval_state,
        &input.checks_freshness_state,
        &invalidation_reasons,
    );

    let actionable = blocked_reasons.is_empty()
        && input.diff_review.diff_review_state == "approved_with_checkpoints"
        && !matches!(
            input.flow_state.as_str(),
            "executing"
                | "paused_conflict"
                | "completed"
                | "aborted_rolled_back"
                | "failed_no_changes_made"
        );

    RewriteFlowRecord {
        record_kind: REWRITE_FLOW_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        rewrite_flow_id: input.rewrite_flow_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        operation_kind: input.operation_kind.clone(),
        flow_state: input.flow_state.clone(),
        source_ref: input.source_ref.clone(),
        target_ref: input.target_ref.clone(),
        base_ref: input.base_ref.clone(),
        worktree_identity_ref: input.worktree_identity_ref.clone(),
        review_pack_digest_ref: input.review_pack_digest_ref.clone(),
        environment_capsule_digest_ref: input.environment_capsule_digest_ref.clone(),
        protected_branch_posture: input.protected_branch_posture.clone(),
        divergence_class: input.divergence_class.clone(),
        approval_state: input.approval_state.clone(),
        checks_freshness_state: input.checks_freshness_state.clone(),
        restart_session_ref: input.restart_session_ref.clone(),
        invalidation_reasons,
        blocked_reasons,
        actionable,
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn diff_first_review_record(
    input: &DiffFirstReviewInput,
    flow: &RewriteFlowRecord,
) -> DiffFirstReviewRecord {
    DiffFirstReviewRecord {
        record_kind: DIFF_FIRST_REVIEW_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        rewrite_flow_id_ref: flow.rewrite_flow_id.clone(),
        diff_review_state: input.diff_review_state.clone(),
        diff_preview_ref: input.diff_preview_ref.clone(),
        suspicious_content_flagged: input.suspicious_content_flagged,
        suspicious_content_reviewed: input.suspicious_content_reviewed,
        checkpoint_required_before_apply: input.checkpoint_required_before_apply,
        manual_review_reason: input.manual_review_reason.clone(),
        summary_label: format!(
            "Diff review for {} ({})",
            flow.rewrite_flow_id, input.diff_review_state
        ),
    }
}

fn sequence_edit_proposal_record(
    input: &SequenceEditProposalInput,
    flow: &RewriteFlowRecord,
) -> SequenceEditProposalRecord {
    let remaining = input
        .ordered_operations
        .iter()
        .filter(|op| !op.completed)
        .count() as u32;
    SequenceEditProposalRecord {
        record_kind: SEQUENCE_EDIT_PROPOSAL_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        rewrite_flow_id_ref: flow.rewrite_flow_id.clone(),
        proposal_id: input.proposal_id.clone(),
        ordered_operations: input
            .ordered_operations
            .iter()
            .map(|op| SequenceEditOperationRecord {
                ordinal: op.ordinal,
                verb: op.verb.clone(),
                target_ref: op.target_ref.clone(),
                display_label: op.display_label.clone(),
                current_step: op.current_step,
                completed: op.completed,
                conflict_session_ref: op.conflict_session_ref.clone(),
            })
            .collect(),
        protected_branch_blocked: input.protected_branch_blocked,
        invalidation_state: input.invalidation_state.clone(),
        remaining_step_count: remaining,
        summary_label: format!(
            "Sequence proposal {} ({} remaining)",
            input.proposal_id, remaining
        ),
    }
}

fn recovery_checkpoint_summary_record(
    input: &RecoveryCheckpointSummaryInput,
    flow: &RewriteFlowRecord,
) -> RecoveryCheckpointSummaryRecord {
    RecoveryCheckpointSummaryRecord {
        record_kind: RECOVERY_CHECKPOINT_SUMMARY_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        rewrite_flow_id_ref: flow.rewrite_flow_id.clone(),
        checkpoint_state: input.checkpoint_state.clone(),
        checkpoint_ref: input.checkpoint_ref.clone(),
        restore_command_id: input.restore_command_id.clone(),
        disclosure_label: input.disclosure_label.clone(),
        restorable_offline: input.restorable_offline,
        summary_label: format!(
            "Checkpoint for {} ({})",
            flow.rewrite_flow_id, input.checkpoint_state
        ),
    }
}

fn rewrite_flow_command_record(
    input: &RewriteFlowCommandInput,
    flow: &RewriteFlowRecord,
) -> RewriteFlowCommandRecord {
    RewriteFlowCommandRecord {
        record_kind: REWRITE_FLOW_COMMAND_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        rewrite_flow_id_ref: flow.rewrite_flow_id.clone(),
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

fn rewrite_flow_support_export_packet(
    input: &RewriteFlowSupportExportInput,
    flow: &RewriteFlowRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    commands: &[RewriteFlowCommandRecord],
    diff_review: &DiffFirstReviewRecord,
    checkpoint: &RecoveryCheckpointSummaryRecord,
) -> RewriteFlowSupportExportPacket {
    RewriteFlowSupportExportPacket {
        record_kind: REWRITE_FLOW_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        rewrite_flow_id_ref: flow.rewrite_flow_id.clone(),
        review_workspace_id_ref: workspace_packet
            .review_workspace
            .review_workspace_id
            .clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/diff_first_rewrite_flow.schema.json".to_string(),
            "schemas/git/recovery_checkpoint.schema.json".to_string(),
            "schemas/git/sequence_edit_session.schema.json".to_string(),
            "schemas/review/review_workspace.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        raw_path_export_allowed: false,
        raw_branch_name_export_allowed: false,
        raw_patch_body_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        restart_snapshot: RewriteFlowRestartSnapshot {
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

fn rewrite_flow_inspection_record(
    flow: &RewriteFlowRecord,
    diff_review: &DiffFirstReviewRecord,
    sequence_edit_proposal: Option<&SequenceEditProposalRecord>,
    checkpoint: &RecoveryCheckpointSummaryRecord,
    commands: &[RewriteFlowCommandRecord],
    support_export: &RewriteFlowSupportExportPacket,
) -> RewriteFlowInspectionRecord {
    let diff_approved = diff_review.diff_review_state == "approved_with_checkpoints";
    let diff_pending = diff_review.diff_review_state == "pending";
    let diff_rejected = diff_review.diff_review_state == "rejected";
    let checkpoint_ready = checkpoint.checkpoint_state == "captured_ready";
    let executing = flow.flow_state == "executing";
    let paused_conflict = flow.flow_state == "paused_conflict";
    let completed = flow.flow_state == "completed";
    let aborted = flow.flow_state == "aborted_rolled_back";
    let protected_branch_blocked = flow.protected_branch_posture == "protected_branch_blocked";
    let policy_blocks_apply = flow.protected_branch_posture == "policy_lock_active";
    let approval_invalidated = flow.approval_state == "approval_invalidated_by_changes";
    let checks_stale_blocks_apply = flow.checks_freshness_state == "checks_stale_blocks_apply";
    let actionable = flow.actionable;
    let restartable = !flow.restart_session_ref.trim().is_empty()
        && support_export_can_reopen(support_export, commands);
    let preview_capable = commands.iter().any(|command| command.preview_supported);
    let support_export_reopenable = support_export_can_reopen(support_export, commands);
    let suspicious_content_unreviewed =
        diff_review.suspicious_content_flagged && !diff_review.suspicious_content_reviewed;
    let remaining_step_count = sequence_edit_proposal
        .map(|prop| prop.remaining_step_count)
        .unwrap_or(0);

    RewriteFlowInspectionRecord {
        record_kind: REWRITE_FLOW_INSPECTION_RECORD_KIND.to_string(),
        schema_version: DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        rewrite_flow_id_ref: flow.rewrite_flow_id.clone(),
        review_workspace_id_ref: flow.review_workspace_id_ref.clone(),
        diff_approved,
        diff_pending,
        diff_rejected,
        checkpoint_ready,
        executing,
        paused_conflict,
        completed,
        aborted,
        protected_branch_blocked,
        policy_blocks_apply,
        approval_invalidated,
        checks_stale_blocks_apply,
        actionable,
        restartable,
        command_count: commands.len(),
        preview_capable,
        support_export_reopenable,
        suspicious_content_unreviewed,
        remaining_step_count,
        summary_label: format!(
            "Rewrite flow {} ({} / {})",
            flow.rewrite_flow_id, flow.operation_kind, flow.flow_state
        ),
    }
}

fn support_export_can_reopen(
    export: &RewriteFlowSupportExportPacket,
    commands: &[RewriteFlowCommandRecord],
) -> bool {
    !export.reopen_context_ref.trim().is_empty()
        && !export.reopen_command_id_ref.trim().is_empty()
        && !export.raw_url_export_allowed
        && !export.raw_provider_payload_export_allowed
        && !export.raw_path_export_allowed
        && !export.raw_branch_name_export_allowed
        && !export.raw_patch_body_export_allowed
        && !commands.is_empty()
        && export
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export")
}

// ---------------------------------------------------------------------------
// Derivation helpers
// ---------------------------------------------------------------------------

fn derive_invalidation_reasons(
    approval_state: &str,
    checks_freshness_state: &str,
    protected_branch_posture: &str,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if approval_state == "approval_invalidated_by_changes" {
        reasons.push("approval_invalidated".to_string());
    }
    if checks_freshness_state == "checks_stale_blocks_apply" {
        reasons.push("checks_stale".to_string());
    }
    if protected_branch_posture == "policy_lock_active" {
        reasons.push("policy_blocked".to_string());
    }
    reasons
}

fn derive_blocked_reasons(
    diff_review_state: &str,
    checkpoint_state: &str,
    protected_branch_posture: &str,
    approval_state: &str,
    checks_freshness_state: &str,
    invalidation_reasons: &[String],
) -> Vec<String> {
    let mut blocked = Vec::new();
    if diff_review_state != "approved_with_checkpoints" {
        blocked.push("diff_not_approved".to_string());
    }
    if checkpoint_state == "missing_blocks_apply" {
        blocked.push("missing_recovery_checkpoint".to_string());
    }
    if protected_branch_posture == "protected_branch_blocked" {
        blocked.push("protected_branch_blocked".to_string());
    }
    if protected_branch_posture == "policy_lock_active" {
        blocked.push("policy_lock_active".to_string());
    }
    if approval_state == "approval_required_outstanding" {
        blocked.push("approval_required_outstanding".to_string());
    }
    if approval_state == "approval_invalidated_by_changes" {
        blocked.push("approval_invalidated".to_string());
    }
    if checks_freshness_state == "checks_stale_blocks_apply" {
        blocked.push("checks_stale_blocks_apply".to_string());
    }
    for reason in invalidation_reasons {
        if !blocked.contains(reason) {
            blocked.push(reason.clone());
        }
    }
    blocked
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &RewriteFlowInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), RewriteFlowValidationError> {
    ensure_nonempty(&input.rewrite_flow_id, "rewrite_flow_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.source_ref, "source_ref")?;
    ensure_nonempty(&input.target_ref, "target_ref")?;
    ensure_nonempty(&input.base_ref, "base_ref")?;
    ensure_nonempty(&input.worktree_identity_ref, "worktree_identity_ref")?;
    ensure_nonempty(&input.review_pack_digest_ref, "review_pack_digest_ref")?;
    ensure_nonempty(
        &input.environment_capsule_digest_ref,
        "environment_capsule_digest_ref",
    )?;
    ensure_nonempty(&input.restart_session_ref, "restart_session_ref")?;
    ensure_token(
        REWRITE_FLOW_OPERATION_KINDS,
        &input.operation_kind,
        "operation_kind",
    )?;
    ensure_token(REWRITE_FLOW_STATES, &input.flow_state, "flow_state")?;
    ensure_token(
        PROTECTED_BRANCH_POSTURES,
        &input.protected_branch_posture,
        "protected_branch_posture",
    )?;
    ensure_token(
        DIVERGENCE_CLASSES,
        &input.divergence_class,
        "divergence_class",
    )?;
    ensure_token(
        REWRITE_FLOW_APPROVAL_STATES,
        &input.approval_state,
        "approval_state",
    )?;
    ensure_token(
        REWRITE_FLOW_CHECKS_FRESHNESS_STATES,
        &input.checks_freshness_state,
        "checks_freshness_state",
    )?;
    ensure_token(
        DIFF_FIRST_REVIEW_STATES,
        &input.diff_review.diff_review_state,
        "diff_review.diff_review_state",
    )?;
    ensure_token(
        CHECKPOINT_SUMMARY_STATES,
        &input.recovery_checkpoint_summary.checkpoint_state,
        "recovery_checkpoint_summary.checkpoint_state",
    )?;
    for reason in &input.invalidation_reasons {
        ensure_token(
            REWRITE_FLOW_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }
    for command in &input.commands {
        ensure_nonempty(&command.command_id, "command.command_id")?;
        ensure_token(
            REWRITE_FLOW_COMMAND_CLASSES,
            &command.command_class,
            "command.command_class",
        )?;
    }
    for surface in &input.support_export.consumer_surfaces {
        ensure_token(
            REWRITE_FLOW_CONSUMER_SURFACES,
            surface,
            "support_export.consumer_surfaces",
        )?;
    }

    // Sequence-edit proposal required for interactive_rebase and cherry_pick
    if matches!(
        input.operation_kind.as_str(),
        "interactive_rebase" | "cherry_pick"
    ) && input.sequence_edit_proposal.is_none()
    {
        return Err(rewrite_flow_validation_error(
            "interactive_rebase and cherry_pick require a sequence_edit_proposal",
        ));
    }

    // Reset operations must require a checkpoint
    if input.operation_kind == "reset" && !input.diff_review.checkpoint_required_before_apply {
        return Err(rewrite_flow_validation_error(
            "reset operation must set checkpoint_required_before_apply=true",
        ));
    }

    ensure_nonempty(
        &workspace_packet.review_workspace.review_workspace_id,
        "workspace_packet.review_workspace.review_workspace_id",
    )?;
    Ok(())
}

fn validate_rewrite_flow_record(
    record: &RewriteFlowRecord,
    expected_workspace_id: &str,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REWRITE_FLOW_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_nonempty(&record.rewrite_flow_id, "rewrite_flow_id")?;
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        expected_workspace_id,
        "review_workspace_id_ref",
    )?;
    ensure_token(
        REWRITE_FLOW_OPERATION_KINDS,
        &record.operation_kind,
        "operation_kind",
    )?;
    ensure_token(REWRITE_FLOW_STATES, &record.flow_state, "flow_state")?;
    ensure_token(
        PROTECTED_BRANCH_POSTURES,
        &record.protected_branch_posture,
        "protected_branch_posture",
    )?;
    ensure_token(
        DIVERGENCE_CLASSES,
        &record.divergence_class,
        "divergence_class",
    )?;
    ensure_token(
        REWRITE_FLOW_APPROVAL_STATES,
        &record.approval_state,
        "approval_state",
    )?;
    ensure_token(
        REWRITE_FLOW_CHECKS_FRESHNESS_STATES,
        &record.checks_freshness_state,
        "checks_freshness_state",
    )?;
    ensure_nonempty(&record.restart_session_ref, "restart_session_ref")?;
    for reason in &record.invalidation_reasons {
        ensure_token(
            REWRITE_FLOW_INVALIDATION_REASONS,
            reason,
            "invalidation_reason",
        )?;
    }
    Ok(())
}

fn validate_diff_first_review_record(
    record: &DiffFirstReviewRecord,
    expected_flow_id: &str,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        DIFF_FIRST_REVIEW_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_eq(
        record.rewrite_flow_id_ref.as_str(),
        expected_flow_id,
        "rewrite_flow_id_ref",
    )?;
    ensure_token(
        DIFF_FIRST_REVIEW_STATES,
        &record.diff_review_state,
        "diff_review_state",
    )?;
    ensure_nonempty(&record.diff_preview_ref, "diff_preview_ref")?;
    if record.diff_review_state == "requires_manual_review" && record.manual_review_reason.is_none()
    {
        return Err(rewrite_flow_validation_error(
            "diff_review_state requires_manual_review must include manual_review_reason",
        ));
    }
    Ok(())
}

fn validate_sequence_edit_proposal_record(
    record: &SequenceEditProposalRecord,
    expected_flow_id: &str,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        SEQUENCE_EDIT_PROPOSAL_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_eq(
        record.rewrite_flow_id_ref.as_str(),
        expected_flow_id,
        "rewrite_flow_id_ref",
    )?;
    ensure_nonempty(&record.proposal_id, "proposal_id")?;
    if record.ordered_operations.is_empty() {
        return Err(rewrite_flow_validation_error(
            "sequence_edit_proposal must have at least one ordered_operation",
        ));
    }
    let mut ordinals: Vec<u32> = record
        .ordered_operations
        .iter()
        .map(|op| op.ordinal)
        .collect();
    ordinals.sort();
    for window in ordinals.windows(2) {
        if window[0] == window[1] {
            return Err(rewrite_flow_validation_error(
                "sequence_edit_proposal ordered_operations must have unique ordinals",
            ));
        }
    }
    Ok(())
}

fn validate_recovery_checkpoint_summary_record(
    record: &RecoveryCheckpointSummaryRecord,
    expected_flow_id: &str,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        RECOVERY_CHECKPOINT_SUMMARY_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_eq(
        record.rewrite_flow_id_ref.as_str(),
        expected_flow_id,
        "rewrite_flow_id_ref",
    )?;
    ensure_token(
        CHECKPOINT_SUMMARY_STATES,
        &record.checkpoint_state,
        "checkpoint_state",
    )?;
    if matches!(
        record.checkpoint_state.as_str(),
        "captured_ready" | "captured_pending" | "restored"
    ) && record.checkpoint_ref.is_none()
    {
        return Err(rewrite_flow_validation_error(
            "captured/restored checkpoint_state requires checkpoint_ref",
        ));
    }
    if matches!(
        record.checkpoint_state.as_str(),
        "captured_ready" | "captured_pending" | "restored"
    ) && record.restore_command_id.is_none()
    {
        return Err(rewrite_flow_validation_error(
            "captured/restored checkpoint_state requires restore_command_id",
        ));
    }
    Ok(())
}

fn validate_rewrite_flow_command_record(
    record: &RewriteFlowCommandRecord,
    expected_flow_id: &str,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        REWRITE_FLOW_COMMAND_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        record.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_eq(
        record.rewrite_flow_id_ref.as_str(),
        expected_flow_id,
        "rewrite_flow_id_ref",
    )?;
    ensure_nonempty(&record.command_id, "command_id")?;
    ensure_token(
        REWRITE_FLOW_COMMAND_CLASSES,
        &record.command_class,
        "command_class",
    )?;
    Ok(())
}

fn validate_support_export(
    export: &RewriteFlowSupportExportPacket,
    flow: &RewriteFlowRecord,
    commands: &[RewriteFlowCommandRecord],
    diff_review: &DiffFirstReviewRecord,
    checkpoint: &RecoveryCheckpointSummaryRecord,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        REWRITE_FLOW_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        export.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_eq(
        export.rewrite_flow_id_ref.as_str(),
        flow.rewrite_flow_id.as_str(),
        "rewrite_flow_id_ref",
    )?;
    if export.raw_url_export_allowed {
        return Err(rewrite_flow_validation_error(
            "raw_url_export_allowed must be false",
        ));
    }
    if export.raw_provider_payload_export_allowed {
        return Err(rewrite_flow_validation_error(
            "raw_provider_payload_export_allowed must be false",
        ));
    }
    if export.raw_path_export_allowed {
        return Err(rewrite_flow_validation_error(
            "raw_path_export_allowed must be false",
        ));
    }
    if export.raw_branch_name_export_allowed {
        return Err(rewrite_flow_validation_error(
            "raw_branch_name_export_allowed must be false",
        ));
    }
    if export.raw_patch_body_export_allowed {
        return Err(rewrite_flow_validation_error(
            "raw_patch_body_export_allowed must be false",
        ));
    }
    if !export
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "support_export")
    {
        return Err(rewrite_flow_validation_error(
            "support_export consumer_surface must be present",
        ));
    }
    if !export
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "audit_lane")
    {
        return Err(rewrite_flow_validation_error(
            "audit_lane consumer_surface must be present",
        ));
    }
    // Restart snapshot must mirror current truth
    if export.restart_snapshot.flow_state != flow.flow_state {
        return Err(rewrite_flow_validation_error(
            "restart_snapshot.flow_state must match flow.flow_state",
        ));
    }
    if export.restart_snapshot.diff_review_state != diff_review.diff_review_state {
        return Err(rewrite_flow_validation_error(
            "restart_snapshot.diff_review_state must match diff_first_review.diff_review_state",
        ));
    }
    if export.restart_snapshot.checkpoint_state != checkpoint.checkpoint_state {
        return Err(rewrite_flow_validation_error(
            "restart_snapshot.checkpoint_state must match recovery_checkpoint_summary.checkpoint_state",
        ));
    }
    // Command ids must match
    let expected_ids: BTreeSet<String> = commands.iter().map(|c| c.command_id.clone()).collect();
    let actual_ids: BTreeSet<String> = export.command_id_refs.iter().cloned().collect();
    if expected_ids != actual_ids {
        return Err(rewrite_flow_validation_error(
            "support_export.command_id_refs must match commands",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &RewriteFlowInspectionRecord,
    packet: &DiffFirstRewriteFlowPacket,
) -> Result<(), RewriteFlowValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        REWRITE_FLOW_INSPECTION_RECORD_KIND,
        "record_kind",
    )?;
    ensure_eq(
        inspection.schema_version,
        DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
        "schema_version",
    )?;
    ensure_eq(
        inspection.rewrite_flow_id_ref.as_str(),
        packet.rewrite_flow.rewrite_flow_id.as_str(),
        "rewrite_flow_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.review_workspace.review_workspace_id.as_str(),
        "review_workspace_id_ref",
    )?;
    if inspection.diff_approved
        != (packet.diff_first_review.diff_review_state == "approved_with_checkpoints")
    {
        return Err(rewrite_flow_validation_error(
            "inspection.diff_approved must match diff_review_state",
        ));
    }
    if inspection.checkpoint_ready
        != (packet.recovery_checkpoint_summary.checkpoint_state == "captured_ready")
    {
        return Err(rewrite_flow_validation_error(
            "inspection.checkpoint_ready must match checkpoint_state",
        ));
    }
    if inspection.actionable != packet.rewrite_flow.actionable {
        return Err(rewrite_flow_validation_error(
            "inspection.actionable must match rewrite_flow.actionable",
        ));
    }
    if inspection.command_count != packet.commands.len() {
        return Err(rewrite_flow_validation_error(
            "inspection.command_count must match commands.len()",
        ));
    }
    if inspection.preview_capable != packet.commands.iter().any(|c| c.preview_supported) {
        return Err(rewrite_flow_validation_error(
            "inspection.preview_capable must match any command.preview_supported",
        ));
    }
    if inspection.support_export_reopenable
        != support_export_can_reopen(&packet.support_export, &packet.commands)
    {
        return Err(rewrite_flow_validation_error(
            "inspection.support_export_reopenable must match support_export",
        ));
    }
    if inspection.remaining_step_count
        != packet
            .sequence_edit_proposal
            .as_ref()
            .map(|p| p.remaining_step_count)
            .unwrap_or(0)
    {
        return Err(rewrite_flow_validation_error(
            "inspection.remaining_step_count must match sequence_edit_proposal",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Small utilities
// ---------------------------------------------------------------------------

fn ensure_nonempty(value: &str, field: &str) -> Result<(), RewriteFlowValidationError> {
    if value.trim().is_empty() {
        Err(rewrite_flow_validation_error(format!(
            "{field} must be non-empty"
        )))
    } else {
        Ok(())
    }
}

fn ensure_eq<T: PartialEq + std::fmt::Debug>(
    actual: T,
    expected: T,
    field: &str,
) -> Result<(), RewriteFlowValidationError> {
    if actual != expected {
        Err(rewrite_flow_validation_error(format!(
            "{field} must be {expected:?}, got {actual:?}"
        )))
    } else {
        Ok(())
    }
}

fn ensure_token(
    vocab: &[&str],
    value: &str,
    field: &str,
) -> Result<(), RewriteFlowValidationError> {
    if vocab.iter().any(|token| *token == value) {
        Ok(())
    } else {
        Err(rewrite_flow_validation_error(format!(
            "{field} must be one of {vocab:?}, got {value}"
        )))
    }
}

fn contains_token(vocab: &[&str], value: &str) -> bool {
    vocab.iter().any(|token| *token == value)
}
