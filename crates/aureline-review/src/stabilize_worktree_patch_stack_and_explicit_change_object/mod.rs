//! Stabilized worktree, patch-stack, and explicit change-object orchestration
//! for stable review lanes.
//!
//! This module owns the bounded beta contract that keeps worktree operations,
//! patch-stack mutations, and change-object orchestration previewable,
//! checkpointed, and rooted in exact repo topology. Every orchestration packet
//! carries repo-root identity, submodule or nested-repo boundary, shallow/partial
//! state, and pointer-backed asset posture so preview and recovery never target
//! the wrong root.
//!
//! The record family includes:
//!
//! - [`ChangeObjectOrchestrationRecord`] — stable identity, operation provenance,
//!   repo-root ref, repo topology classes, submodule/nested-repo/shallow
//!   boundary refs, and pointer-backed asset posture.
//! - [`WorktreeOrchestrationRecord`] — worktree-specific operation state with
//!   source and target worktree refs, attachment class, and checked-out ref.
//! - [`PatchStackOrchestrationRecord`] — patch-stack-specific operation state
//!   with target class, patch state, patch count, and affected patch refs.
//! - [`PublishProposalRecord`] — provider publish proposal with readiness class,
//!   provider publish posture, and explicit object identity.
//! - [`MutationCheckpointRecord`] — checkpoint summary before destructive apply.
//! - [`ChangeObjectCommandRecord`] — command-graph operations surfaced to the
//!   inspector (preview, approve, capture checkpoint, apply, rollback, abort,
//!   request external handoff).
//! - [`ChangeObjectOrchestrationSupportExportPacket`] — redaction-safe support
//!   export that can reopen the same structured session truth after restart.
//! - [`ChangeObjectOrchestrationInspectionRecord`] — compact boolean projection
//!   for CLI and inspector surfaces.
//!
//! The companion schema lives at
//! `schemas/review/change_object_orchestration.schema.json`. Canonical fixtures
//! live under
//! `fixtures/review/m4/stabilize-worktree-patch-stack-and-explicit-change-object/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceBetaPacket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every change-object orchestration record.
pub const CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ChangeObjectOrchestrationPacket`].
pub const CHANGE_OBJECT_ORCHESTRATION_PACKET_RECORD_KIND: &str =
    "review_change_object_orchestration_packet";

/// Stable record-kind tag for [`ChangeObjectOrchestrationRecord`].
pub const CHANGE_OBJECT_ORCHESTRATION_RECORD_KIND: &str =
    "review_change_object_orchestration_record";

/// Stable record-kind tag for [`WorktreeOrchestrationRecord`].
pub const WORKTREE_ORCHESTRATION_RECORD_KIND: &str = "review_worktree_orchestration_record";

/// Stable record-kind tag for [`PatchStackOrchestrationRecord`].
pub const PATCH_STACK_ORCHESTRATION_RECORD_KIND: &str = "review_patch_stack_orchestration_record";

/// Stable record-kind tag for [`PublishProposalRecord`].
pub const PUBLISH_PROPOSAL_RECORD_KIND: &str = "review_publish_proposal_record";

/// Stable record-kind tag for [`MutationCheckpointRecord`].
pub const MUTATION_CHECKPOINT_RECORD_KIND: &str = "review_mutation_checkpoint_record";

/// Stable record-kind tag for [`ChangeObjectCommandRecord`].
pub const CHANGE_OBJECT_COMMAND_RECORD_KIND: &str = "review_change_object_command_record";

/// Stable record-kind tag for [`ChangeObjectOrchestrationSupportExportPacket`].
pub const CHANGE_OBJECT_ORCHESTRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_change_object_orchestration_support_export_packet";

/// Stable record-kind tag for [`ChangeObjectOrchestrationInspectionRecord`].
pub const CHANGE_OBJECT_ORCHESTRATION_INSPECTION_RECORD_KIND: &str =
    "review_change_object_orchestration_inspection_record";

/// Closed set of change-object orchestration operation kinds.
pub const CHANGE_OBJECT_ORCHESTRATION_OPERATION_KINDS: &[&str] = &[
    "worktree_switch",
    "worktree_create",
    "worktree_remove",
    "patch_stack_reorder",
    "patch_stack_rebase",
    "patch_stack_publish",
    "change_object_publish",
    "change_object_merge",
    "change_object_apply",
];

/// Closed set of change-object orchestration flow states.
pub const CHANGE_OBJECT_ORCHESTRATION_FLOW_STATES: &[&str] = &[
    "preview_pending",
    "preview_approved",
    "checkpoint_pending",
    "checkpoint_captured",
    "executing",
    "completed",
    "failed",
    "rolled_back",
    "aborted",
];

/// Closed set of mutation checkpoint states.
pub const MUTATION_CHECKPOINT_STATES: &[&str] = &[
    "none_required",
    "captured_ready",
    "captured_pending",
    "restored",
    "expired",
    "missing_blocks_apply",
];

/// Closed set of repo topology classes carried on every orchestration record.
pub const REPO_TOPOLOGY_CLASSES: &[&str] = &[
    "current_repo_root",
    "worktree_root",
    "submodule_root",
    "nested_independent_repo_root",
    "shallow_history_root",
    "partial_clone_promisor_root",
    "lfs_hydration_boundary",
];

/// Closed set of pointer-backed asset postures.
pub const POINTER_BACKED_ASSET_POSTURES: &[&str] = &[
    "no_pointer_assets",
    "lfs_pointer_present",
    "submodule_gitlink_present",
    "promisor_partial_object_present",
    "mixed_pointer_assets",
];

/// Closed set of publish readiness classes.
pub const PUBLISH_READINESS_CLASSES: &[&str] = &[
    "ready_to_publish",
    "ready_to_merge",
    "ready_to_apply",
    "blocked_by_conflicts",
    "blocked_by_authority",
    "blocked_by_review_required",
    "not_applicable_inspect_only",
    "readiness_unknown_requires_review",
];

/// Closed set of command classes for the change-object orchestration lane.
pub const CHANGE_OBJECT_ORCHESTRATION_COMMAND_CLASSES: &[&str] = &[
    "preview_mutation",
    "approve_preview",
    "reject_preview",
    "capture_checkpoint",
    "restore_checkpoint",
    "apply_mutation",
    "rollback_mutation",
    "abort_flow",
    "request_external_handoff",
    "continue_after_resolve",
];

/// Closed set of consumer surfaces for change-object orchestration packets.
pub const CHANGE_OBJECT_ORCHESTRATION_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "change_object_inspector",
    "cli_headless_entry",
    "support_export",
    "audit_lane",
    "browser_companion",
];

/// Closed set of invalidation reasons that mark an orchestration stale.
pub const CHANGE_OBJECT_ORCHESTRATION_INVALIDATION_REASONS: &[&str] = &[
    "stale_base",
    "worktree_scope_changed",
    "provider_overlay_stale",
    "repo_topology_changed",
    "submodule_state_changed",
    "nested_repo_boundary_changed",
    "shallow_boundary_changed",
    "pointer_asset_changed",
    "approval_invalidated",
    "checks_stale",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a change-object orchestration to materialize on top of a
/// review-workspace packet and an optional change-object record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectOrchestrationInput {
    /// Stable orchestration identity.
    pub orchestration_id: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Operation kind from the closed vocabulary.
    pub operation_kind: String,
    /// Current flow lifecycle state.
    pub flow_state: String,
    /// Change-object ref the orchestration targets.
    pub change_object_ref: String,
    /// Change-object kind (`branch`, `worktree`, `patch_stack`).
    pub change_object_kind: String,
    /// Opaque stable repo root identifier.
    pub repo_root_ref: String,
    /// Repo topology classes from the closed vocabulary.
    pub repo_topology_classes: Vec<String>,
    /// Optional parent repo root ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_repo_root_ref: Option<String>,
    /// Optional submodule boundary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submodule_boundary_ref: Option<String>,
    /// Optional nested-repo boundary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested_repo_boundary_ref: Option<String>,
    /// Optional shallow-history boundary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shallow_history_boundary_ref: Option<String>,
    /// Pointer-backed asset posture from the closed vocabulary.
    pub pointer_backed_asset_posture: String,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
    /// Approval state observed at flow start.
    pub approval_state: String,
    /// Checks-freshness state observed at flow start.
    pub checks_freshness_state: String,
    /// Stable session ref used to reopen this flow after restart.
    pub restart_session_ref: String,
    /// Worktree operation input (required for worktree_* operation kinds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree_operation: Option<WorktreeOperationInput>,
    /// Patch-stack operation input (required for patch_stack_* operation kinds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_stack_operation: Option<PatchStackOperationInput>,
    /// Publish proposal input (required for change_object_publish and
    /// patch_stack_publish operation kinds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_proposal: Option<PublishProposalInput>,
    /// Recovery checkpoint summary input.
    pub mutation_checkpoint: MutationCheckpointInput,
    /// Command-graph operations defined for this orchestration.
    pub commands: Vec<ChangeObjectCommandInput>,
    /// Support/export envelope input.
    pub support_export: ChangeObjectOrchestrationSupportExportInput,
    /// Active invalidation reasons; empty when none apply.
    #[serde(default)]
    pub invalidation_reasons: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input describing a worktree-specific operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeOperationInput {
    /// Source worktree ref.
    pub source_worktree_ref: String,
    /// Target worktree ref.
    pub target_worktree_ref: String,
    /// Worktree kind class.
    pub worktree_kind_class: String,
    /// Worktree attachment class.
    pub worktree_attachment_class: String,
    /// Checked-out ref label on the target worktree.
    pub checked_out_ref_label: String,
    /// Optional linked branch ref label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_branch_ref_label: Option<String>,
}

/// Input describing a patch-stack-specific operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStackOperationInput {
    /// Patch stack target class.
    pub patch_stack_target_class: String,
    /// Patch state class.
    pub patch_state_class: String,
    /// Number of patches in the stack.
    pub patch_count: u32,
    /// Top patch label.
    pub top_patch_label: String,
    /// Affected patch refs for this operation.
    pub affected_patch_refs: Vec<String>,
    /// Optional review class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_class: Option<String>,
}

/// Input describing a provider publish proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishProposalInput {
    /// Stable proposal identity.
    pub proposal_id: String,
    /// Provider-side object identity ref.
    pub provider_object_identity_ref: String,
    /// Publish readiness class from the closed vocabulary.
    pub publish_readiness_class: String,
    /// Provider publish posture.
    pub provider_publish_posture: String,
    /// Target provider class.
    pub target_provider_class: String,
    /// True when the proposal requires explicit browser handoff.
    pub requires_browser_handoff: bool,
    /// Handoff origin class.
    pub handoff_origin_class: String,
    /// Handoff destination class.
    pub handoff_destination_class: String,
    /// Return anchor ref for reversible handoff.
    pub return_anchor_ref: String,
    /// Freshness class at proposal time.
    pub freshness_class: String,
    /// Actor that minted the proposal.
    pub actor_ref: String,
}

/// Input describing the mutation checkpoint summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationCheckpointInput {
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

/// Input describing one command-graph operation for a change-object
/// orchestration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectCommandInput {
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

/// Input row for the change-object orchestration support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectOrchestrationSupportExportInput {
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

/// Change-object orchestration record materialized from input plus workspace
/// truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectOrchestrationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable orchestration identity.
    pub orchestration_id: String,
    /// Review workspace this orchestration belongs to.
    pub review_workspace_id_ref: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Current flow lifecycle state.
    pub flow_state: String,
    /// Change-object ref.
    pub change_object_ref: String,
    /// Change-object kind.
    pub change_object_kind: String,
    /// Opaque stable repo root identifier.
    pub repo_root_ref: String,
    /// Repo topology classes.
    pub repo_topology_classes: Vec<String>,
    /// Optional parent repo root ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_repo_root_ref: Option<String>,
    /// Optional submodule boundary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submodule_boundary_ref: Option<String>,
    /// Optional nested-repo boundary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested_repo_boundary_ref: Option<String>,
    /// Optional shallow-history boundary ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shallow_history_boundary_ref: Option<String>,
    /// Pointer-backed asset posture.
    pub pointer_backed_asset_posture: String,
    /// Review-pack digest pinned at review time.
    pub review_pack_digest_ref: String,
    /// Environment capsule digest pinned at review time.
    pub environment_capsule_digest_ref: String,
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

/// Worktree orchestration record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeOrchestrationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Orchestration this worktree record belongs to.
    pub orchestration_id_ref: String,
    /// Source worktree ref.
    pub source_worktree_ref: String,
    /// Target worktree ref.
    pub target_worktree_ref: String,
    /// Worktree kind class.
    pub worktree_kind_class: String,
    /// Worktree attachment class.
    pub worktree_attachment_class: String,
    /// Checked-out ref label.
    pub checked_out_ref_label: String,
    /// Optional linked branch ref label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_branch_ref_label: Option<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Patch-stack orchestration record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStackOrchestrationRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Orchestration this patch-stack record belongs to.
    pub orchestration_id_ref: String,
    /// Patch stack target class.
    pub patch_stack_target_class: String,
    /// Patch state class.
    pub patch_state_class: String,
    /// Number of patches.
    pub patch_count: u32,
    /// Top patch label.
    pub top_patch_label: String,
    /// Affected patch refs.
    pub affected_patch_refs: Vec<String>,
    /// Optional review class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_class: Option<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Publish proposal record materialized from input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishProposalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Orchestration this proposal belongs to.
    pub orchestration_id_ref: String,
    /// Stable proposal identity.
    pub proposal_id: String,
    /// Provider-side object identity ref.
    pub provider_object_identity_ref: String,
    /// Publish readiness class.
    pub publish_readiness_class: String,
    /// Provider publish posture.
    pub provider_publish_posture: String,
    /// Target provider class.
    pub target_provider_class: String,
    /// True when the proposal requires explicit browser handoff.
    pub requires_browser_handoff: bool,
    /// Handoff origin class.
    pub handoff_origin_class: String,
    /// Handoff destination class.
    pub handoff_destination_class: String,
    /// Return anchor ref.
    pub return_anchor_ref: String,
    /// Freshness class.
    pub freshness_class: String,
    /// Actor that minted the proposal.
    pub actor_ref: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Mutation checkpoint record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationCheckpointRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Orchestration this checkpoint belongs to.
    pub orchestration_id_ref: String,
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

/// Command-graph operation record for a change-object orchestration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectCommandRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable command identity.
    pub command_id: String,
    /// Orchestration this command belongs to.
    pub orchestration_id_ref: String,
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
pub struct ChangeObjectOrchestrationInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Orchestration inspected by this row.
    pub orchestration_id_ref: String,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// True when the preview has been approved.
    pub preview_approved: bool,
    /// True when the preview is still pending.
    pub preview_pending: bool,
    /// True when the preview was rejected.
    pub preview_rejected: bool,
    /// True when a checkpoint is captured and ready.
    pub checkpoint_ready: bool,
    /// True when the flow is currently executing.
    pub executing: bool,
    /// True when the flow completed successfully.
    pub completed: bool,
    /// True when the flow failed.
    pub failed: bool,
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
    /// True when the publish proposal requires browser handoff.
    pub requires_browser_handoff: bool,
    /// True when the handoff carries a return anchor.
    pub handoff_reversible: bool,
    /// Number of command-graph operations attached.
    pub command_count: usize,
    /// True when at least one command supports preview/dry-run.
    pub preview_capable: bool,
    /// True when support/export can reopen the flow context.
    pub support_export_reopenable: bool,
    /// True when repo topology includes a submodule boundary.
    pub submodule_boundary_present: bool,
    /// True when repo topology includes a nested-repo boundary.
    pub nested_repo_boundary_present: bool,
    /// True when repo topology includes a shallow-history boundary.
    pub shallow_boundary_present: bool,
    /// True when pointer-backed assets are present.
    pub pointer_backed_assets_present: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet for the change-object orchestration lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectOrchestrationSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Orchestration this packet exports.
    pub orchestration_id_ref: String,
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
    pub restart_snapshot: ChangeObjectOrchestrationRestartSnapshot,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Restart snapshot embedded in the support-export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectOrchestrationRestartSnapshot {
    /// Flow state at export time.
    pub flow_state: String,
    /// Operation kind at export time.
    pub operation_kind: String,
    /// Checkpoint state at export time.
    pub checkpoint_state: String,
    /// Repo root ref at export time.
    pub repo_root_ref: String,
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

/// Change-object orchestration packet consumed by review surfaces and support
/// exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeObjectOrchestrationPacket {
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
    /// Change-object orchestration record.
    pub orchestration: ChangeObjectOrchestrationRecord,
    /// Optional worktree orchestration record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree_operation: Option<WorktreeOrchestrationRecord>,
    /// Optional patch-stack orchestration record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_stack_operation: Option<PatchStackOrchestrationRecord>,
    /// Optional publish proposal record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_proposal: Option<PublishProposalRecord>,
    /// Mutation checkpoint record.
    pub mutation_checkpoint: MutationCheckpointRecord,
    /// Command-graph operation records.
    pub commands: Vec<ChangeObjectCommandRecord>,
    /// Support/export packet.
    pub support_export: ChangeObjectOrchestrationSupportExportPacket,
    /// Inspection row.
    pub inspection: ChangeObjectOrchestrationInspectionRecord,
}

impl ChangeObjectOrchestrationPacket {
    /// Builds a change-object orchestration packet from a beta review-workspace
    /// packet and orchestration input.
    ///
    /// # Errors
    ///
    /// Returns [`ChangeObjectOrchestrationValidationError`] when the input
    /// violates an orchestration invariant.
    pub fn from_workspace_packet(
        input: ChangeObjectOrchestrationInput,
        workspace_packet: &ReviewWorkspaceBetaPacket,
    ) -> Result<Self, ChangeObjectOrchestrationValidationError> {
        validate_input(&input, workspace_packet)?;

        let orchestration = orchestration_record(&input, workspace_packet);
        let worktree_operation = input
            .worktree_operation
            .as_ref()
            .map(|op| worktree_orchestration_record(op, &orchestration));
        let patch_stack_operation = input
            .patch_stack_operation
            .as_ref()
            .map(|op| patch_stack_orchestration_record(op, &orchestration));
        let publish_proposal = input
            .publish_proposal
            .as_ref()
            .map(|prop| publish_proposal_record(prop, &orchestration));
        let mutation_checkpoint =
            mutation_checkpoint_record(&input.mutation_checkpoint, &orchestration);
        let commands = input
            .commands
            .iter()
            .map(|command| change_object_command_record(command, &orchestration))
            .collect::<Vec<_>>();
        let support_export = change_object_orchestration_support_export_packet(
            &input.support_export,
            &orchestration,
            workspace_packet,
            &commands,
            &mutation_checkpoint,
        );
        let inspection = change_object_orchestration_inspection_record(
            &orchestration,
            worktree_operation.as_ref(),
            patch_stack_operation.as_ref(),
            publish_proposal.as_ref(),
            &mutation_checkpoint,
            &commands,
            &support_export,
        );

        let packet = Self {
            record_kind: CHANGE_OBJECT_ORCHESTRATION_PACKET_RECORD_KIND.to_string(),
            schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: workspace_packet.review_workspace.clone(),
            orchestration,
            worktree_operation,
            patch_stack_operation,
            publish_proposal,
            mutation_checkpoint,
            commands,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the orchestration invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ChangeObjectOrchestrationValidationError`] when an invariant
    /// is violated.
    pub fn validate(&self) -> Result<(), ChangeObjectOrchestrationValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            CHANGE_OBJECT_ORCHESTRATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        if self.schema_version != CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION {
            return Err(change_object_orchestration_validation_error(format!(
                "schema_version mismatch: expected {}, got {}",
                CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
                self.schema_version
            )));
        }
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_orchestration_record(
            &self.orchestration,
            &self.review_workspace.review_workspace_id,
        )?;
        if let Some(worktree) = &self.worktree_operation {
            validate_worktree_orchestration_record(worktree, &self.orchestration.orchestration_id)?;
        }
        if let Some(patch_stack) = &self.patch_stack_operation {
            validate_patch_stack_orchestration_record(
                patch_stack,
                &self.orchestration.orchestration_id,
            )?;
        }
        if let Some(proposal) = &self.publish_proposal {
            validate_publish_proposal_record(proposal, &self.orchestration.orchestration_id)?;
        }
        validate_mutation_checkpoint_record(
            &self.mutation_checkpoint,
            &self.orchestration.orchestration_id,
        )?;
        for command in &self.commands {
            validate_change_object_command_record(command, &self.orchestration.orchestration_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.orchestration,
            &self.commands,
            &self.mutation_checkpoint,
        )?;
        validate_inspection(&self.inspection, self)?;

        // Cross-record invariants
        if self.orchestration.flow_state == "preview_approved" {
            if self.mutation_checkpoint.checkpoint_state == "missing_blocks_apply" {
                return Err(change_object_orchestration_validation_error(
                    "preview_approved flow cannot have missing_blocks_apply checkpoint",
                ));
            }
        }
        if matches!(
            self.orchestration.flow_state.as_str(),
            "executing" | "completed" | "rolled_back"
        ) {
            if self.mutation_checkpoint.checkpoint_state == "missing_blocks_apply" {
                return Err(change_object_orchestration_validation_error(
                    "executing/completed/rolled_back flow cannot have missing_blocks_apply checkpoint",
                ));
            }
        }
        if self.orchestration.flow_state == "completed" && self.inspection.failed {
            return Err(change_object_orchestration_validation_error(
                "completed flow cannot be marked failed",
            ));
        }
        if self.orchestration.flow_state == "rolled_back" && self.inspection.completed {
            return Err(change_object_orchestration_validation_error(
                "rolled_back flow cannot be marked completed",
            ));
        }

        // Operation-kind / sub-record consistency
        match self.orchestration.operation_kind.as_str() {
            "worktree_switch" | "worktree_create" | "worktree_remove" => {
                if self.worktree_operation.is_none() {
                    return Err(change_object_orchestration_validation_error(
                        "worktree operations require a worktree_orchestration record",
                    ));
                }
            }
            "patch_stack_reorder" | "patch_stack_rebase" | "patch_stack_publish" => {
                if self.patch_stack_operation.is_none() {
                    return Err(change_object_orchestration_validation_error(
                        "patch_stack operations require a patch_stack_orchestration record",
                    ));
                }
            }
            "change_object_publish" | "change_object_merge" | "change_object_apply" => {
                // These may optionally carry publish_proposal; no required sub-record.
            }
            other => {
                if !contains_token(CHANGE_OBJECT_ORCHESTRATION_OPERATION_KINDS, other) {
                    return Err(change_object_orchestration_validation_error(format!(
                        "unsupported operation_kind {other}"
                    )));
                }
            }
        }

        // Publish proposal consistency
        if let Some(proposal) = &self.publish_proposal {
            if proposal.requires_browser_handoff
                && (proposal.return_anchor_ref.trim().is_empty()
                    || proposal.handoff_destination_class.trim().is_empty())
            {
                return Err(change_object_orchestration_validation_error(
                    "publish proposal requiring browser handoff must declare destination and return_anchor",
                ));
            }
        }

        Ok(())
    }

    /// Returns true when orchestration truth axes are surfaced as separable
    /// inspectable truths.
    pub fn truths_are_separable(&self) -> bool {
        let orch = &self.orchestration;
        contains_token(CHANGE_OBJECT_ORCHESTRATION_OPERATION_KINDS, &orch.operation_kind)
            && contains_token(CHANGE_OBJECT_ORCHESTRATION_FLOW_STATES, &orch.flow_state)
            && contains_token(REPO_TOPOLOGY_CLASSES, &orch.repo_topology_classes[0])
            && contains_token(POINTER_BACKED_ASSET_POSTURES, &orch.pointer_backed_asset_posture)
            && contains_token(MUTATION_CHECKPOINT_STATES, &self.mutation_checkpoint.checkpoint_state)
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
pub struct ChangeObjectOrchestrationProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Orchestration identity.
    pub orchestration_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Flow state.
    pub flow_state: String,
    /// Checkpoint state.
    pub checkpoint_state: String,
    /// Repo root ref.
    pub repo_root_ref: String,
    /// True when the preview is approved.
    pub preview_approved: bool,
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
    /// True when the publish proposal requires browser handoff.
    pub requires_browser_handoff: bool,
    /// True when repo topology includes a submodule boundary.
    pub submodule_boundary_present: bool,
    /// True when repo topology includes a nested-repo boundary.
    pub nested_repo_boundary_present: bool,
    /// True when repo topology includes a shallow-history boundary.
    pub shallow_boundary_present: bool,
}

