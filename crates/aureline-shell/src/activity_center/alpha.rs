//! Alpha activity-center rows for durable, reopenable work.
//!
//! This module is the shell-owned runtime contract for long-running and
//! reviewable work that must survive look-away, focus loss, sleep/resume, and
//! ordinary shell restarts. It does not replace notification routing; it
//! consumes the same source subsystem, privacy, severity, redaction, and
//! reopen-target vocabulary used by notifications and projects it into
//! first-class activity rows.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::{
    PrivacyClass, RedactionClass, ReopenTarget, ReopenTargetKind, SeverityClass, SourceSubsystem,
};

use super::git_review::GitReviewActivityContext;

/// Stable record kind for one activity-center row.
pub const ACTIVITY_ROW_RECORD_KIND: &str = "activity_row_record";

/// Schema version for [`ActivityRow`] records.
pub const ACTIVITY_ROW_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for an alpha activity-center snapshot.
pub const ACTIVITY_CENTER_ALPHA_SNAPSHOT_RECORD_KIND: &str =
    "activity_center_alpha_snapshot_record";

/// Schema version for [`ActivityCenterAlphaSnapshot`] records.
pub const ACTIVITY_CENTER_ALPHA_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the support/export-safe activity projection.
pub const ACTIVITY_CENTER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "activity_center_support_export_record";

/// Schema version for [`ActivityCenterSupportExport`] records.
pub const ACTIVITY_CENTER_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing scope notice carried by alpha snapshots.
pub const ACTIVITY_CENTER_ALPHA_SCOPE_NOTICE: &str =
    "Activity center alpha: indexing, restore, install/update, task, and test \
     work share one durable row model with stable row/job ids, typed actions, \
     impact flags, exact reopen identity, persistence, and support-export \
     projection. Rows are retained until resolved or archived.";

/// Work family represented by an activity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityJobFamily {
    /// Background indexing or search-readiness work.
    Indexing,
    /// Session restore, recovery, or restore-provenance work.
    Restore,
    /// Install, update, download, package, or bundle work.
    InstallUpdate,
    /// Task execution work.
    TaskRun,
    /// Test execution work.
    TestRun,
    /// Git and review work that must preserve branch, target, and action identity.
    GitReview,
}

impl ActivityJobFamily {
    /// Stable token recorded on rows and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indexing => "indexing",
            Self::Restore => "restore",
            Self::InstallUpdate => "install_update",
            Self::TaskRun => "task_run",
            Self::TestRun => "test_run",
            Self::GitReview => "git_review",
        }
    }

    /// Human-readable label rendered by shell chrome.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Indexing => "Indexing",
            Self::Restore => "Restore",
            Self::InstallUpdate => "Install/update",
            Self::TaskRun => "Task",
            Self::TestRun => "Test",
            Self::GitReview => "Git/review",
        }
    }
}

/// Activity-center partition used for grouping and badge counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityPartition {
    /// Running or queued work.
    CurrentWork,
    /// Failed, partial, blocked, or approval-needed work.
    NeedsAttention,
    /// Terminal successful work that remains reviewable.
    Completed,
    /// Held or suppressed work that still preserves history.
    SuppressedHeld,
}

impl ActivityPartition {
    /// Stable token recorded in row snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentWork => "current_work",
            Self::NeedsAttention => "needs_attention",
            Self::Completed => "completed",
            Self::SuppressedHeld => "suppressed_held",
        }
    }
}

/// Lifecycle state for an alpha activity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityRowStateClass {
    /// Work is accepted but waiting for a queue, boundary, or resource.
    QueuedWaiting,
    /// Pre-flight or setup has started.
    Preparing,
    /// Work is actively progressing.
    Running,
    /// Work is waiting on approval or review.
    NeedsApproval,
    /// Work finished successfully.
    Completed,
    /// Work failed and needs follow-up.
    Failed,
    /// Some parts succeeded while others failed, were skipped, or were excluded.
    PartiallyCompleted,
    /// Work was cancelled by a user, subsystem, or policy actor.
    Cancelled,
    /// Work was superseded by a newer authoritative row.
    Superseded,
    /// Interruption was held by quiet hours, focus mode, presentation mode, or
    /// comparable reduced-attention posture.
    QuietHoursHeld,
    /// Policy blocked full display or fanout while preserving durable history.
    PolicySuppressed,
}

impl ActivityRowStateClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueuedWaiting => "queued_waiting",
            Self::Preparing => "preparing",
            Self::Running => "running",
            Self::NeedsApproval => "needs_approval",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::PartiallyCompleted => "partially_completed",
            Self::Cancelled => "cancelled",
            Self::Superseded => "superseded",
            Self::QuietHoursHeld => "quiet_hours_held",
            Self::PolicySuppressed => "policy_suppressed",
        }
    }

    /// Human-readable label rendered by shell chrome.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QueuedWaiting => "Queued",
            Self::Preparing => "Preparing",
            Self::Running => "Running",
            Self::NeedsApproval => "Needs approval",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::PartiallyCompleted => "Partially completed",
            Self::Cancelled => "Cancelled",
            Self::Superseded => "Superseded",
            Self::QuietHoursHeld => "Held",
            Self::PolicySuppressed => "Policy suppressed",
        }
    }

    /// Returns the partition implied by the state.
    pub const fn partition(self) -> ActivityPartition {
        match self {
            Self::QueuedWaiting | Self::Preparing | Self::Running => ActivityPartition::CurrentWork,
            Self::NeedsApproval | Self::Failed | Self::PartiallyCompleted => {
                ActivityPartition::NeedsAttention
            }
            Self::Completed | Self::Cancelled | Self::Superseded => ActivityPartition::Completed,
            Self::QuietHoursHeld | Self::PolicySuppressed => ActivityPartition::SuppressedHeld,
        }
    }

    /// True when the row is terminal but remains reviewable.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::PartiallyCompleted
                | Self::Cancelled
                | Self::Superseded
        )
    }
}

