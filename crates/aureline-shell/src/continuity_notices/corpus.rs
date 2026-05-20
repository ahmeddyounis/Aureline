//! Cross-surface maintenance, drain, failover, and tenant-migration
//! continuity-notice drill corpus.
//!
//! ## Why a corpus, not a single seeded view
//!
//! The model in [`crate::continuity_notices::model`] proves the
//! no-silent-current and boundary-preserved invariants in isolation, but the
//! beta-claim grade is about the shell, the activity center / durable history,
//! CLI / headless inspect, diagnostics, and support exports *agreeing* on what a
//! maintenance, drain, failover, or migration window means for the same notice.
//! This corpus mints one [`ContinuityNoticeScenario`] per named drill and pins
//! each rendered [`ContinuityNoticeView`] bit-for-bit on disk under
//! `fixtures/ops/m3/maintenance_and_failover_notices/`, so a regression in the
//! freshness-downgrade rule, the queued-publish-later preservation, the
//! boundary-change derivation, or the display-copy invariants fails the
//! fixture-replay test instead of shipping silently.
//!
//! The drills deliberately exercise every [`NoticeKindClass`], every
//! [`FreshnessClass`] and [`EffectiveFreshnessClass`], every
//! [`WriteContinuityPostureClass`], every [`SaferThanRetryGuidanceClass`],
//! every [`BoundaryAxisStateClass`], and every [`DowngradeReasonClass`].

use super::model::{
    BlockStateClass, BlockedWriteInput, BoundaryAxisClass, BoundaryAxisInput,
    BoundaryAxisStateClass, BoundaryChangeInput, ContinuityNoticeInput, ContinuityNoticeView,
    DeploymentProfileClass, EffectiveFreshnessClass, FreshnessClass, HostedMutationInput,
    LifecycleInput, LocalContinuityInput, LocalCoreStatusClass, ManagedActionClass,
    NoticeCategoryClass, NoticeKindClass, ResidencyScopeClass, ResumeTriggerClass,
    SaferThanRetryGuidanceClass, ScheduleInput, ScopeInput, ServiceClass, TimeBasisClass,
    WriteContinuityPostureClass,
};

/// Stable `as_of` instant the whole corpus is evaluated against. Pinned so the
/// on-disk fixtures stay deterministic.
pub const CORPUS_AS_OF: &str = "2026-05-20T12:00:00Z";

/// Stable view-id prefix shared by every scenario.
pub const CORPUS_VIEW_ID_PREFIX: &str = "continuity_notice_view:m3.beta.corpus.";

// Pinned refresh instants relative to CORPUS_AS_OF (12:00).
const REFRESH_FRESH: &str = "2026-05-20T11:58:00Z"; // 2 min  -> fresh
const REFRESH_RECENT: &str = "2026-05-20T11:20:00Z"; // 40 min -> recent
const REFRESH_STALE: &str = "2026-05-20T06:00:00Z"; // 6 h    -> stale
const REFRESH_VERY_STALE: &str = "2026-05-19T06:00:00Z"; // >24 h -> very_stale

/// One drill. Surfaces under review MUST reproduce the same view projection
/// bit-for-bit; the test in
/// `crates/aureline-shell/tests/continuity_notices_fixtures.rs` pins each
/// scenario against the on-disk fixture under
/// `fixtures/ops/m3/maintenance_and_failover_notices/`.
#[derive(Clone)]
pub struct ContinuityNoticeScenario {
    /// Stable identifier, quoted in the matrix, the report, and the doc.
    pub scenario_id: &'static str,
    /// Stable human-readable label.
    pub scenario_label: &'static str,
    /// One-sentence narrative the report and matrix quote.
    pub narrative: &'static str,
    /// On-disk fixture filename (relative to the corpus fixture dir).
    pub fixture_filename: &'static str,
    /// Expected coarse category.
    pub expected_category: NoticeCategoryClass,
    /// Expected derived effective freshness.
    pub expected_effective_freshness: EffectiveFreshnessClass,
    /// Expected honesty-marker value.
    pub expected_honesty_marker_present: bool,
    /// Expected count of preserved (queued + local-draft) intents.
    pub expected_preserved_intent_count: u32,
    /// Expected count of meaningfully changed boundary axes.
    pub expected_changed_boundary_axis_count: u32,
    /// Expected boundary-change-unresolved value.
    pub expected_boundary_change_unresolved: bool,
    input: ContinuityNoticeInput,
}

impl ContinuityNoticeScenario {
    /// Build the rendered view for this scenario. The corpus inputs are
    /// deterministic and validated, so a build failure is a bug.
    pub fn view(&self) -> ContinuityNoticeView {
        ContinuityNoticeView::build(self.input.clone(), CORPUS_AS_OF)
            .expect("continuity-notice corpus scenario must build")
    }
}

// --------------------------------------------------------------------------- //
// Compact constructors
// --------------------------------------------------------------------------- //

#[allow(clippy::too_many_arguments)]
fn sched(
    basis: TimeBasisClass,
    starts: &str,
    ends: Option<&str>,
    completed: Option<&str>,
    tz: &str,
    offset: &str,
    refresh: Option<&str>,
) -> ScheduleInput {
    ScheduleInput {
        time_basis: basis,
        starts_at: starts.to_owned(),
        expected_or_actual_ends_at: ends.map(str::to_owned),
        completed_at: completed.map(str::to_owned),
        timezone_id: tz.to_owned(),
        utc_offset_at_start: offset.to_owned(),
        latest_refresh_at: refresh.map(str::to_owned),
    }
}