/// Parses and validates a materialized change-object orchestration packet.
///
/// # Errors
///
/// Returns [`ChangeObjectOrchestrationError`] when the payload fails to parse
/// or violates the orchestration invariants.
pub fn project_change_object_orchestration_packet(
    payload: &str,
) -> Result<ChangeObjectOrchestrationProjection, ChangeObjectOrchestrationError> {
    let packet: ChangeObjectOrchestrationPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(ChangeObjectOrchestrationProjection::from(packet))
}

impl From<ChangeObjectOrchestrationPacket> for ChangeObjectOrchestrationProjection {
    fn from(packet: ChangeObjectOrchestrationPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            orchestration_id: packet.orchestration.orchestration_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            operation_kind: packet.orchestration.operation_kind,
            flow_state: packet.orchestration.flow_state,
            checkpoint_state: packet.mutation_checkpoint.checkpoint_state,
            repo_root_ref: packet.orchestration.repo_root_ref,
            preview_approved: packet.inspection.preview_approved,
            checkpoint_ready: packet.inspection.checkpoint_ready,
            actionable: packet.inspection.actionable,
            restartable: packet.inspection.restartable,
            approval_invalidated: packet.inspection.approval_invalidated,
            invalidation_reasons: packet.orchestration.invalidation_reasons,
            blocked_reasons: packet.orchestration.blocked_reasons,
            command_count: packet.commands.len(),
            preview_capable: packet.inspection.preview_capable,
            support_export_reopenable: packet.inspection.support_export_reopenable,
            consumer_surfaces: packet.support_export.consumer_surfaces,
            requires_browser_handoff: packet.inspection.requires_browser_handoff,
            submodule_boundary_present: packet.inspection.submodule_boundary_present,
            nested_repo_boundary_present: packet.inspection.nested_repo_boundary_present,
            shallow_boundary_present: packet.inspection.shallow_boundary_present,
        }
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error returned when a change-object orchestration payload cannot be
/// projected.
#[derive(Debug)]
pub enum ChangeObjectOrchestrationError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the orchestration invariants.
    Validation(ChangeObjectOrchestrationValidationError),
}