/// Progress form present on a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityProgressForm {
    /// Determinate progress bar with unit and percentage.
    LabeledProgressBar,
    /// Indeterminate spinner with reason.
    SpinnerOnly,
    /// Phase label only.
    PhaseOnly,
    /// Queue reason and expected boundary.
    QueueReason,
    /// Terminal completion summary.
    CompletionSummary,
    /// Failure or partial-result summary.
    FailureOrPartialSummary,
    /// Held or suppressed reason.
    HeldOrSuppressedReason,
}

impl ActivityProgressForm {
    /// Stable token recorded in snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabeledProgressBar => "labeled_progress_bar",
            Self::SpinnerOnly => "spinner_only",
            Self::PhaseOnly => "phase_only",
            Self::QueueReason => "queue_reason",
            Self::CompletionSummary => "completion_summary",
            Self::FailureOrPartialSummary => "failure_or_partial_summary",
            Self::HeldOrSuppressedReason => "held_or_suppressed_reason",
        }
    }
}

/// Cancellability posture for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityCancellabilityClass {
    /// Row can be cancelled safely through a command-backed action.
    Cancellable,
    /// Row is not cancellable in the current phase.
    NotCancellable,
    /// Row is already terminal.
    AlreadyTerminal,
}

impl ActivityCancellabilityClass {
    /// Stable token recorded in snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cancellable => "cancellable",
            Self::NotCancellable => "not_cancellable",
            Self::AlreadyTerminal => "already_terminal",
        }
    }
}

/// Determinate progress bar used by durable rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityProgressBar {
    /// Label describing the unit being counted.
    pub label: String,
    /// Completed unit count.
    pub numerator: u32,
    /// Total unit count.
    pub denominator: u32,
    /// Unit label, such as `files`, `packages`, or `tests`.
    pub unit_label: String,
    /// Pinned percentage label.
    pub percent_label: String,
}

impl ActivityProgressBar {
    /// Builds a progress bar with a deterministic percent label.
    pub fn new(
        label: impl Into<String>,
        numerator: u32,
        denominator: u32,
        unit_label: impl Into<String>,
    ) -> Self {
        let percent = if denominator == 0 {
            0
        } else {
            (u64::from(numerator) * 100 / u64::from(denominator)) as u32
        };
        Self {
            label: label.into(),
            numerator,
            denominator,
            unit_label: unit_label.into(),
            percent_label: format!("{percent}%"),
        }
    }
}

/// Progress, phase, and detail projection for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowProgress {
    /// Progress forms present on this row.
    pub forms: Vec<ActivityProgressForm>,
    /// Current phase label.
    pub phase_label: String,
    /// Optional determinate progress bar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_bar: Option<ActivityProgressBar>,
    /// Queue reason when the row is queued or waiting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_reason_label: Option<String>,
    /// Approval source when the row is blocked on review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_source_label: Option<String>,
    /// Actor or subsystem label rendered with progress.
    pub actor_or_subsystem_label: String,
    /// Elapsed, queued, or finished age label.
    pub age_label: String,
    /// Reason for spinner-only progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indeterminate_reason_label: Option<String>,
    /// Boundary where the work is expected to run.
    pub expected_boundary_class: String,
    /// Cancellability posture.
    pub cancellability_class: ActivityCancellabilityClass,
    /// Evidence, history, review, or support ref for the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_or_evidence_ref: Option<String>,
}

/// Impact flags that make a row consequence-bearing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowImpact {
    /// True when the job can affect cost or quota.
    pub affects_cost: bool,
    /// True when the job can affect policy.
    pub affects_policy: bool,
    /// True when the job can use network or route state.
    pub affects_network: bool,
    /// True when the job can affect trust.
    pub affects_trust: bool,
    /// True when the job can affect provider-owned state.
    pub affects_provider_state: bool,
    /// True when the job can affect restore or recovery posture.
    pub affects_recovery_posture: bool,
    /// True when the row must expose evidence or support detail.
    pub detail_or_evidence_required: bool,
    /// Reviewable impact sentence.
    pub impact_summary_sentence: String,
}

impl ActivityRowImpact {
    /// Builds a routine local-impact record.
    pub fn routine(summary: impl Into<String>) -> Self {
        Self {
            affects_cost: false,
            affects_policy: false,
            affects_network: false,
            affects_trust: false,
            affects_provider_state: false,
            affects_recovery_posture: false,
            detail_or_evidence_required: false,
            impact_summary_sentence: summary.into(),
        }
    }

    /// True when any consequence-bearing flag is set.
    pub const fn has_sensitive_axis(&self) -> bool {
        self.affects_cost
            || self.affects_policy
            || self.affects_network
            || self.affects_trust
            || self.affects_provider_state
            || self.affects_recovery_posture
    }
}

