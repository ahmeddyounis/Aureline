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
//! The redacted support-export boundary schema lives at
//! `schemas/git/daily_loop_support_export.schema.json`.
//! Canonical fixtures live under `fixtures/git/m4/daily_loop_beta/`.

use std::collections::HashSet;
use std::fmt;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::commit::{
    GitCommitActorRef, GitCommitMode, GitCommitOutcomeState, GitCommitPreview,
    GitCommitPreviewState, GitCommitRequest, GitCommitService,
};
use crate::mutations::{
    GitMutationActorRef, GitMutationOperationKind, GitMutationOutcomeState, GitMutationPreview,
    GitMutationPreviewState, GitMutationRequest, GitMutationService,
};
use crate::{digest, hardened_git};

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
const DAILY_LOOP_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 2;
const DAILY_LOOP_JOURNAL_SCHEMA_VERSION: u32 = 1;
const STASH_SHELF_ENTRY_SCHEMA_VERSION: u32 = 1;
const BLAME_LINE_SCHEMA_VERSION: u32 = 1;
const HISTORY_COMMIT_SCHEMA_VERSION: u32 = 1;

const MAX_DAILY_LOOP_PATHS: usize = 4096;
const MAX_DAILY_LOOP_PATH_BYTES: usize = 4096;
const MAX_DAILY_LOOP_SCOPE_BYTES: usize = 1024 * 1024;
const MAX_DAILY_LOOP_REF_BYTES: usize = 512;
const MAX_DAILY_LOOP_ROWS: usize = 4096;

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
pub const DAILY_LOOP_PREVIEW_STATES: &[&str] = &["ready", "blocked", "degraded"];

/// Outcome states for the daily loop.
pub const DAILY_LOOP_OUTCOME_STATES: &[&str] =
    &["completed", "blocked_no_changes_made", "failed", "partial"];

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
        let requested_root = worktree_root.into();
        let worktree_root = std::fs::canonicalize(&requested_root).unwrap_or(requested_root);
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
    /// In-memory, single-process apply authority. It is never serialized into
    /// preview, support, or handoff records.
    #[serde(skip)]
    apply_authority: DailyLoopApplyAuthority,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DailyLoopApplyAuthority {
    projection_digest: Option<String>,
    actor_ref: Option<String>,
    mutation_preview: Option<GitMutationPreview>,
    commit_preview: Option<GitCommitPreview>,
}

impl DailyLoopPreview {
    fn seal(mut self) -> Self {
        self.apply_authority.projection_digest = serde_json::to_vec(&self)
            .ok()
            .filter(|bytes| bytes.len() <= MAX_DAILY_LOOP_SCOPE_BYTES * 2)
            .map(|bytes| digest::sha256_token(&bytes));
        if self.state == DailyLoopPreviewState::Ready
            && self.apply_authority.projection_digest.is_none()
        {
            self.state = DailyLoopPreviewState::Blocked;
            self.blocked_reason =
                Some("daily-loop preview exceeds the in-process authority boundary".to_string());
            self.apply_authority.actor_ref = None;
            self.apply_authority.mutation_preview = None;
            self.apply_authority.commit_preview = None;
        }
        self
    }

    fn projection_matches_authority(&self) -> bool {
        let Some(expected) = self.apply_authority.projection_digest.as_deref() else {
            return false;
        };
        serde_json::to_vec(self)
            .ok()
            .filter(|bytes| bytes.len() <= MAX_DAILY_LOOP_SCOPE_BYTES * 2)
            .map(|bytes| digest::sha256_token(&bytes))
            .as_deref()
            == Some(expected)
    }
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

/// Redacted target identity carried by daily-loop support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopSupportTargetProjection {
    /// Domain-separated digest of the caller's workspace ref.
    pub workspace_ref_digest: String,
    /// Domain-separated digest of the repository ref.
    pub repo_ref_digest: String,
    /// Domain-separated digest of the worktree ref.
    pub worktree_ref_digest: String,
    /// Repository topology class without any filesystem location.
    pub repository_class: String,
    /// Whether the repository is shallow.
    pub is_shallow: bool,
    /// Whether this is a linked worktree.
    pub is_linked_worktree: bool,
    /// Attached/detached/unavailable HEAD class without a branch name.
    pub head_state_class: String,
}

/// Support-export record for the daily loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopSupportExportRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Redacted target projection. Raw target paths and display refs are never
    /// embedded in this record family.
    pub target: DailyLoopSupportTargetProjection,
    /// Operation kind.
    pub kind: DailyLoopOperationKind,
    /// Outcome state.
    pub outcome: DailyLoopOutcomeState,
    /// Redaction class.
    pub redaction_class: String,
    /// Redaction profile applied to target and path fields.
    pub redaction_profile_ref: String,
    /// True when raw paths are allowed in export (always false for daily loop).
    pub raw_path_export_allowed: bool,
    /// True when raw branch/ref labels are allowed (always false).
    pub raw_ref_name_export_allowed: bool,
    /// Number of affected path rows represented without their path bodies.
    pub affected_path_count: usize,
    /// Explicitly omitted field families so absence cannot look like missing
    /// collection.
    pub omitted_fields: Vec<String>,
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
            target: DailyLoopSupportTargetProjection::from_target(&result.target),
            kind: result.kind,
            outcome: result.outcome,
            redaction_class: "metadata_safe_default".to_string(),
            redaction_profile_ref: "support.redaction.local_first_default".to_string(),
            raw_path_export_allowed: false,
            raw_ref_name_export_allowed: false,
            affected_path_count: result.affected_paths.len(),
            omitted_fields: vec![
                "target.workspace_ref".to_string(),
                "target.repo.repo_ref".to_string(),
                "target.repo.repo_root".to_string(),
                "target.repo.git_dir".to_string(),
                "target.repo.display_label".to_string(),
                "target.worktree.repo_ref".to_string(),
                "target.worktree.worktree_ref".to_string(),
                "target.worktree.worktree_root".to_string(),
                "target.worktree.head_label".to_string(),
                "target.worktree.display_label".to_string(),
                "affected_paths".to_string(),
                "outcome_reason".to_string(),
            ],
            observed_at: support_safe_timestamp(&result.observed_at),
        }
    }
}