fn scope(
    profiles: Vec<DeploymentProfileClass>,
    tenants: &[&str],
    regions: &[&str],
    residency: Vec<ResidencyScopeClass>,
    services: Vec<ServiceClass>,
    summary: &str,
) -> ScopeInput {
    ScopeInput {
        deployment_profiles: profiles,
        tenant_refs: tenants.iter().map(|s| (*s).to_owned()).collect(),
        region_refs: regions.iter().map(|s| (*s).to_owned()).collect(),
        residency_scope_classes: residency,
        service_classes: services,
        scope_summary: summary.to_owned(),
    }
}

fn axis(
    class: BoundaryAxisClass,
    state: BoundaryAxisStateClass,
    previous: Option<&str>,
    current: Option<&str>,
    summary: &str,
) -> BoundaryAxisInput {
    BoundaryAxisInput {
        axis_class: class,
        axis_state_class: state,
        previous_ref: previous.map(str::to_owned),
        current_ref: current.map(str::to_owned),
        summary: summary.to_owned(),
    }
}

fn boundary(
    required: bool,
    reviewed: bool,
    axes: Vec<BoundaryAxisInput>,
    summary: &str,
) -> BoundaryChangeInput {
    BoundaryChangeInput {
        boundary_change_required: required,
        review_completed: reviewed,
        axes,
        summary: summary.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn blocked(
    action: ManagedActionClass,
    block: BlockStateClass,
    posture: WriteContinuityPostureClass,
    guidance: SaferThanRetryGuidanceClass,
    queue_ref: Option<&str>,
    idempotent: bool,
    resume: ResumeTriggerClass,
    note: &str,
) -> BlockedWriteInput {
    BlockedWriteInput {
        action_class: action,
        block_state_class: block,
        continuity_posture: posture,
        safer_guidance: guidance,
        queue_or_intent_ref: queue_ref.map(str::to_owned),
        idempotency_key_present: idempotent,
        resume_trigger: resume,
        note: note.to_owned(),
    }
}

fn succeeded(action: ManagedActionClass, result: &str, at: &str, note: &str) -> HostedMutationInput {
    HostedMutationInput {
        action_class: action,
        result_ref: result.to_owned(),
        completed_at: at.to_owned(),
        note: note.to_owned(),
    }
}

fn local(
    status: LocalCoreStatusClass,
    caps: &[&str],
    guidance_required: bool,
    summary: &str,
) -> LocalContinuityInput {
    LocalContinuityInput {
        local_core_status: status,
        retained_local_safe_capabilities: caps.iter().map(|s| (*s).to_owned()).collect(),
        continue_local_guidance_required: guidance_required,
        continuity_summary: summary.to_owned(),
    }
}

fn lifecycle(
    freshness: FreshnessClass,
    supersedes: Option<&str>,
    superseded_by: Option<&str>,
    retained_until: Option<&str>,
    history: &[&str],
) -> LifecycleInput {
    LifecycleInput {
        freshness_class: freshness,
        supersedes_id: supersedes.map(str::to_owned),
        superseded_by_id: superseded_by.map(str::to_owned),
        retained_until_at: retained_until.map(str::to_owned),
        history_refs: history.iter().map(|s| (*s).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn input(
    scenario_id: &str,
    notice_id: &str,
    kind: NoticeKindClass,
    title: &str,
    summary: &str,
    schedule: ScheduleInput,
    affected_scope: ScopeInput,
    boundary_change: BoundaryChangeInput,
    blocked_writes: Vec<BlockedWriteInput>,
    succeeded_hosted_mutations: Vec<HostedMutationInput>,
    local_continuity: LocalContinuityInput,
    lifecycle: LifecycleInput,
) -> ContinuityNoticeInput {
    ContinuityNoticeInput {
        view_id: format!("{CORPUS_VIEW_ID_PREFIX}{scenario_id}"),
        notice_id: notice_id.to_owned(),
        notice_kind: kind,
        title: title.to_owned(),
        summary: summary.to_owned(),
        created_at: "2026-05-20T08:00:00Z".to_owned(),
        updated_at: REFRESH_FRESH.to_owned(),
        schedule,
        affected_scope,
        boundary_change,
        blocked_writes,
        succeeded_hosted_mutations,
        local_continuity,
        lifecycle,
        history_ref: format!("aureline://continuity_notice_history/{notice_id}"),
        support_export_ref: format!("aureline://support_export/{notice_id}"),
        evidence_refs: vec![format!("evidence.continuity.{scenario_id}")],
        narrative_refs: vec!["docs/ops/m3/maintenance_failover_truth.md".to_owned()],
    }
}

// --------------------------------------------------------------------------- //
// The corpus
// --------------------------------------------------------------------------- //

/// The full deterministic continuity-notice drill corpus.
pub fn continuity_notice_corpus() -> Vec<ContinuityNoticeScenario> {
    vec![
        scheduled_maintenance_window(),
        read_only_window_publish_later(),
        drain_before_failover(),
        scheduled_export_freeze(),
        regional_failover_changed_boundary(),
        tenant_migration_new_region(),
        control_plane_failover(),
        region_migration_reconciling(),
        post_event_reconciliation_completed(),
        superseded_notice_downgraded(),
        imported_offline_history(),
        stale_refresh_active_downgraded(),
    ]
}

fn scheduled_maintenance_window() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "scheduled_maintenance_window",
        scenario_label: "Scheduled maintenance window with publish-later queued",
        narrative: "A planned full maintenance window declares its exact start, timezone, and \
                    offset; review-comment publishes queue for replay and a settings-sync write \
                    is retryable when the window ends.",
        fixture_filename: "scheduled_maintenance_window.json",
        expected_category: NoticeCategoryClass::Maintenance,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: false,
        expected_preserved_intent_count: 1,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: false,
        input: input(
            "scheduled_maintenance_window",
            "notice.maintenance.window",
            NoticeKindClass::ScheduledMaintenanceWindow,
            "Sync service maintenance window",
            "Sync service pauses for scheduled maintenance; reads and local work continue.",
            sched(
                TimeBasisClass::ScheduledExact,
                "2026-05-20T12:00:00Z",
                Some("2026-05-20T13:00:00Z"),
                None,
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![
                    DeploymentProfileClass::ManagedCloud,
                    DeploymentProfileClass::EnterpriseOnline,
                ],
                &["tenant.ref.acme"],
                &["region.ref.eu-central"],
                vec![ResidencyScopeClass::CustomerRegionPinned],
                vec![ServiceClass::SyncService, ServiceClass::RegistryService],
                "Managed-cloud and enterprise sync in the EU-central region.",
            ),
            boundary(
                false,
                false,
                vec![
                    axis(
                        BoundaryAxisClass::Tenant,
                        BoundaryAxisStateClass::Unchanged,
                        None,
                        None,
                        "Tenant boundary unchanged.",
                    ),
                    axis(
                        BoundaryAxisClass::Residency,
                        BoundaryAxisStateClass::NotApplicable,
                        None,
                        None,
                        "Residency not affected by this window.",
                    ),
                ],
                "No tenant, region, or endpoint boundary change.",
            ),
            vec![
                blocked(
                    ManagedActionClass::ManagedReviewCommentPublish,
                    BlockStateClass::ScheduledToBlock,
                    WriteContinuityPostureClass::QueuedPublishLater,
                    SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                    Some("aureline://publish_later_queue/maintenance.comments"),
                    true,
                    ResumeTriggerClass::WindowEnds,
                    "Review comments queue for publish-later and replay when the window ends.",
                ),
                blocked(
                    ManagedActionClass::ProfileSettingsSyncWrite,
                    BlockStateClass::ScheduledToBlock,
                    WriteContinuityPostureClass::RetryableWhenConnected,
                    SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                    None,
                    false,
                    ResumeTriggerClass::WindowEnds,
                    "Settings sync is retryable as soon as the window ends.",
                ),
            ],
            vec![succeeded(
                ManagedActionClass::ManagedReviewApproval,
                "aureline://change_review/cr-4100",
                "2026-05-20T11:50:00Z",
                "Approval landed before the window opened.",
            )],
            local(
                LocalCoreStatusClass::LocalCoreUnaffected,
                &[
                    "Editing, saving, and local search continue.",
                    "Local Git commit and branch continue.",
                ],
                true,
                "All local-first work continues; only managed sync pauses.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn read_only_window_publish_later() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "read_only_window_publish_later",
        scenario_label: "Read-only window preserves queued and local-draft work",
        narrative: "A read-only window blocks hosted writes; review-comment publishes queue for \
                    replay and a provider publish is captured as a local draft, both visibly \
                    separated from a hosted mutation that already landed.",
        fixture_filename: "read_only_window_publish_later.json",
        expected_category: NoticeCategoryClass::Drain,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: false,
        expected_preserved_intent_count: 2,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: false,
        input: input(
            "read_only_window_publish_later",
            "notice.read_only.window",
            NoticeKindClass::ReadOnlyWindow,
            "Provider review read-only window",
            "Hosted writes pause; reads, local edits, and queued work continue.",
            sched(
                TimeBasisClass::InProgressExact,
                "2026-05-20T11:30:00Z",
                Some("2026-05-20T12:30:00Z"),
                None,
                "America/New_York",
                "-04:00",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme"],
                &["region.ref.us-east"],
                vec![ResidencyScopeClass::VendorRegionDefault],
                vec![
                    ServiceClass::ProviderReviewService,
                    ServiceClass::MergeQueueService,
                ],
                "Managed-cloud provider review and merge queue in US-east.",
            ),
            boundary(
                false,
                false,
                vec![axis(
                    BoundaryAxisClass::Tenant,
                    BoundaryAxisStateClass::Unchanged,
                    None,
                    None,
                    "Tenant boundary unchanged.",
                )],
                "No boundary change during the read-only window.",
            ),
            vec![
                blocked(
                    ManagedActionClass::ManagedReviewCommentPublish,
                    BlockStateClass::BlockedReadOnly,
                    WriteContinuityPostureClass::QueuedPublishLater,
                    SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                    Some("aureline://publish_later_queue/ro.comments"),
                    true,
                    ResumeTriggerClass::WindowEnds,
                    "Comments queue for publish-later; replay is idempotent.",
                ),
                blocked(
                    ManagedActionClass::ProviderPublishLocalDraft,
                    BlockStateClass::BlockedReadOnly,
                    WriteContinuityPostureClass::LocalDraftPreserved,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    Some("aureline://local_draft/ro.publish"),
                    false,
                    ResumeTriggerClass::WindowEnds,
                    "Provider publish is captured as a local draft to publish after the window.",
                ),
            ],
            vec![succeeded(
                ManagedActionClass::MergeQueueEnqueue,
                "aureline://merge_queue/mq-2210",
                "2026-05-20T11:25:00Z",
                "Merge enqueue landed before the read-only window.",
            )],
            local(
                LocalCoreStatusClass::MeaningfulSafeSubsetAvailable,
                &[
                    "Editing and saving continue.",
                    "Cached provider snapshots remain inspectable with a freshness label.",
                ],
                true,
                "Local edits and cached reads continue; hosted writes wait for the window to end.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn drain_before_failover() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "drain_before_failover",
        scenario_label: "Drain window ahead of a regional failover",
        narrative: "A planned drain lets existing collaboration sessions finish while new writes \
                    queue; the region is flagged unknown-recheck so the next publish needs a \
                    boundary review, and postponing is safer than retrying into the drain.",
        fixture_filename: "drain_before_failover.json",
        expected_category: NoticeCategoryClass::Drain,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 1,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: true,
        input: input(
            "drain_before_failover",
            "notice.drain.before_failover",
            NoticeKindClass::DrainWindow,
            "Collaboration relay drain ahead of failover",
            "Existing collaboration sessions finish; new writes queue while the relay drains.",
            sched(
                TimeBasisClass::InProgressExact,
                "2026-05-20T11:45:00Z",
                Some("2026-05-20T12:15:00Z"),
                None,
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme"],
                &["region.ref.eu-central"],
                vec![ResidencyScopeClass::CustomerRegionPinned],
                vec![ServiceClass::RelayService],
                "Managed-cloud collaboration relay in EU-central, draining before failover.",
            ),
            boundary(
                true,
                false,
                vec![axis(
                    BoundaryAxisClass::Region,
                    BoundaryAxisStateClass::UnknownRecheckRequired,
                    Some("region.ref.eu-central"),
                    Some("aureline://region/eu-failover-pending"),
                    "Region may move on failover; recheck before the next publish.",
                )],
                "Region boundary may change on failover; recheck required.",
            ),
            vec![
                blocked(
                    ManagedActionClass::CollaborationPresenceWrite,
                    BlockStateClass::DrainingExistingOnly,
                    WriteContinuityPostureClass::DrainingExistingOnly,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    None,
                    false,
                    ResumeTriggerClass::DrainCompletes,
                    "Existing presence writes finish; new ones wait for the drain to complete.",
                ),
                blocked(
                    ManagedActionClass::MergeQueueEnqueue,
                    BlockStateClass::BlockedDrainNewActions,
                    WriteContinuityPostureClass::QueuedPublishLater,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    Some("aureline://publish_later_queue/drain.merge"),
                    true,
                    ResumeTriggerClass::DrainCompletes,
                    "New merge enqueues queue for replay after the drain.",
                ),
            ],
            vec![],
            local(
                LocalCoreStatusClass::MeaningfulSafeSubsetAvailable,
                &["Editing, saving, and local Git continue."],
                true,
                "Local work continues; the relay drains existing sessions before failover.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn scheduled_export_freeze() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "scheduled_export_freeze",
        scenario_label: "Export-freeze window where export-now is safer than retry",
        narrative: "A pre-operation export freeze blocks support-bundle uploads with no safe \
                    retry, so exporting now is safer; a telemetry upload stays retryable.",
        fixture_filename: "scheduled_export_freeze.json",
        expected_category: NoticeCategoryClass::Maintenance,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: false,
        expected_preserved_intent_count: 0,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: false,
        input: input(
            "scheduled_export_freeze",
            "notice.export_freeze",
            NoticeKindClass::ScheduledExportFreeze,
            "Support ingest export freeze",
            "Support-bundle uploads freeze before the migration; export locally now if needed.",
            sched(
                TimeBasisClass::ScheduledExact,
                "2026-05-20T12:30:00Z",
                Some("2026-05-20T14:00:00Z"),
                None,
                "UTC",
                "Z",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![DeploymentProfileClass::EnterpriseOnline],
                &["tenant.ref.acme"],
                &["region.ref.eu-central"],
                vec![ResidencyScopeClass::SovereignRegionPinned],
                vec![ServiceClass::SupportIngestService],
                "Enterprise support ingest in a sovereign region.",
            ),
            boundary(
                false,
                false,
                vec![axis(
                    BoundaryAxisClass::Residency,
                    BoundaryAxisStateClass::Unchanged,
                    None,
                    None,
                    "Residency unchanged during the freeze.",
                )],
                "No boundary change during the export freeze.",
            ),
            vec![
                blocked(
                    ManagedActionClass::SupportBundleUpload,
                    BlockStateClass::ScheduledToBlock,
                    WriteContinuityPostureClass::BlockedNoSafeRetry,
                    SaferThanRetryGuidanceClass::ExportNowSafer,
                    None,
                    false,
                    ResumeTriggerClass::ManualExportRequired,
                    "Support uploads cannot queue across the freeze; export the bundle locally now.",
                ),
                blocked(
                    ManagedActionClass::TelemetryUpload,
                    BlockStateClass::ScheduledToBlock,
                    WriteContinuityPostureClass::RetryableWhenConnected,
                    SaferThanRetryGuidanceClass::NoSafeRetryEscalate,
                    None,
                    false,
                    ResumeTriggerClass::WindowEnds,
                    "Telemetry retries after the freeze; escalate if it must land sooner.",
                ),
            ],
            vec![],
            local(
                LocalCoreStatusClass::LocalCoreUnaffected,
                &[
                    "Editing and saving continue.",
                    "Local diagnostics export continues.",
                ],
                true,
                "Local work and local export continue; only hosted uploads freeze.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn regional_failover_changed_boundary() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "regional_failover_changed_boundary",
        scenario_label: "Emergency regional failover with a changed region/endpoint boundary",
        narrative: "An emergency failover moved the region and endpoint identity; publishes are \
                    blocked pending a boundary recheck and an in-flight write needs a manual \
                    rerun. The changed identity stays visible and the notice cannot read as \
                    routine.",
        fixture_filename: "regional_failover_changed_boundary.json",
        expected_category: NoticeCategoryClass::Failover,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 0,
        expected_changed_boundary_axis_count: 2,
        expected_boundary_change_unresolved: true,
        input: input(
            "regional_failover_changed_boundary",
            "notice.failover.regional",
            NoticeKindClass::RegionalFailover,
            "Regional failover to the standby region",
            "Managed writes failed over to the standby region; the region and endpoint changed.",
            sched(
                TimeBasisClass::DetectedExact,
                "2026-05-20T11:40:00Z",
                None,
                None,
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme"],
                &["region.ref.eu-central", "region.ref.eu-west"],
                vec![ResidencyScopeClass::CustomerRegionPinned],
                vec![
                    ServiceClass::WorkspaceControlPlaneService,
                    ServiceClass::ProviderReviewService,
                ],
                "Managed-cloud control plane failed over from EU-central to EU-west.",
            ),
            boundary(
                true,
                false,
                vec![
                    axis(
                        BoundaryAxisClass::Region,
                        BoundaryAxisStateClass::Changed,
                        Some("region.ref.eu-central"),
                        Some("aureline://region/eu-west"),
                        "Region moved to the standby region.",
                    ),
                    axis(
                        BoundaryAxisClass::EndpointIdentity,
                        BoundaryAxisStateClass::Changed,
                        Some("endpoint.ref.eu-central"),
                        Some("aureline://endpoint/eu-west"),
                        "Endpoint identity changed with the failover.",
                    ),
                ],
                "Region and endpoint identity changed; review before the next publish.",
            ),
            vec![
                blocked(
                    ManagedActionClass::ProviderPublishImmediate,
                    BlockStateClass::BlockedPendingBoundaryRecheck,
                    WriteContinuityPostureClass::BlockedPendingBoundaryRecheck,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    None,
                    false,
                    ResumeTriggerClass::BoundaryReviewCompleted,
                    "Immediate publishes wait until the new region boundary is reviewed.",
                ),
                blocked(
                    ManagedActionClass::ManagedWorkspaceLifecycleWrite,
                    BlockStateClass::BlockedPendingBoundaryRecheck,
                    WriteContinuityPostureClass::RequiresManualRerun,
                    SaferThanRetryGuidanceClass::ManualRerunRequired,
                    None,
                    false,
                    ResumeTriggerClass::FreshApprovalIssued,
                    "A lifecycle write in flight at cutover needs a manual rerun after review.",
                ),
            ],
            vec![],
            local(
                LocalCoreStatusClass::MeaningfulSafeSubsetAvailable,
                &[
                    "Editing, saving, and local search continue.",
                    "Cached reads remain inspectable with a freshness label.",
                ],
                true,
                "Local work continues; managed writes wait for the new-region boundary review.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn tenant_migration_new_region() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "tenant_migration_new_region",
        scenario_label: "Tenant migration to a new region with residency change",
        narrative: "A planned tenant migration moves the tenant, region, and residency; a \
                    provider publish is captured as a local draft while publishes wait for the \
                    new-boundary review.",
        fixture_filename: "tenant_migration_new_region.json",
        expected_category: NoticeCategoryClass::TenantMigration,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 1,
        expected_changed_boundary_axis_count: 3,
        expected_boundary_change_unresolved: true,
        input: input(
            "tenant_migration_new_region",
            "notice.migration.tenant",
            NoticeKindClass::TenantMigration,
            "Tenant migration to the EU sovereign region",
            "The tenant migrates to a new region and residency; review the new boundary before publishing.",
            sched(
                TimeBasisClass::InProgressExact,
                "2026-05-20T11:50:00Z",
                Some("2026-05-20T12:50:00Z"),
                None,
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme", "tenant.ref.acme-eu"],
                &["region.ref.us-east", "region.ref.eu-central"],
                vec![ResidencyScopeClass::ResidencyChangedReviewRequired],
                vec![
                    ServiceClass::WorkspaceControlPlaneService,
                    ServiceClass::PolicyService,
                ],
                "Managed-cloud tenant migrating from US-east to the EU sovereign region.",
            ),
            boundary(
                true,
                false,
                vec![
                    axis(
                        BoundaryAxisClass::Tenant,
                        BoundaryAxisStateClass::Changed,
                        Some("tenant.ref.acme"),
                        Some("aureline://tenant/acme-eu"),
                        "Tenant identity moved to the EU tenant.",
                    ),
                    axis(
                        BoundaryAxisClass::Region,
                        BoundaryAxisStateClass::Changed,
                        Some("region.ref.us-east"),
                        Some("aureline://region/eu-central"),
                        "Region moved to EU-central.",
                    ),
                    axis(
                        BoundaryAxisClass::Residency,
                        BoundaryAxisStateClass::Changed,
                        Some("residency.ref.vendor-default"),
                        Some("aureline://residency/eu-sovereign"),
                        "Residency moved to the EU sovereign scope.",
                    ),
                ],
                "Tenant, region, and residency changed; review before the next publish.",
            ),
            vec![
                blocked(
                    ManagedActionClass::ProviderPublishLocalDraft,
                    BlockStateClass::BlockedPendingBoundaryRecheck,
                    WriteContinuityPostureClass::LocalDraftPreserved,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    Some("aureline://local_draft/migration.publish"),
                    false,
                    ResumeTriggerClass::BoundaryReviewCompleted,
                    "Provider publish is held as a local draft until the new boundary is reviewed.",
                ),
                blocked(
                    ManagedActionClass::PolicyAdminWrite,
                    BlockStateClass::BlockedPendingBoundaryRecheck,
                    WriteContinuityPostureClass::BlockedPendingBoundaryRecheck,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    None,
                    false,
                    ResumeTriggerClass::FreshApprovalIssued,
                    "Policy admin writes wait for a fresh approval under the new tenant.",
                ),
            ],
            vec![succeeded(
                ManagedActionClass::ProfileSettingsSyncWrite,
                "aureline://settings_profile/sp-77",
                "2026-05-20T11:48:00Z",
                "Settings sync replicated to the new tenant before cutover.",
            )],
            local(
                LocalCoreStatusClass::MeaningfulSafeSubsetAvailable,
                &[
                    "Editing, saving, and local search continue.",
                    "Local export of the workspace continues.",
                ],
                true,
                "Local work continues; managed publishes wait for the new-boundary review.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn control_plane_failover() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "control_plane_failover",
        scenario_label: "Control-plane failover with key-ownership recheck",
        narrative: "An emergency control-plane failover leaves key ownership unknown pending a \
                    recheck; remote sessions are blocked pending reconnect and an AI prompt is \
                    retryable. The last refresh is recent so the notice still reads as current.",
        fixture_filename: "control_plane_failover.json",
        expected_category: NoticeCategoryClass::Failover,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 0,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: true,
        input: input(
            "control_plane_failover",
            "notice.failover.control_plane",
            NoticeKindClass::ControlPlaneFailover,
            "Control-plane failover to the standby plane",
            "The control plane failed over; key ownership needs a recheck before policy writes.",
            sched(
                TimeBasisClass::DetectedExact,
                "2026-05-20T11:35:00Z",
                None,
                None,
                "UTC",
                "Z",
                Some(REFRESH_RECENT),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme"],
                &["region.ref.eu-central"],
                vec![ResidencyScopeClass::TenantResidencyRecheckRequired],
                vec![
                    ServiceClass::AuthIdentityService,
                    ServiceClass::PolicyService,
                ],
                "Managed-cloud auth/policy control plane failed over to standby.",
            ),
            boundary(
                true,
                false,
                vec![axis(
                    BoundaryAxisClass::KeyOwnership,
                    BoundaryAxisStateClass::UnknownRecheckRequired,
                    Some("key.ref.primary"),
                    Some("aureline://key_ownership/standby-pending"),
                    "Key ownership unknown after failover; recheck before policy writes.",
                )],
                "Key ownership requires a recheck after the control-plane failover.",
            ),
            vec![
                blocked(
                    ManagedActionClass::RemoteControlSessionJoin,
                    BlockStateClass::BlockedPendingReconnect,
                    WriteContinuityPostureClass::BlockedPendingReconnect,
                    SaferThanRetryGuidanceClass::PostponeSafer,
                    None,
                    false,
                    ResumeTriggerClass::ManualReconnectRequired,
                    "Remote sessions must reconnect to the standby plane.",
                ),
                blocked(
                    ManagedActionClass::AiGatewayPromptSubmit,
                    BlockStateClass::BlockedPendingReconnect,
                    WriteContinuityPostureClass::RetryableWhenConnected,
                    SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                    None,
                    false,
                    ResumeTriggerClass::ReconciliationCompletes,
                    "AI gateway prompts retry once the standby plane is reachable.",
                ),
            ],
            vec![],
            local(
                LocalCoreStatusClass::LocalOnlyAvailable,
                &["Editing, saving, and local Git continue offline."],
                true,
                "Local-only work continues; managed control-plane actions wait for reconnect.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn region_migration_reconciling() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "region_migration_reconciling",
        scenario_label: "Region migration reconciling with a queued replay",
        narrative: "A region migration is reconciling; residency changed and a merge enqueue is \
                    queued for replay while the new region settles. The last refresh is recent.",
        fixture_filename: "region_migration_reconciling.json",
        expected_category: NoticeCategoryClass::TenantMigration,
        expected_effective_freshness: EffectiveFreshnessClass::Current,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 1,
        expected_changed_boundary_axis_count: 2,
        expected_boundary_change_unresolved: true,
        input: input(
            "region_migration_reconciling",
            "notice.migration.region",
            NoticeKindClass::RegionMigration,
            "Region migration reconciliation",
            "The region migration is reconciling; queued writes replay as the new region settles.",
            sched(
                TimeBasisClass::InProgressExact,
                "2026-05-20T11:10:00Z",
                Some("2026-05-20T12:20:00Z"),
                None,
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_RECENT),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme"],
                &["region.ref.eu-central", "region.ref.eu-west"],
                vec![ResidencyScopeClass::ResidencyChangedReviewRequired],
                vec![ServiceClass::MergeQueueService, ServiceClass::SyncService],
                "Managed-cloud merge queue and sync reconciling after a region move.",
            ),
            boundary(
                true,
                false,
                vec![
                    axis(
                        BoundaryAxisClass::Region,
                        BoundaryAxisStateClass::Changed,
                        Some("region.ref.eu-central"),
                        Some("aureline://region/eu-west"),
                        "Region moved to EU-west.",
                    ),
                    axis(
                        BoundaryAxisClass::Residency,
                        BoundaryAxisStateClass::Changed,
                        Some("residency.ref.eu-central"),
                        Some("aureline://residency/eu-west"),
                        "Residency moved with the region.",
                    ),
                ],
                "Region and residency changed; reconciliation in progress.",
            ),
            vec![blocked(
                ManagedActionClass::MergeQueueEnqueue,
                BlockStateClass::BlockedDrainNewActions,
                WriteContinuityPostureClass::QueuedPublishLater,
                SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                Some("aureline://publish_later_queue/reconcile.merge"),
                true,
                ResumeTriggerClass::ReconciliationCompletes,
                "Merge enqueues queue for replay once reconciliation completes.",
            )],
            vec![succeeded(
                ManagedActionClass::ManagedReviewApproval,
                "aureline://change_review/cr-5200",
                "2026-05-20T11:05:00Z",
                "Approval landed in the source region before the move.",
            )],
            local(
                LocalCoreStatusClass::MeaningfulSafeSubsetAvailable,
                &["Editing, saving, and local search continue."],
                true,
                "Local work continues; queued merges replay after reconciliation.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}

fn post_event_reconciliation_completed() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "post_event_reconciliation_completed",
        scenario_label: "Completed reconciliation that keeps the changed boundary visible",
        narrative: "A completed reconciliation after a tenant migration is retained for history; \
                    the boundary review is done but the changed tenant identity stays visible, \
                    and the completed notice cannot read as current.",
        fixture_filename: "post_event_reconciliation_completed.json",
        expected_category: NoticeCategoryClass::Maintenance,
        expected_effective_freshness: EffectiveFreshnessClass::CompletedHistorical,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 0,
        expected_changed_boundary_axis_count: 1,
        expected_boundary_change_unresolved: false,
        input: input(
            "post_event_reconciliation_completed",
            "notice.reconciliation.completed",
            NoticeKindClass::PostEventReconciliation,
            "Post-migration reconciliation complete",
            "Reconciliation after the tenant migration is complete; the tenant boundary changed.",
            sched(
                TimeBasisClass::CompletedExact,
                "2026-05-20T05:00:00Z",
                Some("2026-05-20T05:45:00Z"),
                Some("2026-05-20T05:45:00Z"),
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_STALE),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme-eu"],
                &["region.ref.eu-central"],
                vec![ResidencyScopeClass::SovereignRegionPinned],
                vec![ServiceClass::WorkspaceControlPlaneService],
                "Managed-cloud tenant reconciled in the EU sovereign region.",
            ),
            boundary(
                true,
                true,
                vec![axis(
                    BoundaryAxisClass::Tenant,
                    BoundaryAxisStateClass::Changed,
                    Some("tenant.ref.acme"),
                    Some("aureline://tenant/acme-eu"),
                    "Tenant moved to the EU tenant and was reviewed.",
                )],
                "Tenant boundary changed and was reviewed during reconciliation.",
            ),
            vec![blocked(
                ManagedActionClass::PolicyAdminWrite,
                BlockStateClass::DrainingExistingOnly,
                WriteContinuityPostureClass::RetryableWhenConnected,
                SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                None,
                false,
                ResumeTriggerClass::NotApplicable,
                "Policy writes resumed under the reviewed tenant; this row is historical.",
            )],
            vec![succeeded(
                ManagedActionClass::ManagedWorkspaceLifecycleWrite,
                "aureline://managed_workspace/mw-90",
                "2026-05-20T05:40:00Z",
                "Workspace lifecycle reconciled under the new tenant.",
            )],
            local(
                LocalCoreStatusClass::LocalCoreUnaffected,
                &["Editing, saving, and local search continue."],
                false,
                "Reconciliation is complete; the changed tenant identity is retained for history.",
            ),
            lifecycle(
                FreshnessClass::CompletedHistorical,
                Some("notice.migration.tenant"),
                None,
                Some("2026-08-20T00:00:00Z"),
                &["aureline://continuity_notice_history/notice.migration.tenant"],
            ),
        ),
    }
}

fn superseded_notice_downgraded() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "superseded_notice_downgraded",
        scenario_label: "Superseded notice cannot read as current despite a fresh refresh",
        narrative: "A maintenance notice was superseded by a rescheduled one; even with a fresh \
                    refresh it downgrades to superseded and points at the replacement so it \
                    cannot masquerade as current.",
        fixture_filename: "superseded_notice_downgraded.json",
        expected_category: NoticeCategoryClass::Maintenance,
        expected_effective_freshness: EffectiveFreshnessClass::SupersededStale,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 0,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: false,
        input: input(
            "superseded_notice_downgraded",
            "notice.maintenance.v1",
            NoticeKindClass::ScheduledMaintenanceWindow,
            "Sync maintenance window (rescheduled)",
            "This maintenance window was rescheduled; a newer notice replaces it.",
            sched(
                TimeBasisClass::ScheduledExact,
                "2026-05-20T14:00:00Z",
                Some("2026-05-20T15:00:00Z"),
                None,
                "Europe/Berlin",
                "+02:00",
                Some(REFRESH_FRESH),
            ),
            scope(
                vec![DeploymentProfileClass::ManagedCloud],
                &["tenant.ref.acme"],
                &["region.ref.eu-central"],
                vec![ResidencyScopeClass::CustomerRegionPinned],
                vec![ServiceClass::SyncService],
                "Managed-cloud sync maintenance, rescheduled.",
            ),
            boundary(
                false,
                false,
                vec![axis(
                    BoundaryAxisClass::Tenant,
                    BoundaryAxisStateClass::Unchanged,
                    None,
                    None,
                    "Tenant unchanged.",
                )],
                "No boundary change.",
            ),
            vec![blocked(
                ManagedActionClass::ManagedReviewCommentPublish,
                BlockStateClass::ScheduledToBlock,
                WriteContinuityPostureClass::RetryableWhenConnected,
                SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                None,
                false,
                ResumeTriggerClass::WindowEnds,
                "Comments retry after the rescheduled window; see the replacement notice.",
            )],
            vec![],
            local(
                LocalCoreStatusClass::LocalCoreUnaffected,
                &["Editing and saving continue."],
                false,
                "Local work continues; this notice is superseded by a rescheduled one.",
            ),
            lifecycle(
                FreshnessClass::SupersededStale,
                None,
                Some("notice.maintenance.v2"),
                Some("2026-06-20T00:00:00Z"),
                &["aureline://continuity_notice_history/notice.maintenance.v2"],
            ),
        ),
    }
}

fn imported_offline_history() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "imported_offline_history",
        scenario_label: "Imported air-gapped history with no live refresh",
        narrative: "A maintenance notice imported from an air-gapped capture has no live refresh; \
                    it is labeled imported history and never reads as current operational truth.",
        fixture_filename: "imported_offline_history.json",
        expected_category: NoticeCategoryClass::Maintenance,
        expected_effective_freshness: EffectiveFreshnessClass::ImportedHistorical,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 0,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: false,
        input: input(
            "imported_offline_history",
            "notice.maintenance.imported",
            NoticeKindClass::ScheduledMaintenanceWindow,
            "Imported maintenance window (offline capture)",
            "This maintenance notice was imported from an offline capture for the record.",
            sched(
                TimeBasisClass::HistoricalExact,
                "2026-05-18T02:00:00Z",
                Some("2026-05-18T03:00:00Z"),
                Some("2026-05-18T03:00:00Z"),
                "UTC",
                "Z",
                None,
            ),
            scope(
                vec![DeploymentProfileClass::AirGapped],
                &["tenant.ref.gov"],
                &["region.ref.sovereign"],
                vec![ResidencyScopeClass::SovereignRegionPinned],
                vec![ServiceClass::DocsPackService, ServiceClass::CatalogService],
                "Air-gapped docs-pack and catalog maintenance, imported for the record.",
            ),
            boundary(
                false,
                false,
                vec![axis(
                    BoundaryAxisClass::Tenant,
                    BoundaryAxisStateClass::Unchanged,
                    None,
                    None,
                    "Tenant unchanged in the imported record.",
                )],
                "No boundary change in the imported record.",
            ),
            vec![blocked(
                ManagedActionClass::ExtensionRegistryPublishOrInstall,
                BlockStateClass::BlockedByPolicy,
                WriteContinuityPostureClass::RetryableWhenConnected,
                SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                None,
                false,
                ResumeTriggerClass::NotApplicable,
                "Registry installs in the imported window are historical.",
            )],
            vec![],
            local(
                LocalCoreStatusClass::LocalCoreUnaffected,
                &["Editing, saving, and offline docs inspect continue."],
                false,
                "Imported history; local-first work was unaffected during the window.",
            ),
            lifecycle(
                FreshnessClass::ImportedHistorical,
                None,
                None,
                Some("2026-11-18T00:00:00Z"),
                &["aureline://continuity_notice_history/notice.maintenance.imported.source"],
            ),
        ),
    }
}

fn stale_refresh_active_downgraded() -> ContinuityNoticeScenario {
    ContinuityNoticeScenario {
        scenario_id: "stale_refresh_active_downgraded",
        scenario_label: "Active notice whose last refresh aged out downgrades",
        narrative: "A read-only window is still declared active, but its last refresh aged out \
                    more than a day ago, so it downgrades to stale and names why instead of \
                    presenting as current.",
        fixture_filename: "stale_refresh_active_downgraded.json",
        expected_category: NoticeCategoryClass::Drain,
        expected_effective_freshness: EffectiveFreshnessClass::RefreshStale,
        expected_honesty_marker_present: true,
        expected_preserved_intent_count: 1,
        expected_changed_boundary_axis_count: 0,
        expected_boundary_change_unresolved: false,
        input: input(
            "stale_refresh_active_downgraded",
            "notice.read_only.stale",
            NoticeKindClass::ReadOnlyWindow,
            "Read-only window (refresh aged out)",
            "A read-only window whose status has not refreshed recently.",
            sched(
                TimeBasisClass::InProgressExact,
                "2026-05-19T05:00:00Z",
                Some("2026-05-19T06:00:00Z"),
                None,
                "America/New_York",
                "-04:00",
                Some(REFRESH_VERY_STALE),
            ),
            scope(
                vec![DeploymentProfileClass::SelfHosted],
                &["tenant.ref.self"],
                &["region.ref.on-prem"],
                vec![ResidencyScopeClass::NotApplicable],
                vec![ServiceClass::TelemetrySinkService],
                "Self-hosted telemetry sink read-only window.",
            ),
            boundary(
                false,
                false,
                vec![axis(
                    BoundaryAxisClass::Tenant,
                    BoundaryAxisStateClass::Unchanged,
                    None,
                    None,
                    "Tenant unchanged.",
                )],
                "No boundary change.",
            ),
            vec![blocked(
                ManagedActionClass::TelemetryUpload,
                BlockStateClass::BlockedReadOnly,
                WriteContinuityPostureClass::QueuedPublishLater,
                SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                Some("aureline://publish_later_queue/stale.telemetry"),
                true,
                ResumeTriggerClass::WindowEnds,
                "Telemetry queues for replay; the window status is stale and needs a refresh.",
            )],
            vec![],
            local(
                LocalCoreStatusClass::LocalCoreUnaffected,
                &["Editing, saving, and local diagnostics continue."],
                true,
                "Local work continues; the notice status is stale and must be re-checked.",
            ),
            lifecycle(FreshnessClass::ActiveCurrent, None, None, None, &[]),
        ),
    }
}
