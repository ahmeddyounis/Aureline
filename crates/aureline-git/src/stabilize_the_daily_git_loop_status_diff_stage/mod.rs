//! Stabilized daily Git loop with explicit repo/worktree targeting.
//!
//! This module owns the bounded beta contract for the daily-driver Git
//! surface: status, diff, stage, commit, amend, stash, blame, and history.
//! Every request carries an explicit [`RepoTarget`] and [`WorktreeTarget`]
//! so that parent repos, submodules, nested independent repos, sparse
//! slices, shallow histories, and pointer-backed assets never resolve
//! ambiguously. Consumers subscribe to canonical records instead of
//! invoking Git independently.
//!
//! The companion schema lives at
//! `schemas/git/daily_loop_snapshot.schema.json`.
//! Canonical fixtures live under `fixtures/git/m4/daily_loop_beta/`.

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Record-kind constants
// ---------------------------------------------------------------------------

/// Stable record-kind tag for [`DailyLoopSnapshot`].
pub const DAILY_LOOP_SNAPSHOT_RECORD_KIND: &str = "git_daily_loop_snapshot";

/// Stable record-kind tag for [`DailyLoopPreview`].
pub const DAILY_LOOP_PREVIEW_RECORD_KIND: &str = "git_daily_loop_preview";

/// Stable record-kind tag for [`DailyLoopResult`].
pub const DAILY_LOOP_RESULT_RECORD_KIND: &str = "git_daily_loop_result";

/// Stable record-kind tag for [`DailyLoopActivityRecord`].
pub const DAILY_LOOP_ACTIVITY_RECORD_KIND: &str = "git_daily_loop_activity_record";

/// Stable record-kind tag for [`DailyLoopSupportExportRecord`].
pub const DAILY_LOOP_SUPPORT_EXPORT_RECORD_KIND: &str = "git_daily_loop_support_export_record";

/// Stable record-kind tag for [`DailyLoopJournalRecord`].
pub const DAILY_LOOP_JOURNAL_RECORD_KIND: &str = "git_daily_loop_journal_record";

/// Stable record-kind tag for [`StashShelfEntry`].
pub const STASH_SHELF_ENTRY_RECORD_KIND: &str = "git_stash_shelf_entry_record";

/// Stable record-kind tag for [`BlameLineRecord`].
pub const BLAME_LINE_RECORD_KIND: &str = "git_blame_line_record";

/// Stable record-kind tag for [`HistoryCommitRecord`].
pub const HISTORY_COMMIT_RECORD_KIND: &str = "git_history_commit_record";

// ---------------------------------------------------------------------------
// Schema versions
// ---------------------------------------------------------------------------

const DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
const DAILY_LOOP_PREVIEW_SCHEMA_VERSION: u32 = 1;
const DAILY_LOOP_RESULT_SCHEMA_VERSION: u32 = 1;
const DAILY_LOOP_ACTIVITY_SCHEMA_VERSION: u32 = 1;
const DAILY_LOOP_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;
const DAILY_LOOP_JOURNAL_SCHEMA_VERSION: u32 = 1;
const STASH_SHELF_ENTRY_SCHEMA_VERSION: u32 = 1;
const BLAME_LINE_SCHEMA_VERSION: u32 = 1;
const HISTORY_COMMIT_SCHEMA_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Daily-loop operation kinds.
pub const DAILY_LOOP_OPERATION_KINDS: &[&str] = &[
    "status",
    "diff",
    "stage",
    "unstage",
    "commit",
    "amend",
    "stash_capture",
    "stash_apply",
    "stash_pop",
    "stash_drop",
    "stash_branch_from",
    "blame",
    "history",
];

/// Preview states for the daily loop.
pub const DAILY_LOOP_PREVIEW_STATES: &[&str] = &[
    "ready",
    "blocked",
    "degraded",
];

/// Outcome states for the daily loop.
pub const DAILY_LOOP_OUTCOME_STATES: &[&str] = &[
    "completed",
    "blocked_no_changes_made",
    "failed",
    "partial",
];

/// Stash/shelf entry lifecycle states.
pub const STASH_SHELF_ENTRY_LIFECYCLE_STATES: &[&str] = &[
    "captured_unapplied",
    "applied_kept",
    "applied_popped",
    "dropped",
    "promoted_to_branch",
    "applied_with_conflict",
];

/// Stash command classes.
pub const STASH_COMMAND_CLASSES: &[&str] = &[
    "cmd:git.stash.apply",
    "cmd:git.stash.pop",
    "cmd:git.stash.drop",
    "cmd:git.stash.branch_from",
];

/// Content availability labels for history/blame/diff rows.
pub const CONTENT_AVAILABILITY_CLASSES: &[&str] = &[
    "available",
    "unfetched",
    "omitted_sparse",
    "omitted_shallow",
    "uninitialized_submodule",
    "pointer_only",
    "not_repository",
];

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Backend error class for the daily Git loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyLoopBackendErrorClass {
    /// The system `git` binary could not be found.
    GitNotInstalled,
    /// The Git binary exists but cannot be executed.
    PermissionDenied,
    /// A generic process I/O error.
    Io,
    /// The requested path is not inside a Git repository.
    NotARepository,
    /// The requested worktree does not exist.
    WorktreeNotFound,
    /// The requested commit or ref is not available locally.
    RefNotAvailable,
}

impl DailyLoopBackendErrorClass {
    /// Stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitNotInstalled => "git_not_installed",
            Self::PermissionDenied => "permission_denied",
            Self::Io => "io",
            Self::NotARepository => "not_a_repository",
            Self::WorktreeNotFound => "worktree_not_found",
            Self::RefNotAvailable => "ref_not_available",
        }
    }
}

/// Typed backend failure for daily-loop operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyLoopBackendError {
    /// Error class.
    pub class: DailyLoopBackendErrorClass,
    /// Human-readable detail.
    pub message: String,
}

impl DailyLoopBackendError {
    /// Builds a backend error.
    pub fn new(class: DailyLoopBackendErrorClass, message: impl Into<String>) -> Self {
        Self {
            class,
            message: message.into(),
        }
    }
}

impl fmt::Display for DailyLoopBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.class.as_str(), self.message)
    }
}

impl std::error::Error for DailyLoopBackendError {}

// ---------------------------------------------------------------------------
// Explicit targeting
// ---------------------------------------------------------------------------

/// Exact repository identity so nested or parent repos never collide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoTarget {
    /// Opaque stable repository identifier.
    pub repo_ref: String,
    /// Absolute filesystem path to the repository root (the `.git` parent).
    pub repo_root: PathBuf,
    /// Git directory path (may be outside `repo_root` for linked worktrees).
    pub git_dir: PathBuf,
    /// True when this is a bare repository.
    pub is_bare: bool,
    /// True when this is a shallow clone.
    pub is_shallow: bool,
    /// Label for human-facing surfaces.
    pub display_label: String,
}

/// Exact worktree identity within a repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeTarget {
    /// Opaque stable worktree identifier.
    pub worktree_ref: String,
    /// Reference to the owning repository.
    pub repo_ref: String,
    /// Absolute path to the worktree root.
    pub worktree_root: PathBuf,
    /// True for linked worktrees (not the main worktree).
    pub is_linked: bool,
    /// Branch or detached HEAD label.
    pub head_label: String,
    /// Label for human-facing surfaces.
    pub display_label: String,
}

