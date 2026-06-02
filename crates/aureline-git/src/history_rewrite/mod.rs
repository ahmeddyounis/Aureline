//! History-rewrite, conflict-session, stash-entry, sequence-edit, and recovery-checkpoint
//! beta contracts for risky Git mutation flows.
//!
//! This module owns the durable session vocabulary that conflict surfaces,
//! interactive sequence-edit surfaces, stash or shelf objects, recovery
//! checkpoints, and ref-update proposals all project from a single bounded
//! truth. Desktop, CLI/headless, and support/export views read the same
//! [`HistoryRewriteRecord`] family so an in-progress rebase, cherry-pick,
//! revert, reset, patch-apply, or stash flow keeps operation provenance,
//! affected refs, recovery posture, protected-branch decisions, and explicit
//! next-safe routes consistent across surfaces and restarts.
//!
//! The companion schemas live at `schemas/git/conflict_session.schema.json`,
//! `schemas/git/sequence_edit_session.schema.json`,
//! `schemas/git/stash_entry.schema.json`, and
//! `schemas/git/recovery_checkpoint.schema.json`. The reviewer doc lives at
//! `docs/source_control/m3/history_rewrite_beta.md`. Canonical fixtures live
//! under `fixtures/git/m3/history_rewrite_and_stash/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every beta history-rewrite record.
pub const HISTORY_REWRITE_BETA_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for [`ConflictSessionRecord`].
pub const CONFLICT_SESSION_RECORD_KIND: &str = "history_rewrite_conflict_session_record";

/// Record-kind tag for [`SequenceEditSessionRecord`].
pub const SEQUENCE_EDIT_SESSION_RECORD_KIND: &str = "history_rewrite_sequence_edit_session_record";

/// Record-kind tag for [`StashEntryRecord`].
pub const STASH_ENTRY_RECORD_KIND: &str = "history_rewrite_stash_entry_record";

/// Record-kind tag for [`RecoveryCheckpointRecord`].
pub const RECOVERY_CHECKPOINT_RECORD_KIND: &str = "history_rewrite_recovery_checkpoint_record";

/// Record-kind tag for [`RefUpdateProposalRecord`].
pub const REF_UPDATE_PROPOSAL_RECORD_KIND: &str = "history_rewrite_ref_update_proposal_record";

/// Closed set of risky Git operation kinds tracked by this contract.
pub const HISTORY_REWRITE_OPERATION_KINDS: &[&str] = &[
    "rebase",
    "interactive_rebase",
    "cherry_pick",
    "revert",
    "reset",
    "patch_apply",
    "stash_apply",
    "stash_pop",
    "stash_drop",
    "branch_from_stash",
    "merge",
    "amend",
];

/// Closed set of conflict-session lifecycle states.
pub const CONFLICT_SESSION_LIFECYCLE_STATES: &[&str] = &[
    "draft_pending_admit",
    "active_awaiting_resolution",
    "paused_awaiting_user_input",
    "paused_awaiting_external_tool",
    "continuing_after_resolution",
    "skipped_conflicted_step",
    "aborted_rolled_back",
    "completed_committed",
    "completed_handed_off",
    "failed_no_changes_made",
];

/// Closed set of sequence-edit-session lifecycle states.
pub const SEQUENCE_EDIT_SESSION_LIFECYCLE_STATES: &[&str] = &[
    "draft_unsaved",
    "saved_ready_to_run",
    "running",
    "paused_for_conflict",
    "paused_for_user_edit",
    "completed_admitted",
    "aborted_rolled_back",
    "failed_no_changes_made",
];

/// Closed set of stash-entry lifecycle states.
pub const STASH_ENTRY_LIFECYCLE_STATES: &[&str] = &[
    "captured_unapplied",
    "applied_kept",
    "applied_popped",
    "dropped",
    "promoted_to_branch",
    "applied_with_conflict",
];

/// Closed set of recovery-checkpoint lifecycle states.
pub const RECOVERY_CHECKPOINT_LIFECYCLE_STATES: &[&str] = &[
    "captured_ready_to_restore",
    "captured_pending_admit",
    "restored",
    "expired_pending_prune",
    "missing_pending_review",
];

/// Closed set of ref-update-proposal lifecycle states.
pub const REF_UPDATE_PROPOSAL_LIFECYCLE_STATES: &[&str] = &[
    "drafted_pending_review",
    "blocked_protected_branch",
    "blocked_policy",
    "blocked_collaboration",
    "ready_to_apply",
    "applied",
    "withdrawn",
];

/// Closed set of recovery postures recorded before a destructive step.
pub const RECOVERY_POSTURE_CLASSES: &[&str] = &[
    "recovery_checkpoint_captured",
    "reflog_only_disclosure_acknowledged",
    "no_recovery_available_blocks_apply",
    "external_handoff_pending",
];

/// Closed set of block reasons that gate a ref-update proposal.
pub const REF_UPDATE_BLOCK_CLASSES: &[&str] = &[
    "no_block",
    "protected_branch_no_force_push",
    "protected_branch_no_deletion",
    "policy_admin_lock",
    "policy_required_review",
    "collaboration_active_session",
    "missing_recovery_disclosure",
    "block_class_unknown_requires_review",
];

/// Closed set of next-safe paths surfaced when a risky op is blocked.
pub const NEXT_SAFE_PATH_CLASSES: &[&str] = &[
    "open_alternate_worktree",
    "create_temporary_branch",
    "export_history_plan",
    "review_only_mode",
    "request_approval",
    "abort_operation",
    "restore_checkpoint",
    "switch_to_reflog_disclosure",
    "no_safe_path_blocked_requires_human",
];

/// Closed set of conflict-session actor actions.
pub const CONFLICT_ACTION_CLASSES: &[&str] = &[
    "continue_after_resolution",
    "skip_conflicted_step",
    "abort_and_restore",
    "open_resolver",
    "request_external_handoff",
    "pause_for_break",
    "restore_recovery_checkpoint",
];

/// Closed set of sequence-edit action verbs (interactive rebase todos / cherry-pick lists).
pub const SEQUENCE_EDIT_VERBS: &[&str] = &[
    "pick",
    "reword",
    "edit",
    "squash",
    "fixup",
    "drop",
    "exec",
    "label",
    "reset",
    "merge",
    "break_point",
    "skip",
];

/// Closed set of consumer surfaces that may quote a history-rewrite record.
pub const HISTORY_REWRITE_CONSUMER_SURFACES: &[&str] = &[
    "desktop_history_panel",
    "desktop_conflict_resolver",
    "desktop_sequence_editor",
    "desktop_stash_browser",
    "activity_center",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "review_panel",
];

/// Closed set of audit events emitted against this contract.
pub const HISTORY_REWRITE_AUDIT_EVENTS: &[&str] = &[
    "session_admitted",
    "session_resumed_after_restart",
    "conflict_continue_requested",
    "conflict_skip_requested",
    "conflict_abort_requested",
    "sequence_edit_saved",
    "sequence_edit_step_started",
    "sequence_edit_step_completed",
    "stash_captured",
    "stash_applied",
    "stash_dropped",
    "stash_promoted_to_branch",
    "recovery_checkpoint_captured",
    "recovery_checkpoint_restored",
    "ref_update_proposal_drafted",
    "ref_update_proposal_blocked",
    "ref_update_proposal_applied",
    "next_safe_path_offered",
    "next_safe_path_accepted",
];

