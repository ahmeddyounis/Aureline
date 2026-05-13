//! Preview-first commit flow with author, staged-scope, and publish-later truth.
//!
//! This module owns the bounded alpha contract for creating local commits from
//! the Git index. It reuses the canonical status snapshot for staged scope,
//! resolves the author identity before mutation, blocks ambiguous history-edit
//! requests, and emits result records that keep local commits distinct from any
//! later provider publication.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::status::{
    ChangeKind, ChangeSummary, ConsumerProjectionBundle, GitChange, GitServiceState,
    GitStatusRequest, GitStatusService,
};

/// Stable record-kind tag for [`GitCommitPreview`].
pub const GIT_COMMIT_PREVIEW_RECORD_KIND: &str = "git_commit_preview";

/// Stable record-kind tag for [`GitCommitResult`].
pub const GIT_COMMIT_RESULT_RECORD_KIND: &str = "git_commit_result";

/// Stable record-kind tag for [`GitCommitActivityRecord`].
pub const GIT_COMMIT_ACTIVITY_RECORD_KIND: &str = "git_commit_activity_record";

/// Stable record-kind tag for [`GitCommitSupportExportRecord`].
pub const GIT_COMMIT_SUPPORT_EXPORT_RECORD_KIND: &str = "git_commit_support_export_record";

/// Stable record-kind tag for [`GitCommitJournalRecord`].
pub const GIT_COMMIT_JOURNAL_RECORD_KIND: &str = "git_commit_journal_record";

/// Stable record-kind tag for [`GitCommitPublishReadinessRecord`].
pub const GIT_COMMIT_PUBLISH_READINESS_RECORD_KIND: &str = "git_commit_publish_readiness_record";

const GIT_COMMIT_PREVIEW_SCHEMA_VERSION: u32 = 1;
const GIT_COMMIT_RESULT_SCHEMA_VERSION: u32 = 1;
const GIT_COMMIT_ACTIVITY_SCHEMA_VERSION: u32 = 1;
const GIT_COMMIT_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;
const GIT_COMMIT_JOURNAL_SCHEMA_VERSION: u32 = 1;
const GIT_COMMIT_PUBLISH_READINESS_SCHEMA_VERSION: u32 = 1;

/// Commit mode requested by the local commit flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitCommitMode {
    /// Create a new local commit from the current index.
    Normal,
    /// Replace the current `HEAD` commit with a new commit.
    Amend,
    /// Create an autosquash marker commit targeting an existing commit.
    Squash,
}

impl GitCommitMode {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Amend => "amend",
            Self::Squash => "squash",
        }
    }

    /// Canonical command id for this commit mode.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Normal => "cmd:git.commit.create",
            Self::Amend => "cmd:git.commit.amend",
            Self::Squash => "cmd:git.commit.squash_marker",
        }
    }

    /// Reviewer-facing mode label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Normal => "Create commit",
            Self::Amend => "Amend current commit",
            Self::Squash => "Create squash marker commit",
        }
    }

    /// Returns true when this mode needs explicit history guardrail review.
    pub const fn requires_guardrail_ack(self) -> bool {
        matches!(self, Self::Amend | Self::Squash)
    }
}

/// State of a commit preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitCommitPreviewState {
    /// The preview can be applied if the staged scope remains unchanged.
    ReadyToCommit,
    /// The preview exists, but validation or guardrails block commit.
    Blocked,
    /// Local Git state is unavailable, so no commit may proceed.
    Degraded,
}

impl GitCommitPreviewState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToCommit => "ready_to_commit",
            Self::Blocked => "blocked",
            Self::Degraded => "degraded",
        }
    }
}

/// Final state of an attempted commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitCommitOutcomeState {
    /// Git created a local commit.
    Committed,
    /// No mutation was attempted because preview validation failed.
    BlockedNoChangesMade,
    /// Git returned a failure while attempting the commit.
    Failed,
}

impl GitCommitOutcomeState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::BlockedNoChangesMade => "blocked_no_changes_made",
            Self::Failed => "failed",
        }
    }

    fn activity_state_class(self) -> &'static str {
        match self {
            Self::Committed => "completed",
            Self::BlockedNoChangesMade => "blocked",
            Self::Failed => "failed",
        }
    }
}

/// Source used to resolve the commit author.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitCommitAuthorSource {
    /// The caller supplied an explicit author name and email.
    ExplicitRequest,
    /// The author was read from local Git configuration.
    RepositoryConfig,
    /// No usable author source was available.
    Missing,
}

impl GitCommitAuthorSource {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitRequest => "explicit_request",
            Self::RepositoryConfig => "repository_config",
            Self::Missing => "missing",
        }
    }
}

/// Validation state of the commit author identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitCommitAuthorState {
    /// A syntactically usable author identity is available.
    Resolved,
    /// No author identity could be found.
    Missing,
    /// An author identity was present but failed validation.
    Invalid,
}

impl GitCommitAuthorState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Missing => "missing",
            Self::Invalid => "invalid",
        }
    }
}

/// Actor identity attached to commit preview, apply, and support records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitActorRef {
    /// Actor class token from the local mutation lineage vocabulary.
    pub actor_class: String,
    /// Redaction-safe actor label.
    pub display_label: String,
    /// Optional stable principal or process ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
}

impl Default for GitCommitActorRef {
    fn default() -> Self {
        Self {
            actor_class: "local_user".to_string(),
            display_label: "Local user".to_string(),
            stable_id: None,
        }
    }
}

/// Author source requested by the caller.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitAuthorInput {
    /// Requested author display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Requested author email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl GitCommitAuthorInput {
    /// Resolves author identity from local Git configuration.
    pub fn repository_config() -> Self {
        Self::default()
    }

    /// Uses an explicit author identity for preview and commit.
    pub fn explicit(display_name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            display_name: Some(display_name.into()),
            email: Some(email.into()),
        }
    }

    fn is_explicit(&self) -> bool {
        self.display_name.is_some() || self.email.is_some()
    }
}

/// Request for a preview-first local commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitRequest {
    /// Stable workspace identity copied into downstream records.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub root_path: PathBuf,
    /// Commit mode requested by the caller.
    pub mode: GitCommitMode,
    /// Full commit message supplied by the caller.
    pub message: String,
    /// Author source requested by the caller.
    pub author: GitCommitAuthorInput,
    /// Actor that initiated the commit flow.
    pub actor: GitCommitActorRef,
    /// True when the caller explicitly acknowledged amend or squash guardrails.
    pub history_guardrail_acknowledged: bool,
    /// Commit-ish targeted by squash marker mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_target: Option<String>,
    /// Public row or surface ref that launched the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Timestamp supplied by the caller for deterministic exports.
    pub requested_at: String,
}

