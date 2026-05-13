//! Git and review activity-center event projection.
//!
//! This module keeps source-control and review actions as structured activity
//! events. The row model preserves branch, target, and action identity next to
//! the durable activity row, then exports the same record family into support
//! bundles without scraping rendered labels.

use serde::{Deserialize, Serialize};

use aureline_git::{
    GitBranchActivityRecord, GitBranchSupportExportRecord, GitCommitActivityRecord,
    GitCommitSupportExportRecord, GitMutationActivityRecord, GitMutationSupportExportRecord,
    GitPublishActivityRecord, GitPublishFailureRecoveryRecord, GitPublishSupportExportRecord,
};
use aureline_review::ReviewWorkspaceSeedPacket;

use crate::notifications::{PrivacyClass, RedactionClass, SeverityClass, SourceSubsystem};

use super::alpha::{
    ActivityCancellabilityClass, ActivityJobFamily, ActivityProgressForm, ActivityRow,
    ActivityRowAction, ActivityRowActionAvailability, ActivityRowActionKind,
    ActivityRowCollapseState, ActivityRowDisplayState, ActivityRowImpact, ActivityRowInput,
    ActivityRowProgress, ActivityRowStateClass, ActivityRowSupportLink, ActivityRowTimeline,
};

/// Stable record-kind tag for a Git/review event record.
pub const GIT_REVIEW_EVENT_RECORD_KIND: &str = "git_review_event_alpha_record";

/// Stable record-kind tag for a Git/review event snapshot.
pub const GIT_REVIEW_EVENT_SNAPSHOT_RECORD_KIND: &str = "git_review_event_alpha_snapshot";

/// Stable record-kind tag for the support/export projection.
pub const GIT_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str = "git_review_event_support_export";

/// Schema version for Git/review event records.
pub const GIT_REVIEW_EVENT_SCHEMA_VERSION: u32 = 1;

/// Schema version for Git/review event snapshots.
pub const GIT_REVIEW_EVENT_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Schema version for Git/review support exports.
pub const GIT_REVIEW_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing scope notice for Git/review activity fixtures.
pub const GIT_REVIEW_EVENT_SCOPE_NOTICE: &str =
    "Git/review activity alpha: Git mutation, publish, and review-workspace events preserve \
     branch, target, action, exact reopen, and support-export identity as structured fields. \
     Raw patch bodies, remote URLs, command output, comment bodies, and secrets stay out of \
     the default export.";

/// Git/review event family recorded in the activity center.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitReviewEventFamily {
    /// Local Git status refresh or state inspection.
    GitStatus,
    /// Path-level Git index or worktree mutation.
    GitMutation,
    /// Branch switch, create, checkout, or branch-target review.
    GitBranch,
    /// Local commit preview or result.
    GitCommit,
    /// Publish or push preview/result.
    GitPublish,
    /// Local review workspace, diff review, or comment-anchor seed.
    ReviewWorkspace,
}

impl GitReviewEventFamily {
    /// Stable token recorded on events and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GitStatus => "git_status",
            Self::GitMutation => "git_mutation",
            Self::GitBranch => "git_branch",
            Self::GitCommit => "git_commit",
            Self::GitPublish => "git_publish",
            Self::ReviewWorkspace => "review_workspace",
        }
    }

    /// Human-readable family label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GitStatus => "Git status",
            Self::GitMutation => "Git mutation",
            Self::GitBranch => "Git branch",
            Self::GitCommit => "Git commit",
            Self::GitPublish => "Git publish",
            Self::ReviewWorkspace => "Review workspace",
        }
    }

    /// Source subsystem used by activity rows for this family.
    pub const fn source_subsystem(self) -> SourceSubsystem {
        match self {
            Self::GitPublish => SourceSubsystem::ProviderBearing,
            Self::ReviewWorkspace => SourceSubsystem::ReviewAndDiff,
            Self::GitStatus | Self::GitMutation | Self::GitBranch | Self::GitCommit => {
                SourceSubsystem::ReviewAndDiff
            }
        }
    }
}

/// Event phase recorded for the Git/review lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitReviewEventPhase {
    /// A reviewed preview exists before mutation.
    Preview,
    /// A mutation completed locally.
    Applied,
    /// The event is blocked pending review, target repair, or policy.
    Blocked,
    /// The event failed and remains reopenable.
    Failed,
    /// Publish completed on the configured provider or remote.
    Published,
    /// A review workspace was opened or refreshed.
    ReviewOpened,
    /// A state refresh completed.
    Refreshed,
}

impl GitReviewEventPhase {
    /// Stable token recorded on events and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Applied => "applied",
            Self::Blocked => "blocked",
            Self::Failed => "failed",
            Self::Published => "published",
            Self::ReviewOpened => "review_opened",
            Self::Refreshed => "refreshed",
        }
    }
}

/// Git/review action class preserved on rows and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitReviewActionClass {
    /// Inspect repository status.
    InspectStatus,
    /// Preview or apply staging.
    Stage,
    /// Preview or apply unstaging.
    Unstage,
    /// Preview or apply a local discard.
    Discard,
    /// Switch to an existing branch.
    SwitchBranch,
    /// Create a branch.
    CreateBranch,
    /// Create or preview a commit.
    Commit,
    /// Reopen or execute a publish review.
    PublishReview,
    /// Open a local review workspace.
    OpenReviewWorkspace,
    /// Open a diff review target.
    OpenDiffReview,
}

impl GitReviewActionClass {
    /// Stable token recorded on events and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectStatus => "inspect_status",
            Self::Stage => "stage",
            Self::Unstage => "unstage",
            Self::Discard => "discard",
            Self::SwitchBranch => "switch_branch",
            Self::CreateBranch => "create_branch",
            Self::Commit => "commit",
            Self::PublishReview => "publish_review",
            Self::OpenReviewWorkspace => "open_review_workspace",
            Self::OpenDiffReview => "open_diff_review",
        }
    }

    /// True when this action can write to a provider or remote.
    pub const fn affects_provider_state(self) -> bool {
        matches!(self, Self::PublishReview)
    }

    /// True when this action can mutate local Git state.
    pub const fn mutates_local_git(self) -> bool {
        matches!(
            self,
            Self::Stage
                | Self::Unstage
                | Self::Discard
                | Self::SwitchBranch
                | Self::CreateBranch
                | Self::Commit
        )
    }
}

/// Target class for a Git/review event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitReviewTargetKind {
    /// Repository or worktree target.
    Repository,
    /// Path scope target.
    PathScope,
    /// Branch or ref target.
    BranchRef,
    /// Commit scope target.
    CommitScope,
    /// Remote ref target.
    RemoteRef,
    /// Review workspace target.
    ReviewWorkspace,
    /// Diff target inside a review workspace.
    DiffTarget,
}