/// Reference to a Git ref or revision quoted without raw branch / remote names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteRefId {
    /// Opaque ref id used by activity, audit, and support records.
    pub ref_id: String,
    /// Class token: `local_branch`, `remote_tracking_branch`, `tag`, `commit`,
    /// `worktree_head`, `stash_handle`, or `reflog_position`.
    pub ref_class: String,
    /// Redaction-safe display label (no raw branch or remote names).
    pub display_label: String,
}

/// Worktree or repository context the session is bound to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteWorktreeContext {
    /// Stable workspace ref carried into every projection.
    pub workspace_ref: String,
    /// Opaque worktree ref.
    pub worktree_ref: String,
    /// Display-safe worktree label.
    pub worktree_label: String,
    /// True when the worktree is detached from the primary worktree.
    pub is_linked_worktree: bool,
}

/// Recovery-posture envelope carried on every risky session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteRecoveryPosture {
    /// Recovery posture class from [`RECOVERY_POSTURE_CLASSES`].
    pub posture_class: String,
    /// Optional checkpoint ref captured before the destructive step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// True when a recovery checkpoint was captured before the destructive step.
    pub checkpoint_captured: bool,
    /// True when the user explicitly accepted a reflog-only fallback disclosure.
    pub reflog_only_disclosure_acknowledged: bool,
    /// Redaction-safe disclosure label shown to reviewers.
    pub disclosure_label: String,
}

impl HistoryRewriteRecoveryPosture {
    /// Returns true when this posture satisfies the destructive-action gate.
    pub fn satisfies_destructive_gate(&self) -> bool {
        match self.posture_class.as_str() {
            "recovery_checkpoint_captured" => {
                self.checkpoint_captured && self.recovery_checkpoint_ref.is_some()
            }
            "reflog_only_disclosure_acknowledged" => self.reflog_only_disclosure_acknowledged,
            "external_handoff_pending" => false,
            "no_recovery_available_blocks_apply" => false,
            _ => false,
        }
    }
}

/// One next-safe path quoted on a blocked session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteNextSafePath {
    /// Class token from [`NEXT_SAFE_PATH_CLASSES`].
    pub path_class: String,
    /// Stable command id surfaces should invoke to take this path.
    pub command_id: String,
    /// Redaction-safe action label.
    pub display_label: String,
    /// True when this path keeps protected refs unchanged.
    pub preserves_protected_refs: bool,
    /// Optional ref id of an alternate worktree, temporary branch, or plan
    /// produced by this path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_ref: Option<HistoryRewriteRefId>,
}

/// Protected-branch / policy / collaboration block placed on a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteBlock {
    /// Block class from [`REF_UPDATE_BLOCK_CLASSES`].
    pub block_class: String,
    /// Stable block id used by audit and support records.
    pub block_id: String,
    /// Redaction-safe reviewer-facing explanation.
    pub display_label: String,
    /// Next safe paths offered to the user while this block is active.
    pub next_safe_paths: Vec<HistoryRewriteNextSafePath>,
}

/// Audit event projected from a history-rewrite session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteAuditEvent {
    /// Event id from [`HISTORY_REWRITE_AUDIT_EVENTS`].
    pub event_id: String,
    /// Stable record id of the emitting session record.
    pub session_ref: String,
    /// Redaction-safe reviewer summary.
    pub summary_label: String,
    /// Timestamp captured for deterministic replay.
    pub occurred_at: String,
    /// Optional next-safe-path class when the event is `next_safe_path_*`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_safe_path_class: Option<String>,
    /// Optional checkpoint ref the event touched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
}

/// Support-export disclosure attached to every record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRewriteSupportExport {
    /// Stable support-export refs that quote this record.
    pub export_packet_refs: Vec<String>,
    /// Redaction class for support exports.
    pub redaction_class: String,
    /// True when raw paths may cross the export boundary.
    pub raw_path_export_allowed: bool,
    /// True when raw branch / remote names may cross the export boundary.
    pub raw_branch_name_export_allowed: bool,
    /// True when raw patch bodies may cross the export boundary.
    pub raw_patch_body_export_allowed: bool,
    /// True when raw reflog bodies may cross the export boundary.
    pub raw_reflog_body_export_allowed: bool,
    /// True when raw stash bodies may cross the export boundary.
    pub raw_stash_body_export_allowed: bool,
}

/// Sequence step quoted in a sequence-edit session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditStep {
    /// Zero-based ordinal in the saved sequence.
    pub ordinal: u32,
    /// Verb class from [`SEQUENCE_EDIT_VERBS`].
    pub verb: String,
    /// Opaque target ref (typically a commit or label).
    pub target_ref: HistoryRewriteRefId,
    /// Redaction-safe summary line (no raw subject bodies).
    pub display_label: String,
    /// True when this step is currently selected by the cursor.
    pub current_step: bool,
    /// True when this step has already executed in the current run.
    pub completed: bool,
    /// Optional conflict-session ref if this step paused for a conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_session_ref: Option<String>,
}

/// Conflict-session record describing one paused merge / rebase / cherry-pick /
/// revert / patch-apply / stash-apply / merge conflict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictSessionRecord {
    pub schema_version: u32,
    pub conflict_session_id: String,
    pub operation_kind: String,
    pub lifecycle_state: String,
    pub display_label: String,
    pub summary: String,
    pub worktree_context: HistoryRewriteWorktreeContext,
    /// Parent sequence-edit session ref when this conflict belongs to an
    /// interactive sequence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_edit_session_ref: Option<String>,
    /// Onto / target ref the operation is replaying onto.
    pub onto_ref: HistoryRewriteRefId,
    /// Source ref the operation is replaying from.
    pub source_ref: HistoryRewriteRefId,
    /// Ordered conflicted-path tokens (no raw paths).
    pub conflicted_path_tokens: Vec<String>,
    /// Available conflict actions ordered by reviewer preference.
    pub available_actions: Vec<String>,
    /// Recovery posture for the destructive continue / abort gate.
    pub recovery_posture: HistoryRewriteRecoveryPosture,
    /// Active proposal ref describing the ref move continue/abort would do.
    pub ref_update_proposal_ref: String,
    /// Surfaces wired to project this record.
    pub consumer_surfaces: Vec<String>,
    /// Support-export envelope.
    pub support_export: HistoryRewriteSupportExport,
    /// Audit events attached to this session.
    pub audit_events: Vec<HistoryRewriteAuditEvent>,
    /// Stable raw todo-text ref (kept opaque so structured cards and raw todo
    /// stay bound to the same underlying object).
    pub raw_todo_text_ref: String,
    pub minted_at: String,
    pub updated_at: String,
}

