//! Preview-first Git push and publish review flows.
//!
//! This module owns the bounded alpha contract for publishing one local Git ref
//! to one configured remote ref. The preview names the client origin, route,
//! remote, local source, remote target, and recovery posture before any network
//! mutation runs. Result packets keep failed publishes reopenable and local
//! state recoverable without implying hosted-review or merge-queue maturity.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::status::{
    BranchState, ConsumerProjectionBundle, GitServiceState, GitStatusRequest, GitStatusService,
};

/// Stable record-kind tag for [`GitPublishPreview`].
pub const GIT_PUBLISH_PREVIEW_RECORD_KIND: &str = "git_publish_preview";

/// Stable record-kind tag for [`GitPublishResult`].
pub const GIT_PUBLISH_RESULT_RECORD_KIND: &str = "git_publish_result";

/// Stable record-kind tag for [`GitPublishActivityRecord`].
pub const GIT_PUBLISH_ACTIVITY_RECORD_KIND: &str = "git_publish_activity_record";

/// Stable record-kind tag for [`GitPublishSupportExportRecord`].
pub const GIT_PUBLISH_SUPPORT_EXPORT_RECORD_KIND: &str = "git_publish_support_export_record";

/// Stable record-kind tag for [`GitPublishJournalRecord`].
pub const GIT_PUBLISH_JOURNAL_RECORD_KIND: &str = "git_publish_journal_record";

/// Stable record-kind tag for [`GitPublishFailureRecoveryRecord`].
pub const GIT_PUBLISH_FAILURE_RECOVERY_RECORD_KIND: &str = "git_publish_failure_recovery_record";

const GIT_PUBLISH_PREVIEW_SCHEMA_VERSION: u32 = 1;
const GIT_PUBLISH_RESULT_SCHEMA_VERSION: u32 = 1;
const GIT_PUBLISH_ACTIVITY_SCHEMA_VERSION: u32 = 1;
const GIT_PUBLISH_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;
const GIT_PUBLISH_JOURNAL_SCHEMA_VERSION: u32 = 1;
const GIT_PUBLISH_FAILURE_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Publish mode requested by the Git publish lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPublishMode {
    /// Publish with Git's default fast-forward safety.
    Push,
    /// Publish with `--force-with-lease` after explicit review.
    ForceWithLease,
}

impl GitPublishMode {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::ForceWithLease => "force_with_lease",
        }
    }

    /// Canonical command id for this publish mode.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Push => "cmd:git.publish.push",
            Self::ForceWithLease => "cmd:git.publish.force_with_lease",
        }
    }

    /// Reviewer-facing label for this publish mode.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Push => "Publish branch",
            Self::ForceWithLease => "Force publish with lease",
        }
    }

    /// Returns true when this mode can rewrite remote history.
    pub const fn requires_force_review(self) -> bool {
        matches!(self, Self::ForceWithLease)
    }
}

/// State of a publish preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPublishPreviewState {
    /// Remote, route, target, and recovery posture are ready for explicit publish.
    ReadyToPublish,
    /// The preview exists, but target or guardrail honesty blocks publish.
    Blocked,
    /// Local Git state is unavailable, so no publish may proceed.
    Degraded,
}

impl GitPublishPreviewState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToPublish => "ready_to_publish",
            Self::Blocked => "blocked",
            Self::Degraded => "degraded",
        }
    }
}

/// Final state of a publish attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPublishOutcomeState {
    /// Git reported that the remote ref update completed.
    Published,
    /// No network mutation was attempted because the preview was not admissible.
    BlockedNoChangesMade,
    /// Git returned a failure while publishing.
    Failed,
}

impl GitPublishOutcomeState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::BlockedNoChangesMade => "blocked_no_changes_made",
            Self::Failed => "failed",
        }
    }

    fn activity_state_class(self) -> &'static str {
        match self {
            Self::Published => "completed",
            Self::BlockedNoChangesMade => "blocked",
            Self::Failed => "failed",
        }
    }
}

/// Client origin for the publish route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPublishOriginScope {
    /// Publish was initiated from the local desktop client.
    LocalDesktop,
    /// Publish was initiated from a remote target session.
    RemoteTarget,
    /// Publish was initiated from a managed workspace.
    ManagedWorkspace,
    /// Publish was initiated from a headless runner or CLI automation surface.
    HeadlessRunner,
}

impl GitPublishOriginScope {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDesktop => "local_desktop",
            Self::RemoteTarget => "remote_target",
            Self::ManagedWorkspace => "managed_workspace",
            Self::HeadlessRunner => "headless_runner",
        }
    }

    /// Reviewer-facing label for the origin scope.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDesktop => "Local desktop",
            Self::RemoteTarget => "Remote target",
            Self::ManagedWorkspace => "Managed workspace",
            Self::HeadlessRunner => "Headless runner",
        }
    }
}

/// Route class used to reach the Git remote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPublishRouteClass {
    /// Direct Git remote access from the acting client origin.
    DirectGitRemote,
    /// Enterprise mirror or approved proxy route.
    MirrorOrProxy,
    /// Provider gateway route with a delegated provider credential.
    ProviderGateway,
    /// Browser handoff route; this alpha lane cannot publish through it.
    BrowserHandoff,
}

impl GitPublishRouteClass {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectGitRemote => "direct_git_remote",
            Self::MirrorOrProxy => "mirror_or_proxy",
            Self::ProviderGateway => "provider_gateway",
            Self::BrowserHandoff => "browser_handoff",
        }
    }

    /// Reviewer-facing label for the route class.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DirectGitRemote => "Direct Git remote",
            Self::MirrorOrProxy => "Mirror or proxy",
            Self::ProviderGateway => "Provider gateway",
            Self::BrowserHandoff => "Browser handoff",
        }
    }
}

/// Local knowledge about the remote ref before publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitPublishRemoteState {
    /// The target remote ref is known from a local remote-tracking ref.
    ExistingRemoteRef,
    /// The target remote ref is not known locally and will be created if Git accepts it.
    NewRemoteRef,
    /// The requested remote name is not configured in the repository.
    RemoteMissing,
    /// The local source ref is missing or not a commit.
    LocalRefMissing,
    /// The target branch or source branch is not a valid branch name.
    InvalidTarget,
    /// The target could not be classified because Git state is degraded.
    Unknown,
}

impl GitPublishRemoteState {
    /// Stable token used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExistingRemoteRef => "existing_remote_ref",
            Self::NewRemoteRef => "new_remote_ref",
            Self::RemoteMissing => "remote_missing",
            Self::LocalRefMissing => "local_ref_missing",
            Self::InvalidTarget => "invalid_target",
            Self::Unknown => "unknown",
        }
    }
}

/// Actor identity attached to publish preview, result, and support records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishActorRef {
    /// Actor class token from the mutation lineage vocabulary.
    pub actor_class: String,
    /// Redaction-safe actor label.
    pub display_label: String,
    /// Optional stable principal or process ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
}

impl Default for GitPublishActorRef {
    fn default() -> Self {
        Self {
            actor_class: "local_user".to_string(),
            display_label: "Local user".to_string(),
            stable_id: None,
        }
    }
}