impl GitReviewTargetKind {
    /// Stable token recorded on events and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Repository => "repository",
            Self::PathScope => "path_scope",
            Self::BranchRef => "branch_ref",
            Self::CommitScope => "commit_scope",
            Self::RemoteRef => "remote_ref",
            Self::ReviewWorkspace => "review_workspace",
            Self::DiffTarget => "diff_target",
        }
    }
}

/// Exact reopen link class for Git/review events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitReviewReopenKind {
    /// Reopens the durable activity-center row.
    ActivityRow,
    /// Reopens a Git mutation, branch, or commit details surface.
    GitReviewDetails,
    /// Reopens the publish review packet.
    GitPublishReview,
    /// Reopens the review workspace.
    ReviewWorkspace,
    /// Reopens a diff review surface.
    DiffReview,
    /// Reopens the structured support export.
    SupportExport,
}

impl GitReviewReopenKind {
    /// Stable token recorded on events and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityRow => "activity_row",
            Self::GitReviewDetails => "git_review_details",
            Self::GitPublishReview => "git_publish_review",
            Self::ReviewWorkspace => "review_workspace",
            Self::DiffReview => "diff_review",
            Self::SupportExport => "support_export",
        }
    }
}

/// Branch and head context preserved for a Git/review event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewBranchContext {
    /// Redaction-safe current branch label when attached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    /// Opaque branch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_ref: Option<String>,
    /// Opaque upstream ref when configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_ref: Option<String>,
    /// Opaque head revision ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_ref: Option<String>,
    /// Provider overlay state from the owning Git/review packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_overlay_state: Option<String>,
    /// Local diff authority from the review seed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_diff_authority: Option<String>,
}

impl GitReviewBranchContext {
    /// Builds a branch context from explicit optional refs.
    pub fn new(
        branch_label: Option<String>,
        branch_ref: Option<String>,
        upstream_ref: Option<String>,
        head_revision_ref: Option<String>,
    ) -> Self {
        Self {
            branch_label,
            branch_ref,
            upstream_ref,
            head_revision_ref,
            provider_overlay_state: None,
            local_diff_authority: None,
        }
    }

    /// Builds a detached or unknown branch context from a head ref.
    pub fn detached(head_revision_ref: impl Into<String>) -> Self {
        Self {
            branch_label: None,
            branch_ref: None,
            upstream_ref: None,
            head_revision_ref: Some(head_revision_ref.into()),
            provider_overlay_state: None,
            local_diff_authority: None,
        }
    }

    /// Returns a copy with provider overlay and local authority labels.
    pub fn with_review_authority(
        mut self,
        provider_overlay_state: impl Into<String>,
        local_diff_authority: impl Into<String>,
    ) -> Self {
        self.provider_overlay_state = Some(provider_overlay_state.into());
        self.local_diff_authority = Some(local_diff_authority.into());
        self
    }

    /// True when the context carries branch, upstream, or head identity.
    pub fn has_identity(&self) -> bool {
        self.branch_label
            .as_deref()
            .or(self.branch_ref.as_deref())
            .or(self.upstream_ref.as_deref())
            .or(self.head_revision_ref.as_deref())
            .is_some_and(|value| !value.trim().is_empty())
    }

    fn scope_label(&self, fallback: &str) -> String {
        self.branch_label
            .clone()
            .or_else(|| self.branch_ref.clone())
            .or_else(|| self.head_revision_ref.clone())
            .unwrap_or_else(|| fallback.to_string())
    }
}

/// Action identity preserved for activity-center and support consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewActionIdentity {
    /// Stable action id.
    pub action_id: String,
    /// Action class.
    pub action_class: GitReviewActionClass,
    /// Command id that opens, previews, or mutates the target.
    pub command_id: String,
    /// Source record that produced this action.
    pub source_record_ref: String,
    /// Preview ref when the action was reviewed before mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_ref: Option<String>,
    /// Result ref when the action resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Journal ref when a mutation or publish attempt occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub journal_ref: Option<String>,
    /// Recovery ref when a failed publish or destructive action has a reopen path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_ref: Option<String>,
    /// Redaction-safe side-effect class.
    pub side_effect_class: String,
    /// True only for an action that intentionally reissues a mutation.
    pub reissues_original_side_effect: bool,
}

impl GitReviewActionIdentity {
    /// Returns true when this action has stable class, id, and command identity.
    pub fn has_identity(&self) -> bool {
        !self.action_id.trim().is_empty()
            && !self.command_id.trim().is_empty()
            && !self.source_record_ref.trim().is_empty()
    }
}

/// Target identity preserved for activity-center and support consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewTargetIdentity {
    /// Opaque canonical target ref.
    pub canonical_target_ref: String,
    /// Target kind.
    pub target_kind: GitReviewTargetKind,
    /// Redaction-safe target label.
    pub target_label: String,
    /// Opaque scope ref when the event targets a scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_ref: Option<String>,
    /// Target refs included in the action.
    #[serde(default)]
    pub target_refs: Vec<String>,
    /// Review workspace ref when the event opens or publishes review state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_workspace_ref: Option<String>,
    /// Route ref when the event crosses a provider or remote boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_ref: Option<String>,
}

impl GitReviewTargetIdentity {
    /// Returns true when the target has an exact ref and visible kind/label.
    pub fn has_identity(&self) -> bool {
        !self.canonical_target_ref.trim().is_empty() && !self.target_label.trim().is_empty()
    }
}

/// Exact reopen link for a Git/review event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewExactReopenLink {
    /// Stable link ref.
    pub reopen_link_ref: String,
    /// Reopen link kind.
    pub reopen_kind: GitReviewReopenKind,
    /// Command id that resolves the link.
    pub command_id: String,
    /// Exact target identity the command opens.
    pub target_identity_ref: String,
    /// Reviewable link label.
    pub label: String,
    /// Reason when a link must revalidate before mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revalidation_required_reason_label: Option<String>,
}

impl GitReviewExactReopenLink {
    /// Builds an exact reopen link.
    pub fn new(
        reopen_kind: GitReviewReopenKind,
        command_id: impl Into<String>,
        target_identity_ref: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        let command_id = command_id.into();
        let target_identity_ref = target_identity_ref.into();
        Self {
            reopen_link_ref: format!(
                "reopen.git_review.{}.{}",
                reopen_kind.as_str(),
                sanitize_id(&target_identity_ref)
            ),
            reopen_kind,
            command_id,
            target_identity_ref,
            label: label.into(),
            revalidation_required_reason_label: None,
        }
    }

    /// Returns true when the link resolves to a command and exact target.
    pub fn resolves_exact_target(&self) -> bool {
        !self.command_id.trim().is_empty() && !self.target_identity_ref.trim().is_empty()
    }
}

