//! Preview-first branch switch, create, and checkout flows.
//!
//! This module keeps branch-changing commands on the same local Git truth path
//! as status, change-list, mutation, and commit surfaces. A branch operation is
//! previewed against the canonical status snapshot, discloses current-work and
//! detached-head posture, then applies only if the reviewed basis still matches.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::status::{
    BranchState, ChangeSummary, ConsumerProjectionBundle, GitServiceState, GitShellStatusRecord,
    GitStatusRequest, GitStatusService, HeadIdentity,
};

/// Stable record-kind tag for [`GitBranchPreview`].
pub const GIT_BRANCH_PREVIEW_RECORD_KIND: &str = "git_branch_preview";

/// Stable record-kind tag for [`GitBranchResult`].
pub const GIT_BRANCH_RESULT_RECORD_KIND: &str = "git_branch_result";

/// Stable record-kind tag for [`GitBranchActivityRecord`].
pub const GIT_BRANCH_ACTIVITY_RECORD_KIND: &str = "git_branch_activity_record";

/// Stable record-kind tag for [`GitBranchSupportExportRecord`].
pub const GIT_BRANCH_SUPPORT_EXPORT_RECORD_KIND: &str = "git_branch_support_export_record";

/// Stable record-kind tag for [`GitBranchJournalRecord`].
pub const GIT_BRANCH_JOURNAL_RECORD_KIND: &str = "git_branch_journal_record";

const GIT_BRANCH_PREVIEW_SCHEMA_VERSION: u32 = 1;
const GIT_BRANCH_RESULT_SCHEMA_VERSION: u32 = 1;
const GIT_BRANCH_ACTIVITY_SCHEMA_VERSION: u32 = 1;
const GIT_BRANCH_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;
const GIT_BRANCH_JOURNAL_SCHEMA_VERSION: u32 = 1;

/// Branch operation requested by the local branch-management lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitBranchOperationKind {
    /// Switch to an existing local branch.
    Switch,
    /// Create a new branch and switch to it.
    Create,
    /// Checkout a branch or revision, disclosing detached-head posture.
    Checkout,
}

impl GitBranchOperationKind {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Switch => "switch",
            Self::Create => "create",
            Self::Checkout => "checkout",
        }
    }

    /// Canonical command id for this branch operation.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Switch => "cmd:git.branch.switch",
            Self::Create => "cmd:git.branch.create",
            Self::Checkout => "cmd:git.branch.checkout",
        }
    }

    /// Reviewer-facing label for this operation.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Switch => "Switch branch",
            Self::Create => "Create branch",
            Self::Checkout => "Checkout revision",
        }
    }

    /// Consequence class shown in preview and support packets.
    pub const fn consequence_class(self) -> &'static str {
        match self {
            Self::Switch => "branch_switch",
            Self::Create => "branch_create_and_switch",
            Self::Checkout => "checkout_ref",
        }
    }
}

/// State of a branch-operation preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitBranchPreviewState {
    /// Scope, current work, and target identity are ready for explicit apply.
    ReadyToApply,
    /// The preview exists, but target or current-work honesty blocks apply.
    Blocked,
    /// Git state was unavailable or stale, so no apply may proceed.
    Degraded,
}

impl GitBranchPreviewState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToApply => "ready_to_apply",
            Self::Blocked => "blocked",
            Self::Degraded => "degraded",
        }
    }
}

/// Final state of an attempted branch operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitBranchOutcomeState {
    /// Git applied the requested branch operation.
    Applied,
    /// No Git mutation was attempted because the preview was not admissible.
    BlockedNoChangesMade,
    /// Git returned a failure while attempting the operation.
    Failed,
}

impl GitBranchOutcomeState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::BlockedNoChangesMade => "blocked_no_changes_made",
            Self::Failed => "failed",
        }
    }

    fn activity_state_class(self) -> &'static str {
        match self {
            Self::Applied => "completed",
            Self::BlockedNoChangesMade => "blocked",
            Self::Failed => "failed",
        }
    }
}

/// Branch-target class disclosed before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitBranchTargetKind {
    /// Target is an existing local branch.
    LocalBranch,
    /// Target is the new local branch that will be created.
    NewBranch,
    /// Target resolves to a commit but will leave `HEAD` detached.
    DetachedHead,
    /// Target is a remote-tracking branch rather than a local branch.
    RemoteTrackingBranch,
    /// Target could not be classified.
    Unknown,
}

impl GitBranchTargetKind {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalBranch => "local_branch",
            Self::NewBranch => "new_branch",
            Self::DetachedHead => "detached_head",
            Self::RemoteTrackingBranch => "remote_tracking_branch",
            Self::Unknown => "unknown",
        }
    }
}

/// Remote or upstream state disclosed by branch previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitBranchRemoteState {
    /// Remote state does not apply to this target.
    NotApplicable,
    /// A target branch has an upstream configured.
    UpstreamConfigured,
    /// A target branch has no upstream configured.
    UpstreamMissing,
    /// The requested remote exists and the target resolved.
    TargetRemoteAvailable,
    /// The requested remote name is not configured locally.
    TargetRemoteMissing,
    /// The remote exists but the requested remote branch is absent.
    TargetRemoteBranchMissing,
}

impl GitBranchRemoteState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::UpstreamConfigured => "upstream_configured",
            Self::UpstreamMissing => "upstream_missing",
            Self::TargetRemoteAvailable => "target_remote_available",
            Self::TargetRemoteMissing => "target_remote_missing",
            Self::TargetRemoteBranchMissing => "target_remote_branch_missing",
        }
    }

    /// Returns true when the preview must explicitly call out missing remote truth.
    pub const fn is_missing(self) -> bool {
        matches!(
            self,
            Self::UpstreamMissing | Self::TargetRemoteMissing | Self::TargetRemoteBranchMissing
        )
    }
}

/// Actor identity attached to branch preview, apply, and support records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchActorRef {
    /// Actor class token from the local mutation lineage vocabulary.
    pub actor_class: String,
    /// Redaction-safe actor label.
    pub display_label: String,
    /// Optional stable principal or process ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
}

impl Default for GitBranchActorRef {
    fn default() -> Self {
        Self {
            actor_class: "local_user".to_string(),
            display_label: "Local user".to_string(),
            stable_id: None,
        }
    }
}

/// Request for a preview-first branch operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchRequest {
    /// Stable workspace identity copied into downstream records.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub root_path: PathBuf,
    /// Operation being requested.
    pub operation: GitBranchOperationKind,
    /// Branch name or revision requested by the caller.
    pub target: String,
    /// Optional start point for branch creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_point: Option<String>,
    /// True when branch creation should request upstream tracking.
    pub track_remote: bool,
    /// Actor that initiated the request.
    pub actor: GitBranchActorRef,
    /// Public row or surface ref that launched the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Timestamp supplied by the caller for deterministic exports.
    pub requested_at: String,
}

