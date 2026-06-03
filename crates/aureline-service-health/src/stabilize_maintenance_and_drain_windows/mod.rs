//! Scheduled maintenance, read-only/drain windows, tenant migration, failover,
//! and publish-later/local-draft continuity for managed and provider-linked surfaces.
//!
//! This module owns the typed service-health continuity records that answer:
//! **For every planned or unplanned managed window — maintenance, read-only,
//! drain, tenant migration, failover — does the product know exact start/end
//! time, what becomes read-only or blocked, what remains safely local, and how
//! publish-later or local drafts reconcile afterward?**
//!
//! The model reuses the provider deferred-intent and reconciliation vocabulary
//! ([`aureline_provider::publish_later::QueueState`],
//! [`aureline_provider::reconciliation::ReconciliationResult`],
//! [`aureline_provider::reconciliation::ProviderDriftClass`]) so drain-window
//! local-draft continuity does not become a one-off special case.
//!
//! Every notice names:
//! - exact time window with timezone and latest-refresh time;
//! - affected surfaces and blocked write classes;
//! - local-safe actions that remain available;
//! - defer and export options;
//! - stale-notice downgrade rules so outdated cards cannot overclaim current posture.
//!
//! Post-window reconciliation revalidates tenant, region, endpoint, policy, auth,
//! and target identity and shows a reconciliation sheet whenever any dimension
//! drifted during the window.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use aureline_provider::{
    reconciliation::{ProviderDriftClass, ReconciliationNextActionClass},
    registry::{FreshnessLabel, FreshnessTruth, RedactionClass},
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for service-health continuity records.
pub const SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by service-health continuity records.
pub const SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF: &str =
    "service_health:continuity:v1";

/// Stable record-kind tag for [`ServiceHealthContinuityPage`].
pub const SERVICE_HEALTH_CONTINUITY_PAGE_RECORD_KIND: &str =
    "service_health_continuity_page_record";

/// Stable record-kind tag for [`ScheduledMaintenanceNotice`].
pub const SCHEDULED_MAINTENANCE_NOTICE_RECORD_KIND: &str =
    "scheduled_maintenance_notice_record";

/// Stable record-kind tag for [`MaintenanceWindowStateRecord`].
pub const MAINTENANCE_WINDOW_STATE_RECORD_KIND: &str =
    "maintenance_window_state_record";

/// Stable record-kind tag for [`BlockedWriteDisclosure`].
pub const BLOCKED_WRITE_DISCLOSURE_RECORD_KIND: &str =
    "blocked_write_disclosure_record";

/// Stable record-kind tag for [`StaleNoticeDowngradeRule`].
pub const STALE_NOTICE_DOWNGRADE_RULE_RECORD_KIND: &str =
    "stale_notice_downgrade_rule_record";

/// Stable record-kind tag for [`PostWindowReconciliationResult`].
pub const POST_WINDOW_RECONCILIATION_RESULT_RECORD_KIND: &str =
    "post_window_reconciliation_result_record";

/// Stable record-kind tag for [`ServiceHealthContinuityValidationReport`].
pub const SERVICE_HEALTH_CONTINUITY_VALIDATION_REPORT_RECORD_KIND: &str =
    "service_health_continuity_validation_report_record";

/// Stable record-kind tag for [`ServiceHealthContinuitySupportExport`].
pub const SERVICE_HEALTH_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "service_health_continuity_support_export_record";

// ---------------------------------------------------------------------------
// Kinds and classes
// ---------------------------------------------------------------------------

/// The kind of maintenance or continuity notice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceNoticeKind {
    /// Planned maintenance with exact start and end time.
    ScheduledMaintenance,
    /// Unplanned interruption that became known after onset.
    UnplannedInterruption,
    /// Tenant migration window.
    TenantMigration,
    /// Failover test or drill window.
    FailoverTest,
    /// Failover recovery window.
    FailoverRecovery,
    /// Read-only window imposed on managed surfaces.
    ReadOnlyWindow,
    /// Drain-in-progress window where new writes are blocked.
    DrainInProgress,
}

/// The operational state of a managed or provider-linked window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceWindowState {
    /// Window is scheduled but has not started.
    Scheduled,
    /// Managed surfaces are read-only.
    ReadOnly,
    /// Drain is in progress; new high-risk work is blocked.
    DrainInProgress,
    /// Tenant migration is underway.
    Migration,
    /// Failover is active.
    Failover,
    /// System is reconciling after a window closed.
    Reconciling,
    /// Window has resolved and normal operation resumed.
    Resolved,
}

/// Classification of a write operation that is blocked during a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedWriteClass {
    /// Provider-side mutations (publish, update, delete).
    ProviderMutation,
    /// Review publish or comment actions.
    ReviewPublish,
    /// Work-item create or update.
    WorkItemCreateOrUpdate,
    /// Comment publish on provider-backed threads.
    CommentPublish,
    /// Issue or incident report creation.
    IssueReportCreate,
    /// Share or join operations on collaborative sessions.
    ShareOrJoin,
    /// Collaboration sync push.
    CollaborationSyncPush,
    /// Settings sync mutation.
    SettingsSyncMutation,
}

/// Local-safe action that remains available during a read-only or drain window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSafeActionClass {
    /// Continue local-draft authoring.
    LocalDraftAuthoring,
    /// Inspect-only viewing of cached provider state.
    InspectOnly,
    /// Export an evidence-safe packet.
    ExportEvidencePacket,
    /// Queue intent into publish-later for post-window drain.
    QueuePublishLater,
    /// Defer the action until the window closes.
    DeferToPostWindow,
    /// Open the object in the provider through typed handoff.
    OpenInProviderBrowserHandoff,
}

/// Surface affected by a maintenance or continuity notice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedSurfaceClass {
    /// Desktop IDE shell.
    Desktop,
    /// Review surface (diff, comments, approvals).
    Review,
    /// Collaboration surface (shared sessions, presence).
    Collab,
    /// Provider sync and mutation surfaces.
    ProviderSync,
    /// Update center.
    UpdateCenter,
    /// Service-health card or banner.
    ServiceHealthCard,
    /// Support export packet.
    SupportExport,
    /// CLI/headless inspect output.
    CliHeadless,
}

/// Stale-notice downgrade class for cached maintenance cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleNoticeDowngradeClass {
    /// Notice is current and may be shown at full severity.
    Current,
    /// Notice is stale but still within a bounded grace window; show with degraded severity.
    StaleWithinGrace,
    /// Notice has expired beyond the grace window; require refresh before display.
    ExpiredRequiresRefresh,
}

/// Post-window reconciliation state for queued or publish-later intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostWindowReconciliationState {
    /// Intent is still queued awaiting drain.
    Queued,
    /// Intent was replayed and committed after the window.
    Replayed,
    /// Intent expired before the window closed.
    Expired,
    /// Intent was cancelled by the user or policy.
    Cancelled,
    /// Intent requires explicit review because a revalidation dimension drifted.
    NeedsExplicitReview,
}