/// Support projection fields attached to a Git/review event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSupportProjection {
    /// Stable support-pack item id.
    pub support_pack_item_id: String,
    /// Stable support export ref.
    pub support_export_ref: String,
    /// Bundle member path ref for the structured export.
    pub bundle_member_path_ref: String,
    /// Redaction class for the structured export.
    pub redaction_class: RedactionClass,
    /// Structured fields included in support export.
    pub export_field_refs: Vec<String>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl GitReviewSupportProjection {
    /// Builds the default metadata-only support projection for an event.
    pub fn metadata_safe(
        support_pack_item_id: impl Into<String>,
        support_export_ref: impl Into<String>,
        bundle_member_path_ref: impl Into<String>,
    ) -> Self {
        Self {
            support_pack_item_id: support_pack_item_id.into(),
            support_export_ref: support_export_ref.into(),
            bundle_member_path_ref: bundle_member_path_ref.into(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            export_field_refs: vec![
                "export.git_review.branch".to_string(),
                "export.git_review.target".to_string(),
                "export.git_review.action".to_string(),
                "export.git_review.reopen".to_string(),
            ],
            raw_private_material_excluded: true,
        }
    }
}

/// Structured Git/review context embedded in an activity-center row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewActivityContext {
    /// Source Git/review event id.
    pub event_id: String,
    /// Event family.
    pub event_family: GitReviewEventFamily,
    /// Branch and head context.
    pub branch: GitReviewBranchContext,
    /// Target identity.
    pub target: GitReviewTargetIdentity,
    /// Action identity.
    pub action: GitReviewActionIdentity,
    /// Exact reopen links carried by the row.
    pub exact_reopen_links: Vec<GitReviewExactReopenLink>,
    /// Support export ref for the same event.
    pub support_export_ref: String,
}

impl GitReviewActivityContext {
    /// Returns true when branch, target, and action identity are all present.
    pub fn has_branch_target_action_identity(&self) -> bool {
        self.branch.has_identity() && self.target.has_identity() && self.action.has_identity()
    }

    /// Returns true when every advertised reopen link is exact.
    pub fn exact_reopen_links_resolve(&self) -> bool {
        !self.exact_reopen_links.is_empty()
            && self
                .exact_reopen_links
                .iter()
                .all(GitReviewExactReopenLink::resolves_exact_target)
    }
}

/// Input used to build a Git/review event record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitReviewEventInput {
    /// Stable event id.
    pub event_id: String,
    /// Timestamp for deterministic fixtures.
    pub occurred_at: String,
    /// Event family.
    pub event_family: GitReviewEventFamily,
    /// Event phase.
    pub phase: GitReviewEventPhase,
    /// Activity-row state class.
    pub state_class: ActivityRowStateClass,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Actor identity ref.
    pub actor_identity_ref: String,
    /// Actor label.
    pub actor_label: String,
    /// Workspace ref.
    pub workspace_ref: String,
    /// Activity summary label.
    pub summary_label: String,
    /// Activity detail label.
    pub detail_label: String,
    /// Branch context.
    pub branch: GitReviewBranchContext,
    /// Target identity.
    pub target: GitReviewTargetIdentity,
    /// Action identity.
    pub action: GitReviewActionIdentity,
    /// Exact reopen links.
    pub exact_reopen_links: Vec<GitReviewExactReopenLink>,
    /// Support projection.
    pub support_projection: GitReviewSupportProjection,
}

