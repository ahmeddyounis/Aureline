// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Preview-first Git mutation flows for source-control rows.
//!
//! This module owns the first bounded contract for path-level Git mutations.
//! Callers create a preview for a stage, unstage, discard, or checkpoint
//! restore request, inspect the exact target scope and checkpoint posture, and
//! then apply the preview through the same service so activity and support
//! records can quote one lineage.

use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::status::{
    ChangeKind, ConsumerProjectionBundle, GitChange, GitServiceState, GitStatusRequest,
    GitStatusService, GitStatusSnapshot,
};
use crate::{digest, hardened_git};

/// Stable record-kind tag for [`GitMutationPreview`].
pub const GIT_MUTATION_PREVIEW_RECORD_KIND: &str = "git_mutation_preview";

/// Stable record-kind tag for [`GitMutationResult`].
pub const GIT_MUTATION_RESULT_RECORD_KIND: &str = "git_mutation_result";

/// Stable record-kind tag for [`GitMutationActivityRecord`].
pub const GIT_MUTATION_ACTIVITY_RECORD_KIND: &str = "git_mutation_activity_record";

/// Stable record-kind tag for [`GitMutationSupportExportRecord`].
pub const GIT_MUTATION_SUPPORT_EXPORT_RECORD_KIND: &str = "git_mutation_support_export_record";

/// Stable record-kind tag for [`GitMutationJournalRecord`].
pub const GIT_MUTATION_JOURNAL_RECORD_KIND: &str = "git_mutation_journal_record";

const GIT_MUTATION_PREVIEW_SCHEMA_VERSION: u32 = 1;
const GIT_MUTATION_RESULT_SCHEMA_VERSION: u32 = 1;
const GIT_MUTATION_ACTIVITY_SCHEMA_VERSION: u32 = 1;
const GIT_MUTATION_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 2;
const GIT_MUTATION_JOURNAL_SCHEMA_VERSION: u32 = 1;

const MAX_GIT_MUTATION_PATHS: usize = 4096;
const MAX_GIT_MUTATION_PATH_BYTES: usize = 4096;
const MAX_GIT_MUTATION_SCOPE_BYTES: usize = 1024 * 1024;
const MAX_GIT_MUTATION_EVIDENCE_BYTES: usize = 16 * 1024 * 1024;
const MAX_GIT_MUTATION_RECORD_BYTES: usize = 16 * 1024 * 1024;
const MAX_GIT_MUTATION_METADATA_BYTES: usize = 4096;

/// Path-level Git operation reviewed by the mutation lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitMutationOperationKind {
    /// Add worktree or untracked content to the Git index.
    Stage,
    /// Remove selected paths from the Git index while keeping worktree bytes.
    Unstage,
    /// Restore tracked worktree changes from the Git index.
    Discard,
    /// Restore the state captured by a prior mutation checkpoint.
    RevertCheckpoint,
}

impl GitMutationOperationKind {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stage => "stage",
            Self::Unstage => "unstage",
            Self::Discard => "discard",
            Self::RevertCheckpoint => "revert_checkpoint",
        }
    }

    /// Canonical command id for this operation.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Stage => "cmd:git.mutation.stage",
            Self::Unstage => "cmd:git.mutation.unstage",
            Self::Discard => "cmd:git.mutation.discard",
            Self::RevertCheckpoint => "cmd:git.mutation.revert_checkpoint",
        }
    }

    /// Reviewer-facing operation label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Stage => "Stage changes",
            Self::Unstage => "Unstage changes",
            Self::Discard => "Discard worktree changes",
            Self::RevertCheckpoint => "Restore checkpoint",
        }
    }

    /// Consequence class shown in preview and support packets.
    pub const fn consequence_class(self) -> &'static str {
        match self {
            Self::Stage | Self::Unstage => "index_only",
            Self::Discard => "destructive_local_worktree",
            Self::RevertCheckpoint => "checkpoint_restore",
        }
    }

    /// Returns true when applying this operation removes worktree content.
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Discard)
    }
}

/// State of a Git mutation preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitMutationPreviewState {
    /// Scope, preview, and checkpoint posture are ready for explicit apply.
    ReadyToApply,
    /// The preview exists, but at least one selected target blocks apply.
    Blocked,
    /// Git state was unavailable or stale, so no apply may proceed.
    Degraded,
}

impl GitMutationPreviewState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToApply => "ready_to_apply",
            Self::Blocked => "blocked",
            Self::Degraded => "degraded",
        }
    }
}

/// Final state of an applied or reverted Git mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitMutationOutcomeState {
    /// Forward operation applied to every included target.
    Applied,
    /// Checkpoint restore applied to every included target.
    Reverted,
    /// No mutation was attempted because the preview was blocked.
    BlockedNoChangesMade,
    /// A Git command failed while applying or restoring the preview.
    Failed,
}

impl GitMutationOutcomeState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Reverted => "reverted",
            Self::BlockedNoChangesMade => "blocked_no_changes_made",
            Self::Failed => "failed",
        }
    }

    fn activity_state_class(self) -> &'static str {
        match self {
            Self::Applied | Self::Reverted => "completed",
            Self::BlockedNoChangesMade => "blocked",
            Self::Failed => "failed",
        }
    }
}

/// Actor identity attached to preview, apply, and support records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationActorRef {
    /// Actor class token from the local mutation lineage vocabulary.
    pub actor_class: String,
    /// Redaction-safe actor label.
    pub display_label: String,
    /// Optional stable principal or process ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
}

impl Default for GitMutationActorRef {
    fn default() -> Self {
        Self {
            actor_class: "local_user".to_string(),
            display_label: "Local user".to_string(),
            stable_id: None,
        }
    }
}

/// Request for a preview-first Git mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationRequest {
    /// Stable workspace identity copied into every downstream record.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub root_path: PathBuf,
    /// Operation being requested.
    pub operation: GitMutationOperationKind,
    /// Repository-relative or absolute paths selected by the caller.
    pub paths: Vec<PathBuf>,
    /// Actor that initiated the request.
    pub actor: GitMutationActorRef,
    /// Public row or surface ref that launched the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Timestamp supplied by the caller for deterministic exports.
    pub requested_at: String,
}

impl GitMutationRequest {
    /// Builds a path-scoped request with a derived local workspace identity.
    pub fn for_paths(
        root_path: impl Into<PathBuf>,
        operation: GitMutationOperationKind,
        paths: impl IntoIterator<Item = impl Into<PathBuf>>,
    ) -> Self {
        let root_path = root_path.into();
        let workspace_ref = root_path
            .file_name()
            .and_then(OsStr::to_str)
            .map(|label| format!("workspace.local.{}", sanitize_id(label)))
            .filter(|label| label != "workspace.local.")
            .unwrap_or_else(|| "workspace.local.root".to_string());
        Self {
            workspace_ref,
            root_path,
            operation,
            paths: paths.into_iter().map(Into::into).collect(),
            actor: GitMutationActorRef::default(),
            launch_source_ref: None,
            requested_at: "now".to_string(),
        }
    }

    /// Builds a path-scoped request with explicit identity and timestamp.
    pub fn with_observed_at(
        workspace_ref: impl Into<String>,
        root_path: impl Into<PathBuf>,
        operation: GitMutationOperationKind,
        paths: impl IntoIterator<Item = impl Into<PathBuf>>,
        requested_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            root_path: root_path.into(),
            operation,
            paths: paths.into_iter().map(Into::into).collect(),
            actor: GitMutationActorRef::default(),
            launch_source_ref: None,
            requested_at: requested_at.into(),
        }
    }

    /// Attaches a public launch-source ref to the request.
    pub fn with_launch_source_ref(mut self, launch_source_ref: impl Into<String>) -> Self {
        self.launch_source_ref = Some(launch_source_ref.into());
        self
    }
}

/// One selected path in a mutation preview or result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationTargetReview {
    /// Stable target ref used by activity, support, and journal records.
    pub target_ref: String,
    /// Path-truth ref that joins to change-list or diff rows.
    pub path_truth_ref: String,
    /// Repository-relative path selected by the caller.
    pub repo_relative_path: PathBuf,
    /// Display-safe path label.
    pub path_label: String,
    /// Git status code observed at preview time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// File-state token observed at preview time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_state_token: Option<String>,
    /// True when this path is included in the apply set.
    pub included_in_apply: bool,
    /// Optional blocking reason when this path cannot apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    /// True when this row needs a destructive-action review cue.
    pub protected_review_required: bool,
    /// Diff-preview ref that must stay visible before apply.
    pub preview_diff_ref: String,
    /// Checkpoint ref covering this path when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
}

/// Scope review shown before any Git mutation applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationScopeReview {
    /// Stable scope ref for the path set.
    pub scope_ref: String,
    /// Number of paths selected by the caller.
    pub requested_count: usize,
    /// Number of paths admitted for apply.
    pub included_count: usize,
    /// Number of paths blocked before apply.
    pub blocked_count: usize,
    /// Stable basis snapshot ref used to compute the scope.
    pub basis_snapshot_ref: String,
    /// True when apply must not use a widened or recomputed path set.
    pub scope_rebind_forbidden: bool,
    /// Selected path rows with scope, preview, and checkpoint refs.
    pub targets: Vec<GitMutationTargetReview>,
}

impl GitMutationScopeReview {
    /// Returns true when every selected row has visible scope and preview refs.
    pub fn all_rows_have_visible_scope_and_preview(&self) -> bool {
        self.targets.iter().all(|target| {
            !target.path_label.trim().is_empty()
                && !target.path_truth_ref.trim().is_empty()
                && !target.preview_diff_ref.trim().is_empty()
        })
    }
}

/// Diff-preview metadata retained without exporting raw patch bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationDiffPreview {
    /// Stable diff-preview ref.
    pub preview_diff_ref: String,
    /// Preview class for the operation.
    pub preview_class: String,
    /// Source side label shown before apply.
    pub source_side_label: String,
    /// Target side label shown before apply.
    pub target_side_label: String,
    /// True when Git produced diff bytes for the selected scope.
    pub diff_available: bool,
    /// Number of diff text lines observed before redaction.
    pub diff_line_count: usize,
    /// True when the patch contains binary-diff markers.
    pub binary_diff_present: bool,
    /// Redaction-safe reviewer label.
    pub display_label: String,
}