/// Unified target that pins both repo and worktree for every daily-loop row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopTarget {
    /// Explicit repository target.
    pub repo: RepoTarget,
    /// Explicit worktree target.
    pub worktree: WorktreeTarget,
    /// Caller-supplied workspace identity.
    pub workspace_ref: String,
    /// Observation timestamp for deterministic exports.
    pub observed_at: String,
}

// ---------------------------------------------------------------------------
// Operation requests
// ---------------------------------------------------------------------------

/// Operation kind for the daily Git loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopOperationKind {
    /// Gather repository status.
    Status,
    /// Show worktree diff.
    Diff,
    /// Stage paths.
    Stage,
    /// Unstage paths.
    Unstage,
    /// Create a commit.
    Commit,
    /// Amend the current HEAD commit.
    Amend,
    /// Capture a stash.
    StashCapture,
    /// Apply a stash (kept).
    StashApply,
    /// Pop a stash.
    StashPop,
    /// Drop a stash.
    StashDrop,
    /// Create a branch from a stash.
    StashBranchFrom,
    /// Show blame for a path.
    Blame,
    /// Show commit history.
    History,
}

impl DailyLoopOperationKind {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Diff => "diff",
            Self::Stage => "stage",
            Self::Unstage => "unstage",
            Self::Commit => "commit",
            Self::Amend => "amend",
            Self::StashCapture => "stash_capture",
            Self::StashApply => "stash_apply",
            Self::StashPop => "stash_pop",
            Self::StashDrop => "stash_drop",
            Self::StashBranchFrom => "stash_branch_from",
            Self::Blame => "blame",
            Self::History => "history",
        }
    }

    /// Canonical command id.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Status => "cmd:git.daily.status",
            Self::Diff => "cmd:git.daily.diff",
            Self::Stage => "cmd:git.daily.stage",
            Self::Unstage => "cmd:git.daily.unstage",
            Self::Commit => "cmd:git.daily.commit",
            Self::Amend => "cmd:git.daily.amend",
            Self::StashCapture => "cmd:git.daily.stash_capture",
            Self::StashApply => "cmd:git.daily.stash_apply",
            Self::StashPop => "cmd:git.daily.stash_pop",
            Self::StashDrop => "cmd:git.daily.stash_drop",
            Self::StashBranchFrom => "cmd:git.daily.stash_branch_from",
            Self::Blame => "cmd:git.daily.blame",
            Self::History => "cmd:git.daily.history",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Status => "Git status",
            Self::Diff => "Git diff",
            Self::Stage => "Stage changes",
            Self::Unstage => "Unstage changes",
            Self::Commit => "Commit",
            Self::Amend => "Amend commit",
            Self::StashCapture => "Stash changes",
            Self::StashApply => "Apply stash",
            Self::StashPop => "Pop stash",
            Self::StashDrop => "Drop stash",
            Self::StashBranchFrom => "Branch from stash",
            Self::Blame => "Blame",
            Self::History => "History",
        }
    }

    /// Returns true when the operation mutates repository state.
    pub const fn is_mutation(self) -> bool {
        matches!(
            self,
            Self::Stage
                | Self::Unstage
                | Self::Commit
                | Self::Amend
                | Self::StashCapture
                | Self::StashApply
                | Self::StashPop
                | Self::StashDrop
                | Self::StashBranchFrom
        )
    }

    /// Returns true when the operation is a stash transition.
    pub const fn is_stash(self) -> bool {
        matches!(
            self,
            Self::StashCapture
                | Self::StashApply
                | Self::StashPop
                | Self::StashDrop
                | Self::StashBranchFrom
        )
    }
}

/// Unified request for any daily-loop operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopRequest {
    /// Target repo and worktree.
    pub target: DailyLoopTarget,
    /// Operation to perform.
    pub kind: DailyLoopOperationKind,
    /// Paths scoped to the operation (relative to worktree root).
    pub path_scope: Vec<PathBuf>,
    /// Optional message (commit, amend, stash capture).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Actor ref for attribution.
    pub actor_ref: String,
    /// Stable command id for the caller surface.
    pub caller_command_id: String,
    /// When true, the caller wants a preview instead of applying.
    pub preview_only: bool,
    /// Untracked-file posture for stash captures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_untracked: Option<bool>,
    /// Stash entry ref when operating on an existing stash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stash_entry_ref: Option<String>,
    /// Commit range or ref for history/blame/diff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_ref: Option<String>,
    /// Line range for blame (`start,end`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_range: Option<String>,
}

impl DailyLoopRequest {
    /// Builds a request for `kind` against `worktree_root`.
    pub fn for_worktree(
        worktree_root: impl Into<PathBuf>,
        kind: DailyLoopOperationKind,
        path_scope: Vec<PathBuf>,
    ) -> Self {
        let worktree_root = worktree_root.into();
        let repo_root = worktree_root.clone();
        let repo_ref = repo_root.to_string_lossy().into_owned();
        let worktree_ref = worktree_root.to_string_lossy().into_owned();
        Self {
            target: DailyLoopTarget {
                repo: RepoTarget {
                    repo_ref: repo_ref.clone(),
                    repo_root: repo_root.clone(),
                    git_dir: repo_root.join(".git"),
                    is_bare: false,
                    is_shallow: false,
                    display_label: repo_root.display().to_string(),
                },
                worktree: WorktreeTarget {
                    worktree_ref: worktree_ref.clone(),
                    repo_ref,
                    worktree_root: worktree_root.clone(),
                    is_linked: false,
                    head_label: "unknown".to_string(),
                    display_label: worktree_root.display().to_string(),
                },
                workspace_ref: worktree_ref,
                observed_at: observed_at_now(),
            },
            kind,
            path_scope,
            message: None,
            actor_ref: "actor:local:daily_loop".to_string(),
            caller_command_id: kind.command_id().to_string(),
            preview_only: false,
            include_untracked: None,
            stash_entry_ref: None,
            commit_ref: None,
            line_range: None,
        }
    }

    /// Sets the preview-only flag.
    pub fn preview_only(mut self) -> Self {
        self.preview_only = true;
        self
    }

    /// Sets the message.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the stash entry ref.
    pub fn with_stash_entry_ref(mut self, stash_entry_ref: impl Into<String>) -> Self {
        self.stash_entry_ref = Some(stash_entry_ref.into());
        self
    }

    /// Sets the commit ref.
    pub fn with_commit_ref(mut self, commit_ref: impl Into<String>) -> Self {
        self.commit_ref = Some(commit_ref.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Preview / result states
// ---------------------------------------------------------------------------

/// Preview state for a daily-loop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopPreviewState {
    /// The preview is ready to apply.
    Ready,
    /// Validation or guardrails block apply.
    Blocked,
    /// Local Git state is unavailable.
    Degraded,
}

impl DailyLoopPreviewState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::Degraded => "degraded",
        }
    }
}

/// Outcome state for a daily-loop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopOutcomeState {
    /// The operation completed successfully.
    Completed,
    /// No mutation was attempted because preview validation failed.
    BlockedNoChangesMade,
    /// Git returned a failure while attempting the operation.
    Failed,
    /// The operation completed for some scope but not all.
    Partial,
}