/// Command-backed action exposed by an activity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowAction {
    /// Stable action id.
    pub action_id: String,
    /// Action kind.
    pub action_kind: ActivityRowActionKind,
    /// Human-readable label.
    pub label: String,
    /// Stable command id when the action is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Availability posture.
    pub availability_class: ActivityRowActionAvailability,
    /// Disabled reason when unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_label: Option<String>,
    /// Target identity the command opens or mutates.
    pub target_identity_ref: String,
    /// True when the action preserves the row rather than deleting history.
    pub preserves_durable_history: bool,
    /// True only for actions that intentionally replay or start new work.
    pub reissues_original_side_effect: bool,
}

impl ActivityRowAction {
    /// Builds an enabled open-details action.
    pub fn open_details(
        action_id: impl Into<String>,
        label: impl Into<String>,
        target_identity_ref: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            action_kind: ActivityRowActionKind::OpenDetails,
            label: label.into(),
            command_id: Some("cmd:activity.open_job_details".into()),
            availability_class: ActivityRowActionAvailability::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target_identity_ref.into(),
            preserves_durable_history: true,
            reissues_original_side_effect: false,
        }
    }
}

/// Activity-row action kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityRowActionKind {
    /// Opens the durable row details.
    OpenDetails,
    /// Cancels running or queued work.
    CancelJob,
    /// Retries terminal failed work as a reviewed new invocation.
    RetryJob,
    /// Opens evidence or support detail.
    OpenEvidence,
    /// Opens history.
    OpenHistory,
    /// Archives a resolved row.
    Archive,
    /// Acknowledges attention without resolving the underlying issue.
    Acknowledge,
}

impl ActivityRowActionKind {
    /// Stable token recorded in snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetails => "open_details",
            Self::CancelJob => "cancel_job",
            Self::RetryJob => "retry_job",
            Self::OpenEvidence => "open_evidence",
            Self::OpenHistory => "open_history",
            Self::Archive => "archive",
            Self::Acknowledge => "acknowledge",
        }
    }
}

/// Availability posture for row actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityRowActionAvailability {
    /// Action is enabled.
    Enabled,
    /// Action is visible but disabled.
    Disabled,
    /// Action is not meaningful for this state.
    NotApplicable,
    /// Action requires revalidation before it can run.
    RequiresRevalidation,
}

impl ActivityRowActionAvailability {
    /// Stable token recorded in snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::NotApplicable => "not_applicable",
            Self::RequiresRevalidation => "requires_revalidation",
        }
    }
}

/// Expand/collapse behavior for a durable row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowDisplayState {
    /// Current collapse state.
    pub collapse_state: ActivityRowCollapseState,
    /// True when the row can expand inline.
    pub can_expand_inline: bool,
    /// Stable label naming what expand reveals.
    pub expand_reveals_label: String,
}

/// Collapse state for an activity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityRowCollapseState {
    /// Row is collapsed but retains visible summary and actions.
    CollapsedSummary,
    /// Row is expanded inline.
    ExpandedInline,
    /// Row opens a detail sheet instead of expanding inline.
    DetailSheetOnly,
}

impl ActivityRowCollapseState {
    /// Stable token recorded in snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CollapsedSummary => "collapsed_summary",
            Self::ExpandedInline => "expanded_inline",
            Self::DetailSheetOnly => "detail_sheet_only",
        }
    }
}

/// Chronology for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowTimeline {
    /// First time this row appeared.
    pub minted_at: String,
    /// Queue time when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<String>,
    /// Start time when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Last lifecycle observation.
    pub last_observed_at: String,
    /// Finish time when terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// Archive time after explicit archive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    /// New row that superseded this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by_row_id_ref: Option<String>,
    /// Retention summary.
    pub retention_label: String,
}

/// Structured support/export projection flags for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowSupportLink {
    /// True when the row can be exported to support or review artifacts.
    pub exportable: bool,
    /// Support-pack item id when exportable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_pack_item_id: Option<String>,
    /// Optional support bundle member ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_member_path_ref: Option<String>,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Structured export field refs.
    pub export_field_refs: Vec<String>,
}

/// One durable activity-center row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRow {
    /// Optional schema ref used by checked-in fixtures.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub activity_row_id: String,
    /// Stable durable-job id.
    pub durable_job_id: String,
    /// Shared canonical event id.
    pub canonical_event_id: String,
    /// Authoritative object target.
    pub canonical_object_target_ref: String,
    /// Exact identity reopened by row actions.
    pub exact_reopen_identity_ref: String,
    /// Job family.
    pub job_family: ActivityJobFamily,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Actor identity.
    pub actor_identity_ref: String,
    /// Actor or subsystem label.
    pub actor_or_subsystem_label: String,
    /// Execution origin class.
    pub execution_origin_class: String,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Privacy class.
    pub privacy_class: PrivacyClass,
    /// Summary label.
    pub summary_label: String,
    /// Target label.
    pub target_label: String,
    /// Target scope label.
    pub target_scope_label: String,
    /// Current state class.
    pub state_class: ActivityRowStateClass,
    /// State token.
    pub state_token: String,
    /// State label.
    pub state_label: String,
    /// Activity partition.
    pub activity_partition: ActivityPartition,
    /// True when the row is terminal.
    pub is_terminal: bool,
    /// Progress and phase projection.
    pub progress: ActivityRowProgress,
    /// Timeline projection.
    pub timeline: ActivityRowTimeline,
    /// Row impact flags.
    pub impact: ActivityRowImpact,
    /// Row actions.
    pub actions: Vec<ActivityRowAction>,
    /// Display and collapse behavior.
    pub display: ActivityRowDisplayState,
    /// Exact reopen target.
    pub reopen_target: ReopenTarget,
    /// Support/export projection.
    pub support_link: ActivityRowSupportLink,
    /// Structured Git/review event context when this row represents source-control work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_review_context: Option<GitReviewActivityContext>,
    /// Repeated observations for this canonical event.
    pub occurrence_count: u32,
    /// True when history is retained until resolved or archived.
    pub retained_until_resolved_or_archived: bool,
}