impl GitCommitRequest {
    /// Builds a normal commit request with a derived local workspace identity.
    pub fn for_message(root_path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
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
            mode: GitCommitMode::Normal,
            message: message.into(),
            author: GitCommitAuthorInput::repository_config(),
            actor: GitCommitActorRef::default(),
            history_guardrail_acknowledged: false,
            squash_target: None,
            launch_source_ref: None,
            requested_at: observed_at_now(),
        }
    }

    /// Builds a request with explicit identity, mode, and timestamp fields.
    pub fn with_observed_at(
        workspace_ref: impl Into<String>,
        root_path: impl Into<PathBuf>,
        mode: GitCommitMode,
        message: impl Into<String>,
        requested_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            root_path: root_path.into(),
            mode,
            message: message.into(),
            author: GitCommitAuthorInput::repository_config(),
            actor: GitCommitActorRef::default(),
            history_guardrail_acknowledged: false,
            squash_target: None,
            launch_source_ref: None,
            requested_at: requested_at.into(),
        }
    }

    /// Attaches an explicit author source to the request.
    pub fn with_author(mut self, author: GitCommitAuthorInput) -> Self {
        self.author = author;
        self
    }

    /// Attaches an actor identity to the request.
    pub fn with_actor(mut self, actor: GitCommitActorRef) -> Self {
        self.actor = actor;
        self
    }

    /// Marks the history guardrail as explicitly acknowledged.
    pub fn acknowledge_history_guardrail(mut self) -> Self {
        self.history_guardrail_acknowledged = true;
        self
    }

    /// Sets the target commit-ish for squash marker mode.
    pub fn with_squash_target(mut self, squash_target: impl Into<String>) -> Self {
        self.squash_target = Some(squash_target.into());
        self
    }

    /// Attaches a public launch-source ref to the request.
    pub fn with_launch_source_ref(mut self, launch_source_ref: impl Into<String>) -> Self {
        self.launch_source_ref = Some(launch_source_ref.into());
        self
    }
}

/// Resolved author identity shown before commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitAuthorIdentity {
    /// Stable author ref used by support and journal records.
    pub author_ref: String,
    /// Source used to resolve the identity.
    pub source: GitCommitAuthorSource,
    /// Validation state of the resolved identity.
    pub state: GitCommitAuthorState,
    /// Author display name when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Author email address when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Reviewer-facing label for the author row.
    pub display_label: String,
    /// Redaction class for support exports containing this identity.
    pub redaction_class: String,
    /// Validation or lookup reasons when the identity is not usable.
    pub failure_reasons: Vec<String>,
}

impl GitCommitAuthorIdentity {
    /// Returns true when the identity can be passed to Git.
    pub fn is_resolved(&self) -> bool {
        self.state == GitCommitAuthorState::Resolved
            && self.display_name.is_some()
            && self.email.is_some()
    }
}

/// Commit-message validation record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitMessageReview {
    /// Stable message-review ref.
    pub message_ref: String,
    /// Summary line extracted from the message.
    pub summary: String,
    /// Number of non-summary body lines.
    pub body_line_count: usize,
    /// True when the message can be used for commit.
    pub message_valid: bool,
    /// Validation failures that block commit.
    pub validation_errors: Vec<String>,
}

/// One staged path included in a commit preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitScopeTarget {
    /// Stable target ref used by activity, support, and journal records.
    pub target_ref: String,
    /// Path-truth ref that joins to status, change-list, and diff rows.
    pub path_truth_ref: String,
    /// Repository-relative path included from the Git index.
    pub repo_relative_path: PathBuf,
    /// Display-safe path label.
    pub path_label: String,
    /// Git status code observed at preview time.
    pub status_code: String,
    /// File-state token observed at preview time.
    pub file_state_token: String,
    /// True when the index side of this path is included in the commit.
    pub included_in_commit: bool,
}

/// Staged-scope review shown before commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitStagedScopeReview {
    /// Stable scope ref for the index tree being committed.
    pub scope_ref: String,
    /// Stable basis snapshot ref used to compute the staged scope.
    pub basis_snapshot_ref: String,
    /// Current Git index tree object observed at preview time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_tree_oid: Option<String>,
    /// Number of staged paths included in the commit.
    pub staged_count: usize,
    /// Number of unstaged tracked paths visible but not fully committed.
    pub unstaged_count: u32,
    /// Number of untracked paths visible but not committed.
    pub untracked_count: u32,
    /// Number of conflicted paths blocking commit.
    pub conflicted_count: u32,
    /// True when apply must not use a widened or recomputed path set.
    pub scope_rebind_forbidden: bool,
    /// Staged rows included from the index.
    pub staged_targets: Vec<GitCommitScopeTarget>,
    /// Changed paths that remain outside the commit scope.
    pub remaining_worktree_path_labels: Vec<String>,
}

impl GitCommitStagedScopeReview {
    /// Returns true when every staged row has visible path and status truth.
    pub fn staged_scope_is_visible(&self) -> bool {
        self.staged_targets.iter().all(|target| {
            !target.path_label.trim().is_empty()
                && !target.path_truth_ref.trim().is_empty()
                && !target.status_code.trim().is_empty()
                && target.included_in_commit
        })
    }
}

/// History-edit guardrail shown before commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitHistoryGuardrail {
    /// Stable guardrail ref.
    pub guardrail_ref: String,
    /// Commit mode being guarded.
    pub mode: GitCommitMode,
    /// Stable state token for the guardrail.
    pub guardrail_state: String,
    /// True when this mode requires explicit acknowledgement.
    pub explicit_ack_required: bool,
    /// True when the caller supplied the acknowledgement.
    pub explicit_ack_received: bool,
    /// True when the apply command rewrites the current `HEAD`.
    pub rewrites_existing_history: bool,
    /// True when squash behavior is deferred into an autosquash marker.
    pub deferred_squash_marker: bool,
    /// Full `HEAD` object id observed before commit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preflight_head_oid: Option<String>,
    /// Compact preflight `HEAD` ref for review and support rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preflight_head_ref: Option<String>,
    /// Resolved target commit ref for amend or squash behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_commit_ref: Option<String>,
    /// Recovery class advertised for this history-edit mode.
    pub recovery_class: String,
    /// Recovery ref or route available after apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_ref: Option<String>,
    /// Command id that opens the recovery or sequence-review route.
    pub recovery_command_id: String,
    /// Reasons that block this guardrail.
    pub blocked_reasons: Vec<String>,
}

impl GitCommitHistoryGuardrail {
    /// Returns true when the requested history mode is sufficiently explicit.
    pub fn is_satisfied(&self) -> bool {
        self.blocked_reasons.is_empty()
    }
}

