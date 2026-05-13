//! Repository status, branch identity, and shared Git consumer projections.
//!
//! The service in this module gathers local Git truth once through a bounded
//! backend, parses it into a canonical snapshot, and derives shell, activity,
//! and review records from that same snapshot. Consumers should subscribe to
//! these records instead of invoking Git independently.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`GitStatusSnapshot`].
pub const GIT_STATUS_SNAPSHOT_RECORD_KIND: &str = "git_status_snapshot";

/// Stable record-kind tag for [`GitShellStatusRecord`].
pub const GIT_SHELL_STATUS_RECORD_KIND: &str = "git_shell_status_record";

/// Stable record-kind tag for [`GitActivityRecord`].
pub const GIT_ACTIVITY_RECORD_KIND: &str = "git_activity_record";

/// Stable record-kind tag for [`GitReviewSeedRecord`].
pub const GIT_REVIEW_SEED_RECORD_KIND: &str = "git_review_seed_record";

const GIT_STATUS_SNAPSHOT_SCHEMA_VERSION: u32 = 1;
const CONSUMER_PROJECTION_BUNDLE_RECORD_KIND: &str = "git_consumer_projection_bundle";
const CONSUMER_PROJECTION_BUNDLE_SCHEMA_VERSION: u32 = 1;
const GIT_SHELL_STATUS_SCHEMA_VERSION: u32 = 1;
const GIT_ACTIVITY_SCHEMA_VERSION: u32 = 1;
const GIT_REVIEW_SEED_SCHEMA_VERSION: u32 = 1;

/// Request for a repository status snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitStatusRequest {
    /// Stable workspace identity used by downstream projections.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub root_path: PathBuf,
    /// Observation timestamp supplied by the caller for deterministic exports.
    pub observed_at: String,
}

impl GitStatusRequest {
    /// Builds a request for `root_path` with a derived local workspace identity.
    pub fn for_root(root_path: impl Into<PathBuf>) -> Self {
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
            observed_at: observed_at_now(),
        }
    }

    /// Builds a request with explicit identity and timestamp fields.
    pub fn with_observed_at(
        workspace_ref: impl Into<String>,
        root_path: impl Into<PathBuf>,
        observed_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            root_path: root_path.into(),
            observed_at: observed_at.into(),
        }
    }
}

/// Canonical snapshot emitted by the Git service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitStatusSnapshot {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable service identity for the producer.
    pub service_ref: String,
    /// Workspace identity copied from the request.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub requested_root: PathBuf,
    /// Repository identity when the root belongs to a Git worktree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryIdentity>,
    /// Branch or detached-head identity for the current worktree.
    pub head: HeadIdentity,
    /// Coarse availability and freshness state for the snapshot.
    pub service_state: GitServiceState,
    /// Human-readable reason when the state is not current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<String>,
    /// Parsed status summary and discovery coverage.
    pub discovery: ChangeDiscovery,
    /// Count summary for status-bar, activity, and review surfaces.
    pub change_summary: ChangeSummary,
    /// Per-path changes parsed from Git status output.
    pub changes: Vec<GitChange>,
    /// Consumer projections authorized to use this snapshot as truth.
    pub consumer_refs: Vec<GitConsumerRef>,
    /// Observation timestamp supplied by the caller.
    pub observed_at: String,
}

impl GitStatusSnapshot {
    /// Builds an honest unavailable snapshot for a non-current service state.
    pub fn degraded(
        request: &GitStatusRequest,
        state: GitServiceState,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: GIT_STATUS_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: GIT_STATUS_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "service.git.status.alpha".to_string(),
            workspace_ref: request.workspace_ref.clone(),
            requested_root: request.root_path.clone(),
            repository: None,
            head: HeadIdentity::unknown(),
            service_state: state,
            degraded_reason: Some(reason.into()),
            discovery: ChangeDiscovery::unavailable(),
            change_summary: ChangeSummary::default(),
            changes: Vec::new(),
            consumer_refs: consumer_refs_for(&request.workspace_ref),
            observed_at: request.observed_at.clone(),
        }
    }

    /// Returns true when a current local Git worktree was inspected.
    pub fn is_current(&self) -> bool {
        self.service_state == GitServiceState::Current
    }
}

/// Identity of the local repository and selected worktree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryIdentity {
    /// Stable repository identity generated from the canonical root.
    pub repo_ref: String,
    /// Stable worktree identity generated from the canonical root.
    pub worktree_ref: String,
    /// Repository label safe for shell chrome.
    pub repo_label: String,
    /// Canonical worktree root reported by Git.
    pub repo_root: PathBuf,
    /// Git directory used by this worktree.
    pub git_dir: PathBuf,
    /// Common Git directory used by linked worktrees.
    pub common_dir: PathBuf,
}