/// One dimension that must be revalidated after a maintenance, migration, or failover window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationDimension {
    /// Tenant or organization identity.
    Tenant,
    /// Region or datacenter placement.
    Region,
    /// Service endpoint URL or identity.
    Endpoint,
    /// Effective policy epoch or bundle.
    Policy,
    /// Authentication scope or session validity.
    Auth,
    /// Provider target identity (repository, project, org).
    TargetIdentity,
}

/// Defer option offered on a maintenance notice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferOptionClass {
    /// Remind again at a specific time before the window starts.
    RemindAt,
    /// Silence this notice until the window starts.
    SilenceUntilStart,
    /// Export the notice to a calendar entry.
    ExportToCalendar,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthContinuityFindingSeverity {
    /// Error that blocks validation.
    Error,
    /// Warning that keeps output reviewable but degraded.
    Warning,
}

// ---------------------------------------------------------------------------
// Time and window records
// ---------------------------------------------------------------------------

/// Exact time window with timezone and freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactTimeWindow {
    /// ISO-8601 start timestamp.
    pub start_time: String,
    /// ISO-8601 end timestamp.
    pub end_time: String,
    /// IANA timezone name (e.g. `UTC`, `America/New_York`).
    pub timezone: String,
    /// ISO-8601 timestamp of the latest refresh of this window.
    pub latest_refresh_time: String,
}

impl ExactTimeWindow {
    /// Returns true when all time fields are non-empty.
    pub fn is_complete(&self) -> bool {
        !self.start_time.trim().is_empty()
            && !self.end_time.trim().is_empty()
            && !self.timezone.trim().is_empty()
            && !self.latest_refresh_time.trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Notice and disclosure records
// ---------------------------------------------------------------------------

/// One blocked-write disclosure naming the class, rationale, and prevented action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedWriteDisclosure {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable disclosure id.
    pub disclosure_id: String,
    /// Write class that is blocked.
    pub write_class: BlockedWriteClass,
    /// Redaction-safe rationale for the block.
    pub rationale: String,
    /// Summary of the prevented user action.
    pub prevents_action_summary: String,
    /// Surfaces where this block applies.
    pub affected_surfaces: Vec<AffectedSurfaceClass>,
    /// Whether local-draft continuity is offered as an alternative.
    pub local_draft_alternative_offered: bool,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// One local-safe action offered during a window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafeAction {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable action id.
    pub action_id: String,
    /// Action class that remains safe.
    pub action_class: LocalSafeActionClass,
    /// Redaction-safe rationale.
    pub rationale: String,
    /// Surfaces where this action is available.
    pub available_on_surfaces: Vec<AffectedSurfaceClass>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// One scheduled or unplanned maintenance notice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledMaintenanceNotice {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable notice id.
    pub notice_id: String,
    /// Notice kind.
    pub notice_kind: MaintenanceNoticeKind,
    /// Exact time window.
    pub time_window: ExactTimeWindow,
    /// Surfaces affected by this notice.
    pub affected_surfaces: Vec<AffectedSurfaceClass>,
    /// Blocked write disclosures.
    pub blocked_writes: Vec<BlockedWriteDisclosure>,
    /// Local-safe actions offered.
    pub local_safe_actions: Vec<LocalSafeAction>,
    /// Defer options offered to the user.
    pub defer_options: Vec<DeferOptionClass>,
    /// Whether an export-to-calendar or export-to-packet option is offered.
    pub export_option_offered: bool,
    /// Freshness truth for this notice.
    pub freshness: FreshnessTruth,
    /// Stale-notice downgrade class.
    pub stale_downgrade_class: StaleNoticeDowngradeClass,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Guardrail: no vague copy such as `Service interruption soon` is present.
    pub no_vague_copy: bool,
    /// Guardrail: notice names at least one local-safe action.
    pub local_safe_action_named: bool,
}

// ---------------------------------------------------------------------------
// Window state and stale-notice records
// ---------------------------------------------------------------------------

/// Current operational state record for one maintenance or continuity window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceWindowStateRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable state record id.
    pub state_id: String,
    /// Current window state.
    pub current_state: MaintenanceWindowState,
    /// Previous window state, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<MaintenanceWindowState>,
    /// Exact time window.
    pub time_window: ExactTimeWindow,
    /// Blocked write disclosures active in this state.
    pub blocked_writes: Vec<BlockedWriteDisclosure>,
    /// Local-safe actions active in this state.
    pub local_safe_actions: Vec<LocalSafeAction>,
    /// Next expected state transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_expected_transition: Option<MaintenanceWindowState>,
    /// Estimated time of next transition, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_transition_time: Option<String>,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Stale-notice downgrade rule ensuring outdated cards do not overclaim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleNoticeDowngradeRule {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable rule id.
    pub rule_id: String,
    /// Freshness label that triggers this rule.
    pub freshness_class: FreshnessLabel,
    /// Downgrade class applied.
    pub downgrade_class: StaleNoticeDowngradeClass,
    /// Visible label the user sees after downgrade.
    pub visible_label: String,
    /// Whether the notice must be hidden entirely.
    pub hide_notice: bool,
    /// Whether a refresh action is offered.
    pub refresh_action_offered: bool,
    /// Redaction-safe rationale.
    pub rationale: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

// ---------------------------------------------------------------------------
// Post-window reconciliation records
// ---------------------------------------------------------------------------

/// Result of revalidating one dimension after a window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevalidationDimensionResult {
    /// Dimension revalidated.
    pub dimension: RevalidationDimension,
    /// Value observed before the window.
    pub pre_window_value: String,
    /// Value observed after the window.
    pub post_window_value: String,
    /// Whether the dimension drifted.
    pub drifted: bool,
    /// Redaction-safe drift explanation, when drifted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_explanation: Option<String>,
}

/// Post-window reconciliation result for queued or publish-later intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostWindowReconciliationResult {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable result id.
    pub result_id: String,
    /// Ref to the pre-window state record.
    pub pre_window_state_ref: String,
    /// Ref to the post-window state record.
    pub post_window_state_ref: String,
    /// Revalidation results per dimension.
    pub revalidation_results: Vec<RevalidationDimensionResult>,
    /// Drifted dimensions.
    pub drifted_dimensions: Vec<RevalidationDimension>,
    /// Reconciliation state.
    pub reconciliation_state: PostWindowReconciliationState,
    /// Queue item refs that require review.
    pub queued_intent_refs: Vec<String>,
    /// Whether replay requires explicit review.
    pub replay_requires_review: bool,
    /// Provider drift class observed.
    pub provider_drift_class: ProviderDriftClass,
    /// Next safe action.
    pub next_safe_action: ReconciliationNextActionClass,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Guardrail: no write intent replays invisibly.
    pub no_invisible_replay: bool,
}