/// Checkpoint and recovery posture for a Git mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationCheckpointRecord {
    /// Stable checkpoint ref.
    pub checkpoint_ref: String,
    /// Checkpoint kind used for recovery.
    pub checkpoint_kind: String,
    /// Whether a checkpoint is required before apply may proceed.
    pub checkpoint_required: bool,
    /// Whether the checkpoint was captured.
    pub checkpoint_captured: bool,
    /// Recovery class from the shared preview/apply/revert vocabulary.
    pub rollback_path_class: String,
    /// Command id that restores this checkpoint.
    pub restore_command_id: String,
    /// Retention class for support and local-history surfaces.
    pub retention_class: String,
    /// Paths covered by this checkpoint.
    pub covered_path_labels: Vec<String>,
}

impl GitMutationCheckpointRecord {
    /// Returns true when a required checkpoint has a concrete restore path.
    pub fn satisfies_required_recovery(&self) -> bool {
        !self.checkpoint_required
            || (self.checkpoint_captured
                && !self.checkpoint_ref.trim().is_empty()
                && self.rollback_path_class != "no_recovery_available")
    }
}

/// Review-first preview packet for a Git mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationPreview {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable preview ref.
    pub preview_ref: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace identity copied from the request.
    pub workspace_ref: String,
    /// Repository root resolved by local Git when available.
    pub repo_root: PathBuf,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
    /// Operation being reviewed.
    pub operation: GitMutationOperationKind,
    /// Canonical command id for apply.
    pub command_id: String,
    /// User-facing operation label.
    pub operation_label: String,
    /// Current preview state.
    pub preview_state: GitMutationPreviewState,
    /// Consequence class for review sheets.
    pub consequence_class: String,
    /// Whether the operation needs explicit protected-row review.
    pub destructive_review_required: bool,
    /// Actor that initiated the preview.
    pub actor: GitMutationActorRef,
    /// Public row or surface ref that launched the preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Scope review shown before apply.
    pub scope: GitMutationScopeReview,
    /// Diff-preview metadata shown before apply.
    pub diff_preview: GitMutationDiffPreview,
    /// Checkpoint or equivalent recovery posture.
    pub checkpoint: GitMutationCheckpointRecord,
    /// Activity projection for the preview state.
    pub activity: GitMutationActivityRecord,
    /// Support-export projection for the preview state.
    pub support_export: GitMutationSupportExportRecord,
    /// In-memory evidence for the exact selected-path state that was reviewed.
    /// It is deliberately omitted from serialized/exported records so raw patch
    /// bodies cannot become portable apply authority.
    #[serde(skip)]
    basis_state: PreviewPatch,
    #[serde(skip)]
    rollback_material: GitRollbackMaterial,
    /// Digest of the public projection that was bound to the in-process
    /// evidence. A serialized or modified preview has no apply authority.
    #[serde(skip)]
    projection_digest: Option<String>,
}

impl GitMutationPreview {
    fn seal(mut self) -> Self {
        self.projection_digest = bounded_projection_digest(&self);
        if self.preview_state == GitMutationPreviewState::ReadyToApply
            && self.projection_digest.is_none()
        {
            self.preview_state = GitMutationPreviewState::Blocked;
            self.basis_state = PreviewPatch::default();
            self.rollback_material = GitRollbackMaterial::default();
        }
        self
    }

    fn projection_matches_authority(&self) -> bool {
        let Some(expected) = self.projection_digest.as_deref() else {
            return false;
        };
        bounded_projection_digest(self).as_deref() == Some(expected)
    }

    /// Returns true when apply may proceed without recomputing scope.
    pub fn ready_to_apply(&self) -> bool {
        self.preview_state == GitMutationPreviewState::ReadyToApply
            && self.scope.blocked_count == 0
            && self.scope.included_count > 0
            && self.scope.all_rows_have_visible_scope_and_preview()
            && self.checkpoint.satisfies_required_recovery()
            && self.basis_state.has_evidence()
            && self.rollback_material.supports(self.operation)
            && self.projection_matches_authority()
    }

    /// Returns true when destructive previews carry an explicit checkpoint.
    pub fn destructive_actions_have_checkpoint(&self) -> bool {
        !self.destructive_review_required || self.checkpoint.satisfies_required_recovery()
    }
}

/// Activity-center projection for a Git mutation preview or result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationActivityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable activity row id.
    pub activity_row_id: String,
    /// Activity family token.
    pub job_family: String,
    /// Lifecycle state token.
    pub state_class: String,
    /// Activity-center partition token.
    pub partition: String,
    /// Reviewer-facing summary.
    pub summary_label: String,
    /// Reviewer-facing detail label.
    pub detail_label: String,
    /// Preview ref used for lineage joins.
    pub preview_ref: String,
    /// Mutation id when an apply or restore command was attempted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_id: Option<String>,
    /// Checkpoint refs linked to the row.
    pub checkpoint_refs: Vec<String>,
    /// Command id that reopens mutation details.
    pub open_details_command_id: String,
    /// Support-export ref that carries the same attribution.
    pub support_export_ref: String,
}

/// Redaction-safe support export projection for a Git mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationSupportExportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable support-export ref.
    pub support_export_ref: String,
    /// Redaction mode for this export row.
    pub redaction_mode: String,
    /// Redaction profile governing opaque identity projections.
    pub redaction_profile_ref: String,
    /// Retention class for this export row.
    pub retention_class: String,
    /// Operation token.
    pub operation_kind: String,
    /// Phase token for preview, apply, or restore.
    pub phase: String,
    /// Domain-separated digest of the workspace identity.
    pub workspace_ref_digest: String,
    /// Domain-separated digest of the reviewed scope ref.
    pub scope_ref_digest: String,
    /// Domain-separated digest of the preview ref.
    pub preview_ref_digest: String,
    /// Result-ref digest when a mutation completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref_digest: Option<String>,
    /// Mutation-journal-ref digest when a mutation completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref_digest: Option<String>,
    /// Digests of checkpoint refs available to support and recovery surfaces.
    pub checkpoint_ref_digests: Vec<String>,
    /// Digests of evidence refs included without raw patch bodies.
    pub evidence_ref_digests: Vec<String>,
    /// Raw filesystem paths are never exportable in this record family.
    pub raw_path_export_allowed: bool,
    /// Raw actor values are never exportable in this record family.
    pub raw_actor_export_allowed: bool,
    /// Fields deliberately omitted from export.
    pub omitted_fields: Vec<String>,
}

/// Mutation-journal shaped record emitted after apply or restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationJournalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable mutation id.
    pub mutation_id: String,
    /// Canonical command id that mutated Git state.
    pub command_id: String,
    /// Actor that initiated the mutation.
    pub actor: GitMutationActorRef,
    /// Source class for the mutation.
    pub source_class: String,
    /// Scope ref copied from the preview.
    pub scope_ref: String,
    /// Target refs copied from included path rows.
    pub target_refs: Vec<String>,
    /// Timestamp when apply started.
    pub started_at: String,
    /// Timestamp when apply resolved.
    pub resolved_at: String,
    /// Reversal class advertised for this mutation.
    pub reversal_class: String,
    /// Checkpoint refs linked to the mutation.
    pub checkpoint_refs: Vec<String>,
    /// Redaction class for support exports.
    pub redaction_class: String,
    /// Redaction-safe side-effect summary.
    pub side_effect_summary: String,
}

/// Result packet emitted after applying or restoring a preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitMutationResult {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable result ref.
    pub result_ref: String,
    /// Preview ref this result applied.
    pub preview_ref: String,
    /// Timestamp supplied by the caller.
    pub resolved_at: String,
    /// Workspace identity copied from the preview.
    pub workspace_ref: String,
    /// Repository root copied from the preview.
    pub repo_root: PathBuf,
    /// Source snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
    /// Operation that was applied.
    pub operation: GitMutationOperationKind,
    /// Final outcome state.
    pub outcome_state: GitMutationOutcomeState,
    /// Paths that were included in the attempted mutation.
    pub applied_targets: Vec<GitMutationTargetReview>,
    /// Paths that blocked before mutation.
    pub blocked_targets: Vec<GitMutationTargetReview>,
    /// Checkpoint or equivalent recovery posture.
    pub checkpoint: GitMutationCheckpointRecord,
    /// Mutation-journal shaped lineage record.
    pub mutation_journal: GitMutationJournalRecord,
    /// Activity projection for the result.
    pub activity: GitMutationActivityRecord,
    /// Support-export projection for the result.
    pub support_export: GitMutationSupportExportRecord,
    /// True when a checkpoint restore preview can be opened.
    pub rollback_available: bool,
    /// Command id that opens the restore flow.
    pub revert_command_id: String,
    /// Failure reason when apply failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(skip)]
    rollback_material: GitRollbackMaterial,
    /// Digest binding the public result projection to its in-process restore
    /// material. Serialized or modified results cannot mint restore authority.
    #[serde(skip)]
    projection_digest: Option<String>,
}

impl GitMutationResult {
    fn seal(mut self) -> Self {
        self.projection_digest = bounded_projection_digest(&self);
        if self.rollback_available && self.projection_digest.is_none() {
            self.rollback_available = false;
            self.rollback_material = GitRollbackMaterial::default();
        }
        self
    }

    fn projection_matches_authority(&self) -> bool {
        let Some(expected) = self.projection_digest.as_deref() else {
            return false;
        };
        bounded_projection_digest(self).as_deref() == Some(expected)
    }

    /// Returns true when activity and support rows cite the journal record.
    pub fn attribution_is_exportable(&self) -> bool {
        self.activity.mutation_id.as_deref() == Some(self.mutation_journal.mutation_id.as_str())
            && self.support_export.mutation_journal_ref_digest.as_deref()
                == Some(
                    mutation_support_digest(
                        "mutation_journal_ref",
                        &self.mutation_journal.mutation_id,
                    )
                    .as_str(),
                )
            && self
                .support_export
                .checkpoint_ref_digests
                .contains(&mutation_support_digest(
                    "checkpoint_ref",
                    &self.checkpoint.checkpoint_ref,
                ))
    }