impl GitBranchRequest {
    /// Builds a request with a derived local workspace identity.
    pub fn for_target(
        root_path: impl Into<PathBuf>,
        operation: GitBranchOperationKind,
        target: impl Into<String>,
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
            target: target.into(),
            start_point: None,
            track_remote: false,
            actor: GitBranchActorRef::default(),
            launch_source_ref: None,
            requested_at: observed_at_now(),
        }
    }

    /// Builds a request with explicit identity and timestamp fields.
    pub fn with_observed_at(
        workspace_ref: impl Into<String>,
        root_path: impl Into<PathBuf>,
        operation: GitBranchOperationKind,
        target: impl Into<String>,
        requested_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            root_path: root_path.into(),
            operation,
            target: target.into(),
            start_point: None,
            track_remote: false,
            actor: GitBranchActorRef::default(),
            launch_source_ref: None,
            requested_at: requested_at.into(),
        }
    }

    /// Attaches a branch-creation start point to the request.
    pub fn with_start_point(mut self, start_point: impl Into<String>) -> Self {
        self.start_point = Some(start_point.into());
        self
    }

    /// Requests upstream tracking when creating a branch from a remote target.
    pub fn with_track_remote(mut self, track_remote: bool) -> Self {
        self.track_remote = track_remote;
        self
    }

    /// Attaches an actor identity to the request.
    pub fn with_actor(mut self, actor: GitBranchActorRef) -> Self {
        self.actor = actor;
        self
    }

    /// Attaches a public launch-source ref to the request.
    pub fn with_launch_source_ref(mut self, launch_source_ref: impl Into<String>) -> Self {
        self.launch_source_ref = Some(launch_source_ref.into());
        self
    }
}

/// Target identity and risk disclosures shown before branch apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchTargetReview {
    /// Stable target ref used by activity, support, and journal records.
    pub target_ref: String,
    /// Target string supplied by the caller after trimming.
    pub requested_target: String,
    /// Target class disclosed to shell and review surfaces.
    pub target_kind: GitBranchTargetKind,
    /// Full local branch ref when the target is or will become a branch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_ref: Option<String>,
    /// Optional start point used by branch creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_point: Option<String>,
    /// Commit object id resolved before apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_oid: Option<String>,
    /// Compact commit object id resolved before apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_short_oid: Option<String>,
    /// Upstream or remote branch ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref: Option<String>,
    /// Remote name involved in this target when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_name: Option<String>,
    /// Remote or upstream state disclosed before apply.
    pub remote_state: GitBranchRemoteState,
    /// True when checkout will leave `HEAD` detached and the preview says so.
    pub detached_head_disclosed: bool,
    /// True when missing remote or upstream state is visible in the preview.
    pub missing_remote_disclosed: bool,
    /// True when the target can be passed to Git apply.
    pub target_resolved: bool,
    /// Reasons that block this target before apply.
    pub blocked_reasons: Vec<String>,
}

impl GitBranchTargetReview {
    /// Returns true when target identity is sufficiently explicit for apply.
    pub fn is_satisfied(&self) -> bool {
        self.target_resolved && self.blocked_reasons.is_empty()
    }
}

/// Current-work review shown before changing branches or checkout state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchCurrentWorkReview {
    /// Stable current-work review ref.
    pub current_work_ref: String,
    /// Stable basis snapshot ref used to compute this review.
    pub basis_snapshot_ref: String,
    /// Redaction-safe fingerprint for drift detection before apply.
    pub basis_worktree_fingerprint: String,
    /// Count summary observed before apply.
    pub change_summary: ChangeSummary,
    /// Current branch, detached, or unknown state.
    pub head_state: BranchState,
    /// Current branch label when attached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Current compact revision ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_ref: Option<String>,
    /// Remote state for the current branch.
    pub current_branch_remote_state: String,
    /// True when staged, unstaged, untracked, or conflicted work is present.
    pub uncommitted_work_present: bool,
    /// True when the preview must show current-work warning copy.
    pub uncommitted_warning_required: bool,
    /// Stable warning state for UI and fixtures.
    pub warning_state: String,
    /// Human-readable warning label.
    pub warning_label: String,
    /// True when conflicts block branch switching before Git runs.
    pub apply_blocked_by_current_work: bool,
    /// Changed paths disclosed by status, without raw patch bodies.
    pub changed_path_labels: Vec<String>,
}

/// Activity-center projection for branch preview or result state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchActivityRecord {
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
    /// Result ref when an apply command resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Branch journal ref when an apply command resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_journal_ref: Option<String>,
    /// Command id that reopens branch operation details.
    pub open_details_command_id: String,
    /// Support-export ref that carries the same attribution.
    pub support_export_ref: String,
    /// Status snapshot ref used for shell synchronization joins.
    pub truth_source_ref: String,
}

/// Redaction-safe support export projection for a branch operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchSupportExportRecord {
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
    /// Phase token for preview or apply.
    pub phase: String,
    /// Workspace identity copied from the preview.
    pub workspace_ref: String,
    /// Target ref copied from the preview.
    pub target_ref: String,
    /// Current-work review ref copied from the preview.
    pub current_work_ref: String,
    /// Preview ref copied from the preview.
    pub preview_ref: String,
    /// Result ref when a branch operation completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Branch journal ref when a branch operation completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_journal_ref: Option<String>,
    /// Evidence refs included without raw command or patch bodies.
    pub evidence_refs: Vec<String>,
    /// Fields deliberately omitted from export.
    pub omitted_fields: Vec<String>,
}

/// Mutation-journal shaped record emitted after branch apply resolves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchJournalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable journal ref.
    pub branch_journal_ref: String,
    /// Canonical command id that mutated Git state.
    pub command_id: String,
    /// Actor that initiated the mutation.
    pub actor: GitBranchActorRef,
    /// Source class for the mutation journal.
    pub source_class: String,
    /// Operation token.
    pub operation_kind: String,
    /// Target ref copied from the preview.
    pub target_ref: String,
    /// Current-work review ref copied from the preview.
    pub current_work_ref: String,
    /// Timestamp when preview started.
    pub started_at: String,
    /// Timestamp when apply resolved.
    pub resolved_at: String,
    /// Status snapshot ref before the branch operation.
    pub before_truth_source_ref: String,
    /// Status snapshot ref after the branch operation when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_truth_source_ref: Option<String>,
    /// Branch or revision ref before the branch operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_head_ref: Option<String>,
    /// Branch or revision ref after the branch operation when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_head_ref: Option<String>,
    /// Reversal or recovery class advertised for this branch operation.
    pub recovery_class: String,
    /// Redaction class for support exports.
    pub redaction_class: String,
    /// Redaction-safe side-effect summary.
    pub side_effect_summary: String,
}