/// Canonical Git/review event record consumed by activity and support paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewEventRecord {
    /// Optional schema ref used by checked-in fixtures.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable event id.
    pub event_id: String,
    /// Canonical event id used by activity and support joins.
    pub canonical_event_id: String,
    /// Activity row id generated for this event.
    pub activity_row_id: String,
    /// Durable job id generated for this event.
    pub durable_job_id: String,
    /// Timestamp for this event.
    pub occurred_at: String,
    /// Event family.
    pub event_family: GitReviewEventFamily,
    /// Event phase.
    pub phase: GitReviewEventPhase,
    /// Activity-row state class.
    pub state_class: ActivityRowStateClass,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Actor identity ref.
    pub actor_identity_ref: String,
    /// Actor label.
    pub actor_label: String,
    /// Workspace ref.
    pub workspace_ref: String,
    /// Activity summary label.
    pub summary_label: String,
    /// Activity detail label.
    pub detail_label: String,
    /// Branch and head context.
    pub branch: GitReviewBranchContext,
    /// Target identity.
    pub target: GitReviewTargetIdentity,
    /// Action identity.
    pub action: GitReviewActionIdentity,
    /// Exact reopen links.
    pub exact_reopen_links: Vec<GitReviewExactReopenLink>,
    /// Support projection for this event.
    pub support_projection: GitReviewSupportProjection,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl GitReviewEventRecord {
    /// Builds a Git/review event from explicit structured input.
    pub fn from_input(input: GitReviewEventInput) -> Self {
        let safe_id = sanitize_id(&input.event_id);
        Self {
            schema_ref: None,
            record_kind: GIT_REVIEW_EVENT_RECORD_KIND.to_string(),
            schema_version: GIT_REVIEW_EVENT_SCHEMA_VERSION,
            event_id: input.event_id.clone(),
            canonical_event_id: input.event_id,
            activity_row_id: format!("activity.git_review.{safe_id}"),
            durable_job_id: format!("durable.git_review.{safe_id}"),
            occurred_at: input.occurred_at,
            event_family: input.event_family,
            phase: input.phase,
            state_class: input.state_class,
            severity_class: input.severity_class,
            source_subsystem: input.event_family.source_subsystem(),
            actor_identity_ref: input.actor_identity_ref,
            actor_label: input.actor_label,
            workspace_ref: input.workspace_ref,
            summary_label: input.summary_label,
            detail_label: input.detail_label,
            branch: input.branch,
            target: input.target,
            action: input.action,
            exact_reopen_links: input.exact_reopen_links,
            raw_private_material_excluded: input.support_projection.raw_private_material_excluded,
            support_projection: input.support_projection,
        }
    }

    /// Builds a Git/review event from a Git mutation activity/support pair.
    pub fn from_git_mutation_records(
        occurred_at: impl Into<String>,
        workspace_ref: impl Into<String>,
        branch: GitReviewBranchContext,
        activity: &GitMutationActivityRecord,
        support: &GitMutationSupportExportRecord,
    ) -> Self {
        let event_id = format!(
            "event.git_review.{}",
            sanitize_id(&activity.activity_row_id)
        );
        let action_class = match support.operation_kind.as_str() {
            "stage" => GitReviewActionClass::Stage,
            "unstage" => GitReviewActionClass::Unstage,
            "discard" => GitReviewActionClass::Discard,
            _ => GitReviewActionClass::OpenDiffReview,
        };
        let action = GitReviewActionIdentity {
            action_id: format!(
                "action.git_review.{}",
                sanitize_id(&activity.activity_row_id)
            ),
            action_class,
            command_id: activity.open_details_command_id.clone(),
            source_record_ref: activity.preview_ref.clone(),
            preview_ref: Some(activity.preview_ref.clone()),
            result_ref: support.result_ref.clone(),
            journal_ref: support.mutation_journal_ref.clone(),
            recovery_ref: activity.checkpoint_refs.first().cloned(),
            side_effect_class: support.operation_kind.clone(),
            reissues_original_side_effect: false,
        };
        let target = GitReviewTargetIdentity {
            canonical_target_ref: support.scope_ref.clone(),
            target_kind: GitReviewTargetKind::PathScope,
            target_label: "Selected Git paths".to_string(),
            scope_ref: Some(support.scope_ref.clone()),
            target_refs: Vec::new(),
            review_workspace_ref: None,
            route_ref: None,
        };
        let support_projection = GitReviewSupportProjection::metadata_safe(
            "support.item.git_review.mutation",
            support.support_export_ref.clone(),
            format!(
                "manifest/git_review_activity/{}.json",
                sanitize_id(&support.support_export_ref)
            ),
        );
        Self::from_input(GitReviewEventInput {
            event_id,
            occurred_at: occurred_at.into(),
            event_family: GitReviewEventFamily::GitMutation,
            phase: phase_from_support_token(&support.phase),
            state_class: state_from_activity_token(&activity.state_class),
            severity_class: severity_from_state_token(&activity.state_class),
            actor_identity_ref: "id:actor:local-user".to_string(),
            actor_label: "Local user".to_string(),
            workspace_ref: workspace_ref.into(),
            summary_label: activity.summary_label.clone(),
            detail_label: activity.detail_label.clone(),
            branch,
            target,
            action,
            exact_reopen_links: vec![GitReviewExactReopenLink::new(
                GitReviewReopenKind::GitReviewDetails,
                activity.open_details_command_id.clone(),
                activity.preview_ref.clone(),
                "Open Git mutation details",
            )],
            support_projection,
        })
    }

    /// Builds a Git/review event from a branch operation activity/support pair.
    pub fn from_git_branch_records(
        occurred_at: impl Into<String>,
        workspace_ref: impl Into<String>,
        branch: GitReviewBranchContext,
        activity: &GitBranchActivityRecord,
        support: &GitBranchSupportExportRecord,
    ) -> Self {
        let action_class = match support.operation_kind.as_str() {
            "create_branch" => GitReviewActionClass::CreateBranch,
            _ => GitReviewActionClass::SwitchBranch,
        };
        let event_id = format!(
            "event.git_review.{}",
            sanitize_id(&activity.activity_row_id)
        );
        let action = GitReviewActionIdentity {
            action_id: format!(
                "action.git_review.{}",
                sanitize_id(&activity.activity_row_id)
            ),
            action_class,
            command_id: activity.open_details_command_id.clone(),
            source_record_ref: activity.preview_ref.clone(),
            preview_ref: Some(activity.preview_ref.clone()),
            result_ref: support.result_ref.clone(),
            journal_ref: support.branch_journal_ref.clone(),
            recovery_ref: None,
            side_effect_class: support.operation_kind.clone(),
            reissues_original_side_effect: false,
        };
        let target = GitReviewTargetIdentity {
            canonical_target_ref: support.target_ref.clone(),
            target_kind: GitReviewTargetKind::BranchRef,
            target_label: "Branch target".to_string(),
            scope_ref: None,
            target_refs: vec![support.target_ref.clone()],
            review_workspace_ref: None,
            route_ref: None,
        };
        let support_projection = GitReviewSupportProjection::metadata_safe(
            "support.item.git_review.branch",
            support.support_export_ref.clone(),
            format!(
                "manifest/git_review_activity/{}.json",
                sanitize_id(&support.support_export_ref)
            ),
        );
        Self::from_input(GitReviewEventInput {
            event_id,
            occurred_at: occurred_at.into(),
            event_family: GitReviewEventFamily::GitBranch,
            phase: phase_from_support_token(&support.phase),
            state_class: state_from_activity_token(&activity.state_class),
            severity_class: severity_from_state_token(&activity.state_class),
            actor_identity_ref: "id:actor:local-user".to_string(),
            actor_label: "Local user".to_string(),
            workspace_ref: workspace_ref.into(),
            summary_label: activity.summary_label.clone(),
            detail_label: activity.detail_label.clone(),
            branch,
            target,
            action,
            exact_reopen_links: vec![GitReviewExactReopenLink::new(
                GitReviewReopenKind::GitReviewDetails,
                activity.open_details_command_id.clone(),
                activity.preview_ref.clone(),
                "Open branch operation details",
            )],
            support_projection,
        })
    }

    /// Builds a Git/review event from a commit activity/support pair.
    pub fn from_git_commit_records(
        occurred_at: impl Into<String>,
        branch: GitReviewBranchContext,
        activity: &GitCommitActivityRecord,
        support: &GitCommitSupportExportRecord,
    ) -> Self {
        let event_id = format!(
            "event.git_review.{}",
            sanitize_id(&activity.activity_row_id)
        );
        let action = GitReviewActionIdentity {
            action_id: format!(
                "action.git_review.{}",
                sanitize_id(&activity.activity_row_id)
            ),
            action_class: GitReviewActionClass::Commit,
            command_id: activity.open_details_command_id.clone(),
            source_record_ref: activity.preview_ref.clone(),
            preview_ref: Some(activity.preview_ref.clone()),
            result_ref: support.result_ref.clone(),
            journal_ref: support.commit_journal_ref.clone(),
            recovery_ref: None,
            side_effect_class: support.commit_mode.clone(),
            reissues_original_side_effect: false,
        };
        let target = GitReviewTargetIdentity {
            canonical_target_ref: support.scope_ref.clone(),
            target_kind: GitReviewTargetKind::CommitScope,
            target_label: "Commit scope".to_string(),
            scope_ref: Some(support.scope_ref.clone()),
            target_refs: Vec::new(),
            review_workspace_ref: None,
            route_ref: None,
        };
        let support_projection = GitReviewSupportProjection::metadata_safe(
            "support.item.git_review.commit",
            support.support_export_ref.clone(),
            format!(
                "manifest/git_review_activity/{}.json",
                sanitize_id(&support.support_export_ref)
            ),
        );
        Self::from_input(GitReviewEventInput {
            event_id,
            occurred_at: occurred_at.into(),
            event_family: GitReviewEventFamily::GitCommit,
            phase: phase_from_support_token(&support.phase),
            state_class: state_from_activity_token(&activity.state_class),
            severity_class: severity_from_state_token(&activity.state_class),
            actor_identity_ref: "id:actor:local-user".to_string(),
            actor_label: "Local user".to_string(),
            workspace_ref: support.workspace_ref.clone(),
            summary_label: activity.summary_label.clone(),
            detail_label: activity.detail_label.clone(),
            branch,
            target,
            action,
            exact_reopen_links: vec![GitReviewExactReopenLink::new(
                GitReviewReopenKind::GitReviewDetails,
                activity.open_details_command_id.clone(),
                activity.preview_ref.clone(),
                "Open commit details",
            )],
            support_projection,
        })
    }

    /// Builds a Git/review event from a publish activity/support pair.
    pub fn from_git_publish_records(
        occurred_at: impl Into<String>,
        branch: GitReviewBranchContext,
        activity: &GitPublishActivityRecord,
        support: &GitPublishSupportExportRecord,
        recovery: Option<&GitPublishFailureRecoveryRecord>,
    ) -> Self {
        let event_id = format!(
            "event.git_review.{}",
            sanitize_id(&activity.activity_row_id)
        );
        let action = GitReviewActionIdentity {
            action_id: format!(
                "action.git_review.{}",
                sanitize_id(&activity.activity_row_id)
            ),
            action_class: GitReviewActionClass::PublishReview,
            command_id: recovery
                .map(|record| record.reopen_command_id.clone())
                .unwrap_or_else(|| activity.open_details_command_id.clone()),
            source_record_ref: activity.preview_ref.clone(),
            preview_ref: Some(activity.preview_ref.clone()),
            result_ref: support.result_ref.clone(),
            journal_ref: support.publish_journal_ref.clone(),
            recovery_ref: Some(activity.recovery_ref.clone()),
            side_effect_class: support.publish_mode.clone(),
            reissues_original_side_effect: false,
        };
        let target = GitReviewTargetIdentity {
            canonical_target_ref: support.target_ref.clone(),
            target_kind: GitReviewTargetKind::RemoteRef,
            target_label: "Remote ref publish target".to_string(),
            scope_ref: None,
            target_refs: vec![support.target_ref.clone()],
            review_workspace_ref: None,
            route_ref: Some(support.route_ref.clone()),
        };
        let support_projection = GitReviewSupportProjection::metadata_safe(
            "support.item.git_review.publish",
            support.support_export_ref.clone(),
            format!(
                "manifest/git_review_activity/{}.json",
                sanitize_id(&support.support_export_ref)
            ),
        );
        let mut exact_reopen_links = vec![GitReviewExactReopenLink::new(
            GitReviewReopenKind::GitReviewDetails,
            activity.open_details_command_id.clone(),
            activity.preview_ref.clone(),
            "Open publish details",
        )];
        if let Some(recovery) = recovery {
            exact_reopen_links.push(GitReviewExactReopenLink::new(
                GitReviewReopenKind::GitPublishReview,
                recovery.reopen_command_id.clone(),
                recovery.reopen_preview_ref.clone(),
                "Reopen publish review",
            ));
        }
        Self::from_input(GitReviewEventInput {
            event_id,
            occurred_at: occurred_at.into(),
            event_family: GitReviewEventFamily::GitPublish,
            phase: phase_from_support_token(&support.phase),
            state_class: state_from_activity_token(&activity.state_class),
            severity_class: severity_from_state_token(&activity.state_class),
            actor_identity_ref: "id:actor:local-user".to_string(),
            actor_label: "Local user".to_string(),
            workspace_ref: support.workspace_ref.clone(),
            summary_label: activity.summary_label.clone(),
            detail_label: activity.detail_label.clone(),
            branch,
            target,
            action,
            exact_reopen_links,
            support_projection,
        })
    }

    /// Builds a Git/review event from a review workspace seed packet.
    pub fn from_review_workspace_seed(
        packet: &ReviewWorkspaceSeedPacket,
        actor_identity_ref: impl Into<String>,
    ) -> Self {
        let workspace = &packet.review_workspace;
        let local = workspace.local_locator.as_ref();
        let branch = GitReviewBranchContext {
            branch_label: local.map(|locator| locator.branch_or_worktree_ref.clone()),
            branch_ref: local.map(|locator| locator.branch_or_worktree_ref.clone()),
            upstream_ref: local.and_then(|locator| locator.base_revision_ref.clone()),
            head_revision_ref: local.and_then(|locator| locator.head_revision_ref.clone()),
            provider_overlay_state: workspace
                .provider_overlay
                .as_ref()
                .map(|overlay| overlay.provider_overlay_freshness_class.clone()),
            local_diff_authority: Some(workspace.provider_authority_class.clone()),
        };
        let target = GitReviewTargetIdentity {
            canonical_target_ref: workspace.review_workspace_id.clone(),
            target_kind: GitReviewTargetKind::ReviewWorkspace,
            target_label: workspace.summary_label.clone(),
            scope_ref: local.map(|locator| locator.branch_or_worktree_ref.clone()),
            target_refs: packet
                .diff_entries
                .iter()
                .map(|entry| entry.compare_target_ref.clone())
                .collect(),
            review_workspace_ref: Some(workspace.review_workspace_id.clone()),
            route_ref: None,
        };
        let action = GitReviewActionIdentity {
            action_id: format!(
                "action.git_review.review_workspace.{}",
                sanitize_id(&workspace.review_workspace_id)
            ),
            action_class: GitReviewActionClass::OpenReviewWorkspace,
            command_id: "cmd:review.workspace.open".to_string(),
            source_record_ref: workspace.review_workspace_id.clone(),
            preview_ref: None,
            result_ref: None,
            journal_ref: None,
            recovery_ref: None,
            side_effect_class: "local_review_only".to_string(),
            reissues_original_side_effect: false,
        };
        let support_export_ref = format!(
            "support.export.git_review.review_workspace.{}",
            sanitize_id(&workspace.review_workspace_id)
        );
        let support_projection = GitReviewSupportProjection::metadata_safe(
            "support.item.git_review.review_workspace",
            support_export_ref.clone(),
            format!(
                "manifest/git_review_activity/{}.json",
                sanitize_id(&support_export_ref)
            ),
        );
        Self::from_input(GitReviewEventInput {
            event_id: format!(
                "event.git_review.review_workspace.{}",
                sanitize_id(&workspace.review_workspace_id)
            ),
            occurred_at: packet.generated_at.clone(),
            event_family: GitReviewEventFamily::ReviewWorkspace,
            phase: GitReviewEventPhase::ReviewOpened,
            state_class: ActivityRowStateClass::Completed,
            severity_class: SeverityClass::Info,
            actor_identity_ref: actor_identity_ref.into(),
            actor_label: "Reviewer".to_string(),
            workspace_ref: local
                .map(|locator| locator.workspace_id_ref.clone())
                .unwrap_or_else(|| workspace.review_workspace_id.clone()),
            summary_label: "Review workspace opened".to_string(),
            detail_label: packet.inspection.summary_label.clone(),
            branch,
            target,
            action,
            exact_reopen_links: vec![GitReviewExactReopenLink::new(
                GitReviewReopenKind::ReviewWorkspace,
                "cmd:review.workspace.open",
                workspace.review_workspace_id.clone(),
                "Open review workspace",
            )],
            support_projection,
        })
    }

    /// Returns the context embedded in an activity-center row.
    pub fn activity_context(&self) -> GitReviewActivityContext {
        GitReviewActivityContext {
            event_id: self.event_id.clone(),
            event_family: self.event_family,
            branch: self.branch.clone(),
            target: self.target.clone(),
            action: self.action.clone(),
            exact_reopen_links: self.exact_reopen_links.clone(),
            support_export_ref: self.support_projection.support_export_ref.clone(),
        }
    }

    /// Builds an activity-center row that preserves the structured event context.
    pub fn to_activity_row(&self) -> ActivityRow {
        let primary_reopen = self.primary_reopen_link();
        ActivityRow::from_input(ActivityRowInput {
            activity_row_id: self.activity_row_id.clone(),
            durable_job_id: self.durable_job_id.clone(),
            canonical_event_id: self.canonical_event_id.clone(),
            canonical_object_target_ref: self.target.canonical_target_ref.clone(),
            exact_reopen_identity_ref: primary_reopen.target_identity_ref.clone(),
            job_family: ActivityJobFamily::GitReview,
            source_subsystem: self.source_subsystem,
            actor_identity_ref: self.actor_identity_ref.clone(),
            actor_or_subsystem_label: self.actor_label.clone(),
            execution_origin_class: self.event_family.as_str().to_string(),
            severity_class: self.severity_class,
            privacy_class: PrivacyClass::WorkspaceSensitive,
            summary_label: self.summary_label.clone(),
            target_label: self.target.target_label.clone(),
            target_scope_label: self.branch.scope_label(&self.workspace_ref),
            state_class: self.state_class,
            progress: ActivityRowProgress {
                forms: vec![progress_form_for_state(self.state_class)],
                phase_label: self.phase.as_str().to_string(),
                progress_bar: None,
                queue_reason_label: None,
                approval_source_label: if matches!(
                    self.state_class,
                    ActivityRowStateClass::NeedsApproval
                ) {
                    Some("Git/review target requires review".to_string())
                } else {
                    None
                },
                actor_or_subsystem_label: self.event_family.label().to_string(),
                age_label: "Recorded".to_string(),
                indeterminate_reason_label: None,
                expected_boundary_class: expected_boundary_for(self.event_family).to_string(),
                cancellability_class: if self.state_class.is_terminal() {
                    ActivityCancellabilityClass::AlreadyTerminal
                } else {
                    ActivityCancellabilityClass::NotCancellable
                },
                detail_or_evidence_ref: Some(self.support_projection.support_export_ref.clone()),
            },
            timeline: ActivityRowTimeline {
                minted_at: self.occurred_at.clone(),
                queued_at: None,
                started_at: Some(self.occurred_at.clone()),
                last_observed_at: self.occurred_at.clone(),
                finished_at: self
                    .state_class
                    .is_terminal()
                    .then(|| self.occurred_at.clone()),
                archived_at: None,
                superseded_by_row_id_ref: None,
                retention_label: "Retained until resolved or archived with Git/review context"
                    .to_string(),
            },
            impact: self.activity_impact(),
            actions: vec![ActivityRowAction {
                action_id: format!("action.activity.git_review.{}", sanitize_id(&self.event_id)),
                action_kind: ActivityRowActionKind::OpenDetails,
                label: primary_reopen.label.clone(),
                command_id: Some(primary_reopen.command_id.clone()),
                availability_class: ActivityRowActionAvailability::Enabled,
                disabled_reason_label: None,
                target_identity_ref: primary_reopen.target_identity_ref.clone(),
                preserves_durable_history: true,
                reissues_original_side_effect: false,
            }],
            display: ActivityRowDisplayState {
                collapse_state: ActivityRowCollapseState::CollapsedSummary,
                can_expand_inline: true,
                expand_reveals_label: "branch, target, action, and reopen links".to_string(),
            },
            support_link: ActivityRowSupportLink {
                exportable: true,
                support_pack_item_id: Some(self.support_projection.support_pack_item_id.clone()),
                bundle_member_path_ref: Some(
                    self.support_projection.bundle_member_path_ref.clone(),
                ),
                redaction_class: self.support_projection.redaction_class,
                raw_private_material_excluded: self
                    .support_projection
                    .raw_private_material_excluded,
                export_field_refs: self.support_projection.export_field_refs.clone(),
            },
            git_review_context: Some(self.activity_context()),
            occurrence_count: 1,
        })
    }

    /// Returns true when branch, target, and action identity are all present.
    pub fn has_branch_target_action_identity(&self) -> bool {
        self.branch.has_identity() && self.target.has_identity() && self.action.has_identity()
    }

    /// Returns true when review/publish events have exact reopen links.
    pub fn review_or_publish_has_exact_reopen(&self) -> bool {
        match self.event_family {
            GitReviewEventFamily::GitPublish => {
                self.has_exact_reopen_kind(GitReviewReopenKind::GitPublishReview)
            }
            GitReviewEventFamily::ReviewWorkspace => {
                self.has_exact_reopen_kind(GitReviewReopenKind::ReviewWorkspace)
            }
            _ => true,
        }
    }

    /// Returns true when at least one exact reopen link of `kind` resolves.
    pub fn has_exact_reopen_kind(&self, kind: GitReviewReopenKind) -> bool {
        self.exact_reopen_links
            .iter()
            .any(|link| link.reopen_kind == kind && link.resolves_exact_target())
    }

    fn primary_reopen_link(&self) -> &GitReviewExactReopenLink {
        self.exact_reopen_links
            .iter()
            .find(|link| link.reopen_kind != GitReviewReopenKind::ActivityRow)
            .or_else(|| self.exact_reopen_links.first())
            .expect("Git/review events require at least one exact reopen link")
    }

    fn activity_impact(&self) -> ActivityRowImpact {
        ActivityRowImpact {
            affects_cost: false,
            affects_policy: false,
            affects_network: matches!(self.event_family, GitReviewEventFamily::GitPublish),
            affects_trust: self.action.action_class.mutates_local_git()
                || self.action.action_class.affects_provider_state(),
            affects_provider_state: self.action.action_class.affects_provider_state(),
            affects_recovery_posture: matches!(
                self.action.action_class,
                GitReviewActionClass::Discard
                    | GitReviewActionClass::SwitchBranch
                    | GitReviewActionClass::CreateBranch
                    | GitReviewActionClass::PublishReview
            ),
            detail_or_evidence_required: true,
            impact_summary_sentence: format!(
                "{} event preserves branch, target, action, and exact reopen identity.",
                self.event_family.label()
            ),
        }
    }
}