/// Request for a preview-first Git publish operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishRequest {
    /// Stable workspace identity copied into downstream records.
    pub workspace_ref: String,
    /// Root path selected by the workspace or launch wedge.
    pub root_path: PathBuf,
    /// Publish mode requested by the caller.
    pub mode: GitPublishMode,
    /// Optional remote name. When omitted, the current upstream remote is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_name: Option<String>,
    /// Optional local branch. When omitted, the attached current branch is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_branch: Option<String>,
    /// Optional target branch on the remote. When omitted, upstream or local branch is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    /// True when the caller explicitly acknowledged force-publish guardrails.
    pub force_review_acknowledged: bool,
    /// Expected remote object id for `--force-with-lease`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_remote_oid: Option<String>,
    /// Client origin that will initiate the Git push.
    pub origin_scope: GitPublishOriginScope,
    /// Route class used to reach the Git remote.
    pub route_class: GitPublishRouteClass,
    /// Actor that initiated the request.
    pub actor: GitPublishActorRef,
    /// Public row or surface ref that launched the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Timestamp supplied by the caller for deterministic exports.
    pub requested_at: String,
}

impl GitPublishRequest {
    /// Builds a publish request with a derived local workspace identity.
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
            mode: GitPublishMode::Push,
            remote_name: None,
            local_branch: None,
            target_branch: None,
            force_review_acknowledged: false,
            expected_remote_oid: None,
            origin_scope: GitPublishOriginScope::LocalDesktop,
            route_class: GitPublishRouteClass::DirectGitRemote,
            actor: GitPublishActorRef::default(),
            launch_source_ref: None,
            requested_at: observed_at_now(),
        }
    }

    /// Builds a request with explicit identity and timestamp fields.
    pub fn with_observed_at(
        workspace_ref: impl Into<String>,
        root_path: impl Into<PathBuf>,
        requested_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_ref: workspace_ref.into(),
            root_path: root_path.into(),
            mode: GitPublishMode::Push,
            remote_name: None,
            local_branch: None,
            target_branch: None,
            force_review_acknowledged: false,
            expected_remote_oid: None,
            origin_scope: GitPublishOriginScope::LocalDesktop,
            route_class: GitPublishRouteClass::DirectGitRemote,
            actor: GitPublishActorRef::default(),
            launch_source_ref: None,
            requested_at: requested_at.into(),
        }
    }

    /// Sets the publish mode.
    pub fn with_mode(mut self, mode: GitPublishMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the remote name to publish to.
    pub fn with_remote_name(mut self, remote_name: impl Into<String>) -> Self {
        self.remote_name = Some(remote_name.into());
        self
    }

    /// Sets the local branch to publish.
    pub fn with_local_branch(mut self, local_branch: impl Into<String>) -> Self {
        self.local_branch = Some(local_branch.into());
        self
    }

    /// Sets the target branch on the remote.
    pub fn with_target_branch(mut self, target_branch: impl Into<String>) -> Self {
        self.target_branch = Some(target_branch.into());
        self
    }

    /// Marks force-publish guardrails as explicitly acknowledged.
    pub fn acknowledge_force_review(mut self) -> Self {
        self.force_review_acknowledged = true;
        self
    }

    /// Sets the expected remote object id for a force-with-lease publish.
    pub fn with_expected_remote_oid(mut self, expected_remote_oid: impl Into<String>) -> Self {
        self.expected_remote_oid = Some(expected_remote_oid.into());
        self
    }

    /// Sets the route origin and route class labels.
    pub fn with_route(
        mut self,
        origin_scope: GitPublishOriginScope,
        route_class: GitPublishRouteClass,
    ) -> Self {
        self.origin_scope = origin_scope;
        self.route_class = route_class;
        self
    }

    /// Attaches an actor identity to the request.
    pub fn with_actor(mut self, actor: GitPublishActorRef) -> Self {
        self.actor = actor;
        self
    }

    /// Attaches a public launch-source ref to the request.
    pub fn with_launch_source_ref(mut self, launch_source_ref: impl Into<String>) -> Self {
        self.launch_source_ref = Some(launch_source_ref.into());
        self
    }
}

/// Route and origin labels shown before any publish attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishRouteReview {
    /// Stable route review ref.
    pub route_ref: String,
    /// Acting client origin.
    pub origin_scope: GitPublishOriginScope,
    /// Route class used for the Git push.
    pub route_class: GitPublishRouteClass,
    /// Human-readable origin label.
    pub traffic_origin_label: String,
    /// Remote name selected for the push.
    pub remote_name: String,
    /// Redaction-safe remote URL label.
    pub remote_url_label: String,
    /// Redaction-safe target host label when a host can be derived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_host_label: Option<String>,
    /// Exposure posture for the route.
    pub exposure_posture: String,
    /// Authentication posture visible before publish.
    pub auth_posture: String,
    /// Policy source label for this alpha route.
    pub policy_source: String,
    /// True when route class and origin are visible in the preview.
    pub route_disclosed: bool,
    /// True when remote and URL posture are visible in the preview.
    pub remote_disclosed: bool,
    /// True when this route can execute in the local alpha lane.
    pub executable_in_alpha: bool,
}

impl GitPublishRouteReview {
    /// Returns true when route, origin, and remote labels are complete enough for publish.
    pub fn labels_are_complete(&self) -> bool {
        self.route_disclosed
            && self.remote_disclosed
            && !self.remote_name.trim().is_empty()
            && !self.remote_url_label.trim().is_empty()
            && self.executable_in_alpha
    }
}

/// Remote target and divergence review shown before publish.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishTargetReview {
    /// Stable target ref used by activity, support, and journal records.
    pub target_ref: String,
    /// Route ref used to reach this target.
    pub route_ref: String,
    /// Local branch selected as the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_branch: Option<String>,
    /// Full local ref selected as the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_ref: Option<String>,
    /// Local source commit object id observed at preview time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_oid: Option<String>,
    /// Compact local source commit object id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_short_oid: Option<String>,
    /// Remote name selected for publish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_name: Option<String>,
    /// Remote target branch selected for publish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    /// Full remote ref selected as the target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_ref: Option<String>,
    /// Local remote-tracking ref used as the last-known target basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_tracking_ref: Option<String>,
    /// Last-known remote target object id when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_oid: Option<String>,
    /// Compact last-known remote target object id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_short_oid: Option<String>,
    /// Remote target state inferred from local Git truth.
    pub remote_state: GitPublishRemoteState,
    /// Commit count present on the target but not the local source when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behind_count: Option<u32>,
    /// Commit count present on the local source but not the target when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ahead_count: Option<u32>,
    /// True when the remote is configured locally.
    pub remote_configured: bool,
    /// True when the preview identifies a local source and remote target.
    pub target_disclosed: bool,
    /// True when a detached `HEAD` blocked implicit source selection.
    pub detached_head_blocked: bool,
    /// True when a force-publish action needs explicit review.
    pub force_review_required: bool,
    /// True when force-publish review acknowledgement was supplied.
    pub force_review_acknowledged: bool,
    /// Expected remote object id that will be passed to `--force-with-lease`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_with_lease_expected_oid: Option<String>,
    /// True when this alpha path deliberately excludes merge queues.
    pub merge_queue_supported: bool,
    /// Provider overlay state for this local-first publish path.
    pub provider_overlay_state: String,
    /// Reasons that block publish before Git runs.
    pub blocked_reasons: Vec<String>,
}