/// Review-first preview packet for a branch operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchPreview {
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
    pub operation: GitBranchOperationKind,
    /// Canonical command id for apply.
    pub command_id: String,
    /// User-facing operation label.
    pub operation_label: String,
    /// Consequence class for review sheets.
    pub consequence_class: String,
    /// Current preview state.
    pub preview_state: GitBranchPreviewState,
    /// Actor that initiated the preview.
    pub actor: GitBranchActorRef,
    /// Public row or surface ref that launched the preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Current `HEAD` identity from the source snapshot.
    pub current_head: HeadIdentity,
    /// Target identity and remote/detached disclosures.
    pub target: GitBranchTargetReview,
    /// Current-work warning and drift basis.
    pub current_work: GitBranchCurrentWorkReview,
    /// Shell projection derived from the source snapshot.
    pub before_shell: GitShellStatusRecord,
    /// Activity projection for the preview state.
    pub activity: GitBranchActivityRecord,
    /// Support-export projection for the preview state.
    pub support_export: GitBranchSupportExportRecord,
    /// Reasons that block apply from this preview.
    pub blocked_reasons: Vec<String>,
    #[serde(skip)]
    target_for_apply: String,
    #[serde(skip)]
    start_point_for_apply: Option<String>,
    #[serde(skip)]
    track_remote_for_apply: bool,
}

impl GitBranchPreview {
    /// Returns true when branch apply may proceed without recomputing scope.
    pub fn ready_to_apply(&self) -> bool {
        self.preview_state == GitBranchPreviewState::ReadyToApply
            && self.blocked_reasons.is_empty()
            && self.target.is_satisfied()
            && !self.current_work.apply_blocked_by_current_work
            && !self.target_for_apply.trim().is_empty()
    }
}

/// Result packet emitted after applying or blocking a branch preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitBranchResult {
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
    /// Branch operation that was applied.
    pub operation: GitBranchOperationKind,
    /// Final outcome state.
    pub outcome_state: GitBranchOutcomeState,
    /// Target identity copied from the preview.
    pub target: GitBranchTargetReview,
    /// Current-work review copied from the preview.
    pub current_work: GitBranchCurrentWorkReview,
    /// Shell projection derived before the branch operation.
    pub before_shell: GitShellStatusRecord,
    /// Shell projection derived after the branch operation when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_shell: Option<GitShellStatusRecord>,
    /// Status snapshot ref before the branch operation.
    pub before_truth_source_ref: String,
    /// Status snapshot ref after the branch operation when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_truth_source_ref: Option<String>,
    /// Head identity after the branch operation when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_head: Option<HeadIdentity>,
    /// Mutation-journal shaped lineage record.
    pub branch_journal: GitBranchJournalRecord,
    /// Activity projection for the result.
    pub activity: GitBranchActivityRecord,
    /// Support-export projection for the result.
    pub support_export: GitBranchSupportExportRecord,
    /// Failure reason when apply failed after starting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Reasons that blocked or failed the operation.
    pub blocked_reasons: Vec<String>,
}

impl GitBranchResult {
    /// Returns true when after-branch identity and shell projection share truth.
    pub fn branch_identity_synchronized(&self) -> bool {
        if self.outcome_state != GitBranchOutcomeState::Applied {
            return true;
        }
        let (Some(after_shell), Some(after_head), Some(after_truth_source_ref)) = (
            self.after_shell.as_ref(),
            self.after_head.as_ref(),
            self.after_truth_source_ref.as_ref(),
        ) else {
            return false;
        };
        after_shell.truth_source_ref == *after_truth_source_ref
            && after_shell.branch_label == after_head.branch_label
    }
}

/// Output captured from a Git branch command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitBranchCommandOutput {
    /// True when Git exited successfully.
    pub success: bool,
    /// Process exit status code when available.
    pub status_code: Option<i32>,
    /// Captured stdout bytes.
    pub stdout: Vec<u8>,
    /// Captured stderr bytes.
    pub stderr: Vec<u8>,
}

/// Error raised before a Git branch command can be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitBranchBackendError {
    /// Redaction-safe error message.
    pub message: String,
}

impl std::fmt::Display for GitBranchBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GitBranchBackendError {}

/// Backend used by [`GitBranchService`] to execute local Git commands.
pub trait GitBranchBackend {
    /// Runs `git -C root args`.
    ///
    /// # Errors
    ///
    /// Returns [`GitBranchBackendError`] when the backend cannot launch or
    /// supervise the Git process.
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitBranchCommandOutput, GitBranchBackendError>;
}

/// Git backend that shells out to the system `git` executable.
#[derive(Debug, Clone)]
pub struct SystemGitBranchBackend {
    git_binary: PathBuf,
}

impl Default for SystemGitBranchBackend {
    fn default() -> Self {
        Self::new("git")
    }
}