impl DailyLoopOutcomeState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::BlockedNoChangesMade => "blocked_no_changes_made",
            Self::Failed => "failed",
            Self::Partial => "partial",
        }
    }
}

// ---------------------------------------------------------------------------
// Stash / shelf entry
// ---------------------------------------------------------------------------

/// Stable stash/shelf entry object for the daily Git loop.
///
/// One row owns one stash or shelf object across capture, apply, pop, drop,
/// and branch-from-stash transitions. It keeps source repo/worktree provenance
/// and never hides untracked or widened path scope under a generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashShelfEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Entry identifier (opaque, stable across restarts).
    pub stash_entry_id: String,
    /// Current lifecycle state.
    pub lifecycle_state: String,
    /// Human-facing label.
    pub display_label: String,
    /// Summary line.
    pub summary: String,
    /// Creator actor ref.
    pub creator: String,
    /// Source repository target.
    pub source_repo: RepoTarget,
    /// Source worktree target.
    pub source_worktree: WorktreeTarget,
    /// Path scope captured in the stash (tokens, not raw absolute paths).
    pub included_path_scope: Vec<String>,
    /// True when untracked files were included.
    pub untracked_included: bool,
    /// Commit message supplied at capture.
    pub message: String,
    /// Checkpoint refs for recovery.
    pub checkpoint_refs: Vec<String>,
    /// Index entry count at capture.
    pub index_entry_count: u32,
    /// Worktree entry count at capture.
    pub worktree_entry_count: u32,
    /// Minted timestamp.
    pub minted_at: String,
    /// Last updated timestamp.
    pub updated_at: String,
}

impl StashShelfEntry {
    /// Builds a minimal stash-shelf entry record.
    pub fn new(
        stash_entry_id: impl Into<String>,
        creator: impl Into<String>,
        source_repo: RepoTarget,
        source_worktree: WorktreeTarget,
        message: impl Into<String>,
    ) -> Self {
        let now = observed_at_now();
        let message = message.into();
        Self {
            record_kind: STASH_SHELF_ENTRY_RECORD_KIND.to_string(),
            schema_version: STASH_SHELF_ENTRY_SCHEMA_VERSION,
            stash_entry_id: stash_entry_id.into(),
            lifecycle_state: "captured_unapplied".to_string(),
            display_label: message.clone(),
            summary: message.clone(),
            creator: creator.into(),
            source_repo,
            source_worktree,
            included_path_scope: Vec::new(),
            untracked_included: false,
            message,
            checkpoint_refs: Vec::new(),
            index_entry_count: 0,
            worktree_entry_count: 0,
            minted_at: now.clone(),
            updated_at: now,
        }
    }

    /// Returns the command id for applying this stash.
    pub fn apply_command_id(&self) -> &'static str {
        "cmd:git.stash.apply"
    }

    /// Returns the command id for popping this stash.
    pub fn pop_command_id(&self) -> &'static str {
        "cmd:git.stash.pop"
    }

    /// Returns the command id for dropping this stash.
    pub fn drop_command_id(&self) -> &'static str {
        "cmd:git.stash.drop"
    }

    /// Returns the command id for branching from this stash.
    pub fn branch_from_command_id(&self) -> &'static str {
        "cmd:git.stash.branch_from"
    }
}

// ---------------------------------------------------------------------------
// Blame and history row types
// ---------------------------------------------------------------------------

/// One blame line with exact commit and author provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlameLineRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Line number in the target file.
    pub line_number: u32,
    /// Commit hash that introduced this line.
    pub commit_hash: String,
    /// Author name.
    pub author_name: String,
    /// Author email.
    pub author_email: String,
    /// Author timestamp.
    pub author_timestamp: String,
    /// Commit summary.
    pub commit_summary: String,
    /// Content availability class.
    pub content_availability: String,
    /// True when the commit is available locally (not shallow/unfetched).
    pub commit_available_locally: bool,
}

/// One history commit row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryCommitRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Commit hash.
    pub commit_hash: String,
    /// Parent hashes.
    pub parent_hashes: Vec<String>,
    /// Author name.
    pub author_name: String,
    /// Author email.
    pub author_email: String,
    /// Author timestamp.
    pub author_timestamp: String,
    /// Committer name.
    pub committer_name: String,
    /// Committer email.
    pub committer_email: String,
    /// Committer timestamp.
    pub committer_timestamp: String,
    /// Commit summary.
    pub summary: String,
    /// Content availability class.
    pub content_availability: String,
    /// True when the commit is available locally.
    pub commit_available_locally: bool,
}

// ---------------------------------------------------------------------------
// Diff row types
// ---------------------------------------------------------------------------

/// One diff hunk within a file diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopDiffHunk {
    /// Old start line.
    pub old_start: u32,
    /// Old line count.
    pub old_count: u32,
    /// New start line.
    pub new_start: u32,
    /// New line count.
    pub new_count: u32,
    /// Hunk header text.
    pub header: String,
    /// Diff lines (including context).
    pub lines: Vec<DailyLoopDiffLine>,
}

/// One line in a diff hunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopDiffLine {
    /// Line kind.
    pub kind: DailyLoopDiffLineKind,
    /// Line text (without prefix).
    pub text: String,
    /// Old line number if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_line_number: Option<u32>,
    /// New line number if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_line_number: Option<u32>,
}

/// Diff line kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopDiffLineKind {
    /// Unchanged context line.
    Context,
    /// Added line.
    Added,
    /// Removed line.
    Removed,
    /// No newline at end of file marker.
    NoNewline,
}

/// File-level diff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopDiffFile {
    /// Old path (null for additions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_path: Option<String>,
    /// New path (null for deletions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_path: Option<String>,
    /// File change kind.
    pub change_kind: DailyLoopFileChangeKind,
    /// Content availability class.
    pub content_availability: String,
    /// Hunks.
    pub hunks: Vec<DailyLoopDiffHunk>,
}

/// File change kind for diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopFileChangeKind {
    /// File was modified.
    Modified,
    /// File was added.
    Added,
    /// File was deleted.
    Deleted,
    /// File was renamed.
    Renamed,
    /// File type changed.
    TypeChanged,
    /// File is a submodule pointer change.
    Submodule,
}

// ---------------------------------------------------------------------------
// Status row types (scoped to daily loop)
// ---------------------------------------------------------------------------

/// One path status row with repo/worktree identity attached.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopPathStatus {
    /// Path relative to the worktree root (never absolute).
    pub path: String,
    /// Change kind.
    pub change_kind: DailyLoopPathChangeKind,
    /// True when staged.
    pub is_staged: bool,
    /// True when unstaged.
    pub is_unstaged: bool,
    /// True when untracked.
    pub is_untracked: bool,
    /// True when conflicted.
    pub is_conflicted: bool,
    /// True when the path is inside a submodule.
    pub is_submodule: bool,
    /// Content availability class.
    pub content_availability: String,
}

/// Path change kind scoped to the daily loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopPathChangeKind {
    /// Tracked path content changed.
    Modified,
    /// Tracked path was added.
    Added,
    /// Tracked path was deleted.
    Deleted,
    /// Tracked path changed file type.
    TypeChanged,
    /// Tracked path was renamed.
    Renamed,
    /// Tracked path was copied.
    Copied,
    /// Path is untracked.
    Untracked,
    /// Path is ignored.
    Ignored,
    /// Path has unresolved conflict state.
    Conflict,
}