/// Sequence-edit-session record (interactive rebase todos, cherry-pick lists,
/// patch sequences).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceEditSessionRecord {
    pub schema_version: u32,
    pub sequence_edit_session_id: String,
    pub operation_kind: String,
    pub lifecycle_state: String,
    pub display_label: String,
    pub summary: String,
    pub worktree_context: HistoryRewriteWorktreeContext,
    pub onto_ref: HistoryRewriteRefId,
    pub head_ref: HistoryRewriteRefId,
    /// Structured sequence-edit steps. Always bound to [`raw_todo_text_ref`]
    /// so structured cards and raw todo stay one underlying object.
    pub steps: Vec<SequenceEditStep>,
    /// Stable opaque ref to the raw todo text persisted alongside the steps.
    pub raw_todo_text_ref: String,
    /// Active conflict-session ref when paused for a conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_conflict_session_ref: Option<String>,
    pub recovery_posture: HistoryRewriteRecoveryPosture,
    pub ref_update_proposal_ref: String,
    pub consumer_surfaces: Vec<String>,
    pub support_export: HistoryRewriteSupportExport,
    pub audit_events: Vec<HistoryRewriteAuditEvent>,
    pub minted_at: String,
    pub updated_at: String,
}

/// Stash-entry record (also covers shelf-style stashes and branch-from-stash).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashEntryRecord {
    pub schema_version: u32,
    pub stash_entry_id: String,
    pub lifecycle_state: String,
    pub display_label: String,
    pub summary: String,
    pub worktree_context: HistoryRewriteWorktreeContext,
    /// Stash handle ref (always opaque).
    pub stash_handle_ref: HistoryRewriteRefId,
    /// Base ref captured when the stash was created.
    pub base_ref: HistoryRewriteRefId,
    /// Path tokens captured in the stash (no raw paths).
    pub captured_path_tokens: Vec<String>,
    /// Number of staged file rows stored in the stash.
    pub captured_index_entry_count: u32,
    /// Number of worktree file rows stored in the stash.
    pub captured_worktree_entry_count: u32,
    /// True when the stash captured untracked files.
    pub captured_untracked: bool,
    /// Optional active conflict-session ref when an apply is paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_session_ref: Option<String>,
    /// Optional branch ref produced by branch-from-stash promotion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_branch_ref: Option<HistoryRewriteRefId>,
    pub recovery_posture: HistoryRewriteRecoveryPosture,
    pub consumer_surfaces: Vec<String>,
    pub support_export: HistoryRewriteSupportExport,
    pub audit_events: Vec<HistoryRewriteAuditEvent>,
    pub minted_at: String,
    pub updated_at: String,
}

/// Recovery-checkpoint record describing one rollback-safe pre-mutation snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCheckpointRecord {
    pub schema_version: u32,
    pub recovery_checkpoint_id: String,
    pub lifecycle_state: String,
    pub display_label: String,
    pub summary: String,
    pub worktree_context: HistoryRewriteWorktreeContext,
    /// Operation kind the checkpoint was captured for.
    pub operation_kind: String,
    /// Refs captured by the checkpoint (e.g. HEAD, index, worktree snapshot).
    pub captured_refs: Vec<HistoryRewriteRefId>,
    /// Retention class for support / local-history surfaces.
    pub retention_class: String,
    /// Stable command id that restores this checkpoint.
    pub restore_command_id: String,
    /// True when the checkpoint is restorable without network egress.
    pub restorable_offline: bool,
    /// Optional session refs that reference this checkpoint.
    pub referencing_session_refs: Vec<String>,
    /// Consumer surfaces wired to project this checkpoint.
    pub consumer_surfaces: Vec<String>,
    /// Support-export envelope.
    pub support_export: HistoryRewriteSupportExport,
    pub audit_events: Vec<HistoryRewriteAuditEvent>,
    pub captured_at: String,
    pub updated_at: String,
}

/// Ref-update proposal that gates every ref-moving step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefUpdateProposalRecord {
    pub schema_version: u32,
    pub ref_update_proposal_id: String,
    pub lifecycle_state: String,
    pub display_label: String,
    pub summary: String,
    pub worktree_context: HistoryRewriteWorktreeContext,
    /// Operation that produced this proposal.
    pub operation_kind: String,
    /// Ref being moved or rewritten.
    pub target_ref: HistoryRewriteRefId,
    /// Old ref position (before the move).
    pub old_position_ref: HistoryRewriteRefId,
    /// New ref position (after the move).
    pub new_position_ref: HistoryRewriteRefId,
    /// True when this update requires a force-move.
    pub force_move_required: bool,
    /// Active blocks (zero or more).
    pub blocks: Vec<HistoryRewriteBlock>,
    /// Next safe paths offered when the proposal is blocked.
    pub next_safe_paths: Vec<HistoryRewriteNextSafePath>,
    pub recovery_posture: HistoryRewriteRecoveryPosture,
    pub consumer_surfaces: Vec<String>,
    pub support_export: HistoryRewriteSupportExport,
    pub audit_events: Vec<HistoryRewriteAuditEvent>,
    pub minted_at: String,
    pub updated_at: String,
}

/// Discriminated union for the history-rewrite record family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "record_kind", rename_all = "snake_case")]
pub enum HistoryRewriteRecord {
    /// Conflict-session record.
    #[serde(rename = "history_rewrite_conflict_session_record")]
    ConflictSession(ConflictSessionRecord),
    /// Sequence-edit-session record.
    #[serde(rename = "history_rewrite_sequence_edit_session_record")]
    SequenceEditSession(SequenceEditSessionRecord),
    /// Stash-entry record.
    #[serde(rename = "history_rewrite_stash_entry_record")]
    StashEntry(StashEntryRecord),
    /// Recovery-checkpoint record.
    #[serde(rename = "history_rewrite_recovery_checkpoint_record")]
    RecoveryCheckpoint(RecoveryCheckpointRecord),
    /// Ref-update proposal record.
    #[serde(rename = "history_rewrite_ref_update_proposal_record")]
    RefUpdateProposal(RefUpdateProposalRecord),
}

/// Compact projection consumed by shell, CLI / headless, audit, and support
/// surfaces. Every projection carries the operation, lifecycle, recovery,
/// and next-safe-route truth needed before the user takes the next step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryRewriteProjection {
    pub record_id: String,
    pub record_kind: String,
    pub operation_kind: String,
    pub lifecycle_state: String,
    pub display_label: String,
    pub summary: String,
    pub workspace_ref: String,
    pub worktree_ref: String,
    pub worktree_label: String,
    pub primary_target_ref: String,
    pub primary_target_label: String,
    pub recovery_posture_class: String,
    pub recovery_checkpoint_ref: Option<String>,
    pub destructive_gate_satisfied: bool,
    pub blocks_summary: Vec<String>,
    pub next_safe_path_classes: Vec<String>,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub audit_event_ids: Vec<String>,
    pub redaction_class: String,
    pub raw_path_export_allowed: bool,
    pub raw_branch_name_export_allowed: bool,
    pub raw_patch_body_export_allowed: bool,
    pub raw_reflog_body_export_allowed: bool,
    pub raw_stash_body_export_allowed: bool,
}

impl HistoryRewriteRecord {
    /// Validates the record against the beta contract.
    ///
    /// # Errors
    ///
    /// Returns [`HistoryRewriteValidationError`] when any frozen guarantee is
    /// violated.
    pub fn validate(&self) -> Result<(), HistoryRewriteValidationError> {
        match self {
            Self::ConflictSession(record) => validate_conflict_session(record),
            Self::SequenceEditSession(record) => validate_sequence_edit_session(record),
            Self::StashEntry(record) => validate_stash_entry(record),
            Self::RecoveryCheckpoint(record) => validate_recovery_checkpoint(record),
            Self::RefUpdateProposal(record) => validate_ref_update_proposal(record),
        }
    }