// ---------------------------------------------------------------------------
// Contract refs
// ---------------------------------------------------------------------------

/// Upstream schema and contract refs consumed by the continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthContinuityContractRefs {
    /// Provider publish-later queue schema ref.
    pub publish_later_queue_schema_ref: String,
    /// Provider deferred-publish queue schema ref.
    pub deferred_publish_queue_schema_ref: String,
    /// Provider event reconciliation schema ref.
    pub provider_event_reconciliation_schema_ref: String,
    /// Provider object model schema ref.
    pub provider_object_model_schema_ref: String,
    /// Settings sync state schema ref.
    pub settings_sync_state_schema_ref: String,
    /// Backup-restore-failover continuity schema ref.
    pub backup_restore_failover_schema_ref: String,
}

impl ServiceHealthContinuityContractRefs {
    fn all_refs(&self) -> [&str; 6] {
        [
            &self.publish_later_queue_schema_ref,
            &self.deferred_publish_queue_schema_ref,
            &self.provider_event_reconciliation_schema_ref,
            &self.provider_object_model_schema_ref,
            &self.settings_sync_state_schema_ref,
            &self.backup_restore_failover_schema_ref,
        ]
    }
}

// ---------------------------------------------------------------------------
// Page, validation, and support export
// ---------------------------------------------------------------------------

/// Fixture metadata used by protected cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthContinuityFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
}

/// Service-health continuity page containing notices, state records, downgrade rules, and reconciliation results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthContinuityPage {
    /// Optional fixture metadata for validation lanes.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<ServiceHealthContinuityFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Contract refs consumed.
    pub contract_refs: ServiceHealthContinuityContractRefs,
    /// Scheduled maintenance and continuity notices.
    pub maintenance_notices: Vec<ScheduledMaintenanceNotice>,
    /// Current window state records.
    pub window_state_records: Vec<MaintenanceWindowStateRecord>,
    /// Stale-notice downgrade rules.
    pub stale_notice_rules: Vec<StaleNoticeDowngradeRule>,
    /// Post-window reconciliation results.
    pub reconciliation_results: Vec<PostWindowReconciliationResult>,
    /// Redaction-safe page summary.
    pub support_export_summary: String,
}

impl ServiceHealthContinuityPage {
    /// Validates the page against service-health continuity invariants.
    pub fn validate(&self) -> ServiceHealthContinuityValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Builds a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> ServiceHealthContinuitySupportExport {
        ServiceHealthContinuitySupportExport {
            record_kind: SERVICE_HEALTH_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            notice_summaries: self
                .maintenance_notices
                .iter()
                .map(ScheduledMaintenanceNoticeSummary::from)
                .collect(),
            window_state_summaries: self
                .window_state_records
                .iter()
                .map(MaintenanceWindowStateSummary::from)
                .collect(),
            stale_rule_summaries: self
                .stale_notice_rules
                .iter()
                .map(StaleNoticeDowngradeRuleSummary::from)
                .collect(),
            reconciliation_summaries: self
                .reconciliation_results
                .iter()
                .map(PostWindowReconciliationSummary::from)
                .collect(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }
}

/// Validation report emitted by service-health continuity validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthContinuityValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: ServiceHealthContinuityCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ServiceHealthContinuityFinding>,
}

/// Coverage observed during service-health continuity validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ServiceHealthContinuityCoverage {
    /// Notice kinds covered.
    pub notice_kinds: BTreeSet<MaintenanceNoticeKind>,
    /// Window states covered.
    pub window_states: BTreeSet<MaintenanceWindowState>,
    /// Blocked write classes covered.
    pub blocked_write_classes: BTreeSet<BlockedWriteClass>,
    /// Local safe action classes covered.
    pub local_safe_action_classes: BTreeSet<LocalSafeActionClass>,
    /// Stale downgrade classes covered.
    pub stale_downgrade_classes: BTreeSet<StaleNoticeDowngradeClass>,
    /// Post-window reconciliation states covered.
    pub reconciliation_states: BTreeSet<PostWindowReconciliationState>,
    /// Affected surface classes covered.
    pub affected_surfaces: BTreeSet<AffectedSurfaceClass>,
    /// Revalidation dimensions covered.
    pub revalidation_dimensions: BTreeSet<RevalidationDimension>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthContinuityFinding {
    /// Severity.
    pub severity: ServiceHealthContinuityFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe finding message.
    pub message: String,
}

/// Redaction-safe support export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthContinuitySupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version exported.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Notice summaries.
    pub notice_summaries: Vec<ScheduledMaintenanceNoticeSummary>,
    /// Window state summaries.
    pub window_state_summaries: Vec<MaintenanceWindowStateSummary>,
    /// Stale rule summaries.
    pub stale_rule_summaries: Vec<StaleNoticeDowngradeRuleSummary>,
    /// Reconciliation summaries.
    pub reconciliation_summaries: Vec<PostWindowReconciliationSummary>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
}

/// Redaction-safe scheduled maintenance notice summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledMaintenanceNoticeSummary {
    /// Notice id.
    pub notice_id: String,
    /// Notice kind.
    pub notice_kind: MaintenanceNoticeKind,
    /// Start time.
    pub start_time: String,
    /// End time.
    pub end_time: String,
    /// Timezone.
    pub timezone: String,
    /// Stale downgrade class.
    pub stale_downgrade_class: StaleNoticeDowngradeClass,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&ScheduledMaintenanceNotice> for ScheduledMaintenanceNoticeSummary {
    fn from(notice: &ScheduledMaintenanceNotice) -> Self {
        Self {
            notice_id: notice.notice_id.clone(),
            notice_kind: notice.notice_kind,
            start_time: notice.time_window.start_time.clone(),
            end_time: notice.time_window.end_time.clone(),
            timezone: notice.time_window.timezone.clone(),
            stale_downgrade_class: notice.stale_downgrade_class,
            support_export_summary: notice.support_export_summary.clone(),
        }
    }
}

/// Redaction-safe window state summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceWindowStateSummary {
    /// State id.
    pub state_id: String,
    /// Current state.
    pub current_state: MaintenanceWindowState,
    /// Start time.
    pub start_time: String,
    /// End time.
    pub end_time: String,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&MaintenanceWindowStateRecord> for MaintenanceWindowStateSummary {
    fn from(record: &MaintenanceWindowStateRecord) -> Self {
        Self {
            state_id: record.state_id.clone(),
            current_state: record.current_state,
            start_time: record.time_window.start_time.clone(),
            end_time: record.time_window.end_time.clone(),
            support_export_summary: record.support_export_summary.clone(),
        }
    }
}

/// Redaction-safe stale-notice downgrade rule summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleNoticeDowngradeRuleSummary {
    /// Rule id.
    pub rule_id: String,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Downgrade class.
    pub downgrade_class: StaleNoticeDowngradeClass,
    /// Visible label.
    pub visible_label: String,
    /// Hide notice flag.
    pub hide_notice: bool,
}