impl ActivityRow {
    /// Builds a row from a typed input record.
    pub fn from_input(input: ActivityRowInput) -> Self {
        let state_token = input.state_class.as_str().to_owned();
        let state_label = input.state_class.label().to_owned();
        let activity_partition = input.state_class.partition();
        let is_terminal = input.state_class.is_terminal();
        Self {
            schema_ref: None,
            record_kind: ACTIVITY_ROW_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_ROW_SCHEMA_VERSION,
            activity_row_id: input.activity_row_id.clone(),
            durable_job_id: input.durable_job_id,
            canonical_event_id: input.canonical_event_id,
            canonical_object_target_ref: input.canonical_object_target_ref,
            exact_reopen_identity_ref: input.exact_reopen_identity_ref.clone(),
            job_family: input.job_family,
            source_subsystem: input.source_subsystem,
            actor_identity_ref: input.actor_identity_ref,
            actor_or_subsystem_label: input.actor_or_subsystem_label,
            execution_origin_class: input.execution_origin_class,
            severity_class: input.severity_class,
            privacy_class: input.privacy_class,
            summary_label: input.summary_label,
            target_label: input.target_label,
            target_scope_label: input.target_scope_label,
            state_class: input.state_class,
            state_token,
            state_label,
            activity_partition,
            is_terminal,
            progress: input.progress,
            timeline: input.timeline,
            impact: input.impact,
            actions: input.actions,
            display: input.display,
            reopen_target: ReopenTarget {
                reopen_target_ref: format!("ux:reopen:activity-row:{}", input.activity_row_id),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                exact_target_identity_ref: Some(input.exact_reopen_identity_ref),
                placeholder_announcement_label: None,
                revalidation_required_reason_label: None,
            },
            support_link: input.support_link,
            git_review_context: input.git_review_context,
            occurrence_count: input.occurrence_count.max(1),
            retained_until_resolved_or_archived: true,
        }
    }

    /// True when the row opens a durable object rather than replaying the
    /// original side effect.
    pub fn has_exact_reopen_identity(&self) -> bool {
        self.reopen_target.exact_target_identity_ref.as_deref()
            == Some(self.exact_reopen_identity_ref.as_str())
            && self
                .actions
                .iter()
                .any(|action| action.action_kind == ActivityRowActionKind::OpenDetails)
    }

    /// True when the row has all required sensitive-impact evidence.
    pub fn satisfies_sensitive_detail_rule(&self) -> bool {
        !self.impact.has_sensitive_axis()
            || (self.impact.detail_or_evidence_required
                && self.progress.detail_or_evidence_ref.is_some())
    }

    /// True when the row is exportable to structured support artifacts.
    pub fn is_support_exportable(&self) -> bool {
        self.support_link.exportable
            && self.support_link.support_pack_item_id.is_some()
            && self.support_link.raw_private_material_excluded
    }
}

/// Input record used to build an [`ActivityRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityRowInput {
    /// Stable row id.
    pub activity_row_id: String,
    /// Stable durable-job id.
    pub durable_job_id: String,
    /// Shared canonical event id.
    pub canonical_event_id: String,
    /// Authoritative object target.
    pub canonical_object_target_ref: String,
    /// Exact identity reopened by row actions.
    pub exact_reopen_identity_ref: String,
    /// Job family.
    pub job_family: ActivityJobFamily,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Actor identity.
    pub actor_identity_ref: String,
    /// Actor or subsystem label.
    pub actor_or_subsystem_label: String,
    /// Execution origin class.
    pub execution_origin_class: String,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Privacy class.
    pub privacy_class: PrivacyClass,
    /// Summary label.
    pub summary_label: String,
    /// Target label.
    pub target_label: String,
    /// Target scope label.
    pub target_scope_label: String,
    /// Current state class.
    pub state_class: ActivityRowStateClass,
    /// Progress and phase projection.
    pub progress: ActivityRowProgress,
    /// Timeline projection.
    pub timeline: ActivityRowTimeline,
    /// Row impact flags.
    pub impact: ActivityRowImpact,
    /// Row actions.
    pub actions: Vec<ActivityRowAction>,
    /// Display and collapse behavior.
    pub display: ActivityRowDisplayState,
    /// Support/export projection.
    pub support_link: ActivityRowSupportLink,
    /// Structured Git/review event context when this row represents source-control work.
    pub git_review_context: Option<GitReviewActivityContext>,
    /// Repeated observations for this canonical event.
    pub occurrence_count: u32,
}

/// Count for one partition in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityPartitionCount {
    /// Partition.
    pub activity_partition: ActivityPartition,
    /// Number of rows in the partition.
    pub row_count: usize,
}