impl DailyLoopSupportTargetProjection {
    fn from_target(target: &DailyLoopTarget) -> Self {
        Self {
            workspace_ref_digest: redacted_support_digest("workspace_ref", &target.workspace_ref),
            repo_ref_digest: redacted_support_digest("repo_ref", &target.repo.repo_ref),
            worktree_ref_digest: redacted_support_digest(
                "worktree_ref",
                &target.worktree.worktree_ref,
            ),
            repository_class: if target.repo.is_bare {
                "bare_repository"
            } else {
                "worktree_repository"
            }
            .to_string(),
            is_shallow: target.repo.is_shallow,
            is_linked_worktree: target.worktree.is_linked,
            head_state_class: match target.worktree.head_label.as_str() {
                "unknown" | "" => "unavailable",
                "HEAD" => "detached",
                _ => "attached",
            }
            .to_string(),
        }
    }
}

impl DailyLoopJournalRecord {
    /// Builds a journal record from a preview and result.
    pub fn from_preview_and_result(preview: &DailyLoopPreview, result: &DailyLoopResult) -> Self {
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
        let output = self.run_git(root, &["rev-parse", "--absolute-git-dir"])?;
        if !output.success {
            return Err(DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::NotARepository,
                "git repository identity could not be resolved",
            ));
        }
        let git_dir_path = parse_absolute_git_path(&output.stdout).ok_or_else(|| {
            DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::NotARepository,
                "git directory identity was not an absolute path",
            )
        })?;

        let is_bare = read_git_bool(self, root, &["rev-parse", "--is-bare-repository"])?;
        let is_shallow = read_git_bool(self, root, &["rev-parse", "--is-shallow-repository"])?;

        let repo_root = if is_bare {
            root.to_path_buf()
        } else {
            let output = self.run_git(root, &["rev-parse", "--show-toplevel"])?;
            if !output.success {
                return Err(DailyLoopBackendError::new(
                    DailyLoopBackendErrorClass::NotARepository,
                    "git worktree root could not be resolved",
                ));
            }
            parse_absolute_git_path(&output.stdout).ok_or_else(|| {
                DailyLoopBackendError::new(
                    DailyLoopBackendErrorClass::NotARepository,
                    "git worktree root was not an absolute path",
                )
            })?
        };
        let repo_ref = repo_root.to_string_lossy().into_owned();
        let display_label = repo_root.display().to_string();

        Ok(RepoTarget {
            repo_ref,
            repo_root,
            git_dir: git_dir_path,
            is_bare,
            is_shallow,
            display_label,
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
            parse_bounded_git_label(&output.stdout).unwrap_or_else(|| "unknown".to_string())
        } else {
            "unknown".to_string()
        };

        let output = self.run_git(root, &["rev-parse", "--show-toplevel"])?;
        if !output.success {
            return Err(DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::WorktreeNotFound,
                "git worktree root could not be resolved",
            ));
        }
        let worktree_root = parse_absolute_git_path(&output.stdout).ok_or_else(|| {
            DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::WorktreeNotFound,
                "git worktree root was not an absolute path",
            )
        })?;
        let common_dir = self.run_git(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?;
        if !common_dir.success {
            return Err(DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::WorktreeNotFound,
                "git common repository identity could not be resolved",
            ));
        }
        let common_dir = parse_absolute_git_path(&common_dir.stdout).ok_or_else(|| {
            DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::WorktreeNotFound,
                "git common repository identity was not an absolute path",
            )
        })?;
        let git_dir = std::fs::canonicalize(&repo.git_dir).unwrap_or_else(|_| repo.git_dir.clone());
        let common_dir = std::fs::canonicalize(&common_dir).unwrap_or(common_dir);
        let is_linked = git_dir != common_dir;
        let worktree_ref = worktree_root.to_string_lossy().into_owned();
        let display_label = worktree_root.display().to_string();

        Ok(WorktreeTarget {
            worktree_ref,
            repo_ref: repo.repo_ref.clone(),
            worktree_root,
            is_linked,
            head_label,
            display_label,
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
        let args = args
            .iter()
            .map(|arg| (*arg).to_string())
            .collect::<Vec<_>>();
        let output = hardened_git::run(hardened_git::command(&self.git_binary, root, &args))
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    DailyLoopBackendError::new(
                        DailyLoopBackendErrorClass::GitNotInstalled,
                        "git binary was not found",
                    )
                } else if err.kind() == std::io::ErrorKind::PermissionDenied {
                    DailyLoopBackendError::new(
                        DailyLoopBackendErrorClass::PermissionDenied,
                        "git binary could not be launched under the safe execution profile",
                    )
                } else {
                    DailyLoopBackendError::new(
                        DailyLoopBackendErrorClass::Io,
                        "git command exceeded a safe process, time, input, or output boundary",
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
    commit_service: GitCommitService,
}

impl Default for DailyLoopService<SystemDailyLoopBackend> {
    fn default() -> Self {
        Self {
            backend: SystemDailyLoopBackend::default(),
            commit_service: GitCommitService::default(),
        }
    }
}

impl<B> DailyLoopService<B> {
    /// Builds a service around a custom backend.
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            commit_service: GitCommitService::default(),
        }
    }
}