impl From<&StaleNoticeDowngradeRule> for StaleNoticeDowngradeRuleSummary {
    fn from(rule: &StaleNoticeDowngradeRule) -> Self {
        Self {
            rule_id: rule.rule_id.clone(),
            freshness_class: rule.freshness_class,
            downgrade_class: rule.downgrade_class,
            visible_label: rule.visible_label.clone(),
            hide_notice: rule.hide_notice,
        }
    }
}

/// Redaction-safe post-window reconciliation summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostWindowReconciliationSummary {
    /// Result id.
    pub result_id: String,
    /// Pre-window state ref.
    pub pre_window_state_ref: String,
    /// Post-window state ref.
    pub post_window_state_ref: String,
    /// Drifted dimensions.
    pub drifted_dimensions: Vec<RevalidationDimension>,
    /// Reconciliation state.
    pub reconciliation_state: PostWindowReconciliationState,
    /// Replay requires review.
    pub replay_requires_review: bool,
    /// Provider drift class.
    pub provider_drift_class: ProviderDriftClass,
    /// Next safe action.
    pub next_safe_action: ReconciliationNextActionClass,
    /// Support summary.
    pub support_export_summary: String,
}

impl From<&PostWindowReconciliationResult> for PostWindowReconciliationSummary {
    fn from(result: &PostWindowReconciliationResult) -> Self {
        Self {
            result_id: result.result_id.clone(),
            pre_window_state_ref: result.pre_window_state_ref.clone(),
            post_window_state_ref: result.post_window_state_ref.clone(),
            drifted_dimensions: result.drifted_dimensions.clone(),
            reconciliation_state: result.reconciliation_state,
            replay_requires_review: result.replay_requires_review,
            provider_drift_class: result.provider_drift_class,
            next_safe_action: result.next_safe_action,
            support_export_summary: result.support_export_summary.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Validator
// ---------------------------------------------------------------------------

struct Validator<'a> {
    page: &'a ServiceHealthContinuityPage,
    notice_ids: BTreeSet<&'a str>,
    state_ids: BTreeSet<&'a str>,
    rule_ids: BTreeSet<&'a str>,
    result_ids: BTreeSet<&'a str>,
    coverage: ServiceHealthContinuityCoverage,
    findings: Vec<ServiceHealthContinuityFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a ServiceHealthContinuityPage) -> Self {
        Self {
            page,
            notice_ids: BTreeSet::new(),
            state_ids: BTreeSet::new(),
            rule_ids: BTreeSet::new(),
            result_ids: BTreeSet::new(),
            coverage: ServiceHealthContinuityCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.collect_ids();
        self.validate_notices();
        self.validate_window_states();
        self.validate_stale_rules();
        self.validate_reconciliation_results();
        self.validate_required_coverage();
    }

    fn finish(self) -> ServiceHealthContinuityValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ServiceHealthContinuityFindingSeverity::Error);
        ServiceHealthContinuityValidationReport {
            record_kind: SERVICE_HEALTH_CONTINUITY_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        self.expect(
            self.page.record_kind == SERVICE_HEALTH_CONTINUITY_PAGE_RECORD_KIND,
            "service_health_continuity.page_record_kind",
            "page.record_kind must match the service-health continuity page record kind",
        );
        self.expect(
            self.page.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            "service_health_continuity.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            self.page.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
            "service_health_continuity.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !self.page.page_id.trim().is_empty(),
            "service_health_continuity.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !self.page.support_export_summary.trim().is_empty(),
            "service_health_continuity.page_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in self.page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "service_health_continuity.contract_ref_missing",
                "every consumed contract ref must be non-empty",
            );
        }
    }

    fn collect_ids(&mut self) {
        for notice in &self.page.maintenance_notices {
            let inserted = self.notice_ids.insert(notice.notice_id.as_str());
            self.expect(
                inserted,
                "service_health_continuity.notice_id_duplicate",
                "notice.notice_id must be unique",
            );
        }
        for state in &self.page.window_state_records {
            let inserted = self.state_ids.insert(state.state_id.as_str());
            self.expect(
                inserted,
                "service_health_continuity.state_id_duplicate",
                "window_state.state_id must be unique",
            );
        }
        for rule in &self.page.stale_notice_rules {
            let inserted = self.rule_ids.insert(rule.rule_id.as_str());
            self.expect(
                inserted,
                "service_health_continuity.rule_id_duplicate",
                "stale_notice_rule.rule_id must be unique",
            );
        }
        for result in &self.page.reconciliation_results {
            let inserted = self.result_ids.insert(result.result_id.as_str());
            self.expect(
                inserted,
                "service_health_continuity.result_id_duplicate",
                "reconciliation_result.result_id must be unique",
            );
        }
    }

    fn validate_notices(&mut self) {
        for notice in &self.page.maintenance_notices {
            self.coverage.notice_kinds.insert(notice.notice_kind);
            self.expect(
                notice.record_kind == SCHEDULED_MAINTENANCE_NOTICE_RECORD_KIND,
                "service_health_continuity.notice_record_kind",
                "notice.record_kind must match scheduled_maintenance_notice_record",
            );
            self.expect(
                notice.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
                "service_health_continuity.notice_schema_version",
                "notice.schema_version must match the crate constant",
            );
            self.expect(
                notice.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
                "service_health_continuity.notice_shared_contract_ref",
                "notice.shared_contract_ref must match the shared contract id",
            );
            self.expect(
                notice.time_window.is_complete(),
                "service_health_continuity.notice_time_window_incomplete",
                "notice.time_window must have start, end, timezone, and latest_refresh",
            );
            self.expect(
                !notice.affected_surfaces.is_empty(),
                "service_health_continuity.notice_affected_surfaces_missing",
                "notice must name at least one affected surface",
            );
            self.expect(
                !notice.blocked_writes.is_empty(),
                "service_health_continuity.notice_blocked_writes_missing",
                "notice must name at least one blocked write class",
            );
            self.expect(
                !notice.local_safe_actions.is_empty(),
                "service_health_continuity.notice_local_safe_actions_missing",
                "notice must name at least one local-safe action",
            );
            self.expect(
                notice.no_vague_copy,
                "service_health_continuity.notice_vague_copy_present",
                "notice must not carry vague copy such as `Service interruption soon`",
            );
            self.expect(
                notice.local_safe_action_named,
                "service_health_continuity.notice_local_safe_action_flag_false",
                "notice.local_safe_action_named must be true",
            );
            self.expect(
                !notice.support_export_summary.trim().is_empty(),
                "service_health_continuity.notice_summary_missing",
                "notice.support_export_summary must be non-empty",
            );
            for disclosure in &notice.blocked_writes {
                self.validate_blocked_write_disclosure(disclosure);
            }
            for action in &notice.local_safe_actions {
                self.validate_local_safe_action(action);
            }
        }
    }