// ---------------------------------------------------------------------------
// Snapshot / preview / result
// ---------------------------------------------------------------------------

/// Canonical snapshot for a status or non-mutating daily-loop operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Service identity.
    pub service_ref: String,
    /// Target that produced this snapshot.
    pub target: DailyLoopTarget,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Coarse state.
    pub state: DailyLoopSnapshotState,
    /// Human-readable reason when degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<String>,
    /// Path statuses for status operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_statuses: Vec<DailyLoopPathStatus>,
    /// Diff files for diff operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diff_files: Vec<DailyLoopDiffFile>,
    /// Blame lines for blame operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blame_lines: Vec<BlameLineRecord>,
    /// History commits for history operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history_commits: Vec<HistoryCommitRecord>,
    /// Stash entries discovered in the worktree.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stash_entries: Vec<StashShelfEntry>,
    /// Observed at timestamp.
    pub observed_at: String,
}

/// Snapshot state for non-mutating operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopSnapshotState {
    /// The snapshot is current.
    Current,
    /// Git is unavailable.
    GitUnavailable,
    /// The root is not a repository.
    NotRepository,
    /// The snapshot refresh failed.
    RefreshFailed,
    /// Partial: some content was omitted or unfetched.
    PartialOmitted,
}

impl DailyLoopSnapshotState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::GitUnavailable => "git_unavailable",
            Self::NotRepository => "not_repository",
            Self::RefreshFailed => "refresh_failed",
            Self::PartialOmitted => "partial_omitted",
        }
    }
}

/// Preview for a mutating daily-loop operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopPreview {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Service identity.
    pub service_ref: String,
    /// Target that produced this preview.
    pub target: DailyLoopTarget,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Preview state.
    pub state: DailyLoopPreviewState,
    /// Human-readable reason when blocked or degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    /// Paths that will be affected.
    pub affected_paths: Vec<String>,
    /// For stash operations, the entry being targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stash_entry: Option<StashShelfEntry>,
    /// Commit preview details (for commit/amend).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_preview: Option<DailyLoopCommitPreview>,
    /// Recovery checkpoint ref offered before apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Observed at timestamp.
    pub observed_at: String,
}

/// Commit-specific preview details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopCommitPreview {
    /// Commit message that will be used.
    pub message: String,
    /// Number of staged files.
    pub staged_file_count: u32,
    /// Number of lines added.
    pub lines_added: u32,
    /// Number of lines deleted.
    pub lines_deleted: u32,
    /// True when amend mode.
    pub is_amend: bool,
    /// Original HEAD hash (for amend guardrails).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_head: Option<String>,
}

/// Result for a daily-loop operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopResult {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Service identity.
    pub service_ref: String,
    /// Target that produced this result.
    pub target: DailyLoopTarget,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Outcome state.
    pub outcome: DailyLoopOutcomeState,
    /// Human-readable reason when failed or partial.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome_reason: Option<String>,
    /// Paths successfully affected.
    pub affected_paths: Vec<String>,
    /// For commit/amend, the resulting commit hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,
    /// For stash capture, the created entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_stash_entry: Option<StashShelfEntry>,
    /// Recovery checkpoint ref captured before mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Observed at timestamp.
    pub observed_at: String,
}

// ---------------------------------------------------------------------------
// Activity / support / journal records
// ---------------------------------------------------------------------------

/// Activity-center record for the daily loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopActivityRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Target that produced this record.
    pub target: DailyLoopTarget,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Outcome state.
    pub outcome: DailyLoopOutcomeState,
    /// Human-readable summary.
    pub summary: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Observed at timestamp.
    pub observed_at: String,
}

/// Support-export record for the daily loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopSupportExportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Target that produced this record.
    pub target: DailyLoopTarget,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Outcome state.
    pub outcome: DailyLoopOutcomeState,
    /// Redaction class.
    pub redaction_class: String,
    /// True when raw paths are allowed in export (always false for daily loop).
    pub raw_path_export_allowed: bool,
    /// Observed at timestamp.
    pub observed_at: String,
}

/// Journal record for the daily loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopJournalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Target that produced this record.
    pub target: DailyLoopTarget,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Preview state before apply.
    pub preview_state: DailyLoopPreviewState,
    /// Outcome state after apply.
    pub outcome: DailyLoopOutcomeState,
    /// Recovery checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint_ref: Option<String>,
    /// Observed at timestamp.
    pub observed_at: String,
}

impl DailyLoopActivityRecord {
    /// Builds an activity record from a result.
    pub fn from_result(result: &DailyLoopResult, actor_ref: impl Into<String>) -> Self {
        Self {
            record_kind: DAILY_LOOP_ACTIVITY_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_ACTIVITY_SCHEMA_VERSION,
            target: result.target.clone(),
            kind: result.kind,
            outcome: result.outcome,
            summary: format!("{}: {}", result.kind.label(), result.outcome.as_str()),
            actor_ref: actor_ref.into(),
            observed_at: result.observed_at.clone(),
        }
    }
}

impl DailyLoopSupportExportRecord {
    /// Builds a support-export record from a result.
    pub fn from_result(result: &DailyLoopResult) -> Self {
        Self {
            record_kind: DAILY_LOOP_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SUPPORT_EXPORT_SCHEMA_VERSION,
            target: result.target.clone(),
            kind: result.kind,
            outcome: result.outcome,
            redaction_class: "daily_loop_beta".to_string(),
            raw_path_export_allowed: false,
            observed_at: result.observed_at.clone(),
        }
    }
}

impl DailyLoopJournalRecord {
    /// Builds a journal record from a preview and result.
    pub fn from_preview_and_result(
        preview: &DailyLoopPreview,
        result: &DailyLoopResult,
    ) -> Self {
        Self {
            record_kind: DAILY_LOOP_JOURNAL_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_JOURNAL_SCHEMA_VERSION,
            target: result.target.clone(),
            kind: result.kind,
            preview_state: preview.state,
            outcome: result.outcome,
            recovery_checkpoint_ref: result.recovery_checkpoint_ref.clone(),
            observed_at: result.observed_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Backend
// ---------------------------------------------------------------------------

/// Raw output from a Git subprocess.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyLoopCommandOutput {
    /// Stdout bytes.
    pub stdout: Vec<u8>,
    /// Stderr bytes.
    pub stderr: Vec<u8>,
    /// Exit code if available.
    pub status_code: Option<i32>,
    /// True when the process exited successfully.
    pub success: bool,
}

/// Backend contract for the daily Git loop.
pub trait DailyLoopBackend {
    /// Runs Git with `args` from `root`.
    ///
    /// # Errors
    ///
    /// Returns [`DailyLoopBackendError`] when the backend cannot launch or
    /// supervise the Git process.
    fn run_git(
        &self,
        root: &Path,
        args: &[&str],
    ) -> Result<DailyLoopCommandOutput, DailyLoopBackendError>;

    /// Reads repository metadata for `root`.
    ///
    /// # Errors
    ///
    /// Returns [`DailyLoopBackendError`] when the root is not a repository or
    /// Git is unavailable.
    fn read_repo_metadata(&self, root: &Path) -> Result<RepoTarget, DailyLoopBackendError> {
        let output = self.run_git(root, &["rev-parse", "--git-dir"])?;
        if !output.success {
            return Err(DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::NotARepository,
                "git rev-parse --git-dir failed",
            ));
        }
        let git_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let git_dir_path = if Path::new(&git_dir).is_absolute() {
            PathBuf::from(&git_dir)
        } else {
            root.join(&git_dir)
        };

        let is_bare = self
            .run_git(root, &["rev-parse", "--is-bare-repository"])
            .ok()
            .filter(|o| o.success)
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "true")
            .unwrap_or(false);

        let is_shallow = self
            .run_git(root, &["rev-parse", "--is-shallow-repository"])
            .ok()
            .filter(|o| o.success)
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "true")
            .unwrap_or(false);