impl SystemGitBranchBackend {
    /// Creates a backend that invokes `git_binary`.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl GitBranchBackend for SystemGitBranchBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitBranchCommandOutput, GitBranchBackendError> {
        let output = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|err| GitBranchBackendError {
                message: format!("git command failed to launch: {err}"),
            })?;
        Ok(GitBranchCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

/// Service that creates and applies branch operation previews.
#[derive(Debug, Clone)]
pub struct GitBranchService<B = SystemGitBranchBackend> {
    backend: B,
}

impl Default for GitBranchService<SystemGitBranchBackend> {
    fn default() -> Self {
        Self::new(SystemGitBranchBackend::default())
    }
}

impl<B: GitBranchBackend> GitBranchService<B> {
    /// Creates a service backed by `backend`.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Builds a reviewable branch preview without mutating Git state.
    pub fn preview(&self, request: &GitBranchRequest) -> GitBranchPreview {
        let status_request = GitStatusRequest::with_observed_at(
            request.workspace_ref.clone(),
            request.root_path.clone(),
            request.requested_at.clone(),
        );
        let snapshot = GitStatusService::default().snapshot(&status_request);
        let bundle =
            ConsumerProjectionBundle::from_snapshot(request.requested_at.clone(), &snapshot);
        let truth_source_ref = bundle.truth_source_ref.clone();
        let repo_root = snapshot
            .repository
            .as_ref()
            .map(|repo| repo.repo_root.clone())
            .unwrap_or_else(|| request.root_path.clone());
        let preview_ref = preview_ref(request);

        if snapshot.service_state != GitServiceState::Current {
            return degraded_preview(
                request,
                repo_root,
                truth_source_ref,
                preview_ref,
                snapshot.head,
                bundle.shell,
                snapshot.service_state.as_str(),
            );
        }

        let current_work = current_work_review(&preview_ref, &truth_source_ref, &snapshot);
        let target = self.target_review(request, &preview_ref, &repo_root, &snapshot);
        let mut blocked_reasons = target.blocked_reasons.clone();
        if current_work.apply_blocked_by_current_work {
            blocked_reasons.push(
                "conflicted current work requires conflict review before changing branches"
                    .to_string(),
            );
        }
        let preview_state = if blocked_reasons.is_empty() {
            GitBranchPreviewState::ReadyToApply
        } else {
            GitBranchPreviewState::Blocked
        };
        let support_export =
            support_export_for_preview(&preview_ref, request, &target, &current_work);
        let activity = activity_for_preview(
            &preview_ref,
            request.operation,
            preview_state,
            &target,
            &current_work,
            &support_export.support_export_ref,
            &truth_source_ref,
        );

        GitBranchPreview {
            record_kind: GIT_BRANCH_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_BRANCH_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            operation: request.operation,
            command_id: request.operation.command_id().to_string(),
            operation_label: request.operation.label().to_string(),
            consequence_class: request.operation.consequence_class().to_string(),
            preview_state,
            actor: request.actor.clone(),
            launch_source_ref: request.launch_source_ref.clone(),
            current_head: snapshot.head,
            target,
            current_work,
            before_shell: bundle.shell,
            activity,
            support_export,
            blocked_reasons,
            target_for_apply: request.target.trim().to_string(),
            start_point_for_apply: request
                .start_point
                .as_ref()
                .map(|value| value.trim().to_string()),
            track_remote_for_apply: request.track_remote,
        }
    }

    /// Applies an admitted branch preview and returns an attributable result packet.
    pub fn apply(
        &self,
        preview: &GitBranchPreview,
        resolved_at: impl Into<String>,
    ) -> GitBranchResult {
        let resolved_at = resolved_at.into();
        if !preview.ready_to_apply() {
            return result_for_preview(
                preview,
                &resolved_at,
                GitBranchOutcomeState::BlockedNoChangesMade,
                None,
                preview.blocked_reasons.clone(),
                None,
            );
        }

        let status_request = GitStatusRequest::with_observed_at(
            preview.workspace_ref.clone(),
            preview.repo_root.clone(),
            resolved_at.clone(),
        );
        let pre_apply_snapshot = GitStatusService::default().snapshot(&status_request);
        if pre_apply_snapshot.service_state != GitServiceState::Current {
            return result_for_preview(
                preview,
                &resolved_at,
                GitBranchOutcomeState::BlockedNoChangesMade,
                Some("Git status became unavailable before branch apply".to_string()),
                vec!["Git status became unavailable before branch apply".to_string()],
                None,
            );
        }
        if current_work_fingerprint(&pre_apply_snapshot)
            != preview.current_work.basis_worktree_fingerprint
        {
            return result_for_preview(
                preview,
                &resolved_at,
                GitBranchOutcomeState::BlockedNoChangesMade,
                Some("current work changed after preview; reopen branch review".to_string()),
                vec!["current work changed after preview".to_string()],
                None,
            );
        }

        let output = self.apply_preview(preview);
        let after_bundle = self.after_bundle(preview, &resolved_at);
        let (outcome_state, failure_reason) = match output {
            Ok(output) if output.success => (GitBranchOutcomeState::Applied, None),
            Ok(output) => (
                GitBranchOutcomeState::Failed,
                Some(stderr_or_status(&output)),
            ),
            Err(err) => (GitBranchOutcomeState::Failed, Some(err.message)),
        };
        let blocked_reasons = failure_reason.clone().into_iter().collect();
        result_for_preview(
            preview,
            &resolved_at,
            outcome_state,
            failure_reason,
            blocked_reasons,
            after_bundle,
        )
    }

    fn target_review(
        &self,
        request: &GitBranchRequest,
        preview_ref: &str,
        repo_root: &Path,
        snapshot: &crate::status::GitStatusSnapshot,
    ) -> GitBranchTargetReview {
        match request.operation {
            GitBranchOperationKind::Switch => {
                self.switch_target_review(request, preview_ref, repo_root, snapshot)
            }
            GitBranchOperationKind::Create => {
                self.create_target_review(request, preview_ref, repo_root)
            }
            GitBranchOperationKind::Checkout => {
                self.checkout_target_review(request, preview_ref, repo_root)
            }
        }
    }

    fn switch_target_review(
        &self,
        request: &GitBranchRequest,
        preview_ref: &str,
        repo_root: &Path,
        _snapshot: &crate::status::GitStatusSnapshot,
    ) -> GitBranchTargetReview {
        let requested_target = request.target.trim().to_string();
        let target_ref = target_ref(preview_ref, &requested_target);
        let mut blocked_reasons = Vec::new();
        let branch_name = match self.valid_branch_name(repo_root, &requested_target) {
            Ok(name) => name,
            Err(reason) => {
                blocked_reasons.push(reason);
                requested_target.clone()
            }
        };
        let branch_exists =
            blocked_reasons.is_empty() && self.local_branch_exists(repo_root, &branch_name);
        let target_oid = branch_exists
            .then(|| self.resolve_commit_oid(repo_root, &branch_name))
            .flatten();
        let upstream_ref = branch_exists
            .then(|| self.upstream_for_branch(repo_root, &branch_name))
            .flatten();
        let mut remote_state = if branch_exists {
            if upstream_ref.is_some() {
                GitBranchRemoteState::UpstreamConfigured
            } else {
                GitBranchRemoteState::UpstreamMissing
            }
        } else {
            GitBranchRemoteState::NotApplicable
        };
        let mut remote_name = None;
        let target_kind = if branch_exists {
            GitBranchTargetKind::LocalBranch
        } else {
            let remotes = self.remote_names(repo_root);
            let remote = if self
                .resolve_commit_oid(repo_root, &requested_target)
                .is_some()
            {
                remote_name_from_candidate(&requested_target, &remotes)
            } else {
                remote_name_from_unresolved_candidate(&requested_target, &remotes)
            };
            remote_name = remote.name;
            remote_state = remote.state;
            if remote_state == GitBranchRemoteState::TargetRemoteAvailable {
                GitBranchTargetKind::RemoteTrackingBranch
            } else {
                GitBranchTargetKind::Unknown
            }
        };
        if !branch_exists {
            if target_kind == GitBranchTargetKind::RemoteTrackingBranch {
                blocked_reasons.push(
                    "target is a remote-tracking branch; create a local branch before switch"
                        .to_string(),
                );
            } else if remote_state.is_missing() {
                blocked_reasons.push("target remote state is not available locally".to_string());
            } else if blocked_reasons.is_empty() {
                blocked_reasons.push("local branch does not exist".to_string());
            }
        }
        if target_kind == GitBranchTargetKind::RemoteTrackingBranch {
            remote_state = GitBranchRemoteState::TargetRemoteAvailable;
        }

        GitBranchTargetReview {
            target_ref,
            requested_target,
            target_kind,
            branch_ref: branch_exists.then(|| format!("refs/heads/{branch_name}")),
            start_point: None,
            target_short_oid: target_oid.as_deref().map(short_oid),
            target_oid,
            upstream_ref,
            remote_name,
            remote_state,
            detached_head_disclosed: false,
            missing_remote_disclosed: remote_state.is_missing(),
            target_resolved: branch_exists,
            blocked_reasons,
        }
    }

    fn create_target_review(
        &self,
        request: &GitBranchRequest,
        preview_ref: &str,
        repo_root: &Path,
    ) -> GitBranchTargetReview {
        let requested_target = request.target.trim().to_string();
        let target_ref = target_ref(preview_ref, &requested_target);
        let mut blocked_reasons = Vec::new();
        let branch_name = match self.valid_branch_name(repo_root, &requested_target) {
            Ok(name) => name,
            Err(reason) => {
                blocked_reasons.push(reason);
                requested_target.clone()
            }
        };
        if blocked_reasons.is_empty() && self.local_branch_exists(repo_root, &branch_name) {
            blocked_reasons.push("local branch already exists".to_string());
        }

        let start_point = request
            .start_point
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("HEAD")
            .to_string();
        let target_oid = self.resolve_commit_oid(repo_root, &start_point);
        let remotes = self.remote_names(repo_root);
        let remote = if target_oid.is_some() {
            remote_name_from_candidate(&start_point, &remotes)
        } else {
            remote_name_from_unresolved_candidate(&start_point, &remotes)
        };
        if target_oid.is_none() {
            if remote.state.is_missing() {
                blocked_reasons
                    .push("start point remote state is not available locally".to_string());
            } else {
                blocked_reasons.push("branch start point could not be resolved".to_string());
            }
        }

        GitBranchTargetReview {
            target_ref,
            requested_target,
            target_kind: GitBranchTargetKind::NewBranch,
            branch_ref: Some(format!("refs/heads/{branch_name}")),
            start_point: Some(start_point.clone()),
            target_short_oid: target_oid.as_deref().map(short_oid),
            target_oid,
            upstream_ref: request
                .track_remote
                .then(|| Some(start_point.clone()))
                .flatten()
                .filter(|_| remote.state == GitBranchRemoteState::TargetRemoteAvailable),
            remote_name: remote.name,
            remote_state: remote.state,
            detached_head_disclosed: false,
            missing_remote_disclosed: remote.state.is_missing(),
            target_resolved: blocked_reasons.is_empty(),
            blocked_reasons,
        }
    }

    fn checkout_target_review(
        &self,
        request: &GitBranchRequest,
        preview_ref: &str,
        repo_root: &Path,
    ) -> GitBranchTargetReview {
        let requested_target = request.target.trim().to_string();
        let target_ref = target_ref(preview_ref, &requested_target);
        let mut blocked_reasons = Vec::new();
        if requested_target.is_empty() {
            blocked_reasons.push("checkout target is required".to_string());
        }
        let target_oid = if blocked_reasons.is_empty() {
            self.resolve_commit_oid(repo_root, &requested_target)
        } else {
            None
        };
        let local_branch_exists = target_oid.is_some()
            && self
                .valid_branch_name(repo_root, &requested_target)
                .ok()
                .is_some_and(|branch| self.local_branch_exists(repo_root, &branch));
        let remotes = self.remote_names(repo_root);
        let remote = if target_oid.is_some() {
            remote_name_from_candidate(&requested_target, &remotes)
        } else {
            remote_name_from_unresolved_candidate(&requested_target, &remotes)
        };
        if target_oid.is_none() && blocked_reasons.is_empty() {
            if remote.state.is_missing() {
                blocked_reasons
                    .push("checkout target remote state is not available locally".to_string());
            } else {
                blocked_reasons.push("checkout target could not be resolved".to_string());
            }
        }
        let target_kind = if local_branch_exists {
            GitBranchTargetKind::LocalBranch
        } else if target_oid.is_some() {
            GitBranchTargetKind::DetachedHead
        } else {
            GitBranchTargetKind::Unknown
        };
        let upstream_ref = if local_branch_exists {
            self.upstream_for_branch(repo_root, &requested_target)
        } else {
            None
        };
        let remote_state = if local_branch_exists {
            if upstream_ref.is_some() {
                GitBranchRemoteState::UpstreamConfigured
            } else {
                GitBranchRemoteState::UpstreamMissing
            }
        } else {
            remote.state
        };

        GitBranchTargetReview {
            target_ref,
            requested_target: requested_target.clone(),
            target_kind,
            branch_ref: local_branch_exists.then(|| format!("refs/heads/{requested_target}")),
            start_point: None,
            target_short_oid: target_oid.as_deref().map(short_oid),
            target_oid,
            upstream_ref,
            remote_name: remote.name,
            remote_state,
            detached_head_disclosed: target_kind == GitBranchTargetKind::DetachedHead,
            missing_remote_disclosed: remote_state.is_missing(),
            target_resolved: blocked_reasons.is_empty(),
            blocked_reasons,
        }
    }

    fn valid_branch_name(&self, repo_root: &Path, value: &str) -> Result<String, String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("branch target is required".to_string());
        }
        if trimmed.starts_with('-') {
            return Err("branch names starting with '-' are not accepted".to_string());
        }
        let args = vec![
            "check-ref-format".to_string(),
            "--branch".to_string(),
            trimmed.to_string(),
        ];
        match self.backend.run_git(repo_root, &args) {
            Ok(output) if output.success => {
                let normalized = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if normalized.is_empty() {
                    Ok(trimmed.to_string())
                } else {
                    Ok(normalized)
                }
            }
            Ok(output) => Err(stderr_or_status(&output)),
            Err(err) => Err(err.message),
        }
    }

    fn local_branch_exists(&self, repo_root: &Path, branch: &str) -> bool {
        let args = vec![
            "show-ref".to_string(),
            "--verify".to_string(),
            "--quiet".to_string(),
            format!("refs/heads/{branch}"),
        ];
        self.backend
            .run_git(repo_root, &args)
            .is_ok_and(|output| output.success)
    }

    fn resolve_commit_oid(&self, repo_root: &Path, target: &str) -> Option<String> {
        let args = vec![
            "rev-parse".to_string(),
            "--verify".to_string(),
            format!("{target}^{{commit}}"),
        ];
        let output = self.backend.run_git(repo_root, &args).ok()?;
        if !output.success {
            return None;
        }
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        (!value.is_empty()).then_some(value)
    }

    fn upstream_for_branch(&self, repo_root: &Path, branch: &str) -> Option<String> {
        let args = vec![
            "rev-parse".to_string(),
            "--abbrev-ref".to_string(),
            format!("{branch}@{{upstream}}"),
        ];
        let output = self.backend.run_git(repo_root, &args).ok()?;
        if !output.success {
            return None;
        }
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        (!value.is_empty()).then_some(value)
    }

    fn remote_names(&self, repo_root: &Path) -> Vec<String> {
        let args = vec!["remote".to_string()];
        let output = match self.backend.run_git(repo_root, &args) {
            Ok(output) if output.success => output,
            _ => return Vec::new(),
        };
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect()
    }

    fn apply_preview(
        &self,
        preview: &GitBranchPreview,
    ) -> Result<GitBranchCommandOutput, GitBranchBackendError> {
        let args = match preview.operation {
            GitBranchOperationKind::Switch => vec![
                "switch".to_string(),
                "--".to_string(),
                preview.target_for_apply.clone(),
            ],
            GitBranchOperationKind::Create => {
                let mut args = vec!["switch".to_string()];
                if preview.track_remote_for_apply {
                    args.push("--track".to_string());
                }
                args.extend(["-c".to_string(), preview.target_for_apply.clone()]);
                if let Some(start_point) = preview.start_point_for_apply.as_ref() {
                    args.push(start_point.clone());
                }
                args
            }
            GitBranchOperationKind::Checkout => {
                if preview.target.target_kind == GitBranchTargetKind::LocalBranch {
                    vec!["checkout".to_string(), preview.target_for_apply.clone()]
                } else {
                    vec![
                        "checkout".to_string(),
                        "--detach".to_string(),
                        preview.target_for_apply.clone(),
                    ]
                }
            }
        };
        self.backend.run_git(&preview.repo_root, &args)
    }

    fn after_bundle(
        &self,
        preview: &GitBranchPreview,
        resolved_at: &str,
    ) -> Option<BranchAfterBundle> {
        let status_request = GitStatusRequest::with_observed_at(
            preview.workspace_ref.clone(),
            preview.repo_root.clone(),
            resolved_at.to_string(),
        );
        let snapshot = GitStatusService::default().snapshot(&status_request);
        if snapshot.service_state != GitServiceState::Current {
            return None;
        }
        let bundle = ConsumerProjectionBundle::from_snapshot(resolved_at.to_string(), &snapshot);
        Some(BranchAfterBundle {
            truth_source_ref: bundle.truth_source_ref,
            shell: bundle.shell,
            head: snapshot.head,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RemoteInspection {
    name: Option<String>,
    state: GitBranchRemoteState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BranchAfterBundle {
    truth_source_ref: String,
    shell: GitShellStatusRecord,
    head: HeadIdentity,
}

#[allow(clippy::too_many_arguments)]
fn degraded_preview(
    request: &GitBranchRequest,
    repo_root: PathBuf,
    truth_source_ref: String,
    preview_ref: String,
    current_head: HeadIdentity,
    before_shell: GitShellStatusRecord,
    reason: &str,
) -> GitBranchPreview {
    let target = GitBranchTargetReview {
        target_ref: target_ref(&preview_ref, request.target.trim()),
        requested_target: request.target.trim().to_string(),
        target_kind: GitBranchTargetKind::Unknown,
        branch_ref: None,
        start_point: request.start_point.clone(),
        target_oid: None,
        target_short_oid: None,
        upstream_ref: None,
        remote_name: None,
        remote_state: GitBranchRemoteState::NotApplicable,
        detached_head_disclosed: false,
        missing_remote_disclosed: false,
        target_resolved: false,
        blocked_reasons: vec![format!("Git service degraded: {reason}")],
    };
    let current_work = GitBranchCurrentWorkReview {
        current_work_ref: format!("{}.current_work", preview_ref),
        basis_snapshot_ref: truth_source_ref.clone(),
        basis_worktree_fingerprint: "git-status-unavailable".to_string(),
        change_summary: ChangeSummary::default(),
        head_state: current_head.state,
        branch_label: current_head.branch_label.clone(),
        head_revision_ref: current_head
            .head_short_oid
            .as_deref()
            .map(head_revision_ref),
        current_branch_remote_state: "unknown".to_string(),
        uncommitted_work_present: false,
        uncommitted_warning_required: false,
        warning_state: "git_status_unavailable".to_string(),
        warning_label: "Git status unavailable before branch review".to_string(),
        apply_blocked_by_current_work: true,
        changed_path_labels: Vec::new(),
    };
    let blocked_reasons = vec![format!("Git service degraded: {reason}")];
    let support_export = support_export_for_preview(&preview_ref, request, &target, &current_work);
    let activity = activity_for_preview(
        &preview_ref,
        request.operation,
        GitBranchPreviewState::Degraded,
        &target,
        &current_work,
        &support_export.support_export_ref,
        &truth_source_ref,
    );
    GitBranchPreview {
        record_kind: GIT_BRANCH_PREVIEW_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_PREVIEW_SCHEMA_VERSION,
        preview_ref,
        generated_at: request.requested_at.clone(),
        workspace_ref: request.workspace_ref.clone(),
        repo_root,
        truth_source_ref,
        operation: request.operation,
        command_id: request.operation.command_id().to_string(),
        operation_label: request.operation.label().to_string(),
        consequence_class: request.operation.consequence_class().to_string(),
        preview_state: GitBranchPreviewState::Degraded,
        actor: request.actor.clone(),
        launch_source_ref: request.launch_source_ref.clone(),
        current_head,
        target,
        current_work,
        before_shell,
        activity,
        support_export,
        blocked_reasons,
        target_for_apply: request.target.trim().to_string(),
        start_point_for_apply: request.start_point.clone(),
        track_remote_for_apply: request.track_remote,
    }
}

fn current_work_review(
    preview_ref: &str,
    truth_source_ref: &str,
    snapshot: &crate::status::GitStatusSnapshot,
) -> GitBranchCurrentWorkReview {
    let summary = snapshot.change_summary.clone();
    let uncommitted_work_present = summary.total_changed_count > 0 || summary.conflicted_count > 0;
    let warning_state = if summary.conflicted_count > 0 {
        "conflicts_blocking"
    } else if uncommitted_work_present {
        "uncommitted_work_present"
    } else {
        "clean"
    };
    let warning_label = if summary.conflicted_count > 0 {
        format!(
            "{} conflicted path(s) must be resolved before changing branches",
            summary.conflicted_count
        )
    } else if uncommitted_work_present {
        format!("Uncommitted work present: {}", summary.compact_label())
    } else {
        "No uncommitted work detected".to_string()
    };
    let changed_path_labels = snapshot
        .changes
        .iter()
        .map(|change| change.path.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    GitBranchCurrentWorkReview {
        current_work_ref: format!("{}.current_work", preview_ref),
        basis_snapshot_ref: truth_source_ref.to_string(),
        basis_worktree_fingerprint: current_work_fingerprint(snapshot),
        change_summary: summary.clone(),
        head_state: snapshot.head.state,
        branch_label: snapshot.head.branch_label.clone(),
        head_revision_ref: snapshot
            .head
            .head_short_oid
            .as_deref()
            .map(head_revision_ref),
        current_branch_remote_state: current_branch_remote_state(&snapshot.head),
        uncommitted_work_present,
        uncommitted_warning_required: uncommitted_work_present,
        warning_state: warning_state.to_string(),
        warning_label,
        apply_blocked_by_current_work: summary.conflicted_count > 0,
        changed_path_labels,
    }
}

fn current_branch_remote_state(head: &HeadIdentity) -> String {
    match head.state {
        BranchState::Attached | BranchState::Unborn => {
            if head.upstream.is_some() {
                "upstream_configured"
            } else {
                "upstream_missing"
            }
        }
        BranchState::Detached => "detached_head",
        BranchState::Unknown => "unknown",
    }
    .to_string()
}

fn current_work_fingerprint(snapshot: &crate::status::GitStatusSnapshot) -> String {
    let mut parts = vec![
        snapshot.head.state.as_str().to_string(),
        snapshot.head.branch_ref.clone().unwrap_or_default(),
        snapshot.head.head_oid.clone().unwrap_or_default(),
    ];
    let mut changes = snapshot
        .changes
        .iter()
        .map(|change| {
            format!(
                "{}:{}",
                change.status_code,
                change.path.to_string_lossy().replace('\\', "/")
            )
        })
        .collect::<Vec<_>>();
    changes.sort();
    parts.extend(changes);
    parts.join("|")
}

fn remote_name_from_candidate(candidate: &str, remotes: &[String]) -> RemoteInspection {
    let Some(name) = candidate_remote_name(candidate, remotes, false) else {
        return RemoteInspection {
            name: None,
            state: GitBranchRemoteState::NotApplicable,
        };
    };
    let state = if remotes.iter().any(|remote| remote == &name) {
        GitBranchRemoteState::TargetRemoteAvailable
    } else {
        GitBranchRemoteState::TargetRemoteMissing
    };
    RemoteInspection {
        name: Some(name),
        state,
    }
}

fn remote_name_from_unresolved_candidate(candidate: &str, remotes: &[String]) -> RemoteInspection {
    let Some(name) = candidate_remote_name(candidate, remotes, true) else {
        return RemoteInspection {
            name: None,
            state: GitBranchRemoteState::NotApplicable,
        };
    };
    let state = if remotes.iter().any(|remote| remote == &name) {
        GitBranchRemoteState::TargetRemoteBranchMissing
    } else {
        GitBranchRemoteState::TargetRemoteMissing
    };
    RemoteInspection {
        name: Some(name),
        state,
    }
}

fn candidate_remote_name(
    candidate: &str,
    remotes: &[String],
    allow_common_missing: bool,
) -> Option<String> {
    let candidate = candidate.trim();
    if let Some(rest) = candidate.strip_prefix("refs/remotes/") {
        return rest
            .split('/')
            .next()
            .filter(|name| !name.is_empty())
            .map(str::to_string);
    }
    let (first, _) = candidate.split_once('/')?;
    if remotes.iter().any(|remote| remote == first) {
        return Some(first.to_string());
    }
    if allow_common_missing && matches!(first, "origin" | "upstream") {
        return Some(first.to_string());
    }
    None
}

fn activity_for_preview(
    preview_ref: &str,
    operation: GitBranchOperationKind,
    preview_state: GitBranchPreviewState,
    target: &GitBranchTargetReview,
    current_work: &GitBranchCurrentWorkReview,
    support_export_ref: &str,
    truth_source_ref: &str,
) -> GitBranchActivityRecord {
    let state_class = match preview_state {
        GitBranchPreviewState::ReadyToApply => "waiting_review",
        GitBranchPreviewState::Blocked => "blocked",
        GitBranchPreviewState::Degraded => "degraded",
    };
    let detail_label = format!(
        "{}; target {}; current work {}; remote {}",
        preview_state.as_str(),
        target.target_kind.as_str(),
        current_work.warning_state,
        target.remote_state.as_str()
    );
    GitBranchActivityRecord {
        record_kind: GIT_BRANCH_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(preview_ref)),
        job_family: "git_branch".to_string(),
        state_class: state_class.to_string(),
        partition: if preview_state == GitBranchPreviewState::ReadyToApply {
            "active_review"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: format!("{} preview", operation.label()),
        detail_label,
        preview_ref: preview_ref.to_string(),
        result_ref: None,
        branch_journal_ref: None,
        open_details_command_id: "cmd:git.branch.open_details".to_string(),
        support_export_ref: support_export_ref.to_string(),
        truth_source_ref: truth_source_ref.to_string(),
    }
}

fn activity_for_result(
    result_ref: &str,
    preview: &GitBranchPreview,
    outcome_state: GitBranchOutcomeState,
    branch_journal_ref: &str,
    support_export_ref: &str,
    after_shell: Option<&GitShellStatusRecord>,
    after_truth_source_ref: Option<&str>,
) -> GitBranchActivityRecord {
    let after_label = after_shell
        .map(|shell| shell.current_value_label.clone())
        .unwrap_or_else(|| "branch identity unavailable after apply".to_string());
    GitBranchActivityRecord {
        record_kind: GIT_BRANCH_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(result_ref)),
        job_family: "git_branch".to_string(),
        state_class: outcome_state.activity_state_class().to_string(),
        partition: if outcome_state == GitBranchOutcomeState::Applied {
            "completed"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: format!("{} {}", preview.operation.label(), outcome_state.as_str()),
        detail_label: format!(
            "{} -> {}; current work {}",
            preview.before_shell.current_value_label,
            after_label,
            preview.current_work.warning_state
        ),
        preview_ref: preview.preview_ref.clone(),
        result_ref: Some(result_ref.to_string()),
        branch_journal_ref: Some(branch_journal_ref.to_string()),
        open_details_command_id: "cmd:git.branch.open_details".to_string(),
        support_export_ref: support_export_ref.to_string(),
        truth_source_ref: after_truth_source_ref
            .unwrap_or(preview.truth_source_ref.as_str())
            .to_string(),
    }
}

fn support_export_for_preview(
    preview_ref: &str,
    request: &GitBranchRequest,
    target: &GitBranchTargetReview,
    current_work: &GitBranchCurrentWorkReview,
) -> GitBranchSupportExportRecord {
    GitBranchSupportExportRecord {
        record_kind: GIT_BRANCH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.export.{}", sanitize_id(preview_ref)),
        redaction_mode: "metadata_safe_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        operation_kind: request.operation.as_str().to_string(),
        phase: "preview".to_string(),
        workspace_ref: request.workspace_ref.clone(),
        target_ref: target.target_ref.clone(),
        current_work_ref: current_work.current_work_ref.clone(),
        preview_ref: preview_ref.to_string(),
        result_ref: None,
        branch_journal_ref: None,
        evidence_refs: vec![
            target.target_ref.clone(),
            current_work.current_work_ref.clone(),
        ],
        omitted_fields: vec![
            "raw_command_line".to_string(),
            "raw_patch_body".to_string(),
            "raw_actor_secret".to_string(),
        ],
    }
}

fn support_export_for_result(
    result_ref: &str,
    preview: &GitBranchPreview,
    branch_journal_ref: &str,
) -> GitBranchSupportExportRecord {
    GitBranchSupportExportRecord {
        record_kind: GIT_BRANCH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.export.{}", sanitize_id(result_ref)),
        redaction_mode: "metadata_safe_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        operation_kind: preview.operation.as_str().to_string(),
        phase: "apply".to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        target_ref: preview.target.target_ref.clone(),
        current_work_ref: preview.current_work.current_work_ref.clone(),
        preview_ref: preview.preview_ref.clone(),
        result_ref: Some(result_ref.to_string()),
        branch_journal_ref: Some(branch_journal_ref.to_string()),
        evidence_refs: vec![
            preview.target.target_ref.clone(),
            preview.current_work.current_work_ref.clone(),
            branch_journal_ref.to_string(),
        ],
        omitted_fields: vec![
            "raw_command_line".to_string(),
            "raw_patch_body".to_string(),
            "raw_actor_secret".to_string(),
        ],
    }
}

fn result_for_preview(
    preview: &GitBranchPreview,
    resolved_at: &str,
    outcome_state: GitBranchOutcomeState,
    failure_reason: Option<String>,
    blocked_reasons: Vec<String>,
    after_bundle: Option<BranchAfterBundle>,
) -> GitBranchResult {
    let result_ref = format!(
        "{}.result.{}",
        preview.preview_ref,
        sanitize_id(resolved_at)
    );
    let branch_journal_ref = format!("git.branch.journal.{}", sanitize_id(&result_ref));
    let support_export = support_export_for_result(&result_ref, preview, &branch_journal_ref);
    let after_truth_source_ref = after_bundle
        .as_ref()
        .map(|bundle| bundle.truth_source_ref.clone());
    let after_shell = after_bundle.as_ref().map(|bundle| bundle.shell.clone());
    let after_head = after_bundle.as_ref().map(|bundle| bundle.head.clone());
    let activity = activity_for_result(
        &result_ref,
        preview,
        outcome_state,
        &branch_journal_ref,
        &support_export.support_export_ref,
        after_shell.as_ref(),
        after_truth_source_ref.as_deref(),
    );
    let branch_journal = GitBranchJournalRecord {
        record_kind: GIT_BRANCH_JOURNAL_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_JOURNAL_SCHEMA_VERSION,
        branch_journal_ref: branch_journal_ref.clone(),
        command_id: preview.command_id.clone(),
        actor: preview.actor.clone(),
        source_class: "source_control_branch_review".to_string(),
        operation_kind: preview.operation.as_str().to_string(),
        target_ref: preview.target.target_ref.clone(),
        current_work_ref: preview.current_work.current_work_ref.clone(),
        started_at: preview.generated_at.clone(),
        resolved_at: resolved_at.to_string(),
        before_truth_source_ref: preview.truth_source_ref.clone(),
        after_truth_source_ref: after_truth_source_ref.clone(),
        before_head_ref: head_ref(&preview.current_head),
        after_head_ref: after_head.as_ref().and_then(head_ref),
        recovery_class: match preview.operation {
            GitBranchOperationKind::Switch | GitBranchOperationKind::Checkout => {
                "switch_back_to_previous_ref"
            }
            GitBranchOperationKind::Create => "switch_back_and_delete_created_branch_review",
        }
        .to_string(),
        redaction_class: "metadata_safe_default".to_string(),
        side_effect_summary: side_effect_summary(preview, outcome_state, after_head.as_ref()),
    };
    GitBranchResult {
        record_kind: GIT_BRANCH_RESULT_RECORD_KIND.to_string(),
        schema_version: GIT_BRANCH_RESULT_SCHEMA_VERSION,
        result_ref,
        preview_ref: preview.preview_ref.clone(),
        resolved_at: resolved_at.to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        repo_root: preview.repo_root.clone(),
        operation: preview.operation,
        outcome_state,
        target: preview.target.clone(),
        current_work: preview.current_work.clone(),
        before_shell: preview.before_shell.clone(),
        after_shell,
        before_truth_source_ref: preview.truth_source_ref.clone(),
        after_truth_source_ref,
        after_head,
        branch_journal,
        activity,
        support_export,
        failure_reason,
        blocked_reasons,
    }
}

fn side_effect_summary(
    preview: &GitBranchPreview,
    outcome_state: GitBranchOutcomeState,
    after_head: Option<&HeadIdentity>,
) -> String {
    match outcome_state {
        GitBranchOutcomeState::Applied => {
            let after = after_head
                .and_then(head_ref)
                .unwrap_or_else(|| "git.head.unknown".to_string());
            format!(
                "{} applied target {}; resulting head {}",
                preview.operation.label(),
                preview.target.requested_target,
                after
            )
        }
        GitBranchOutcomeState::BlockedNoChangesMade => {
            "branch operation blocked before Git mutation".to_string()
        }
        GitBranchOutcomeState::Failed => "git branch operation failed".to_string(),
    }
}

fn preview_ref(request: &GitBranchRequest) -> String {
    let mut parts = vec![
        "git.branch.preview".to_string(),
        sanitize_id(&request.workspace_ref),
        request.operation.as_str().to_string(),
        sanitize_id(&request.target),
    ];
    if let Some(start_point) = request.start_point.as_ref() {
        parts.push(sanitize_id(start_point));
    }
    parts.join(".")
}

fn target_ref(preview_ref: &str, target: &str) -> String {
    format!(
        "{}.target.{}",
        preview_ref,
        sanitize_id(if target.is_empty() { "empty" } else { target })
    )
}

fn head_ref(head: &HeadIdentity) -> Option<String> {
    head.branch_ref
        .clone()
        .or_else(|| head.head_short_oid.as_deref().map(head_revision_ref))
}

fn head_revision_ref(short_oid: &str) -> String {
    format!("git.rev.{short_oid}")
}

fn short_oid(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .take(7)
        .collect()
}

fn stderr_or_status(output: &GitBranchCommandOutput) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stderr.is_empty() {
        return stderr;
    }
    output
        .status_code
        .map(|code| format!("git exited with status {code}"))
        .unwrap_or_else(|| "git exited unsuccessfully".to_string())
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

fn observed_at_now() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("unix:{millis}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_missing_remote_is_disclosed_for_unresolved_origin_ref() {
        let remote = remote_name_from_unresolved_candidate("origin/feature", &[]);
        assert_eq!(remote.name.as_deref(), Some("origin"));
        assert_eq!(remote.state, GitBranchRemoteState::TargetRemoteMissing);
    }
}