/// Read-only projection consumed by activity-center chrome and tests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterAlphaSnapshot {
    /// Optional schema ref used by checked-in fixtures.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Scope notice.
    pub scope_notice: String,
    /// Durable rows.
    pub rows: Vec<ActivityRow>,
    /// Partition counts.
    pub partition_counts: Vec<ActivityPartitionCount>,
    /// Job families present in this snapshot.
    pub job_families_present: Vec<ActivityJobFamily>,
    /// Count of rows with exact reopen identity.
    pub exact_reopen_row_count: usize,
    /// Count of support/exportable rows.
    pub support_exportable_row_count: usize,
    /// True when at least one row honestly shows failure, partial success,
    /// cancellation, suppression, or supersession.
    pub honesty_marker_present: bool,
}

impl ActivityCenterAlphaSnapshot {
    /// Builds a snapshot from rows.
    pub fn from_rows(mut rows: Vec<ActivityRow>) -> Self {
        rows.sort_by(|a, b| {
            a.activity_partition
                .cmp(&b.activity_partition)
                .then_with(|| a.timeline.minted_at.cmp(&b.timeline.minted_at))
                .then_with(|| a.activity_row_id.cmp(&b.activity_row_id))
        });
        let mut partition_counts = Vec::new();
        for partition in [
            ActivityPartition::CurrentWork,
            ActivityPartition::NeedsAttention,
            ActivityPartition::Completed,
            ActivityPartition::SuppressedHeld,
        ] {
            let row_count = rows
                .iter()
                .filter(|row| row.activity_partition == partition)
                .count();
            if row_count > 0 {
                partition_counts.push(ActivityPartitionCount {
                    activity_partition: partition,
                    row_count,
                });
            }
        }
        let mut families = rows.iter().map(|row| row.job_family).collect::<Vec<_>>();
        families.sort();
        families.dedup();
        let exact_reopen_row_count = rows
            .iter()
            .filter(|row| row.has_exact_reopen_identity())
            .count();
        let support_exportable_row_count = rows
            .iter()
            .filter(|row| row.is_support_exportable())
            .count();
        let honesty_marker_present = rows.iter().any(|row| {
            matches!(
                row.state_class,
                ActivityRowStateClass::Failed
                    | ActivityRowStateClass::PartiallyCompleted
                    | ActivityRowStateClass::Cancelled
                    | ActivityRowStateClass::Superseded
                    | ActivityRowStateClass::QuietHoursHeld
                    | ActivityRowStateClass::PolicySuppressed
            )
        });
        Self {
            schema_ref: None,
            record_kind: ACTIVITY_CENTER_ALPHA_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_CENTER_ALPHA_SNAPSHOT_SCHEMA_VERSION,
            scope_notice: ACTIVITY_CENTER_ALPHA_SCOPE_NOTICE.to_owned(),
            rows,
            partition_counts,
            job_families_present: families,
            exact_reopen_row_count,
            support_exportable_row_count,
            honesty_marker_present,
        }
    }

    /// Number of rows in the snapshot.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True when the snapshot has no rows.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Finds a row by row id.
    pub fn find_row(&self, activity_row_id: &str) -> Option<&ActivityRow> {
        self.rows
            .iter()
            .find(|row| row.activity_row_id == activity_row_id)
    }

    /// True when every required alpha family appears in the snapshot.
    pub fn covers_required_alpha_families(&self) -> bool {
        [
            ActivityJobFamily::Indexing,
            ActivityJobFamily::Restore,
            ActivityJobFamily::InstallUpdate,
            ActivityJobFamily::TaskRun,
            ActivityJobFamily::TestRun,
        ]
        .iter()
        .all(|family| self.job_families_present.contains(family))
    }
}

/// Export-safe support row for one activity item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivitySupportExportRow {
    /// Stable row id.
    pub activity_row_id: String,
    /// Stable durable-job id.
    pub durable_job_id: String,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Job family.
    pub job_family: ActivityJobFamily,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// State class.
    pub state_class: ActivityRowStateClass,
    /// Exact reopen identity.
    pub exact_reopen_identity_ref: String,
    /// Evidence ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_or_evidence_ref: Option<String>,
    /// Support-pack item id.
    pub support_pack_item_id: String,
    /// Export field refs.
    pub export_field_refs: Vec<String>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Structured support/export projection for activity rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterSupportExport {
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
    pub rows: Vec<ActivitySupportExportRow>,
    /// Source snapshot row count.
    pub source_snapshot_row_count: usize,
    /// True when no raw private material is included.
    pub raw_private_material_excluded: bool,
}

impl ActivityCenterSupportExport {
    /// Builds a support-export projection from a snapshot.
    pub fn from_snapshot(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        snapshot: &ActivityCenterAlphaSnapshot,
    ) -> Self {
        let rows = snapshot
            .rows
            .iter()
            .filter(|row| row.is_support_exportable())
            .map(|row| ActivitySupportExportRow {
                activity_row_id: row.activity_row_id.clone(),
                durable_job_id: row.durable_job_id.clone(),
                canonical_event_id: row.canonical_event_id.clone(),
                job_family: row.job_family,
                source_subsystem: row.source_subsystem,
                state_class: row.state_class,
                exact_reopen_identity_ref: row.exact_reopen_identity_ref.clone(),
                detail_or_evidence_ref: row.progress.detail_or_evidence_ref.clone(),
                support_pack_item_id: row
                    .support_link
                    .support_pack_item_id
                    .clone()
                    .expect("support-exportable rows carry a support item id"),
                export_field_refs: row.support_link.export_field_refs.clone(),
                raw_private_material_excluded: row.support_link.raw_private_material_excluded,
            })
            .collect::<Vec<_>>();
        Self {
            schema_ref: None,
            record_kind: ACTIVITY_CENTER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_CENTER_SUPPORT_EXPORT_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            source_snapshot_row_count: snapshot.rows.len(),
            raw_private_material_excluded: true,
        }
    }