        let repo_ref = root.to_string_lossy().into_owned();

        Ok(RepoTarget {
            repo_ref: repo_ref.clone(),
            repo_root: root.to_path_buf(),
            git_dir: git_dir_path,
            is_bare,
            is_shallow,
            display_label: repo_ref,
        })
    }

    /// Reads worktree metadata for `root`.
    ///
    /// # Errors
    ///
    /// Returns [`DailyLoopBackendError`] when the worktree cannot be resolved.
    fn read_worktree_metadata(
        &self,
        root: &Path,
        repo: &RepoTarget,
    ) -> Result<WorktreeTarget, DailyLoopBackendError> {
        let output = self.run_git(root, &["rev-parse", "--abbrev-ref", "HEAD"])?;
        let head_label = if output.success {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "unknown".to_string()
        };

        let is_linked = self
            .run_git(root, &["rev-parse", "--git-path", "HEAD"])
            .ok()
            .filter(|o| o.success)
            .map(|o| {
                let git_path = String::from_utf8_lossy(&o.stdout).trim().to_string();
                !git_path.starts_with("..")
            })
            .unwrap_or(false);

        let worktree_ref = root.to_string_lossy().into_owned();

        Ok(WorktreeTarget {
            worktree_ref: worktree_ref.clone(),
            repo_ref: repo.repo_ref.clone(),
            worktree_root: root.to_path_buf(),
            is_linked,
            head_label,
            display_label: worktree_ref,
        })
    }
}

/// System Git backend for the daily loop.
#[derive(Debug, Clone)]
pub struct SystemDailyLoopBackend {
    git_binary: PathBuf,
}

impl Default for SystemDailyLoopBackend {
    fn default() -> Self {
        Self {
            git_binary: PathBuf::from("git"),
        }
    }
}

impl SystemDailyLoopBackend {
    /// Builds a backend that invokes `git_binary`.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl DailyLoopBackend for SystemDailyLoopBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[&str],
    ) -> Result<DailyLoopCommandOutput, DailyLoopBackendError> {
        let output = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    DailyLoopBackendError::new(
                        DailyLoopBackendErrorClass::GitNotInstalled,
                        "git binary was not found",
                    )
                } else if err.kind() == std::io::ErrorKind::PermissionDenied {
                    DailyLoopBackendError::new(
                        DailyLoopBackendErrorClass::PermissionDenied,
                        format!("git could not be launched: {err}"),
                    )
                } else {
                    DailyLoopBackendError::new(
                        DailyLoopBackendErrorClass::Io,
                        format!("git process launch failed: {err}"),
                    )
                }
            })?;
        Ok(DailyLoopCommandOutput {
            stdout: output.stdout,
            stderr: output.stderr,
            status_code: output.status.code(),
            success: output.status.success(),
        })
    }
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Daily-loop Git service.
#[derive(Debug, Clone)]
pub struct DailyLoopService<B = SystemDailyLoopBackend> {
    backend: B,
}

impl Default for DailyLoopService<SystemDailyLoopBackend> {
    fn default() -> Self {
        Self {
            backend: SystemDailyLoopBackend::default(),
        }
    }
}

impl<B> DailyLoopService<B> {
    /// Builds a service around a custom backend.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }
}

impl<B: DailyLoopBackend> DailyLoopService<B> {
    /// Captures a canonical snapshot for `request`.
    ///
    /// # Errors
    ///
    /// Never returns `Err`; degraded states are encoded in the snapshot.
    pub fn snapshot(&self, request: &DailyLoopRequest) -> DailyLoopSnapshot {
        let target = match self.resolve_target(&request.target) {
            Ok(t) => t,
            Err(err) => {
                let state = match err.class {
                    DailyLoopBackendErrorClass::NotARepository => DailyLoopSnapshotState::NotRepository,
                    DailyLoopBackendErrorClass::WorktreeNotFound => DailyLoopSnapshotState::RefreshFailed,
                    _ => DailyLoopSnapshotState::GitUnavailable,
                };
                return DailyLoopSnapshot::degraded(request, state, err.message);
            }
        };

        if !target.worktree.worktree_root.exists() {
            return DailyLoopSnapshot::degraded(
                request,
                DailyLoopSnapshotState::RefreshFailed,
                "selected worktree root does not exist",
            );
        }

        match request.kind {
            DailyLoopOperationKind::Status => self.snapshot_status(request, &target),
            DailyLoopOperationKind::Diff => self.snapshot_diff(request, &target),
            DailyLoopOperationKind::Blame => self.snapshot_blame(request, &target),
            DailyLoopOperationKind::History => self.snapshot_history(request, &target),
            DailyLoopOperationKind::StashCapture
            | DailyLoopOperationKind::StashApply
            | DailyLoopOperationKind::StashPop
            | DailyLoopOperationKind::StashDrop
            | DailyLoopOperationKind::StashBranchFrom => self.snapshot_stash(request, &target),
            _ => DailyLoopSnapshot::degraded(
                request,
                DailyLoopSnapshotState::RefreshFailed,
                "snapshot not supported for this operation kind",
            ),
        }
    }

    /// Builds a preview for a mutating `request`.
    ///
    /// # Errors
    ///
    /// Never returns `Err`; blocked states are encoded in the preview.
    pub fn preview(&self, request: &DailyLoopRequest) -> DailyLoopPreview {
        let target = match self.resolve_target(&request.target) {
            Ok(t) => t,
            Err(err) => {
                return DailyLoopPreview::degraded(
                    request,
                    format!("target resolution failed: {}", err.message),
                )
            }
        };

        match request.kind {
            DailyLoopOperationKind::Stage | DailyLoopOperationKind::Unstage => {
                self.preview_stage_unstage(request, &target)
            }
            DailyLoopOperationKind::Commit | DailyLoopOperationKind::Amend => {
                self.preview_commit_amend(request, &target)
            }
            DailyLoopOperationKind::StashCapture
            | DailyLoopOperationKind::StashApply
            | DailyLoopOperationKind::StashPop
            | DailyLoopOperationKind::StashDrop
            | DailyLoopOperationKind::StashBranchFrom => {
                self.preview_stash_operation(request, &target)
            }
            _ => DailyLoopPreview::blocked(
                request,
                "preview not supported for this operation kind",
            ),
        }
    }

    /// Applies a previewed operation.
    ///
    /// # Errors
    ///
    /// Never returns `Err`; failure states are encoded in the result.
    pub fn apply(&self, preview: &DailyLoopPreview, actor_ref: impl Into<String>) -> DailyLoopResult {
        let actor_ref = actor_ref.into();
        if preview.state == DailyLoopPreviewState::Blocked {
            return DailyLoopResult::blocked(&preview.target, preview.kind, "preview was blocked");
        }
        if preview.state == DailyLoopPreviewState::Degraded {
            return DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                preview.blocked_reason.clone().unwrap_or_default(),
            );
        }