    fn validate_blocked_write_disclosure(&mut self, disclosure: &BlockedWriteDisclosure) {
        self.coverage.blocked_write_classes.insert(disclosure.write_class);
        self.expect(
            disclosure.record_kind == BLOCKED_WRITE_DISCLOSURE_RECORD_KIND,
            "service_health_continuity.disclosure_record_kind",
            "blocked_write_disclosure.record_kind must match blocked_write_disclosure_record",
        );
        self.expect(
            disclosure.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            "service_health_continuity.disclosure_schema_version",
            "blocked_write_disclosure.schema_version must match the crate constant",
        );
        self.expect(
            disclosure.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
            "service_health_continuity.disclosure_shared_contract_ref",
            "blocked_write_disclosure.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !disclosure.disclosure_id.trim().is_empty(),
            "service_health_continuity.disclosure_id_missing",
            "blocked_write_disclosure.disclosure_id must be non-empty",
        );
        self.expect(
            !disclosure.rationale.trim().is_empty(),
            "service_health_continuity.disclosure_rationale_missing",
            "blocked_write_disclosure.rationale must be non-empty",
        );
        self.expect(
            !disclosure.prevents_action_summary.trim().is_empty(),
            "service_health_continuity.disclosure_prevents_action_missing",
            "blocked_write_disclosure.prevents_action_summary must be non-empty",
        );
        self.expect(
            !disclosure.affected_surfaces.is_empty(),
            "service_health_continuity.disclosure_affected_surfaces_missing",
            "blocked_write_disclosure must name at least one affected surface",
        );
    }

    fn validate_local_safe_action(&mut self, action: &LocalSafeAction) {
        self.coverage.local_safe_action_classes.insert(action.action_class);
        self.expect(
            action.record_kind == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
            "service_health_continuity.action_record_kind",
            "local_safe_action.record_kind must match the shared contract ref",
        );
        self.expect(
            action.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            "service_health_continuity.action_schema_version",
            "local_safe_action.schema_version must match the crate constant",
        );
        self.expect(
            action.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
            "service_health_continuity.action_shared_contract_ref",
            "local_safe_action.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !action.action_id.trim().is_empty(),
            "service_health_continuity.action_id_missing",
            "local_safe_action.action_id must be non-empty",
        );
        self.expect(
            !action.rationale.trim().is_empty(),
            "service_health_continuity.action_rationale_missing",
            "local_safe_action.rationale must be non-empty",
        );
        self.expect(
            !action.available_on_surfaces.is_empty(),
            "service_health_continuity.action_surfaces_missing",
            "local_safe_action must name at least one available surface",
        );
    }

    fn validate_window_states(&mut self) {
        for state in &self.page.window_state_records {
            self.coverage.window_states.insert(state.current_state);
            self.expect(
                state.record_kind == MAINTENANCE_WINDOW_STATE_RECORD_KIND,
                "service_health_continuity.state_record_kind",
                "window_state.record_kind must match maintenance_window_state_record",
            );
            self.expect(
                state.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
                "service_health_continuity.state_schema_version",
                "window_state.schema_version must match the crate constant",
            );
            self.expect(
                state.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
                "service_health_continuity.state_shared_contract_ref",
                "window_state.shared_contract_ref must match the shared contract id",
            );
            self.expect(
                !state.state_id.trim().is_empty(),
                "service_health_continuity.state_id_missing",
                "window_state.state_id must be non-empty",
            );
            self.expect(
                state.time_window.is_complete(),
                "service_health_continuity.state_time_window_incomplete",
                "window_state.time_window must have start, end, timezone, and latest_refresh",
            );
            // Resolved states have no active blocked writes by definition; all
            // other active states must name at least one blocked write class.
            if state.current_state != MaintenanceWindowState::Resolved {
                self.expect(
                    !state.blocked_writes.is_empty(),
                    "service_health_continuity.state_blocked_writes_missing",
                    "window_state must name at least one blocked write class",
                );
            }
            self.expect(
                !state.local_safe_actions.is_empty(),
                "service_health_continuity.state_local_safe_actions_missing",
                "window_state must name at least one local-safe action",
            );
            self.expect(
                !state.support_export_summary.trim().is_empty(),
                "service_health_continuity.state_summary_missing",
                "window_state.support_export_summary must be non-empty",
            );
            for disclosure in &state.blocked_writes {
                self.validate_blocked_write_disclosure(disclosure);
            }
            for action in &state.local_safe_actions {
                self.validate_local_safe_action(action);
            }
        }
    }

    fn validate_stale_rules(&mut self) {
        for rule in &self.page.stale_notice_rules {
            self.coverage.stale_downgrade_classes.insert(rule.downgrade_class);
            self.expect(
                rule.record_kind == STALE_NOTICE_DOWNGRADE_RULE_RECORD_KIND,
                "service_health_continuity.rule_record_kind",
                "stale_notice_rule.record_kind must match stale_notice_downgrade_rule_record",
            );
            self.expect(
                rule.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
                "service_health_continuity.rule_schema_version",
                "stale_notice_rule.schema_version must match the crate constant",
            );
            self.expect(
                rule.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
                "service_health_continuity.rule_shared_contract_ref",
                "stale_notice_rule.shared_contract_ref must match the shared contract id",
            );
            self.expect(
                !rule.rule_id.trim().is_empty(),
                "service_health_continuity.rule_id_missing",
                "stale_notice_rule.rule_id must be non-empty",
            );
            self.expect(
                !rule.visible_label.trim().is_empty(),
                "service_health_continuity.rule_visible_label_missing",
                "stale_notice_rule.visible_label must be non-empty",
            );
            self.expect(
                !rule.rationale.trim().is_empty(),
                "service_health_continuity.rule_rationale_missing",
                "stale_notice_rule.rationale must be non-empty",
            );
        }
    }