impl GitPublishTargetReview {
    /// Returns true when target identity is explicit and unblocked.
    pub fn is_satisfied(&self) -> bool {
        self.remote_configured
            && self.target_disclosed
            && self.local_oid.is_some()
            && self.local_ref.is_some()
            && self.remote_ref.is_some()
            && self.blocked_reasons.is_empty()
    }
}

/// Failure recovery record retained by preview and result packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishFailureRecoveryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable recovery ref.
    pub recovery_ref: String,
    /// Phase token for preview, blocked, failed, or published.
    pub phase: String,
    /// Recovery class from the shared preview/apply/revert vocabulary.
    pub recovery_class: String,
    /// True when the same publish preview can be reopened after failure.
    pub same_review_reopen_available: bool,
    /// Preview ref to reopen after failure.
    pub reopen_preview_ref: String,
    /// Command id that reopens the publish review.
    pub reopen_command_id: String,
    /// Command id that retries publish after renewed review.
    pub retry_command_id: String,
    /// Redaction-safe export packet ref for offline or support handoff.
    pub export_packet_ref: String,
    /// True when local repository state remains available after failure.
    pub local_recovery_available: bool,
    /// True when the failed publish path does not discard local commits or worktree state.
    pub local_state_preserved: bool,
    /// Provider write state after the phase.
    pub provider_write_state: String,
    /// Reviewer-facing recovery label.
    pub recovery_label: String,
    /// Failure reason copied into recovery after a failed publish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

/// Activity-center projection for publish preview or result state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishActivityRecord {
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
    /// Result ref when publish resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Publish journal ref when publish resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_journal_ref: Option<String>,
    /// Command id that reopens publish details.
    pub open_details_command_id: String,
    /// Support-export ref that carries the same attribution.
    pub support_export_ref: String,
    /// Recovery ref used for failure and retry joins.
    pub recovery_ref: String,
}

/// Redaction-safe support export projection for a publish flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishSupportExportRecord {
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
    /// Publish mode token.
    pub publish_mode: String,
    /// Phase token for preview, blocked, failed, or published.
    pub phase: String,
    /// Workspace identity copied from the preview.
    pub workspace_ref: String,
    /// Route ref copied from the preview.
    pub route_ref: String,
    /// Target ref copied from the preview.
    pub target_ref: String,
    /// Preview ref copied from the preview.
    pub preview_ref: String,
    /// Result ref when publish completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Publish journal ref when publish completed or failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_journal_ref: Option<String>,
    /// Recovery ref for failure and retry paths.
    pub recovery_ref: String,
    /// Evidence refs included without raw remote URLs or command output.
    pub evidence_refs: Vec<String>,
    /// Fields deliberately omitted from export.
    pub omitted_fields: Vec<String>,
}

/// Mutation-journal shaped record emitted after publish resolves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishJournalRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable publish journal ref.
    pub publish_journal_ref: String,
    /// Canonical command id that attempted the external effect.
    pub command_id: String,
    /// Actor that initiated the publish.
    pub actor: GitPublishActorRef,
    /// Source class for the mutation journal.
    pub source_class: String,
    /// Publish mode token.
    pub publish_mode: String,
    /// Route ref copied from the preview.
    pub route_ref: String,
    /// Target ref copied from the preview.
    pub target_ref: String,
    /// Timestamp when preview started.
    pub started_at: String,
    /// Timestamp when publish resolved.
    pub resolved_at: String,
    /// Status snapshot ref before publish.
    pub before_truth_source_ref: String,
    /// Remote side-effect summary.
    pub side_effect_summary: String,
    /// Recovery class advertised for this external-effect mutation.
    pub recovery_class: String,
    /// Redaction class for support exports.
    pub redaction_class: String,
}

/// Review-first preview packet for a Git publish operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishPreview {
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
    /// Publish mode being reviewed.
    pub mode: GitPublishMode,
    /// Canonical command id for publish.
    pub command_id: String,
    /// Reviewer-facing operation label.
    pub operation_label: String,
    /// Consequence class for review sheets.
    pub consequence_class: String,
    /// Current preview state.
    pub preview_state: GitPublishPreviewState,
    /// Actor that initiated the preview.
    pub actor: GitPublishActorRef,
    /// Public row or surface ref that launched the preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_source_ref: Option<String>,
    /// Route and origin review shown before publish.
    pub route: GitPublishRouteReview,
    /// Source, remote target, and divergence review shown before publish.
    pub target: GitPublishTargetReview,
    /// Failure recovery and retry posture.
    pub failure_recovery: GitPublishFailureRecoveryRecord,
    /// Activity projection for the preview state.
    pub activity: GitPublishActivityRecord,
    /// Support-export projection for the preview state.
    pub support_export: GitPublishSupportExportRecord,
    /// Reasons that block publish from this preview.
    pub blocked_reasons: Vec<String>,
    /// True when this alpha path deliberately excludes merge queues.
    pub merge_queue_supported: bool,
    /// Review-platform maturity label for this local Git lane.
    pub review_platform_state: String,
    #[serde(skip)]
    remote_for_apply: Option<String>,
    #[serde(skip)]
    local_ref_for_apply: Option<String>,
    #[serde(skip)]
    remote_ref_for_apply: Option<String>,
    #[serde(skip)]
    expected_local_oid_for_apply: Option<String>,
    #[serde(skip)]
    force_with_lease_for_apply: Option<String>,
}

impl GitPublishPreview {
    /// Returns true when publish may proceed without recomputing route or target.
    pub fn ready_to_publish(&self) -> bool {
        self.preview_state == GitPublishPreviewState::ReadyToPublish
            && self.blocked_reasons.is_empty()
            && self.route.labels_are_complete()
            && self.target.is_satisfied()
            && self.remote_for_apply.is_some()
            && self.local_ref_for_apply.is_some()
            && self.remote_ref_for_apply.is_some()
            && self.expected_local_oid_for_apply.is_some()
    }
}

/// Result packet emitted after applying or blocking a publish preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitPublishResult {
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
    /// Publish mode that was applied.
    pub mode: GitPublishMode,
    /// Final outcome state.
    pub outcome_state: GitPublishOutcomeState,
    /// Route and origin review copied from the preview.
    pub route: GitPublishRouteReview,
    /// Target review copied from the preview.
    pub target: GitPublishTargetReview,
    /// Mutation-journal shaped lineage record.
    pub publish_journal: GitPublishJournalRecord,
    /// Activity projection for the result.
    pub activity: GitPublishActivityRecord,
    /// Support-export projection for the result.
    pub support_export: GitPublishSupportExportRecord,
    /// Failure recovery and retry posture.
    pub failure_recovery: GitPublishFailureRecoveryRecord,
    /// Failure reason when publish failed after starting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Reasons that blocked or failed publish.
    pub blocked_reasons: Vec<String>,
    /// True when this alpha path deliberately excludes merge queues.
    pub merge_queue_supported: bool,
    /// Review-platform maturity label for this local Git lane.
    pub review_platform_state: String,
}

impl GitPublishResult {
    /// Returns true when a failed publish can reopen the original review state.
    pub fn failure_can_reopen_review(&self) -> bool {
        self.outcome_state != GitPublishOutcomeState::Failed
            || (self.failure_recovery.same_review_reopen_available
                && self.failure_recovery.reopen_preview_ref == self.preview_ref
                && self.failure_recovery.local_state_preserved)
    }
}