/// Publish-later readiness record for local commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitPublishReadinessRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable publish-readiness ref.
    pub publish_readiness_ref: String,
    /// Phase token for preview or result.
    pub phase: String,
    /// Queue state for later publication.
    pub queue_state: String,
    /// True when the commit operation itself is local only.
    pub local_only_commit: bool,
    /// True when this alpha path supports publishing immediately.
    pub publish_now_supported: bool,
    /// Branch label when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Upstream ref when configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref: Option<String>,
    /// Local commit ref after a successful result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_commit_ref: Option<String>,
    /// Provider overlay state for this local-first alpha path.
    pub provider_overlay_state: String,
    /// Reasons publication is not ready or must be reviewed later.
    pub blockers: Vec<String>,
    /// Command id that opens the publish-later review queue.
    pub next_command_id: String,
    /// Redaction-safe handoff packet ref for support or browser flows.
    pub handoff_packet_ref: String,
}

/// Activity-center projection for commit preview or result state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitActivityRecord {
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
    /// Commit journal id when a mutation was attempted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_journal_ref: Option<String>,
    /// Command id that reopens commit details.
    pub open_details_command_id: String,
    /// Support-export ref that carries the same attribution.
    pub support_export_ref: String,
}

/// Redaction-safe support export projection for a commit flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitSupportExportRecord {
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
    /// Commit mode token.
    pub commit_mode: String,
    /// Phase token for preview or apply.
    pub phase: String,
    /// Workspace identity copied from the preview.
    pub workspace_ref: String,
    /// Scope ref copied from the preview.
    pub scope_ref: String,
    /// Author ref copied from the preview.
    pub author_ref: String,
    /// Preview ref copied from the preview.
    pub preview_ref: String,
    /// Result ref when a commit completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Commit journal ref when a commit completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_journal_ref: Option<String>,
    /// Publish-readiness ref for local-vs-publish joins.
    pub publish_readiness_ref: String,
    /// Evidence refs included without raw patch bodies.
    pub evidence_refs: Vec<String>,
    /// Fields deliberately omitted from export.
    pub omitted_fields: Vec<String>,
}

/// Mutation-journal shaped record emitted after commit apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitJournalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable journal ref.
    pub commit_journal_ref: String,
    /// Canonical command id that mutated Git state.
    pub command_id: String,
    /// Actor that initiated the commit.
    pub actor: GitCommitActorRef,
    /// Author identity used for the Git commit.
    pub author: GitCommitAuthorIdentity,
    /// Source class for the mutation journal.
    pub source_class: String,
    /// Commit mode token.
    pub commit_mode: String,
    /// Scope ref copied from the preview.
    pub scope_ref: String,
    /// Target refs copied from staged rows.
    pub target_refs: Vec<String>,
    /// Timestamp when preview started.
    pub started_at: String,
    /// Timestamp when apply resolved.
    pub resolved_at: String,
    /// Local commit object id when Git created one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
    /// Reversal or recovery class advertised for this commit.
    pub recovery_class: String,
    /// Publish-readiness ref linked to this commit.
    pub publish_readiness_ref: String,
    /// Redaction class for support exports.
    pub redaction_class: String,
    /// Redaction-safe side-effect summary.
    pub side_effect_summary: String,
}

/// Review-first preview packet for a local Git commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitPreview {
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
    /// Commit mode being reviewed.
    pub mode: GitCommitMode,
    /// Canonical command id for apply.
    pub command_id: String,
    /// Reviewer-facing operation label.
    pub operation_label: String,
    /// Current preview state.
    pub preview_state: GitCommitPreviewState,
    /// Actor that initiated the preview.
    pub actor: GitCommitActorRef,
    /// Public row or surface ref that launched the preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Author identity disclosed before apply.
    pub author: GitCommitAuthorIdentity,
    /// Commit-message validation.
    pub message: GitCommitMessageReview,
    /// Staged-scope review shown before apply.
    pub scope: GitCommitStagedScopeReview,
    /// Amend or squash guardrail state.
    pub history_guardrail: GitCommitHistoryGuardrail,
    /// Publish-later readiness before local commit creation.
    pub publish_readiness: GitCommitPublishReadinessRecord,
    /// Activity projection for the preview state.
    pub activity: GitCommitActivityRecord,
    /// Support-export projection for the preview state.
    pub support_export: GitCommitSupportExportRecord,
    /// Reasons that block commit from this preview.
    pub blocked_reasons: Vec<String>,
    #[serde(skip)]
    message_body: String,
    #[serde(skip)]
    squash_target_for_apply: Option<String>,
}

impl GitCommitPreview {
    /// Returns true when commit may proceed if the index tree still matches.
    pub fn ready_to_commit(&self) -> bool {
        self.preview_state == GitCommitPreviewState::ReadyToCommit
            && self.blocked_reasons.is_empty()
            && self.author.is_resolved()
            && self.message.message_valid
            && self.scope.staged_count > 0
            && self.scope.staged_scope_is_visible()
            && self.scope.index_tree_oid.is_some()
            && self.history_guardrail.is_satisfied()
    }
}

/// Result packet emitted after applying or blocking a commit preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitCommitResult {
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
    /// Commit mode that was applied.
    pub mode: GitCommitMode,
    /// Final outcome state.
    pub outcome_state: GitCommitOutcomeState,
    /// Local commit object id when Git created one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
    /// Compact local commit object id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_short_oid: Option<String>,
    /// Paths included in the attempted commit.
    pub committed_targets: Vec<GitCommitScopeTarget>,
    /// Reasons that blocked or failed the commit.
    pub blocked_reasons: Vec<String>,
    /// Author identity used for the Git commit.
    pub author: GitCommitAuthorIdentity,
    /// Staged scope from the preview.
    pub scope: GitCommitStagedScopeReview,
    /// History guardrail copied from the preview.
    pub history_guardrail: GitCommitHistoryGuardrail,
    /// Publish-later readiness after the result.
    pub publish_readiness: GitCommitPublishReadinessRecord,
    /// Mutation-journal shaped lineage record.
    pub commit_journal: GitCommitJournalRecord,
    /// Activity projection for the result.
    pub activity: GitCommitActivityRecord,
    /// Support-export projection for the result.
    pub support_export: GitCommitSupportExportRecord,
    /// Failure reason when apply failed after starting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

impl GitCommitResult {
    /// Returns true when activity and support rows cite the journal record.
    pub fn attribution_is_exportable(&self) -> bool {
        self.activity.commit_journal_ref.as_deref()
            == Some(self.commit_journal.commit_journal_ref.as_str())
            && self.support_export.commit_journal_ref.as_deref()
                == Some(self.commit_journal.commit_journal_ref.as_str())
            && self.support_export.publish_readiness_ref
                == self.publish_readiness.publish_readiness_ref
    }
}