/// Snapshot of Git/review events and their activity-center projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewEventSnapshot {
    /// Optional schema ref used by checked-in fixtures.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Generation timestamp.
    pub generated_at: String,
    /// Scope notice.
    pub scope_notice: String,
    /// Canonical Git/review events.
    pub events: Vec<GitReviewEventRecord>,
    /// Activity rows projected from the events.
    pub activity_rows: Vec<ActivityRow>,
    /// Count of events carrying branch, target, and action identity.
    pub branch_target_action_complete_count: usize,
    /// Count of exact reopen links carried by all events.
    pub exact_reopen_link_count: usize,
    /// Count of events that can be included in support exports.
    pub support_exportable_event_count: usize,
}

impl GitReviewEventSnapshot {
    /// Builds a deterministic snapshot from events.
    pub fn from_events(
        generated_at: impl Into<String>,
        mut events: Vec<GitReviewEventRecord>,
    ) -> Self {
        events.sort_by(|left, right| {
            left.occurred_at
                .cmp(&right.occurred_at)
                .then_with(|| left.event_id.cmp(&right.event_id))
        });
        let activity_rows = events
            .iter()
            .map(GitReviewEventRecord::to_activity_row)
            .collect::<Vec<_>>();
        let branch_target_action_complete_count = events
            .iter()
            .filter(|event| event.has_branch_target_action_identity())
            .count();
        let exact_reopen_link_count = events
            .iter()
            .map(|event| event.exact_reopen_links.len())
            .sum();
        let support_exportable_event_count = events
            .iter()
            .filter(|event| event.support_projection.raw_private_material_excluded)
            .count();
        Self {
            schema_ref: None,
            record_kind: GIT_REVIEW_EVENT_SNAPSHOT_RECORD_KIND.to_string(),
            schema_version: GIT_REVIEW_EVENT_SNAPSHOT_SCHEMA_VERSION,
            generated_at: generated_at.into(),
            scope_notice: GIT_REVIEW_EVENT_SCOPE_NOTICE.to_string(),
            events,
            activity_rows,
            branch_target_action_complete_count,
            exact_reopen_link_count,
            support_exportable_event_count,
        }
    }