/// Output captured from a Git publish command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitPublishCommandOutput {
    /// True when Git exited successfully.
    pub success: bool,
    /// Process exit status code when available.
    pub status_code: Option<i32>,
    /// Captured stdout bytes.
    pub stdout: Vec<u8>,
    /// Captured stderr bytes.
    pub stderr: Vec<u8>,
}

/// Error raised before a Git publish command can be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitPublishBackendError {
    /// Redaction-safe error message.
    pub message: String,
}

impl std::fmt::Display for GitPublishBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GitPublishBackendError {}

/// Backend used by [`GitPublishService`] to execute local Git commands.
pub trait GitPublishBackend {
    /// Runs `git -C root args`.
    ///
    /// # Errors
    ///
    /// Returns [`GitPublishBackendError`] when the backend cannot launch or
    /// supervise the Git process.
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitPublishCommandOutput, GitPublishBackendError>;
}

/// Git backend that shells out to the system `git` executable.
#[derive(Debug, Clone)]
pub struct SystemGitPublishBackend {
    git_binary: PathBuf,
}

impl Default for SystemGitPublishBackend {
    fn default() -> Self {
        Self::new("git")
    }
}

impl SystemGitPublishBackend {
    /// Creates a backend that invokes `git_binary`.
    pub fn new(git_binary: impl Into<PathBuf>) -> Self {
        Self {
            git_binary: git_binary.into(),
        }
    }
}

