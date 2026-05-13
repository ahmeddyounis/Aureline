//! Preview-first Git mutation flows for source-control rows.
//!
//! This module owns the first bounded contract for path-level Git mutations.
//! Callers create a preview for a stage, unstage, discard, or checkpoint
//! restore request, inspect the exact target scope and checkpoint posture, and
//! then apply the preview through the same service so activity and support
//! records can quote one lineage.

use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

use crate::status::{
    ChangeKind, ConsumerProjectionBundle, GitChange, GitServiceState, GitStatusRequest,
    GitStatusService,
};

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
const GIT_MUTATION_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;
const GIT_MUTATION_JOURNAL_SCHEMA_VERSION: u32 = 1;

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
    #[serde(skip)]
    rollback_material: GitRollbackMaterial,
}

impl GitMutationPreview {
    /// Returns true when apply may proceed without recomputing scope.
    pub fn ready_to_apply(&self) -> bool {
        self.preview_state == GitMutationPreviewState::ReadyToApply
            && self.scope.blocked_count == 0
            && self.scope.included_count > 0
            && self.scope.all_rows_have_visible_scope_and_preview()
            && self.checkpoint.satisfies_required_recovery()
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
    /// Retention class for this export row.
    pub retention_class: String,
    /// Operation token.
    pub operation_kind: String,
    /// Phase token for preview, apply, or restore.
    pub phase: String,
    /// Workspace identity copied from the preview.
    pub workspace_ref: String,
    /// Scope ref copied from the preview.
    pub scope_ref: String,
    /// Preview ref copied from the preview.
    pub preview_ref: String,
    /// Result ref when a mutation completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Mutation journal ref when a mutation completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Checkpoint refs available to support and recovery surfaces.
    pub checkpoint_refs: Vec<String>,
    /// Evidence refs included without raw patch bodies.
    pub evidence_refs: Vec<String>,
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
}

impl GitMutationResult {
    /// Returns true when activity and support rows cite the journal record.
    pub fn attribution_is_exportable(&self) -> bool {
        self.activity.mutation_id.as_deref() == Some(self.mutation_journal.mutation_id.as_str())
            && self.support_export.mutation_journal_ref.as_deref()
                == Some(self.mutation_journal.mutation_id.as_str())
            && self
                .support_export
                .checkpoint_refs
                .contains(&self.checkpoint.checkpoint_ref)
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
        let output = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|err| GitMutationBackendError {
                message: format!("git command failed to launch: {err}"),
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
        let mut child = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| GitMutationBackendError {
                message: format!("git command failed to launch: {err}"),
            })?;
        if let Some(mut child_stdin) = child.stdin.take() {
            child_stdin
                .write_all(stdin)
                .map_err(|err| GitMutationBackendError {
                    message: format!("git command stdin failed: {err}"),
                })?;
        }
        let output = child
            .wait_with_output()
            .map_err(|err| GitMutationBackendError {
                message: format!("git command failed to finish: {err}"),
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

        let repo_paths = request
            .paths
            .iter()
            .map(|path| normalize_requested_path(path, &repo_root))
            .collect::<Vec<_>>();
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
        let patch = self.preview_patch(&repo_root, request.operation, &repo_paths);
        let preview_state = if request.paths.is_empty() || blocked_count > 0 || included_count == 0
        {
            GitMutationPreviewState::Blocked
        } else {
            GitMutationPreviewState::ReadyToApply
        };
        let rollback_material = rollback_material_for(request.operation, repo_paths.clone(), patch);
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
            rollback_material,
        }
    }

    /// Applies an admitted preview and returns an attributable result packet.
    pub fn apply(
        &self,
        preview: &GitMutationPreview,
        resolved_at: impl Into<String>,
    ) -> GitMutationResult {
        let resolved_at = resolved_at.into();
        if !preview.ready_to_apply() {
            return result_for_blocked_preview(preview, &resolved_at);
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
                Some(stderr_or_status(&output)),
            ),
            Err(err) => (GitMutationOutcomeState::Failed, Some(err.message)),
        };
        result_for_preview(preview, &resolved_at, outcome_state, failure_reason)
    }