    /// Projects the record into the shared surface row.
    pub fn project(&self) -> HistoryRewriteProjection {
        match self {
            Self::ConflictSession(record) => project_conflict_session(record),
            Self::SequenceEditSession(record) => project_sequence_edit_session(record),
            Self::StashEntry(record) => project_stash_entry(record),
            Self::RecoveryCheckpoint(record) => project_recovery_checkpoint(record),
            Self::RefUpdateProposal(record) => project_ref_update_proposal(record),
        }
    }

    /// Returns the stable record id.
    pub fn record_id(&self) -> &str {
        match self {
            Self::ConflictSession(record) => &record.conflict_session_id,
            Self::SequenceEditSession(record) => &record.sequence_edit_session_id,
            Self::StashEntry(record) => &record.stash_entry_id,
            Self::RecoveryCheckpoint(record) => &record.recovery_checkpoint_id,
            Self::RefUpdateProposal(record) => &record.ref_update_proposal_id,
        }
    }
}

/// Validation failure for a history-rewrite record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryRewriteValidationError {
    message: String,
}

impl HistoryRewriteValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for HistoryRewriteValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "history-rewrite validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for HistoryRewriteValidationError {}

/// Error returned when a history-rewrite JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryRewriteError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the beta history-rewrite contract.
    Validation(HistoryRewriteValidationError),
}

impl HistoryRewriteError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for HistoryRewriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "history-rewrite JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for HistoryRewriteError {}

/// Parses and validates a beta history-rewrite JSON payload, returning the
/// shared projection.
pub fn project_history_rewrite_record(
    payload: &str,
) -> Result<HistoryRewriteProjection, HistoryRewriteError> {
    let record: HistoryRewriteRecord =
        serde_json::from_str(payload).map_err(|err| HistoryRewriteError::Json(err.to_string()))?;
    record.validate().map_err(HistoryRewriteError::Validation)?;
    Ok(record.project())
}

/// Parses and validates a beta history-rewrite JSON payload, returning the
/// raw record.
pub fn parse_history_rewrite_record(
    payload: &str,
) -> Result<HistoryRewriteRecord, HistoryRewriteError> {
    let record: HistoryRewriteRecord =
        serde_json::from_str(payload).map_err(|err| HistoryRewriteError::Json(err.to_string()))?;
    record.validate().map_err(HistoryRewriteError::Validation)?;
    Ok(record)
}