/// Branch, detached, or unborn HEAD identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadIdentity {
    /// Coarse branch state for shell and review surfaces.
    pub state: BranchState,
    /// Current branch label when HEAD is attached or unborn.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Full Git ref for the current branch when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_ref: Option<String>,
    /// Full HEAD object id when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_oid: Option<String>,
    /// Short HEAD object id for compact status surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_short_oid: Option<String>,
    /// Upstream branch name when Git reports one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
    /// Commit count ahead of upstream when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ahead: Option<u32>,
    /// Commit count behind upstream when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behind: Option<u32>,
}

impl HeadIdentity {
    fn unknown() -> Self {
        Self {
            state: BranchState::Unknown,
            branch_label: None,
            branch_ref: None,
            head_oid: None,
            head_short_oid: None,
            upstream: None,
            ahead: None,
            behind: None,
        }
    }
}

/// Coarse Git HEAD state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchState {
    /// HEAD points to a named branch.
    Attached,
    /// HEAD points directly to a commit.
    Detached,
    /// A named branch exists but has no first commit yet.
    Unborn,
    /// Branch identity could not be determined.
    Unknown,
}

impl BranchState {
    /// Stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attached => "attached",
            Self::Detached => "detached",
            Self::Unborn => "unborn",
            Self::Unknown => "unknown",
        }
    }
}

/// Coarse availability state for a status snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitServiceState {
    /// Git status was read successfully from a local worktree.
    Current,
    /// The selected root is not inside a Git repository.
    NotRepository,
    /// The Git executable or backend was not available.
    GitUnavailable,
    /// Git was available, but the refresh failed.
    RefreshFailed,
}

impl GitServiceState {
    /// Stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::NotRepository => "not_repository",
            Self::GitUnavailable => "git_unavailable",
            Self::RefreshFailed => "refresh_failed",
        }
    }

    /// Human-readable label rendered by shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Current => "Git current",
            Self::NotRepository => "No repository",
            Self::GitUnavailable => "Git unavailable",
            Self::RefreshFailed => "Git refresh failed",
        }
    }

    /// True when the state must not be rendered as full/current Git truth.
    pub const fn narrows_current_claim(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// Optional degraded-state token for shell status vocabulary.
    pub const fn degraded_token(self) -> Option<&'static str> {
        match self {
            Self::Current => None,
            Self::NotRepository => Some("Unsupported"),
            Self::GitUnavailable => Some("Offline"),
            Self::RefreshFailed => Some("Stale"),
        }
    }
}

/// Status discovery coverage for one snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeDiscovery {
    /// True when the worktree status command completed.
    pub status_available: bool,
    /// True when branch headers were present in the status output.
    pub branch_identity_available: bool,
    /// True when path-level changes were parsed from Git status output.
    pub change_list_available: bool,
    /// True when omitted or degraded status prevents a full current claim.
    pub current_claim_narrowed: bool,
    /// Human-readable coverage label for support exports and status details.
    pub coverage_label: String,
}

impl ChangeDiscovery {
    fn current(branch_identity_available: bool, changes: usize) -> Self {
        Self {
            status_available: true,
            branch_identity_available,
            change_list_available: true,
            current_claim_narrowed: false,
            coverage_label: format!("current status with {changes} changed path(s)"),
        }
    }

    fn unavailable() -> Self {
        Self {
            status_available: false,
            branch_identity_available: false,
            change_list_available: false,
            current_claim_narrowed: true,
            coverage_label: "Git status unavailable for this root".to_string(),
        }
    }
}

/// Count summary derived from a single change list.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeSummary {
    /// Number of staged or index-visible paths.
    pub staged_count: u32,
    /// Number of unstaged tracked paths.
    pub unstaged_count: u32,
    /// Number of untracked paths.
    pub untracked_count: u32,
    /// Number of ignored paths when ignored paths are included.
    pub ignored_count: u32,
    /// Number of conflicted paths.
    pub conflicted_count: u32,
    /// Total number of changed tracked or untracked paths.
    pub total_changed_count: u32,
}

impl ChangeSummary {
    /// Builds a count summary from parsed path changes.
    pub fn from_changes(changes: &[GitChange]) -> Self {
        let mut summary = Self::default();
        for change in changes {
            if change.is_staged {
                summary.staged_count += 1;
            }
            if change.is_unstaged {
                summary.unstaged_count += 1;
            }
            if change.change_kind == ChangeKind::Untracked {
                summary.untracked_count += 1;
            }
            if change.change_kind == ChangeKind::Ignored {
                summary.ignored_count += 1;
            }
            if change.is_conflicted {
                summary.conflicted_count += 1;
            }
            if change.change_kind != ChangeKind::Ignored {
                summary.total_changed_count += 1;
            }
        }
        summary
    }

    /// Returns a compact human-readable label.
    pub fn compact_label(&self) -> String {
        if self.total_changed_count == 0 && self.ignored_count == 0 {
            return "clean".to_string();
        }

        let mut parts = Vec::new();
        if self.staged_count > 0 {
            parts.push(format!("{} staged", self.staged_count));
        }
        if self.unstaged_count > 0 {
            parts.push(format!("{} unstaged", self.unstaged_count));
        }
        if self.untracked_count > 0 {
            parts.push(format!("{} untracked", self.untracked_count));
        }
        if self.conflicted_count > 0 {
            parts.push(format!("{} conflicted", self.conflicted_count));
        }
        if parts.is_empty() && self.ignored_count > 0 {
            parts.push(format!("{} ignored", self.ignored_count));
        }
        parts.join(", ")
    }
}