impl GitPublishBackend for SystemGitPublishBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitPublishCommandOutput, GitPublishBackendError> {
        let output = Command::new(&self.git_binary)
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|err| GitPublishBackendError {
                message: format!("git command failed to launch: {err}"),
            })?;
        Ok(GitPublishCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

/// Service that creates and applies Git publish previews.
#[derive(Debug, Clone)]
pub struct GitPublishService<B = SystemGitPublishBackend> {
    backend: B,
}

impl Default for GitPublishService<SystemGitPublishBackend> {
    fn default() -> Self {
        Self::new(SystemGitPublishBackend::default())
    }
}

impl<B: GitPublishBackend> GitPublishService<B> {
    /// Creates a service backed by `backend`.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Builds a reviewable publish preview without mutating remote state.
    pub fn preview(&self, request: &GitPublishRequest) -> GitPublishPreview {
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
        let preview_ref = preview_ref(request);

        if snapshot.service_state != GitServiceState::Current {
            return degraded_preview(
                request,
                repo_root,
                truth_source_ref,
                preview_ref,
                snapshot.service_state.as_str(),
            );
        }

        let target = self.target_review(request, &preview_ref, &repo_root, &snapshot);
        let route = self.route_review(request, &preview_ref, &repo_root, &target);
        let mut blocked_reasons = target.blocked_reasons.clone();
        if !route.labels_are_complete() {
            if route.route_class == GitPublishRouteClass::BrowserHandoff {
                blocked_reasons.push(
                    "browser handoff is inspect-only for this local Git publish lane".to_string(),
                );
            } else {
                blocked_reasons.push("publish route labels are incomplete".to_string());
            }
        }
        let preview_state = if blocked_reasons.is_empty() {
            GitPublishPreviewState::ReadyToPublish
        } else {
            GitPublishPreviewState::Blocked
        };
        let failure_recovery = recovery_for_preview(&preview_ref);
        let support_export =
            support_export_for_preview(&preview_ref, request, &route, &target, &failure_recovery);
        let activity = activity_for_preview(
            &preview_ref,
            request.mode,
            preview_state,
            &route,
            &target,
            &support_export.support_export_ref,
            &failure_recovery.recovery_ref,
        );

        GitPublishPreview {
            record_kind: GIT_PUBLISH_PREVIEW_RECORD_KIND.to_string(),
            schema_version: GIT_PUBLISH_PREVIEW_SCHEMA_VERSION,
            preview_ref,
            generated_at: request.requested_at.clone(),
            workspace_ref: request.workspace_ref.clone(),
            repo_root,
            truth_source_ref,
            mode: request.mode,
            command_id: request.mode.command_id().to_string(),
            operation_label: request.mode.label().to_string(),
            consequence_class: "external_git_ref_update".to_string(),
            preview_state,
            actor: request.actor.clone(),
            launch_source_ref: request.launch_source_ref.clone(),
            route,
            remote_for_apply: target.remote_name.clone(),
            local_ref_for_apply: target.local_ref.clone(),
            remote_ref_for_apply: target.remote_ref.clone(),
            expected_local_oid_for_apply: target.local_oid.clone(),
            force_with_lease_for_apply: target.force_with_lease_expected_oid.clone(),
            target,
            failure_recovery,
            activity,
            support_export,
            blocked_reasons,
            merge_queue_supported: false,
            review_platform_state: "local_git_publish_only".to_string(),
        }
    }

    /// Applies an admitted publish preview and returns an attributable result packet.
    pub fn apply(
        &self,
        preview: &GitPublishPreview,
        resolved_at: impl Into<String>,
    ) -> GitPublishResult {
        let resolved_at = resolved_at.into();
        if !preview.ready_to_publish() {
            return result_for_preview(
                preview,
                &resolved_at,
                GitPublishOutcomeState::BlockedNoChangesMade,
                None,
                preview.blocked_reasons.clone(),
            );
        }

        let local_ref = preview.local_ref_for_apply.as_deref().unwrap_or_default();
        let current_oid = self.resolve_commit_oid(&preview.repo_root, local_ref);
        if current_oid != preview.expected_local_oid_for_apply {
            return result_for_preview(
                preview,
                &resolved_at,
                GitPublishOutcomeState::BlockedNoChangesMade,
                Some(
                    "local publish source changed after preview; reopen publish review".to_string(),
                ),
                vec!["local publish source changed after preview".to_string()],
            );
        }

        let output = self.apply_preview(preview);
        let (outcome_state, failure_reason) = match output {
            Ok(output) if output.success => (GitPublishOutcomeState::Published, None),
            Ok(output) => (
                GitPublishOutcomeState::Failed,
                Some(stderr_or_status(&output)),
            ),
            Err(err) => (GitPublishOutcomeState::Failed, Some(err.message)),
        };
        let blocked_reasons = failure_reason.clone().into_iter().collect();
        result_for_preview(
            preview,
            &resolved_at,
            outcome_state,
            failure_reason,
            blocked_reasons,
        )
    }

    fn target_review(
        &self,
        request: &GitPublishRequest,
        preview_ref: &str,
        repo_root: &Path,
        snapshot: &crate::status::GitStatusSnapshot,
    ) -> GitPublishTargetReview {
        let route_ref = format!("{}.route", preview_ref);
        let mut blocked_reasons = Vec::new();
        let upstream = snapshot.head.upstream.as_deref().and_then(parse_upstream);
        let remote_name = request
            .remote_name
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .or_else(|| upstream.as_ref().map(|(remote, _)| remote.clone()));
        if remote_name.is_none() {
            blocked_reasons
                .push("publish remote requires upstream or explicit remote selection".to_string());
        }

        let source_branch = match request.local_branch.as_deref() {
            Some(value) => normalized_branch_name(value),
            None if snapshot.head.state == BranchState::Attached => {
                snapshot.head.branch_label.clone()
            }
            None if snapshot.head.state == BranchState::Detached => {
                blocked_reasons.push(
                    "detached HEAD requires explicit local branch before publish".to_string(),
                );
                None
            }
            None => {
                blocked_reasons
                    .push("current branch could not be resolved before publish".to_string());
                None
            }
        };
        let target_branch = request
            .target_branch
            .as_deref()
            .and_then(normalized_branch_name)
            .or_else(|| upstream.as_ref().map(|(_, branch)| branch.clone()))
            .or_else(|| source_branch.clone());
        if target_branch.is_none() {
            blocked_reasons.push("publish target branch is required".to_string());
        }

        let mut invalid_target = false;
        let source_branch =
            source_branch.and_then(|branch| match self.valid_branch_name(repo_root, &branch) {
                Ok(branch) => Some(branch),
                Err(reason) => {
                    invalid_target = true;
                    blocked_reasons.push(format!("local branch invalid: {reason}"));
                    None
                }
            });
        let target_branch =
            target_branch.and_then(|branch| match self.valid_branch_name(repo_root, &branch) {
                Ok(branch) => Some(branch),
                Err(reason) => {
                    invalid_target = true;
                    blocked_reasons.push(format!("target branch invalid: {reason}"));
                    None
                }
            });

        let local_ref = source_branch
            .as_ref()
            .map(|branch| format!("refs/heads/{branch}"));
        let remote_ref = target_branch
            .as_ref()
            .map(|branch| format!("refs/heads/{branch}"));
        let local_oid = local_ref
            .as_deref()
            .and_then(|local_ref| self.resolve_commit_oid(repo_root, local_ref));
        if local_ref.is_some() && local_oid.is_none() {
            blocked_reasons.push("local source ref could not be resolved".to_string());
        }

        let remote_configured = remote_name
            .as_deref()
            .is_some_and(|remote| self.remote_exists(repo_root, remote));
        if remote_name.is_some() && !remote_configured {
            blocked_reasons.push("publish remote is not configured locally".to_string());
        }

        let remote_tracking_ref = remote_name
            .as_ref()
            .zip(target_branch.as_ref())
            .map(|(remote, branch)| format!("refs/remotes/{remote}/{branch}"));
        let remote_oid = if remote_configured {
            remote_tracking_ref
                .as_deref()
                .and_then(|tracking_ref| self.resolve_commit_oid(repo_root, tracking_ref))
        } else {
            None
        };
        let (behind_count, ahead_count) = match (remote_oid.as_deref(), local_oid.as_deref()) {
            (Some(remote_oid), Some(local_oid)) => {
                self.rev_list_counts(repo_root, remote_oid, local_oid)
            }
            _ => (None, None),
        };
        if request.mode == GitPublishMode::Push && behind_count.unwrap_or(0) > 0 {
            blocked_reasons.push(
                "remote target contains commits missing from the local source; fetch and review before publish"
                    .to_string(),
            );
        }

        let mut force_with_lease_expected_oid = None;
        if request.mode.requires_force_review() {
            if !request.force_review_acknowledged {
                blocked_reasons
                    .push("force publish requires explicit force-with-lease review".to_string());
            }
            match (
                remote_oid.as_deref(),
                request.expected_remote_oid.as_deref(),
            ) {
                (Some(remote_oid), Some(expected)) if normalize_oid(expected) == remote_oid => {
                    force_with_lease_expected_oid = Some(remote_oid.to_string());
                }
                (Some(_), Some(_)) => {
                    blocked_reasons.push(
                        "force-with-lease expected remote object does not match the preview target"
                            .to_string(),
                    );
                }
                (Some(_), None) => blocked_reasons
                    .push("force-with-lease requires the expected remote object id".to_string()),
                (None, _) => blocked_reasons.push(
                    "force-with-lease requires an existing last-known remote target".to_string(),
                ),
            }
        }

        let remote_state = if invalid_target {
            GitPublishRemoteState::InvalidTarget
        } else if !remote_configured {
            GitPublishRemoteState::RemoteMissing
        } else if local_oid.is_none() {
            GitPublishRemoteState::LocalRefMissing
        } else if remote_oid.is_some() {
            GitPublishRemoteState::ExistingRemoteRef
        } else {
            GitPublishRemoteState::NewRemoteRef
        };
        let target_disclosed = remote_name.is_some() && local_ref.is_some() && remote_ref.is_some();
        GitPublishTargetReview {
            target_ref: format!("{}.target", preview_ref),
            route_ref,
            local_branch: source_branch,
            local_ref,
            local_short_oid: local_oid.as_deref().map(short_oid),
            local_oid,
            remote_name,
            target_branch,
            remote_ref,
            remote_tracking_ref,
            remote_short_oid: remote_oid.as_deref().map(short_oid),
            remote_oid,
            remote_state,
            behind_count,
            ahead_count,
            remote_configured,
            target_disclosed,
            detached_head_blocked: snapshot.head.state == BranchState::Detached
                && request.local_branch.is_none(),
            force_review_required: request.mode.requires_force_review(),
            force_review_acknowledged: request.force_review_acknowledged,
            force_with_lease_expected_oid,
            merge_queue_supported: false,
            provider_overlay_state: "not_configured_alpha".to_string(),
            blocked_reasons,
        }
    }

    fn route_review(
        &self,
        request: &GitPublishRequest,
        preview_ref: &str,
        repo_root: &Path,
        target: &GitPublishTargetReview,
    ) -> GitPublishRouteReview {
        let remote_name = target
            .remote_name
            .clone()
            .unwrap_or_else(|| "<remote not selected>".to_string());
        let raw_url = target
            .remote_name
            .as_deref()
            .filter(|_| target.remote_configured)
            .and_then(|remote| self.remote_url(repo_root, remote));
        let remote_url_label = raw_url
            .as_deref()
            .map(redacted_remote_url_label)
            .unwrap_or_else(|| "<remote url unavailable>".to_string());
        let target_host_label = raw_url.as_deref().and_then(remote_host_label);
        let executable_in_alpha = request.route_class != GitPublishRouteClass::BrowserHandoff;
        GitPublishRouteReview {
            route_ref: format!("{}.route", preview_ref),
            origin_scope: request.origin_scope,
            route_class: request.route_class,
            traffic_origin_label: request.origin_scope.label().to_string(),
            remote_name,
            remote_url_label,
            target_host_label,
            exposure_posture: "git_remote_write".to_string(),
            auth_posture: "git_credential_helper_or_remote_config".to_string(),
            policy_source: "workspace_git_policy_alpha".to_string(),
            route_disclosed: true,
            remote_disclosed: true,
            executable_in_alpha,
        }
    }

    fn valid_branch_name(&self, repo_root: &Path, value: &str) -> Result<String, String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("branch name is required".to_string());
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
                let value = if normalized.is_empty() {
                    trimmed.to_string()
                } else {
                    normalized
                };
                Ok(value
                    .strip_prefix("refs/heads/")
                    .unwrap_or(&value)
                    .to_string())
            }
            Ok(output) => Err(stderr_or_status(&output)),
            Err(err) => Err(err.message),
        }
    }

    fn remote_exists(&self, repo_root: &Path, remote: &str) -> bool {
        let args = vec!["remote".to_string()];
        let output = match self.backend.run_git(repo_root, &args) {
            Ok(output) if output.success => output,
            _ => return false,
        };
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::trim)
            .any(|line| line == remote)
    }

    fn remote_url(&self, repo_root: &Path, remote: &str) -> Option<String> {
        let args = vec![
            "remote".to_string(),
            "get-url".to_string(),
            remote.to_string(),
        ];
        let output = self.backend.run_git(repo_root, &args).ok()?;
        if !output.success {
            return None;
        }
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        (!value.is_empty()).then_some(value)
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

    fn rev_list_counts(
        &self,
        repo_root: &Path,
        remote_oid: &str,
        local_oid: &str,
    ) -> (Option<u32>, Option<u32>) {
        let args = vec![
            "rev-list".to_string(),
            "--left-right".to_string(),
            "--count".to_string(),
            format!("{remote_oid}...{local_oid}"),
        ];
        let output = match self.backend.run_git(repo_root, &args) {
            Ok(output) if output.success => output,
            _ => return (None, None),
        };
        let text = String::from_utf8_lossy(&output.stdout);
        let mut parts = text.split_whitespace();
        let behind = parts.next().and_then(|value| value.parse::<u32>().ok());
        let ahead = parts.next().and_then(|value| value.parse::<u32>().ok());
        (behind, ahead)
    }

    fn apply_preview(
        &self,
        preview: &GitPublishPreview,
    ) -> Result<GitPublishCommandOutput, GitPublishBackendError> {
        let remote = preview
            .remote_for_apply
            .as_ref()
            .ok_or_else(|| GitPublishBackendError {
                message: "publish remote missing".to_string(),
            })?;
        let local_ref =
            preview
                .local_ref_for_apply
                .as_ref()
                .ok_or_else(|| GitPublishBackendError {
                    message: "publish local ref missing".to_string(),
                })?;
        let remote_ref =
            preview
                .remote_ref_for_apply
                .as_ref()
                .ok_or_else(|| GitPublishBackendError {
                    message: "publish remote ref missing".to_string(),
                })?;
        let mut args = vec!["push".to_string(), "--porcelain".to_string()];
        if preview.mode == GitPublishMode::ForceWithLease {
            let expected = preview.force_with_lease_for_apply.as_ref().ok_or_else(|| {
                GitPublishBackendError {
                    message: "force-with-lease expected remote object missing".to_string(),
                }
            })?;
            args.push(format!("--force-with-lease={remote_ref}:{expected}"));
        }
        args.push(remote.clone());
        args.push(format!("{local_ref}:{remote_ref}"));
        self.backend.run_git(&preview.repo_root, &args)
    }
}