impl fmt::Display for ChangeObjectOrchestrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => {
                write!(formatter, "change-object orchestration parse error: {err}")
            }
            Self::Validation(err) => {
                write!(formatter, "change-object orchestration validation error: {err}")
            }
        }
    }
}

impl std::error::Error for ChangeObjectOrchestrationError {}

impl From<serde_json::Error> for ChangeObjectOrchestrationError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<ChangeObjectOrchestrationValidationError> for ChangeObjectOrchestrationError {
    fn from(err: ChangeObjectOrchestrationValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for change-object orchestration packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeObjectOrchestrationValidationError {
    message: String,
}

impl ChangeObjectOrchestrationValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ChangeObjectOrchestrationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ChangeObjectOrchestrationValidationError {}

fn change_object_orchestration_validation_error(
    message: impl Into<String>,
) -> ChangeObjectOrchestrationValidationError {
    ChangeObjectOrchestrationValidationError {
        message: message.into(),
    }
}

// ---------------------------------------------------------------------------
// Internal constructors
// ---------------------------------------------------------------------------

fn orchestration_record(
    input: &ChangeObjectOrchestrationInput,
    workspace_packet: &ReviewWorkspaceBetaPacket,
) -> ChangeObjectOrchestrationRecord {
    let actionable = compute_actionable(input);
    let blocked_reasons = compute_blocked_reasons(input);
    ChangeObjectOrchestrationRecord {
        record_kind: CHANGE_OBJECT_ORCHESTRATION_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        orchestration_id: input.orchestration_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        operation_kind: input.operation_kind.clone(),
        flow_state: input.flow_state.clone(),
        change_object_ref: input.change_object_ref.clone(),
        change_object_kind: input.change_object_kind.clone(),
        repo_root_ref: input.repo_root_ref.clone(),
        repo_topology_classes: input.repo_topology_classes.clone(),
        parent_repo_root_ref: input.parent_repo_root_ref.clone(),
        submodule_boundary_ref: input.submodule_boundary_ref.clone(),
        nested_repo_boundary_ref: input.nested_repo_boundary_ref.clone(),
        shallow_history_boundary_ref: input.shallow_history_boundary_ref.clone(),
        pointer_backed_asset_posture: input.pointer_backed_asset_posture.clone(),
        review_pack_digest_ref: input.review_pack_digest_ref.clone(),
        environment_capsule_digest_ref: input.environment_capsule_digest_ref.clone(),
        approval_state: input.approval_state.clone(),
        checks_freshness_state: input.checks_freshness_state.clone(),
        restart_session_ref: input.restart_session_ref.clone(),
        invalidation_reasons: input.invalidation_reasons.clone(),
        blocked_reasons,
        actionable,
        generated_at: input.generated_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn worktree_orchestration_record(
    input: &WorktreeOperationInput,
    orchestration: &ChangeObjectOrchestrationRecord,
) -> WorktreeOrchestrationRecord {
    WorktreeOrchestrationRecord {
        record_kind: WORKTREE_ORCHESTRATION_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        orchestration_id_ref: orchestration.orchestration_id.clone(),
        source_worktree_ref: input.source_worktree_ref.clone(),
        target_worktree_ref: input.target_worktree_ref.clone(),
        worktree_kind_class: input.worktree_kind_class.clone(),
        worktree_attachment_class: input.worktree_attachment_class.clone(),
        checked_out_ref_label: input.checked_out_ref_label.clone(),
        linked_branch_ref_label: input.linked_branch_ref_label.clone(),
        summary_label: format!(
            "worktree {} -> {}",
            input.source_worktree_ref, input.target_worktree_ref
        ),
    }
}

fn patch_stack_orchestration_record(
    input: &PatchStackOperationInput,
    orchestration: &ChangeObjectOrchestrationRecord,
) -> PatchStackOrchestrationRecord {
    PatchStackOrchestrationRecord {
        record_kind: PATCH_STACK_ORCHESTRATION_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        orchestration_id_ref: orchestration.orchestration_id.clone(),
        patch_stack_target_class: input.patch_stack_target_class.clone(),
        patch_state_class: input.patch_state_class.clone(),
        patch_count: input.patch_count,
        top_patch_label: input.top_patch_label.clone(),
        affected_patch_refs: input.affected_patch_refs.clone(),
        review_class: input.review_class.clone(),
        summary_label: format!(
            "patch stack {} with {} patches",
            input.patch_state_class, input.patch_count
        ),
    }
}

fn publish_proposal_record(
    input: &PublishProposalInput,
    orchestration: &ChangeObjectOrchestrationRecord,
) -> PublishProposalRecord {
    PublishProposalRecord {
        record_kind: PUBLISH_PROPOSAL_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        orchestration_id_ref: orchestration.orchestration_id.clone(),
        proposal_id: input.proposal_id.clone(),
        provider_object_identity_ref: input.provider_object_identity_ref.clone(),
        publish_readiness_class: input.publish_readiness_class.clone(),
        provider_publish_posture: input.provider_publish_posture.clone(),
        target_provider_class: input.target_provider_class.clone(),
        requires_browser_handoff: input.requires_browser_handoff,
        handoff_origin_class: input.handoff_origin_class.clone(),
        handoff_destination_class: input.handoff_destination_class.clone(),
        return_anchor_ref: input.return_anchor_ref.clone(),
        freshness_class: input.freshness_class.clone(),
        actor_ref: input.actor_ref.clone(),
        summary_label: format!("publish proposal {} to {}", input.proposal_id, input.target_provider_class),
    }
}

fn mutation_checkpoint_record(
    input: &MutationCheckpointInput,
    orchestration: &ChangeObjectOrchestrationRecord,
) -> MutationCheckpointRecord {
    MutationCheckpointRecord {
        record_kind: MUTATION_CHECKPOINT_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        orchestration_id_ref: orchestration.orchestration_id.clone(),
        checkpoint_state: input.checkpoint_state.clone(),
        checkpoint_ref: input.checkpoint_ref.clone(),
        restore_command_id: input.restore_command_id.clone(),
        disclosure_label: input.disclosure_label.clone(),
        restorable_offline: input.restorable_offline,
        summary_label: format!("checkpoint: {}", input.checkpoint_state),
    }
}

fn change_object_command_record(
    input: &ChangeObjectCommandInput,
    orchestration: &ChangeObjectOrchestrationRecord,
) -> ChangeObjectCommandRecord {
    let actionable = input.blocked_reasons.is_empty()
        && orchestration.actionable
        && contains_token(CHANGE_OBJECT_ORCHESTRATION_COMMAND_CLASSES, &input.command_class);
    ChangeObjectCommandRecord {
        record_kind: CHANGE_OBJECT_COMMAND_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        command_id: input.command_id.clone(),
        orchestration_id_ref: orchestration.orchestration_id.clone(),
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

fn change_object_orchestration_support_export_packet(
    input: &ChangeObjectOrchestrationSupportExportInput,
    orchestration: &ChangeObjectOrchestrationRecord,
    workspace_packet: &ReviewWorkspaceBetaPacket,
    commands: &[ChangeObjectCommandRecord],
    mutation_checkpoint: &MutationCheckpointRecord,
) -> ChangeObjectOrchestrationSupportExportPacket {
    ChangeObjectOrchestrationSupportExportPacket {
        record_kind: CHANGE_OBJECT_ORCHESTRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        orchestration_id_ref: orchestration.orchestration_id.clone(),
        review_workspace_id_ref: workspace_packet.review_workspace.review_workspace_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        command_id_refs: commands.iter().map(|c| c.command_id.clone()).collect(),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/change_object_orchestration.schema.json".to_string(),
        ],
        raw_url_export_allowed: false,
        raw_provider_payload_export_allowed: false,
        raw_path_export_allowed: false,
        raw_branch_name_export_allowed: false,
        raw_patch_body_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        restart_snapshot: ChangeObjectOrchestrationRestartSnapshot {
            flow_state: orchestration.flow_state.clone(),
            operation_kind: orchestration.operation_kind.clone(),
            checkpoint_state: mutation_checkpoint.checkpoint_state.clone(),
            repo_root_ref: orchestration.repo_root_ref.clone(),
            restart_session_ref: orchestration.restart_session_ref.clone(),
            invalidation_reasons: orchestration.invalidation_reasons.clone(),
            blocked_reasons: orchestration.blocked_reasons.clone(),
        },
        summary_label: input.summary_label.clone(),
    }
}

fn change_object_orchestration_inspection_record(
    orchestration: &ChangeObjectOrchestrationRecord,
    _worktree_operation: Option<&WorktreeOrchestrationRecord>,
    _patch_stack_operation: Option<&PatchStackOrchestrationRecord>,
    publish_proposal: Option<&PublishProposalRecord>,
    mutation_checkpoint: &MutationCheckpointRecord,
    commands: &[ChangeObjectCommandRecord],
    support_export: &ChangeObjectOrchestrationSupportExportPacket,
) -> ChangeObjectOrchestrationInspectionRecord {
    ChangeObjectOrchestrationInspectionRecord {
        record_kind: CHANGE_OBJECT_ORCHESTRATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
        orchestration_id_ref: orchestration.orchestration_id.clone(),
        review_workspace_id_ref: orchestration.review_workspace_id_ref.clone(),
        preview_approved: matches!(
            orchestration.flow_state.as_str(),
            "preview_approved"
                | "checkpoint_pending"
                | "checkpoint_captured"
                | "executing"
                | "completed"
                | "failed"
                | "rolled_back"
        ),
        preview_pending: orchestration.flow_state == "preview_pending",
        preview_rejected: matches!(orchestration.flow_state.as_str(), "aborted" | "failed"),
        checkpoint_ready: matches!(
            mutation_checkpoint.checkpoint_state.as_str(),
            "captured_ready" | "captured_pending" | "restored"
        ),
        executing: orchestration.flow_state == "executing",
        completed: orchestration.flow_state == "completed",
        failed: orchestration.flow_state == "failed",
        rolled_back: orchestration.flow_state == "rolled_back",
        aborted: orchestration.flow_state == "aborted",
        approval_invalidated: orchestration
            .invalidation_reasons
            .contains(&"approval_invalidated".to_string()),
        checks_stale_blocks_apply: orchestration
            .invalidation_reasons
            .contains(&"checks_stale".to_string()),
        actionable: orchestration.actionable,
        restartable: !orchestration.restart_session_ref.trim().is_empty(),
        requires_browser_handoff: publish_proposal
            .map(|p| p.requires_browser_handoff)
            .unwrap_or(false),
        handoff_reversible: publish_proposal
            .map(|p| !p.return_anchor_ref.trim().is_empty())
            .unwrap_or(false),
        command_count: commands.len(),
        preview_capable: commands.iter().any(|c| c.preview_supported && c.actionable),
        support_export_reopenable: !support_export.reopen_context_ref.trim().is_empty()
            && !support_export.reopen_command_id_ref.trim().is_empty(),
        submodule_boundary_present: orchestration.submodule_boundary_ref.is_some(),
        nested_repo_boundary_present: orchestration.nested_repo_boundary_ref.is_some(),
        shallow_boundary_present: orchestration.shallow_history_boundary_ref.is_some(),
        pointer_backed_assets_present: orchestration.pointer_backed_asset_posture != "no_pointer_assets",
        summary_label: orchestration.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &ChangeObjectOrchestrationInput,
    _workspace_packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_nonempty(&input.orchestration_id, "orchestration_id")?;
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_one_of(
        "operation_kind",
        CHANGE_OBJECT_ORCHESTRATION_OPERATION_KINDS,
        &input.operation_kind,
    )?;
    ensure_one_of(
        "flow_state",
        CHANGE_OBJECT_ORCHESTRATION_FLOW_STATES,
        &input.flow_state,
    )?;
    ensure_nonempty(&input.change_object_ref, "change_object_ref")?;
    ensure_nonempty(&input.change_object_kind, "change_object_kind")?;
    ensure_nonempty(&input.repo_root_ref, "repo_root_ref")?;
    if input.repo_topology_classes.is_empty() {
        return Err(change_object_orchestration_validation_error(
            "repo_topology_classes must not be empty",
        ));
    }
    for class in &input.repo_topology_classes {
        ensure_one_of("repo_topology_classes[]", REPO_TOPOLOGY_CLASSES, class)?;
    }
    ensure_one_of(
        "pointer_backed_asset_posture",
        POINTER_BACKED_ASSET_POSTURES,
        &input.pointer_backed_asset_posture,
    )?;
    ensure_nonempty(&input.restart_session_ref, "restart_session_ref")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    if let Some(worktree) = &input.worktree_operation {
        ensure_nonempty(&worktree.source_worktree_ref, "worktree_operation.source_worktree_ref")?;
        ensure_nonempty(&worktree.target_worktree_ref, "worktree_operation.target_worktree_ref")?;
        ensure_nonempty(&worktree.checked_out_ref_label, "worktree_operation.checked_out_ref_label")?;
    }

    if let Some(patch_stack) = &input.patch_stack_operation {
        ensure_nonempty(&patch_stack.top_patch_label, "patch_stack_operation.top_patch_label")?;
    }

    if let Some(proposal) = &input.publish_proposal {
        ensure_nonempty(&proposal.proposal_id, "publish_proposal.proposal_id")?;
        ensure_nonempty(&proposal.provider_object_identity_ref, "publish_proposal.provider_object_identity_ref")?;
        ensure_one_of(
            "publish_proposal.publish_readiness_class",
            PUBLISH_READINESS_CLASSES,
            &proposal.publish_readiness_class,
        )?;
    }

    ensure_one_of(
        "mutation_checkpoint.checkpoint_state",
        MUTATION_CHECKPOINT_STATES,
        &input.mutation_checkpoint.checkpoint_state,
    )?;

    if input.commands.is_empty() {
        return Err(change_object_orchestration_validation_error(
            "commands must not be empty",
        ));
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for command in &input.commands {
        ensure_one_of(
            "commands[].command_class",
            CHANGE_OBJECT_ORCHESTRATION_COMMAND_CLASSES,
            &command.command_class,
        )?;
        if !seen.insert(command.command_id.as_str()) {
            return Err(change_object_orchestration_validation_error(format!(
                "commands contains a duplicate command_id: {}",
                command.command_id
            )));
        }
    }

    if input.support_export.consumer_surfaces.is_empty() {
        return Err(change_object_orchestration_validation_error(
            "support_export.consumer_surfaces must not be empty",
        ));
    }
    for surface in &input.support_export.consumer_surfaces {
        ensure_one_of(
            "support_export.consumer_surfaces[]",
            CHANGE_OBJECT_ORCHESTRATION_CONSUMER_SURFACES,
            surface,
        )?;
    }

    for reason in &input.invalidation_reasons {
        ensure_one_of(
            "invalidation_reasons[]",
            CHANGE_OBJECT_ORCHESTRATION_INVALIDATION_REASONS,
            reason,
        )?;
    }

    Ok(())
}

fn validate_orchestration_record(
    record: &ChangeObjectOrchestrationRecord,
    expected_workspace_id: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        CHANGE_OBJECT_ORCHESTRATION_RECORD_KIND,
        "orchestration.record_kind",
    )?;
    if record.schema_version != CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION {
        return Err(change_object_orchestration_validation_error(format!(
            "orchestration.schema_version mismatch: expected {}, got {}",
            CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
            record.schema_version
        )));
    }
    ensure_eq(
        record.review_workspace_id_ref.as_str(),
        expected_workspace_id,
        "orchestration.review_workspace_id_ref",
    )?;
    ensure_nonempty(&record.orchestration_id, "orchestration.orchestration_id")?;
    ensure_one_of(
        "orchestration.operation_kind",
        CHANGE_OBJECT_ORCHESTRATION_OPERATION_KINDS,
        &record.operation_kind,
    )?;
    ensure_one_of(
        "orchestration.flow_state",
        CHANGE_OBJECT_ORCHESTRATION_FLOW_STATES,
        &record.flow_state,
    )?;
    ensure_nonempty(&record.repo_root_ref, "orchestration.repo_root_ref")?;
    if record.repo_topology_classes.is_empty() {
        return Err(change_object_orchestration_validation_error(
            "orchestration.repo_topology_classes must not be empty",
        ));
    }
    for class in &record.repo_topology_classes {
        ensure_one_of("orchestration.repo_topology_classes[]", REPO_TOPOLOGY_CLASSES, class)?;
    }
    ensure_one_of(
        "orchestration.pointer_backed_asset_posture",
        POINTER_BACKED_ASSET_POSTURES,
        &record.pointer_backed_asset_posture,
    )?;
    Ok(())
}

fn validate_worktree_orchestration_record(
    record: &WorktreeOrchestrationRecord,
    expected_orchestration_id: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        WORKTREE_ORCHESTRATION_RECORD_KIND,
        "worktree_operation.record_kind",
    )?;
    ensure_eq(
        record.orchestration_id_ref.as_str(),
        expected_orchestration_id,
        "worktree_operation.orchestration_id_ref",
    )?;
    Ok(())
}

fn validate_patch_stack_orchestration_record(
    record: &PatchStackOrchestrationRecord,
    expected_orchestration_id: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PATCH_STACK_ORCHESTRATION_RECORD_KIND,
        "patch_stack_operation.record_kind",
    )?;
    ensure_eq(
        record.orchestration_id_ref.as_str(),
        expected_orchestration_id,
        "patch_stack_operation.orchestration_id_ref",
    )?;
    Ok(())
}

fn validate_publish_proposal_record(
    record: &PublishProposalRecord,
    expected_orchestration_id: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        PUBLISH_PROPOSAL_RECORD_KIND,
        "publish_proposal.record_kind",
    )?;
    ensure_eq(
        record.orchestration_id_ref.as_str(),
        expected_orchestration_id,
        "publish_proposal.orchestration_id_ref",
    )?;
    ensure_one_of(
        "publish_proposal.publish_readiness_class",
        PUBLISH_READINESS_CLASSES,
        &record.publish_readiness_class,
    )?;
    Ok(())
}

fn validate_mutation_checkpoint_record(
    record: &MutationCheckpointRecord,
    expected_orchestration_id: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        MUTATION_CHECKPOINT_RECORD_KIND,
        "mutation_checkpoint.record_kind",
    )?;
    ensure_eq(
        record.orchestration_id_ref.as_str(),
        expected_orchestration_id,
        "mutation_checkpoint.orchestration_id_ref",
    )?;
    ensure_one_of(
        "mutation_checkpoint.checkpoint_state",
        MUTATION_CHECKPOINT_STATES,
        &record.checkpoint_state,
    )?;
    Ok(())
}

fn validate_change_object_command_record(
    record: &ChangeObjectCommandRecord,
    expected_orchestration_id: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        record.record_kind.as_str(),
        CHANGE_OBJECT_COMMAND_RECORD_KIND,
        "command.record_kind",
    )?;
    ensure_eq(
        record.orchestration_id_ref.as_str(),
        expected_orchestration_id,
        "command.orchestration_id_ref",
    )?;
    ensure_one_of(
        "command.command_class",
        CHANGE_OBJECT_ORCHESTRATION_COMMAND_CLASSES,
        &record.command_class,
    )?;
    Ok(())
}

fn validate_support_export(
    export: &ChangeObjectOrchestrationSupportExportPacket,
    orchestration: &ChangeObjectOrchestrationRecord,
    commands: &[ChangeObjectCommandRecord],
    mutation_checkpoint: &MutationCheckpointRecord,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        CHANGE_OBJECT_ORCHESTRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq(
        export.orchestration_id_ref.as_str(),
        orchestration.orchestration_id.as_str(),
        "support_export.orchestration_id_ref",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        orchestration.review_workspace_id_ref.as_str(),
        "support_export.review_workspace_id_ref",
    )?;
    if export.raw_url_export_allowed
        || export.raw_provider_payload_export_allowed
        || export.raw_path_export_allowed
        || export.raw_branch_name_export_allowed
        || export.raw_patch_body_export_allowed
    {
        return Err(change_object_orchestration_validation_error(
            "support_export must keep all raw_*_export_allowed flags false",
        ));
    }
    if export.command_id_refs.len() != commands.len() {
        return Err(change_object_orchestration_validation_error(
            "support_export.command_id_refs must match the number of commands",
        ));
    }
    if export.restart_snapshot.flow_state != orchestration.flow_state {
        return Err(change_object_orchestration_validation_error(
            "support_export.restart_snapshot.flow_state must match orchestration.flow_state",
        ));
    }
    if export.restart_snapshot.checkpoint_state != mutation_checkpoint.checkpoint_state {
        return Err(change_object_orchestration_validation_error(
            "support_export.restart_snapshot.checkpoint_state must match mutation_checkpoint.checkpoint_state",
        ));
    }
    if !export
        .consumer_surfaces
        .iter()
        .any(|s| s == "support_export")
    {
        return Err(change_object_orchestration_validation_error(
            "support_export.consumer_surfaces must include support_export",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &ChangeObjectOrchestrationInspectionRecord,
    packet: &ChangeObjectOrchestrationPacket,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        CHANGE_OBJECT_ORCHESTRATION_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq(
        inspection.orchestration_id_ref.as_str(),
        packet.orchestration.orchestration_id.as_str(),
        "inspection.orchestration_id_ref",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        packet.orchestration.review_workspace_id_ref.as_str(),
        "inspection.review_workspace_id_ref",
    )?;
    if inspection.preview_approved && inspection.preview_pending {
        return Err(change_object_orchestration_validation_error(
            "inspection cannot be both preview_approved and preview_pending",
        ));
    }
    if inspection.completed && inspection.failed {
        return Err(change_object_orchestration_validation_error(
            "inspection cannot be both completed and failed",
        ));
    }
    if inspection.completed && inspection.rolled_back {
        return Err(change_object_orchestration_validation_error(
            "inspection cannot be both completed and rolled_back",
        ));
    }
    if inspection.command_count != packet.commands.len() {
        return Err(change_object_orchestration_validation_error(
            "inspection.command_count must match the number of commands",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Utility helpers
// ---------------------------------------------------------------------------

fn compute_actionable(input: &ChangeObjectOrchestrationInput) -> bool {
    let base_actionable = matches!(
        input.flow_state.as_str(),
        "preview_approved" | "checkpoint_captured" | "executing"
    );
    let no_blocks = input.invalidation_reasons.is_empty();
    base_actionable && no_blocks
}

fn compute_blocked_reasons(input: &ChangeObjectOrchestrationInput) -> Vec<String> {
    let mut reasons = Vec::new();
    if input.flow_state == "preview_pending" {
        reasons.push("preview_not_approved".to_string());
    }
    if input.mutation_checkpoint.checkpoint_state == "missing_blocks_apply" {
        reasons.push("checkpoint_missing".to_string());
    }
    for inv in &input.invalidation_reasons {
        reasons.push(format!("invalidated:{inv}"));
    }
    reasons
}

fn contains_token(haystack: &[&str], needle: &str) -> bool {
    haystack.iter().any(|candidate| *candidate == needle)
}

fn ensure_eq(
    actual: &str,
    expected: &str,
    label: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(change_object_orchestration_validation_error(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn ensure_nonempty(
    value: &str,
    label: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    if value.trim().is_empty() {
        Err(change_object_orchestration_validation_error(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn ensure_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), ChangeObjectOrchestrationValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(change_object_orchestration_validation_error(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}