/// One parsed path-level Git status change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitChange {
    /// Path reported by Git relative to the repository root.
    pub path: PathBuf,
    /// Original path for rename or copy records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_path: Option<PathBuf>,
    /// Two-character Git status code.
    pub status_code: String,
    /// High-level change kind for summary and filtering.
    pub change_kind: ChangeKind,
    /// True when the index side contains a change.
    pub is_staged: bool,
    /// True when the worktree side contains a change.
    pub is_unstaged: bool,
    /// True when Git reports an unresolved conflict.
    pub is_conflicted: bool,
}

/// High-level path change class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
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

impl ChangeKind {
    /// Stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::TypeChanged => "type_changed",
            Self::Renamed => "renamed",
            Self::Copied => "copied",
            Self::Untracked => "untracked",
            Self::Ignored => "ignored",
            Self::Conflict => "conflict",
        }
    }
}

/// Consumer projection reference derived from the canonical snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitConsumerRef {
    /// Surface or export lane that may consume this snapshot.
    pub consumer_surface: String,
    /// Stable projection identity for that consumer.
    pub projection_ref: String,
    /// Authority note explaining that local Git remains the truth source.
    pub authority_note: String,
}

/// Shared consumer bundle derived from one canonical snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjectionBundle {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Snapshot identity used as the single truth source.
    pub truth_source_ref: String,
    /// Shell status projection derived from the snapshot.
    pub shell: GitShellStatusRecord,
    /// Activity-center projection derived from the snapshot.
    pub activity: GitActivityRecord,
    /// Review seed projection derived from the snapshot.
    pub review: GitReviewSeedRecord,
}

impl ConsumerProjectionBundle {
    /// Materializes all first consumers from one Git status snapshot.
    pub fn from_snapshot(generated_at: impl Into<String>, snapshot: &GitStatusSnapshot) -> Self {
        let generated_at = generated_at.into();
        let truth_source_ref = snapshot_ref(snapshot);
        Self {
            record_kind: CONSUMER_PROJECTION_BUNDLE_RECORD_KIND.to_string(),
            schema_version: CONSUMER_PROJECTION_BUNDLE_SCHEMA_VERSION,
            generated_at: generated_at.clone(),
            truth_source_ref: truth_source_ref.clone(),
            shell: GitShellStatusRecord::from_snapshot(snapshot, &truth_source_ref),
            activity: GitActivityRecord::from_snapshot(snapshot, &truth_source_ref),
            review: GitReviewSeedRecord::from_snapshot(snapshot, &truth_source_ref),
        }
    }
}

/// Shell status projection for Git branch and worktree changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitShellStatusRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable status item identity.
    pub status_item_id: String,
    /// Coarse service-state token.
    pub state_token: String,
    /// Compact label suitable for title/context or status bar chrome.
    pub current_value_label: String,
    /// Repository label when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_label: Option<String>,
    /// Branch label when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Compact changed-path count label.
    pub change_summary_label: String,
    /// Optional degraded token using shell status vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    /// Command id that opens the Git status details surface.
    pub primary_command_id: String,
    /// Surface ref opened by the primary command.
    pub opens_surface_ref: String,
    /// True when this row must not be rendered as a current/full claim.
    pub current_claim_narrowed: bool,
    /// Snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
}

impl GitShellStatusRecord {
    /// Materializes a shell row from a canonical Git snapshot.
    pub fn from_snapshot(snapshot: &GitStatusSnapshot, truth_source_ref: &str) -> Self {
        let branch_label = snapshot.head.branch_label.clone();
        let repo_label = snapshot
            .repository
            .as_ref()
            .map(|repo| repo.repo_label.clone());
        let change_label = snapshot.change_summary.compact_label();
        let current_value_label = if snapshot.service_state == GitServiceState::Current {
            match branch_label.as_deref() {
                Some(branch) => format!("{branch} · {change_label}"),
                None if snapshot.head.state == BranchState::Detached => {
                    let rev = snapshot
                        .head
                        .head_short_oid
                        .as_deref()
                        .unwrap_or("detached");
                    format!("Detached HEAD · {rev} · {change_label}")
                }
                None => format!("Git · {change_label}"),
            }
        } else {
            snapshot.service_state.label().to_string()
        };

        Self {
            record_kind: GIT_SHELL_STATUS_RECORD_KIND.to_string(),
            schema_version: GIT_SHELL_STATUS_SCHEMA_VERSION,
            status_item_id: format!("status.item.git.{}", sanitize_id(&snapshot.workspace_ref)),
            state_token: snapshot.service_state.as_str().to_string(),
            current_value_label,
            repo_label,
            branch_label,
            change_summary_label: change_label,
            degraded_token: snapshot.service_state.degraded_token().map(str::to_string),
            primary_command_id: "cmd:git.status.inspect".to_string(),
            opens_surface_ref: format!(
                "surface.git.status.{}",
                sanitize_id(&snapshot.workspace_ref)
            ),
            current_claim_narrowed: snapshot.service_state.narrows_current_claim(),
            truth_source_ref: truth_source_ref.to_string(),
        }
    }
}