impl<B: DailyLoopBackend> DailyLoopService<B> {
    /// Captures a canonical snapshot for `request`.
    ///
    /// # Errors
    ///
    /// Never returns `Err`; degraded states are encoded in the snapshot.
    pub fn snapshot(&self, request: &DailyLoopRequest) -> DailyLoopSnapshot {
        if let Err(reason) = validate_daily_request(request) {
            return DailyLoopSnapshot::degraded(
                request,
                DailyLoopSnapshotState::RefreshFailed,
                reason,
            );
        }
        let target = match self.resolve_target(&request.target) {
            Ok(t) => t,
            Err(err) => {
                let state = match err.class {
                    DailyLoopBackendErrorClass::NotARepository => {
                        DailyLoopSnapshotState::NotRepository
                    }
                    DailyLoopBackendErrorClass::WorktreeNotFound => {
                        DailyLoopSnapshotState::RefreshFailed
                    }
                    _ => DailyLoopSnapshotState::GitUnavailable,
                };
                return DailyLoopSnapshot::degraded(request, state, backend_failure_reason(&err));
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
        if let Err(reason) = validate_daily_request(request) {
            return DailyLoopPreview::blocked(request, reason);
        }
        let target = match self.resolve_target(&request.target) {
            Ok(t) => t,
            Err(err) => {
                return DailyLoopPreview::degraded(
                    request,
                    format!("target resolution failed: {}", backend_failure_reason(&err)),
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
            _ => {
                DailyLoopPreview::blocked(request, "preview not supported for this operation kind")
            }
        }
    }

    /// Applies a previewed operation.
    ///
    /// # Errors
    ///
    /// Never returns `Err`; failure states are encoded in the result.
    pub fn apply(
        &self,
        preview: &DailyLoopPreview,
        actor_ref: impl Into<String>,
    ) -> DailyLoopResult {
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
        if !preview.projection_matches_authority() {
            return DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                "preview authority is unavailable or changed; reopen review",
            );
        }
        if preview.apply_authority.actor_ref.as_deref() != Some(actor_ref.as_str()) {
            return DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                "apply actor does not match the reviewed preview actor; reopen review",
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
            | DailyLoopOperationKind::StashBranchFrom => DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                "stash mutation remains inspect-only in the daily-loop adapter",
            ),
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
        let declared_common_git_dir = self.common_git_dir(&target.repo.repo_root)?;
        let worktree_common_git_dir = self.common_git_dir(&target.worktree.worktree_root)?;
        if declared_common_git_dir != worktree_common_git_dir {
            return Err(DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::WorktreeNotFound,
                "declared repository and worktree identities do not match",
            ));
        }
        let declared_repo = self.backend.read_repo_metadata(&target.repo.repo_root)?;
        let worktree_repo = self
            .backend
            .read_repo_metadata(&target.worktree.worktree_root)?;
        let mut worktree = self
            .backend
            .read_worktree_metadata(&target.worktree.worktree_root, &worktree_repo)?;
        worktree.repo_ref.clone_from(&declared_repo.repo_ref);
        Ok(DailyLoopTarget {
            repo: declared_repo,
            worktree,
            workspace_ref: target.workspace_ref.clone(),
            observed_at: target.observed_at.clone(),
        })
    }

    fn common_git_dir(&self, root: &Path) -> Result<PathBuf, DailyLoopBackendError> {
        let output = self.backend.run_git(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?;
        if !output.success {
            return Err(DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::NotARepository,
                "git common repository identity could not be resolved",
            ));
        }
        let common_git_dir = parse_absolute_git_path(&output.stdout).ok_or_else(|| {
            DailyLoopBackendError::new(
                DailyLoopBackendErrorClass::NotARepository,
                "git common repository identity was not an absolute path",
            )
        })?;
        Ok(std::fs::canonicalize(&common_git_dir).unwrap_or(common_git_dir))
    }

    // -----------------------------------------------------------------------
    // Internal: snapshot implementations
    // -----------------------------------------------------------------------

    fn snapshot_status(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopSnapshot {
        let mut args = vec![
            "-c",
            "status.relativePaths=true",
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
        ];
        let scoped_paths = request
            .path_scope
            .iter()
            .filter_map(|path| path.to_str())
            .collect::<Vec<_>>();
        if !scoped_paths.is_empty() {
            args.push("--");
            args.extend(scoped_paths);
        }
        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    backend_failure_reason(&err),
                )
            }
        };

        if !output.success {
            return DailyLoopSnapshot::degraded(
                request,
                DailyLoopSnapshotState::RefreshFailed,
                daily_loop_failure_reason(&output),
            );
        }

        let path_statuses = match parse_porcelain_status(&output.stdout) {
            Ok(rows) => rows,
            Err(reason) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::PartialOmitted,
                    reason,
                )
            }
        };

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
        let mut args = vec!["diff", "--quiet", "--no-ext-diff", "--no-textconv"];
        if let Some(commit_ref) = &request.commit_ref {
            args.push(commit_ref);
        }
        let scoped_paths = request
            .path_scope
            .iter()
            .filter_map(|path| path.to_str())
            .collect::<Vec<_>>();
        if !scoped_paths.is_empty() {
            args.push("--");
            args.extend(scoped_paths);
        }

        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    backend_failure_reason(&err),
                )
            }
        };

        let (state, degraded_reason) = match (output.success, output.status_code) {
            (true, _) => (DailyLoopSnapshotState::Current, None),
            (false, Some(1)) => (
                DailyLoopSnapshotState::PartialOmitted,
                Some(
                    "diff exists, but structured file/hunk rows are unavailable in this bounded adapter"
                        .to_string(),
                ),
            ),
            (false, _) => (
                DailyLoopSnapshotState::RefreshFailed,
                Some(daily_loop_failure_reason(&output)),
            ),
        };

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state,
            degraded_reason,
            path_statuses: Vec::new(),
            diff_files: Vec::new(),
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
        if request.path_scope.len() != 1 {
            return DailyLoopSnapshot::degraded(
                request,
                DailyLoopSnapshotState::RefreshFailed,
                "blame requires exactly one path",
            );
        }
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

        let mut args = vec!["blame", "--line-porcelain"];
        if let Some(line_range) = &request.line_range {
            args.extend(["-L", line_range]);
        } else {
            args.extend(["-L", "1,4097"]);
        }
        if let Some(commit_ref) = &request.commit_ref {
            args.push(commit_ref);
        }
        args.extend(["--", path_str]);

        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    backend_failure_reason(&err),
                )
            }
        };

        let mut blame_lines = Vec::new();
        let mut rows_omitted = false;
        if output.success {
            let Some(stdout) = std::str::from_utf8(&output.stdout).ok() else {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::PartialOmitted,
                    "blame output is not valid UTF-8 for this adapter",
                );
            };
            let mut current_hash: Option<String> = None;
            let mut current_author = None;
            let mut current_email = None;
            let mut current_time = None;
            let mut current_summary = None;
            let mut line_number = 1u32;
            for line in stdout.lines() {
                if let Some(rest) = line.strip_prefix("author ") {
                    if !bounded_record_text(rest, MAX_DAILY_LOOP_REF_BYTES) {
                        rows_omitted = true;
                        break;
                    }
                    current_author = Some(rest.to_string());
                } else if let Some(rest) = line.strip_prefix("author-mail ") {
                    let email = rest.trim_start_matches('<').trim_end_matches('>');
                    if !bounded_record_text(email, MAX_DAILY_LOOP_REF_BYTES) {
                        rows_omitted = true;
                        break;
                    }
                    current_email = Some(email.to_string());
                } else if let Some(rest) = line.strip_prefix("author-time ") {
                    if !valid_git_epoch(rest) {
                        rows_omitted = true;
                        break;
                    }
                    current_time = Some(rest.to_string());
                } else if let Some(rest) = line.strip_prefix("summary ") {
                    if !bounded_record_text(rest, MAX_DAILY_LOOP_PATH_BYTES) {
                        rows_omitted = true;
                        break;
                    }
                    current_summary = Some(rest.to_string());
                } else if line.starts_with('\t') {
                    if blame_lines.len() >= MAX_DAILY_LOOP_ROWS {
                        rows_omitted = true;
                        break;
                    }
                    let (
                        Some(commit_hash),
                        Some(author_name),
                        Some(author_email),
                        Some(author_time),
                        Some(summary),
                    ) = (
                        current_hash.as_ref(),
                        current_author.as_ref(),
                        current_email.as_ref(),
                        current_time.as_ref(),
                        current_summary.as_ref(),
                    )
                    else {
                        rows_omitted = true;
                        break;
                    };
                    blame_lines.push(BlameLineRecord {
                        record_kind: BLAME_LINE_RECORD_KIND.to_string(),
                        schema_version: BLAME_LINE_SCHEMA_VERSION,
                        line_number,
                        commit_hash: commit_hash.clone(),
                        author_name: author_name.clone(),
                        author_email: author_email.clone(),
                        author_timestamp: author_time.clone(),
                        commit_summary: summary.clone(),
                        content_availability: "available".to_string(),
                        commit_available_locally: true,
                    });
                    let Some(next_line_number) = line_number.checked_add(1) else {
                        rows_omitted = true;
                        break;
                    };
                    line_number = next_line_number;
                    current_hash = None;
                    current_author = None;
                    current_email = None;
                    current_time = None;
                    current_summary = None;
                } else if let Some((hash, final_line)) = blame_header(line) {
                    current_hash = Some(hash.to_string());
                    current_author = None;
                    current_email = None;
                    current_time = None;
                    current_summary = None;
                    line_number = final_line;
                }
            }
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success && rows_omitted {
                DailyLoopSnapshotState::PartialOmitted
            } else if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if rows_omitted {
                Some("blame rows were malformed or exceeded the bounded adapter window".to_string())
            } else if output.success {
                None
            } else {
                Some(daily_loop_failure_reason(&output))
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
        let mut args = vec![
            "log",
            "--max-count=4097",
            "--format=%H%x00%P%x00%an%x00%ae%x00%at%x00%cn%x00%ce%x00%ct%x00%s",
        ];
        if let Some(commit_ref) = &request.commit_ref {
            args.push(commit_ref);
        }
        let scoped_paths = request
            .path_scope
            .iter()
            .filter_map(|path| path.to_str())
            .collect::<Vec<_>>();
        if !scoped_paths.is_empty() {
            args.push("--");
            args.extend(scoped_paths);
        }
        let output = match self.backend.run_git(&target.worktree.worktree_root, &args) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    backend_failure_reason(&err),
                )
            }
        };

        let mut history_commits = Vec::new();
        let mut rows_omitted = false;
        if output.success {
            let Some(stdout) = std::str::from_utf8(&output.stdout).ok() else {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::PartialOmitted,
                    "history output is not valid UTF-8 for this adapter",
                );
            };
            for record in stdout.split('\n') {
                if record.is_empty() {
                    continue;
                }
                let parts = record.splitn(10, '\0').collect::<Vec<_>>();
                if parts.len() != 9 {
                    rows_omitted = true;
                    continue;
                }
                let parents = parts[1].split_whitespace().take(33).collect::<Vec<_>>();
                let valid = valid_git_oid(parts[0])
                    && parents.len() <= 32
                    && parents.iter().all(|parent| valid_git_oid(parent))
                    && bounded_record_text(parts[2], MAX_DAILY_LOOP_REF_BYTES)
                    && bounded_record_text(parts[3], MAX_DAILY_LOOP_REF_BYTES)
                    && valid_git_epoch(parts[4])
                    && bounded_record_text(parts[5], MAX_DAILY_LOOP_REF_BYTES)
                    && bounded_record_text(parts[6], MAX_DAILY_LOOP_REF_BYTES)
                    && valid_git_epoch(parts[7])
                    && bounded_record_text(parts[8], MAX_DAILY_LOOP_PATH_BYTES);
                if !valid {
                    rows_omitted = true;
                    continue;
                }
                if history_commits.len() >= MAX_DAILY_LOOP_ROWS {
                    rows_omitted = true;
                    break;
                }
                history_commits.push(HistoryCommitRecord {
                    record_kind: HISTORY_COMMIT_RECORD_KIND.to_string(),
                    schema_version: HISTORY_COMMIT_SCHEMA_VERSION,
                    commit_hash: parts[0].to_string(),
                    parent_hashes: parents.iter().map(|parent| (*parent).to_string()).collect(),
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

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success && rows_omitted {
                DailyLoopSnapshotState::PartialOmitted
            } else if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if rows_omitted {
                Some(
                    "history rows were malformed or exceeded the bounded adapter window"
                        .to_string(),
                )
            } else if output.success {
                None
            } else {
                Some(daily_loop_failure_reason(&output))
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
            &[
                "stash",
                "list",
                "--max-count=4097",
                "--format=%gd%x00%H%x00%ct%x00%s",
            ],
        ) {
            Ok(o) => o,
            Err(err) => {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::GitUnavailable,
                    backend_failure_reason(&err),
                )
            }
        };

        let mut stash_entries = Vec::new();
        let mut rows_omitted = false;
        if output.success {
            let Some(stdout) = std::str::from_utf8(&output.stdout).ok() else {
                return DailyLoopSnapshot::degraded(
                    request,
                    DailyLoopSnapshotState::PartialOmitted,
                    "stash output is not valid UTF-8 for this adapter",
                );
            };
            for record in stdout.lines() {
                let parts = record.splitn(5, '\0').collect::<Vec<_>>();
                if parts.len() != 4 {
                    rows_omitted = true;
                    continue;
                }
                if !valid_git_ref_input(parts[0])
                    || !valid_git_oid(parts[1])
                    || !valid_git_epoch(parts[2])
                    || !bounded_record_text(parts[3], 64 * 1024)
                {
                    rows_omitted = true;
                    continue;
                }
                if stash_entries.len() >= MAX_DAILY_LOOP_ROWS {
                    rows_omitted = true;
                    break;
                }
                let mut entry = StashShelfEntry::new(
                    format!("git.stash.object.{}", parts[1]),
                    "actor:git:stash".to_string(),
                    target.repo.clone(),
                    target.worktree.clone(),
                    parts[3].to_string(),
                );
                entry.minted_at = format!("{}Z", parts[2]);
                entry.updated_at.clone_from(&entry.minted_at);
                stash_entries.push(entry);
            }
        }

        DailyLoopSnapshot {
            record_kind: DAILY_LOOP_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state: if output.success && rows_omitted {
                DailyLoopSnapshotState::PartialOmitted
            } else if output.success {
                DailyLoopSnapshotState::Current
            } else {
                DailyLoopSnapshotState::RefreshFailed
            },
            degraded_reason: if rows_omitted {
                Some("stash rows were malformed or exceeded the bounded adapter window".to_string())
            } else if output.success {
                None
            } else {
                Some(daily_loop_failure_reason(&output))
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
        if request.path_scope.is_empty() {
            return DailyLoopPreview::blocked(request, "no paths provided");
        }
        let operation = if request.kind == DailyLoopOperationKind::Stage {
            GitMutationOperationKind::Stage
        } else {
            GitMutationOperationKind::Unstage
        };
        let mut mutation_request = GitMutationRequest::with_observed_at(
            request.target.workspace_ref.clone(),
            target.worktree.worktree_root.clone(),
            operation,
            request.path_scope.clone(),
            request.target.observed_at.clone(),
        )
        .with_launch_source_ref(request.caller_command_id.clone());
        mutation_request.actor = GitMutationActorRef {
            actor_class: "local_user".to_string(),
            display_label: "Local Git actor".to_string(),
            stable_id: Some(request.actor_ref.clone()),
        };
        let mutation_preview = GitMutationService::default().preview(&mutation_request);
        let state = match mutation_preview.preview_state {
            GitMutationPreviewState::ReadyToApply => DailyLoopPreviewState::Ready,
            GitMutationPreviewState::Blocked => DailyLoopPreviewState::Blocked,
            GitMutationPreviewState::Degraded => DailyLoopPreviewState::Degraded,
        };
        let affected_paths = mutation_preview
            .scope
            .targets
            .iter()
            .map(|row| row.repo_relative_path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let blocked_reason = if state == DailyLoopPreviewState::Ready {
            None
        } else {
            mutation_preview
                .scope
                .targets
                .iter()
                .find_map(|row| row.blocked_reason.clone())
                .or_else(|| Some("Git mutation preview is not ready".to_string()))
        };
        let recovery_checkpoint_ref = mutation_preview
            .checkpoint
            .checkpoint_captured
            .then(|| mutation_preview.checkpoint.checkpoint_ref.clone());
        let mut preview = DailyLoopPreview {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state,
            blocked_reason,
            affected_paths,
            stash_entry: None,
            commit_preview: None,
            recovery_checkpoint_ref,
            observed_at: request.target.observed_at.clone(),
            apply_authority: DailyLoopApplyAuthority::default(),
        };
        if state == DailyLoopPreviewState::Ready {
            preview.apply_authority.actor_ref = Some(request.actor_ref.clone());
            preview.apply_authority.mutation_preview = Some(mutation_preview);
        }
        preview.seal()
    }

    fn preview_commit_amend(
        &self,
        request: &DailyLoopRequest,
        target: &DailyLoopTarget,
    ) -> DailyLoopPreview {
        let message = request.message.clone().unwrap_or_default();
        let mode = if request.kind == DailyLoopOperationKind::Amend {
            GitCommitMode::Amend
        } else {
            GitCommitMode::Normal
        };
        let mut commit_request = GitCommitRequest::with_observed_at(
            request.target.workspace_ref.clone(),
            target.worktree.worktree_root.clone(),
            mode,
            message.clone(),
            request.target.observed_at.clone(),
        )
        .with_actor(GitCommitActorRef {
            actor_class: "local_user".to_string(),
            display_label: "Local Git actor".to_string(),
            stable_id: Some(request.actor_ref.clone()),
        })
        .with_launch_source_ref(request.caller_command_id.clone());
        if mode == GitCommitMode::Amend {
            commit_request = commit_request.acknowledge_history_guardrail();
        }
        let canonical = self.commit_service.preview(&commit_request);
        let state = match canonical.preview_state {
            GitCommitPreviewState::ReadyToCommit => DailyLoopPreviewState::Ready,
            GitCommitPreviewState::Blocked => DailyLoopPreviewState::Blocked,
            GitCommitPreviewState::Degraded => DailyLoopPreviewState::Degraded,
        };
        let blocked_reason = if state == DailyLoopPreviewState::Ready {
            None
        } else {
            Some(if canonical.blocked_reasons.is_empty() {
                "Git commit preview is not ready".to_string()
            } else {
                canonical.blocked_reasons.join("; ")
            })
        };
        let mut preview = DailyLoopPreview {
            record_kind: DAILY_LOOP_PREVIEW_RECORD_KIND.to_string(),
            schema_version: DAILY_LOOP_PREVIEW_SCHEMA_VERSION,
            service_ref: "aureline-git.daily_loop".to_string(),
            target: target.clone(),
            kind: request.kind,
            state,
            blocked_reason,
            affected_paths: Vec::new(),
            stash_entry: None,
            commit_preview: Some(DailyLoopCommitPreview {
                message,
                staged_file_count: u32::try_from(canonical.scope.staged_count).unwrap_or(u32::MAX),
                lines_added: 0,
                lines_deleted: 0,
                is_amend: request.kind == DailyLoopOperationKind::Amend,
                original_head: canonical.history_guardrail.preflight_head_oid.clone(),
            }),
            recovery_checkpoint_ref: canonical.history_guardrail.recovery_ref.clone(),
            observed_at: request.target.observed_at.clone(),
            apply_authority: DailyLoopApplyAuthority::default(),
        };
        if state == DailyLoopPreviewState::Ready {
            preview.apply_authority.actor_ref = Some(request.actor_ref.clone());
            preview.apply_authority.commit_preview = Some(canonical);
        }
        preview.seal()
    }

    fn preview_stash_operation(
        &self,
        request: &DailyLoopRequest,
        _target: &DailyLoopTarget,
    ) -> DailyLoopPreview {
        DailyLoopPreview::blocked(
            request,
            "stash mutation is inspect-only until exact checkpoint and stale-evidence authority is available",
        )
    }

    // -----------------------------------------------------------------------
    // Internal: apply implementations
    // -----------------------------------------------------------------------

    fn apply_stage_unstage(&self, preview: &DailyLoopPreview, _actor_ref: &str) -> DailyLoopResult {
        let Some(canonical) = preview.apply_authority.mutation_preview.as_ref() else {
            return DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                "mutation preview authority is unavailable; reopen review",
            );
        };
        let result = GitMutationService::default().apply(canonical, observed_at_now());
        match result.outcome_state {
            GitMutationOutcomeState::Applied | GitMutationOutcomeState::Reverted => {
                let mut daily = DailyLoopResult::completed(
                    &preview.target,
                    preview.kind,
                    preview.affected_paths.clone(),
                );
                daily.recovery_checkpoint_ref = Some(result.checkpoint.checkpoint_ref);
                daily
            }
            GitMutationOutcomeState::BlockedNoChangesMade => DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                result
                    .failure_reason
                    .unwrap_or_else(|| "mutation evidence became stale; reopen review".to_string()),
            ),
            GitMutationOutcomeState::Failed => DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                result
                    .failure_reason
                    .unwrap_or_else(|| "Git mutation failed safely".to_string()),
            ),
        }
    }

    fn apply_commit_amend(&self, preview: &DailyLoopPreview, _actor_ref: &str) -> DailyLoopResult {
        let Some(canonical) = preview.apply_authority.commit_preview.as_ref() else {
            return DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                "commit preview authority is unavailable; reopen review",
            );
        };
        let result = self.commit_service.apply(canonical, observed_at_now());
        match result.outcome_state {
            GitCommitOutcomeState::Committed => DailyLoopResult {
                record_kind: DAILY_LOOP_RESULT_RECORD_KIND.to_string(),
                schema_version: DAILY_LOOP_RESULT_SCHEMA_VERSION,
                service_ref: "aureline-git.daily_loop".to_string(),
                target: preview.target.clone(),
                kind: preview.kind,
                outcome: DailyLoopOutcomeState::Completed,
                outcome_reason: None,
                affected_paths: result
                    .committed_targets
                    .iter()
                    .map(|target| target.repo_relative_path.to_string_lossy().to_string())
                    .collect(),
                commit_hash: result.commit_oid,
                created_stash_entry: None,
                recovery_checkpoint_ref: preview.recovery_checkpoint_ref.clone(),
                observed_at: preview.observed_at.clone(),
            },
            GitCommitOutcomeState::BlockedNoChangesMade => DailyLoopResult::blocked(
                &preview.target,
                preview.kind,
                result.failure_reason.unwrap_or_else(|| {
                    "commit evidence became stale; reopen commit review".to_string()
                }),
            ),
            GitCommitOutcomeState::Failed => DailyLoopResult::failed(
                &preview.target,
                preview.kind,
                result
                    .failure_reason
                    .unwrap_or_else(|| "Git commit failed safely".to_string()),
            ),
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
            apply_authority: DailyLoopApplyAuthority::default(),
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
            apply_authority: DailyLoopApplyAuthority::default(),
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
    pub fn blocked(
        target: &DailyLoopTarget,
        kind: DailyLoopOperationKind,
        reason: impl Into<String>,
    ) -> Self {
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
    pub fn failed(
        target: &DailyLoopTarget,
        kind: DailyLoopOperationKind,
        reason: impl Into<String>,
    ) -> Self {
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

fn validate_daily_request(request: &DailyLoopRequest) -> Result<(), &'static str> {
    if request.caller_command_id != request.kind.command_id() {
        return Err("caller command id does not match the requested Git operation");
    }
    if !request.target.repo.repo_root.is_absolute()
        || !request.target.worktree.worktree_root.is_absolute()
    {
        return Err("daily-loop repository and worktree roots must be absolute");
    }
    bounded_daily_paths(&request.path_scope)?;
    if request
        .commit_ref
        .as_deref()
        .is_some_and(|value| !valid_git_ref_input(value))
    {
        return Err("commit/ref input is not safe for this Git adapter");
    }
    if request
        .stash_entry_ref
        .as_deref()
        .is_some_and(|value| !valid_git_ref_input(value))
    {
        return Err("stash entry ref is not safe for this Git adapter");
    }
    if request
        .line_range
        .as_deref()
        .is_some_and(|value| !valid_blame_line_range(value))
    {
        return Err("blame line range is not a bounded start,end pair");
    }
    if request.message.as_deref().is_some_and(|message| {
        message.len() > 64 * 1024
            || message
                .chars()
                .any(|ch| (ch.is_control() && !matches!(ch, '\n' | '\t')) || ch == '\u{7f}')
    }) {
        return Err("Git message exceeds the review boundary or contains control bytes");
    }
    if !valid_identity_metadata(&request.actor_ref)
        || !valid_identity_metadata(&request.caller_command_id)
        || !valid_identity_metadata(&request.target.workspace_ref)
    {
        return Err("daily-loop identity metadata is outside the review boundary");
    }
    if !valid_observed_timestamp(&request.target.observed_at) {
        return Err("daily-loop observation timestamp is invalid");
    }
    Ok(())
}

fn bounded_daily_paths(paths: &[PathBuf]) -> Result<(), &'static str> {
    if paths.len() > MAX_DAILY_LOOP_PATHS {
        return Err("daily-loop path count exceeds the review limit");
    }
    let mut total_bytes = 0usize;
    let mut seen = HashSet::with_capacity(paths.len());
    for path in paths {
        if path.as_os_str().is_empty()
            || path.is_absolute()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err("daily-loop paths must be normalized repository-relative paths");
        }
        let Some(path) = path.to_str() else {
            return Err("daily-loop path is not valid UTF-8 for this adapter");
        };
        if !normalized_repo_path_text(path) {
            return Err("daily-loop paths must be normalized repository-relative paths");
        }
        if path.len() > MAX_DAILY_LOOP_PATH_BYTES {
            return Err("daily-loop path exceeds the per-path review limit");
        }
        total_bytes = total_bytes
            .checked_add(path.len())
            .and_then(|value| value.checked_add(1))
            .ok_or("daily-loop path scope exceeds the review limit")?;
        if total_bytes > MAX_DAILY_LOOP_SCOPE_BYTES {
            return Err("daily-loop path scope exceeds the review limit");
        }
        if !seen.insert(path) {
            return Err("daily-loop path scope contains a duplicate path");
        }
    }
    Ok(())
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

fn valid_git_ref_input(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_DAILY_LOOP_REF_BYTES
        && !value.starts_with('-')
        && value
            .chars()
            .all(|ch| !ch.is_control() && !ch.is_whitespace())
}

fn valid_identity_metadata(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_DAILY_LOOP_REF_BYTES
        && value.chars().all(|ch| !ch.is_control() && ch != '\u{7f}')
}

fn valid_observed_timestamp(value: &str) -> bool {
    let Some(timestamp) = value.strip_suffix('Z') else {
        return false;
    };
    if timestamp.is_empty() || value.len() > 64 {
        return false;
    }
    let (seconds, fraction) = timestamp
        .split_once('.')
        .map_or((timestamp, None), |(seconds, fraction)| {
            (seconds, Some(fraction))
        });
    let valid_fraction = match fraction {
        Some(fraction) => {
            !fraction.is_empty()
                && fraction.len() <= 9
                && fraction.bytes().all(|byte| byte.is_ascii_digit())
        }
        None => true,
    };
    !seconds.is_empty() && seconds.bytes().all(|byte| byte.is_ascii_digit()) && valid_fraction
}

fn support_safe_timestamp(value: &str) -> String {
    if valid_observed_timestamp(value) {
        value.to_string()
    } else {
        "unavailable".to_string()
    }
}

fn valid_blame_line_range(value: &str) -> bool {
    let Some((start, end)) = value.split_once(',') else {
        return false;
    };
    if value.len() > 64 || start.is_empty() || end.is_empty() || end.contains(',') {
        return false;
    }
    let (Ok(start), Ok(end)) = (start.parse::<u32>(), end.parse::<u32>()) else {
        return false;
    };
    start > 0 && end >= start && end - start < MAX_DAILY_LOOP_ROWS as u32
}

fn parse_absolute_git_path(bytes: &[u8]) -> Option<PathBuf> {
    let text = std::str::from_utf8(bytes)
        .ok()?
        .trim_end_matches(['\n', '\r']);
    if text.is_empty() || text.len() > 32 * 1024 || text.chars().any(char::is_control) {
        return None;
    }
    let path = PathBuf::from(text);
    path.is_absolute().then_some(path)
}

fn parse_bounded_git_label(bytes: &[u8]) -> Option<String> {
    let label = std::str::from_utf8(bytes)
        .ok()?
        .trim_end_matches(['\n', '\r']);
    if label.is_empty()
        || label.len() > MAX_DAILY_LOOP_REF_BYTES
        || label.chars().any(|ch| ch.is_control() || ch == '\u{7f}')
    {
        return None;
    }
    Some(label.to_string())
}

fn read_git_bool<B: DailyLoopBackend + ?Sized>(
    backend: &B,
    root: &Path,
    args: &[&str],
) -> Result<bool, DailyLoopBackendError> {
    let output = backend.run_git(root, args)?;
    if !output.success {
        return Err(DailyLoopBackendError::new(
            DailyLoopBackendErrorClass::Io,
            "git repository metadata could not be read safely",
        ));
    }
    match output.stdout.as_slice() {
        b"true\n" | b"true\r\n" => Ok(true),
        b"false\n" | b"false\r\n" => Ok(false),
        _ => Err(DailyLoopBackendError::new(
            DailyLoopBackendErrorClass::Io,
            "git repository metadata returned an invalid boolean",
        )),
    }
}

fn redacted_support_digest(field: &str, value: &str) -> String {
    let bytes = value.as_bytes();
    digest::sha256_framed_token(&[
        b"support.redaction.local_first_default",
        b"git_daily_loop_support_export_record.v2",
        field.as_bytes(),
        bytes,
    ])
}

fn backend_failure_reason(error: &DailyLoopBackendError) -> String {
    format!("Git backend unavailable ({})", error.class.as_str())
}

fn daily_loop_failure_reason(output: &DailyLoopCommandOutput) -> String {
    output
        .status_code
        .map(|code| format!("Git command failed with exit status {code}"))
        .unwrap_or_else(|| "Git command failed without an exit status".to_string())
}

fn bounded_record_text(value: &str, max_bytes: usize) -> bool {
    value.len() <= max_bytes && value.chars().all(|ch| !ch.is_control() && ch != '\u{7f}')
}

fn valid_git_oid(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn valid_git_epoch(value: &str) -> bool {
    let digits = value.strip_prefix('-').unwrap_or(value);
    !digits.is_empty() && value.len() <= 32 && digits.bytes().all(|byte| byte.is_ascii_digit())
}

fn blame_header(line: &str) -> Option<(&str, u32)> {
    let mut fields = line.split_whitespace();
    let hash = fields.next()?;
    let original_line = fields.next()?;
    let final_line = fields.next()?;
    let final_line = final_line.parse::<u32>().ok()?;
    if !valid_git_oid(hash) || original_line.parse::<u32>().is_err() {
        return None;
    }
    Some((hash, final_line))
}

fn parse_porcelain_status(bytes: &[u8]) -> Result<Vec<DailyLoopPathStatus>, &'static str> {
    let mut fields = bytes.split(|byte| *byte == 0);
    let mut rows = Vec::new();
    let mut total_path_bytes = 0usize;
    while let Some(record) = fields.next() {
        if record.is_empty() {
            continue;
        }
        if record.len() < 4 || record[2] != b' ' {
            return Err("Git status returned an unsupported porcelain record");
        }
        let index = record[0];
        let worktree = record[1];
        let path = std::str::from_utf8(&record[3..])
            .map_err(|_| "Git status contains a non-UTF-8 path omitted by this adapter")?;
        if path.is_empty()
            || path.len() > MAX_DAILY_LOOP_PATH_BYTES
            || !normalized_repo_path_text(path)
        {
            return Err("Git status contains a path exceeding the adapter limit");
        }
        if Path::new(path)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err("Git status contains a non-normalized path");
        }
        total_path_bytes = total_path_bytes
            .checked_add(path.len())
            .and_then(|value| value.checked_add(1))
            .ok_or("Git status path evidence exceeds the adapter limit")?;
        if rows.len() >= MAX_DAILY_LOOP_ROWS || total_path_bytes > MAX_DAILY_LOOP_SCOPE_BYTES {
            return Err("Git status path evidence exceeds the adapter limit");
        }
        let is_rename_or_copy = matches!(index, b'R' | b'C') || matches!(worktree, b'R' | b'C');
        if is_rename_or_copy {
            let old_path = fields
                .next()
                .ok_or("Git status rename record is incomplete")?;
            let old_path = std::str::from_utf8(old_path)
                .map_err(|_| "Git status rename source is not valid UTF-8")?;
            if old_path.is_empty()
                || old_path.len() > MAX_DAILY_LOOP_PATH_BYTES
                || !normalized_repo_path_text(old_path)
                || Path::new(old_path)
                    .components()
                    .any(|component| !matches!(component, Component::Normal(_)))
            {
                return Err("Git status rename source exceeds the adapter boundary");
            }
            total_path_bytes = total_path_bytes
                .checked_add(old_path.len())
                .and_then(|value| value.checked_add(1))
                .ok_or("Git status path evidence exceeds the adapter limit")?;
            if total_path_bytes > MAX_DAILY_LOOP_SCOPE_BYTES {
                return Err("Git status path evidence exceeds the adapter limit");
            }
        }
        let is_untracked = index == b'?' && worktree == b'?';
        let is_conflicted = matches!(
            (index, worktree),
            (b'D', b'D')
                | (b'A', b'U')
                | (b'U', b'D')
                | (b'U', b'A')
                | (b'D', b'U')
                | (b'A', b'A')
                | (b'U', b'U')
        );
        let is_staged = index != b' ' && index != b'?' && !is_conflicted;
        let is_unstaged = worktree != b' ' && worktree != b'?' && !is_conflicted;
        let change = if is_conflicted {
            b'U'
        } else if is_untracked {
            b'?'
        } else if is_staged {
            index
        } else {
            worktree
        };
        let change_kind =
            parse_status_char(change).ok_or("Git status returned an unknown porcelain state")?;
        rows.push(DailyLoopPathStatus {
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
    Ok(rows)
}

fn observed_at_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}.{:03}Z", dur.as_secs(), dur.subsec_millis())
}

fn parse_status_char(c: u8) -> Option<DailyLoopPathChangeKind> {
    Some(match c {
        b'M' => DailyLoopPathChangeKind::Modified,
        b'A' => DailyLoopPathChangeKind::Added,
        b'D' => DailyLoopPathChangeKind::Deleted,
        b'R' => DailyLoopPathChangeKind::Renamed,
        b'C' => DailyLoopPathChangeKind::Copied,
        b'T' => DailyLoopPathChangeKind::TypeChanged,
        b'U' => DailyLoopPathChangeKind::Conflict,
        b'?' => DailyLoopPathChangeKind::Untracked,
        b'!' => DailyLoopPathChangeKind::Ignored,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_unmerged_porcelain_pair_projects_as_conflict() {
        for pair in ["DD", "AU", "UD", "UA", "DU", "AA", "UU"] {
            let record = format!("{pair} conflicted.txt\0");
            let rows = parse_porcelain_status(record.as_bytes()).expect("parse conflict row");
            assert_eq!(rows.len(), 1);
            assert!(rows[0].is_conflicted, "{pair} must be conflicted");
            assert_eq!(rows[0].change_kind, DailyLoopPathChangeKind::Conflict);
        }
    }

    #[test]
    fn unknown_porcelain_states_do_not_fabricate_modified_rows() {
        assert_eq!(
            parse_porcelain_status(b"X  unexpected.txt\0"),
            Err("Git status returned an unknown porcelain state")
        );
    }
}