    fn validate_reconciliation_results(&mut self) {
        for result in &self.page.reconciliation_results {
            self.coverage
                .reconciliation_states
                .insert(result.reconciliation_state);
            self.expect(
                result.record_kind == POST_WINDOW_RECONCILIATION_RESULT_RECORD_KIND,
                "service_health_continuity.result_record_kind",
                "reconciliation_result.record_kind must match post_window_reconciliation_result_record",
            );
            self.expect(
                result.schema_version == SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
                "service_health_continuity.result_schema_version",
                "reconciliation_result.schema_version must match the crate constant",
            );
            self.expect(
                result.shared_contract_ref == SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
                "service_health_continuity.result_shared_contract_ref",
                "reconciliation_result.shared_contract_ref must match the shared contract id",
            );
            self.expect(
                !result.result_id.trim().is_empty(),
                "service_health_continuity.result_id_missing",
                "reconciliation_result.result_id must be non-empty",
            );
            self.expect(
                !result.pre_window_state_ref.trim().is_empty(),
                "service_health_continuity.result_pre_window_ref_missing",
                "reconciliation_result.pre_window_state_ref must be non-empty",
            );
            self.expect(
                !result.post_window_state_ref.trim().is_empty(),
                "service_health_continuity.result_post_window_ref_missing",
                "reconciliation_result.post_window_state_ref must be non-empty",
            );
            self.expect(
                !result.revalidation_results.is_empty(),
                "service_health_continuity.result_revalidation_missing",
                "reconciliation_result must contain at least one revalidation dimension result",
            );
            for dim_result in &result.revalidation_results {
                self.coverage
                    .revalidation_dimensions
                    .insert(dim_result.dimension);
                if dim_result.drifted {
                    self.expect(
                        dim_result
                            .drift_explanation
                            .as_deref()
                            .is_some_and(|s| !s.trim().is_empty()),
                        "service_health_continuity.result_drift_explanation_missing",
                        "drifted revalidation dimension must include a drift_explanation",
                    );
                }
            }
            self.expect(
                !result.support_export_summary.trim().is_empty(),
                "service_health_continuity.result_summary_missing",
                "reconciliation_result.support_export_summary must be non-empty",
            );
            self.expect(
                result.no_invisible_replay,
                "service_health_continuity.result_invisible_replay",
                "reconciliation_result.no_invisible_replay must be true",
            );
            if result.replay_requires_review {
                self.expect(
                    result.reconciliation_state == PostWindowReconciliationState::NeedsExplicitReview,
                    "service_health_continuity.result_review_state_mismatch",
                    "replay_requires_review=true must pair with NeedsExplicitReview state",
                );
            }
            if result.reconciliation_state == PostWindowReconciliationState::NeedsExplicitReview {
                self.expect(
                    result.replay_requires_review,
                    "service_health_continuity.result_review_flag_missing",
                    "NeedsExplicitReview state must set replay_requires_review=true",
                );
            }
        }
    }