/// Activity-center projection for Git refresh state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitActivityRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable durable row identity.
    pub activity_row_id: String,
    /// Activity family token.
    pub job_family: String,
    /// Lifecycle state token for the row.
    pub state_class: String,
    /// Activity-center partition token.
    pub partition: String,
    /// User-facing summary label.
    pub summary_label: String,
    /// Detail label that preserves degraded reasons.
    pub detail_label: String,
    /// Command id that reopens Git status.
    pub open_details_command_id: String,
    /// Surface ref opened by the command.
    pub opens_surface_ref: String,
    /// Snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
}

impl GitActivityRecord {
    /// Materializes an activity row from a canonical Git snapshot.
    pub fn from_snapshot(snapshot: &GitStatusSnapshot, truth_source_ref: &str) -> Self {
        let (state_class, partition, summary_label, detail_label) = match snapshot.service_state {
            GitServiceState::Current => (
                "completed",
                "completed",
                "Git status refreshed".to_string(),
                format!(
                    "{} path(s) discovered; {}",
                    snapshot.change_summary.total_changed_count,
                    snapshot.change_summary.compact_label()
                ),
            ),
            GitServiceState::NotRepository => (
                "completed",
                "completed",
                "No Git repository attached".to_string(),
                snapshot
                    .degraded_reason
                    .clone()
                    .unwrap_or_else(|| "Selected root is outside a Git worktree.".to_string()),
            ),
            GitServiceState::GitUnavailable | GitServiceState::RefreshFailed => (
                "failed",
                "needs_attention",
                snapshot.service_state.label().to_string(),
                snapshot
                    .degraded_reason
                    .clone()
                    .unwrap_or_else(|| "Git status could not be refreshed.".to_string()),
            ),
        };

        Self {
            record_kind: GIT_ACTIVITY_RECORD_KIND.to_string(),
            schema_version: GIT_ACTIVITY_SCHEMA_VERSION,
            activity_row_id: format!("activity.git.{}", sanitize_id(&snapshot.workspace_ref)),
            job_family: "git_status_refresh".to_string(),
            state_class: state_class.to_string(),
            partition: partition.to_string(),
            summary_label,
            detail_label,
            open_details_command_id: "cmd:git.status.inspect".to_string(),
            opens_surface_ref: format!(
                "surface.git.status.{}",
                sanitize_id(&snapshot.workspace_ref)
            ),
            truth_source_ref: truth_source_ref.to_string(),
        }
    }
}

/// Review seed projection for local diff authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSeedRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable review workspace seed identity.
    pub review_workspace_ref: String,
    /// Local diff authority token.
    pub local_diff_authority: String,
    /// Provider overlay state token.
    pub provider_overlay_state: String,
    /// Branch label or detached-head label used by review surfaces.
    pub branch_or_head_label: String,
    /// Base revision ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_revision_ref: Option<String>,
    /// Head revision ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_ref: Option<String>,
    /// Changed path count available for review seed routing.
    pub changed_path_count: u32,
    /// Optional degraded or stale reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<String>,
    /// Snapshot ref used for support/debug joins.
    pub truth_source_ref: String,
}

impl GitReviewSeedRecord {
    /// Materializes a review seed record from a canonical Git snapshot.
    pub fn from_snapshot(snapshot: &GitStatusSnapshot, truth_source_ref: &str) -> Self {
        let head_revision_ref = snapshot
            .head
            .head_short_oid
            .as_ref()
            .map(|oid| format!("git.rev.{oid}"));
        let branch_or_head_label = snapshot
            .head
            .branch_label
            .clone()
            .or_else(|| {
                snapshot
                    .head
                    .head_short_oid
                    .as_ref()
                    .map(|oid| format!("detached {oid}"))
            })
            .unwrap_or_else(|| snapshot.service_state.label().to_string());
        let local_diff_authority = if snapshot.service_state == GitServiceState::Current {
            "authoritative_local_git"
        } else {
            "unavailable_local_git"
        };

        Self {
            record_kind: GIT_REVIEW_SEED_RECORD_KIND.to_string(),
            schema_version: GIT_REVIEW_SEED_SCHEMA_VERSION,
            review_workspace_ref: format!("review.git.{}", sanitize_id(&snapshot.workspace_ref)),
            local_diff_authority: local_diff_authority.to_string(),
            provider_overlay_state: "not_configured_alpha".to_string(),
            branch_or_head_label,
            base_revision_ref: snapshot
                .head
                .upstream
                .as_ref()
                .map(|upstream| format!("git.upstream.{}", sanitize_id(upstream))),
            head_revision_ref,
            changed_path_count: snapshot.change_summary.total_changed_count,
            degraded_reason: snapshot.degraded_reason.clone(),
            truth_source_ref: truth_source_ref.to_string(),
        }
    }
}