    /// Returns true when every activity row carries structured Git/review context.
    pub fn all_activity_rows_preserve_context(&self) -> bool {
        self.activity_rows.iter().all(|row| {
            row.git_review_context
                .as_ref()
                .is_some_and(GitReviewActivityContext::has_branch_target_action_identity)
        })
    }

    /// Returns true when publish and review actions carry exact reopen links.
    pub fn publish_and_review_actions_have_exact_reopen(&self) -> bool {
        self.events
            .iter()
            .all(GitReviewEventRecord::review_or_publish_has_exact_reopen)
    }
}

/// Support-export row for one Git/review event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSupportExportRow {
    /// Stable event id.
    pub event_id: String,
    /// Activity row id.
    pub activity_row_id: String,
    /// Event family.
    pub event_family: GitReviewEventFamily,
    /// Event phase.
    pub phase: GitReviewEventPhase,
    /// Activity-row state class.
    pub state_class: ActivityRowStateClass,
    /// Workspace ref.
    pub workspace_ref: String,
    /// Branch and head context.
    pub branch: GitReviewBranchContext,
    /// Target identity.
    pub target: GitReviewTargetIdentity,
    /// Action identity.
    pub action: GitReviewActionIdentity,
    /// Exact reopen links.
    pub exact_reopen_links: Vec<GitReviewExactReopenLink>,
    /// Stable support-pack item id.
    pub support_pack_item_id: String,
    /// Stable support export ref.
    pub support_export_ref: String,
    /// Bundle member path ref.
    pub bundle_member_path_ref: String,
    /// Export field refs included by this row.
    pub export_field_refs: Vec<String>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl GitReviewSupportExportRow {
    /// Builds a support row from a Git/review event.
    pub fn from_event(event: &GitReviewEventRecord) -> Self {
        Self {
            event_id: event.event_id.clone(),
            activity_row_id: event.activity_row_id.clone(),
            event_family: event.event_family,
            phase: event.phase,
            state_class: event.state_class,
            workspace_ref: event.workspace_ref.clone(),
            branch: event.branch.clone(),
            target: event.target.clone(),
            action: event.action.clone(),
            exact_reopen_links: event.exact_reopen_links.clone(),
            support_pack_item_id: event.support_projection.support_pack_item_id.clone(),
            support_export_ref: event.support_projection.support_export_ref.clone(),
            bundle_member_path_ref: event.support_projection.bundle_member_path_ref.clone(),
            export_field_refs: event.support_projection.export_field_refs.clone(),
            raw_private_material_excluded: event.support_projection.raw_private_material_excluded,
        }
    }

    /// Returns true when this row preserves branch, target, and action identity.
    pub fn has_branch_target_action_identity(&self) -> bool {
        self.branch.has_identity() && self.target.has_identity() && self.action.has_identity()
    }
}

