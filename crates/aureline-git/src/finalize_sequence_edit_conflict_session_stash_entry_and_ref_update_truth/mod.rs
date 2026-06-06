//! Stable risky-VCS truth objects for sequence edit, conflict, stash, ref update, and recovery lanes.
//!
//! This module finalizes the object vocabulary shared by desktop Git surfaces,
//! review panes, CLI/headless entry points, provider handoff, restart restore,
//! and support export. It does not execute Git commands; it validates and
//! projects durable truth packets so risky operations remain attributable,
//! previewable, restartable, and honest about rollback posture.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every stable risky-VCS truth packet.
pub const RISKY_VCS_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RiskyVcsTruthPacket`].
pub const RISKY_VCS_TRUTH_PACKET_RECORD_KIND: &str =
    "review_sequence_edit_conflict_session_stash_entry_ref_update_packet";

/// Stable record-kind tag for [`RiskyVcsConflictSession`].
pub const RISKY_VCS_CONFLICT_SESSION_RECORD_KIND: &str = "review_risky_vcs_conflict_session_object";

/// Stable record-kind tag for [`RiskyVcsSequenceEditSession`].
pub const RISKY_VCS_SEQUENCE_EDIT_SESSION_RECORD_KIND: &str =
    "review_risky_vcs_sequence_edit_session_object";

/// Stable record-kind tag for [`RiskyVcsStashEntry`].
pub const RISKY_VCS_STASH_ENTRY_RECORD_KIND: &str = "review_risky_vcs_stash_entry_object";

/// Stable record-kind tag for [`RiskyVcsRecoveryCheckpoint`].
pub const RISKY_VCS_RECOVERY_CHECKPOINT_RECORD_KIND: &str =
    "review_risky_vcs_recovery_checkpoint_object";

/// Stable record-kind tag for [`RiskyVcsRefUpdateProposal`].
pub const RISKY_VCS_REF_UPDATE_PROPOSAL_RECORD_KIND: &str =
    "review_risky_vcs_ref_update_proposal_object";

/// Stable record-kind tag for [`RiskyVcsCommandBinding`].
pub const RISKY_VCS_COMMAND_BINDING_RECORD_KIND: &str = "review_risky_vcs_command_binding_record";

/// Stable record-kind tag for [`RiskyVcsSupportExport`].
pub const RISKY_VCS_SUPPORT_EXPORT_RECORD_KIND: &str = "review_risky_vcs_support_export_packet";

/// Stable record-kind tag for [`RiskyVcsInspectionRecord`].
pub const RISKY_VCS_INSPECTION_RECORD_KIND: &str = "review_risky_vcs_inspection_record";

/// Closed set of risky VCS operation kinds.
pub const RISKY_VCS_OPERATION_KINDS: &[&str] = &[
    "merge",
    "rebase",
    "interactive_rebase",
    "cherry_pick",
    "revert",
    "reset",
    "stash_apply",
    "stash_pop",
    "stash_drop",
    "branch_from_stash",
    "publish",
];

/// Closed set of conflict-session lifecycle states.
pub const RISKY_VCS_CONFLICT_STATES: &[&str] = &[
    "active_awaiting_resolution",
    "paused_awaiting_user_input",
    "paused_awaiting_external_tool",
    "continuing_after_resolution",
    "aborted_rolled_back",
    "completed_committed",
    "completed_handed_off",
    "downgraded_structured_to_raw",
    "failed_no_changes_made",
];

/// Closed set of conflict resolution modes.
pub const RISKY_VCS_CONFLICT_RESOLUTION_MODES: &[&str] =
    &["structured", "raw", "structured_downgraded_to_raw"];

/// Closed set of sequence-edit lifecycle states.
pub const RISKY_VCS_SEQUENCE_STATES: &[&str] = &[
    "draft_unsaved",
    "saved_ready_to_run",
    "running",
    "paused_for_conflict",
    "paused_for_user_edit",
    "completed_admitted",
    "aborted_rolled_back",
    "failed_no_changes_made",
];

/// Closed set of sequence-edit todo version states.
pub const RISKY_VCS_TODO_VERSION_STATES: &[&str] = &[
    "current",
    "modified_since_review",
    "stale_base",
    "conflict_session_bound",
];

/// Closed set of sequence-edit verbs.
pub const RISKY_VCS_SEQUENCE_VERBS: &[&str] = &[
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

/// Closed set of protected-branch postures.
pub const RISKY_VCS_PROTECTED_BRANCH_POSTURES: &[&str] = &[
    "no_protected_refs",
    "protected_branch_readonly",
    "protected_branch_blocked",
    "policy_lock_active",
];

/// Closed set of stash-entry lifecycle states.
pub const RISKY_VCS_STASH_STATES: &[&str] = &[
    "captured_unapplied",
    "applied_kept",
    "applied_popped",
    "dropped",
    "promoted_to_branch",
    "applied_with_conflict",
];

/// Closed set of stash untracked-state postures.
pub const RISKY_VCS_UNTRACKED_STATE_POSTURES: &[&str] = &[
    "tracked_only",
    "untracked_included",
    "untracked_excluded",
    "untracked_unknown_blocks_apply",
];

/// Closed set of recovery checkpoint lifecycle states.
pub const RISKY_VCS_CHECKPOINT_STATES: &[&str] = &[
    "captured_ready_to_restore",
    "captured_pending_admit",
    "restored",
    "expired_pending_prune",
    "missing_reflog_only_disclosed",
    "missing_blocks_apply",
];

/// Closed set of recovery checkpoint trigger kinds.
pub const RISKY_VCS_CHECKPOINT_TRIGGER_KINDS: &[&str] = &[
    "before_rebase",
    "before_reset",
    "before_revert",
    "before_cherry_pick",
    "before_merge",
    "before_stash_apply",
    "before_stash_pop",
    "before_stash_drop",
    "before_publish",
];

/// Closed set of restore option classes.
pub const RISKY_VCS_RESTORE_OPTION_CLASSES: &[&str] = &[
    "restore_head_index_worktree",
    "restore_ref_only",
    "restore_to_alternate_worktree",
    "export_patch_bundle",
    "reflog_only_disclosure",
];

/// Closed set of ref-update proposal lifecycle states.
pub const RISKY_VCS_REF_UPDATE_STATES: &[&str] = &[
    "drafted_pending_review",
    "blocked_ambiguous_target",
    "blocked_protected_branch",
    "blocked_policy",
    "blocked_invalidated_approval",
    "ready_to_publish",
    "applied",
    "withdrawn",
];

/// Closed set of divergence summary classes.
pub const RISKY_VCS_DIVERGENCE_CLASSES: &[&str] = &[
    "no_divergence",
    "local_ahead",
    "remote_ahead",
    "diverged_requires_rebase",
    "diverged_requires_merge",
    "unknown_requires_refresh",
];

/// Closed set of approval states.
pub const RISKY_VCS_APPROVAL_STATES: &[&str] = &[
    "approval_not_required",
    "approval_required_outstanding",
    "approved_current",
    "approval_invalidated_by_changes",
];

/// Closed set of check invalidation states.
pub const RISKY_VCS_CHECK_INVALIDATION_STATES: &[&str] = &[
    "checks_current",
    "checks_stale_within_grace",
    "checks_invalidated_blocks_publish",
];

/// Closed set of publish modes.
pub const RISKY_VCS_PUBLISH_MODES: &[&str] = &[
    "local_only",
    "push_branch",
    "force_with_lease",
    "provider_publish",
    "browser_handoff_publish",
];

/// Closed set of target-selection states.
pub const RISKY_VCS_TARGET_SELECTION_STATES: &[&str] =
    &["explicit_target_selected", "ambiguous_target_blocks_apply"];

/// Closed set of command classes.
pub const RISKY_VCS_COMMAND_CLASSES: &[&str] = &[
    "open_conflict_session",
    "open_sequence_editor",
    "preview_diff",
    "capture_checkpoint",
    "restore_checkpoint",
    "apply_stash",
    "pop_stash",
    "drop_stash",
    "branch_from_stash",
    "review_ref_update_proposal",
    "apply_ref_update",
    "request_provider_handoff",
    "export_support_packet",
];

/// Closed set of consumer surfaces.
pub const RISKY_VCS_CONSUMER_SURFACES: &[&str] = &[
    "desktop_ui",
    "review_panel",
    "cli_headless",
    "provider_handoff",
    "restart_restore",
    "support_export",
    "audit_lane",
];

/// One ordered operation in a sequence-edit session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsSequenceOperation {
    /// Zero-based operation ordinal in the persisted todo.
    pub ordinal: u32,
    /// Verb from [`RISKY_VCS_SEQUENCE_VERBS`].
    pub verb: String,
    /// Opaque target revision ref.
    pub target_ref: String,
    /// Redaction-safe label for the operation.
    pub display_label: String,
    /// True when this step is selected by the current session cursor.
    pub current_step: bool,
    /// True when this step has completed in the current run.
    pub completed: bool,
    /// Conflict session caused by this step, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_session_ref: Option<String>,
}

/// Local or remote ref position quoted by a ref-update proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsRefPosition {
    /// Opaque ref id.
    pub ref_id: String,
    /// Opaque position id before or after the proposal.
    pub position_ref: String,
    /// Redaction-safe display label.
    pub display_label: String,
}

/// Stable conflict-session object shared by risky VCS surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsConflictSession {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable conflict-session identity.
    pub session_id: String,
    /// Operation kind from [`RISKY_VCS_OPERATION_KINDS`].
    pub operation_kind: String,
    /// Lifecycle state from [`RISKY_VCS_CONFLICT_STATES`].
    pub lifecycle_state: String,
    /// Source repository ref.
    pub repo_ref: String,
    /// Source worktree ref.
    pub worktree_ref: String,
    /// Base revision ref.
    pub base_ref: String,
    /// Ours/current revision ref.
    pub ours_ref: String,
    /// Theirs/incoming revision ref.
    pub theirs_ref: String,
    /// Affected path tokens, never raw paths.
    pub affected_path_tokens: Vec<String>,
    /// Count of unresolved conflict rows or markers.
    pub unresolved_count: u32,
    /// Resolution mode from [`RISKY_VCS_CONFLICT_RESOLUTION_MODES`].
    pub resolution_mode: String,
    /// Previous session ref used for restart lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_session_ref: Option<String>,
    /// Checkpoint ref captured before continuing or aborting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Start timestamp.
    pub started_at: String,
    /// Update timestamp.
    pub updated_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable sequence-edit session for rebase, cherry-pick, and edit-sequence flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsSequenceEditSession {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable sequence-edit identity.
    pub session_id: String,
    /// Operation kind from [`RISKY_VCS_OPERATION_KINDS`].
    pub operation_kind: String,
    /// Lifecycle state from [`RISKY_VCS_SEQUENCE_STATES`].
    pub lifecycle_state: String,
    /// Source repository ref.
    pub repo_ref: String,
    /// Source worktree ref.
    pub worktree_ref: String,
    /// Target ref the sequence replays onto.
    pub target_ref: String,
    /// Ordered operation list.
    pub ordered_operations: Vec<RiskyVcsSequenceOperation>,
    /// Todo/version state from [`RISKY_VCS_TODO_VERSION_STATES`].
    pub todo_version_state: String,
    /// Stable ref to raw todo text.
    pub raw_todo_text_ref: String,
    /// Stable ref to structured cards for the same session.
    pub structured_cards_ref: String,
    /// Protected-branch posture.
    pub protected_branch_posture: String,
    /// Active blocker refs that must resolve before continuation.
    pub unresolved_blocker_refs: Vec<String>,
    /// Recovery checkpoint ref for this sequence.
    pub checkpoint_ref: String,
    /// Start timestamp.
    pub started_at: String,
    /// Update timestamp.
    pub updated_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable stash or shelf entry used by apply, pop, drop, and branch-from flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsStashEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable stash-entry identity.
    pub entry_id: String,
    /// Lifecycle state from [`RISKY_VCS_STASH_STATES`].
    pub lifecycle_state: String,
    /// Creator actor ref.
    pub creator_actor_ref: String,
    /// Redaction-safe stash message.
    pub message_label: String,
    /// Included path tokens, never raw paths.
    pub included_path_tokens: Vec<String>,
    /// True when untracked files were included.
    pub includes_untracked: bool,
    /// Untracked-state posture from [`RISKY_VCS_UNTRACKED_STATE_POSTURES`].
    pub untracked_state_posture: String,
    /// Source repository ref.
    pub source_repo_ref: String,
    /// Source worktree ref.
    pub source_worktree_ref: String,
    /// Checkpoint refs protecting stash actions.
    pub checkpoint_refs: Vec<String>,
    /// Distinct command classes available for this one entry.
    pub available_command_classes: Vec<String>,
    /// Optional conflict session from apply/pop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_session_ref: Option<String>,
    /// Created timestamp.
    pub created_at: String,
    /// Update timestamp.
    pub updated_at: String,
}

/// Stable recovery checkpoint object for destructive mutations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsRecoveryCheckpoint {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable checkpoint identity.
    pub checkpoint_id: String,
    /// Lifecycle state from [`RISKY_VCS_CHECKPOINT_STATES`].
    pub lifecycle_state: String,
    /// Trigger kind from [`RISKY_VCS_CHECKPOINT_TRIGGER_KINDS`].
    pub trigger_kind: String,
    /// Source refs captured by this checkpoint.
    pub source_refs: Vec<String>,
    /// Hash of the resulting root state after capture.
    pub resulting_root_state_hash: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Restore option classes.
    pub restore_options: Vec<String>,
    /// True when a checkpoint was possible.
    pub checkpoint_possible: bool,
    /// True when reflog-only disclosure must be shown before apply.
    pub reflog_only_disclosure_required: bool,
    /// True when the reflog-only disclosure was acknowledged.
    pub reflog_only_disclosure_acknowledged: bool,
    /// Created timestamp.
    pub created_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Stable ref-update proposal for publish and history-edit review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsRefUpdateProposal {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable proposal identity.
    pub proposal_id: String,
    /// Lifecycle state from [`RISKY_VCS_REF_UPDATE_STATES`].
    pub lifecycle_state: String,
    /// Source repository ref.
    pub repo_ref: String,
    /// Source worktree ref.
    pub worktree_ref: String,
    /// Local ref set affected by the proposal.
    pub local_ref_set: Vec<RiskyVcsRefPosition>,
    /// Remote ref set affected by the proposal.
    pub remote_ref_set: Vec<RiskyVcsRefPosition>,
    /// Divergence class from [`RISKY_VCS_DIVERGENCE_CLASSES`].
    pub divergence_summary: String,
    /// Approval state from [`RISKY_VCS_APPROVAL_STATES`].
    pub approval_state: String,
    /// Check invalidation state from [`RISKY_VCS_CHECK_INVALIDATION_STATES`].
    pub check_invalidation_state: String,
    /// Publish mode from [`RISKY_VCS_PUBLISH_MODES`].
    pub publish_mode: String,
    /// Target selection state from [`RISKY_VCS_TARGET_SELECTION_STATES`].
    pub target_selection_state: String,
    /// Protected-branch posture.
    pub protected_branch_posture: String,
    /// Recovery checkpoint ref, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Rollback hint shown in review and support exports.
    pub rollback_hint: String,
    /// Created timestamp.
    pub created_at: String,
    /// Updated timestamp.
    pub updated_at: String,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Command binding that keeps UI, CLI, provider handoff, and support paths on one object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsCommandBinding {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable command id.
    pub command_id: String,
    /// Command class from [`RISKY_VCS_COMMAND_CLASSES`].
    pub command_class: String,
    /// Target object ref.
    pub target_object_ref: String,
    /// Target object kind.
    pub target_object_kind: String,
    /// Consumer surfaces that invoke the command.
    pub consumer_surfaces: Vec<String>,
    /// Optional diff preview ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_preview_ref: Option<String>,
    /// Optional checkpoint ref required by the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// True when the command is actionable.
    pub actionable: bool,
    /// Blocked reasons.
    pub blocked_reasons: Vec<String>,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Redaction-safe support export for the stable risky-VCS packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Support-export identity.
    pub support_export_id: String,
    /// Source schema ref.
    pub source_schema_ref: String,
    /// Reopen context ref.
    pub reopen_context_ref: String,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Object refs included in lineage reconstruction.
    pub lineage_object_refs: Vec<String>,
    /// Redaction class.
    pub redaction_class: String,
    /// True when raw paths may cross the support boundary.
    pub raw_path_export_allowed: bool,
    /// True when raw branch names may cross the support boundary.
    pub raw_branch_name_export_allowed: bool,
    /// True when raw patch bodies may cross the support boundary.
    pub raw_patch_body_export_allowed: bool,
    /// True when raw reflog bodies may cross the support boundary.
    pub raw_reflog_body_export_allowed: bool,
    /// True when raw stash bodies may cross the support boundary.
    pub raw_stash_body_export_allowed: bool,
    /// Redaction-safe summary.
    pub summary_label: String,
}

/// Compact inspection record for CLI and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// True when every object has restart-safe identity and timestamp lineage.
    pub restartable: bool,
    /// True when conflict sessions preserve base/ours/theirs provenance.
    pub conflict_provenance_preserved: bool,
    /// True when sequence raw todo text and structured cards share a session.
    pub sequence_todo_bound: bool,
    /// True when stash entries preserve path scope and untracked posture.
    pub stash_scope_preserved: bool,
    /// True when destructive mutations are checkpointed or explicitly reflog-only.
    pub recovery_truth_explicit: bool,
    /// True when publish/rewrite proposals are diff-first and target-explicit.
    pub ref_update_reviewable: bool,
    /// True when support export can reconstruct object lineage.
    pub support_export_lineage_complete: bool,
    /// Number of conflict-session objects.
    pub conflict_session_count: usize,
    /// Number of sequence-edit objects.
    pub sequence_edit_count: usize,
    /// Number of stash-entry objects.
    pub stash_entry_count: usize,
    /// Number of recovery-checkpoint objects.
    pub recovery_checkpoint_count: usize,
    /// Number of ref-update proposal objects.
    pub ref_update_proposal_count: usize,
}

/// Top-level stable risky-VCS truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsTruthPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Source repository ref.
    pub repo_ref: String,
    /// Source worktree ref.
    pub worktree_ref: String,
    /// Conflict sessions in this packet.
    pub conflict_sessions: Vec<RiskyVcsConflictSession>,
    /// Sequence-edit sessions in this packet.
    pub sequence_edit_sessions: Vec<RiskyVcsSequenceEditSession>,
    /// Stash entries in this packet.
    pub stash_entries: Vec<RiskyVcsStashEntry>,
    /// Recovery checkpoints in this packet.
    pub recovery_checkpoints: Vec<RiskyVcsRecoveryCheckpoint>,
    /// Ref-update proposals in this packet.
    pub ref_update_proposals: Vec<RiskyVcsRefUpdateProposal>,
    /// Command bindings across UI, CLI, provider, restart, and support paths.
    pub command_bindings: Vec<RiskyVcsCommandBinding>,
    /// Support export packet.
    pub support_export: RiskyVcsSupportExport,
    /// Inspection projection.
    pub inspection: RiskyVcsInspectionRecord,
}

impl RiskyVcsTruthPacket {
    /// Validates the packet against the stable risky-VCS truth contract.
    ///
    /// # Errors
    ///
    /// Returns [`RiskyVcsTruthValidationError`] when object truth is missing,
    /// ambiguous, lossy, or unsafe for support export.
    pub fn validate(&self) -> Result<(), RiskyVcsTruthValidationError> {
        validate_packet(self)
    }

    /// Projects the packet into a compact shared surface row.
    pub fn project(&self) -> RiskyVcsTruthProjection {
        RiskyVcsTruthProjection {
            packet_id: self.packet_id.clone(),
            repo_ref: self.repo_ref.clone(),
            worktree_ref: self.worktree_ref.clone(),
            object_count: self.object_refs().len(),
            restartable: self.inspection.restartable,
            conflict_provenance_preserved: self.inspection.conflict_provenance_preserved,
            sequence_todo_bound: self.inspection.sequence_todo_bound,
            stash_scope_preserved: self.inspection.stash_scope_preserved,
            recovery_truth_explicit: self.inspection.recovery_truth_explicit,
            ref_update_reviewable: self.inspection.ref_update_reviewable,
            support_export_lineage_complete: self.inspection.support_export_lineage_complete,
            support_export_id: self.support_export.support_export_id.clone(),
            consumer_surfaces: self.support_export.consumer_surfaces.clone(),
            raw_export_allowed: self.support_export.raw_path_export_allowed
                || self.support_export.raw_branch_name_export_allowed
                || self.support_export.raw_patch_body_export_allowed
                || self.support_export.raw_reflog_body_export_allowed
                || self.support_export.raw_stash_body_export_allowed,
        }
    }

    fn object_refs(&self) -> BTreeSet<String> {
        let mut refs = BTreeSet::new();
        refs.extend(
            self.conflict_sessions
                .iter()
                .map(|record| record.session_id.clone()),
        );
        refs.extend(
            self.sequence_edit_sessions
                .iter()
                .map(|record| record.session_id.clone()),
        );
        refs.extend(
            self.stash_entries
                .iter()
                .map(|record| record.entry_id.clone()),
        );
        refs.extend(
            self.recovery_checkpoints
                .iter()
                .map(|record| record.checkpoint_id.clone()),
        );
        refs.extend(
            self.ref_update_proposals
                .iter()
                .map(|record| record.proposal_id.clone()),
        );
        refs
    }
}

/// Compact projection consumed by UI, CLI/headless, restart, provider, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskyVcsTruthProjection {
    /// Packet id.
    pub packet_id: String,
    /// Repository ref.
    pub repo_ref: String,
    /// Worktree ref.
    pub worktree_ref: String,
    /// Number of durable risky-VCS objects.
    pub object_count: usize,
    /// True when objects can be reopened after restart.
    pub restartable: bool,
    /// True when conflict provenance is preserved.
    pub conflict_provenance_preserved: bool,
    /// True when sequence todo/card truth is bound.
    pub sequence_todo_bound: bool,
    /// True when stash scope is preserved.
    pub stash_scope_preserved: bool,
    /// True when checkpoint/reflog posture is explicit.
    pub recovery_truth_explicit: bool,
    /// True when ref updates remain reviewable.
    pub ref_update_reviewable: bool,
    /// True when support export lineage is complete.
    pub support_export_lineage_complete: bool,
    /// Support export id.
    pub support_export_id: String,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// True when any raw export flag is enabled.
    pub raw_export_allowed: bool,
}

/// Validation failure for stable risky-VCS truth packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskyVcsTruthValidationError {
    message: String,
}

impl RiskyVcsTruthValidationError {
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

impl fmt::Display for RiskyVcsTruthValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "risky VCS truth validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for RiskyVcsTruthValidationError {}

/// Error returned when a stable risky-VCS truth payload cannot be parsed or projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskyVcsTruthError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the stable risky-VCS truth contract.
    Validation(RiskyVcsTruthValidationError),
}

impl RiskyVcsTruthError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for RiskyVcsTruthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "risky VCS truth JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for RiskyVcsTruthError {}

/// Parses and validates a risky-VCS truth JSON payload.
pub fn parse_risky_vcs_truth_packet(
    payload: &str,
) -> Result<RiskyVcsTruthPacket, RiskyVcsTruthError> {
    let packet: RiskyVcsTruthPacket =
        serde_json::from_str(payload).map_err(|err| RiskyVcsTruthError::Json(err.to_string()))?;
    packet.validate().map_err(RiskyVcsTruthError::Validation)?;
    Ok(packet)
}

/// Parses, validates, and projects a risky-VCS truth JSON payload.
pub fn project_risky_vcs_truth_packet(
    payload: &str,
) -> Result<RiskyVcsTruthProjection, RiskyVcsTruthError> {
    Ok(parse_risky_vcs_truth_packet(payload)?.project())
}

fn validate_packet(packet: &RiskyVcsTruthPacket) -> Result<(), RiskyVcsTruthValidationError> {
    require_schema_version(packet.schema_version)?;
    require_const(
        "record_kind",
        RISKY_VCS_TRUTH_PACKET_RECORD_KIND,
        &packet.record_kind,
    )?;
    require_non_empty("packet_id", &packet.packet_id)?;
    require_non_empty("generated_at", &packet.generated_at)?;
    require_non_empty("repo_ref", &packet.repo_ref)?;
    require_non_empty("worktree_ref", &packet.worktree_ref)?;

    for record in &packet.conflict_sessions {
        validate_conflict_session(record, &packet.repo_ref, &packet.worktree_ref)?;
    }
    for record in &packet.sequence_edit_sessions {
        validate_sequence_edit_session(record, &packet.repo_ref, &packet.worktree_ref)?;
    }
    for record in &packet.stash_entries {
        validate_stash_entry(record, &packet.repo_ref, &packet.worktree_ref)?;
    }
    for record in &packet.recovery_checkpoints {
        validate_recovery_checkpoint(record)?;
    }
    for record in &packet.ref_update_proposals {
        validate_ref_update_proposal(record, &packet.repo_ref, &packet.worktree_ref)?;
    }

    let object_refs = packet.object_refs();
    if object_refs.is_empty() {
        return Err(RiskyVcsTruthValidationError::new(
            "packet must include at least one durable risky-VCS object",
        ));
    }

    let checkpoint_refs: BTreeSet<&str> = packet
        .recovery_checkpoints
        .iter()
        .map(|record| record.checkpoint_id.as_str())
        .collect();
    validate_cross_references(packet, &object_refs, &checkpoint_refs)?;
    validate_commands(&packet.command_bindings, &object_refs)?;
    validate_support_export(&packet.support_export, &object_refs)?;
    validate_inspection(packet, &object_refs)?;
    Ok(())
}

fn validate_conflict_session(
    record: &RiskyVcsConflictSession,
    packet_repo_ref: &str,
    packet_worktree_ref: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_const(
        "conflict_sessions[].record_kind",
        RISKY_VCS_CONFLICT_SESSION_RECORD_KIND,
        &record.record_kind,
    )?;
    require_schema_version(record.schema_version)?;
    require_non_empty("conflict_sessions[].session_id", &record.session_id)?;
    require_one_of(
        "conflict_sessions[].operation_kind",
        RISKY_VCS_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    require_one_of(
        "conflict_sessions[].lifecycle_state",
        RISKY_VCS_CONFLICT_STATES,
        &record.lifecycle_state,
    )?;
    require_same(
        "conflict_sessions[].repo_ref",
        packet_repo_ref,
        &record.repo_ref,
    )?;
    require_same(
        "conflict_sessions[].worktree_ref",
        packet_worktree_ref,
        &record.worktree_ref,
    )?;
    for (label, value) in [
        ("base_ref", &record.base_ref),
        ("ours_ref", &record.ours_ref),
        ("theirs_ref", &record.theirs_ref),
        ("started_at", &record.started_at),
        ("updated_at", &record.updated_at),
        ("summary_label", &record.summary_label),
    ] {
        require_non_empty(&format!("conflict_sessions[].{label}"), value)?;
    }
    require_non_empty_list(
        "conflict_sessions[].affected_path_tokens",
        &record.affected_path_tokens,
    )?;
    require_unique(
        "conflict_sessions[].affected_path_tokens",
        &record.affected_path_tokens,
    )?;
    require_one_of(
        "conflict_sessions[].resolution_mode",
        RISKY_VCS_CONFLICT_RESOLUTION_MODES,
        &record.resolution_mode,
    )?;
    if record.lifecycle_state == "continuing_after_resolution"
        && record.recovery_checkpoint_ref.is_none()
    {
        return Err(RiskyVcsTruthValidationError::new(
            "continuing conflict sessions require a recovery_checkpoint_ref",
        ));
    }
    if matches!(
        record.lifecycle_state.as_str(),
        "completed_committed" | "completed_handed_off"
    ) && record.unresolved_count != 0
    {
        return Err(RiskyVcsTruthValidationError::new(
            "completed conflict sessions must have unresolved_count=0",
        ));
    }
    if record.resolution_mode == "structured_downgraded_to_raw"
        && (matches!(
            record.lifecycle_state.as_str(),
            "completed_committed" | "completed_handed_off"
        ) || record.unresolved_count == 0)
    {
        return Err(RiskyVcsTruthValidationError::new(
            "structured_downgraded_to_raw must remain unresolved and not completed",
        ));
    }
    Ok(())
}

fn validate_sequence_edit_session(
    record: &RiskyVcsSequenceEditSession,
    packet_repo_ref: &str,
    packet_worktree_ref: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_const(
        "sequence_edit_sessions[].record_kind",
        RISKY_VCS_SEQUENCE_EDIT_SESSION_RECORD_KIND,
        &record.record_kind,
    )?;
    require_schema_version(record.schema_version)?;
    require_non_empty("sequence_edit_sessions[].session_id", &record.session_id)?;
    require_one_of(
        "sequence_edit_sessions[].operation_kind",
        RISKY_VCS_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    if !matches!(
        record.operation_kind.as_str(),
        "rebase" | "interactive_rebase" | "cherry_pick"
    ) {
        return Err(RiskyVcsTruthValidationError::new(
            "sequence-edit sessions are limited to rebase, interactive_rebase, and cherry_pick",
        ));
    }
    require_one_of(
        "sequence_edit_sessions[].lifecycle_state",
        RISKY_VCS_SEQUENCE_STATES,
        &record.lifecycle_state,
    )?;
    require_same(
        "sequence_edit_sessions[].repo_ref",
        packet_repo_ref,
        &record.repo_ref,
    )?;
    require_same(
        "sequence_edit_sessions[].worktree_ref",
        packet_worktree_ref,
        &record.worktree_ref,
    )?;
    require_non_empty("sequence_edit_sessions[].target_ref", &record.target_ref)?;
    require_non_empty(
        "sequence_edit_sessions[].raw_todo_text_ref",
        &record.raw_todo_text_ref,
    )?;
    require_non_empty(
        "sequence_edit_sessions[].structured_cards_ref",
        &record.structured_cards_ref,
    )?;
    require_non_empty(
        "sequence_edit_sessions[].checkpoint_ref",
        &record.checkpoint_ref,
    )?;
    require_one_of(
        "sequence_edit_sessions[].todo_version_state",
        RISKY_VCS_TODO_VERSION_STATES,
        &record.todo_version_state,
    )?;
    require_one_of(
        "sequence_edit_sessions[].protected_branch_posture",
        RISKY_VCS_PROTECTED_BRANCH_POSTURES,
        &record.protected_branch_posture,
    )?;
    validate_sequence_operations(&record.ordered_operations)?;
    if matches!(
        record.protected_branch_posture.as_str(),
        "protected_branch_blocked" | "policy_lock_active"
    ) && record.unresolved_blocker_refs.is_empty()
    {
        return Err(RiskyVcsTruthValidationError::new(
            "blocked sequence-edit sessions must list unresolved_blocker_refs",
        ));
    }
    if matches!(
        record.lifecycle_state.as_str(),
        "running" | "paused_for_conflict" | "completed_admitted"
    ) && record.checkpoint_ref.trim().is_empty()
    {
        return Err(RiskyVcsTruthValidationError::new(
            "running or completed sequence-edit sessions require checkpoint_ref",
        ));
    }
    require_non_empty("sequence_edit_sessions[].started_at", &record.started_at)?;
    require_non_empty("sequence_edit_sessions[].updated_at", &record.updated_at)?;
    require_non_empty(
        "sequence_edit_sessions[].summary_label",
        &record.summary_label,
    )?;
    Ok(())
}

fn validate_sequence_operations(
    operations: &[RiskyVcsSequenceOperation],
) -> Result<(), RiskyVcsTruthValidationError> {
    if operations.is_empty() {
        return Err(RiskyVcsTruthValidationError::new(
            "sequence-edit sessions must list at least one ordered operation",
        ));
    }
    let mut ordinals = BTreeSet::new();
    let mut current_count = 0usize;
    for operation in operations {
        if !ordinals.insert(operation.ordinal) {
            return Err(RiskyVcsTruthValidationError::new(format!(
                "sequence operation ordinal {} is duplicated",
                operation.ordinal
            )));
        }
        require_one_of(
            "ordered_operations[].verb",
            RISKY_VCS_SEQUENCE_VERBS,
            &operation.verb,
        )?;
        require_non_empty("ordered_operations[].target_ref", &operation.target_ref)?;
        require_non_empty(
            "ordered_operations[].display_label",
            &operation.display_label,
        )?;
        if operation.current_step {
            current_count += 1;
        }
    }
    if current_count > 1 {
        return Err(RiskyVcsTruthValidationError::new(
            "sequence-edit sessions may mark at most one current_step",
        ));
    }
    Ok(())
}

fn validate_stash_entry(
    record: &RiskyVcsStashEntry,
    packet_repo_ref: &str,
    packet_worktree_ref: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_const(
        "stash_entries[].record_kind",
        RISKY_VCS_STASH_ENTRY_RECORD_KIND,
        &record.record_kind,
    )?;
    require_schema_version(record.schema_version)?;
    require_non_empty("stash_entries[].entry_id", &record.entry_id)?;
    require_one_of(
        "stash_entries[].lifecycle_state",
        RISKY_VCS_STASH_STATES,
        &record.lifecycle_state,
    )?;
    require_non_empty(
        "stash_entries[].creator_actor_ref",
        &record.creator_actor_ref,
    )?;
    require_non_empty("stash_entries[].message_label", &record.message_label)?;
    require_non_empty_list(
        "stash_entries[].included_path_tokens",
        &record.included_path_tokens,
    )?;
    require_unique(
        "stash_entries[].included_path_tokens",
        &record.included_path_tokens,
    )?;
    require_one_of(
        "stash_entries[].untracked_state_posture",
        RISKY_VCS_UNTRACKED_STATE_POSTURES,
        &record.untracked_state_posture,
    )?;
    if record.includes_untracked && record.untracked_state_posture != "untracked_included" {
        return Err(RiskyVcsTruthValidationError::new(
            "stash entries with includes_untracked=true must use untracked_included posture",
        ));
    }
    require_same(
        "stash_entries[].source_repo_ref",
        packet_repo_ref,
        &record.source_repo_ref,
    )?;
    require_same(
        "stash_entries[].source_worktree_ref",
        packet_worktree_ref,
        &record.source_worktree_ref,
    )?;
    require_non_empty_list("stash_entries[].checkpoint_refs", &record.checkpoint_refs)?;
    require_unique("stash_entries[].checkpoint_refs", &record.checkpoint_refs)?;
    require_non_empty_list(
        "stash_entries[].available_command_classes",
        &record.available_command_classes,
    )?;
    require_unique(
        "stash_entries[].available_command_classes",
        &record.available_command_classes,
    )?;
    let mut stash_action_count = 0usize;
    for command_class in &record.available_command_classes {
        require_one_of(
            "stash_entries[].available_command_classes[]",
            RISKY_VCS_COMMAND_CLASSES,
            command_class,
        )?;
        if matches!(
            command_class.as_str(),
            "apply_stash" | "pop_stash" | "drop_stash" | "branch_from_stash"
        ) {
            stash_action_count += 1;
        }
    }
    if stash_action_count < 2 {
        return Err(RiskyVcsTruthValidationError::new(
            "stash entries must preserve distinct action classes against one entry",
        ));
    }
    if record.lifecycle_state == "applied_with_conflict" && record.conflict_session_ref.is_none() {
        return Err(RiskyVcsTruthValidationError::new(
            "stash applied_with_conflict requires conflict_session_ref",
        ));
    }
    require_non_empty("stash_entries[].created_at", &record.created_at)?;
    require_non_empty("stash_entries[].updated_at", &record.updated_at)?;
    Ok(())
}

fn validate_recovery_checkpoint(
    record: &RiskyVcsRecoveryCheckpoint,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_const(
        "recovery_checkpoints[].record_kind",
        RISKY_VCS_RECOVERY_CHECKPOINT_RECORD_KIND,
        &record.record_kind,
    )?;
    require_schema_version(record.schema_version)?;
    require_non_empty(
        "recovery_checkpoints[].checkpoint_id",
        &record.checkpoint_id,
    )?;
    require_one_of(
        "recovery_checkpoints[].lifecycle_state",
        RISKY_VCS_CHECKPOINT_STATES,
        &record.lifecycle_state,
    )?;
    require_one_of(
        "recovery_checkpoints[].trigger_kind",
        RISKY_VCS_CHECKPOINT_TRIGGER_KINDS,
        &record.trigger_kind,
    )?;
    require_non_empty_list("recovery_checkpoints[].source_refs", &record.source_refs)?;
    require_unique("recovery_checkpoints[].source_refs", &record.source_refs)?;
    require_non_empty(
        "recovery_checkpoints[].resulting_root_state_hash",
        &record.resulting_root_state_hash,
    )?;
    require_non_empty("recovery_checkpoints[].expires_at", &record.expires_at)?;
    require_non_empty_list(
        "recovery_checkpoints[].restore_options",
        &record.restore_options,
    )?;
    require_unique(
        "recovery_checkpoints[].restore_options",
        &record.restore_options,
    )?;
    for option in &record.restore_options {
        require_one_of(
            "recovery_checkpoints[].restore_options[]",
            RISKY_VCS_RESTORE_OPTION_CLASSES,
            option,
        )?;
    }
    if !record.checkpoint_possible {
        if !record.reflog_only_disclosure_required
            || !record.reflog_only_disclosure_acknowledged
            || !record
                .restore_options
                .iter()
                .any(|option| option == "reflog_only_disclosure")
        {
            return Err(RiskyVcsTruthValidationError::new(
                "impossible checkpoints require acknowledged reflog-only disclosure",
            ));
        }
    }
    if record.lifecycle_state == "missing_blocks_apply"
        && record.reflog_only_disclosure_acknowledged
    {
        return Err(RiskyVcsTruthValidationError::new(
            "missing_blocks_apply cannot also acknowledge reflog-only recovery",
        ));
    }
    require_non_empty("recovery_checkpoints[].created_at", &record.created_at)?;
    require_non_empty(
        "recovery_checkpoints[].summary_label",
        &record.summary_label,
    )?;
    Ok(())
}

fn validate_ref_update_proposal(
    record: &RiskyVcsRefUpdateProposal,
    packet_repo_ref: &str,
    packet_worktree_ref: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_const(
        "ref_update_proposals[].record_kind",
        RISKY_VCS_REF_UPDATE_PROPOSAL_RECORD_KIND,
        &record.record_kind,
    )?;
    require_schema_version(record.schema_version)?;
    require_non_empty("ref_update_proposals[].proposal_id", &record.proposal_id)?;
    require_one_of(
        "ref_update_proposals[].lifecycle_state",
        RISKY_VCS_REF_UPDATE_STATES,
        &record.lifecycle_state,
    )?;
    require_same(
        "ref_update_proposals[].repo_ref",
        packet_repo_ref,
        &record.repo_ref,
    )?;
    require_same(
        "ref_update_proposals[].worktree_ref",
        packet_worktree_ref,
        &record.worktree_ref,
    )?;
    validate_ref_positions(
        "ref_update_proposals[].local_ref_set",
        &record.local_ref_set,
    )?;
    validate_ref_positions(
        "ref_update_proposals[].remote_ref_set",
        &record.remote_ref_set,
    )?;
    require_one_of(
        "ref_update_proposals[].divergence_summary",
        RISKY_VCS_DIVERGENCE_CLASSES,
        &record.divergence_summary,
    )?;
    require_one_of(
        "ref_update_proposals[].approval_state",
        RISKY_VCS_APPROVAL_STATES,
        &record.approval_state,
    )?;
    require_one_of(
        "ref_update_proposals[].check_invalidation_state",
        RISKY_VCS_CHECK_INVALIDATION_STATES,
        &record.check_invalidation_state,
    )?;
    require_one_of(
        "ref_update_proposals[].publish_mode",
        RISKY_VCS_PUBLISH_MODES,
        &record.publish_mode,
    )?;
    require_one_of(
        "ref_update_proposals[].target_selection_state",
        RISKY_VCS_TARGET_SELECTION_STATES,
        &record.target_selection_state,
    )?;
    require_one_of(
        "ref_update_proposals[].protected_branch_posture",
        RISKY_VCS_PROTECTED_BRANCH_POSTURES,
        &record.protected_branch_posture,
    )?;
    require_non_empty(
        "ref_update_proposals[].rollback_hint",
        &record.rollback_hint,
    )?;
    if matches!(
        record.lifecycle_state.as_str(),
        "ready_to_publish" | "applied"
    ) {
        if record.target_selection_state != "explicit_target_selected" {
            return Err(RiskyVcsTruthValidationError::new(
                "ready/applied ref-update proposals require explicit target selection",
            ));
        }
        if record.approval_state == "approval_invalidated_by_changes"
            || record.check_invalidation_state == "checks_invalidated_blocks_publish"
        {
            return Err(RiskyVcsTruthValidationError::new(
                "ready/applied ref-update proposals cannot carry invalidated approvals or checks",
            ));
        }
        if record.rollback_checkpoint_ref.is_none() {
            return Err(RiskyVcsTruthValidationError::new(
                "ready/applied ref-update proposals require rollback_checkpoint_ref",
            ));
        }
    }
    if record.lifecycle_state == "blocked_ambiguous_target"
        && record.target_selection_state != "ambiguous_target_blocks_apply"
    {
        return Err(RiskyVcsTruthValidationError::new(
            "ambiguous-target blocks require ambiguous_target_blocks_apply target state",
        ));
    }
    require_non_empty("ref_update_proposals[].created_at", &record.created_at)?;
    require_non_empty("ref_update_proposals[].updated_at", &record.updated_at)?;
    require_non_empty(
        "ref_update_proposals[].summary_label",
        &record.summary_label,
    )?;
    Ok(())
}

fn validate_ref_positions(
    label: &str,
    positions: &[RiskyVcsRefPosition],
) -> Result<(), RiskyVcsTruthValidationError> {
    if positions.is_empty() {
        return Err(RiskyVcsTruthValidationError::new(format!(
            "{label} must list at least one ref position"
        )));
    }
    let mut refs = BTreeSet::new();
    for position in positions {
        require_non_empty(&format!("{label}[].ref_id"), &position.ref_id)?;
        require_non_empty(&format!("{label}[].position_ref"), &position.position_ref)?;
        require_non_empty(&format!("{label}[].display_label"), &position.display_label)?;
        if !refs.insert(position.ref_id.as_str()) {
            return Err(RiskyVcsTruthValidationError::new(format!(
                "{label} contains duplicate ref_id {}",
                position.ref_id
            )));
        }
    }
    Ok(())
}

fn validate_cross_references(
    packet: &RiskyVcsTruthPacket,
    object_refs: &BTreeSet<String>,
    checkpoint_refs: &BTreeSet<&str>,
) -> Result<(), RiskyVcsTruthValidationError> {
    for sequence in &packet.sequence_edit_sessions {
        if !checkpoint_refs.contains(sequence.checkpoint_ref.as_str()) {
            return Err(RiskyVcsTruthValidationError::new(format!(
                "sequence-edit checkpoint_ref {} does not resolve",
                sequence.checkpoint_ref
            )));
        }
        for operation in &sequence.ordered_operations {
            if let Some(conflict_ref) = &operation.conflict_session_ref {
                require_object_ref(
                    "ordered_operations[].conflict_session_ref",
                    conflict_ref,
                    object_refs,
                )?;
            }
        }
    }
    for stash in &packet.stash_entries {
        for checkpoint_ref in &stash.checkpoint_refs {
            if !checkpoint_refs.contains(checkpoint_ref.as_str()) {
                return Err(RiskyVcsTruthValidationError::new(format!(
                    "stash checkpoint_ref {checkpoint_ref} does not resolve"
                )));
            }
        }
        if let Some(conflict_ref) = &stash.conflict_session_ref {
            require_object_ref(
                "stash_entries[].conflict_session_ref",
                conflict_ref,
                object_refs,
            )?;
        }
    }
    for proposal in &packet.ref_update_proposals {
        if let Some(checkpoint_ref) = &proposal.rollback_checkpoint_ref {
            if !checkpoint_refs.contains(checkpoint_ref.as_str()) {
                return Err(RiskyVcsTruthValidationError::new(format!(
                    "ref-update rollback_checkpoint_ref {checkpoint_ref} does not resolve"
                )));
            }
        }
    }
    for conflict in &packet.conflict_sessions {
        if let Some(checkpoint_ref) = &conflict.recovery_checkpoint_ref {
            if !checkpoint_refs.contains(checkpoint_ref.as_str()) {
                return Err(RiskyVcsTruthValidationError::new(format!(
                    "conflict recovery_checkpoint_ref {checkpoint_ref} does not resolve"
                )));
            }
        }
    }
    Ok(())
}

fn validate_commands(
    commands: &[RiskyVcsCommandBinding],
    object_refs: &BTreeSet<String>,
) -> Result<(), RiskyVcsTruthValidationError> {
    if commands.is_empty() {
        return Err(RiskyVcsTruthValidationError::new(
            "command_bindings must list at least one command",
        ));
    }
    let mut command_ids = BTreeSet::new();
    let mut command_classes_by_target: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for command in commands {
        require_const(
            "command_bindings[].record_kind",
            RISKY_VCS_COMMAND_BINDING_RECORD_KIND,
            &command.record_kind,
        )?;
        require_schema_version(command.schema_version)?;
        require_non_empty("command_bindings[].command_id", &command.command_id)?;
        if !command_ids.insert(command.command_id.as_str()) {
            return Err(RiskyVcsTruthValidationError::new(format!(
                "command_bindings contains duplicate command_id {}",
                command.command_id
            )));
        }
        require_one_of(
            "command_bindings[].command_class",
            RISKY_VCS_COMMAND_CLASSES,
            &command.command_class,
        )?;
        require_object_ref(
            "command_bindings[].target_object_ref",
            &command.target_object_ref,
            object_refs,
        )?;
        require_non_empty(
            "command_bindings[].target_object_kind",
            &command.target_object_kind,
        )?;
        validate_consumer_surfaces(
            "command_bindings[].consumer_surfaces",
            &command.consumer_surfaces,
        )?;
        require_non_empty("command_bindings[].summary_label", &command.summary_label)?;
        command_classes_by_target
            .entry(command.target_object_ref.as_str())
            .or_default()
            .insert(command.command_class.as_str());
    }
    for (target_ref, classes) in command_classes_by_target {
        if classes.contains("apply_stash")
            || classes.contains("pop_stash")
            || classes.contains("drop_stash")
            || classes.contains("branch_from_stash")
        {
            let distinct_stash_actions = [
                "apply_stash",
                "pop_stash",
                "drop_stash",
                "branch_from_stash",
            ]
            .iter()
            .filter(|class| classes.contains(**class))
            .count();
            if distinct_stash_actions < 2 {
                return Err(RiskyVcsTruthValidationError::new(format!(
                    "stash target {target_ref} must keep apply/pop/drop/branch-from distinct"
                )));
            }
        }
    }
    Ok(())
}

fn validate_support_export(
    export: &RiskyVcsSupportExport,
    object_refs: &BTreeSet<String>,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_const(
        "support_export.record_kind",
        RISKY_VCS_SUPPORT_EXPORT_RECORD_KIND,
        &export.record_kind,
    )?;
    require_schema_version(export.schema_version)?;
    require_non_empty(
        "support_export.support_export_id",
        &export.support_export_id,
    )?;
    require_non_empty(
        "support_export.source_schema_ref",
        &export.source_schema_ref,
    )?;
    require_non_empty(
        "support_export.reopen_context_ref",
        &export.reopen_context_ref,
    )?;
    validate_consumer_surfaces(
        "support_export.consumer_surfaces",
        &export.consumer_surfaces,
    )?;
    require_non_empty_list(
        "support_export.lineage_object_refs",
        &export.lineage_object_refs,
    )?;
    let lineage_refs: BTreeSet<&str> = export
        .lineage_object_refs
        .iter()
        .map(String::as_str)
        .collect();
    for object_ref in object_refs {
        if !lineage_refs.contains(object_ref.as_str()) {
            return Err(RiskyVcsTruthValidationError::new(format!(
                "support export lineage is missing object {object_ref}"
            )));
        }
    }
    require_non_empty("support_export.redaction_class", &export.redaction_class)?;
    if export.raw_path_export_allowed
        || export.raw_branch_name_export_allowed
        || export.raw_patch_body_export_allowed
        || export.raw_reflog_body_export_allowed
        || export.raw_stash_body_export_allowed
    {
        return Err(RiskyVcsTruthValidationError::new(
            "support export must keep every raw_*_export_allowed flag false",
        ));
    }
    require_non_empty("support_export.summary_label", &export.summary_label)?;
    Ok(())
}

fn validate_inspection(
    packet: &RiskyVcsTruthPacket,
    object_refs: &BTreeSet<String>,
) -> Result<(), RiskyVcsTruthValidationError> {
    let inspection = &packet.inspection;
    require_const(
        "inspection.record_kind",
        RISKY_VCS_INSPECTION_RECORD_KIND,
        &inspection.record_kind,
    )?;
    require_schema_version(inspection.schema_version)?;
    let expected = derive_inspection(packet, object_refs);
    if *inspection != expected {
        return Err(RiskyVcsTruthValidationError::new(
            "inspection record does not match derived risky-VCS truth",
        ));
    }
    Ok(())
}

fn derive_inspection(
    packet: &RiskyVcsTruthPacket,
    object_refs: &BTreeSet<String>,
) -> RiskyVcsInspectionRecord {
    let lineage_refs: BTreeSet<&str> = packet
        .support_export
        .lineage_object_refs
        .iter()
        .map(String::as_str)
        .collect();
    RiskyVcsInspectionRecord {
        record_kind: RISKY_VCS_INSPECTION_RECORD_KIND.to_string(),
        schema_version: RISKY_VCS_TRUTH_SCHEMA_VERSION,
        restartable: objects_have_lineage(packet),
        conflict_provenance_preserved: packet.conflict_sessions.iter().all(|record| {
            !record.base_ref.trim().is_empty()
                && !record.ours_ref.trim().is_empty()
                && !record.theirs_ref.trim().is_empty()
        }),
        sequence_todo_bound: packet.sequence_edit_sessions.iter().all(|record| {
            !record.raw_todo_text_ref.trim().is_empty()
                && !record.structured_cards_ref.trim().is_empty()
                && record.raw_todo_text_ref.contains(&record.session_id)
                && record.structured_cards_ref.contains(&record.session_id)
        }),
        stash_scope_preserved: packet.stash_entries.iter().all(|record| {
            !record.included_path_tokens.is_empty()
                && !record.untracked_state_posture.trim().is_empty()
                && !record.source_repo_ref.trim().is_empty()
                && !record.source_worktree_ref.trim().is_empty()
        }),
        recovery_truth_explicit: packet.recovery_checkpoints.iter().all(|record| {
            record.checkpoint_possible
                || (record.reflog_only_disclosure_required
                    && record.reflog_only_disclosure_acknowledged)
        }),
        ref_update_reviewable: packet.ref_update_proposals.iter().all(|record| {
            !record.local_ref_set.is_empty()
                && !record.remote_ref_set.is_empty()
                && !record.rollback_hint.trim().is_empty()
                && record.target_selection_state == "explicit_target_selected"
                && record.approval_state != "approval_invalidated_by_changes"
                && record.check_invalidation_state != "checks_invalidated_blocks_publish"
        }),
        support_export_lineage_complete: object_refs
            .iter()
            .all(|object_ref| lineage_refs.contains(object_ref.as_str())),
        conflict_session_count: packet.conflict_sessions.len(),
        sequence_edit_count: packet.sequence_edit_sessions.len(),
        stash_entry_count: packet.stash_entries.len(),
        recovery_checkpoint_count: packet.recovery_checkpoints.len(),
        ref_update_proposal_count: packet.ref_update_proposals.len(),
    }
}

fn objects_have_lineage(packet: &RiskyVcsTruthPacket) -> bool {
    packet
        .conflict_sessions
        .iter()
        .all(|record| !record.session_id.trim().is_empty() && !record.started_at.trim().is_empty())
        && packet.sequence_edit_sessions.iter().all(|record| {
            !record.session_id.trim().is_empty() && !record.started_at.trim().is_empty()
        })
        && packet.stash_entries.iter().all(|record| {
            !record.entry_id.trim().is_empty() && !record.created_at.trim().is_empty()
        })
        && packet.recovery_checkpoints.iter().all(|record| {
            !record.checkpoint_id.trim().is_empty() && !record.created_at.trim().is_empty()
        })
        && packet.ref_update_proposals.iter().all(|record| {
            !record.proposal_id.trim().is_empty() && !record.created_at.trim().is_empty()
        })
}

fn require_schema_version(actual: u32) -> Result<(), RiskyVcsTruthValidationError> {
    if actual == RISKY_VCS_TRUTH_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(RiskyVcsTruthValidationError::new(format!(
            "schema_version is {actual}, expected {RISKY_VCS_TRUTH_SCHEMA_VERSION}"
        )))
    }
}

fn require_const(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(RiskyVcsTruthValidationError::new(format!(
            "{label} is {actual}, expected {expected}"
        )))
    }
}

fn require_same(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(RiskyVcsTruthValidationError::new(format!(
            "{label} is {actual}, expected {expected}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), RiskyVcsTruthValidationError> {
    if value.trim().is_empty() {
        Err(RiskyVcsTruthValidationError::new(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn require_non_empty_list(
    label: &str,
    values: &[String],
) -> Result<(), RiskyVcsTruthValidationError> {
    if values.is_empty() {
        return Err(RiskyVcsTruthValidationError::new(format!(
            "{label} must list at least one value"
        )));
    }
    for value in values {
        require_non_empty(label, value)?;
    }
    Ok(())
}

fn require_unique(label: &str, values: &[String]) -> Result<(), RiskyVcsTruthValidationError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(RiskyVcsTruthValidationError::new(format!(
                "{label} contains duplicate value {value}"
            )));
        }
    }
    Ok(())
}

fn require_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), RiskyVcsTruthValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(RiskyVcsTruthValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_object_ref(
    label: &str,
    value: &str,
    object_refs: &BTreeSet<String>,
) -> Result<(), RiskyVcsTruthValidationError> {
    require_non_empty(label, value)?;
    if object_refs.contains(value) {
        Ok(())
    } else {
        Err(RiskyVcsTruthValidationError::new(format!(
            "{label} {value} does not resolve to an object in this packet"
        )))
    }
}

fn validate_consumer_surfaces(
    label: &str,
    surfaces: &[String],
) -> Result<(), RiskyVcsTruthValidationError> {
    require_non_empty_list(label, surfaces)?;
    require_unique(label, surfaces)?;
    for surface in surfaces {
        require_one_of(label, RISKY_VCS_CONSUMER_SURFACES, surface)?;
    }
    let has_support = surfaces.iter().any(|surface| surface == "support_export");
    let has_audit = surfaces.iter().any(|surface| surface == "audit_lane");
    if !has_support || !has_audit {
        return Err(RiskyVcsTruthValidationError::new(format!(
            "{label} must include support_export and audit_lane"
        )));
    }
    Ok(())
}