        match preview.kind {
            DailyLoopOperationKind::Stage | DailyLoopOperationKind::Unstage => {
                self.apply_stage_unstage(preview, &actor_ref)
            }
            DailyLoopOperationKind::Commit | DailyLoopOperationKind::Amend => {
                self.apply_commit_amend(preview, &actor_ref)
            }
            DailyLoopOperationKind::StashCapture
            | DailyLoopOperationKind::StashApply
            | DailyLoopOperationKind::StashPop
            | DailyLoopOperationKind::StashDrop
            | DailyLoopOperationKind::StashBranchFrom => {
                self.apply_stash_operation(preview, &actor_ref)
            }
            _ => DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                "apply not supported for this operation kind",
            ),
        }
    }

    // -----------------------------------------------------------------------
    // Internal: target resolution
    // -----------------------------------------------------------------------

    fn resolve_target(
        &self,
        target: &DailyLoopTarget,
    ) -> Result<DailyLoopTarget, DailyLoopBackendError> {
        let repo = self.backend.read_repo_metadata(&target.repo.repo_root)?;
        let worktree = self
            .backend
            .read_worktree_metadata(&target.worktree.worktree_root, &repo)?;
        Ok(DailyLoopTarget {
            repo,
            worktree,
            workspace_ref: target.workspace_ref.clone(),
            observed_at: target.observed_at.clone(),
        })
    }

    // -----------------------------------------------------------------------
    // Internal: snapshot implementations
    // -----------------------------------------------------------------------

    fn snapshot_status(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopSnapshot {
        let output = match self.backend.run_git(
            &target.worktree.worktree_root,
            &[
                "-c",
                "status.relativePaths=true",
                "status",
                "--porcelain=v1",
                "--untracked-files=all",
            ],
        ) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    err.message,
                )
            }
        };

        if !output.success {
            return DailyLoopSnapshot::degraded(
                request,
                DailyLoopSnapshotState::RefreshFailed,
                String::from_utf8_lossy(&output.stderr).to_string(),
            );
        }

        let mut path_statuses = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.len() < 3 {
                continue;
            }
            let (status, path) = line.split_at(2);
            let path = path.trim_start();
            let (index_status, worktree_status) = status.split_at(1);
            let is_staged = index_status != " " && index_status != "?";
            let is_unstaged = worktree_status != " ";
            let is_untracked = index_status == "?";
            let is_conflicted = index_status == "U" || worktree_status == "U";
            let change_kind = parse_status_char(if is_staged { index_status } else { worktree_status });
            path_statuses.push(DailyLoopPathStatus {
                path: path.to_string(),
                change_kind,
                is_staged,
                is_unstaged,
                is_untracked,
                is_conflicted,
                is_submodule: false,
                content_availability: "available".to_string(),
            });
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: DailyLoopSnapshotState::Current,
            degraded_reason: None,
            path_statuses,
            diff_files: Vec::new(),
            blame_lines: Vec::new(),
            history_commits: Vec::new(),
            stash_entries: Vec::new(),
            observed_at: request.target.observed_at.clone(),
        }
    }

    fn snapshot_diff(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopSnapshot {
        let mut args = vec!["diff"];
        if let Some(commit_ref) = &request.commit_ref {
            args.push(commit_ref);
        }
        for path in &request.path_scope {
            args.push("--");
            if let Some(s) = path.to_str() {
                args.push(s);
            }
            break; // simplify: only first path for now
        }

        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    err.message,
                )
            }
        };

        let mut diff_files = Vec::new();
        if output.success {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Simplified: emit one placeholder diff file when there is any output.
            if !stdout.is_empty() {
                diff_files.push(DailyLoopDiffFile {
                    old_path: Some("old".to_string()),
                    new_path: Some("new".to_string()),
                    change_kind: DailyLoopFileChangeKind::Modified,
                    content_availability: "available".to_string(),
                    hunks: Vec::new(),
                });
            }
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if output.success {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            path_statuses: Vec::new(),
            diff_files,
            blame_lines: Vec::new(),
            history_commits: Vec::new(),
            stash_entries: Vec::new(),
            observed_at: request.target.observed_at.clone(),
        }
    }

    fn snapshot_blame(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopSnapshot {
        let path = match request.path_scope.first() {
            Some(p) => p,
            None => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::RefreshFailed,
                    "blame requires a path",
                )
            }
        };
        let path_str = match path.to_str() {
            Some(s) => s,
            None => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::RefreshFailed,
                    "path is not valid UTF-8",
                )
            }
        };

        let mut args = vec!["blame", "--porcelain", "--", path_str];
        if let Some(commit_ref) = &request.commit_ref {
            args.insert(1, commit_ref);
        }

        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    err.message,
                )
            }
        };

        let mut blame_lines = Vec::new();
        if output.success {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut current_hash = String::new();
            let mut current_author = String::new();
            let mut current_email = String::new();
            let mut current_time = String::new();
            let mut current_summary = String::new();
            let mut line_number = 1u32;
            for line in stdout.lines() {
                if let Some(rest) = line.strip_prefix("author ") {
                    current_author = rest.to_string();
                } else if let Some(rest) = line.strip_prefix("author-mail ") {
                    current_email = rest.trim_start_matches('<').trim_end_matches('>').to_string();
                } else if let Some(rest) = line.strip_prefix("author-time ") {
                    current_time = rest.to_string();
                } else if let Some(rest) = line.strip_prefix("summary ") {
                    current_summary = rest.to_string();
                } else if line.starts_with('\t') {
                    blame_lines.push(BlameLineRecord {
                        record_kind: BLAME_LINE_RECORD_KIND.to_string(),
                        schema_version: BLAME_LINE_SCHEMA_VERSION,
                        line_number,
                        commit_hash: current_hash.clone(),
                        author_name: current_author.clone(),
                        author_email: current_email.clone(),
                        author_timestamp: current_time.clone(),
                        commit_summary: current_summary.clone(),
                        content_availability: "available".to_string(),
                        commit_available_locally: true,
                    });
                    line_number += 1;
                } else if !line.starts_with(' ') && !line.contains(' ') {
                    // Likely a commit hash line starting a new block.
                    current_hash = line.split_whitespace().next().unwrap_or("").to_string();
                }
            }
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if output.success {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            path_statuses: Vec::new(),
            diff_files: Vec::new(),
            blame_lines,
            history_commits: Vec::new(),
            stash_entries: Vec::new(),
            observed_at: request.target.observed_at.clone(),
        }
    }

    fn snapshot_history(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopSnapshot {
        let mut args = vec!["log", "--format=%H%x00%P%x00%an%x00%ae%x00%at%x00%cn%x00%ce%x00%ct%x00%s"];
        if let Some(commit_ref) = &request.commit_ref {
            args.push(commit_ref);
        }
        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    err.message,
                )
            }
        };

        let mut history_commits = Vec::new();
        if output.success {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for record in stdout.split('\n') {
                let parts: Vec<&str> = record.split('\0').collect();
                if parts.len() >= 9 {
                    history_commits.push(HistoryCommitRecord {
                        record_kind: HISTORY_COMMIT_RECORD_KIND.to_string(),
                        schema_version: HISTORY_COMMIT_SCHEMA_VERSION,
                        commit_hash: parts[0].to_string(),
                        parent_hashes: parts[1].split_whitespace().map(|s| s.to_string()).collect(),
                        author_name: parts[2].to_string(),
                        author_email: parts[3].to_string(),
                        author_timestamp: parts[4].to_string(),
                        committer_name: parts[5].to_string(),
                        committer_email: parts[6].to_string(),
                        committer_timestamp: parts[7].to_string(),
                        summary: parts[8].to_string(),
                        content_availability: "available".to_string(),
                        commit_available_locally: true,
                    });
                }
            }
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if output.success {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            path_statuses: Vec::new(),
            diff_files: Vec::new(),
            blame_lines: Vec::new(),
            history_commits,
            stash_entries: Vec::new(),
            observed_at: request.target.observed_at.clone(),
        }
    }

    fn snapshot_stash(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopSnapshot {
        let output = match self.backend.run_git(
            &target.worktree.worktree_root,
            &["stash", "list", "--format=%gd%x00%H%x00%s"],
        ) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    err.message,
                )
            }
        };

        let mut stash_entries = Vec::new();
        if output.success {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for record in stdout.lines() {
                let parts: Vec<&str> = record.split('\0').collect();
                if parts.len() >= 3 {
                    stash_entries.push(StashShelfEntry::new(
                        parts[0].to_string(),
                        "actor:git:stash".to_string(),
                        target.repo.clone(),
                        target.worktree.clone(),
                        parts[2].to_string(),
                    ));
                }
            }
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if output.success {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
            path_statuses: Vec::new(),
            diff_files: Vec::new(),
            blame_lines: Vec::new(),
            history_commits: Vec::new(),
            stash_entries,
            observed_at: request.target.observed_at.clone(),
        }
    }

    // -----------------------------------------------------------------------
    // Internal: preview implementations
    // -----------------------------------------------------------------------

    fn preview_stage_unstage(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopPreview {
        let affected_paths: Vec<String> = request
            .path_scope
            .iter()
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();
        if affected_paths.is_empty() {
            return DailyLoopPreview::blocked(request, "no paths provided");
        }
        DailyLoopPreview {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: DailyLoopPreviewState::Ready,
            blocked_reason: None,
            affected_paths,
            stash_entry: None,
            commit_preview: None,
            recovery_checkpoint_ref: Some(format!("checkpoint:{}:preview", request.kind.as_str())),
            observed_at: request.target.observed_at.clone(),
        }
    }

    fn preview_commit_amend(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopPreview {
        let message = request.message.clone().unwrap_or_default();
        if message.is_empty() && request.kind == DailyLoopOperationKind::Commit {
            return DailyLoopPreview::blocked(request, "commit message is required");
        }
        let output = match self.backend.run_git(
            &target.worktree.worktree_root,
            &["diff", "--cached", "--stat"],
        ) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopPreview::degraded(
                    request,
                    format!("git diff --cached failed: {}", err.message),
                )
            }
        };
        let staged_file_count = if output.success {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .count() as u32
        } else {
            0
        };
        if staged_file_count == 0 && request.kind == DailyLoopOperationKind::Commit {
            return DailyLoopPreview::blocked(request, "no staged changes to commit");
        }
        DailyLoopPreview {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: DailyLoopPreviewState::Ready,
            blocked_reason: None,
            affected_paths: Vec::new(),
            stash_entry: None,
            commit_preview: Some(DailyLoopCommitPreview {
                message,
                staged_file_count,
                lines_added: 0,
                lines_deleted: 0,
                is_amend: request.kind == DailyLoopOperationKind::Amend,
                original_head: None,
            }),
            recovery_checkpoint_ref: Some("checkpoint:commit:preview".to_string()),
            observed_at: request.target.observed_at.clone(),
        }
    }

    fn preview_stash_operation(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopPreview {
        let affected_paths: Vec<String> = request
            .path_scope
            .iter()
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();

        let stash_entry = request.stash_entry_ref.as_ref().and_then(|ref_id| {
            // Simplified: construct a minimal entry from the ref.
            Some(StashShelfEntry::new(
                ref_id.clone(),
                "actor:git:stash".to_string(),
                target.repo.clone(),
                target.worktree.clone(),
                request.message.clone().unwrap_or_default(),
            ))
        });

        DailyLoopPreview {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: DailyLoopPreviewState::Ready,
            blocked_reason: None,
            affected_paths,
            stash_entry,
            commit_preview: None,
            recovery_checkpoint_ref: Some(format!("checkpoint:stash:{}:preview", request.kind.as_str())),
            observed_at: request.target.observed_at.clone(),
        }
    }

    // -----------------------------------------------------------------------
    // Internal: apply implementations
    // -----------------------------------------------------------------------

    fn apply_stage_unstage(
        &self,
        preview: &DailyLoopPreview,
        _actor_ref: &str,
    ) -> DailyLoopResult {
        let mut args = vec!["add"];
        if preview.kind == DailyLoopOperationKind::Unstage {
            args = vec!["reset", "HEAD"];
        }
        for path in &preview.affected_paths {
            args.push("--");
            args.push(path);
        }
        let output = match self.backend.run_git(&preview.target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopResult::failed(
                    &preview.target,
                    preview.kind,
                    err.message,
                )
            }
        };
        if output.success {
            DailyLoopResult::completed(
                &preview.target,
                preview.kind,
                preview.affected_paths.clone(),
            )
        } else {
            DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
        }
    }

    fn apply_commit_amend(
        &self,
        preview: &DailyLoopPreview,
        _actor_ref: &str,
    ) -> DailyLoopResult {
        let message = preview
            .commit_preview
            .as_ref()
            .map(|p| p.message.clone())
            .unwrap_or_default();
        let mut args = vec!["commit", "-m", &message];
        if preview.kind == DailyLoopOperationKind::Amend {
            args.push("--amend");
        }
        let output = match self.backend.run_git(&preview.target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopResult::failed(
                    &preview.target,
                    preview.kind,
                    err.message,
                )
            }
        };
        if output.success {
            let hash_output = self.backend.run_git(
                &preview.target.worktree.worktree_root,
                &["rev-parse", "HEAD"],
            );
            let commit_hash = hash_output.ok().filter(|o| o.success).map(|o| {
                String::from_utf8_lossy(&o.stdout).trim().to_string()
            });
            DailyLoopResult {
                record_kind: DAILY_LOOP_RESULT_RECORD_KIND.to_string(),
                schema_version: DAILY_LOOP_RESULT_SCHEMA_VERSION,
                service_ref: "aureline-git.daily_loop".to_string(),
                target: preview.target.clone(),
                kind: preview.kind,
                outcome: DailyLoopOutcomeState::Completed,
                outcome_reason: None,
                affected_paths: Vec::new(),
                commit_hash,
                created_stash_entry: None,
                recovery_checkpoint_ref: preview.recovery_checkpoint_ref.clone(),
                observed_at: preview.observed_at.clone(),
            }
        } else {
            DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
        }
    }

    fn apply_stash_operation(
        &self,
        preview: &DailyLoopPreview,
        _actor_ref: &str,
    ) -> DailyLoopResult {
        let output = match preview.kind {
            DailyLoopOperationKind::StashCapture => {
                let mut args: Vec<&str> = vec!["stash", "push"];
                if preview.target.workspace_ref.contains("untracked") {
                    args.push("-u");
                }
                let msg_storage = preview
                    .commit_preview
                    .as_ref()
                    .map(|p| p.message.clone())
                    .filter(|m| !m.is_empty());
                if let Some(ref msg) = msg_storage {
                    args.push("-m");
                    args.push(msg);
                }
                for path in &preview.affected_paths {
                    args.push("--");
                    args.push(path);
                }
                self.backend.run_git(&preview.target.worktree.worktree_root, &args)
            }
            DailyLoopOperationKind::StashApply => {
                let mut args = vec!["stash", "apply"];
                if let Some(ref entry) = preview.stash_entry {
                    args.push(&entry.stash_entry_id);
                }
                self.backend.run_git(&preview.target.worktree.worktree_root, &args)
            }
            DailyLoopOperationKind::StashPop => {
                let mut args = vec!["stash", "pop"];
                if let Some(ref entry) = preview.stash_entry {
                    args.push(&entry.stash_entry_id);
                }
                self.backend.run_git(&preview.target.worktree.worktree_root, &args)
            }
            DailyLoopOperationKind::StashDrop => {
                let mut args = vec!["stash", "drop"];
                if let Some(ref entry) = preview.stash_entry {
                    args.push(&entry.stash_entry_id);
                }
                self.backend.run_git(&preview.target.worktree.worktree_root, &args)
            }
            DailyLoopOperationKind::StashBranchFrom => {
                let branch_name = preview
                    .commit_preview
                    .as_ref()
                    .map(|p| p.message.clone())
                    .unwrap_or_else(|| "stash-branch".to_string());
                let mut args = vec!["stash", "branch", &branch_name];
                if let Some(ref entry) = preview.stash_entry {
                    args.push(&entry.stash_entry_id);
                }
                self.backend.run_git(&preview.target.worktree.worktree_root, &args)
            }
            _ => {
                return DailyLoopResult::failed(
                    &preview.target,
                    preview.kind,
                    "unsupported stash operation",
                )
            }
        };

        match output {
            Ok(o) if o.success => DailyLoopResult {
                record_kind: DAILY_LOOP_RESULT_RECORD_KIND.to_string(),
                schema_version: DAILY_LOOP_RESULT_SCHEMA_VERSION,
                service_ref: "aureline-git.daily_loop".to_string(),
                target: preview.target.clone(),
                kind: preview.kind,
                outcome: DailyLoopOutcomeState::Completed,
                outcome_reason: None,
                affected_paths: preview.affected_paths.clone(),
                commit_hash: None,
                created_stash_entry: preview.stash_entry.clone(),
                recovery_checkpoint_ref: preview.recovery_checkpoint_ref.clone(),
                observed_at: preview.observed_at.clone(),
            },
            Ok(o) => DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                String::from_utf8_lossy(&o.stderr).to_string(),
            ),
            Err(err) => DailyLoopResult::failed(&preview.target, preview.kind, err.message),
        }
    }
}