fn degraded_preview(
    request: &GitPublishRequest,
    repo_root: PathBuf,
    truth_source_ref: String,
    preview_ref: String,
    reason: &str,
) -> GitPublishPreview {
    let route = GitPublishRouteReview {
        route_ref: format!("{}.route", preview_ref),
        origin_scope: request.origin_scope,
        route_class: request.route_class,
        traffic_origin_label: request.origin_scope.label().to_string(),
        remote_name: request
            .remote_name
            .clone()
            .unwrap_or_else(|| "<remote unavailable>".to_string()),
        remote_url_label: "<git status unavailable>".to_string(),
        target_host_label: None,
        exposure_posture: "git_remote_write".to_string(),
        auth_posture: "unknown".to_string(),
        policy_source: "workspace_git_policy_alpha".to_string(),
        route_disclosed: true,
        remote_disclosed: true,
        executable_in_alpha: request.route_class != GitPublishRouteClass::BrowserHandoff,
    };
    let target = GitPublishTargetReview {
        target_ref: format!("{}.target", preview_ref),
        route_ref: route.route_ref.clone(),
        local_branch: request.local_branch.clone(),
        local_ref: request
            .local_branch
            .as_ref()
            .and_then(|branch| normalized_branch_name(branch))
            .map(|branch| format!("refs/heads/{branch}")),
        local_oid: None,
        local_short_oid: None,
        remote_name: request.remote_name.clone(),
        target_branch: request.target_branch.clone(),
        remote_ref: request
            .target_branch
            .as_ref()
            .and_then(|branch| normalized_branch_name(branch))
            .map(|branch| format!("refs/heads/{branch}")),
        remote_tracking_ref: None,
        remote_oid: None,
        remote_short_oid: None,
        remote_state: GitPublishRemoteState::Unknown,
        behind_count: None,
        ahead_count: None,
        remote_configured: false,
        target_disclosed: request.remote_name.is_some() && request.target_branch.is_some(),
        detached_head_blocked: false,
        force_review_required: request.mode.requires_force_review(),
        force_review_acknowledged: request.force_review_acknowledged,
        force_with_lease_expected_oid: request.expected_remote_oid.clone(),
        merge_queue_supported: false,
        provider_overlay_state: "not_configured_alpha".to_string(),
        blocked_reasons: vec![format!("Git service degraded: {reason}")],
    };
    let blocked_reasons = vec![format!("Git service degraded: {reason}")];
    let failure_recovery = recovery_for_preview(&preview_ref);
    let support_export =
        support_export_for_preview(&preview_ref, request, &route, &target, &failure_recovery);
    let activity = activity_for_preview(
        &preview_ref,
        request.mode,
        GitPublishPreviewState::Degraded,
        &route,
        &target,
        &support_export.support_export_ref,
        &failure_recovery.recovery_ref,
    );
    GitPublishPreview {
        record_kind: GIT_PUBLISH_PREVIEW_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_PREVIEW_SCHEMA_VERSION,
        preview_ref,
        generated_at: request.requested_at.clone(),
        workspace_ref: request.workspace_ref.clone(),
        repo_root,
        truth_source_ref,
        mode: request.mode,
        command_id: request.mode.command_id().to_string(),
        operation_label: request.mode.label().to_string(),
        consequence_class: "external_git_ref_update".to_string(),
        preview_state: GitPublishPreviewState::Degraded,
        actor: request.actor.clone(),
        launch_source_ref: request.launch_source_ref.clone(),
        route,
        target,
        failure_recovery,
        activity,
        support_export,
        blocked_reasons,
        merge_queue_supported: false,
        review_platform_state: "local_git_publish_only".to_string(),
        remote_for_apply: None,
        local_ref_for_apply: None,
        remote_ref_for_apply: None,
        expected_local_oid_for_apply: None,
        force_with_lease_for_apply: None,
    }
}

fn recovery_for_preview(preview_ref: &str) -> GitPublishFailureRecoveryRecord {
    GitPublishFailureRecoveryRecord {
        record_kind: GIT_PUBLISH_FAILURE_RECOVERY_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_FAILURE_RECOVERY_SCHEMA_VERSION,
        recovery_ref: format!("{}.recovery", preview_ref),
        phase: "preview".to_string(),
        recovery_class: "reopen_same_publish_review".to_string(),
        same_review_reopen_available: true,
        reopen_preview_ref: preview_ref.to_string(),
        reopen_command_id: "cmd:git.publish.review.reopen".to_string(),
        retry_command_id: "cmd:git.publish.retry_after_review".to_string(),
        export_packet_ref: format!("export.{}", sanitize_id(preview_ref)),
        local_recovery_available: true,
        local_state_preserved: true,
        provider_write_state: "not_attempted".to_string(),
        recovery_label: "Reopen publish review or export a local handoff packet".to_string(),
        failure_reason: None,
    }
}