    /// Projects this Git result into the shared local-history actor-lineage row.
    pub fn local_history_actor_lineage_row(&self) -> aureline_history::ActorLineageRow {
        aureline_history::ActorLineageRow::from_git_mutation(
            aureline_history::GitMutationLineageInput {
                row_id: format!("{}.local_history_lineage", self.result_ref),
                display_label: format!(
                    "{} {}",
                    self.operation.label(),
                    self.outcome_state.as_str()
                ),
                mutation_journal_ref: self.mutation_journal.mutation_id.clone(),
                command_id: self.mutation_journal.command_id.clone(),
                actor_class: "git_mutation".to_owned(),
                source_class: "human_local".to_owned(),
                reversal_class: self.mutation_journal.reversal_class.clone(),
                redaction_class: "metadata_only".to_owned(),
                checkpoint_ref: Some(self.checkpoint.checkpoint_ref.clone()),
                side_effect_summary: self.mutation_journal.side_effect_summary.clone(),
            },
        )
    }

    /// Projects this Git result into an export-safe local-history alpha packet.
    pub fn local_history_alpha_packet(
        &self,
        produced_at: impl Into<String>,
    ) -> aureline_history::LocalHistoryAlphaPacket {
        aureline_history::LocalHistoryAlphaPacket::new(
            format!("{}.local_history_alpha", self.result_ref),
            produced_at,
            aureline_history::LocalHistoryConsumerSurface::GitMutationReview,
        )
        .with_actor_lineage_row(self.local_history_actor_lineage_row())
    }
}

/// Output captured from a Git command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitMutationCommandOutput {
    /// True when Git exited successfully.
    pub success: bool,
    /// Process exit status code when available.
    pub status_code: Option<i32>,
    /// Captured stdout bytes.
    pub stdout: Vec<u8>,
    /// Captured stderr bytes.
    pub stderr: Vec<u8>,
}

/// Error raised before a Git command can be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitMutationBackendError {
    /// Redaction-safe error message.
    pub message: String,
}

impl std::fmt::Display for GitMutationBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GitMutationBackendError {}

/// Backend used by [`GitMutationService`] to execute local Git commands.
pub trait GitMutationBackend {
    /// Runs `git -C root args`.
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError>;

    /// Runs `git -C root args` with bytes written to stdin.
    fn run_git_with_stdin(
        &self,
        root: &Path,
        args: &[String],
        stdin: &[u8],
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError>;
}

/// Git backend that shells out to the system `git` executable.
#[derive(Debug, Clone)]
pub struct SystemGitMutationBackend {
    git_binary: PathBuf,
}

impl Default for SystemGitMutationBackend {
    fn default() -> Self {
        Self::new("git")
    }
}

impl SystemGitMutationBackend {
    /// Creates a backend that invokes `git_binary`.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl GitMutationBackend for SystemGitMutationBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError> {
        let output = hardened_git::run(hardened_git::command(&self.git_binary, root, args))
            .map_err(|_| GitMutationBackendError {
                message: "Git mutation command could not be completed safely".to_string(),
            })?;
        Ok(GitMutationCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }

    fn run_git_with_stdin(
        &self,
        root: &Path,
        args: &[String],
        stdin: &[u8],
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError> {
        let output = hardened_git::run_with_stdin(
            hardened_git::command(&self.git_binary, root, args),
            stdin,
        )
        .map_err(|_| GitMutationBackendError {
            message: "Git mutation command could not be completed safely".to_string(),
        })?;
        Ok(GitMutationCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

/// Service that creates, applies, and restores Git mutation previews.
#[derive(Debug, Clone)]
pub struct GitMutationService<B = SystemGitMutationBackend> {
    backend: B,
}

impl Default for GitMutationService<SystemGitMutationBackend> {
    fn default() -> Self {
        Self::new(SystemGitMutationBackend::default())
    }
}

impl<B: GitMutationBackend> GitMutationService<B> {
    /// Creates a service backed by `backend`.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Builds a reviewable preview without mutating Git state.
    pub fn preview(&self, request: &GitMutationRequest) -> GitMutationPreview {
        if let Err(reason) = validate_mutation_request_metadata(request) {
            return self.degraded_preview(
                request,
                request.root_path.clone(),
                "git.status.unavailable.invalid-metadata".to_string(),
                reason,
            );
        }
        if let Err(reason) = bounded_requested_path_shape(&request.paths) {
            return self.degraded_preview(
                request,
                request.root_path.clone(),
                "git.status.unavailable.invalid-scope".to_string(),
                reason,
            );
        }
        let status_request = GitStatusRequest::with_observed_at(
            request.workspace_ref.clone(),
            request.root_path.clone(),
            request.requested_at.clone(),
        );
        let snapshot = GitStatusService::default().snapshot(&status_request);
        let truth_source_ref =
            ConsumerProjectionBundle::from_snapshot(request.requested_at.clone(), &snapshot)
                .truth_source_ref;
        let repo_root = snapshot
            .repository
            .as_ref()
            .map(|repo| repo.repo_root.clone())
            .unwrap_or_else(|| request.root_path.clone());

        if snapshot.service_state != GitServiceState::Current {
            return self.degraded_preview(
                request,
                repo_root,
                truth_source_ref,
                snapshot.service_state.as_str(),
            );
        }

        let repo_paths = match bounded_normalized_request_paths(
            &request.paths,
            &request.root_path,
            &repo_root,
        ) {
            Ok(paths) => paths,
            Err(reason) => {
                return self.degraded_preview(request, repo_root, truth_source_ref, reason)
            }
        };
        let preview_ref = preview_ref(&request.workspace_ref, request.operation, &repo_paths);
        let preview_diff_ref = format!("{}.diff", preview_ref);
        let checkpoint_ref = format!("{}.checkpoint", preview_ref);
        let (targets, blocked_count) = target_reviews(
            &snapshot.changes,
            request.operation,
            &request.workspace_ref,
            &repo_paths,
            &preview_diff_ref,
            &checkpoint_ref,
        );
        let included_count = targets
            .iter()
            .filter(|target| target.included_in_apply)
            .count();
        let basis_state = self.capture_path_state(&repo_root, &repo_paths, &snapshot);
        let preview_state = if request.paths.is_empty()
            || blocked_count > 0
            || included_count == 0
            || !basis_state.has_evidence()
        {
            GitMutationPreviewState::Blocked
        } else {
            GitMutationPreviewState::ReadyToApply
        };
        let rollback_material =
            rollback_material_for(request.operation, repo_paths.clone(), basis_state.clone());
        let scope = GitMutationScopeReview {
            scope_ref: format!("{}.scope", preview_ref),
            requested_count: request.paths.len(),
            included_count,
            blocked_count,
            basis_snapshot_ref: truth_source_ref.clone(),
            scope_rebind_forbidden: true,
            targets,
        };
        let diff_preview =
            diff_preview_for(request.operation, &preview_diff_ref, &rollback_material);
        let checkpoint = checkpoint_for(
            &checkpoint_ref,
            request.operation,
            preview_state,
            &scope,
            &rollback_material,
        );
        let support_export = support_export_for_preview(
            &preview_ref,
            request.operation,
            &request.workspace_ref,
            &scope.scope_ref,
            &checkpoint,
        );
        let activity = activity_for_preview(
            &preview_ref,
            request.operation,
            preview_state,
            &checkpoint,
            &support_export.support_export_ref,
        );

        GitMutationPreview {
            record_kind: GIT_MUTATION_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_MUTATION_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            operation: request.operation,
            command_id: request.operation.command_id().to_string(),
            operation_label: request.operation.label().to_string(),
            preview_state,
            consequence_class: request.operation.consequence_class().to_string(),
            destructive_review_required: request.operation.is_destructive(),
            actor: request.actor.clone(),
            launch_source_ref: request.launch_source_ref.clone(),
            scope,
            diff_preview,
            checkpoint,
            activity,
            support_export,
            basis_state,
            rollback_material,
            projection_digest: None,
        }
        .seal()
    }

    /// Applies an admitted preview and returns an attributable result packet.
    pub fn apply(
        &self,
        preview: &GitMutationPreview,
        resolved_at: impl Into<String>,
    ) -> GitMutationResult {
        let resolved_at = resolved_at.into();
        if !valid_mutation_metadata(&resolved_at) {
            return result_for_preview(
                preview,
                "unavailable",
                GitMutationOutcomeState::BlockedNoChangesMade,
                Some("apply metadata is outside the review boundary".to_string()),
            );
        }
        if !preview.ready_to_apply() {
            return result_for_blocked_preview(preview, &resolved_at);
        }
        if self
            .validate_preview_is_current(preview, &resolved_at)
            .is_err()
        {
            return result_for_preview(
                preview,
                &resolved_at,
                GitMutationOutcomeState::BlockedNoChangesMade,
                Some("preview evidence is stale; refresh before apply".to_string()),
            );
        }

        let output = self.apply_preview(preview);
        let (outcome_state, failure_reason) = match output {
            Ok(output) if output.success => {
                if preview.operation == GitMutationOperationKind::RevertCheckpoint {
                    (GitMutationOutcomeState::Reverted, None)
                } else {
                    (GitMutationOutcomeState::Applied, None)
                }
            }
            Ok(output) => (
                GitMutationOutcomeState::Failed,
                Some(git_mutation_failure_reason(&output)),
            ),
            Err(_) => (
                GitMutationOutcomeState::Failed,
                Some("Git mutation command could not be completed safely".to_string()),
            ),
        };
        result_for_preview(preview, &resolved_at, outcome_state, failure_reason)
    }

    /// Builds the review packet for restoring a prior mutation checkpoint.
    pub fn preview_revert(
        &self,
        result: &GitMutationResult,
        requested_at: impl Into<String>,
    ) -> GitMutationPreview {
        let mut requested_at = requested_at.into();
        if !valid_mutation_metadata(&requested_at) {
            requested_at = "unavailable".to_string();
            return self.degraded_revert_preview(
                result,
                requested_at,
                "checkpoint review metadata is outside the review boundary",
            );
        }
        if !result.projection_matches_authority() {
            return self.degraded_revert_preview(
                result,
                requested_at,
                "checkpoint authority is unavailable or changed",
            );
        }
        if result.applied_targets.len() > MAX_GIT_MUTATION_PATHS {
            return self.degraded_revert_preview(
                result,
                requested_at,
                "checkpoint path count exceeds the review limit",
            );
        }
        let candidate_paths = result
            .applied_targets
            .iter()
            .map(|target| target.repo_relative_path.clone())
            .collect::<Vec<_>>();
        let repo_paths = match bounded_normalized_paths(&candidate_paths, &result.repo_root) {
            Ok(paths) if paths == candidate_paths => paths,
            _ => {
                return self.degraded_revert_preview(
                    result,
                    requested_at,
                    "checkpoint path scope is outside the review boundary",
                )
            }
        };
        let status_request = GitStatusRequest::with_observed_at(
            result.workspace_ref.clone(),
            result.repo_root.clone(),
            requested_at.clone(),
        );
        let snapshot = GitStatusService::default().snapshot(&status_request);
        let truth_source_ref =
            ConsumerProjectionBundle::from_snapshot(requested_at.clone(), &snapshot)
                .truth_source_ref;
        let repository_is_current = snapshot.service_state == GitServiceState::Current
            && snapshot
                .repository
                .as_ref()
                .is_some_and(|repository| repository.repo_root == result.repo_root);
        let basis_state = if repository_is_current {
            self.capture_path_state(&result.repo_root, &repo_paths, &snapshot)
        } else {
            PreviewPatch::default()
        };
        let preview_ref = preview_ref(
            &result.workspace_ref,
            GitMutationOperationKind::RevertCheckpoint,
            &repo_paths,
        );
        let preview_diff_ref = format!("{}.diff", preview_ref);
        let targets = repo_paths
            .iter()
            .map(|path| {
                let change = snapshot.changes.iter().find(|change| change.path == *path);
                included_target(
                    &result.workspace_ref,
                    path,
                    change.map(|change| change.status_code.clone()),
                    change.map(file_state_token),
                    &preview_diff_ref,
                    &result.checkpoint.checkpoint_ref,
                    true,
                )
            })
            .collect::<Vec<_>>();
        let scope = GitMutationScopeReview {
            scope_ref: format!("{}.scope", preview_ref),
            requested_count: targets.len(),
            included_count: targets.len(),
            blocked_count: 0,
            basis_snapshot_ref: truth_source_ref.clone(),
            scope_rebind_forbidden: true,
            targets,
        };
        let rollback_material = result.rollback_material.clone();
        let rollback_is_available = result.rollback_available
            && rollback_material.supports(GitMutationOperationKind::RevertCheckpoint)
            && rollback_material.repo_root == result.repo_root
            && basis_state.repo_root == rollback_material.repo_root
            && rollback_material.paths == repo_paths
            && basis_state.has_evidence();
        let checkpoint = GitMutationCheckpointRecord {
            checkpoint_ref: result.checkpoint.checkpoint_ref.clone(),
            checkpoint_kind: result.checkpoint.checkpoint_kind.clone(),
            checkpoint_required: true,
            checkpoint_captured: rollback_is_available,
            rollback_path_class: "restore_from_checkpoint".to_string(),
            restore_command_id: GitMutationOperationKind::RevertCheckpoint
                .command_id()
                .to_string(),
            retention_class: result.checkpoint.retention_class.clone(),
            covered_path_labels: result.checkpoint.covered_path_labels.clone(),
        };
        let diff_preview = diff_preview_for(
            GitMutationOperationKind::RevertCheckpoint,
            &preview_diff_ref,
            &rollback_material,
        );
        let preview_state = if rollback_is_available {
            GitMutationPreviewState::ReadyToApply
        } else {
            GitMutationPreviewState::Blocked
        };
        let support_export = support_export_for_preview(
            &preview_ref,
            GitMutationOperationKind::RevertCheckpoint,
            &result.workspace_ref,
            &scope.scope_ref,
            &checkpoint,
        );
        let activity = activity_for_preview(
            &preview_ref,
            GitMutationOperationKind::RevertCheckpoint,
            preview_state,
            &checkpoint,
            &support_export.support_export_ref,
        );

        GitMutationPreview {
            record_kind: GIT_MUTATION_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_MUTATION_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at: requested_at,
            workspace_ref: result.workspace_ref.clone(),
            repo_root: result.repo_root.clone(),
            truth_source_ref,
            operation: GitMutationOperationKind::RevertCheckpoint,
            command_id: GitMutationOperationKind::RevertCheckpoint
                .command_id()
                .to_string(),
            operation_label: GitMutationOperationKind::RevertCheckpoint
                .label()
                .to_string(),
            preview_state,
            consequence_class: GitMutationOperationKind::RevertCheckpoint
                .consequence_class()
                .to_string(),
            destructive_review_required: false,
            actor: result.mutation_journal.actor.clone(),
            launch_source_ref: Some(result.result_ref.clone()),
            scope,
            diff_preview,
            checkpoint,
            activity,
            support_export,
            basis_state,
            rollback_material,
            projection_digest: None,
        }
        .seal()
    }

    fn degraded_revert_preview(
        &self,
        result: &GitMutationResult,
        requested_at: String,
        reason: &str,
    ) -> GitMutationPreview {
        let mut request = GitMutationRequest::with_observed_at(
            result.workspace_ref.clone(),
            result.repo_root.clone(),
            GitMutationOperationKind::RevertCheckpoint,
            Vec::<PathBuf>::new(),
            requested_at,
        )
        .with_launch_source_ref(result.result_ref.clone());
        request.actor = result.mutation_journal.actor.clone();
        self.degraded_preview(
            &request,
            result.repo_root.clone(),
            result.truth_source_ref.clone(),
            reason,
        )
    }

    /// Restores a prior mutation result through its checkpoint preview.
    pub fn revert(
        &self,
        result: &GitMutationResult,
        resolved_at: impl Into<String>,
    ) -> GitMutationResult {
        let resolved_at = resolved_at.into();
        let preview = self.preview_revert(result, resolved_at.clone());
        self.apply(&preview, resolved_at)
    }

    fn degraded_preview(
        &self,
        request: &GitMutationRequest,
        repo_root: PathBuf,
        truth_source_ref: String,
        reason: &str,
    ) -> GitMutationPreview {
        let workspace_ref = valid_mutation_metadata(&request.workspace_ref)
            .then(|| request.workspace_ref.clone())
            .unwrap_or_else(|| "workspace.unavailable".to_string());
        let generated_at = valid_mutation_metadata(&request.requested_at)
            .then(|| request.requested_at.clone())
            .unwrap_or_else(|| "unavailable".to_string());
        let actor = valid_mutation_actor(&request.actor)
            .then(|| request.actor.clone())
            .unwrap_or_default();
        let launch_source_ref = request
            .launch_source_ref
            .as_ref()
            .filter(|value| valid_mutation_metadata(value))
            .cloned();
        let safe_paths = bounded_normalized_paths(&request.paths, &repo_root).unwrap_or_default();
        let preview_ref = preview_ref(&workspace_ref, request.operation, &safe_paths);
        let preview_diff_ref = format!("{}.diff", preview_ref);
        let checkpoint_ref = format!("{}.checkpoint", preview_ref);
        let targets = safe_paths
            .iter()
            .map(|path| {
                blocked_target(
                    &workspace_ref,
                    path,
                    None,
                    None,
                    &preview_diff_ref,
                    Some(&checkpoint_ref),
                    format!("Git service degraded: {reason}"),
                    request.operation.is_destructive(),
                )
            })
            .collect::<Vec<_>>();
        let scope = GitMutationScopeReview {
            scope_ref: format!("{}.scope", preview_ref),
            requested_count: request.paths.len(),
            included_count: 0,
            blocked_count: request.paths.len(),
            basis_snapshot_ref: truth_source_ref.clone(),
            scope_rebind_forbidden: true,
            targets,
        };
        let rollback_material = GitRollbackMaterial::default();
        let diff_preview =
            diff_preview_for(request.operation, &preview_diff_ref, &rollback_material);
        let checkpoint = GitMutationCheckpointRecord {
            checkpoint_ref,
            checkpoint_kind: "unavailable".to_string(),
            checkpoint_required: request.operation.is_destructive(),
            checkpoint_captured: false,
            rollback_path_class: "no_recovery_available".to_string(),
            restore_command_id: GitMutationOperationKind::RevertCheckpoint
                .command_id()
                .to_string(),
            retention_class: "none".to_string(),
            covered_path_labels: Vec::new(),
        };
        let support_export = support_export_for_preview(
            &preview_ref,
            request.operation,
            &workspace_ref,
            &scope.scope_ref,
            &checkpoint,
        );
        let activity = activity_for_preview(
            &preview_ref,
            request.operation,
            GitMutationPreviewState::Degraded,
            &checkpoint,
            &support_export.support_export_ref,
        );
        GitMutationPreview {
            record_kind: GIT_MUTATION_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_MUTATION_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at,
            workspace_ref,
            repo_root,
            truth_source_ref,
            operation: request.operation,
            command_id: request.operation.command_id().to_string(),
            operation_label: request.operation.label().to_string(),
            preview_state: GitMutationPreviewState::Degraded,
            consequence_class: request.operation.consequence_class().to_string(),
            destructive_review_required: request.operation.is_destructive(),
            actor,
            launch_source_ref,
            scope,
            diff_preview,
            checkpoint,
            activity,
            support_export,
            basis_state: PreviewPatch::default(),
            rollback_material,
            projection_digest: None,
        }
        .seal()
    }

    fn capture_path_state(
        &self,
        repo_root: &Path,
        paths: &[PathBuf],
        snapshot: &GitStatusSnapshot,
    ) -> PreviewPatch {
        if paths.is_empty() {
            return PreviewPatch::default();
        }
        let Some(repository) = snapshot.repository.as_ref() else {
            return PreviewPatch::default();
        };
        let changes = &snapshot.changes;
        let Some(before_index_patch) = self.run_git_diff_bytes(repo_root, diff_cached_args(paths))
        else {
            return PreviewPatch::default();
        };
        let Some(mut worktree_patch) =
            self.run_git_diff_bytes(repo_root, diff_worktree_args(paths))
        else {
            return PreviewPatch::default();
        };
        if combined_evidence_len(&before_index_patch, &worktree_patch)
            > MAX_GIT_MUTATION_EVIDENCE_BYTES
        {
            return PreviewPatch::default();
        }
        for path in paths.iter().filter(|path| {
            changes
                .iter()
                .any(|change| change.path == **path && change.change_kind == ChangeKind::Untracked)
        }) {
            let Some(untracked_patch) =
                self.run_git_diff_bytes(repo_root, diff_untracked_args(path))
            else {
                return PreviewPatch::default();
            };
            if !append_evidence_bounded(&before_index_patch, &mut worktree_patch, &untracked_patch)
            {
                return PreviewPatch::default();
            }
        }
        PreviewPatch {
            captured: true,
            repo_root: repository.repo_root.clone(),
            repository_ref: repository.repo_ref.clone(),
            worktree_ref: repository.worktree_ref.clone(),
            head_oid: snapshot.head.head_oid.clone(),
            branch_ref: snapshot.head.branch_ref.clone(),
            before_index_patch: before_index_patch.into(),
            worktree_patch: worktree_patch.into(),
        }
    }

    fn run_git_diff_bytes(&self, repo_root: &Path, args: Vec<String>) -> Option<Vec<u8>> {
        match self.backend.run_git(repo_root, &args) {
            Ok(output)
                if (output.success || output.status_code == Some(1))
                    && output.stdout.len() <= MAX_GIT_MUTATION_EVIDENCE_BYTES =>
            {
                Some(output.stdout)
            }
            _ => None,
        }
    }

    fn validate_preview_is_current(
        &self,
        preview: &GitMutationPreview,
        observed_at: &str,
    ) -> Result<(), ()> {
        let paths = preview
            .scope
            .targets
            .iter()
            .map(|target| target.repo_relative_path.clone())
            .collect::<Vec<_>>();
        match bounded_normalized_paths(&paths, &preview.repo_root) {
            Ok(normalized) if normalized == paths => {}
            _ => return Err(()),
        }
        let expected_preview_ref = preview_ref(&preview.workspace_ref, preview.operation, &paths);
        let expected_diff_ref = format!("{expected_preview_ref}.diff");
        let expected_checkpoint_ref = format!("{expected_preview_ref}.checkpoint");
        if preview.record_kind != GIT_MUTATION_PREVIEW_RECORD_KIND
            || preview.schema_version != GIT_MUTATION_PREVIEW_SCHEMA_VERSION
            || preview.preview_ref != expected_preview_ref
            || preview.command_id != preview.operation.command_id()
            || preview.operation_label != preview.operation.label()
            || preview.consequence_class != preview.operation.consequence_class()
            || preview.destructive_review_required != preview.operation.is_destructive()
            || preview.diff_preview.preview_diff_ref != expected_diff_ref
            || preview.scope.scope_ref != format!("{expected_preview_ref}.scope")
            || preview.scope.basis_snapshot_ref != preview.truth_source_ref
            || !preview.scope.scope_rebind_forbidden
            || preview.scope.requested_count != paths.len()
            || preview.scope.included_count != paths.len()
            || preview.scope.blocked_count != 0
            || preview
                .scope
                .targets
                .iter()
                .any(|target| !target.included_in_apply || target.blocked_reason.is_some())
            || preview.rollback_material.repo_root != preview.repo_root
            || preview.rollback_material.paths != paths
            || !preview.rollback_material.supports(preview.operation)
        {
            return Err(());
        }

        let status_request = GitStatusRequest::with_observed_at(
            preview.workspace_ref.clone(),
            preview.repo_root.clone(),
            observed_at,
        );
        let snapshot = GitStatusService::default().snapshot(&status_request);
        if snapshot.service_state != GitServiceState::Current
            || snapshot
                .repository
                .as_ref()
                .map_or(true, |repository| repository.repo_root != preview.repo_root)
        {
            return Err(());
        }
        let current_state = self.capture_path_state(&preview.repo_root, &paths, &snapshot);
        if !current_state.has_evidence() || current_state != preview.basis_state {
            return Err(());
        }

        match preview.operation {
            GitMutationOperationKind::RevertCheckpoint => {
                for target in &preview.scope.targets {
                    let change = snapshot
                        .changes
                        .iter()
                        .find(|change| change.path == target.repo_relative_path);
                    if target.status_code.as_deref()
                        != change.map(|change| change.status_code.as_str())
                        || target.file_state_token.as_deref()
                            != change.map(file_state_token).as_deref()
                        || target.preview_diff_ref != expected_diff_ref
                        || target.checkpoint_ref.as_deref()
                            != Some(preview.checkpoint.checkpoint_ref.as_str())
                    {
                        return Err(());
                    }
                }
            }
            operation => {
                let (expected_targets, blocked_count) = target_reviews(
                    &snapshot.changes,
                    operation,
                    &preview.workspace_ref,
                    &paths,
                    &expected_diff_ref,
                    &expected_checkpoint_ref,
                );
                if blocked_count != 0
                    || expected_targets != preview.scope.targets
                    || preview.checkpoint.checkpoint_ref != expected_checkpoint_ref
                    || rollback_material_for(operation, paths, current_state)
                        != preview.rollback_material
                {
                    return Err(());
                }
            }
        }
        Ok(())
    }

    fn apply_preview(
        &self,
        preview: &GitMutationPreview,
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError> {
        match preview.operation {
            GitMutationOperationKind::Stage => self.backend.run_git_with_stdin(
                &preview.repo_root,
                &git_apply_args(&["--cached"]),
                preview.basis_state.worktree_patch.as_ref(),
            ),
            GitMutationOperationKind::Unstage => self.backend.run_git_with_stdin(
                &preview.repo_root,
                &git_apply_args(&["--cached", "--reverse"]),
                preview.basis_state.before_index_patch.as_ref(),
            ),
            GitMutationOperationKind::Discard => self.backend.run_git_with_stdin(
                &preview.repo_root,
                &git_apply_args(&["--reverse"]),
                preview.basis_state.worktree_patch.as_ref(),
            ),
            GitMutationOperationKind::RevertCheckpoint => {
                self.restore_checkpoint(&preview.repo_root, &preview.rollback_material)
            }
        }
    }

    fn restore_checkpoint(
        &self,
        repo_root: &Path,
        material: &GitRollbackMaterial,
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError> {
        if material.repo_root != repo_root {
            return Err(GitMutationBackendError {
                message: "checkpoint target does not match the reviewed repository".to_string(),
            });
        }
        match material.action {
            GitRollbackAction::None => Ok(GitMutationCommandOutput {
                success: true,
                status_code: Some(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            }),
            GitRollbackAction::ReverseReviewedPatchInIndex => self.backend.run_git_with_stdin(
                repo_root,
                &git_apply_args(&["--cached", "--reverse"]),
                material.worktree_patch.as_ref(),
            ),
            GitRollbackAction::ApplyCapturedIndexPatch => self.backend.run_git_with_stdin(
                repo_root,
                &git_apply_args(&["--cached"]),
                material.before_index_patch.as_ref(),
            ),
            GitRollbackAction::ApplyWorktreePatch => {
                if material.worktree_patch.is_empty() {
                    return Ok(GitMutationCommandOutput {
                        success: true,
                        status_code: Some(0),
                        stdout: Vec::new(),
                        stderr: Vec::new(),
                    });
                }
                self.backend.run_git_with_stdin(
                    repo_root,
                    &["apply", "--whitespace=nowarn", "-"]
                        .into_iter()
                        .map(str::to_string)
                        .collect::<Vec<_>>(),
                    material.worktree_patch.as_ref(),
                )
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PreviewPatch {
    captured: bool,
    repo_root: PathBuf,
    repository_ref: String,
    worktree_ref: String,
    head_oid: Option<String>,
    branch_ref: Option<String>,
    before_index_patch: Arc<[u8]>,
    worktree_patch: Arc<[u8]>,
}

impl PreviewPatch {
    fn has_evidence(&self) -> bool {
        self.captured
            && self.repo_root.is_absolute()
            && !self.repository_ref.is_empty()
            && !self.worktree_ref.is_empty()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum GitRollbackAction {
    #[default]
    None,
    ReverseReviewedPatchInIndex,
    ApplyCapturedIndexPatch,
    ApplyWorktreePatch,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct GitRollbackMaterial {
    action: GitRollbackAction,
    repo_root: PathBuf,
    before_index_patch: Arc<[u8]>,
    worktree_patch: Arc<[u8]>,
    paths: Vec<PathBuf>,
}

impl GitRollbackMaterial {
    fn supports(&self, operation: GitMutationOperationKind) -> bool {
        if self.paths.is_empty() {
            return false;
        }
        if !self.repo_root.is_absolute() {
            return false;
        }
        match operation {
            GitMutationOperationKind::Stage => {
                self.action == GitRollbackAction::ReverseReviewedPatchInIndex
                    && !self.worktree_patch.is_empty()
            }
            GitMutationOperationKind::Unstage => {
                self.action == GitRollbackAction::ApplyCapturedIndexPatch
                    && !self.before_index_patch.is_empty()
            }
            GitMutationOperationKind::Discard => {
                self.action == GitRollbackAction::ApplyWorktreePatch
                    && !self.worktree_patch.is_empty()
            }
            GitMutationOperationKind::RevertCheckpoint => self.action != GitRollbackAction::None,
        }
    }
}

fn rollback_material_for(
    operation: GitMutationOperationKind,
    paths: Vec<PathBuf>,
    patch: PreviewPatch,
) -> GitRollbackMaterial {
    let action = match operation {
        GitMutationOperationKind::Stage => GitRollbackAction::ReverseReviewedPatchInIndex,
        GitMutationOperationKind::Unstage => GitRollbackAction::ApplyCapturedIndexPatch,
        GitMutationOperationKind::Discard => GitRollbackAction::ApplyWorktreePatch,
        GitMutationOperationKind::RevertCheckpoint => GitRollbackAction::None,
    };
    GitRollbackMaterial {
        action,
        repo_root: patch.repo_root,
        before_index_patch: patch.before_index_patch,
        worktree_patch: patch.worktree_patch,
        paths,
    }
}

fn target_reviews(
    changes: &[GitChange],
    operation: GitMutationOperationKind,
    workspace_ref: &str,
    paths: &[PathBuf],
    preview_diff_ref: &str,
    checkpoint_ref: &str,
) -> (Vec<GitMutationTargetReview>, usize) {
    let mut blocked_count = 0usize;
    let targets = paths
        .iter()
        .map(|path| {
            let change = changes.iter().find(|change| change.path == *path);
            let (included, reason) = eligibility(operation, change);
            if !included {
                blocked_count += 1;
            }
            let status_code = change.map(|change| change.status_code.clone());
            let file_state_token = change.map(file_state_token);
            if included {
                included_target(
                    workspace_ref,
                    path,
                    status_code,
                    file_state_token,
                    preview_diff_ref,
                    checkpoint_ref,
                    operation.is_destructive()
                        || operation == GitMutationOperationKind::RevertCheckpoint,
                )
            } else {
                blocked_target(
                    workspace_ref,
                    path,
                    status_code,
                    file_state_token,
                    preview_diff_ref,
                    Some(checkpoint_ref),
                    reason.unwrap_or_else(|| "selected path is not eligible".to_string()),
                    operation.is_destructive(),
                )
            }
        })
        .collect();
    (targets, blocked_count)
}

fn eligibility(
    operation: GitMutationOperationKind,
    change: Option<&GitChange>,
) -> (bool, Option<String>) {
    let Some(change) = change else {
        return (false, Some("selected path has no Git change".to_string()));
    };
    if change.is_conflicted {
        return (
            false,
            Some("conflicted paths require conflict review before mutation".to_string()),
        );
    }
    match operation {
        GitMutationOperationKind::Stage => {
            if change.is_unstaged || change.change_kind == ChangeKind::Untracked {
                (true, None)
            } else {
                (
                    false,
                    Some("selected path has no unstaged content to stage".to_string()),
                )
            }
        }
        GitMutationOperationKind::Unstage => {
            if change.is_staged {
                (true, None)
            } else {
                (
                    false,
                    Some("selected path has no staged content to unstage".to_string()),
                )
            }
        }
        GitMutationOperationKind::Discard => {
            if change.change_kind == ChangeKind::Untracked {
                (
                    false,
                    Some(
                        "untracked discard is blocked until delete preview has a byte checkpoint"
                            .to_string(),
                    ),
                )
            } else if change.is_unstaged {
                (true, None)
            } else {
                (
                    false,
                    Some("selected path has no worktree change to discard".to_string()),
                )
            }
        }
        GitMutationOperationKind::RevertCheckpoint => (
            false,
            Some("checkpoint restore must start from a result".to_string()),
        ),
    }
}

fn included_target(
    workspace_ref: &str,
    path: &Path,
    status_code: Option<String>,
    file_state_token: Option<String>,
    preview_diff_ref: &str,
    checkpoint_ref: &str,
    protected_review_required: bool,
) -> GitMutationTargetReview {
    let path_label = path.to_string_lossy().to_string();
    let target_ref = opaque_mutation_ref(
        "git.mutation.target",
        &[workspace_ref.as_bytes(), path_label.as_bytes()],
    );
    let path_truth_ref = opaque_mutation_ref(
        "path.truth.git.mutation",
        &[workspace_ref.as_bytes(), path_label.as_bytes()],
    );
    GitMutationTargetReview {
        target_ref,
        path_truth_ref,
        repo_relative_path: path.to_path_buf(),
        path_label,
        status_code,
        file_state_token,
        included_in_apply: true,
        blocked_reason: None,
        protected_review_required,
        preview_diff_ref: preview_diff_ref.to_string(),
        checkpoint_ref: Some(checkpoint_ref.to_string()),
    }
}

#[allow(clippy::too_many_arguments)]
fn blocked_target(
    workspace_ref: &str,
    path: &Path,
    status_code: Option<String>,
    file_state_token: Option<String>,
    preview_diff_ref: &str,
    checkpoint_ref: Option<&str>,
    reason: String,
    protected_review_required: bool,
) -> GitMutationTargetReview {
    let path_label = path.to_string_lossy().to_string();
    let target_ref = opaque_mutation_ref(
        "git.mutation.target",
        &[workspace_ref.as_bytes(), path_label.as_bytes()],
    );
    let path_truth_ref = opaque_mutation_ref(
        "path.truth.git.mutation",
        &[workspace_ref.as_bytes(), path_label.as_bytes()],
    );
    GitMutationTargetReview {
        target_ref,
        path_truth_ref,
        repo_relative_path: path.to_path_buf(),
        path_label,
        status_code,
        file_state_token,
        included_in_apply: false,
        blocked_reason: Some(reason),
        protected_review_required,
        preview_diff_ref: preview_diff_ref.to_string(),
        checkpoint_ref: checkpoint_ref.map(str::to_string),
    }
}

fn diff_preview_for(
    operation: GitMutationOperationKind,
    preview_diff_ref: &str,
    material: &GitRollbackMaterial,
) -> GitMutationDiffPreview {
    let bytes: &[u8] = match operation {
        GitMutationOperationKind::Stage => material.worktree_patch.as_ref(),
        GitMutationOperationKind::Unstage => material.before_index_patch.as_ref(),
        GitMutationOperationKind::Discard => material.worktree_patch.as_ref(),
        GitMutationOperationKind::RevertCheckpoint => {
            if material.before_index_patch.is_empty() {
                material.worktree_patch.as_ref()
            } else {
                material.before_index_patch.as_ref()
            }
        }
    };
    let diff_line_count = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .count();
    GitMutationDiffPreview {
        preview_diff_ref: preview_diff_ref.to_string(),
        preview_class: match operation {
            GitMutationOperationKind::Stage => "worktree_to_index".to_string(),
            GitMutationOperationKind::Unstage => "index_to_worktree".to_string(),
            GitMutationOperationKind::Discard => "discard_worktree_delta".to_string(),
            GitMutationOperationKind::RevertCheckpoint => "checkpoint_restore_delta".to_string(),
        },
        source_side_label: match operation {
            GitMutationOperationKind::Stage => "worktree".to_string(),
            GitMutationOperationKind::Unstage => "index".to_string(),
            GitMutationOperationKind::Discard => "worktree".to_string(),
            GitMutationOperationKind::RevertCheckpoint => "checkpoint".to_string(),
        },
        target_side_label: match operation {
            GitMutationOperationKind::Stage => "index".to_string(),
            GitMutationOperationKind::Unstage => "worktree".to_string(),
            GitMutationOperationKind::Discard => "index".to_string(),
            GitMutationOperationKind::RevertCheckpoint => "current Git state".to_string(),
        },
        diff_available: !bytes.is_empty(),
        diff_line_count,
        binary_diff_present: bytes.windows(10).any(|window| window == b"GIT binary"),
        display_label: format!("{} preview", operation.label()),
    }
}

fn checkpoint_for(
    checkpoint_ref: &str,
    operation: GitMutationOperationKind,
    preview_state: GitMutationPreviewState,
    scope: &GitMutationScopeReview,
    material: &GitRollbackMaterial,
) -> GitMutationCheckpointRecord {
    let captured = preview_state == GitMutationPreviewState::ReadyToApply
        && match operation {
            GitMutationOperationKind::Stage | GitMutationOperationKind::Unstage => true,
            GitMutationOperationKind::Discard => !material.worktree_patch.is_empty(),
            GitMutationOperationKind::RevertCheckpoint => true,
        };
    GitMutationCheckpointRecord {
        checkpoint_ref: checkpoint_ref.to_string(),
        checkpoint_kind: match operation {
            GitMutationOperationKind::Stage | GitMutationOperationKind::Unstage => {
                "index_state_patch"
            }
            GitMutationOperationKind::Discard => "worktree_patch",
            GitMutationOperationKind::RevertCheckpoint => "prior_mutation_checkpoint",
        }
        .to_string(),
        checkpoint_required: true,
        checkpoint_captured: captured,
        rollback_path_class: if captured {
            "restore_from_checkpoint"
        } else {
            "no_recovery_available"
        }
        .to_string(),
        restore_command_id: GitMutationOperationKind::RevertCheckpoint
            .command_id()
            .to_string(),
        retention_class: if captured {
            "local_recovery_audit"
        } else {
            "none"
        }
        .to_string(),
        covered_path_labels: scope
            .targets
            .iter()
            .filter(|target| target.included_in_apply)
            .map(|target| target.path_label.clone())
            .collect(),
    }
}

fn activity_for_preview(
    preview_ref: &str,
    operation: GitMutationOperationKind,
    preview_state: GitMutationPreviewState,
    checkpoint: &GitMutationCheckpointRecord,
    support_export_ref: &str,
) -> GitMutationActivityRecord {
    let state_class = match preview_state {
        GitMutationPreviewState::ReadyToApply => "waiting_review",
        GitMutationPreviewState::Blocked => "blocked",
        GitMutationPreviewState::Degraded => "degraded",
    };
    GitMutationActivityRecord {
        record_kind: GIT_MUTATION_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_MUTATION_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(preview_ref)),
        job_family: "git_mutation".to_string(),
        state_class: state_class.to_string(),
        partition: if preview_state == GitMutationPreviewState::ReadyToApply {
            "active_review"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: format!("{} preview", operation.label()),
        detail_label: format!(
            "{}; checkpoint {}",
            preview_state.as_str(),
            if checkpoint.checkpoint_captured {
                "captured"
            } else {
                "unavailable"
            }
        ),
        preview_ref: preview_ref.to_string(),
        mutation_id: None,
        checkpoint_refs: vec![checkpoint.checkpoint_ref.clone()],
        open_details_command_id: "cmd:git.mutation.open_details".to_string(),
        support_export_ref: support_export_ref.to_string(),
    }
}

fn activity_for_result(
    result_ref: &str,
    preview: &GitMutationPreview,
    outcome_state: GitMutationOutcomeState,
    mutation_id: &str,
    support_export_ref: &str,
) -> GitMutationActivityRecord {
    GitMutationActivityRecord {
        record_kind: GIT_MUTATION_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_MUTATION_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(result_ref)),
        job_family: "git_mutation".to_string(),
        state_class: outcome_state.activity_state_class().to_string(),
        partition: if matches!(
            outcome_state,
            GitMutationOutcomeState::Applied | GitMutationOutcomeState::Reverted
        ) {
            "completed"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: format!("{} {}", preview.operation.label(), outcome_state.as_str()),
        detail_label: format!(
            "{} path(s); checkpoint {}",
            preview.scope.included_count, preview.checkpoint.rollback_path_class
        ),
        preview_ref: preview.preview_ref.clone(),
        mutation_id: Some(mutation_id.to_string()),
        checkpoint_refs: vec![preview.checkpoint.checkpoint_ref.clone()],
        open_details_command_id: "cmd:git.mutation.open_details".to_string(),
        support_export_ref: support_export_ref.to_string(),
    }
}

fn support_export_for_preview(
    preview_ref: &str,
    operation: GitMutationOperationKind,
    workspace_ref: &str,
    scope_ref: &str,
    checkpoint: &GitMutationCheckpointRecord,
) -> GitMutationSupportExportRecord {
    GitMutationSupportExportRecord {
        record_kind: GIT_MUTATION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_MUTATION_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: opaque_mutation_ref(
            "git.mutation.support_export",
            &[preview_ref.as_bytes()],
        ),
        redaction_mode: "metadata_safe_default".to_string(),
        redaction_profile_ref: "support.redaction.local_first_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        operation_kind: operation.as_str().to_string(),
        phase: "preview".to_string(),
        workspace_ref_digest: mutation_support_digest("workspace_ref", workspace_ref),
        scope_ref_digest: mutation_support_digest("scope_ref", scope_ref),
        preview_ref_digest: mutation_support_digest("preview_ref", preview_ref),
        result_ref_digest: None,
        mutation_journal_ref_digest: None,
        checkpoint_ref_digests: vec![mutation_support_digest(
            "checkpoint_ref",
            &checkpoint.checkpoint_ref,
        )],
        evidence_ref_digests: vec![
            mutation_support_digest("evidence_ref", scope_ref),
            mutation_support_digest("evidence_ref", &checkpoint.checkpoint_ref),
        ],
        raw_path_export_allowed: false,
        raw_actor_export_allowed: false,
        omitted_fields: mutation_support_omitted_fields(),
    }
}

fn support_export_for_result(
    result_ref: &str,
    preview: &GitMutationPreview,
    mutation_id: &str,
) -> GitMutationSupportExportRecord {
    GitMutationSupportExportRecord {
        record_kind: GIT_MUTATION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_MUTATION_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: opaque_mutation_ref(
            "git.mutation.support_export",
            &[result_ref.as_bytes()],
        ),
        redaction_mode: "metadata_safe_default".to_string(),
        redaction_profile_ref: "support.redaction.local_first_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        operation_kind: preview.operation.as_str().to_string(),
        phase: if preview.operation == GitMutationOperationKind::RevertCheckpoint {
            "revert"
        } else {
            "apply"
        }
        .to_string(),
        workspace_ref_digest: mutation_support_digest("workspace_ref", &preview.workspace_ref),
        scope_ref_digest: mutation_support_digest("scope_ref", &preview.scope.scope_ref),
        preview_ref_digest: mutation_support_digest("preview_ref", &preview.preview_ref),
        result_ref_digest: Some(mutation_support_digest("result_ref", result_ref)),
        mutation_journal_ref_digest: Some(mutation_support_digest(
            "mutation_journal_ref",
            mutation_id,
        )),
        checkpoint_ref_digests: vec![mutation_support_digest(
            "checkpoint_ref",
            &preview.checkpoint.checkpoint_ref,
        )],
        evidence_ref_digests: vec![
            mutation_support_digest("evidence_ref", &preview.scope.scope_ref),
            mutation_support_digest("evidence_ref", &preview.diff_preview.preview_diff_ref),
            mutation_support_digest("evidence_ref", &preview.checkpoint.checkpoint_ref),
            mutation_support_digest("evidence_ref", mutation_id),
        ],
        raw_path_export_allowed: false,
        raw_actor_export_allowed: false,
        omitted_fields: mutation_support_omitted_fields(),
    }
}

fn result_for_preview(
    preview: &GitMutationPreview,
    resolved_at: &str,
    outcome_state: GitMutationOutcomeState,
    failure_reason: Option<String>,
) -> GitMutationResult {
    let result_ref = format!(
        "{}.result.{}",
        preview.preview_ref,
        sanitize_id(resolved_at)
    );
    let mutation_id = format!("git.mutation.{}", sanitize_id(&result_ref));
    let target_refs = preview
        .scope
        .targets
        .iter()
        .filter(|target| target.included_in_apply)
        .map(|target| target.target_ref.clone())
        .collect::<Vec<_>>();
    let support_export = support_export_for_result(&result_ref, preview, &mutation_id);
    let activity = activity_for_result(
        &result_ref,
        preview,
        outcome_state,
        &mutation_id,
        &support_export.support_export_ref,
    );
    let mutation_journal = GitMutationJournalRecord {
        record_kind: GIT_MUTATION_JOURNAL_RECORD_KIND.to_string(),
        schema_version: GIT_MUTATION_JOURNAL_SCHEMA_VERSION,
        mutation_id: mutation_id.clone(),
        command_id: preview.command_id.clone(),
        actor: preview.actor.clone(),
        source_class: "source_control_review".to_string(),
        scope_ref: preview.scope.scope_ref.clone(),
        target_refs,
        started_at: preview.generated_at.clone(),
        resolved_at: resolved_at.to_string(),
        reversal_class: preview.checkpoint.rollback_path_class.clone(),
        checkpoint_refs: vec![preview.checkpoint.checkpoint_ref.clone()],
        redaction_class: "metadata_safe_default".to_string(),
        side_effect_summary: format!(
            "{} {} path(s)",
            preview.operation.label(),
            preview.scope.included_count
        ),
    };
    let applied_targets = preview
        .scope
        .targets
        .iter()
        .filter(|target| target.included_in_apply)
        .cloned()
        .collect::<Vec<_>>();
    let blocked_targets = preview
        .scope
        .targets
        .iter()
        .filter(|target| !target.included_in_apply)
        .cloned()
        .collect::<Vec<_>>();
    GitMutationResult {
        record_kind: GIT_MUTATION_RESULT_RECORD_KIND.to_string(),
        schema_version: GIT_MUTATION_RESULT_SCHEMA_VERSION,
        result_ref,
        preview_ref: preview.preview_ref.clone(),
        resolved_at: resolved_at.to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        repo_root: preview.repo_root.clone(),
        truth_source_ref: preview.truth_source_ref.clone(),
        operation: preview.operation,
        outcome_state,
        applied_targets,
        blocked_targets,
        checkpoint: preview.checkpoint.clone(),
        mutation_journal,
        activity,
        support_export,
        rollback_available: outcome_state == GitMutationOutcomeState::Applied
            && preview.checkpoint.satisfies_required_recovery(),
        revert_command_id: GitMutationOperationKind::RevertCheckpoint
            .command_id()
            .to_string(),
        failure_reason,
        rollback_material: preview.rollback_material.clone(),
        projection_digest: None,
    }
    .seal()
}

fn result_for_blocked_preview(
    preview: &GitMutationPreview,
    resolved_at: &str,
) -> GitMutationResult {
    result_for_preview(
        preview,
        resolved_at,
        GitMutationOutcomeState::BlockedNoChangesMade,
        Some("preview is not ready to apply".to_string()),
    )
}

fn bounded_projection_digest(value: &impl Serialize) -> Option<String> {
    serde_json::to_vec(value)
        .ok()
        .filter(|bytes| bytes.len() <= MAX_GIT_MUTATION_RECORD_BYTES)
        .map(|bytes| digest::sha256_token(&bytes))
}

fn valid_mutation_metadata(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_GIT_MUTATION_METADATA_BYTES
        && value.chars().all(|character| !character.is_control())
}

fn valid_mutation_actor(actor: &GitMutationActorRef) -> bool {
    valid_mutation_metadata(&actor.actor_class)
        && valid_mutation_metadata(&actor.display_label)
        && actor
            .stable_id
            .as_deref()
            .map_or(true, valid_mutation_metadata)
}

fn validate_mutation_request_metadata(request: &GitMutationRequest) -> Result<(), &'static str> {
    if !valid_mutation_metadata(&request.workspace_ref) {
        return Err("workspace identity is outside the review boundary");
    }
    if !valid_mutation_metadata(&request.requested_at) {
        return Err("preview timestamp is outside the review boundary");
    }
    if !valid_mutation_actor(&request.actor) {
        return Err("actor identity is outside the review boundary");
    }
    if request
        .launch_source_ref
        .as_deref()
        .is_some_and(|value| !valid_mutation_metadata(value))
    {
        return Err("launch source identity is outside the review boundary");
    }
    Ok(())
}

fn bounded_requested_path_shape(paths: &[PathBuf]) -> Result<(), &'static str> {
    if paths.len() > MAX_GIT_MUTATION_PATHS {
        return Err("selected Git path count exceeds the review limit");
    }
    let mut total_bytes = 0usize;
    let mut seen = HashSet::with_capacity(paths.len());
    for path in paths {
        if path.as_os_str().is_empty()
            || path.components().any(|component| {
                !matches!(
                    component,
                    Component::Prefix(_) | Component::RootDir | Component::Normal(_)
                )
            })
        {
            return Err("selected Git path is not normalized");
        }
        let Some(path_text) = path.to_str() else {
            return Err("selected Git path is not valid UTF-8 for this adapter");
        };
        if path_text.len() > MAX_GIT_MUTATION_PATH_BYTES {
            return Err("selected Git path exceeds the per-path review limit");
        }
        total_bytes = total_bytes
            .checked_add(path_text.len())
            .and_then(|value| value.checked_add(1))
            .ok_or("selected Git path scope exceeds the review limit")?;
        if total_bytes > MAX_GIT_MUTATION_SCOPE_BYTES {
            return Err("selected Git path scope exceeds the review limit");
        }
        if !seen.insert(path) {
            return Err("selected Git path scope contains a duplicate path");
        }
    }
    Ok(())
}

fn bounded_normalized_paths(
    paths: &[PathBuf],
    repo_root: &Path,
) -> Result<Vec<PathBuf>, &'static str> {
    if paths.len() > MAX_GIT_MUTATION_PATHS {
        return Err("selected Git path count exceeds the review limit");
    }
    let mut total_bytes = 0usize;
    let mut seen = HashSet::with_capacity(paths.len());
    let mut normalized = Vec::with_capacity(paths.len());
    for path in paths {
        let relative = if path.is_absolute() {
            path.strip_prefix(repo_root)
                .map_err(|_| "selected Git path is outside the reviewed root")?
        } else {
            path.as_path()
        };
        if relative.as_os_str().is_empty()
            || relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err("selected Git path is not a normalized repository-relative path");
        }
        let Some(relative_text) = relative.to_str() else {
            return Err("selected Git path is not valid UTF-8 for this adapter");
        };
        if !normalized_repo_path_text(relative_text) {
            return Err("selected Git path is not a normalized repository-relative path");
        }
        if relative_text.len() > MAX_GIT_MUTATION_PATH_BYTES {
            return Err("selected Git path exceeds the per-path review limit");
        }
        total_bytes = total_bytes
            .checked_add(relative_text.len())
            .and_then(|value| value.checked_add(1))
            .ok_or("selected Git path scope exceeds the review limit")?;
        if total_bytes > MAX_GIT_MUTATION_SCOPE_BYTES {
            return Err("selected Git path scope exceeds the review limit");
        }
        let relative = relative.to_path_buf();
        if !seen.insert(relative.clone()) {
            return Err("selected Git path scope contains a duplicate path");
        }
        normalized.push(relative);
    }
    Ok(normalized)
}

fn bounded_normalized_request_paths(
    paths: &[PathBuf],
    requested_root: &Path,
    repo_root: &Path,
) -> Result<Vec<PathBuf>, &'static str> {
    let has_unmatched_absolute_path = paths
        .iter()
        .any(|path| path.is_absolute() && !path.starts_with(repo_root));
    if !has_unmatched_absolute_path {
        return bounded_normalized_paths(paths, repo_root);
    }

    let canonical_requested_root = std::fs::canonicalize(requested_root)
        .map_err(|_| "requested Git root could not be resolved safely")?;
    let requested_root_offset = canonical_requested_root
        .strip_prefix(repo_root)
        .map_err(|_| "requested Git root is outside the reviewed repository")?;
    let mut mapped = Vec::with_capacity(paths.len());
    for path in paths {
        if !path.is_absolute() || path.starts_with(repo_root) {
            mapped.push(path.clone());
            continue;
        }
        if let Ok(suffix) = path.strip_prefix(requested_root) {
            mapped.push(repo_root.join(requested_root_offset).join(suffix));
            continue;
        }
        let canonical_path = std::fs::canonicalize(path)
            .map_err(|_| "selected Git path is outside the requested root")?;
        if !canonical_path.starts_with(&canonical_requested_root) {
            return Err("selected Git path is outside the requested root");
        }
        mapped.push(canonical_path);
    }
    bounded_normalized_paths(&mapped, repo_root)
}

fn normalized_repo_path_text(value: &str) -> bool {
    let is_normal = |segment: &str| !segment.is_empty() && !matches!(segment, "." | "..");
    #[cfg(windows)]
    {
        value.split(|ch| matches!(ch, '/' | '\\')).all(is_normal)
    }
    #[cfg(not(windows))]
    {
        value.split('/').all(is_normal)
    }
}

fn combined_evidence_len(first: &[u8], second: &[u8]) -> usize {
    first.len().saturating_add(second.len())
}

fn append_evidence_bounded(existing: &[u8], target: &mut Vec<u8>, additional: &[u8]) -> bool {
    let Some(total) = existing
        .len()
        .checked_add(target.len())
        .and_then(|value| value.checked_add(additional.len()))
    else {
        return false;
    };
    if total > MAX_GIT_MUTATION_EVIDENCE_BYTES {
        return false;
    }
    target.extend_from_slice(additional);
    true
}

fn file_state_token(change: &GitChange) -> String {
    if change.is_conflicted || change.change_kind == ChangeKind::Conflict {
        "conflicted".to_string()
    } else {
        change.change_kind.as_str().to_string()
    }
}

fn preview_ref(
    workspace_ref: &str,
    operation: GitMutationOperationKind,
    paths: &[PathBuf],
) -> String {
    let path_text = paths
        .iter()
        .map(|path| path.to_string_lossy())
        .collect::<Vec<_>>();
    let mut parts = Vec::with_capacity(path_text.len() + 2);
    parts.push(workspace_ref.as_bytes());
    parts.push(operation.as_str().as_bytes());
    for path in &path_text {
        parts.push(path.as_bytes());
    }
    opaque_mutation_ref("git.mutation.preview", &parts)
}

fn opaque_mutation_ref(prefix: &str, parts: &[&[u8]]) -> String {
    let digest = digest::sha256_framed_token(parts);
    format!(
        "{prefix}.{}",
        digest
            .strip_prefix("sha256:")
            .expect("SHA-256 helper always returns a prefixed token")
    )
}

fn mutation_support_digest(field: &str, value: &str) -> String {
    digest::sha256_framed_token(&[
        b"support.redaction.local_first_default",
        b"git_mutation_support_export_record.v2",
        field.as_bytes(),
        value.as_bytes(),
    ])
}

fn mutation_support_omitted_fields() -> Vec<String> {
    [
        "workspace_ref",
        "repo_root",
        "target_paths",
        "scope_ref",
        "preview_ref",
        "result_ref",
        "mutation_journal_ref",
        "checkpoint_refs",
        "evidence_refs",
        "raw_patch_body",
        "raw_command_line",
        "raw_actor_value",
        "backend_output",
        "failure_detail",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn diff_cached_args(paths: &[PathBuf]) -> Vec<String> {
    let mut args = vec![
        "diff".to_string(),
        "--cached".to_string(),
        "--binary".to_string(),
        "--no-ext-diff".to_string(),
        "--no-textconv".to_string(),
    ];
    args.push("--".to_string());
    args.extend(paths.iter().map(|path| path.to_string_lossy().to_string()));
    args
}

fn diff_worktree_args(paths: &[PathBuf]) -> Vec<String> {
    let mut args = vec![
        "diff".to_string(),
        "--binary".to_string(),
        "--no-ext-diff".to_string(),
        "--no-textconv".to_string(),
    ];
    args.push("--".to_string());
    args.extend(paths.iter().map(|path| path.to_string_lossy().to_string()));
    args
}

fn diff_untracked_args(path: &Path) -> Vec<String> {
    vec![
        "diff".to_string(),
        "--no-index".to_string(),
        "--binary".to_string(),
        "--no-ext-diff".to_string(),
        "--no-textconv".to_string(),
        "--".to_string(),
        hardened_git::null_device_path().to_string(),
        path.to_string_lossy().to_string(),
    ]
}

fn git_apply_args(flags: &[&str]) -> Vec<String> {
    let mut args = vec!["apply".to_string()];
    args.extend(flags.iter().map(|flag| (*flag).to_string()));
    args.extend([
        "--binary".to_string(),
        "--whitespace=nowarn".to_string(),
        "--recount".to_string(),
        "-".to_string(),
    ]);
    args
}

fn git_mutation_failure_reason(output: &GitMutationCommandOutput) -> String {
    output
        .status_code
        .map(|code| format!("Git mutation failed (status {code})"))
        .unwrap_or_else(|| "Git mutation failed".to_string())
}

fn sanitize_id(value: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_sep = false;
            continue;
        }
        if last_sep {
            continue;
        }
        out.push('-');
        last_sep = true;
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "root".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutation_failure_reason_never_exports_git_stderr() {
        let output = GitMutationCommandOutput {
            success: false,
            status_code: Some(128),
            stdout: Vec::new(),
            stderr: b"\x1b[31m/private/workspace/secret-value\x1b[0m".to_vec(),
        };

        let reason = git_mutation_failure_reason(&output);

        assert_eq!(reason, "Git mutation failed (status 128)");
        assert!(!reason.contains("secret-value"));
        assert!(!reason.contains('\u{1b}'));
    }

    #[test]
    fn mutation_scope_and_evidence_aggregation_are_bounded() {
        let too_many = (0..=MAX_GIT_MUTATION_PATHS)
            .map(|index| PathBuf::from(format!("src/{index}.rs")))
            .collect::<Vec<_>>();
        assert_eq!(
            bounded_normalized_paths(&too_many, Path::new("/repo")),
            Err("selected Git path count exceeds the review limit")
        );

        let oversized = vec![PathBuf::from("x".repeat(MAX_GIT_MUTATION_PATH_BYTES + 1))];
        assert_eq!(
            bounded_normalized_paths(&oversized, Path::new("/repo")),
            Err("selected Git path exceeds the per-path review limit")
        );
        assert!(
            bounded_normalized_paths(&[PathBuf::from("../outside.txt")], Path::new("/repo"))
                .is_err()
        );
        assert!(
            bounded_normalized_paths(&[PathBuf::from("src/./same.rs")], Path::new("/repo"))
                .is_err()
        );
        assert!(
            bounded_normalized_paths(&[PathBuf::from("src//same.rs")], Path::new("/repo")).is_err()
        );

        let mut retained = vec![0_u8; MAX_GIT_MUTATION_EVIDENCE_BYTES - 1];
        assert!(!append_evidence_bounded(
            &[0_u8; 1],
            &mut retained,
            &[0_u8; 1]
        ));
    }

    #[test]
    fn oversized_request_blocks_without_materializing_target_rows() {
        let request = GitMutationRequest::with_observed_at(
            "workspace.private",
            "/repo",
            GitMutationOperationKind::Stage,
            (0..=MAX_GIT_MUTATION_PATHS).map(|index| format!("src/{index}.rs")),
            "2026-07-22T00:00:00Z",
        );
        let preview = GitMutationService::default().preview(&request);

        assert_eq!(preview.preview_state, GitMutationPreviewState::Degraded);
        assert!(preview.scope.targets.is_empty());
        assert_eq!(preview.scope.requested_count, MAX_GIT_MUTATION_PATHS + 1);
    }

    #[test]
    fn invalid_request_metadata_is_bounded_and_not_reflected() {
        let private_value = format!("private-{}", "x".repeat(MAX_GIT_MUTATION_METADATA_BYTES));
        let mut request = GitMutationRequest::with_observed_at(
            private_value.clone(),
            "/repo",
            GitMutationOperationKind::Stage,
            ["src/lib.rs"],
            "2026-07-22T00:00:00Z",
        )
        .with_launch_source_ref(private_value.clone());
        request.actor = GitMutationActorRef {
            actor_class: private_value.clone(),
            display_label: private_value.clone(),
            stable_id: Some(private_value.clone()),
        };

        let preview = GitMutationService::default().preview(&request);

        assert_eq!(preview.preview_state, GitMutationPreviewState::Degraded);
        assert_eq!(preview.workspace_ref, "workspace.unavailable");
        assert_eq!(preview.actor, GitMutationActorRef::default());
        assert_eq!(preview.launch_source_ref, None);
        let json = serde_json::to_string(&preview).expect("serialize degraded preview");
        assert!(!json.contains(&private_value));
    }
}