/// Structured support export for Git/review events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitReviewSupportExport {
    /// Optional schema ref used by checked-in fixtures.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Export creation timestamp.
    pub generated_at: String,
    /// Rows included in the export.
    pub rows: Vec<GitReviewSupportExportRow>,
    /// Source event count.
    pub source_event_count: usize,
    /// Count of rows carrying branch, target, and action identity.
    pub branch_target_action_complete_count: usize,
    /// Count of exact reopen links.
    pub exact_reopen_link_count: usize,
    /// True when no raw private material is included.
    pub raw_private_material_excluded: bool,
}

impl GitReviewSupportExport {
    /// Builds a structured support export from events.
    pub fn from_events(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        events: &[GitReviewEventRecord],
    ) -> Self {
        let rows = events
            .iter()
            .filter(|event| event.support_projection.raw_private_material_excluded)
            .map(GitReviewSupportExportRow::from_event)
            .collect::<Vec<_>>();
        let branch_target_action_complete_count = rows
            .iter()
            .filter(|row| row.has_branch_target_action_identity())
            .count();
        let exact_reopen_link_count = rows.iter().map(|row| row.exact_reopen_links.len()).sum();
        Self {
            schema_ref: None,
            record_kind: GIT_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: GIT_REVIEW_SUPPORT_EXPORT_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            source_event_count: events.len(),
            rows,
            branch_target_action_complete_count,
            exact_reopen_link_count,
            raw_private_material_excluded: true,
        }
    }