fn validate_conflict_session(
    record: &ConflictSessionRecord,
) -> Result<(), HistoryRewriteValidationError> {
    require_schema_version(record.schema_version)?;
    require_non_empty("conflict_session_id", &record.conflict_session_id)?;
    require_one_of(
        "operation_kind",
        HISTORY_REWRITE_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    require_one_of(
        "lifecycle_state",
        CONFLICT_SESSION_LIFECYCLE_STATES,
        &record.lifecycle_state,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;
    require_non_empty("updated_at", &record.updated_at)?;
    require_non_empty("raw_todo_text_ref", &record.raw_todo_text_ref)?;
    require_non_empty("ref_update_proposal_ref", &record.ref_update_proposal_ref)?;
    validate_worktree_context(&record.worktree_context)?;
    validate_ref_id("onto_ref", &record.onto_ref)?;
    validate_ref_id("source_ref", &record.source_ref)?;
    require_non_empty_list("conflicted_path_tokens", &record.conflicted_path_tokens)?;
    require_unique("conflicted_path_tokens", &record.conflicted_path_tokens)?;
    require_unique("available_actions", &record.available_actions)?;
    for action in &record.available_actions {
        require_one_of("available_actions[]", CONFLICT_ACTION_CLASSES, action)?;
    }
    validate_recovery_posture(&record.recovery_posture)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    for event in &record.audit_events {
        validate_audit_event(event)?;
    }
    // Continuing or skipping is destructive: posture must satisfy gate.
    if matches!(
        record.lifecycle_state.as_str(),
        "continuing_after_resolution" | "skipped_conflicted_step" | "completed_committed"
    ) && !record.recovery_posture.satisfies_destructive_gate()
    {
        return Err(HistoryRewriteValidationError::new(
            "conflict session continuing/skipping/completing requires a recovery checkpoint or explicit reflog-only disclosure",
        ));
    }
    Ok(())
}

fn validate_sequence_edit_session(
    record: &SequenceEditSessionRecord,
) -> Result<(), HistoryRewriteValidationError> {
    require_schema_version(record.schema_version)?;
    require_non_empty("sequence_edit_session_id", &record.sequence_edit_session_id)?;
    require_one_of(
        "operation_kind",
        HISTORY_REWRITE_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    require_one_of(
        "lifecycle_state",
        SEQUENCE_EDIT_SESSION_LIFECYCLE_STATES,
        &record.lifecycle_state,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;
    require_non_empty("updated_at", &record.updated_at)?;
    require_non_empty("raw_todo_text_ref", &record.raw_todo_text_ref)?;
    require_non_empty("ref_update_proposal_ref", &record.ref_update_proposal_ref)?;
    validate_worktree_context(&record.worktree_context)?;
    validate_ref_id("onto_ref", &record.onto_ref)?;
    validate_ref_id("head_ref", &record.head_ref)?;
    if record.steps.is_empty() {
        return Err(HistoryRewriteValidationError::new(
            "sequence-edit session must list at least one step",
        ));
    }
    let mut seen_ordinals: BTreeSet<u32> = BTreeSet::new();
    let mut current_count = 0usize;
    for step in &record.steps {
        if !seen_ordinals.insert(step.ordinal) {
            return Err(HistoryRewriteValidationError::new(format!(
                "steps contains a duplicate ordinal: {}",
                step.ordinal
            )));
        }
        require_one_of("steps[].verb", SEQUENCE_EDIT_VERBS, &step.verb)?;
        validate_ref_id("steps[].target_ref", &step.target_ref)?;
        require_non_empty("steps[].display_label", &step.display_label)?;
        if step.current_step {
            current_count += 1;
        }
    }
    if current_count > 1 {
        return Err(HistoryRewriteValidationError::new(
            "sequence-edit session must mark at most one step as current_step",
        ));
    }
    validate_recovery_posture(&record.recovery_posture)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    for event in &record.audit_events {
        validate_audit_event(event)?;
    }
    if matches!(
        record.lifecycle_state.as_str(),
        "running" | "completed_admitted"
    ) && !record.recovery_posture.satisfies_destructive_gate()
    {
        return Err(HistoryRewriteValidationError::new(
            "sequence-edit running/completed requires a recovery checkpoint or explicit reflog-only disclosure",
        ));
    }
    Ok(())
}

fn validate_stash_entry(record: &StashEntryRecord) -> Result<(), HistoryRewriteValidationError> {
    require_schema_version(record.schema_version)?;
    require_non_empty("stash_entry_id", &record.stash_entry_id)?;
    require_one_of(
        "lifecycle_state",
        STASH_ENTRY_LIFECYCLE_STATES,
        &record.lifecycle_state,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;
    require_non_empty("updated_at", &record.updated_at)?;
    validate_worktree_context(&record.worktree_context)?;
    validate_ref_id("stash_handle_ref", &record.stash_handle_ref)?;
    validate_ref_id("base_ref", &record.base_ref)?;
    require_unique("captured_path_tokens", &record.captured_path_tokens)?;
    if let Some(promoted) = &record.promoted_branch_ref {
        validate_ref_id("promoted_branch_ref", promoted)?;
    }
    if record.lifecycle_state == "promoted_to_branch" && record.promoted_branch_ref.is_none() {
        return Err(HistoryRewriteValidationError::new(
            "stash promoted_to_branch requires a non-null promoted_branch_ref",
        ));
    }
    if record.lifecycle_state == "applied_with_conflict" && record.conflict_session_ref.is_none() {
        return Err(HistoryRewriteValidationError::new(
            "stash applied_with_conflict requires a non-null conflict_session_ref",
        ));
    }
    validate_recovery_posture(&record.recovery_posture)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    for event in &record.audit_events {
        validate_audit_event(event)?;
    }
    if matches!(
        record.lifecycle_state.as_str(),
        "applied_popped" | "dropped"
    ) && !record.recovery_posture.satisfies_destructive_gate()
    {
        return Err(HistoryRewriteValidationError::new(
            "stash applied_popped/dropped requires a recovery checkpoint or explicit reflog-only disclosure",
        ));
    }
    Ok(())
}

fn validate_recovery_checkpoint(
    record: &RecoveryCheckpointRecord,
) -> Result<(), HistoryRewriteValidationError> {
    require_schema_version(record.schema_version)?;
    require_non_empty("recovery_checkpoint_id", &record.recovery_checkpoint_id)?;
    require_one_of(
        "lifecycle_state",
        RECOVERY_CHECKPOINT_LIFECYCLE_STATES,
        &record.lifecycle_state,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("retention_class", &record.retention_class)?;
    require_non_empty("restore_command_id", &record.restore_command_id)?;
    require_non_empty("captured_at", &record.captured_at)?;
    require_non_empty("updated_at", &record.updated_at)?;
    require_one_of(
        "operation_kind",
        HISTORY_REWRITE_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    validate_worktree_context(&record.worktree_context)?;
    if record.captured_refs.is_empty() {
        return Err(HistoryRewriteValidationError::new(
            "recovery checkpoint must capture at least one ref",
        ));
    }
    let mut seen_refs: BTreeSet<&str> = BTreeSet::new();
    for ref_id in &record.captured_refs {
        validate_ref_id("captured_refs[]", ref_id)?;
        if !seen_refs.insert(ref_id.ref_id.as_str()) {
            return Err(HistoryRewriteValidationError::new(format!(
                "captured_refs contains a duplicate ref_id: {}",
                ref_id.ref_id
            )));
        }
    }
    require_unique("referencing_session_refs", &record.referencing_session_refs)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    for event in &record.audit_events {
        validate_audit_event(event)?;
    }
    Ok(())
}

fn validate_ref_update_proposal(
    record: &RefUpdateProposalRecord,
) -> Result<(), HistoryRewriteValidationError> {
    require_schema_version(record.schema_version)?;
    require_non_empty("ref_update_proposal_id", &record.ref_update_proposal_id)?;
    require_one_of(
        "lifecycle_state",
        REF_UPDATE_PROPOSAL_LIFECYCLE_STATES,
        &record.lifecycle_state,
    )?;
    require_one_of(
        "operation_kind",
        HISTORY_REWRITE_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;
    require_non_empty("updated_at", &record.updated_at)?;
    validate_worktree_context(&record.worktree_context)?;
    validate_ref_id("target_ref", &record.target_ref)?;
    validate_ref_id("old_position_ref", &record.old_position_ref)?;
    validate_ref_id("new_position_ref", &record.new_position_ref)?;
    let mut seen_block_ids: BTreeSet<&str> = BTreeSet::new();
    for block in &record.blocks {
        require_one_of(
            "blocks[].block_class",
            REF_UPDATE_BLOCK_CLASSES,
            &block.block_class,
        )?;
        require_non_empty("blocks[].block_id", &block.block_id)?;
        require_non_empty("blocks[].display_label", &block.display_label)?;
        if !seen_block_ids.insert(block.block_id.as_str()) {
            return Err(HistoryRewriteValidationError::new(format!(
                "blocks contains a duplicate block_id: {}",
                block.block_id
            )));
        }
        for path in &block.next_safe_paths {
            validate_next_safe_path("blocks[].next_safe_paths[]", path)?;
        }
        if block.block_class != "no_block" && block.next_safe_paths.is_empty() {
            return Err(HistoryRewriteValidationError::new(
                "an active block must expose at least one next-safe path",
            ));
        }
    }
    for path in &record.next_safe_paths {
        validate_next_safe_path("next_safe_paths[]", path)?;
    }
    let proposal_is_blocked = matches!(
        record.lifecycle_state.as_str(),
        "blocked_protected_branch" | "blocked_policy" | "blocked_collaboration"
    );
    let any_blocking_block = record
        .blocks
        .iter()
        .any(|block| block.block_class != "no_block");
    if proposal_is_blocked && !any_blocking_block {
        return Err(HistoryRewriteValidationError::new(
            "blocked lifecycle states must record at least one non-`no_block` block",
        ));
    }
    if proposal_is_blocked && record.next_safe_paths.is_empty() {
        return Err(HistoryRewriteValidationError::new(
            "blocked proposals must publish at least one next-safe path",
        ));
    }
    if matches!(
        record.lifecycle_state.as_str(),
        "ready_to_apply" | "applied"
    ) {
        if !record.recovery_posture.satisfies_destructive_gate() {
            return Err(HistoryRewriteValidationError::new(
                "ready_to_apply/applied proposals require a recovery checkpoint or explicit reflog-only disclosure",
            ));
        }
        if any_blocking_block {
            return Err(HistoryRewriteValidationError::new(
                "ready_to_apply/applied proposals must clear every block before apply",
            ));
        }
    }
    if matches!(record.lifecycle_state.as_str(), "applied") && record.force_move_required {
        // Force-moves require a checkpoint, never a reflog-only disclosure.
        if record.recovery_posture.posture_class != "recovery_checkpoint_captured" {
            return Err(HistoryRewriteValidationError::new(
                "applied force-move proposals require a captured recovery checkpoint",
            ));
        }
    }
    validate_recovery_posture(&record.recovery_posture)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    for event in &record.audit_events {
        validate_audit_event(event)?;
    }
    Ok(())
}

fn validate_worktree_context(
    context: &HistoryRewriteWorktreeContext,
) -> Result<(), HistoryRewriteValidationError> {
    require_non_empty("worktree_context.workspace_ref", &context.workspace_ref)?;
    require_non_empty("worktree_context.worktree_ref", &context.worktree_ref)?;
    require_non_empty("worktree_context.worktree_label", &context.worktree_label)?;
    Ok(())
}

fn validate_ref_id(
    label: &str,
    ref_id: &HistoryRewriteRefId,
) -> Result<(), HistoryRewriteValidationError> {
    require_non_empty(&format!("{label}.ref_id"), &ref_id.ref_id)?;
    require_non_empty(&format!("{label}.ref_class"), &ref_id.ref_class)?;
    require_non_empty(&format!("{label}.display_label"), &ref_id.display_label)?;
    Ok(())
}

fn validate_recovery_posture(
    posture: &HistoryRewriteRecoveryPosture,
) -> Result<(), HistoryRewriteValidationError> {
    require_one_of(
        "recovery_posture.posture_class",
        RECOVERY_POSTURE_CLASSES,
        &posture.posture_class,
    )?;
    require_non_empty(
        "recovery_posture.disclosure_label",
        &posture.disclosure_label,
    )?;
    if posture.posture_class == "recovery_checkpoint_captured"
        && (!posture.checkpoint_captured || posture.recovery_checkpoint_ref.is_none())
    {
        return Err(HistoryRewriteValidationError::new(
            "recovery_checkpoint_captured posture requires checkpoint_captured=true and a non-null recovery_checkpoint_ref",
        ));
    }
    if posture.posture_class == "reflog_only_disclosure_acknowledged"
        && !posture.reflog_only_disclosure_acknowledged
    {
        return Err(HistoryRewriteValidationError::new(
            "reflog_only_disclosure_acknowledged posture requires reflog_only_disclosure_acknowledged=true",
        ));
    }
    Ok(())
}

fn validate_next_safe_path(
    label: &str,
    path: &HistoryRewriteNextSafePath,
) -> Result<(), HistoryRewriteValidationError> {
    require_one_of(
        &format!("{label}.path_class"),
        NEXT_SAFE_PATH_CLASSES,
        &path.path_class,
    )?;
    require_non_empty(&format!("{label}.command_id"), &path.command_id)?;
    require_non_empty(&format!("{label}.display_label"), &path.display_label)?;
    if path.path_class == "no_safe_path_blocked_requires_human" && path.preserves_protected_refs {
        // A blocked-requires-human path must not pretend to preserve protected refs.
        return Err(HistoryRewriteValidationError::new(format!(
            "{label} no_safe_path_blocked_requires_human must set preserves_protected_refs=false"
        )));
    }
    if let Some(ref_id) = &path.alternate_ref {
        validate_ref_id(&format!("{label}.alternate_ref"), ref_id)?;
    }
    Ok(())
}

fn validate_consumer_surfaces(surfaces: &[String]) -> Result<(), HistoryRewriteValidationError> {
    if surfaces.is_empty() {
        return Err(HistoryRewriteValidationError::new(
            "consumer_surfaces must list at least one consumer surface",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for surface in surfaces {
        require_one_of(
            "consumer_surfaces[]",
            HISTORY_REWRITE_CONSUMER_SURFACES,
            surface,
        )?;
    }
    let has_support_export = surfaces.iter().any(|surface| surface == "support_export");
    let has_audit_lane = surfaces.iter().any(|surface| surface == "audit_lane");
    if !has_support_export || !has_audit_lane {
        return Err(HistoryRewriteValidationError::new(
            "consumer_surfaces must include both support_export and audit_lane",
        ));
    }
    Ok(())
}

fn validate_support_export(
    export: &HistoryRewriteSupportExport,
) -> Result<(), HistoryRewriteValidationError> {
    require_non_empty("support_export.redaction_class", &export.redaction_class)?;
    if export.raw_path_export_allowed
        || export.raw_branch_name_export_allowed
        || export.raw_patch_body_export_allowed
        || export.raw_reflog_body_export_allowed
        || export.raw_stash_body_export_allowed
    {
        return Err(HistoryRewriteValidationError::new(
            "support_export must keep raw_*_export_allowed false",
        ));
    }
    require_unique(
        "support_export.export_packet_refs",
        &export.export_packet_refs,
    )?;
    Ok(())
}

fn validate_audit_event(
    event: &HistoryRewriteAuditEvent,
) -> Result<(), HistoryRewriteValidationError> {
    require_one_of(
        "audit_events[].event_id",
        HISTORY_REWRITE_AUDIT_EVENTS,
        &event.event_id,
    )?;
    require_non_empty("audit_events[].session_ref", &event.session_ref)?;
    require_non_empty("audit_events[].summary_label", &event.summary_label)?;
    require_non_empty("audit_events[].occurred_at", &event.occurred_at)?;
    if let Some(class) = &event.next_safe_path_class {
        require_one_of(
            "audit_events[].next_safe_path_class",
            NEXT_SAFE_PATH_CLASSES,
            class,
        )?;
    }
    if matches!(
        event.event_id.as_str(),
        "next_safe_path_offered" | "next_safe_path_accepted"
    ) && event.next_safe_path_class.is_none()
    {
        return Err(HistoryRewriteValidationError::new(
            "next_safe_path_* audit events must cite a next_safe_path_class",
        ));
    }
    Ok(())
}

fn project_conflict_session(record: &ConflictSessionRecord) -> HistoryRewriteProjection {
    HistoryRewriteProjection {
        record_id: record.conflict_session_id.clone(),
        record_kind: CONFLICT_SESSION_RECORD_KIND.to_string(),
        operation_kind: record.operation_kind.clone(),
        lifecycle_state: record.lifecycle_state.clone(),
        display_label: record.display_label.clone(),
        summary: record.summary.clone(),
        workspace_ref: record.worktree_context.workspace_ref.clone(),
        worktree_ref: record.worktree_context.worktree_ref.clone(),
        worktree_label: record.worktree_context.worktree_label.clone(),
        primary_target_ref: record.onto_ref.ref_id.clone(),
        primary_target_label: record.onto_ref.display_label.clone(),
        recovery_posture_class: record.recovery_posture.posture_class.clone(),
        recovery_checkpoint_ref: record.recovery_posture.recovery_checkpoint_ref.clone(),
        destructive_gate_satisfied: record.recovery_posture.satisfies_destructive_gate(),
        blocks_summary: Vec::new(),
        next_safe_path_classes: Vec::new(),
        consumer_surfaces: record.consumer_surfaces.clone(),
        support_export_refs: record.support_export.export_packet_refs.clone(),
        audit_event_ids: record
            .audit_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        redaction_class: record.support_export.redaction_class.clone(),
        raw_path_export_allowed: record.support_export.raw_path_export_allowed,
        raw_branch_name_export_allowed: record.support_export.raw_branch_name_export_allowed,
        raw_patch_body_export_allowed: record.support_export.raw_patch_body_export_allowed,
        raw_reflog_body_export_allowed: record.support_export.raw_reflog_body_export_allowed,
        raw_stash_body_export_allowed: record.support_export.raw_stash_body_export_allowed,
    }
}

fn project_sequence_edit_session(record: &SequenceEditSessionRecord) -> HistoryRewriteProjection {
    HistoryRewriteProjection {
        record_id: record.sequence_edit_session_id.clone(),
        record_kind: SEQUENCE_EDIT_SESSION_RECORD_KIND.to_string(),
        operation_kind: record.operation_kind.clone(),
        lifecycle_state: record.lifecycle_state.clone(),
        display_label: record.display_label.clone(),
        summary: record.summary.clone(),
        workspace_ref: record.worktree_context.workspace_ref.clone(),
        worktree_ref: record.worktree_context.worktree_ref.clone(),
        worktree_label: record.worktree_context.worktree_label.clone(),
        primary_target_ref: record.onto_ref.ref_id.clone(),
        primary_target_label: record.onto_ref.display_label.clone(),
        recovery_posture_class: record.recovery_posture.posture_class.clone(),
        recovery_checkpoint_ref: record.recovery_posture.recovery_checkpoint_ref.clone(),
        destructive_gate_satisfied: record.recovery_posture.satisfies_destructive_gate(),
        blocks_summary: Vec::new(),
        next_safe_path_classes: Vec::new(),
        consumer_surfaces: record.consumer_surfaces.clone(),
        support_export_refs: record.support_export.export_packet_refs.clone(),
        audit_event_ids: record
            .audit_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        redaction_class: record.support_export.redaction_class.clone(),
        raw_path_export_allowed: record.support_export.raw_path_export_allowed,
        raw_branch_name_export_allowed: record.support_export.raw_branch_name_export_allowed,
        raw_patch_body_export_allowed: record.support_export.raw_patch_body_export_allowed,
        raw_reflog_body_export_allowed: record.support_export.raw_reflog_body_export_allowed,
        raw_stash_body_export_allowed: record.support_export.raw_stash_body_export_allowed,
    }
}

fn project_stash_entry(record: &StashEntryRecord) -> HistoryRewriteProjection {
    HistoryRewriteProjection {
        record_id: record.stash_entry_id.clone(),
        record_kind: STASH_ENTRY_RECORD_KIND.to_string(),
        operation_kind: "stash_apply".to_string(),
        lifecycle_state: record.lifecycle_state.clone(),
        display_label: record.display_label.clone(),
        summary: record.summary.clone(),
        workspace_ref: record.worktree_context.workspace_ref.clone(),
        worktree_ref: record.worktree_context.worktree_ref.clone(),
        worktree_label: record.worktree_context.worktree_label.clone(),
        primary_target_ref: record.stash_handle_ref.ref_id.clone(),
        primary_target_label: record.stash_handle_ref.display_label.clone(),
        recovery_posture_class: record.recovery_posture.posture_class.clone(),
        recovery_checkpoint_ref: record.recovery_posture.recovery_checkpoint_ref.clone(),
        destructive_gate_satisfied: record.recovery_posture.satisfies_destructive_gate(),
        blocks_summary: Vec::new(),
        next_safe_path_classes: Vec::new(),
        consumer_surfaces: record.consumer_surfaces.clone(),
        support_export_refs: record.support_export.export_packet_refs.clone(),
        audit_event_ids: record
            .audit_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        redaction_class: record.support_export.redaction_class.clone(),
        raw_path_export_allowed: record.support_export.raw_path_export_allowed,
        raw_branch_name_export_allowed: record.support_export.raw_branch_name_export_allowed,
        raw_patch_body_export_allowed: record.support_export.raw_patch_body_export_allowed,
        raw_reflog_body_export_allowed: record.support_export.raw_reflog_body_export_allowed,
        raw_stash_body_export_allowed: record.support_export.raw_stash_body_export_allowed,
    }
}

fn project_recovery_checkpoint(record: &RecoveryCheckpointRecord) -> HistoryRewriteProjection {
    let primary_target =
        record
            .captured_refs
            .first()
            .cloned()
            .unwrap_or_else(|| HistoryRewriteRefId {
                ref_id: format!("{}.captured", record.recovery_checkpoint_id),
                ref_class: "checkpoint".to_string(),
                display_label: record.display_label.clone(),
            });
    HistoryRewriteProjection {
        record_id: record.recovery_checkpoint_id.clone(),
        record_kind: RECOVERY_CHECKPOINT_RECORD_KIND.to_string(),
        operation_kind: record.operation_kind.clone(),
        lifecycle_state: record.lifecycle_state.clone(),
        display_label: record.display_label.clone(),
        summary: record.summary.clone(),
        workspace_ref: record.worktree_context.workspace_ref.clone(),
        worktree_ref: record.worktree_context.worktree_ref.clone(),
        worktree_label: record.worktree_context.worktree_label.clone(),
        primary_target_ref: primary_target.ref_id,
        primary_target_label: primary_target.display_label,
        recovery_posture_class: "recovery_checkpoint_captured".to_string(),
        recovery_checkpoint_ref: Some(record.recovery_checkpoint_id.clone()),
        destructive_gate_satisfied: matches!(
            record.lifecycle_state.as_str(),
            "captured_ready_to_restore" | "restored"
        ),
        blocks_summary: Vec::new(),
        next_safe_path_classes: Vec::new(),
        consumer_surfaces: record.consumer_surfaces.clone(),
        support_export_refs: record.support_export.export_packet_refs.clone(),
        audit_event_ids: record
            .audit_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        redaction_class: record.support_export.redaction_class.clone(),
        raw_path_export_allowed: record.support_export.raw_path_export_allowed,
        raw_branch_name_export_allowed: record.support_export.raw_branch_name_export_allowed,
        raw_patch_body_export_allowed: record.support_export.raw_patch_body_export_allowed,
        raw_reflog_body_export_allowed: record.support_export.raw_reflog_body_export_allowed,
        raw_stash_body_export_allowed: record.support_export.raw_stash_body_export_allowed,
    }
}

fn project_ref_update_proposal(record: &RefUpdateProposalRecord) -> HistoryRewriteProjection {
    let mut blocks_summary: Vec<String> = record
        .blocks
        .iter()
        .filter(|block| block.block_class != "no_block")
        .map(|block| format!("{}:{}", block.block_class, block.block_id))
        .collect();
    blocks_summary.sort();
    blocks_summary.dedup();
    let mut next_safe_path_classes: Vec<String> = record
        .next_safe_paths
        .iter()
        .map(|path| path.path_class.clone())
        .collect();
    next_safe_path_classes.sort();
    next_safe_path_classes.dedup();
    HistoryRewriteProjection {
        record_id: record.ref_update_proposal_id.clone(),
        record_kind: REF_UPDATE_PROPOSAL_RECORD_KIND.to_string(),
        operation_kind: record.operation_kind.clone(),
        lifecycle_state: record.lifecycle_state.clone(),
        display_label: record.display_label.clone(),
        summary: record.summary.clone(),
        workspace_ref: record.worktree_context.workspace_ref.clone(),
        worktree_ref: record.worktree_context.worktree_ref.clone(),
        worktree_label: record.worktree_context.worktree_label.clone(),
        primary_target_ref: record.target_ref.ref_id.clone(),
        primary_target_label: record.target_ref.display_label.clone(),
        recovery_posture_class: record.recovery_posture.posture_class.clone(),
        recovery_checkpoint_ref: record.recovery_posture.recovery_checkpoint_ref.clone(),
        destructive_gate_satisfied: record.recovery_posture.satisfies_destructive_gate(),
        blocks_summary,
        next_safe_path_classes,
        consumer_surfaces: record.consumer_surfaces.clone(),
        support_export_refs: record.support_export.export_packet_refs.clone(),
        audit_event_ids: record
            .audit_events
            .iter()
            .map(|event| event.event_id.clone())
            .collect(),
        redaction_class: record.support_export.redaction_class.clone(),
        raw_path_export_allowed: record.support_export.raw_path_export_allowed,
        raw_branch_name_export_allowed: record.support_export.raw_branch_name_export_allowed,
        raw_patch_body_export_allowed: record.support_export.raw_patch_body_export_allowed,
        raw_reflog_body_export_allowed: record.support_export.raw_reflog_body_export_allowed,
        raw_stash_body_export_allowed: record.support_export.raw_stash_body_export_allowed,
    }
}

fn require_schema_version(actual: u32) -> Result<(), HistoryRewriteValidationError> {
    if actual == HISTORY_REWRITE_BETA_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(HistoryRewriteValidationError::new(format!(
            "schema_version is {actual}, expected {HISTORY_REWRITE_BETA_SCHEMA_VERSION}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), HistoryRewriteValidationError> {
    if value.trim().is_empty() {
        Err(HistoryRewriteValidationError::new(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn require_non_empty_list(
    label: &str,
    values: &[String],
) -> Result<(), HistoryRewriteValidationError> {
    if values.is_empty() {
        Err(HistoryRewriteValidationError::new(format!(
            "{label} must list at least one value"
        )))
    } else {
        for (idx, value) in values.iter().enumerate() {
            require_non_empty(&format!("{label}[{idx}]"), value)?;
        }
        Ok(())
    }
}

fn require_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), HistoryRewriteValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(HistoryRewriteValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(label: &str, values: &[String]) -> Result<(), HistoryRewriteValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(HistoryRewriteValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_REBASE_CONFLICT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m3/history_rewrite_and_stash/rebase_conflict_paused.json"
    ));
    const FIXTURE_INTERACTIVE_REBASE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m3/history_rewrite_and_stash/interactive_rebase_sequence_running.json"
    ));
    const FIXTURE_STASH_APPLY_CONFLICT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m3/history_rewrite_and_stash/stash_apply_with_conflict.json"
    ));
    const FIXTURE_RECOVERY_CHECKPOINT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m3/history_rewrite_and_stash/recovery_checkpoint_captured.json"
    ));
    const FIXTURE_PROTECTED_BRANCH_BLOCKED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m3/history_rewrite_and_stash/ref_update_protected_branch_blocked.json"
    ));

    #[test]
    fn rebase_conflict_fixture_projects() {
        let projection = project_history_rewrite_record(FIXTURE_REBASE_CONFLICT)
            .expect("rebase conflict fixture must project");
        assert_eq!(projection.record_kind, CONFLICT_SESSION_RECORD_KIND);
        assert_eq!(projection.operation_kind, "rebase");
        assert_eq!(projection.lifecycle_state, "active_awaiting_resolution");
        assert!(projection.destructive_gate_satisfied);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export"));
    }

    #[test]
    fn sequence_edit_fixture_projects() {
        let projection = project_history_rewrite_record(FIXTURE_INTERACTIVE_REBASE)
            .expect("interactive rebase fixture must project");
        assert_eq!(projection.record_kind, SEQUENCE_EDIT_SESSION_RECORD_KIND);
        assert_eq!(projection.lifecycle_state, "running");
    }

    #[test]
    fn stash_apply_fixture_projects() {
        let projection = project_history_rewrite_record(FIXTURE_STASH_APPLY_CONFLICT)
            .expect("stash apply fixture must project");
        assert_eq!(projection.record_kind, STASH_ENTRY_RECORD_KIND);
        assert_eq!(projection.lifecycle_state, "applied_with_conflict");
    }

    #[test]
    fn recovery_checkpoint_fixture_projects() {
        let projection = project_history_rewrite_record(FIXTURE_RECOVERY_CHECKPOINT)
            .expect("recovery checkpoint fixture must project");
        assert_eq!(projection.record_kind, RECOVERY_CHECKPOINT_RECORD_KIND);
        assert_eq!(projection.lifecycle_state, "captured_ready_to_restore");
        assert!(projection.destructive_gate_satisfied);
    }

    #[test]
    fn protected_branch_block_fixture_projects() {
        let projection = project_history_rewrite_record(FIXTURE_PROTECTED_BRANCH_BLOCKED)
            .expect("protected-branch fixture must project");
        assert_eq!(projection.record_kind, REF_UPDATE_PROPOSAL_RECORD_KIND);
        assert_eq!(projection.lifecycle_state, "blocked_protected_branch");
        assert!(projection
            .blocks_summary
            .iter()
            .any(|block| block.starts_with("protected_branch_no_force_push")));
        assert!(projection
            .next_safe_path_classes
            .iter()
            .any(|class| class == "create_temporary_branch"));
        assert!(projection
            .next_safe_path_classes
            .iter()
            .any(|class| class == "open_alternate_worktree"));
    }

    #[test]
    fn rejects_continue_without_recovery_posture() {
        let mut record: ConflictSessionRecord =
            serde_json::from_str(FIXTURE_REBASE_CONFLICT).expect("fixture must parse");
        record.lifecycle_state = "continuing_after_resolution".to_string();
        record.recovery_posture.posture_class = "no_recovery_available_blocks_apply".to_string();
        record.recovery_posture.checkpoint_captured = false;
        record.recovery_posture.recovery_checkpoint_ref = None;
        record.recovery_posture.reflog_only_disclosure_acknowledged = false;
        let err = HistoryRewriteRecord::ConflictSession(record)
            .validate()
            .expect_err("continuing must require recovery posture");
        assert!(err.message().contains("recovery checkpoint"));
    }

    #[test]
    fn rejects_blocked_proposal_without_next_safe_path() {
        let mut record: RefUpdateProposalRecord =
            serde_json::from_str(FIXTURE_PROTECTED_BRANCH_BLOCKED).expect("fixture must parse");
        record.next_safe_paths.clear();
        for block in record.blocks.iter_mut() {
            block.next_safe_paths.clear();
        }
        let err = HistoryRewriteRecord::RefUpdateProposal(record)
            .validate()
            .expect_err("blocked proposals must publish next safe paths");
        assert!(err.message().contains("next-safe"));
    }

    #[test]
    fn rejects_raw_patch_body_export() {
        let mut record: ConflictSessionRecord =
            serde_json::from_str(FIXTURE_REBASE_CONFLICT).expect("fixture must parse");
        record.support_export.raw_patch_body_export_allowed = true;
        let err = HistoryRewriteRecord::ConflictSession(record)
            .validate()
            .expect_err("must reject raw patch body export");
        assert!(err.message().contains("raw_"));
    }

    #[test]
    fn rejects_consumer_list_without_support_export() {
        let mut record: ConflictSessionRecord =
            serde_json::from_str(FIXTURE_REBASE_CONFLICT).expect("fixture must parse");
        record
            .consumer_surfaces
            .retain(|surface| surface != "support_export");
        let err = HistoryRewriteRecord::ConflictSession(record)
            .validate()
            .expect_err("must reject consumer list without support_export");
        assert!(err.message().contains("support_export"));
    }

    #[test]
    fn rejects_force_move_applied_with_reflog_only() {
        let mut record: RefUpdateProposalRecord =
            serde_json::from_str(FIXTURE_PROTECTED_BRANCH_BLOCKED).expect("fixture must parse");
        record.lifecycle_state = "applied".to_string();
        record.force_move_required = true;
        record.blocks.clear();
        record.next_safe_paths.clear();
        record.recovery_posture = HistoryRewriteRecoveryPosture {
            posture_class: "reflog_only_disclosure_acknowledged".to_string(),
            recovery_checkpoint_ref: None,
            checkpoint_captured: false,
            reflog_only_disclosure_acknowledged: true,
            disclosure_label: "Reflog-only fallback acknowledged".to_string(),
        };
        let err = HistoryRewriteRecord::RefUpdateProposal(record)
            .validate()
            .expect_err("force-move apply must require a captured checkpoint");
        assert!(err.message().contains("force-move"));
    }
}