fn recovery_for_result(
    preview: &GitPublishPreview,
    result_ref: &str,
    outcome_state: GitPublishOutcomeState,
    failure_reason: Option<String>,
) -> GitPublishFailureRecoveryRecord {
    let (phase, provider_write_state, recovery_label) = match outcome_state {
        GitPublishOutcomeState::Published => (
            "published",
            "remote_ref_update_confirmed",
            "Inspect publish details from Git history or provider overlays",
        ),
        GitPublishOutcomeState::BlockedNoChangesMade => (
            "blocked",
            "not_attempted",
            "Reopen publish review after selecting a valid remote and target",
        ),
        GitPublishOutcomeState::Failed => (
            "failed",
            "not_confirmed_by_alpha_lane",
            "Reopen the same publish review, retry after recovery, or export a local handoff packet",
        ),
    };
    GitPublishFailureRecoveryRecord {
        record_kind: GIT_PUBLISH_FAILURE_RECOVERY_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_FAILURE_RECOVERY_SCHEMA_VERSION,
        recovery_ref: format!("{}.recovery", result_ref),
        phase: phase.to_string(),
        recovery_class: if outcome_state == GitPublishOutcomeState::Published {
            "audit_only_remote_ref_update"
        } else {
            "reopen_same_publish_review"
        }
        .to_string(),
        same_review_reopen_available: outcome_state != GitPublishOutcomeState::Published,
        reopen_preview_ref: preview.preview_ref.clone(),
        reopen_command_id: "cmd:git.publish.review.reopen".to_string(),
        retry_command_id: "cmd:git.publish.retry_after_review".to_string(),
        export_packet_ref: format!("export.{}", sanitize_id(result_ref)),
        local_recovery_available: true,
        local_state_preserved: true,
        provider_write_state: provider_write_state.to_string(),
        recovery_label: recovery_label.to_string(),
        failure_reason,
    }
}

fn support_export_for_preview(
    preview_ref: &str,
    request: &GitPublishRequest,
    route: &GitPublishRouteReview,
    target: &GitPublishTargetReview,
    recovery: &GitPublishFailureRecoveryRecord,
) -> GitPublishSupportExportRecord {
    GitPublishSupportExportRecord {
        record_kind: GIT_PUBLISH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.{}", sanitize_id(preview_ref)),
        redaction_mode: "omit_remote_url_and_command_output".to_string(),
        retention_class: "local_git_publish_review".to_string(),
        publish_mode: request.mode.as_str().to_string(),
        phase: "preview".to_string(),
        workspace_ref: request.workspace_ref.clone(),
        route_ref: route.route_ref.clone(),
        target_ref: target.target_ref.clone(),
        preview_ref: preview_ref.to_string(),
        result_ref: None,
        publish_journal_ref: None,
        recovery_ref: recovery.recovery_ref.clone(),
        evidence_refs: vec![
            route.route_ref.clone(),
            target.target_ref.clone(),
            recovery.recovery_ref.clone(),
        ],
        omitted_fields: vec![
            "raw_remote_url".to_string(),
            "raw_git_stdout".to_string(),
            "raw_git_stderr".to_string(),
        ],
    }
}

fn support_export_for_result(
    preview: &GitPublishPreview,
    result_ref: &str,
    journal_ref: &str,
    recovery_ref: &str,
    outcome_state: GitPublishOutcomeState,
) -> GitPublishSupportExportRecord {
    GitPublishSupportExportRecord {
        record_kind: GIT_PUBLISH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_SUPPORT_EXPORT_SCHEMA_VERSION,
        support_export_ref: format!("support.{}", sanitize_id(result_ref)),
        redaction_mode: "omit_remote_url_and_command_output".to_string(),
        retention_class: "local_git_publish_review".to_string(),
        publish_mode: preview.mode.as_str().to_string(),
        phase: match outcome_state {
            GitPublishOutcomeState::Published => "published",
            GitPublishOutcomeState::BlockedNoChangesMade => "blocked",
            GitPublishOutcomeState::Failed => "failed",
        }
        .to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        route_ref: preview.route.route_ref.clone(),
        target_ref: preview.target.target_ref.clone(),
        preview_ref: preview.preview_ref.clone(),
        result_ref: Some(result_ref.to_string()),
        publish_journal_ref: Some(journal_ref.to_string()),
        recovery_ref: recovery_ref.to_string(),
        evidence_refs: vec![
            preview.route.route_ref.clone(),
            preview.target.target_ref.clone(),
            recovery_ref.to_string(),
            journal_ref.to_string(),
        ],
        omitted_fields: vec![
            "raw_remote_url".to_string(),
            "raw_git_stdout".to_string(),
            "raw_git_stderr".to_string(),
        ],
    }
}

fn activity_for_preview(
    preview_ref: &str,
    mode: GitPublishMode,
    preview_state: GitPublishPreviewState,
    route: &GitPublishRouteReview,
    target: &GitPublishTargetReview,
    support_export_ref: &str,
    recovery_ref: &str,
) -> GitPublishActivityRecord {
    let state_class = match preview_state {
        GitPublishPreviewState::ReadyToPublish => "waiting_review",
        GitPublishPreviewState::Blocked => "blocked",
        GitPublishPreviewState::Degraded => "degraded",
    };
    GitPublishActivityRecord {
        record_kind: GIT_PUBLISH_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(preview_ref)),
        job_family: "git_publish".to_string(),
        state_class: state_class.to_string(),
        partition: if preview_state == GitPublishPreviewState::ReadyToPublish {
            "active_review"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: mode.label().to_string(),
        detail_label: publish_detail_label(route, target),
        preview_ref: preview_ref.to_string(),
        result_ref: None,
        publish_journal_ref: None,
        open_details_command_id: "cmd:git.publish.review.open".to_string(),
        support_export_ref: support_export_ref.to_string(),
        recovery_ref: recovery_ref.to_string(),
    }
}

fn activity_for_result(
    preview: &GitPublishPreview,
    result_ref: &str,
    journal_ref: &str,
    recovery_ref: &str,
    outcome_state: GitPublishOutcomeState,
) -> GitPublishActivityRecord {
    GitPublishActivityRecord {
        record_kind: GIT_PUBLISH_ACTIVITY_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_ACTIVITY_SCHEMA_VERSION,
        activity_row_id: format!("activity.{}", sanitize_id(result_ref)),
        job_family: "git_publish".to_string(),
        state_class: outcome_state.activity_state_class().to_string(),
        partition: if outcome_state == GitPublishOutcomeState::Published {
            "completed"
        } else {
            "needs_attention"
        }
        .to_string(),
        summary_label: preview.mode.label().to_string(),
        detail_label: publish_detail_label(&preview.route, &preview.target),
        preview_ref: preview.preview_ref.clone(),
        result_ref: Some(result_ref.to_string()),
        publish_journal_ref: Some(journal_ref.to_string()),
        open_details_command_id: "cmd:git.publish.review.open".to_string(),
        support_export_ref: format!("support.{}", sanitize_id(result_ref)),
        recovery_ref: recovery_ref.to_string(),
    }
}