    /// Number of rows included in the export.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns true when all rows preserve branch, target, and action identity.
    pub fn all_rows_preserve_branch_target_action_identity(&self) -> bool {
        !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(GitReviewSupportExportRow::has_branch_target_action_identity)
    }
}

fn state_from_activity_token(token: &str) -> ActivityRowStateClass {
    match token {
        "running" => ActivityRowStateClass::Running,
        "queued_waiting" | "queued" => ActivityRowStateClass::QueuedWaiting,
        "needs_approval" | "blocked" | "blocked_no_changes_made" => {
            ActivityRowStateClass::NeedsApproval
        }
        "partially_completed" => ActivityRowStateClass::PartiallyCompleted,
        "failed" => ActivityRowStateClass::Failed,
        "cancelled" => ActivityRowStateClass::Cancelled,
        "superseded" => ActivityRowStateClass::Superseded,
        "quiet_hours_held" => ActivityRowStateClass::QuietHoursHeld,
        "policy_suppressed" => ActivityRowStateClass::PolicySuppressed,
        _ => ActivityRowStateClass::Completed,
    }
}

fn severity_from_state_token(token: &str) -> SeverityClass {
    match token {
        "failed" => SeverityClass::Error,
        "blocked" | "blocked_no_changes_made" | "needs_approval" => SeverityClass::Warning,
        _ => SeverityClass::Info,
    }
}

fn phase_from_support_token(token: &str) -> GitReviewEventPhase {
    match token {
        "apply" | "applied" | "restore" | "reverted" => GitReviewEventPhase::Applied,
        "blocked" | "blocked_no_changes_made" => GitReviewEventPhase::Blocked,
        "failed" => GitReviewEventPhase::Failed,
        "published" => GitReviewEventPhase::Published,
        "refreshed" => GitReviewEventPhase::Refreshed,
        _ => GitReviewEventPhase::Preview,
    }
}

fn progress_form_for_state(state: ActivityRowStateClass) -> ActivityProgressForm {
    match state {
        ActivityRowStateClass::Completed => ActivityProgressForm::CompletionSummary,
        ActivityRowStateClass::Failed | ActivityRowStateClass::PartiallyCompleted => {
            ActivityProgressForm::FailureOrPartialSummary
        }
        ActivityRowStateClass::QueuedWaiting => ActivityProgressForm::QueueReason,
        ActivityRowStateClass::QuietHoursHeld | ActivityRowStateClass::PolicySuppressed => {
            ActivityProgressForm::HeldOrSuppressedReason
        }
        _ => ActivityProgressForm::PhaseOnly,
    }
}

fn expected_boundary_for(family: GitReviewEventFamily) -> &'static str {
    match family {
        GitReviewEventFamily::GitPublish => "local_git_and_remote_provider",
        GitReviewEventFamily::ReviewWorkspace => "local_review_workspace",
        _ => "local_git_worktree",
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
        out.push('_');
        last_sep = true;
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() {
        "event".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn branch() -> GitReviewBranchContext {
        GitReviewBranchContext::new(
            Some("feature/activity".to_string()),
            Some("refs/heads/feature/activity".to_string()),
            Some("refs/remotes/origin/feature/activity".to_string()),
            Some("git.rev.abc1234".to_string()),
        )
    }

    fn publish_event() -> GitReviewEventRecord {
        let input = GitReviewEventInput {
            event_id: "event.git_review.publish.fixture".to_string(),
            occurred_at: "2026-05-13T22:30:00Z".to_string(),
            event_family: GitReviewEventFamily::GitPublish,
            phase: GitReviewEventPhase::Failed,
            state_class: ActivityRowStateClass::Failed,
            severity_class: SeverityClass::Error,
            actor_identity_ref: "id:actor:local-user".to_string(),
            actor_label: "Local user".to_string(),
            workspace_ref: "workspace.repo.aureline".to_string(),
            summary_label: "Publish failed".to_string(),
            detail_label: "Remote rejected the push; review can reopen.".to_string(),
            branch: branch(),
            target: GitReviewTargetIdentity {
                canonical_target_ref: "git.publish.target.origin-feature".to_string(),
                target_kind: GitReviewTargetKind::RemoteRef,
                target_label: "origin/feature/activity".to_string(),
                scope_ref: None,
                target_refs: vec!["refs/heads/feature/activity".to_string()],
                review_workspace_ref: None,
                route_ref: Some("git.publish.route.origin".to_string()),
            },
            action: GitReviewActionIdentity {
                action_id: "action.git.publish.review".to_string(),
                action_class: GitReviewActionClass::PublishReview,
                command_id: "cmd:git.publish.review.reopen".to_string(),
                source_record_ref: "git.publish.preview.fixture".to_string(),
                preview_ref: Some("git.publish.preview.fixture".to_string()),
                result_ref: Some("git.publish.result.failed.fixture".to_string()),
                journal_ref: Some("git.publish.journal.fixture".to_string()),
                recovery_ref: Some("git.publish.recovery.fixture".to_string()),
                side_effect_class: "push_to_upstream".to_string(),
                reissues_original_side_effect: false,
            },
            exact_reopen_links: vec![GitReviewExactReopenLink::new(
                GitReviewReopenKind::GitPublishReview,
                "cmd:git.publish.review.reopen",
                "git.publish.preview.fixture",
                "Reopen publish review",
            )],
            support_projection: GitReviewSupportProjection::metadata_safe(
                "support.item.git_review.publish",
                "support.export.git_review.publish.fixture",
                "manifest/git_review_activity/publish_fixture.json",
            ),
        };
        GitReviewEventRecord::from_input(input)
    }

    #[test]
    fn activity_row_embeds_branch_target_and_action_context() {
        let event = publish_event();
        let row = event.to_activity_row();

        assert_eq!(row.job_family, ActivityJobFamily::GitReview);
        let context = row
            .git_review_context
            .as_ref()
            .expect("Git/review context embedded");
        assert!(context.has_branch_target_action_identity());
        assert!(context.exact_reopen_links_resolve());
        assert_eq!(
            row.actions[0].command_id.as_deref(),
            Some("cmd:git.publish.review.reopen")
        );
        assert_eq!(
            row.actions[0].target_identity_ref,
            "git.publish.preview.fixture"
        );
    }

    #[test]
    fn support_export_keeps_structured_event_family() {
        let event = publish_event();
        let export = GitReviewSupportExport::from_events(
            "support.export.git_review.fixture",
            "2026-05-13T22:31:00Z",
            &[event],
        );

        assert_eq!(export.row_count(), 1);
        assert!(export.all_rows_preserve_branch_target_action_identity());
        assert_eq!(
            export.rows[0].event_family,
            GitReviewEventFamily::GitPublish
        );
        assert_eq!(
            export.rows[0].exact_reopen_links[0].target_identity_ref,
            "git.publish.preview.fixture"
        );
    }
}