/// Process output returned by a Git backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitCommandOutput {
    /// Standard output bytes emitted by Git.
    pub stdout: Vec<u8>,
    /// Standard error bytes emitted by Git.
    pub stderr: Vec<u8>,
    /// Process exit status code, when available.
    pub status_code: Option<i32>,
    /// True when Git exited successfully.
    pub success: bool,
}

/// Backend abstraction used by the Git status service.
pub trait GitStatusBackend {
    /// Runs Git with `args` from `root`.
    ///
    /// # Errors
    ///
    /// Returns [`GitBackendError`] when the backend cannot launch or supervise
    /// the Git process.
    fn run_git(&self, root: &Path, args: &[&str]) -> Result<GitCommandOutput, GitBackendError>;
}

/// System Git backend used by the live alpha service.
#[derive(Debug, Clone)]
pub struct SystemGitStatusBackend {
    git_binary: PathBuf,
}

impl Default for SystemGitStatusBackend {
    fn default() -> Self {
        Self {
            git_binary: PathBuf::from("git"),
        }
    }
}

impl SystemGitStatusBackend {
    /// Builds a backend that invokes `git_binary`.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl GitStatusBackend for SystemGitStatusBackend {
    fn run_git(&self, root: &Path, args: &[&str]) -> Result<GitCommandOutput, GitBackendError> {
        let output = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    GitBackendError::new(
                        GitBackendErrorClass::GitNotInstalled,
                        "git binary was not found",
                    )
                } else if err.kind() == std::io::ErrorKind::PermissionDenied {
                    GitBackendError::new(
                        GitBackendErrorClass::PermissionDenied,
                        format!("git could not be launched: {err}"),
                    )
                } else {
                    GitBackendError::new(
                        GitBackendErrorClass::Io,
                        format!("git process launch failed: {err}"),
                    )
                }
            })?;
        Ok(GitCommandOutput {
            stdout: output.stdout,
            stderr: output.stderr,
            status_code: output.status.code(),
            success: output.status.success(),
        })
    }
}

/// Typed backend failure classes surfaced in degraded snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitBackendErrorClass {
    /// The system `git` binary could not be found.
    GitNotInstalled,
    /// The Git binary exists but cannot be executed.
    PermissionDenied,
    /// The backend hit a generic process I/O error.
    Io,
}

impl GitBackendErrorClass {
    /// Stable token used in degraded snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitNotInstalled => "git_not_installed",
            Self::PermissionDenied => "permission_denied",
            Self::Io => "io",
        }
    }
}

/// Error returned by a Git backend before Git status can be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitBackendError {
    /// Typed backend error class.
    pub class: GitBackendErrorClass,
    /// Human-readable failure detail.
    pub message: String,
}

impl GitBackendError {
    /// Builds a backend error.
    pub fn new(class: GitBackendErrorClass, message: impl Into<String>) -> Self {
        Self {
            class,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GitBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.class.as_str(), self.message)
    }
}

impl std::error::Error for GitBackendError {}

/// Git service that gathers repository status and branch identity.
#[derive(Debug, Clone)]
pub struct GitStatusService<B = SystemGitStatusBackend> {
    backend: B,
}

impl Default for GitStatusService<SystemGitStatusBackend> {
    fn default() -> Self {
        Self {
            backend: SystemGitStatusBackend::default(),
        }
    }
}

impl<B> GitStatusService<B> {
    /// Builds a service around a custom backend.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }
}

impl<B: GitStatusBackend> GitStatusService<B> {
    /// Captures a canonical status snapshot for `request`.
    pub fn snapshot(&self, request: &GitStatusRequest) -> GitStatusSnapshot {
        if !request.root_path.exists() {
            return GitStatusSnapshot::degraded(
                request,
                GitServiceState::RefreshFailed,
                "selected root does not exist",
            );
        }

        let metadata = match self.read_repository_metadata(request) {
            Ok(metadata) => metadata,
            Err(ReadMetadataError::Backend(err)) => {
                return GitStatusSnapshot::degraded(
                    request,
                    GitServiceState::GitUnavailable,
                    err.message,
                )
            }
            Err(ReadMetadataError::NotRepository(reason)) => {
                return GitStatusSnapshot::degraded(request, GitServiceState::NotRepository, reason)
            }
            Err(ReadMetadataError::RefreshFailed(reason)) => {
                return GitStatusSnapshot::degraded(request, GitServiceState::RefreshFailed, reason)
            }
        };

        let output = match self.backend.run_git(
            &request.root_path,
            &[
                "-c",
                "status.relativePaths=false",
                "status",
                "--porcelain=v2",
                "--branch",
                "--untracked-files=all",
                "-z",
            ],
        ) {
            Ok(output) => output,
            Err(err) => {
                return GitStatusSnapshot::degraded(
                    request,
                    GitServiceState::GitUnavailable,
                    err.message,
                )
            }
        };

        if !output.success {
            return GitStatusSnapshot::degraded(
                request,
                classify_failed_status(&output),
                stderr_or_status(&output),
            );
        }

        let parsed = match parse_porcelain_v2(&output.stdout) {
            Ok(parsed) => parsed,
            Err(err) => {
                return GitStatusSnapshot::degraded(
                    request,
                    GitServiceState::RefreshFailed,
                    err.to_string(),
                )
            }
        };
        let branch_identity_available = parsed.head.state != BranchState::Unknown;
        let summary = ChangeSummary::from_changes(&parsed.changes);
        GitStatusSnapshot {
            record_kind: GIT_STATUS_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: GIT_STATUS_SNAPSHOT_SCHEMA_VERSION,
            service_ref: "service.git.status.alpha".to_string(),
            workspace_ref: request.workspace_ref.clone(),
            requested_root: request.root_path.clone(),
            repository: Some(metadata),
            head: parsed.head,
            service_state: GitServiceState::Current,
            degraded_reason: None,
            discovery: ChangeDiscovery::current(branch_identity_available, parsed.changes.len()),
            change_summary: summary,
            changes: parsed.changes,
            consumer_refs: consumer_refs_for(&request.workspace_ref),
            observed_at: request.observed_at.clone(),
        }
    }