fn journal_for_result(
    preview: &GitPublishPreview,
    result_ref: &str,
    resolved_at: &str,
    outcome_state: GitPublishOutcomeState,
) -> GitPublishJournalRecord {
    let publish_journal_ref = format!("journal.{}", sanitize_id(result_ref));
    GitPublishJournalRecord {
        record_kind: GIT_PUBLISH_JOURNAL_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_JOURNAL_SCHEMA_VERSION,
        publish_journal_ref,
        command_id: preview.command_id.clone(),
        actor: preview.actor.clone(),
        source_class: "source_control_publish_review".to_string(),
        publish_mode: preview.mode.as_str().to_string(),
        route_ref: preview.route.route_ref.clone(),
        target_ref: preview.target.target_ref.clone(),
        started_at: preview.generated_at.clone(),
        resolved_at: resolved_at.to_string(),
        before_truth_source_ref: preview.truth_source_ref.clone(),
        side_effect_summary: side_effect_summary(preview, outcome_state),
        recovery_class: if outcome_state == GitPublishOutcomeState::Published {
            "audit_only_remote_ref_update"
        } else {
            "reopen_same_publish_review"
        }
        .to_string(),
        redaction_class: "git_publish_metadata".to_string(),
    }
}

fn result_for_preview(
    preview: &GitPublishPreview,
    resolved_at: &str,
    outcome_state: GitPublishOutcomeState,
    failure_reason: Option<String>,
    blocked_reasons: Vec<String>,
) -> GitPublishResult {
    let result_ref = format!(
        "{}.result.{}",
        preview.preview_ref,
        sanitize_id(outcome_state.as_str())
    );
    let journal = journal_for_result(preview, &result_ref, resolved_at, outcome_state);
    let recovery = recovery_for_result(preview, &result_ref, outcome_state, failure_reason.clone());
    let support_export = support_export_for_result(
        preview,
        &result_ref,
        &journal.publish_journal_ref,
        &recovery.recovery_ref,
        outcome_state,
    );
    let activity = activity_for_result(
        preview,
        &result_ref,
        &journal.publish_journal_ref,
        &recovery.recovery_ref,
        outcome_state,
    );
    GitPublishResult {
        record_kind: GIT_PUBLISH_RESULT_RECORD_KIND.to_string(),
        schema_version: GIT_PUBLISH_RESULT_SCHEMA_VERSION,
        result_ref,
        preview_ref: preview.preview_ref.clone(),
        resolved_at: resolved_at.to_string(),
        workspace_ref: preview.workspace_ref.clone(),
        repo_root: preview.repo_root.clone(),
        truth_source_ref: preview.truth_source_ref.clone(),
        mode: preview.mode,
        outcome_state,
        route: preview.route.clone(),
        target: preview.target.clone(),
        publish_journal: journal,
        activity,
        support_export,
        failure_recovery: recovery,
        failure_reason,
        blocked_reasons,
        merge_queue_supported: false,
        review_platform_state: preview.review_platform_state.clone(),
    }
}

fn publish_detail_label(route: &GitPublishRouteReview, target: &GitPublishTargetReview) -> String {
    let local = target.local_ref.as_deref().unwrap_or("<local ref>");
    let remote = target.remote_ref.as_deref().unwrap_or("<remote ref>");
    format!(
        "{} via {} from {}: {local} -> {}/{}",
        route.remote_name,
        route.route_class.label(),
        route.traffic_origin_label,
        route.remote_name,
        remote
    )
}

fn side_effect_summary(
    preview: &GitPublishPreview,
    outcome_state: GitPublishOutcomeState,
) -> String {
    let local = preview.target.local_ref.as_deref().unwrap_or("<local ref>");
    let remote = preview
        .target
        .remote_ref
        .as_deref()
        .unwrap_or("<remote ref>");
    match outcome_state {
        GitPublishOutcomeState::Published => format!(
            "published {local} to {}/{}",
            preview.route.remote_name, remote
        ),
        GitPublishOutcomeState::BlockedNoChangesMade => {
            "publish blocked before remote mutation".to_string()
        }
        GitPublishOutcomeState::Failed => format!(
            "publish attempt failed for {local} to {}/{}; local state preserved",
            preview.route.remote_name, remote
        ),
    }
}

fn parse_upstream(value: &str) -> Option<(String, String)> {
    let trimmed = value.trim();
    let (remote, branch) = trimmed.split_once('/')?;
    if remote.is_empty() || branch.is_empty() {
        return None;
    }
    Some((remote.to_string(), branch.to_string()))
}

fn normalized_branch_name(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(
        trimmed
            .strip_prefix("refs/heads/")
            .unwrap_or(trimmed)
            .to_string(),
    )
}

fn normalize_oid(value: &str) -> &str {
    value.trim()
}

fn redacted_remote_url_label(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "<remote url unavailable>".to_string();
    }
    if let Some((scheme, rest)) = trimmed.split_once("://") {
        let host = rest
            .split('/')
            .next()
            .unwrap_or("")
            .rsplit('@')
            .next()
            .unwrap_or("");
        if host.is_empty() {
            format!("{scheme}://<redacted>")
        } else {
            format!("{scheme}://{host}/<redacted>")
        }
    } else if let Some((user_host, _path)) = trimmed.split_once(':') {
        if user_host.contains('@') {
            let host = user_host.rsplit('@').next().unwrap_or(user_host);
            format!("ssh://{host}/<redacted>")
        } else {
            "local_path_remote".to_string()
        }
    } else if trimmed.starts_with('/') || trimmed.starts_with('.') {
        "local_path_remote".to_string()
    } else {
        "<remote url redacted>".to_string()
    }
}

fn remote_host_label(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if let Some((_scheme, rest)) = trimmed.split_once("://") {
        let host = rest
            .split('/')
            .next()
            .unwrap_or("")
            .rsplit('@')
            .next()
            .unwrap_or("");
        return (!host.is_empty()).then(|| host.to_string());
    }
    if let Some((user_host, _path)) = trimmed.split_once(':') {
        if user_host.contains('@') {
            let host = user_host.rsplit('@').next().unwrap_or(user_host);
            return (!host.is_empty()).then(|| host.to_string());
        }
    }
    None
}

fn preview_ref(request: &GitPublishRequest) -> String {
    let remote = request
        .remote_name
        .as_deref()
        .unwrap_or("upstream")
        .to_string();
    let branch = request
        .target_branch
        .as_deref()
        .or(request.local_branch.as_deref())
        .unwrap_or("current");
    format!(
        "git.publish.preview.{}.{}.{}.{}",
        sanitize_id(&request.workspace_ref),
        sanitize_id(request.mode.as_str()),
        sanitize_id(&remote),
        sanitize_id(branch)
    )
}

fn short_oid(value: &str) -> String {
    value.chars().take(12).collect()
}

fn stderr_or_status(output: &GitPublishCommandOutput) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stderr.is_empty() {
        return stderr;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !stdout.is_empty() {
        return stdout;
    }
    match output.status_code {
        Some(code) => format!("git exited with status {code}"),
        None => "git process terminated without a status code".to_string(),
    }
}

fn sanitize_id(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }
    let trimmed = output.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed
    }
}

fn observed_at_now() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("unix:{}", duration.as_secs()),
        Err(_) => "unix:0".to_string(),
    }
}