    fn validate_required_coverage(&mut self) {
        for kind in [
            MaintenanceNoticeKind::ScheduledMaintenance,
            MaintenanceNoticeKind::ReadOnlyWindow,
            MaintenanceNoticeKind::DrainInProgress,
        ] {
            self.expect(
                self.coverage.notice_kinds.contains(&kind),
                "service_health_continuity.coverage_notice_kind_missing",
                &format!("page must cover {:?}", kind),
            );
        }
        for state in [
            MaintenanceWindowState::Scheduled,
            MaintenanceWindowState::ReadOnly,
            MaintenanceWindowState::DrainInProgress,
            MaintenanceWindowState::Resolved,
        ] {
            self.expect(
                self.coverage.window_states.contains(&state),
                "service_health_continuity.coverage_window_state_missing",
                &format!("page must cover {:?}", state),
            );
        }
        for write_class in [
            BlockedWriteClass::ProviderMutation,
            BlockedWriteClass::ReviewPublish,
        ] {
            self.expect(
                self.coverage.blocked_write_classes.contains(&write_class),
                "service_health_continuity.coverage_blocked_write_missing",
                &format!("page must cover {:?}", write_class),
            );
        }
        for action in [
            LocalSafeActionClass::LocalDraftAuthoring,
            LocalSafeActionClass::QueuePublishLater,
        ] {
            self.expect(
                self.coverage.local_safe_action_classes.contains(&action),
                "service_health_continuity.coverage_local_safe_action_missing",
                &format!("page must cover {:?}", action),
            );
        }
        for downgrade in [
            StaleNoticeDowngradeClass::Current,
            StaleNoticeDowngradeClass::ExpiredRequiresRefresh,
        ] {
            self.expect(
                self.coverage.stale_downgrade_classes.contains(&downgrade),
                "service_health_continuity.coverage_stale_downgrade_missing",
                &format!("page must cover {:?}", downgrade),
            );
        }
        for recon in [
            PostWindowReconciliationState::Queued,
            PostWindowReconciliationState::NeedsExplicitReview,
        ] {
            self.expect(
                self.coverage.reconciliation_states.contains(&recon),
                "service_health_continuity.coverage_reconciliation_state_missing",
                &format!("page must cover {:?}", recon),
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(ServiceHealthContinuityFinding {
                severity: ServiceHealthContinuityFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_time_window() -> ExactTimeWindow {
        ExactTimeWindow {
            start_time: "2026-06-10T02:00:00Z".to_string(),
            end_time: "2026-06-10T04:00:00Z".to_string(),
            timezone: "UTC".to_string(),
            latest_refresh_time: "2026-06-03T04:08:10Z".to_string(),
        }
    }

    fn sample_freshness() -> FreshnessTruth {
        FreshnessTruth {
            freshness_class: FreshnessLabel::Fresh,
            observed_at: Some("2026-06-03T04:08:10Z".to_string()),
            freshness_floor_ref: "floor:maintenance:1".to_string(),
            stale_after: None,
            degraded_reason: None,
            import_session_ref: None,
        }
    }

    fn sample_blocked_write() -> BlockedWriteDisclosure {
        BlockedWriteDisclosure {
            record_kind: BLOCKED_WRITE_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            disclosure_id: "bw-1".to_string(),
            write_class: BlockedWriteClass::ProviderMutation,
            rationale: "Provider mutations are blocked during the scheduled maintenance window.".to_string(),
            prevents_action_summary: "Prevents publishing reviews, creating issues, or requesting CI reruns.".to_string(),
            affected_surfaces: vec![AffectedSurfaceClass::ProviderSync, AffectedSurfaceClass::Review],
            local_draft_alternative_offered: true,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_blocked_write_review_publish() -> BlockedWriteDisclosure {
        BlockedWriteDisclosure {
            record_kind: BLOCKED_WRITE_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            disclosure_id: "bw-2".to_string(),
            write_class: BlockedWriteClass::ReviewPublish,
            rationale: "Review publish is blocked while provider sync is paused during the window.".to_string(),
            prevents_action_summary: "Prevents submitting review comments or approvals to the provider.".to_string(),
            affected_surfaces: vec![AffectedSurfaceClass::Review],
            local_draft_alternative_offered: true,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_local_safe_action() -> LocalSafeAction {
        LocalSafeAction {
            record_kind: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            action_id: "lsa-1".to_string(),
            action_class: LocalSafeActionClass::LocalDraftAuthoring,
            rationale: "Local drafts remain editable and are preserved for post-window publish.".to_string(),
            available_on_surfaces: vec![AffectedSurfaceClass::Desktop, AffectedSurfaceClass::Review],
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_local_safe_action_queue() -> LocalSafeAction {
        LocalSafeAction {
            record_kind: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            action_id: "lsa-2".to_string(),
            action_class: LocalSafeActionClass::QueuePublishLater,
            rationale: "Queue provider-backed mutations for automatic drain after the window closes.".to_string(),
            available_on_surfaces: vec![AffectedSurfaceClass::Desktop, AffectedSurfaceClass::Review, AffectedSurfaceClass::ProviderSync],
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_notice() -> ScheduledMaintenanceNotice {
        ScheduledMaintenanceNotice {
            record_kind: SCHEDULED_MAINTENANCE_NOTICE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            notice_id: "notice-1".to_string(),
            notice_kind: MaintenanceNoticeKind::ScheduledMaintenance,
            time_window: sample_time_window(),
            affected_surfaces: vec![
                AffectedSurfaceClass::Desktop,
                AffectedSurfaceClass::ProviderSync,
                AffectedSurfaceClass::Review,
            ],
            blocked_writes: vec![sample_blocked_write(), sample_blocked_write_review_publish()],
            local_safe_actions: vec![sample_local_safe_action(), sample_local_safe_action_queue()],
            defer_options: vec![DeferOptionClass::RemindAt, DeferOptionClass::ExportToCalendar],
            export_option_offered: true,
            freshness: sample_freshness(),
            stale_downgrade_class: StaleNoticeDowngradeClass::Current,
            support_export_summary: "Scheduled maintenance from 02:00 to 04:00 UTC.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            no_vague_copy: true,
            local_safe_action_named: true,
        }
    }

    fn sample_notice_read_only() -> ScheduledMaintenanceNotice {
        ScheduledMaintenanceNotice {
            record_kind: SCHEDULED_MAINTENANCE_NOTICE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            notice_id: "notice-2".to_string(),
            notice_kind: MaintenanceNoticeKind::ReadOnlyWindow,
            time_window: ExactTimeWindow {
                start_time: "2026-06-15T01:00:00Z".to_string(),
                end_time: "2026-06-15T01:30:00Z".to_string(),
                timezone: "UTC".to_string(),
                latest_refresh_time: "2026-06-03T04:08:10Z".to_string(),
            },
            affected_surfaces: vec![AffectedSurfaceClass::Collab, AffectedSurfaceClass::ProviderSync],
            blocked_writes: vec![sample_blocked_write()],
            local_safe_actions: vec![sample_local_safe_action()],
            defer_options: vec![DeferOptionClass::SilenceUntilStart],
            export_option_offered: false,
            freshness: sample_freshness(),
            stale_downgrade_class: StaleNoticeDowngradeClass::Current,
            support_export_summary: "Read-only window from 01:00 to 01:30 UTC on 2026-06-15.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            no_vague_copy: true,
            local_safe_action_named: true,
        }
    }

    fn sample_notice_drain() -> ScheduledMaintenanceNotice {
        ScheduledMaintenanceNotice {
            record_kind: SCHEDULED_MAINTENANCE_NOTICE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            notice_id: "notice-3".to_string(),
            notice_kind: MaintenanceNoticeKind::DrainInProgress,
            time_window: ExactTimeWindow {
                start_time: "2026-06-20T03:00:00Z".to_string(),
                end_time: "2026-06-20T03:15:00Z".to_string(),
                timezone: "UTC".to_string(),
                latest_refresh_time: "2026-06-03T04:08:10Z".to_string(),
            },
            affected_surfaces: vec![AffectedSurfaceClass::ProviderSync, AffectedSurfaceClass::UpdateCenter],
            blocked_writes: vec![sample_blocked_write()],
            local_safe_actions: vec![sample_local_safe_action()],
            defer_options: vec![DeferOptionClass::RemindAt],
            export_option_offered: false,
            freshness: sample_freshness(),
            stale_downgrade_class: StaleNoticeDowngradeClass::Current,
            support_export_summary: "Drain-in-progress window from 03:00 to 03:15 UTC on 2026-06-20.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            no_vague_copy: true,
            local_safe_action_named: true,
        }
    }

    fn sample_window_state() -> MaintenanceWindowStateRecord {
        MaintenanceWindowStateRecord {
            record_kind: MAINTENANCE_WINDOW_STATE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            state_id: "state-1".to_string(),
            current_state: MaintenanceWindowState::Scheduled,
            previous_state: None,
            time_window: sample_time_window(),
            blocked_writes: vec![sample_blocked_write()],
            local_safe_actions: vec![sample_local_safe_action()],
            next_expected_transition: Some(MaintenanceWindowState::ReadOnly),
            next_transition_time: Some("2026-06-10T02:00:00Z".to_string()),
            support_export_summary: "Window scheduled to begin at 02:00 UTC.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_stale_rule() -> StaleNoticeDowngradeRule {
        StaleNoticeDowngradeRule {
            record_kind: STALE_NOTICE_DOWNGRADE_RULE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            rule_id: "rule-1".to_string(),
            freshness_class: FreshnessLabel::ExpiredBeyondWindow,
            downgrade_class: StaleNoticeDowngradeClass::ExpiredRequiresRefresh,
            visible_label: "Maintenance notice expired — refresh for current status".to_string(),
            hide_notice: false,
            refresh_action_offered: true,
            rationale: "Expired notices must not overclaim current outage posture.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_stale_rule_current() -> StaleNoticeDowngradeRule {
        StaleNoticeDowngradeRule {
            record_kind: STALE_NOTICE_DOWNGRADE_RULE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            rule_id: "rule-2".to_string(),
            freshness_class: FreshnessLabel::Fresh,
            downgrade_class: StaleNoticeDowngradeClass::Current,
            visible_label: "Current".to_string(),
            hide_notice: false,
            refresh_action_offered: false,
            rationale: "Fresh notices are shown at full severity without downgrade.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_window_state_read_only() -> MaintenanceWindowStateRecord {
        MaintenanceWindowStateRecord {
            record_kind: MAINTENANCE_WINDOW_STATE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            state_id: "state-ro-1".to_string(),
            current_state: MaintenanceWindowState::ReadOnly,
            previous_state: Some(MaintenanceWindowState::Scheduled),
            time_window: sample_time_window(),
            blocked_writes: vec![sample_blocked_write()],
            local_safe_actions: vec![sample_local_safe_action()],
            next_expected_transition: Some(MaintenanceWindowState::DrainInProgress),
            next_transition_time: Some("2026-06-10T03:00:00Z".to_string()),
            support_export_summary: "Managed surfaces are read-only; new writes are blocked.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_window_state_drain() -> MaintenanceWindowStateRecord {
        MaintenanceWindowStateRecord {
            record_kind: MAINTENANCE_WINDOW_STATE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            state_id: "state-drain-1".to_string(),
            current_state: MaintenanceWindowState::DrainInProgress,
            previous_state: Some(MaintenanceWindowState::ReadOnly),
            time_window: sample_time_window(),
            blocked_writes: vec![sample_blocked_write()],
            local_safe_actions: vec![sample_local_safe_action()],
            next_expected_transition: Some(MaintenanceWindowState::Reconciling),
            next_transition_time: Some("2026-06-10T03:30:00Z".to_string()),
            support_export_summary: "Drain in progress; new high-risk writes are blocked.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_window_state_resolved() -> MaintenanceWindowStateRecord {
        MaintenanceWindowStateRecord {
            record_kind: MAINTENANCE_WINDOW_STATE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            state_id: "state-resolved-1".to_string(),
            current_state: MaintenanceWindowState::Resolved,
            previous_state: Some(MaintenanceWindowState::Reconciling),
            time_window: sample_time_window(),
            blocked_writes: vec![],
            local_safe_actions: vec![sample_local_safe_action()],
            next_expected_transition: None,
            next_transition_time: None,
            support_export_summary: "Maintenance window resolved; normal operation resumed.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }

    fn sample_reconciliation_result() -> PostWindowReconciliationResult {
        PostWindowReconciliationResult {
            record_kind: POST_WINDOW_RECONCILIATION_RESULT_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            result_id: "recon-1".to_string(),
            pre_window_state_ref: "state-1".to_string(),
            post_window_state_ref: "state-2".to_string(),
            revalidation_results: vec![
                RevalidationDimensionResult {
                    dimension: RevalidationDimension::Tenant,
                    pre_window_value: "tenant-a".to_string(),
                    post_window_value: "tenant-a".to_string(),
                    drifted: false,
                    drift_explanation: None,
                },
                RevalidationDimensionResult {
                    dimension: RevalidationDimension::Endpoint,
                    pre_window_value: "https://api.old.example.com".to_string(),
                    post_window_value: "https://api.new.example.com".to_string(),
                    drifted: true,
                    drift_explanation: Some("Endpoint changed during failover.".to_string()),
                },
            ],
            drifted_dimensions: vec![RevalidationDimension::Endpoint],
            reconciliation_state: PostWindowReconciliationState::NeedsExplicitReview,
            queued_intent_refs: vec!["queue-item-1".to_string()],
            replay_requires_review: true,
            provider_drift_class: ProviderDriftClass::TargetIdentityChanged,
            next_safe_action: ReconciliationNextActionClass::CompareRebaseReview,
            support_export_summary: "Endpoint drifted during window; explicit review required.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            no_invisible_replay: true,
        }
    }

    fn sample_reconciliation_queued() -> PostWindowReconciliationResult {
        PostWindowReconciliationResult {
            record_kind: POST_WINDOW_RECONCILIATION_RESULT_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            result_id: "recon-2".to_string(),
            pre_window_state_ref: "state-1".to_string(),
            post_window_state_ref: "state-resolved-1".to_string(),
            revalidation_results: vec![RevalidationDimensionResult {
                dimension: RevalidationDimension::Region,
                pre_window_value: "us-east-1".to_string(),
                post_window_value: "us-east-1".to_string(),
                drifted: false,
                drift_explanation: None,
            }],
            drifted_dimensions: vec![],
            reconciliation_state: PostWindowReconciliationState::Queued,
            queued_intent_refs: vec!["queue-item-2".to_string()],
            replay_requires_review: false,
            provider_drift_class: ProviderDriftClass::NoMaterialDrift,
            next_safe_action: ReconciliationNextActionClass::MutateProviderNow,
            support_export_summary: "No drift observed; queued intent may proceed after window.".to_string(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            no_invisible_replay: true,
        }
    }

    fn sample_page() -> ServiceHealthContinuityPage {
        ServiceHealthContinuityPage {
            fixture_metadata: None,
            record_kind: SERVICE_HEALTH_CONTINUITY_PAGE_RECORD_KIND.to_string(),
            schema_version: SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF.to_string(),
            page_id: "page-1".to_string(),
            contract_refs: ServiceHealthContinuityContractRefs {
                publish_later_queue_schema_ref: "schema:publish_later:v1".to_string(),
                deferred_publish_queue_schema_ref: "schema:deferred_publish:v1".to_string(),
                provider_event_reconciliation_schema_ref: "schema:reconciliation:v1".to_string(),
                provider_object_model_schema_ref: "schema:object_model:v1".to_string(),
                settings_sync_state_schema_ref: "schema:settings_sync:v1".to_string(),
                backup_restore_failover_schema_ref: "schema:backup_restore:v1".to_string(),
            },
            maintenance_notices: vec![
                sample_notice(),
                sample_notice_read_only(),
                sample_notice_drain(),
            ],
            window_state_records: vec![
                sample_window_state(),
                sample_window_state_read_only(),
                sample_window_state_drain(),
                sample_window_state_resolved(),
            ],
            stale_notice_rules: vec![sample_stale_rule(), sample_stale_rule_current()],
            reconciliation_results: vec![
                sample_reconciliation_result(),
                sample_reconciliation_queued(),
            ],
            support_export_summary: "Service-health continuity page for M4 stable line.".to_string(),
        }
    }

    #[test]
    fn page_validates_successfully() {
        let page = sample_page();
        let report = page.validate();
        assert!(report.passed, "validation failed: {:?}", report.findings);
    }

    #[test]
    fn notice_requires_exact_time() {
        let mut page = sample_page();
        page.maintenance_notices[0].time_window.start_time.clear();
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "service_health_continuity.notice_time_window_incomplete"));
    }

    #[test]
    fn notice_rejects_vague_copy() {
        let mut page = sample_page();
        page.maintenance_notices[0].no_vague_copy = false;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "service_health_continuity.notice_vague_copy_present"));
    }

    #[test]
    fn stale_rule_requires_visible_label() {
        let mut page = sample_page();
        page.stale_notice_rules[0].visible_label.clear();
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "service_health_continuity.rule_visible_label_missing"));
    }

    #[test]
    fn reconciliation_blocks_invisible_replay() {
        let mut page = sample_page();
        page.reconciliation_results[0].no_invisible_replay = false;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "service_health_continuity.result_invisible_replay"));
    }

    #[test]
    fn reconciliation_review_state_coherence() {
        let mut page = sample_page();
        page.reconciliation_results[0].replay_requires_review = false;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "service_health_continuity.result_review_flag_missing"));
    }

    #[test]
    fn support_export_projection_builds() {
        let page = sample_page();
        let export = page.support_export_projection();
        assert_eq!(export.page_id, page.page_id);
        assert_eq!(export.notice_summaries.len(), page.maintenance_notices.len());
        assert_eq!(
            export.window_state_summaries.len(),
            page.window_state_records.len()
        );
        assert_eq!(
            export.stale_rule_summaries.len(),
            page.stale_notice_rules.len()
        );
        assert_eq!(
            export.reconciliation_summaries.len(),
            page.reconciliation_results.len()
        );
    }

    #[test]
    fn drifted_dimension_requires_explanation() {
        let mut page = sample_page();
        page.reconciliation_results[0].revalidation_results[1].drift_explanation = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "service_health_continuity.result_drift_explanation_missing"));
    }
}