    fn read_repository_metadata(
        &self,
        request: &GitStatusRequest,
    ) -> Result<RepositoryIdentity, ReadMetadataError> {
        let output = self
            .backend
            .run_git(
                &request.root_path,
                &[
                    "rev-parse",
                    "--show-toplevel",
                    "--git-dir",
                    "--git-common-dir",
                    "--is-inside-work-tree",
                    "--is-bare-repository",
                ],
            )
            .map_err(ReadMetadataError::Backend)?;

        if !output.success {
            let message = stderr_or_status(&output);
            if message
                .to_ascii_lowercase()
                .contains("not a git repository")
            {
                return Err(ReadMetadataError::NotRepository(message));
            }
            return Err(ReadMetadataError::RefreshFailed(message));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut lines = stdout.lines();
        let repo_root = lines
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| ReadMetadataError::RefreshFailed("missing repository root".into()))?;
        let git_dir = lines
            .next()
            .map(|line| resolve_git_path(&request.root_path, line))
            .ok_or_else(|| ReadMetadataError::RefreshFailed("missing git dir".into()))?;
        let common_dir = lines
            .next()
            .map(|line| resolve_git_path(&request.root_path, line))
            .ok_or_else(|| ReadMetadataError::RefreshFailed("missing common git dir".into()))?;
        let inside_work_tree = lines.next().unwrap_or("false").trim() == "true";
        let is_bare = lines.next().unwrap_or("true").trim() == "true";
        if !inside_work_tree || is_bare {
            return Err(ReadMetadataError::RefreshFailed(
                "selected Git repository is not an editable worktree".into(),
            ));
        }

        let repo_label = repo_root
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("repository")
            .to_string();
        let repo_id = sanitize_id(&repo_root.to_string_lossy());
        Ok(RepositoryIdentity {
            repo_ref: format!("repo.local.{repo_id}"),
            worktree_ref: format!("worktree.local.{repo_id}"),
            repo_label,
            repo_root,
            git_dir,
            common_dir,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReadMetadataError {
    Backend(GitBackendError),
    NotRepository(String),
    RefreshFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedPorcelain {
    head: HeadIdentity,
    changes: Vec<GitChange>,
}

/// Error returned when porcelain status output cannot be interpreted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusPorcelainParseError {
    message: String,
}

impl StatusPorcelainParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for StatusPorcelainParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for StatusPorcelainParseError {}

fn parse_porcelain_v2(input: &[u8]) -> Result<ParsedPorcelain, StatusPorcelainParseError> {
    let records = split_porcelain_records(input);
    let mut head = HeadIdentity::unknown();
    let mut changes = Vec::new();
    let mut i = 0usize;
    while i < records.len() {
        let record = records[i].as_str();
        if record.starts_with("# ") {
            parse_branch_header(record, &mut head);
            i += 1;
            continue;
        }
        if let Some(path) = record.strip_prefix("? ") {
            changes.push(change_for_path("??", ChangeKind::Untracked, path, None));
            i += 1;
            continue;
        }
        if let Some(path) = record.strip_prefix("! ") {
            changes.push(change_for_path("!!", ChangeKind::Ignored, path, None));
            i += 1;
            continue;
        }
        if record.starts_with("1 ") {
            changes.push(parse_ordinary_change(record)?);
            i += 1;
            continue;
        }
        if record.starts_with("2 ") {
            let (change, consumed_next) =
                parse_rename_or_copy_change(record, records.get(i + 1).map(String::as_str))?;
            changes.push(change);
            i += if consumed_next { 2 } else { 1 };
            continue;
        }
        if record.starts_with("u ") {
            changes.push(parse_unmerged_change(record)?);
            i += 1;
            continue;
        }
        return Err(StatusPorcelainParseError::new(format!(
            "unsupported porcelain record: {record}"
        )));
    }

    Ok(ParsedPorcelain { head, changes })
}

fn split_porcelain_records(input: &[u8]) -> Vec<String> {
    let has_nul = input.contains(&0);
    let parts: Vec<&[u8]> = if has_nul {
        input.split(|byte| *byte == 0).collect()
    } else {
        input.split(|byte| *byte == b'\n').collect()
    };
    parts
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(|part| {
            String::from_utf8_lossy(part)
                .trim_end_matches('\r')
                .to_string()
        })
        .filter(|part| !part.is_empty())
        .collect()
}

fn parse_branch_header(record: &str, head: &mut HeadIdentity) {
    if let Some(value) = record.strip_prefix("# branch.oid ") {
        let value = value.trim();
        if value == "(initial)" {
            head.state = BranchState::Unborn;
            return;
        }
        head.head_oid = Some(value.to_string());
        head.head_short_oid = Some(short_oid(value));
        return;
    }
    if let Some(value) = record.strip_prefix("# branch.head ") {
        let value = value.trim();
        if value == "(detached)" {
            head.state = BranchState::Detached;
            head.branch_label = None;
            head.branch_ref = None;
        } else {
            if head.state != BranchState::Unborn {
                head.state = BranchState::Attached;
            }
            head.branch_label = Some(value.to_string());
            head.branch_ref = Some(format!("refs/heads/{value}"));
        }
        return;
    }
    if let Some(value) = record.strip_prefix("# branch.upstream ") {
        head.upstream = Some(value.trim().to_string());
        return;
    }
    if let Some(value) = record.strip_prefix("# branch.ab ") {
        for part in value.split_whitespace() {
            if let Some(ahead) = part.strip_prefix('+') {
                head.ahead = ahead.parse::<u32>().ok();
            } else if let Some(behind) = part.strip_prefix('-') {
                head.behind = behind.parse::<u32>().ok();
            }
        }
    }
}

fn parse_ordinary_change(record: &str) -> Result<GitChange, StatusPorcelainParseError> {
    let parts: Vec<&str> = record.splitn(9, ' ').collect();
    if parts.len() < 9 {
        return Err(StatusPorcelainParseError::new(format!(
            "ordinary status record missing path: {record}"
        )));
    }
    let status_code = parts[1];
    let path = parts[8];
    let kind = classify_status(status_code);
    Ok(change_for_path(status_code, kind, path, None))
}

fn parse_rename_or_copy_change(
    record: &str,
    next_record: Option<&str>,
) -> Result<(GitChange, bool), StatusPorcelainParseError> {
    let parts: Vec<&str> = record.splitn(10, ' ').collect();
    if parts.len() < 10 {
        return Err(StatusPorcelainParseError::new(format!(
            "rename/copy status record missing path: {record}"
        )));
    }
    let status_code = parts[1];
    let score = parts[8];
    let mut path = parts[9];
    let mut original = None;
    let mut consumed_next = false;
    if let Some((new_path, old_path)) = path.split_once('\t') {
        path = new_path;
        original = Some(PathBuf::from(old_path));
    } else if let Some(next) = next_record {
        if !looks_like_porcelain_record(next) {
            original = Some(PathBuf::from(next));
            consumed_next = true;
        }
    }
    let kind = if score.starts_with('C') {
        ChangeKind::Copied
    } else {
        ChangeKind::Renamed
    };
    Ok((
        change_for_path(status_code, kind, path, original),
        consumed_next,
    ))
}

fn parse_unmerged_change(record: &str) -> Result<GitChange, StatusPorcelainParseError> {
    let parts: Vec<&str> = record.splitn(11, ' ').collect();
    if parts.len() < 11 {
        return Err(StatusPorcelainParseError::new(format!(
            "unmerged status record missing path: {record}"
        )));
    }
    Ok(change_for_path(
        parts[1],
        ChangeKind::Conflict,
        parts[10],
        None,
    ))
}

fn change_for_path(
    status_code: &str,
    kind: ChangeKind,
    path: &str,
    original_path: Option<PathBuf>,
) -> GitChange {
    let index = status_code.chars().next().unwrap_or('.');
    let worktree = status_code.chars().nth(1).unwrap_or('.');
    let is_conflicted = kind == ChangeKind::Conflict || status_code.chars().any(|ch| ch == 'U');
    GitChange {
        path: PathBuf::from(path),
        original_path,
        status_code: status_code.to_string(),
        change_kind: kind,
        is_staged: index != '.' && index != '?' && index != '!' && !is_conflicted,
        is_unstaged: worktree != '.' && worktree != '?' && worktree != '!' && !is_conflicted,
        is_conflicted,
    }
}

fn classify_status(status_code: &str) -> ChangeKind {
    if status_code.chars().any(|ch| ch == 'U') {
        return ChangeKind::Conflict;
    }
    if status_code.starts_with('R') {
        return ChangeKind::Renamed;
    }
    if status_code.starts_with('C') {
        return ChangeKind::Copied;
    }
    if status_code.chars().any(|ch| ch == 'A') {
        return ChangeKind::Added;
    }
    if status_code.chars().any(|ch| ch == 'D') {
        return ChangeKind::Deleted;
    }
    if status_code.chars().any(|ch| ch == 'T') {
        return ChangeKind::TypeChanged;
    }
    ChangeKind::Modified
}

fn looks_like_porcelain_record(record: &str) -> bool {
    record.starts_with("# ")
        || record.starts_with("? ")
        || record.starts_with("! ")
        || record.starts_with("1 ")
        || record.starts_with("2 ")
        || record.starts_with("u ")
}

fn resolve_git_path(root: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value.trim());
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}

fn classify_failed_status(output: &GitCommandOutput) -> GitServiceState {
    let message = stderr_or_status(output).to_ascii_lowercase();
    if message.contains("not a git repository") {
        GitServiceState::NotRepository
    } else {
        GitServiceState::RefreshFailed
    }
}

fn stderr_or_status(output: &GitCommandOutput) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stderr.is_empty() {
        return stderr;
    }
    output
        .status_code
        .map(|code| format!("git exited with status {code}"))
        .unwrap_or_else(|| "git exited unsuccessfully".to_string())
}

fn consumer_refs_for(workspace_ref: &str) -> Vec<GitConsumerRef> {
    let id = sanitize_id(workspace_ref);
    [
        ("shell", "projection.git.shell"),
        ("activity_center", "projection.git.activity"),
        ("review", "projection.git.review_seed"),
        ("support_export", "projection.git.support"),
        ("cli", "projection.git.cli"),
    ]
    .into_iter()
    .map(|(surface, prefix)| GitConsumerRef {
        consumer_surface: surface.to_string(),
        projection_ref: format!("{prefix}.{id}"),
        authority_note: "local Git service snapshot is authoritative for repository status"
            .to_string(),
    })
    .collect()
}

fn snapshot_ref(snapshot: &GitStatusSnapshot) -> String {
    format!(
        "git.status.snapshot.{}.{}",
        sanitize_id(&snapshot.workspace_ref),
        sanitize_id(&snapshot.observed_at)
    )
}

fn short_oid(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .take(7)
        .collect()
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
    out
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
    fn parses_attached_branch_and_change_counts() {
        let raw = [
            b"# branch.oid 1111111111111111111111111111111111111111\0".as_slice(),
            b"# branch.head main\0".as_slice(),
            b"# branch.upstream origin/main\0".as_slice(),
            b"# branch.ab +2 -1\0".as_slice(),
            b"1 M. N... 100644 100644 100644 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb src/lib.rs\0".as_slice(),
            b"? notes/status.txt\0".as_slice(),
        ]
        .concat();
        let parsed = parse_porcelain_v2(&raw).expect("parse porcelain");
        assert_eq!(parsed.head.state, BranchState::Attached);
        assert_eq!(parsed.head.branch_label.as_deref(), Some("main"));
        assert_eq!(parsed.head.ahead, Some(2));
        assert_eq!(parsed.head.behind, Some(1));
        let summary = ChangeSummary::from_changes(&parsed.changes);
        assert_eq!(summary.staged_count, 1);
        assert_eq!(summary.untracked_count, 1);
        assert_eq!(summary.total_changed_count, 2);
    }

    #[test]
    fn parses_z_rename_with_original_path_as_next_record() {
        let raw = [
            b"# branch.oid 1111111111111111111111111111111111111111\0".as_slice(),
            b"# branch.head main\0".as_slice(),
            b"2 R. N... 100644 100644 100644 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb R100 new name.txt\0".as_slice(),
            b"old name.txt\0".as_slice(),
        ]
        .concat();
        let parsed = parse_porcelain_v2(&raw).expect("parse rename");
        assert_eq!(parsed.changes.len(), 1);
        let change = &parsed.changes[0];
        assert_eq!(change.change_kind, ChangeKind::Renamed);
        assert_eq!(change.path, PathBuf::from("new name.txt"));
        assert_eq!(change.original_path, Some(PathBuf::from("old name.txt")));
    }

    #[test]
    fn projections_share_truth_source() {
        let request =
            GitStatusRequest::with_observed_at("workspace.alpha", "/tmp/project", "mono:git:1");
        let mut snapshot = GitStatusSnapshot::degraded(
            &request,
            GitServiceState::NotRepository,
            "selected root is outside a Git worktree",
        );
        snapshot.change_summary.untracked_count = 1;
        let bundle = ConsumerProjectionBundle::from_snapshot("mono:git:2", &snapshot);
        assert_eq!(bundle.shell.truth_source_ref, bundle.truth_source_ref);
        assert_eq!(bundle.activity.truth_source_ref, bundle.truth_source_ref);
        assert_eq!(bundle.review.truth_source_ref, bundle.truth_source_ref);
        assert!(bundle.shell.current_claim_narrowed);
    }
}