/// Output captured from a Git commit command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitCommitCommandOutput {
    /// True when Git exited successfully.
    pub success: bool,
    /// Process exit status code when available.
    pub status_code: Option<i32>,
    /// Captured stdout bytes.
    pub stdout: Vec<u8>,
    /// Captured stderr bytes.
    pub stderr: Vec<u8>,
}

/// Error raised before a Git commit command can be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitCommitBackendError {
    /// Redaction-safe error message.
    pub message: String,
}

impl std::fmt::Display for GitCommitBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GitCommitBackendError {}

/// Backend used by [`GitCommitService`] to execute local Git commands.
pub trait GitCommitBackend {
    /// Runs `git -C root args`.
    ///
    /// # Errors
    ///
    /// Returns [`GitCommitBackendError`] when the backend cannot launch or
    /// supervise the Git process.
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitCommitCommandOutput, GitCommitBackendError>;
}

/// Git backend that shells out to the system `git` executable.
#[derive(Debug, Clone)]
pub struct SystemGitCommitBackend {
    git_binary: PathBuf,
}

impl Default for SystemGitCommitBackend {
    fn default() -> Self {
        Self::new("git")
    }
}

impl SystemGitCommitBackend {
    /// Creates a backend that invokes `git_binary`.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl GitCommitBackend for SystemGitCommitBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitCommitCommandOutput, GitCommitBackendError> {
        let output = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|err| GitCommitBackendError {
                message: format!("git command failed to launch: {err}"),
            })?;
        Ok(GitCommitCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

/// Service that creates and applies local Git commit previews.
#[derive(Debug, Clone)]
pub struct GitCommitService<B = SystemGitCommitBackend> {
    backend: B,
}

impl Default for GitCommitService<SystemGitCommitBackend> {
    fn default() -> Self {
        Self::new(SystemGitCommitBackend::default())
    }
}

impl<B: GitCommitBackend> GitCommitService<B> {
    /// Creates a service backed by `backend`.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Builds a reviewable commit preview without mutating Git state.
    pub fn preview(&self, request: &GitCommitRequest) -> GitCommitPreview {
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
        let preview_ref = preview_ref(&request.workspace_ref, request.mode, &request.requested_at);

        if snapshot.service_state != GitServiceState::Current {
            return self.degraded_preview(request, repo_root, truth_source_ref, preview_ref);
        }

        let author = self.author_identity(&repo_root, &request.workspace_ref, &request.author);
        let message = message_review_for(&preview_ref, &request.message);
        let index_tree_oid = self.index_tree_oid(&repo_root);
        let scope = staged_scope_for(
            &request.workspace_ref,
            &preview_ref,
            &truth_source_ref,
            index_tree_oid,
            &snapshot.changes,
            &snapshot.change_summary,
        );
        let history_guardrail = self.history_guardrail_for(
            request,
            &preview_ref,
            &repo_root,
            snapshot.head.head_oid.as_deref(),
        );
        let mut blocked_reasons = blocked_reasons_for(&author, &message, &scope);
        blocked_reasons.extend(history_guardrail.blocked_reasons.clone());
        let preview_state = if blocked_reasons.is_empty() {
            GitCommitPreviewState::ReadyToCommit
        } else {
            GitCommitPreviewState::Blocked
        };
        let publish_readiness = publish_readiness_for_preview(
            &preview_ref,
            request.mode,
            snapshot.head.branch_label.clone(),
            snapshot.head.upstream.clone(),
        );
        let support_export = support_export_for_preview(
            &preview_ref,
            request,
            &author,
            &scope,
            &history_guardrail,
            &publish_readiness,
        );
        let activity = activity_for_preview(
            &preview_ref,
            request.mode,
            preview_state,
            &scope,
            &support_export.support_export_ref,
        );

        GitCommitPreview {
            record_kind: GIT_COMMIT_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_COMMIT_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            mode: request.mode,
            command_id: request.mode.command_id().to_string(),
            operation_label: request.mode.label().to_string(),
            preview_state,
            actor: request.actor.clone(),
            launch_source_ref: request.launch_source_ref.clone(),
            author,
            message,
            scope,
            history_guardrail,
            publish_readiness,
            activity,
            support_export,
            blocked_reasons,
            message_body: request.message.clone(),
            squash_target_for_apply: request.squash_target.clone(),
        }
    }

    /// Applies an admitted preview and returns an attributable result packet.
    pub fn apply(
        &self,
        preview: &GitCommitPreview,
        resolved_at: impl Into<String>,
    ) -> GitCommitResult {
        let resolved_at = resolved_at.into();
        if !preview.ready_to_commit() {
            return result_for_preview(
                preview,
                &resolved_at,
                GitCommitOutcomeState::BlockedNoChangesMade,
                None,
                None,
                preview.blocked_reasons.clone(),
            );
        }

        if self.index_tree_oid(&preview.repo_root) != preview.scope.index_tree_oid {
            return result_for_preview(
                preview,
                &resolved_at,
                GitCommitOutcomeState::BlockedNoChangesMade,
                None,
                Some("staged scope drifted after preview; reopen commit review".to_string()),
                vec!["staged scope drifted after preview".to_string()],
            );
        }

        let output = self.apply_preview(preview);
        let (outcome_state, failure_reason, commit_oid) = match output {
            Ok(output) if output.success => {
                let commit_oid = self.head_oid(&preview.repo_root);
                (GitCommitOutcomeState::Committed, None, commit_oid)
            }
            Ok(output) => (
                GitCommitOutcomeState::Failed,
                Some(stderr_or_status(&output)),
                None,
            ),
            Err(err) => (GitCommitOutcomeState::Failed, Some(err.message), None),
        };
        let blocked_reasons = failure_reason.clone().into_iter().collect();
        result_for_preview(
            preview,
            &resolved_at,
            outcome_state,
            commit_oid,
            failure_reason,
            blocked_reasons,
        )
    }

    fn degraded_preview(
        &self,
        request: &GitCommitRequest,
        repo_root: PathBuf,
        truth_source_ref: String,
        preview_ref: String,
    ) -> GitCommitPreview {
        let author = explicit_or_missing_author(&request.workspace_ref, &request.author);
        let message = message_review_for(&preview_ref, &request.message);
        let scope = GitCommitStagedScopeReview {
            scope_ref: format!("{}.scope", preview_ref),
            basis_snapshot_ref: truth_source_ref.clone(),
            index_tree_oid: None,
            staged_count: 0,
            unstaged_count: 0,
            untracked_count: 0,
            conflicted_count: 0,
            scope_rebind_forbidden: true,
            staged_targets: Vec::new(),
            remaining_worktree_path_labels: Vec::new(),
        };
        let history_guardrail = GitCommitHistoryGuardrail {
            guardrail_ref: format!("{}.history_guardrail", preview_ref),
            mode: request.mode,
            guardrail_state: "degraded".to_string(),
            explicit_ack_required: request.mode.requires_guardrail_ack(),
            explicit_ack_received: request.history_guardrail_acknowledged,
            rewrites_existing_history: request.mode == GitCommitMode::Amend,
            deferred_squash_marker: request.mode == GitCommitMode::Squash,
            preflight_head_oid: None,
            preflight_head_ref: None,
            target_commit_ref: None,
            recovery_class: "unavailable".to_string(),
            recovery_ref: None,
            recovery_command_id: "cmd:git.history.inspect".to_string(),
            blocked_reasons: vec!["Git service degraded before commit preview".to_string()],
        };
        let publish_readiness =
            publish_readiness_for_preview(&preview_ref, request.mode, None, None);
        let support_export = support_export_for_preview(
            &preview_ref,
            request,
            &author,
            &scope,
            &history_guardrail,
            &publish_readiness,
        );
        let activity = activity_for_preview(
            &preview_ref,
            request.mode,
            GitCommitPreviewState::Degraded,
            &scope,
            &support_export.support_export_ref,
        );
        GitCommitPreview {
            record_kind: GIT_COMMIT_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_COMMIT_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            mode: request.mode,
            command_id: request.mode.command_id().to_string(),
            operation_label: request.mode.label().to_string(),
            preview_state: GitCommitPreviewState::Degraded,
            actor: request.actor.clone(),
            launch_source_ref: request.launch_source_ref.clone(),
            author,
            message,
            scope,
            history_guardrail,
            publish_readiness,
            activity,
            support_export,
            blocked_reasons: vec!["Git service degraded before commit preview".to_string()],
            message_body: request.message.clone(),
            squash_target_for_apply: request.squash_target.clone(),
        }
    }

    fn author_identity(
        &self,
        repo_root: &Path,
        workspace_ref: &str,
        input: &GitCommitAuthorInput,
    ) -> GitCommitAuthorIdentity {
        if input.is_explicit() {
            return explicit_or_missing_author(workspace_ref, input);
        }
        let name = self.git_config_value(repo_root, "user.name");
        let email = self.git_config_value(repo_root, "user.email");
        author_identity_from_parts(
            workspace_ref,
            GitCommitAuthorSource::RepositoryConfig,
            name,
            email,
        )
    }

    fn git_config_value(&self, repo_root: &Path, key: &str) -> Option<String> {
        let args = ["config", "--get", key]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let output = self.backend.run_git(repo_root, &args).ok()?;
        if !output.success {
            return None;
        }
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        (!value.is_empty()).then_some(value)
    }

    fn history_guardrail_for(
        &self,
        request: &GitCommitRequest,
        preview_ref: &str,
        repo_root: &Path,
        head_oid: Option<&str>,
    ) -> GitCommitHistoryGuardrail {
        let explicit_ack_required = request.mode.requires_guardrail_ack();
        let mut blocked_reasons = Vec::new();
        if explicit_ack_required && !request.history_guardrail_acknowledged {
            blocked_reasons.push(format!(
                "{} requires explicit history guardrail acknowledgement",
                request.mode.label()
            ));
        }

        let mut target_commit_ref = None;
        if request.mode == GitCommitMode::Amend {
            if head_oid.is_none() {
                blocked_reasons.push("amend requires an existing HEAD commit".to_string());
            }
            target_commit_ref = head_oid.map(commit_ref);
        } else if request.mode == GitCommitMode::Squash {
            match request.squash_target.as_deref().map(str::trim) {
                Some(target) if !target.is_empty() => {
                    match self.resolve_commit_oid(repo_root, target) {
                        Some(oid) => target_commit_ref = Some(commit_ref(&oid)),
                        None => blocked_reasons
                            .push("squash target commit could not be verified".to_string()),
                    }
                }
                _ => blocked_reasons.push("squash target commit is required".to_string()),
            }
        }

        let preflight_head_ref = head_oid.map(commit_ref);
        let guardrail_state = if blocked_reasons.is_empty() {
            if explicit_ack_required {
                "acknowledged"
            } else {
                "not_required"
            }
        } else {
            "blocked"
        };
        GitCommitHistoryGuardrail {
            guardrail_ref: format!("{}.history_guardrail", preview_ref),
            mode: request.mode,
            guardrail_state: guardrail_state.to_string(),
            explicit_ack_required,
            explicit_ack_received: request.history_guardrail_acknowledged,
            rewrites_existing_history: request.mode == GitCommitMode::Amend,
            deferred_squash_marker: request.mode == GitCommitMode::Squash,
            preflight_head_oid: head_oid.map(str::to_string),
            preflight_head_ref,
            target_commit_ref,
            recovery_class: match request.mode {
                GitCommitMode::Normal => "new_commit_revert_or_reset".to_string(),
                GitCommitMode::Amend => "reflog_previous_head".to_string(),
                GitCommitMode::Squash => "autosquash_sequence_review_required".to_string(),
            },
            recovery_ref: head_oid.map(|oid| format!("git.reflog.previous.{}", short_oid(oid))),
            recovery_command_id: match request.mode {
                GitCommitMode::Squash => "cmd:git.history.sequence_review".to_string(),
                _ => "cmd:git.history.inspect_reflog".to_string(),
            },
            blocked_reasons,
        }
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

    fn index_tree_oid(&self, repo_root: &Path) -> Option<String> {
        let args = vec!["write-tree".to_string()];
        let output = self.backend.run_git(repo_root, &args).ok()?;
        if !output.success {
            return None;
        }
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        (!value.is_empty()).then_some(value)
    }

    fn head_oid(&self, repo_root: &Path) -> Option<String> {
        self.resolve_commit_oid(repo_root, "HEAD")
    }

    fn apply_preview(
        &self,
        preview: &GitCommitPreview,
    ) -> Result<GitCommitCommandOutput, GitCommitBackendError> {
        let name = preview
            .author
            .display_name
            .as_deref()
            .ok_or_else(|| GitCommitBackendError {
                message: "commit author name missing".to_string(),
            })?;
        let email = preview
            .author
            .email
            .as_deref()
            .ok_or_else(|| GitCommitBackendError {
                message: "commit author email missing".to_string(),
            })?;
        let mut args = vec![
            "-c".to_string(),
            format!("user.name={name}"),
            "-c".to_string(),
            format!("user.email={email}"),
            "commit".to_string(),
            "--author".to_string(),
            format!("{name} <{email}>"),
        ];
        match preview.mode {
            GitCommitMode::Normal => {}
            GitCommitMode::Amend => args.push("--amend".to_string()),
            GitCommitMode::Squash => {
                let target = preview.squash_target_for_apply.as_deref().ok_or_else(|| {
                    GitCommitBackendError {
                        message: "squash target missing".to_string(),
                    }
                })?;
                args.push(format!("--squash={target}"));
            }
        }
        args.push("-m".to_string());
        args.push(preview.message_body.clone());
        self.backend.run_git(&preview.repo_root, &args)
    }
}

fn explicit_or_missing_author(
    workspace_ref: &str,
    input: &GitCommitAuthorInput,
) -> GitCommitAuthorIdentity {
    if input.is_explicit() {
        author_identity_from_parts(
            workspace_ref,
            GitCommitAuthorSource::ExplicitRequest,
            input.display_name.clone(),
            input.email.clone(),
        )
    } else {
        author_identity_from_parts(workspace_ref, GitCommitAuthorSource::Missing, None, None)
    }
}

fn author_identity_from_parts(
    workspace_ref: &str,
    source: GitCommitAuthorSource,
    display_name: Option<String>,
    email: Option<String>,
) -> GitCommitAuthorIdentity {
    let display_name = display_name.map(|value| value.trim().to_string());
    let email = email.map(|value| value.trim().to_string());
    let mut failure_reasons = Vec::new();
    match display_name.as_deref() {
        Some(name) if valid_author_name(name) => {}
        Some(_) => failure_reasons.push("author name is invalid".to_string()),
        None => failure_reasons.push("author name is missing".to_string()),
    }
    match email.as_deref() {
        Some(email) if valid_author_email(email) => {}
        Some(_) => failure_reasons.push("author email is invalid".to_string()),
        None => failure_reasons.push("author email is missing".to_string()),
    }
    let state = if failure_reasons.is_empty() {
        GitCommitAuthorState::Resolved
    } else if display_name.is_none() && email.is_none() {
        GitCommitAuthorState::Missing
    } else {
        GitCommitAuthorState::Invalid
    };
    let display_label = match (display_name.as_deref(), email.as_deref(), state) {
        (Some(name), Some(email), GitCommitAuthorState::Resolved) => format!("{name} <{email}>"),
        _ => "Commit author unavailable".to_string(),
    };
    GitCommitAuthorIdentity {
        author_ref: format!(
            "git.commit.author.{}.{}",
            sanitize_id(workspace_ref),
            sanitize_id(source.as_str())
        ),
        source,
        state,
        display_name,
        email,
        display_label,
        redaction_class: "commit_metadata".to_string(),
        failure_reasons,
    }
}

fn valid_author_name(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && !trimmed
            .chars()
            .any(|ch| matches!(ch, '\n' | '\r' | '<' | '>'))
        && trimmed.chars().all(|ch| !ch.is_control())
}

fn valid_author_email(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed.contains('@')
        && !trimmed
            .chars()
            .any(|ch| matches!(ch, '\n' | '\r' | '<' | '>'))
        && !trimmed.chars().any(char::is_whitespace)
        && trimmed.chars().all(|ch| !ch.is_control())
}

fn message_review_for(preview_ref: &str, message: &str) -> GitCommitMessageReview {
    let summary = message.lines().next().unwrap_or("").trim().to_string();
    let body_line_count = message.lines().skip(1).count();
    let mut validation_errors = Vec::new();
    if summary.is_empty() {
        validation_errors.push("commit summary is required".to_string());
    }
    if summary.chars().count() > 120 {
        validation_errors.push("commit summary must be 120 characters or fewer".to_string());
    }
    GitCommitMessageReview {
        message_ref: format!("{}.message", preview_ref),
        summary,
        body_line_count,
        message_valid: validation_errors.is_empty(),
        validation_errors,
    }
}

fn staged_scope_for(
    workspace_ref: &str,
    preview_ref: &str,
    truth_source_ref: &str,
    index_tree_oid: Option<String>,
    changes: &[GitChange],
    summary: &ChangeSummary,
) -> GitCommitStagedScopeReview {
    let staged_targets = changes
        .iter()
        .filter(|change| change.is_staged)
        .map(|change| commit_scope_target(workspace_ref, change))
        .collect::<Vec<_>>();
    let remaining_worktree_path_labels = changes
        .iter()
        .filter(|change| {
            change.is_unstaged
                || change.change_kind == ChangeKind::Untracked
                || change.is_conflicted
        })
        .map(|change| change.path.to_string_lossy().to_string())
        .collect::<Vec<_>>();
    GitCommitStagedScopeReview {
        scope_ref: format!("{}.scope", preview_ref),
        basis_snapshot_ref: truth_source_ref.to_string(),
        index_tree_oid,
        staged_count: staged_targets.len(),
        unstaged_count: summary.unstaged_count,
        untracked_count: summary.untracked_count,
        conflicted_count: summary.conflicted_count,
        scope_rebind_forbidden: true,
        staged_targets,
        remaining_worktree_path_labels,
    }
}

fn commit_scope_target(workspace_ref: &str, change: &GitChange) -> GitCommitScopeTarget {
    let path_label = change.path.to_string_lossy().to_string();
    let workspace_id = sanitize_id(workspace_ref);
    let path_id = sanitize_id(&path_label);
    GitCommitScopeTarget {
        target_ref: format!("git.commit.target.{workspace_id}.{path_id}"),
        path_truth_ref: format!("path.truth.git.commit.{workspace_id}.{path_id}"),
        repo_relative_path: change.path.clone(),
        path_label,
        status_code: change.status_code.clone(),
        file_state_token: file_state_token(change).to_string(),
        included_in_commit: true,
    }
}

fn file_state_token(change: &GitChange) -> &'static str {
    if change.is_conflicted {
        return "conflicted";
    }
    match change.change_kind {
        ChangeKind::Modified => "modified",
        ChangeKind::Added => "added",
        ChangeKind::Deleted => "deleted",
        ChangeKind::TypeChanged => "type_changed",
        ChangeKind::Renamed => "renamed",
        ChangeKind::Copied => "copied",
        ChangeKind::Untracked => "untracked",
        ChangeKind::Ignored => "ignored",
        ChangeKind::Conflict => "conflicted",
    }
}

fn blocked_reasons_for(
    author: &GitCommitAuthorIdentity,
    message: &GitCommitMessageReview,
    scope: &GitCommitStagedScopeReview,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if !author.is_resolved() {
        reasons.extend(author.failure_reasons.clone());
    }
    if !message.message_valid {
        reasons.extend(message.validation_errors.clone());
    }
    if scope.staged_count == 0 {
        reasons.push("no staged changes are available to commit".to_string());
    }
    if scope.conflicted_count > 0 {
        reasons.push("conflicted paths require conflict review before commit".to_string());
    }
    if scope.staged_count > 0 && scope.index_tree_oid.is_none() {
        reasons.push("Git index tree could not be captured for scope drift detection".to_string());
    }
    reasons
}

fn publish_readiness_for_preview(
    preview_ref: &str,
    mode: GitCommitMode,
    branch_label: Option<String>,
    upstream_ref: Option<String>,
) -> GitCommitPublishReadinessRecord {
    let mut blockers = vec!["local commit has not been created yet".to_string()];
    if upstream_ref.is_none() {
        blockers.push("no upstream branch is configured".to_string());
    }
    if mode == GitCommitMode::Amend {
        blockers.push("amended history requires publish review before push".to_string());
    }
    if mode == GitCommitMode::Squash {
        blockers.push("squash marker requires sequence review before publish".to_string());
    }
    GitCommitPublishReadinessRecord {
        record_kind: GIT_COMMIT_PUBLISH_READINESS_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_PUBLISH_READINESS_SCHEMA_VERSION,
        publish_readiness_ref: format!("{}.publish_later", preview_ref),
        phase: "preview".to_string(),
        queue_state: if mode == GitCommitMode::Squash {
            "blocked_for_sequence_review"
        } else if upstream_ref.is_none() {
            "needs_remote_or_branch_selection"
        } else {
            "pending_local_commit"
        }
        .to_string(),
        local_only_commit: true,
        publish_now_supported: false,
        branch_label,
        upstream_ref,
        local_commit_ref: None,
        provider_overlay_state: "not_configured_alpha".to_string(),
        blockers,
        next_command_id: "cmd:git.publish_later.review".to_string(),
        handoff_packet_ref: format!("handoff.{}", sanitize_id(preview_ref)),
    }
}

fn publish_readiness_for_result(
    result_ref: &str,
    preview: &GitCommitPreview,
    outcome_state: GitCommitOutcomeState,
    commit_oid: Option<&str>,
) -> GitCommitPublishReadinessRecord {
    let upstream_ref = preview.publish_readiness.upstream_ref.clone();
    let mut blockers = Vec::new();
    let queue_state = if outcome_state != GitCommitOutcomeState::Committed {
        blockers.push("local commit did not complete".to_string());
        "blocked_until_commit_succeeds"
    } else if preview.mode == GitCommitMode::Squash {
        blockers.push("squash marker requires sequence review before publish".to_string());
        "blocked_for_sequence_review"
    } else if preview.mode == GitCommitMode::Amend && upstream_ref.is_some() {
        blockers.push("amended history requires publish review before push".to_string());
        "ready_for_guarded_publish_review"
    } else if upstream_ref.is_none() {
        if preview.mode == GitCommitMode::Amend {
            blockers.push("amended history requires publish review before push".to_string());
        }
        blockers.push("no upstream branch is configured".to_string());
        "needs_remote_or_branch_selection"
    } else {
        "ready_for_manual_publish_review"
    };
    GitCommitPublishReadinessRecord {
        record_kind: GIT_COMMIT_PUBLISH_READINESS_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_PUBLISH_READINESS_SCHEMA_VERSION,
        publish_readiness_ref: format!("{}.publish_later", result_ref),
        phase: "result".to_string(),
        queue_state: queue_state.to_string(),
        local_only_commit: true,
        publish_now_supported: false,
        branch_label: preview.publish_readiness.branch_label.clone(),
        upstream_ref,
        local_commit_ref: commit_oid.map(commit_ref),
        provider_overlay_state: "not_configured_alpha".to_string(),
        blockers,
        next_command_id: "cmd:git.publish_later.review".to_string(),
        handoff_packet_ref: format!("handoff.{}", sanitize_id(result_ref)),
    }
}

fn activity_for_preview(
    preview_ref: &str,
    mode: GitCommitMode,
    preview_state: GitCommitPreviewState,
    scope: &GitCommitStagedScopeReview,
    support_export_ref: &str,
) -> GitCommitActivityRecord {
    let state_class = match preview_state {
        GitCommitPreviewState::ReadyToCommit => "waiting_review",
        GitCommitPreviewState::Blocked => "blocked",
        GitCommitPreviewState::Degraded => "degraded",
    };
    GitCommitActivityRecord {
        record_kind: GIT_COMMIT_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(preview_ref)),
        job_family: "git_commit".to_string(),
        state_class: state_class.to_string(),
        partition: if preview_state == GitCommitPreviewState::ReadyToCommit {
            "active_review"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: format!("{} preview", mode.label()),
        detail_label: format!(
            "{}; {} staged path(s)",
            preview_state.as_str(),
            scope.staged_count
        ),
        preview_ref: preview_ref.to_string(),
        commit_journal_ref: None,
        open_details_command_id: "cmd:git.commit.open_details".to_string(),
        support_export_ref: support_export_ref.to_string(),
    }
}

fn activity_for_result(
    result_ref: &str,
    preview: &GitCommitPreview,
    outcome_state: GitCommitOutcomeState,
    commit_journal_ref: &str,
    support_export_ref: &str,
    publish_queue_state: &str,
) -> GitCommitActivityRecord {
    GitCommitActivityRecord {
        record_kind: GIT_COMMIT_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(result_ref)),
        job_family: "git_commit".to_string(),
        state_class: outcome_state.activity_state_class().to_string(),
        partition: if outcome_state == GitCommitOutcomeState::Committed {
            "completed"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: format!("{} {}", preview.mode.label(), outcome_state.as_str()),
        detail_label: format!(
            "{} staged path(s); publish later {}",
            preview.scope.staged_count, publish_queue_state
        ),
        preview_ref: preview.preview_ref.clone(),
        commit_journal_ref: Some(commit_journal_ref.to_string()),
        open_details_command_id: "cmd:git.commit.open_details".to_string(),
        support_export_ref: support_export_ref.to_string(),
    }
}

fn support_export_for_preview(
    preview_ref: &str,
    request: &GitCommitRequest,
    author: &GitCommitAuthorIdentity,
    scope: &GitCommitStagedScopeReview,
    history_guardrail: &GitCommitHistoryGuardrail,
    publish_readiness: &GitCommitPublishReadinessRecord,
) -> GitCommitSupportExportRecord {
    GitCommitSupportExportRecord {
        record_kind: GIT_COMMIT_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.export.{}", sanitize_id(preview_ref)),
        redaction_mode: "metadata_safe_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        commit_mode: request.mode.as_str().to_string(),
        phase: "preview".to_string(),
        workspace_ref: request.workspace_ref.clone(),
        scope_ref: scope.scope_ref.clone(),
        author_ref: author.author_ref.clone(),
        preview_ref: preview_ref.to_string(),
        result_ref: None,
        commit_journal_ref: None,
        publish_readiness_ref: publish_readiness.publish_readiness_ref.clone(),
        evidence_refs: vec![
            scope.scope_ref.clone(),
            author.author_ref.clone(),
            history_guardrail.guardrail_ref.clone(),
            publish_readiness.publish_readiness_ref.clone(),
        ],
        omitted_fields: vec![
            "raw_patch_body".to_string(),
            "raw_command_line".to_string(),
            "raw_actor_secret".to_string(),
        ],
    }
}

fn support_export_for_result(
    result_ref: &str,
    preview: &GitCommitPreview,
    commit_journal_ref: &str,
    publish_readiness: &GitCommitPublishReadinessRecord,
) -> GitCommitSupportExportRecord {
    GitCommitSupportExportRecord {
        record_kind: GIT_COMMIT_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.export.{}", sanitize_id(result_ref)),
        redaction_mode: "metadata_safe_default".to_string(),
        retention_class: "local_recovery_audit".to_string(),
        commit_mode: preview.mode.as_str().to_string(),
        phase: "apply".to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        scope_ref: preview.scope.scope_ref.clone(),
        author_ref: preview.author.author_ref.clone(),
        preview_ref: preview.preview_ref.clone(),
        result_ref: Some(result_ref.to_string()),
        commit_journal_ref: Some(commit_journal_ref.to_string()),
        publish_readiness_ref: publish_readiness.publish_readiness_ref.clone(),
        evidence_refs: vec![
            preview.scope.scope_ref.clone(),
            preview.author.author_ref.clone(),
            preview.history_guardrail.guardrail_ref.clone(),
            commit_journal_ref.to_string(),
            publish_readiness.publish_readiness_ref.clone(),
        ],
        omitted_fields: vec![
            "raw_patch_body".to_string(),
            "raw_command_line".to_string(),
            "raw_actor_secret".to_string(),
        ],
    }
}

fn result_for_preview(
    preview: &GitCommitPreview,
    resolved_at: &str,
    outcome_state: GitCommitOutcomeState,
    commit_oid: Option<String>,
    failure_reason: Option<String>,
    blocked_reasons: Vec<String>,
) -> GitCommitResult {
    let result_ref = format!(
        "{}.result.{}",
        preview.preview_ref,
        sanitize_id(resolved_at)
    );
    let commit_journal_ref = format!("git.commit.journal.{}", sanitize_id(&result_ref));
    let publish_readiness =
        publish_readiness_for_result(&result_ref, preview, outcome_state, commit_oid.as_deref());
    let support_export = support_export_for_result(
        &result_ref,
        preview,
        &commit_journal_ref,
        &publish_readiness,
    );
    let activity = activity_for_result(
        &result_ref,
        preview,
        outcome_state,
        &commit_journal_ref,
        &support_export.support_export_ref,
        &publish_readiness.queue_state,
    );
    let commit_journal = GitCommitJournalRecord {
        record_kind: GIT_COMMIT_JOURNAL_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_JOURNAL_SCHEMA_VERSION,
        commit_journal_ref: commit_journal_ref.clone(),
        command_id: preview.command_id.clone(),
        actor: preview.actor.clone(),
        author: preview.author.clone(),
        source_class: "source_control_commit_review".to_string(),
        commit_mode: preview.mode.as_str().to_string(),
        scope_ref: preview.scope.scope_ref.clone(),
        target_refs: preview
            .scope
            .staged_targets
            .iter()
            .map(|target| target.target_ref.clone())
            .collect(),
        started_at: preview.generated_at.clone(),
        resolved_at: resolved_at.to_string(),
        commit_oid: commit_oid.clone(),
        recovery_class: preview.history_guardrail.recovery_class.clone(),
        publish_readiness_ref: publish_readiness.publish_readiness_ref.clone(),
        redaction_class: "metadata_safe_default".to_string(),
        side_effect_summary: side_effect_summary(
            preview,
            outcome_state,
            commit_oid.as_deref(),
            &publish_readiness.queue_state,
        ),
    };
    GitCommitResult {
        record_kind: GIT_COMMIT_RESULT_RECORD_KIND.to_string(),
        schema_version: GIT_COMMIT_RESULT_SCHEMA_VERSION,
        result_ref,
        preview_ref: preview.preview_ref.clone(),
        resolved_at: resolved_at.to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        repo_root: preview.repo_root.clone(),
        truth_source_ref: preview.truth_source_ref.clone(),
        mode: preview.mode,
        outcome_state,
        commit_short_oid: commit_oid.as_deref().map(short_oid),
        commit_oid,
        committed_targets: if outcome_state == GitCommitOutcomeState::Committed {
            preview.scope.staged_targets.clone()
        } else {
            Vec::new()
        },
        blocked_reasons,
        author: preview.author.clone(),
        scope: preview.scope.clone(),
        history_guardrail: preview.history_guardrail.clone(),
        publish_readiness,
        commit_journal,
        activity,
        support_export,
        failure_reason,
    }
}

fn side_effect_summary(
    preview: &GitCommitPreview,
    outcome_state: GitCommitOutcomeState,
    commit_oid: Option<&str>,
    queue_state: &str,
) -> String {
    match (outcome_state, commit_oid) {
        (GitCommitOutcomeState::Committed, Some(oid)) => format!(
            "{} created local commit {}; publish-later state is {}",
            preview.mode.label(),
            short_oid(oid),
            queue_state
        ),
        (GitCommitOutcomeState::Committed, None) => {
            format!("{} created a local commit", preview.mode.label())
        }
        (GitCommitOutcomeState::BlockedNoChangesMade, _) => {
            "commit blocked before Git mutation".to_string()
        }
        (GitCommitOutcomeState::Failed, _) => "git commit command failed".to_string(),
    }
}

fn preview_ref(workspace_ref: &str, mode: GitCommitMode, requested_at: &str) -> String {
    format!(
        "git.commit.preview.{}.{}.{}",
        sanitize_id(workspace_ref),
        mode.as_str(),
        sanitize_id(requested_at)
    )
}

fn commit_ref(oid: &str) -> String {
    format!("git.rev.{}", short_oid(oid))
}

fn short_oid(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .take(7)
        .collect()
}

fn stderr_or_status(output: &GitCommitCommandOutput) -> String {
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
    out
}

fn observed_at_now() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("unix:{millis}")
}