    /// Builds the review packet for restoring a prior mutation checkpoint.
    pub fn preview_revert(
        &self,
        result: &GitMutationResult,
        requested_at: impl Into<String>,
    ) -> GitMutationPreview {
        let requested_at = requested_at.into();
        let repo_paths = result
            .applied_targets
            .iter()
            .map(|target| target.repo_relative_path.clone())
            .collect::<Vec<_>>();
        let preview_ref = preview_ref(
            &result.workspace_ref,
            GitMutationOperationKind::RevertCheckpoint,
            &repo_paths,
        );
        let preview_diff_ref = format!("{}.diff", preview_ref);
        let mut targets = result.applied_targets.clone();
        for target in &mut targets {
            target.preview_diff_ref = preview_diff_ref.clone();
            target.checkpoint_ref = Some(result.checkpoint.checkpoint_ref.clone());
            target.included_in_apply = true;
            target.blocked_reason = None;
            target.protected_review_required = true;
        }
        let scope = GitMutationScopeReview {
            scope_ref: format!("{}.scope", preview_ref),
            requested_count: targets.len(),
            included_count: targets.len(),
            blocked_count: 0,
            basis_snapshot_ref: result.result_ref.clone(),
            scope_rebind_forbidden: true,
            targets,
        };
        let checkpoint = GitMutationCheckpointRecord {
            checkpoint_ref: result.checkpoint.checkpoint_ref.clone(),
            checkpoint_kind: result.checkpoint.checkpoint_kind.clone(),
            checkpoint_required: true,
            checkpoint_captured: result.rollback_available,
            rollback_path_class: "restore_from_checkpoint".to_string(),
            restore_command_id: GitMutationOperationKind::RevertCheckpoint
                .command_id()
                .to_string(),
            retention_class: result.checkpoint.retention_class.clone(),
            covered_path_labels: result.checkpoint.covered_path_labels.clone(),
        };
        let rollback_material = result.rollback_material.clone();
        let diff_preview = diff_preview_for(
            GitMutationOperationKind::RevertCheckpoint,
            &preview_diff_ref,
            &rollback_material,
        );
        let preview_state = if result.rollback_available {
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
            truth_source_ref: result.truth_source_ref.clone(),
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
            rollback_material,
        }
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
        let preview_ref = preview_ref(&request.workspace_ref, request.operation, &request.paths);
        let preview_diff_ref = format!("{}.diff", preview_ref);
        let checkpoint_ref = format!("{}.checkpoint", preview_ref);
        let targets = request
            .paths
            .iter()
            .map(|path| {
                let repo_path = normalize_requested_path(path, &repo_root);
                blocked_target(
                    &request.workspace_ref,
                    &repo_path,
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
            &request.workspace_ref,
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
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            operation: request.operation,
            command_id: request.operation.command_id().to_string(),
            operation_label: request.operation.label().to_string(),
            preview_state: GitMutationPreviewState::Degraded,
            consequence_class: request.operation.consequence_class().to_string(),
            destructive_review_required: request.operation.is_destructive(),
            actor: request.actor.clone(),
            launch_source_ref: request.launch_source_ref.clone(),
            scope,
            diff_preview,
            checkpoint,
            activity,
            support_export,
            rollback_material,
        }
    }

    fn preview_patch(
        &self,
        repo_root: &Path,
        operation: GitMutationOperationKind,
        paths: &[PathBuf],
    ) -> PreviewPatch {
        if operation == GitMutationOperationKind::RevertCheckpoint || paths.is_empty() {
            return PreviewPatch::default();
        }
        let before_index_patch = if matches!(
            operation,
            GitMutationOperationKind::Stage | GitMutationOperationKind::Unstage
        ) {
            self.run_git_bytes(repo_root, diff_cached_args(paths))
        } else {
            Vec::new()
        };
        let worktree_patch = if matches!(
            operation,
            GitMutationOperationKind::Stage | GitMutationOperationKind::Discard
        ) {
            self.run_git_bytes(repo_root, diff_worktree_args(paths))
        } else {
            Vec::new()
        };
        PreviewPatch {
            before_index_patch,
            worktree_patch,
        }
    }

    fn run_git_bytes(&self, repo_root: &Path, args: Vec<String>) -> Vec<u8> {
        match self.backend.run_git(repo_root, &args) {
            Ok(output) if output.success => output.stdout,
            _ => Vec::new(),
        }
    }

    fn apply_preview(
        &self,
        preview: &GitMutationPreview,
    ) -> Result<GitMutationCommandOutput, GitMutationBackendError> {
        match preview.operation {
            GitMutationOperationKind::Stage => self.backend.run_git(
                &preview.repo_root,
                &path_command_args("add", &preview.scope),
            ),
            GitMutationOperationKind::Unstage => self.backend.run_git(
                &preview.repo_root,
                &path_command_args_with_flags("restore", &["--staged"], &preview.scope),
            ),
            GitMutationOperationKind::Discard => self.backend.run_git(
                &preview.repo_root,
                &path_command_args_with_flags("restore", &["--worktree"], &preview.scope),
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
        match material.action {
            GitRollbackAction::None => Ok(GitMutationCommandOutput {
                success: true,
                status_code: Some(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            }),
            GitRollbackAction::RestoreIndexFromPatch => {
                let reset = self.backend.run_git(
                    repo_root,
                    &path_args("restore", &["--staged"], &material.paths),
                )?;
                if !reset.success || material.before_index_patch.is_empty() {
                    return Ok(reset);
                }
                self.backend.run_git_with_stdin(
                    repo_root,
                    &["apply", "--cached", "--whitespace=nowarn", "-"]
                        .into_iter()
                        .map(str::to_string)
                        .collect::<Vec<_>>(),
                    &material.before_index_patch,
                )
            }
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
                    &material.worktree_patch,
                )
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct PreviewPatch {
    before_index_patch: Vec<u8>,
    worktree_patch: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GitRollbackAction {
    #[default]
    None,
    RestoreIndexFromPatch,
    ApplyWorktreePatch,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct GitRollbackMaterial {
    action: GitRollbackAction,
    before_index_patch: Vec<u8>,
    worktree_patch: Vec<u8>,
    paths: Vec<PathBuf>,
}

fn rollback_material_for(
    operation: GitMutationOperationKind,
    paths: Vec<PathBuf>,
    patch: PreviewPatch,
) -> GitRollbackMaterial {
    let action = match operation {
        GitMutationOperationKind::Stage | GitMutationOperationKind::Unstage => {
            GitRollbackAction::RestoreIndexFromPatch
        }
        GitMutationOperationKind::Discard => GitRollbackAction::ApplyWorktreePatch,
        GitMutationOperationKind::RevertCheckpoint => GitRollbackAction::None,
    };
    GitRollbackMaterial {
        action,
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
    let path_id = sanitize_id(&path_label);
    let workspace_id = sanitize_id(workspace_ref);
    GitMutationTargetReview {
        target_ref: format!("git.mutation.target.{workspace_id}.{path_id}"),
        path_truth_ref: format!("path.truth.git.mutation.{workspace_id}.{path_id}"),
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
    let path_id = sanitize_id(&path_label);
    let workspace_id = sanitize_id(workspace_ref);
    GitMutationTargetReview {
        target_ref: format!("git.mutation.target.{workspace_id}.{path_id}"),
        path_truth_ref: format!("path.truth.git.mutation.{workspace_id}.{path_id}"),
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
    let bytes = match operation {
        GitMutationOperationKind::Stage => &material.worktree_patch,
        GitMutationOperationKind::Unstage => &material.before_index_patch,
        GitMutationOperationKind::Discard => &material.worktree_patch,
        GitMutationOperationKind::RevertCheckpoint => {
            if material.before_index_patch.is_empty() {
                &material.worktree_patch
            } else {
                &material.before_index_patch
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
        support_export_ref: format!("support.export.{}", sanitize_id(preview_ref)),
        redaction_mode: "metadata_safe_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        operation_kind: operation.as_str().to_string(),
        phase: "preview".to_string(),
        workspace_ref: workspace_ref.to_string(),
        scope_ref: scope_ref.to_string(),
        preview_ref: preview_ref.to_string(),
        result_ref: None,
        mutation_journal_ref: None,
        checkpoint_refs: vec![checkpoint.checkpoint_ref.clone()],
        evidence_refs: vec![scope_ref.to_string(), checkpoint.checkpoint_ref.clone()],
        omitted_fields: vec![
            "raw_patch_body".to_string(),
            "raw_command_line".to_string(),
            "raw_actor_secret".to_string(),
        ],
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
        support_export_ref: format!("support.export.{}", sanitize_id(result_ref)),
        redaction_mode: "metadata_safe_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        operation_kind: preview.operation.as_str().to_string(),
        phase: if preview.operation == GitMutationOperationKind::RevertCheckpoint {
            "revert"
        } else {
            "apply"
        }
        .to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        scope_ref: preview.scope.scope_ref.clone(),
        preview_ref: preview.preview_ref.clone(),
        result_ref: Some(result_ref.to_string()),
        mutation_journal_ref: Some(mutation_id.to_string()),
        checkpoint_refs: vec![preview.checkpoint.checkpoint_ref.clone()],
        evidence_refs: vec![
            preview.scope.scope_ref.clone(),
            preview.diff_preview.preview_diff_ref.clone(),
            preview.checkpoint.checkpoint_ref.clone(),
            mutation_id.to_string(),
        ],
        omitted_fields: vec![
            "raw_patch_body".to_string(),
            "raw_command_line".to_string(),
            "raw_actor_secret".to_string(),
        ],
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
        rollback_available: matches!(
            outcome_state,
            GitMutationOutcomeState::Applied | GitMutationOutcomeState::Reverted
        ) && preview.checkpoint.satisfies_required_recovery(),
        revert_command_id: GitMutationOperationKind::RevertCheckpoint
            .command_id()
            .to_string(),
        failure_reason,
        rollback_material: preview.rollback_material.clone(),
    }
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

fn normalize_requested_path(path: &Path, repo_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.strip_prefix(repo_root)
            .map(Path::to_path_buf)
            .unwrap_or_else(|_| path.to_path_buf())
    } else {
        path.to_path_buf()
    }
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
    let mut path_part = paths
        .iter()
        .map(|path| sanitize_id(&path.to_string_lossy()))
        .collect::<Vec<_>>()
        .join(".");
    if path_part.is_empty() {
        path_part = "empty".to_string();
    }
    format!(
        "git.mutation.preview.{}.{}.{}",
        sanitize_id(workspace_ref),
        operation.as_str(),
        path_part
    )
}

fn path_command_args(command: &str, scope: &GitMutationScopeReview) -> Vec<String> {
    path_command_args_with_flags(command, &[], scope)
}

fn path_command_args_with_flags(
    command: &str,
    flags: &[&str],
    scope: &GitMutationScopeReview,
) -> Vec<String> {
    let paths = scope
        .targets
        .iter()
        .filter(|target| target.included_in_apply)
        .map(|target| target.repo_relative_path.clone())
        .collect::<Vec<_>>();
    path_args(command, flags, &paths)
}

fn path_args(command: &str, flags: &[&str], paths: &[PathBuf]) -> Vec<String> {
    let mut args = Vec::with_capacity(flags.len() + paths.len() + 2);
    args.push(command.to_string());
    args.extend(flags.iter().map(|flag| (*flag).to_string()));
    args.push("--".to_string());
    args.extend(paths.iter().map(|path| path.to_string_lossy().to_string()));
    args
}

fn diff_cached_args(paths: &[PathBuf]) -> Vec<String> {
    let mut args = vec![
        "diff".to_string(),
        "--cached".to_string(),
        "--binary".to_string(),
    ];
    args.push("--".to_string());
    args.extend(paths.iter().map(|path| path.to_string_lossy().to_string()));
    args
}

fn diff_worktree_args(paths: &[PathBuf]) -> Vec<String> {
    let mut args = vec!["diff".to_string(), "--binary".to_string()];
    args.push("--".to_string());
    args.extend(paths.iter().map(|path| path.to_string_lossy().to_string()));
    args
}

fn stderr_or_status(output: &GitMutationCommandOutput) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if stderr.is_empty() {
        format!("git exited with status {:?}", output.status_code)
    } else {
        stderr
    }
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