    /// Number of rows included in the support export.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Errors raised by alpha activity-center persistence.
#[derive(Debug)]
pub enum ActivityCenterAlphaError {
    /// Filesystem error.
    Io(std::io::Error),
    /// JSON serialization error.
    Serde(serde_json::Error),
}

impl std::fmt::Display for ActivityCenterAlphaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "activity-center alpha io error: {err}"),
            Self::Serde(err) => write!(f, "activity-center alpha serialization error: {err}"),
        }
    }
}

impl std::error::Error for ActivityCenterAlphaError {}

impl From<std::io::Error> for ActivityCenterAlphaError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for ActivityCenterAlphaError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

/// Durable alpha row store, optionally file-backed.
#[derive(Debug, Clone)]
pub struct ActivityCenterAlphaStore {
    rows_path: Option<PathBuf>,
    rows: BTreeMap<String, ActivityRow>,
}

impl ActivityCenterAlphaStore {
    /// Builds an in-memory store.
    pub fn in_memory() -> Self {
        Self {
            rows_path: None,
            rows: BTreeMap::new(),
        }
    }

    /// Opens a file-backed store and reloads prior rows.
    ///
    /// # Errors
    ///
    /// Returns an error when the row file cannot be read or deserialized.
    pub fn file_backed(path: impl Into<PathBuf>) -> Result<Self, ActivityCenterAlphaError> {
        let path = path.into();
        let rows = if path.exists() {
            let bytes = fs::read(&path)?;
            if bytes.is_empty() {
                BTreeMap::new()
            } else {
                let stored: Vec<ActivityRow> = serde_json::from_slice(&bytes)?;
                stored
                    .into_iter()
                    .map(|row| (row.activity_row_id.clone(), row))
                    .collect()
            }
        } else {
            BTreeMap::new()
        };
        Ok(Self {
            rows_path: Some(path),
            rows,
        })
    }

    /// Records a row, updating an existing row with the same stable id.
    ///
    /// # Errors
    ///
    /// Returns an error when the file-backed store cannot persist the
    /// updated row set.
    pub fn record_row(&mut self, mut row: ActivityRow) -> Result<(), ActivityCenterAlphaError> {
        if let Some(existing) = self.rows.get(&row.activity_row_id) {
            row.timeline.minted_at = existing.timeline.minted_at.clone();
            row.occurrence_count = existing
                .occurrence_count
                .saturating_add(row.occurrence_count.max(1));
        }
        self.rows.insert(row.activity_row_id.clone(), row);
        if let Some(path) = self.rows_path.clone() {
            self.persist(&path)?;
        }
        Ok(())
    }

    /// Finds a row by stable row id.
    pub fn find_row(&self, activity_row_id: &str) -> Option<&ActivityRow> {
        self.rows.get(activity_row_id)
    }

    /// Number of rows in the store.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True when the store is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns a read-only snapshot.
    pub fn snapshot(&self) -> ActivityCenterAlphaSnapshot {
        ActivityCenterAlphaSnapshot::from_rows(self.rows.values().cloned().collect())
    }

    /// Explicitly flushes the file-backed store.
    ///
    /// # Errors
    ///
    /// Returns an error when the backing file cannot be written.
    pub fn persist_now(&self) -> Result<(), ActivityCenterAlphaError> {
        if let Some(path) = self.rows_path.as_deref() {
            self.persist(path)?;
        }
        Ok(())
    }

    fn persist(&self, path: &Path) -> Result<(), ActivityCenterAlphaError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let snapshot = self.snapshot();
        let bytes = serde_json::to_vec_pretty(&snapshot.rows)?;
        fs::write(path, bytes)?;
        Ok(())
    }
}

/// Runtime wrapper for alpha activity-center persistence and export.
#[derive(Debug, Clone)]
pub struct ActivityCenterAlphaRuntime {
    store: ActivityCenterAlphaStore,
}

impl ActivityCenterAlphaRuntime {
    /// Builds an in-memory runtime.
    pub fn in_memory() -> Self {
        Self {
            store: ActivityCenterAlphaStore::in_memory(),
        }
    }

    /// Opens a file-backed runtime.
    ///
    /// # Errors
    ///
    /// Returns an error when the row file cannot be read or deserialized.
    pub fn file_backed(path: impl Into<PathBuf>) -> Result<Self, ActivityCenterAlphaError> {
        Ok(Self {
            store: ActivityCenterAlphaStore::file_backed(path)?,
        })
    }

    /// Records one activity row.
    ///
    /// # Errors
    ///
    /// Returns an error when persistence fails.
    pub fn record_row(&mut self, row: ActivityRow) -> Result<(), ActivityCenterAlphaError> {
        self.store.record_row(row)
    }

    /// Returns a snapshot of current rows.
    pub fn snapshot(&self) -> ActivityCenterAlphaSnapshot {
        self.store.snapshot()
    }