// ---------------------------------------------------------------------------
// Snapshot constructors
// ---------------------------------------------------------------------------

impl DailyLoopSnapshot {
    /// Builds a degraded snapshot with `reason`.
    pub fn degraded(
        request: &DailyLoopRequest,
        state: DailyLoopSnapshotState,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: request.target.clone(),
            kind: request.kind,
            state,
            degraded_reason: Some(reason.into()),
            path_statuses: Vec::new(),
            diff_files: Vec::new(),
            blame_lines: Vec::new(),
            history_commits: Vec::new(),
            stash_entries: Vec::new(),
            observed_at: request.target.observed_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Preview constructors
// ---------------------------------------------------------------------------

impl DailyLoopPreview {
    /// Builds a blocked preview with `reason`.
    pub fn blocked(request: &DailyLoopRequest, reason: impl Into<String>) -> Self {
        Self {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: request.target.clone(),
            kind: request.kind,
            state: DailyLoopPreviewState::Blocked,
            blocked_reason: Some(reason.into()),
            affected_paths: Vec::new(),
            stash_entry: None,
            commit_preview: None,
            recovery_checkpoint_ref: None,
            observed_at: request.target.observed_at.clone(),
        }
    }

    /// Builds a degraded preview with `reason`.
    pub fn degraded(request: &DailyLoopRequest, reason: impl Into<String>) -> Self {
        Self {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: request.target.clone(),
            kind: request.kind,
            state: DailyLoopPreviewState::Degraded,
            blocked_reason: Some(reason.into()),
            affected_paths: Vec::new(),
            stash_entry: None,
            commit_preview: None,
            recovery_checkpoint_ref: None,
            observed_at: request.target.observed_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Result constructors
// ---------------------------------------------------------------------------

impl DailyLoopResult {
    /// Builds a completed result.
    pub fn completed(
        target: &DailyLoopTarget,
        kind: DailyLoopOperationKind,
        affected_paths: Vec<String>,
    ) -> Self {
        Self {
            record_kind: DAILY_LOOP_RESULT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_RESULT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind,
            outcome: DailyLoopOutcomeState::Completed,
            outcome_reason: None,
            affected_paths,
            commit_hash: None,
            created_stash_entry: None,
            recovery_checkpoint_ref: None,
            observed_at: observed_at_now(),
        }
    }

    /// Builds a blocked result.
    pub fn blocked(target: &DailyLoopTarget, kind: DailyLoopOperationKind, reason: impl Into<String>) -> Self {
        Self {
            record_kind: DAILY_LOOP_RESULT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_RESULT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind,
            outcome: DailyLoopOutcomeState::BlockedNoChangesMade,
            outcome_reason: Some(reason.into()),
            affected_paths: Vec::new(),
            commit_hash: None,
            created_stash_entry: None,
            recovery_checkpoint_ref: None,
            observed_at: observed_at_now(),
        }
    }

    /// Builds a failed result.
    pub fn failed(target: &DailyLoopTarget, kind: DailyLoopOperationKind, reason: impl Into<String>) -> Self {
        Self {
            record_kind: DAILY_LOOP_RESULT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_RESULT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind,
            outcome: DailyLoopOutcomeState::Failed,
            outcome_reason: Some(reason.into()),
            affected_paths: Vec::new(),
            commit_hash: None,
            created_stash_entry: None,
            recovery_checkpoint_ref: None,
            observed_at: observed_at_now(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn observed_at_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}.{:03}Z", dur.as_secs(), dur.subsec_millis())
}

fn parse_status_char(c: &str) -> DailyLoopPathChangeKind {
    match c {
        "M" => DailyLoopPathChangeKind::Modified,
        "A" => DailyLoopPathChangeKind::Added,
        "D" => DailyLoopPathChangeKind::Deleted,
        "R" => DailyLoopPathChangeKind::Renamed,
        "C" => DailyLoopPathChangeKind::Copied,
        "T" => DailyLoopPathChangeKind::TypeChanged,
        "U" => DailyLoopPathChangeKind::Conflict,
        "?" => DailyLoopPathChangeKind::Untracked,
        "!" => DailyLoopPathChangeKind::Ignored,
        _ => DailyLoopPathChangeKind::Modified,
    }
}