    /// Builds a structured support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> ActivityCenterSupportExport {
        ActivityCenterSupportExport::from_snapshot(export_id, generated_at, &self.snapshot())
    }

    /// Finds a row by stable row id.
    pub fn find_row(&self, activity_row_id: &str) -> Option<&ActivityRow> {
        self.store.find_row(activity_row_id)
    }

    /// Flushes the file-backed store.
    ///
    /// # Errors
    ///
    /// Returns an error when persistence fails.
    pub fn persist_now(&self) -> Result<(), ActivityCenterAlphaError> {
        self.store.persist_now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(
        kind: ActivityRowActionKind,
        id_suffix: &str,
        label: &str,
        command_id: Option<&str>,
        target: &str,
        reissues: bool,
    ) -> ActivityRowAction {
        ActivityRowAction {
            action_id: format!("action:activity:{id_suffix}"),
            action_kind: kind,
            label: label.into(),
            command_id: command_id.map(ToOwned::to_owned),
            availability_class: ActivityRowActionAvailability::Enabled,
            disabled_reason_label: None,
            target_identity_ref: target.into(),
            preserves_durable_history: true,
            reissues_original_side_effect: reissues,
        }
    }

    fn support(exportable: bool, item: &str) -> ActivityRowSupportLink {
        ActivityRowSupportLink {
            exportable,
            support_pack_item_id: exportable.then(|| item.to_owned()),
            bundle_member_path_ref: exportable.then(|| format!("manifest/activity/{item}.json")),
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_private_material_excluded: true,
            export_field_refs: vec![
                "export.activity.identity".into(),
                "export.activity.state".into(),
                "export.activity.reopen".into(),
            ],
        }
    }

    fn row_input(
        row_id: &str,
        family: ActivityJobFamily,
        subsystem: SourceSubsystem,
        state: ActivityRowStateClass,
        summary: &str,
    ) -> ActivityRowInput {
        let target = format!("ux:durable-job:{row_id}");
        ActivityRowInput {
            activity_row_id: format!("ux:activity-row:{row_id}"),
            durable_job_id: target.clone(),
            canonical_event_id: format!("ux:event:{row_id}"),
            canonical_object_target_ref: target.clone(),
            exact_reopen_identity_ref: format!("ux:activity-row:{row_id}"),
            job_family: family,
            source_subsystem: subsystem,
            actor_identity_ref: format!("id:actor:system:{}", family.as_str()),
            actor_or_subsystem_label: family.label().into(),
            execution_origin_class: "system_background".into(),
            severity_class: if matches!(state, ActivityRowStateClass::Failed) {
                SeverityClass::Error
            } else {
                SeverityClass::Info
            },
            privacy_class: PrivacyClass::WorkspaceSensitive,
            summary_label: summary.into(),
            target_label: family.label().into(),
            target_scope_label: "Active workspace".into(),
            state_class: state,
            progress: ActivityRowProgress {
                forms: vec![ActivityProgressForm::PhaseOnly],
                phase_label: state.label().into(),
                progress_bar: None,
                queue_reason_label: None,
                approval_source_label: None,
                actor_or_subsystem_label: family.label().into(),
                age_label: "1 min".into(),
                indeterminate_reason_label: None,
                expected_boundary_class: "local".into(),
                cancellability_class: if state.is_terminal() {
                    ActivityCancellabilityClass::AlreadyTerminal
                } else {
                    ActivityCancellabilityClass::Cancellable
                },
                detail_or_evidence_ref: Some(format!("evidence:{row_id}")),
            },
            timeline: ActivityRowTimeline {
                minted_at: "2026-05-13T03:10:00Z".into(),
                queued_at: Some("2026-05-13T03:10:00Z".into()),
                started_at: Some("2026-05-13T03:10:02Z".into()),
                last_observed_at: "2026-05-13T03:11:00Z".into(),
                finished_at: state.is_terminal().then(|| "2026-05-13T03:11:00Z".into()),
                archived_at: None,
                superseded_by_row_id_ref: None,
                retention_label: "Retained until resolved or archived".into(),
            },
            impact: ActivityRowImpact::routine("Routine local activity row."),
            actions: vec![ActivityRowAction::open_details(
                format!("action:activity:{row_id}:open"),
                "Open details",
                format!("ux:activity-row:{row_id}"),
            )],
            display: ActivityRowDisplayState {
                collapse_state: ActivityRowCollapseState::CollapsedSummary,
                can_expand_inline: true,
                expand_reveals_label: "phase, evidence, and chronology".into(),
            },
            support_link: support(true, &format!("support.item.activity.{row_id}")),
            git_review_context: None,
            occurrence_count: 1,
        }
    }

    fn sample_rows() -> Vec<ActivityRow> {
        let mut index = ActivityRow::from_input(row_input(
            "index:hot-set",
            ActivityJobFamily::Indexing,
            SourceSubsystem::Indexer,
            ActivityRowStateClass::Running,
            "Indexing active workspace hot set",
        ));
        index.progress.forms = vec![
            ActivityProgressForm::LabeledProgressBar,
            ActivityProgressForm::PhaseOnly,
        ];
        index.progress.progress_bar =
            Some(ActivityProgressBar::new("Files indexed", 82, 100, "files"));
        index.actions.push(action(
            ActivityRowActionKind::CancelJob,
            "index:hot-set:cancel",
            "Cancel indexing",
            Some("cmd:indexer.cancel"),
            &index.durable_job_id,
            false,
        ));

        let restore = ActivityRow::from_input(row_input(
            "restore:last-session",
            ActivityJobFamily::Restore,
            SourceSubsystem::Shell,
            ActivityRowStateClass::Completed,
            "Restore completed",
        ));

        let mut update = ActivityRow::from_input(row_input(
            "update:packages",
            ActivityJobFamily::InstallUpdate,
            SourceSubsystem::InstallUpdateAttach,
            ActivityRowStateClass::PartiallyCompleted,
            "Package update needs review",
        ));
        update.impact = ActivityRowImpact {
            affects_cost: false,
            affects_policy: true,
            affects_network: true,
            affects_trust: true,
            affects_provider_state: false,
            affects_recovery_posture: true,
            detail_or_evidence_required: true,
            impact_summary_sentence:
                "Package update crossed policy, network, trust, and rollback boundaries.".into(),
        };

        let mut task = ActivityRow::from_input(row_input(
            "task:dev-server",
            ActivityJobFamily::TaskRun,
            SourceSubsystem::TaskRunner,
            ActivityRowStateClass::QueuedWaiting,
            "Development server queued",
        ));
        task.progress.forms = vec![
            ActivityProgressForm::QueueReason,
            ActivityProgressForm::PhaseOnly,
        ];
        task.progress.queue_reason_label = Some("Waiting for execution profile".into());

        let mut test = ActivityRow::from_input(row_input(
            "test:pytest",
            ActivityJobFamily::TestRun,
            SourceSubsystem::TestRunner,
            ActivityRowStateClass::Failed,
            "Test run failed",
        ));
        test.actions.push(action(
            ActivityRowActionKind::RetryJob,
            "test:pytest:retry",
            "Retry failed tests",
            Some("cmd:test-run.retry-failed"),
            &test.durable_job_id,
            true,
        ));

        vec![index, restore, update, task, test]
    }

    #[test]
    fn snapshot_covers_required_families_and_exact_reopen() {
        let snapshot = ActivityCenterAlphaSnapshot::from_rows(sample_rows());
        assert_eq!(snapshot.len(), 5);
        assert!(snapshot.covers_required_alpha_families());
        assert_eq!(snapshot.exact_reopen_row_count, 5);
        assert!(snapshot.honesty_marker_present);
        assert!(snapshot
            .rows
            .iter()
            .all(ActivityRow::satisfies_sensitive_detail_rule));
        assert!(snapshot
            .rows
            .iter()
            .any(|row| row.actions.iter().any(|action| {
                action.action_kind == ActivityRowActionKind::RetryJob
                    && action.reissues_original_side_effect
            })));
    }

    #[test]
    fn file_backed_store_reopens_terminal_history() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("activity_rows.json");
        let row = sample_rows()
            .into_iter()
            .find(|row| row.job_family == ActivityJobFamily::Restore)
            .expect("restore row");

        {
            let mut runtime = ActivityCenterAlphaRuntime::file_backed(&path).expect("open");
            runtime.record_row(row.clone()).expect("record");
            runtime.persist_now().expect("flush");
        }

        let runtime = ActivityCenterAlphaRuntime::file_backed(&path).expect("reopen");
        let reopened = runtime
            .find_row(&row.activity_row_id)
            .expect("row survives restart");
        assert_eq!(reopened.state_class, ActivityRowStateClass::Completed);
        assert!(reopened.has_exact_reopen_identity());
        assert!(reopened.retained_until_resolved_or_archived);
    }

    #[test]
    fn support_export_is_structured_and_export_safe() {
        let snapshot = ActivityCenterAlphaSnapshot::from_rows(sample_rows());
        let export = ActivityCenterSupportExport::from_snapshot(
            "support-export:activity:center:alpha",
            "2026-05-13T03:12:00Z",
            &snapshot,
        );

        assert_eq!(export.row_count(), 5);
        assert!(export.raw_private_material_excluded);
        assert!(export
            .rows
            .iter()
            .any(|row| row.job_family == ActivityJobFamily::TestRun));
        assert!(export.rows.iter().all(|row| row
            .support_pack_item_id
            .starts_with("support.item.activity.")));
    }

    #[test]
    fn fixture_snapshot_round_trips() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("fixtures")
                    .join("ux")
                    .join("activity_center_alpha")
                    .join("activity_center_alpha_snapshot.json")
            })
            .expect("derive fixture path");
        let bytes = std::fs::read(&path)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let snapshot: ActivityCenterAlphaSnapshot = serde_json::from_slice(&bytes)
            .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

        assert_eq!(
            snapshot.record_kind,
            ACTIVITY_CENTER_ALPHA_SNAPSHOT_RECORD_KIND
        );
        assert!(snapshot.covers_required_alpha_families());
        assert!(snapshot.support_exportable_row_count >= 1);
        assert_eq!(snapshot.exact_reopen_row_count, snapshot.rows.len());
        assert!(snapshot
            .rows
            .iter()
            .all(ActivityRow::satisfies_sensitive_detail_rule));
    }

    #[test]
    fn fixture_support_export_round_trips() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("fixtures")
                    .join("ux")
                    .join("activity_center_alpha")
                    .join("support_export_activity_rows.json")
            })
            .expect("derive fixture path");
        let bytes = std::fs::read(&path)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let export: ActivityCenterSupportExport = serde_json::from_slice(&bytes)
            .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

        assert_eq!(
            export.record_kind,
            ACTIVITY_CENTER_SUPPORT_EXPORT_RECORD_KIND
        );
        assert!(export.row_count() >= 1);
        assert!(export.raw_private_material_excluded);
        assert!(export
            .rows
            .iter()
            .any(|row| row.job_family == ActivityJobFamily::TestRun));
    }
}
