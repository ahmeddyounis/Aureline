//! Maintenance, drain, failover, and tenant-migration continuity-notice truth
//! model for the desktop shell.
//!
//! ## Why one continuity-notice view, not four banners
//!
//! Service-health truth says *"the sync service is degraded"*. Real operational
//! change says something far more specific: *"a planned read-only window starts
//! at 14:00 Europe/Berlin (+02:00), publish-later is queued and survives, your
//! local edits keep working, and your tenant moved to a new region so the next
//! publish needs a boundary review."* When each surface invents its own banner
//! copy for maintenance, drain, failover, and migration, three failures follow:
//!
//! - **Generic-degraded collapse.** A precise planned window or a region
//!   failover is flattened into the same "something is offline" banner, so the
//!   user cannot tell whether to wait, export, or postpone.
//! - **Lost queued work.** A publish-later or local-draft intent that was
//!   captured during a drain or failover is not visibly separated from
//!   successful hosted mutations, so the user cannot trust that it survived.
//! - **Stale-as-current.** A completed window, a superseded notice, or a
//!   notice whose last refresh aged out keeps presenting as current operational
//!   truth, and a recovered state silently hides a tenant/region/endpoint
//!   boundary that actually changed.
//!
//! This module mints one governed [`ContinuityNoticeView`] record that the
//! desktop shell, the activity center / durable history, CLI / headless
//! inspect, diagnostics, and support exports all read verbatim. It composes —
//! it does not fork — the upstream boundary records frozen in
//! `docs/ops/maintenance_migration_failover_contract.md` and
//! `docs/ops/failover_continuity_banner_contract.md`
//! (`maintenance_notice_record`, `tenant_migration_event_record`,
//! `failover_banner_record`, `local_safe_baseline_record`). The view's schema
//! boundary is `schemas/ops/continuity_notice_view.schema.json`.
//!
//! ## The no-silent-current invariant
//!
//! A continuity notice signals an operational change, so it must never quietly
//! present as routine current truth once it has aged out, been superseded, or
//! completed. The model enforces this structurally: a view's
//! [`EffectiveFreshnessClass`] is [`EffectiveFreshnessClass::Current`] **only**
//! when its declared [`FreshnessClass`] is [`FreshnessClass::ActiveCurrent`]
//! **and** its latest refresh is still within the current window
//! ([`RefreshAgeClass::Fresh`] or [`RefreshAgeClass::Recent`]) relative to a
//! caller-supplied `as_of`. Any other combination downgrades the effective
//! freshness, records the precise [`DowngradeReasonClass`] set, lights the
//! honesty marker, and forces a non-null stale label so the surface cannot read
//! the notice as live.
//!
//! ## The boundary-preserved-after-recovery invariant
//!
//! When a failover or migration changed a tenant, region, residency, key
//! ownership, or endpoint boundary, the changed identity stays visible even on a
//! completed/recovered notice. The model derives
//! [`ContinuityNoticeView::boundary_change_unresolved`] and refuses to let the
//! display copy hide a boundary change.
//!
//! ## What never crosses this boundary
//!
//! Raw endpoint URLs, hostnames, credentials, raw tenant/account names, raw
//! payloads, raw policy bodies, and absolute paths never appear on these
//! records. Surfaces carry opaque object refs (`aureline://<class>/<id>`),
//! stable tokens, and short reviewable sentences only.

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized continuity-notice views.
pub const CONTINUITY_NOTICE_VIEW_RECORD_KIND: &str = "continuity_notice_view_record";

/// Schema version for the [`ContinuityNoticeView`] payload shape.
pub const CONTINUITY_NOTICE_VIEW_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every continuity-notice surface.
pub const CONTINUITY_NOTICE_NOTICE: &str =
    "Maintenance & failover truth: every notice declares whether it is a maintenance, drain, \
     failover, or tenant-migration window, the exact window time / timezone / offset, the \
     affected deployment scope and write classes, which queued publish-later or local-draft work \
     survives (kept separate from successful hosted mutations), what stays local-safe, and any \
     changed tenant / region / endpoint boundary. A notice reads as current only while it is \
     active and its last refresh is current — otherwise it downgrades and names why, and never \
     collapses into a generic degraded banner. Shell, activity history, CLI / headless inspect, \
     diagnostics, and support exports read this record verbatim.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a short title.
const MAX_TITLE_CHARS: usize = 200;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Canonical durable-object URI scheme. Every "open history", "open support
/// export", and "open boundary details" affordance must route to one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one of these is rejected so the
/// chrome cannot wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home",
    "dashboard",
    "landing",
    "index",
    "overview",
    "start",
    "root",
];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

// ---------------------------------------------------------------------------
// Notice kind / category / plan vocabulary
// ---------------------------------------------------------------------------

/// The kind of operational change a continuity notice communicates. Closed set;
/// surfaces MUST NOT invent kinds outside it. Re-projects the upstream
/// `maintenance_kind_class` and `event_kind_class` vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeKindClass {
    /// Planned full maintenance window.
    ScheduledMaintenanceWindow,
    /// Planned read-only window (reads work, hosted writes are paused).
    ReadOnlyWindow,
    /// Planned drain window (existing sessions finish, new writes are blocked).
    DrainWindow,
    /// Planned export-freeze window before a larger operation.
    ScheduledExportFreeze,
    /// Tenant migration to a new tenant boundary.
    TenantMigration,
    /// Region or residency migration.
    RegionMigration,
    /// Emergency regional failover.
    RegionalFailover,
    /// Emergency control-plane failover.
    ControlPlaneFailover,
    /// Post-event reconciliation after a window, migration, or failover.
    PostEventReconciliation,
}

impl NoticeKindClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScheduledMaintenanceWindow => "scheduled_maintenance_window",
            Self::ReadOnlyWindow => "read_only_window",
            Self::DrainWindow => "drain_window",
            Self::ScheduledExportFreeze => "scheduled_export_freeze",
            Self::TenantMigration => "tenant_migration",
            Self::RegionMigration => "region_migration",
            Self::RegionalFailover => "regional_failover",
            Self::ControlPlaneFailover => "control_plane_failover",
            Self::PostEventReconciliation => "post_event_reconciliation",
        }
    }

    /// Human-readable label, quoted verbatim across surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ScheduledMaintenanceWindow => "Scheduled maintenance window",
            Self::ReadOnlyWindow => "Read-only window",
            Self::DrainWindow => "Drain window",
            Self::ScheduledExportFreeze => "Scheduled export freeze",
            Self::TenantMigration => "Tenant migration",
            Self::RegionMigration => "Region migration",
            Self::RegionalFailover => "Regional failover",
            Self::ControlPlaneFailover => "Control-plane failover",
            Self::PostEventReconciliation => "Post-event reconciliation",
        }
    }

    /// The coarse category the acceptance criterion asks the user to tell apart:
    /// maintenance, drain, failover, or tenant-migration.
    pub const fn category(self) -> NoticeCategoryClass {
        match self {
            Self::ScheduledMaintenanceWindow
            | Self::ScheduledExportFreeze
            | Self::PostEventReconciliation => NoticeCategoryClass::Maintenance,
            Self::ReadOnlyWindow | Self::DrainWindow => NoticeCategoryClass::Drain,
            Self::RegionalFailover | Self::ControlPlaneFailover => NoticeCategoryClass::Failover,
            Self::TenantMigration | Self::RegionMigration => NoticeCategoryClass::TenantMigration,
        }
    }

    /// Whether this kind is planned (vs an emergency failover). Drives the
    /// `incident_language_for_planned` invariant.
    pub const fn plan_class(self) -> PlanClass {
        match self {
            Self::RegionalFailover | Self::ControlPlaneFailover => PlanClass::Emergency,
            _ => PlanClass::Planned,
        }
    }
}

/// The coarse window category. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeCategoryClass {
    /// Maintenance / export-freeze / reconciliation.
    Maintenance,
    /// Read-only / drain window.
    Drain,
    /// Regional or control-plane failover.
    Failover,
    /// Tenant or region migration.
    TenantMigration,
}

impl NoticeCategoryClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Maintenance => "maintenance",
            Self::Drain => "drain",
            Self::Failover => "failover",
            Self::TenantMigration => "tenant_migration",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Maintenance => "Maintenance",
            Self::Drain => "Drain / read-only",
            Self::Failover => "Failover",
            Self::TenantMigration => "Tenant migration",
        }
    }
}

/// Whether a window is planned or an emergency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanClass {
    /// Planned, announced ahead of time.
    Planned,
    /// Emergency / unplanned failover.
    Emergency,
}

impl PlanClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Emergency => "emergency",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planned => "Planned",
            Self::Emergency => "Emergency",
        }
    }
}

// ---------------------------------------------------------------------------
// Freshness / refresh-age vocabulary
// ---------------------------------------------------------------------------

/// The declared lifecycle freshness of a notice as recorded by the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// The notice is the current, active record.
    ActiveCurrent,
    /// The notice was superseded by a newer one.
    SupersededStale,
    /// The notice's window completed and it is retained for history.
    CompletedHistorical,
    /// The notice was imported from an offline / air-gapped capture.
    ImportedHistorical,
}

impl FreshnessClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveCurrent => "active_current",
            Self::SupersededStale => "superseded_stale",
            Self::CompletedHistorical => "completed_historical",
            Self::ImportedHistorical => "imported_historical",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveCurrent => "Active (current)",
            Self::SupersededStale => "Superseded",
            Self::CompletedHistorical => "Completed (history)",
            Self::ImportedHistorical => "Imported (history)",
        }
    }
}

/// The honest, derived freshness after applying the latest-refresh rule. This is
/// what the chrome MUST render — never the raw declared [`FreshnessClass`] alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveFreshnessClass {
    /// Active and last refresh is current.
    Current,
    /// Declared active, but the last refresh aged out — not safe to read live.
    RefreshStale,
    /// Superseded by a newer notice.
    SupersededStale,
    /// Window completed; retained for history.
    CompletedHistorical,
    /// Imported offline history.
    ImportedHistorical,
}

impl EffectiveFreshnessClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::RefreshStale => "refresh_stale",
            Self::SupersededStale => "superseded_stale",
            Self::CompletedHistorical => "completed_historical",
            Self::ImportedHistorical => "imported_historical",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Current => "Current",
            Self::RefreshStale => "Stale — last refresh aged out",
            Self::SupersededStale => "Superseded — newer notice exists",
            Self::CompletedHistorical => "Completed — historical record",
            Self::ImportedHistorical => "Imported — offline history",
        }
    }

    /// True when the notice may be read as live operational truth.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Bucketed age of the notice's last successful refresh relative to `as_of`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefreshAgeClass {
    /// Refreshed within the last 5 minutes.
    Fresh,
    /// Refreshed within the last hour.
    Recent,
    /// Refreshed hours ago.
    Stale,
    /// Refreshed more than a day ago.
    VeryStale,
    /// No refresh recorded.
    Never,
}

impl RefreshAgeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::VeryStale => "very_stale",
            Self::Never => "never",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Just now",
            Self::Recent => "Within the hour",
            Self::Stale => "Hours ago",
            Self::VeryStale => "More than a day ago",
            Self::Never => "No refresh recorded",
        }
    }

    /// True when the refresh is current (fresh or recent).
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Fresh | Self::Recent)
    }
}

/// Why an effective freshness was downgraded from current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReasonClass {
    /// Declared active but the last refresh aged out.
    RefreshExpired,
    /// A newer notice superseded this one.
    NoticeSuperseded,
    /// The window completed.
    WindowCompleted,
    /// Imported from an offline capture.
    ImportedOffline,
}

impl DowngradeReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshExpired => "refresh_expired",
            Self::NoticeSuperseded => "notice_superseded",
            Self::WindowCompleted => "window_completed",
            Self::ImportedOffline => "imported_offline",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RefreshExpired => "Last refresh aged out",
            Self::NoticeSuperseded => "Superseded by a newer notice",
            Self::WindowCompleted => "Window completed",
            Self::ImportedOffline => "Imported offline history",
        }
    }
}

// ---------------------------------------------------------------------------
// Scope vocabulary
// ---------------------------------------------------------------------------

/// Deployment profile a notice applies to. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    /// Individual local profile.
    IndividualLocal,
    /// Self-hosted profile.
    SelfHosted,
    /// Enterprise online profile.
    EnterpriseOnline,
    /// Air-gapped profile.
    AirGapped,
    /// Managed cloud profile.
    ManagedCloud,
}

impl DeploymentProfileClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }
}

/// Residency scope a notice applies to. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyScopeClass {
    /// Residency is not applicable.
    NotApplicable,
    /// Customer-region-pinned residency.
    CustomerRegionPinned,
    /// Vendor-region-default residency.
    VendorRegionDefault,
    /// Sovereign-region-pinned residency.
    SovereignRegionPinned,
    /// Tenant residency requires a recheck.
    TenantResidencyRecheckRequired,
    /// Residency changed and requires review.
    ResidencyChangedReviewRequired,
}

impl ResidencyScopeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::CustomerRegionPinned => "customer_region_pinned",
            Self::VendorRegionDefault => "vendor_region_default",
            Self::SovereignRegionPinned => "sovereign_region_pinned",
            Self::TenantResidencyRecheckRequired => "tenant_residency_recheck_required",
            Self::ResidencyChangedReviewRequired => "residency_changed_review_required",
        }
    }
}

/// Control-plane service class affected by a notice. Closed set, mirrors the
/// upstream `control_plane_service_class` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceClass {
    /// Sync service.
    SyncService,
    /// Registry service.
    RegistryService,
    /// Relay service.
    RelayService,
    /// AI broker service.
    AiBrokerService,
    /// Auth / identity service.
    AuthIdentityService,
    /// Policy service.
    PolicyService,
    /// Docs-pack service.
    DocsPackService,
    /// Catalog service.
    CatalogService,
    /// Telemetry sink service.
    TelemetrySinkService,
    /// Workspace control-plane service.
    WorkspaceControlPlaneService,
    /// Provider review service.
    ProviderReviewService,
    /// Merge-queue service.
    MergeQueueService,
    /// Support ingest service.
    SupportIngestService,
}

impl ServiceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncService => "sync_service",
            Self::RegistryService => "registry_service",
            Self::RelayService => "relay_service",
            Self::AiBrokerService => "ai_broker_service",
            Self::AuthIdentityService => "auth_identity_service",
            Self::PolicyService => "policy_service",
            Self::DocsPackService => "docs_pack_service",
            Self::CatalogService => "catalog_service",
            Self::TelemetrySinkService => "telemetry_sink_service",
            Self::WorkspaceControlPlaneService => "workspace_control_plane_service",
            Self::ProviderReviewService => "provider_review_service",
            Self::MergeQueueService => "merge_queue_service",
            Self::SupportIngestService => "support_ingest_service",
        }
    }
}

/// Time-basis for a notice schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeBasisClass {
    /// Scheduled exact time.
    ScheduledExact,
    /// Detected exact time (for failover).
    DetectedExact,
    /// In-progress exact time.
    InProgressExact,
    /// Completed exact time.
    CompletedExact,
    /// Historical exact time.
    HistoricalExact,
}

impl TimeBasisClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScheduledExact => "scheduled_exact",
            Self::DetectedExact => "detected_exact",
            Self::InProgressExact => "in_progress_exact",
            Self::CompletedExact => "completed_exact",
            Self::HistoricalExact => "historical_exact",
        }
    }
}

// ---------------------------------------------------------------------------
// Write-class vocabulary
// ---------------------------------------------------------------------------

/// Managed action class a notice blocks, queues, or preserves. Closed set,
/// composes the upstream maintenance / failover action vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedActionClass {
    /// Managed review-comment publish.
    ManagedReviewCommentPublish,
    /// Managed review approval.
    ManagedReviewApproval,
    /// Merge-queue enqueue.
    MergeQueueEnqueue,
    /// Collaboration presence write.
    CollaborationPresenceWrite,
    /// Remote control session join.
    RemoteControlSessionJoin,
    /// Profile / settings sync write.
    ProfileSettingsSyncWrite,
    /// Policy admin write.
    PolicyAdminWrite,
    /// Extension registry publish or install.
    ExtensionRegistryPublishOrInstall,
    /// Managed workspace lifecycle write.
    ManagedWorkspaceLifecycleWrite,
    /// AI gateway prompt submit.
    AiGatewayPromptSubmit,
    /// Support bundle upload.
    SupportBundleUpload,
    /// Telemetry upload.
    TelemetryUpload,
    /// Provider publish to a local draft.
    ProviderPublishLocalDraft,
    /// Provider publish that takes effect immediately.
    ProviderPublishImmediate,
}

impl ManagedActionClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedReviewCommentPublish => "managed_review_comment_publish",
            Self::ManagedReviewApproval => "managed_review_approval",
            Self::MergeQueueEnqueue => "merge_queue_enqueue",
            Self::CollaborationPresenceWrite => "collaboration_presence_write",
            Self::RemoteControlSessionJoin => "remote_control_session_join",
            Self::ProfileSettingsSyncWrite => "profile_settings_sync_write",
            Self::PolicyAdminWrite => "policy_admin_write",
            Self::ExtensionRegistryPublishOrInstall => "extension_registry_publish_or_install",
            Self::ManagedWorkspaceLifecycleWrite => "managed_workspace_lifecycle_write",
            Self::AiGatewayPromptSubmit => "ai_gateway_prompt_submit",
            Self::SupportBundleUpload => "support_bundle_upload",
            Self::TelemetryUpload => "telemetry_upload",
            Self::ProviderPublishLocalDraft => "provider_publish_local_draft",
            Self::ProviderPublishImmediate => "provider_publish_immediate",
        }
    }
}

/// How a blocked write fares during the window — the queued / preserved /
/// blocked posture. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteContinuityPostureClass {
    /// Queued in the publish-later outbox; survives the window.
    QueuedPublishLater,
    /// Captured as a local draft; survives locally.
    LocalDraftPreserved,
    /// Rejected but safe to retry once connectivity / the window resumes.
    RetryableWhenConnected,
    /// Existing sessions drain; no new writes admitted.
    DrainingExistingOnly,
    /// Blocked until reconnect.
    BlockedPendingReconnect,
    /// Blocked until the new boundary is reviewed.
    BlockedPendingBoundaryRecheck,
    /// Blocked with no safe retry; the user must export or abandon.
    BlockedNoSafeRetry,
    /// Requires a manual rerun after the window.
    RequiresManualRerun,
}

impl WriteContinuityPostureClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueuedPublishLater => "queued_publish_later",
            Self::LocalDraftPreserved => "local_draft_preserved",
            Self::RetryableWhenConnected => "retryable_when_connected",
            Self::DrainingExistingOnly => "draining_existing_only",
            Self::BlockedPendingReconnect => "blocked_pending_reconnect",
            Self::BlockedPendingBoundaryRecheck => "blocked_pending_boundary_recheck",
            Self::BlockedNoSafeRetry => "blocked_no_safe_retry",
            Self::RequiresManualRerun => "requires_manual_rerun",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QueuedPublishLater => "Queued for publish-later",
            Self::LocalDraftPreserved => "Saved as local draft",
            Self::RetryableWhenConnected => "Retryable when resumed",
            Self::DrainingExistingOnly => "Draining existing only",
            Self::BlockedPendingReconnect => "Blocked pending reconnect",
            Self::BlockedPendingBoundaryRecheck => "Blocked pending boundary review",
            Self::BlockedNoSafeRetry => "Blocked — no safe retry",
            Self::RequiresManualRerun => "Requires manual rerun",
        }
    }

    /// True when the write's intent is durably preserved (queued or local
    /// draft) and therefore survives the drain / failover path.
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::QueuedPublishLater | Self::LocalDraftPreserved)
    }

    /// True when the write must carry a durable queue / intent ref.
    pub const fn requires_queue_ref(self) -> bool {
        self.is_preserved()
    }
}

/// Guidance on whether a quiet retry is safe, or whether exporting / postponing
/// / a manual rerun is safer. Answers "when manual export or postpone is safer
/// than retry."
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaferThanRetryGuidanceClass {
    /// Retry is safe once the window / connectivity resumes.
    RetrySafeWhenResumed,
    /// Exporting now is safer than waiting to retry.
    ExportNowSafer,
    /// Postponing is safer than retrying into the window.
    PostponeSafer,
    /// A manual rerun is required; a quiet retry is not safe.
    ManualRerunRequired,
    /// No safe retry exists; escalate or abandon.
    NoSafeRetryEscalate,
}

impl SaferThanRetryGuidanceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetrySafeWhenResumed => "retry_safe_when_resumed",
            Self::ExportNowSafer => "export_now_safer",
            Self::PostponeSafer => "postpone_safer",
            Self::ManualRerunRequired => "manual_rerun_required",
            Self::NoSafeRetryEscalate => "no_safe_retry_escalate",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RetrySafeWhenResumed => "Retry is safe once resumed",
            Self::ExportNowSafer => "Export now — safer than retrying",
            Self::PostponeSafer => "Postpone — safer than retrying now",
            Self::ManualRerunRequired => "Manual rerun required",
            Self::NoSafeRetryEscalate => "No safe retry — escalate",
        }
    }
}

/// Block state for a write class during the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockStateClass {
    /// Scheduled to block when the window starts.
    ScheduledToBlock,
    /// Blocked because the window is read-only.
    BlockedReadOnly,
    /// Blocked because the drain refuses new actions.
    BlockedDrainNewActions,
    /// Blocked pending reconnect.
    BlockedPendingReconnect,
    /// Blocked pending boundary recheck.
    BlockedPendingBoundaryRecheck,
    /// Blocked by policy.
    BlockedByPolicy,
    /// Draining: existing actions finish, no new ones.
    DrainingExistingOnly,
}

impl BlockStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScheduledToBlock => "scheduled_to_block",
            Self::BlockedReadOnly => "blocked_read_only",
            Self::BlockedDrainNewActions => "blocked_drain_new_actions",
            Self::BlockedPendingReconnect => "blocked_pending_reconnect",
            Self::BlockedPendingBoundaryRecheck => "blocked_pending_boundary_recheck",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::DrainingExistingOnly => "draining_existing_only",
        }
    }
}

/// What unblocks a write class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeTriggerClass {
    /// The window starting.
    WindowStarts,
    /// The window ending.
    WindowEnds,
    /// The drain completing.
    DrainCompletes,
    /// The cutover completing.
    CutoverCompletes,
    /// Reconciliation completing.
    ReconciliationCompletes,
    /// A boundary review completing.
    BoundaryReviewCompleted,
    /// A fresh approval being issued.
    FreshApprovalIssued,
    /// A manual reconnect.
    ManualReconnectRequired,
    /// A manual export.
    ManualExportRequired,
    /// Not applicable.
    NotApplicable,
}

impl ResumeTriggerClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowStarts => "window_starts",
            Self::WindowEnds => "window_ends",
            Self::DrainCompletes => "drain_completes",
            Self::CutoverCompletes => "cutover_completes",
            Self::ReconciliationCompletes => "reconciliation_completes",
            Self::BoundaryReviewCompleted => "boundary_review_completed",
            Self::FreshApprovalIssued => "fresh_approval_issued",
            Self::ManualReconnectRequired => "manual_reconnect_required",
            Self::ManualExportRequired => "manual_export_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary vocabulary
// ---------------------------------------------------------------------------

/// Boundary axis a migration / failover can change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAxisClass {
    /// Tenant boundary.
    Tenant,
    /// Region boundary.
    Region,
    /// Residency boundary.
    Residency,
    /// Key-ownership boundary.
    KeyOwnership,
    /// Endpoint-identity boundary.
    EndpointIdentity,
}

impl BoundaryAxisClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tenant => "tenant",
            Self::Region => "region",
            Self::Residency => "residency",
            Self::KeyOwnership => "key_ownership",
            Self::EndpointIdentity => "endpoint_identity",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tenant => "Tenant",
            Self::Region => "Region",
            Self::Residency => "Residency",
            Self::KeyOwnership => "Key ownership",
            Self::EndpointIdentity => "Endpoint identity",
        }
    }
}

/// State of a boundary axis after the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAxisStateClass {
    /// Unchanged.
    Unchanged,
    /// Changed to a new value.
    Changed,
    /// Unknown; requires a recheck.
    UnknownRecheckRequired,
    /// Not applicable.
    NotApplicable,
}

impl BoundaryAxisStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Changed => "changed",
            Self::UnknownRecheckRequired => "unknown_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unchanged => "Unchanged",
            Self::Changed => "Changed",
            Self::UnknownRecheckRequired => "Unknown — recheck required",
            Self::NotApplicable => "Not applicable",
        }
    }

    /// True when the axis is a meaningful boundary change a recovered state must
    /// keep visible.
    pub const fn is_meaningful_change(self) -> bool {
        matches!(self, Self::Changed | Self::UnknownRecheckRequired)
    }
}

/// Local-core status during the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCoreStatusClass {
    /// Local core is fully unaffected.
    LocalCoreUnaffected,
    /// A meaningful safe subset is available.
    MeaningfulSafeSubsetAvailable,
    /// Only local-only work is available.
    LocalOnlyAvailable,
    /// No safe local subset.
    NoSafeLocalSubset,
    /// Unknown; requires review.
    UnknownRequiresReview,
}

impl LocalCoreStatusClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreUnaffected => "local_core_unaffected",
            Self::MeaningfulSafeSubsetAvailable => "meaningful_safe_subset_available",
            Self::LocalOnlyAvailable => "local_only_available",
            Self::NoSafeLocalSubset => "no_safe_local_subset",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalCoreUnaffected => "Local core unaffected",
            Self::MeaningfulSafeSubsetAvailable => "Meaningful local subset available",
            Self::LocalOnlyAvailable => "Local-only available",
            Self::NoSafeLocalSubset => "No safe local subset",
            Self::UnknownRequiresReview => "Unknown — requires review",
        }
    }
}

// ---------------------------------------------------------------------------
// Builder inputs
// ---------------------------------------------------------------------------

/// Schedule input for a continuity notice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleInput {
    /// Time basis.
    pub time_basis: TimeBasisClass,
    /// Window start (UTC).
    pub starts_at: String,
    /// Expected or actual window end (UTC), if known.
    pub expected_or_actual_ends_at: Option<String>,
    /// Completion time (UTC), if completed.
    pub completed_at: Option<String>,
    /// IANA display timezone id.
    pub timezone_id: String,
    /// UTC offset in force at window start (for example `+02:00`).
    pub utc_offset_at_start: String,
    /// Latest successful refresh of this notice (UTC), if any.
    pub latest_refresh_at: Option<String>,
}

/// Affected-scope input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeInput {
    /// Deployment profiles in scope.
    pub deployment_profiles: Vec<DeploymentProfileClass>,
    /// Opaque tenant refs in scope.
    pub tenant_refs: Vec<String>,
    /// Opaque region refs in scope.
    pub region_refs: Vec<String>,
    /// Residency scopes in scope.
    pub residency_scope_classes: Vec<ResidencyScopeClass>,
    /// Control-plane service classes in scope.
    pub service_classes: Vec<ServiceClass>,
    /// One-sentence scope summary.
    pub scope_summary: String,
}

/// One boundary-axis input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryAxisInput {
    /// Axis.
    pub axis_class: BoundaryAxisClass,
    /// Axis state after the event.
    pub axis_state_class: BoundaryAxisStateClass,
    /// Opaque previous value ref, if changed.
    pub previous_ref: Option<String>,
    /// Opaque current value ref, if changed.
    pub current_ref: Option<String>,
    /// One-sentence axis summary.
    pub summary: String,
}

/// Boundary-change input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryChangeInput {
    /// Whether a boundary change is in play.
    pub boundary_change_required: bool,
    /// Whether the boundary review is complete.
    pub review_completed: bool,
    /// Per-axis rows.
    pub axes: Vec<BoundaryAxisInput>,
    /// One-sentence boundary summary.
    pub summary: String,
}

/// One blocked / queued / preserved write-class input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockedWriteInput {
    /// Action class.
    pub action_class: ManagedActionClass,
    /// Block state.
    pub block_state_class: BlockStateClass,
    /// Continuity posture (queued / preserved / blocked).
    pub continuity_posture: WriteContinuityPostureClass,
    /// Safer-than-retry guidance.
    pub safer_guidance: SaferThanRetryGuidanceClass,
    /// Opaque queue / intent ref (required for preserved postures).
    pub queue_or_intent_ref: Option<String>,
    /// Whether an idempotency key is present.
    pub idempotency_key_present: bool,
    /// What unblocks the write.
    pub resume_trigger: ResumeTriggerClass,
    /// One-sentence note.
    pub note: String,
}

/// One successful hosted-mutation input. Kept separate from blocked / queued
/// writes so survived queued work is visibly distinct from work that landed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostedMutationInput {
    /// Action class.
    pub action_class: ManagedActionClass,
    /// Opaque result object ref.
    pub result_ref: String,
    /// Completion time (UTC).
    pub completed_at: String,
    /// One-sentence note.
    pub note: String,
}

/// Local-safe continuity input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContinuityInput {
    /// Local-core status.
    pub local_core_status: LocalCoreStatusClass,
    /// Retained local-safe capability sentences.
    pub retained_local_safe_capabilities: Vec<String>,
    /// Whether continue-local guidance is required.
    pub continue_local_guidance_required: bool,
    /// One-sentence continuity summary.
    pub continuity_summary: String,
}

/// Lifecycle input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleInput {
    /// Declared lifecycle freshness.
    pub freshness_class: FreshnessClass,
    /// Notice this one supersedes, if any.
    pub supersedes_id: Option<String>,
    /// Notice that superseded this one, if any.
    pub superseded_by_id: Option<String>,
    /// Retention deadline (UTC), if any.
    pub retained_until_at: Option<String>,
    /// Opaque history refs.
    pub history_refs: Vec<String>,
}

/// The full set of typed inputs the corpus and any producer hand to the
/// builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuityNoticeInput {
    /// Stable view id.
    pub view_id: String,
    /// Stable notice id.
    pub notice_id: String,
    /// Notice kind.
    pub notice_kind: NoticeKindClass,
    /// Title.
    pub title: String,
    /// One-sentence summary.
    pub summary: String,
    /// Creation time (UTC).
    pub created_at: String,
    /// Last-update time (UTC).
    pub updated_at: String,
    /// Schedule.
    pub schedule: ScheduleInput,
    /// Affected scope.
    pub affected_scope: ScopeInput,
    /// Boundary change.
    pub boundary_change: BoundaryChangeInput,
    /// Blocked / queued / preserved write classes.
    pub blocked_writes: Vec<BlockedWriteInput>,
    /// Successful hosted mutations (visibly separated from queued work).
    pub succeeded_hosted_mutations: Vec<HostedMutationInput>,
    /// Local-safe continuity.
    pub local_continuity: LocalContinuityInput,
    /// Lifecycle.
    pub lifecycle: LifecycleInput,
    /// Canonical durable history object ref.
    pub history_ref: String,
    /// Canonical durable support-export object ref.
    pub support_export_ref: String,
    /// Opaque evidence refs.
    pub evidence_refs: Vec<String>,
    /// Repo-relative narrative refs.
    pub narrative_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Projected (serialized) records
// ---------------------------------------------------------------------------

/// Projected schedule with the derived refresh-age class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeSchedule {
    /// Time basis.
    pub time_basis: TimeBasisClass,
    /// Time-basis label.
    pub time_basis_label: String,
    /// Window start (UTC).
    pub starts_at: String,
    /// Expected or actual window end (UTC), if known.
    pub expected_or_actual_ends_at: Option<String>,
    /// Completion time (UTC), if completed.
    pub completed_at: Option<String>,
    /// IANA display timezone id.
    pub timezone_id: String,
    /// UTC offset at start.
    pub utc_offset_at_start: String,
    /// Latest refresh (UTC), if any.
    pub latest_refresh_at: Option<String>,
    /// Derived refresh age relative to `as_of`.
    pub refresh_age: RefreshAgeClass,
    /// Refresh-age label.
    pub refresh_age_label: String,
}

/// Projected affected scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedScope {
    /// Deployment profiles in scope.
    pub deployment_profiles: Vec<DeploymentProfileClass>,
    /// Opaque tenant refs.
    pub tenant_refs: Vec<String>,
    /// Opaque region refs.
    pub region_refs: Vec<String>,
    /// Residency scopes.
    pub residency_scope_classes: Vec<ResidencyScopeClass>,
    /// Control-plane service classes.
    pub service_classes: Vec<ServiceClass>,
    /// Scope summary.
    pub scope_summary: String,
}

/// Projected boundary-axis row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryAxisRow {
    /// Axis.
    pub axis_class: BoundaryAxisClass,
    /// Axis label.
    pub axis_label: String,
    /// Axis state.
    pub axis_state_class: BoundaryAxisStateClass,
    /// Axis-state label.
    pub axis_state_label: String,
    /// Opaque previous value ref.
    pub previous_ref: Option<String>,
    /// Opaque current value ref.
    pub current_ref: Option<String>,
    /// Axis summary.
    pub summary: String,
}

/// Projected boundary-change block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryChange {
    /// Whether a boundary change is in play.
    pub boundary_change_required: bool,
    /// Whether the boundary review is complete.
    pub review_completed: bool,
    /// Whether the boundary change is unresolved (changed/unknown axis without a
    /// completed review). A recovered state must keep this visible.
    pub boundary_change_unresolved: bool,
    /// Count of meaningfully changed axes.
    pub changed_axis_count: u32,
    /// Count of unknown-recheck axes.
    pub unknown_axis_count: u32,
    /// Per-axis rows.
    pub axes: Vec<BoundaryAxisRow>,
    /// Boundary summary.
    pub summary: String,
}

/// Projected blocked / queued / preserved write row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedWriteRow {
    /// Action class.
    pub action_class: ManagedActionClass,
    /// Block state.
    pub block_state_class: BlockStateClass,
    /// Continuity posture.
    pub continuity_posture: WriteContinuityPostureClass,
    /// Continuity-posture label.
    pub continuity_posture_label: String,
    /// Safer-than-retry guidance.
    pub safer_guidance: SaferThanRetryGuidanceClass,
    /// Safer-than-retry label.
    pub safer_guidance_label: String,
    /// Opaque queue / intent ref.
    pub queue_or_intent_ref: Option<String>,
    /// Whether an idempotency key is present.
    pub idempotency_key_present: bool,
    /// Whether the write's intent is durably preserved.
    pub intent_preserved: bool,
    /// Resume trigger.
    pub resume_trigger: ResumeTriggerClass,
    /// Note.
    pub note: String,
}

/// Projected successful hosted-mutation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostedMutationRow {
    /// Action class.
    pub action_class: ManagedActionClass,
    /// Opaque result object ref.
    pub result_ref: String,
    /// Completion time (UTC).
    pub completed_at: String,
    /// Note.
    pub note: String,
}

/// Projected local-safe continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalContinuity {
    /// Local-core status.
    pub local_core_status: LocalCoreStatusClass,
    /// Local-core status label.
    pub local_core_status_label: String,
    /// Retained local-safe capability sentences.
    pub retained_local_safe_capabilities: Vec<String>,
    /// Whether continue-local guidance is required.
    pub continue_local_guidance_required: bool,
    /// Continuity summary.
    pub continuity_summary: String,
}

/// Projected lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeLifecycle {
    /// Declared lifecycle freshness.
    pub freshness_class: FreshnessClass,
    /// Declared-freshness label.
    pub freshness_label: String,
    /// Notice this one supersedes.
    pub supersedes_id: Option<String>,
    /// Notice that superseded this one.
    pub superseded_by_id: Option<String>,
    /// Retention deadline (UTC).
    pub retained_until_at: Option<String>,
    /// Opaque history refs.
    pub history_refs: Vec<String>,
}

/// Summary counters the chrome reads without recomputing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityNoticeSummary {
    /// Total blocked / queued / preserved write rows.
    pub blocked_write_count: u32,
    /// Rows queued for publish-later.
    pub queued_publish_later_count: u32,
    /// Rows saved as local drafts.
    pub local_draft_preserved_count: u32,
    /// Rows whose intent is durably preserved (queued + local draft).
    pub preserved_intent_count: u32,
    /// Rows blocked with no safe retry.
    pub blocked_no_safe_retry_count: u32,
    /// Rows requiring a manual rerun.
    pub requires_manual_rerun_count: u32,
    /// Successful hosted mutations.
    pub succeeded_hosted_mutation_count: u32,
    /// Meaningfully changed boundary axes.
    pub changed_boundary_axis_count: u32,
    /// Unknown-recheck boundary axes.
    pub unknown_boundary_axis_count: u32,
}

/// Display copy and the closed set of "no lie" invariant flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayCopy {
    /// Primary status line.
    pub primary_status_line: String,
    /// Schedule line (time + timezone + offset).
    pub schedule_line: String,
    /// Scope line.
    pub scope_line: String,
    /// Blocked-writes line.
    pub blocked_writes_line: String,
    /// Queued / preserved line.
    pub queued_preserved_line: String,
    /// Succeeded-mutations line (kept distinct from queued work).
    pub succeeded_line: String,
    /// Local-continuity line.
    pub local_continuity_line: String,
    /// Boundary-change line.
    pub boundary_change_line: String,
    /// Freshness line.
    pub freshness_line: String,
    /// Follow-up line.
    pub follow_up_line: String,
    /// Stale label; non-null whenever the effective freshness is not current.
    pub stale_label: Option<String>,
    /// MUST be false: a notice cannot imply all work is broken when a safe local
    /// subset exists.
    pub all_work_broken_implied: bool,
    /// MUST be false for planned windows: planned notices cannot reuse incident
    /// language.
    pub incident_language_for_planned_used: bool,
    /// MUST be false: a notice cannot collapse into a generic degraded banner.
    pub generic_degraded_banner_used: bool,
    /// MUST be false: queued / preserved work and successful hosted mutations
    /// cannot share one line.
    pub queued_and_succeeded_collapsed: bool,
    /// MUST be false: a stale / superseded / completed notice cannot present as
    /// current.
    pub stale_presented_as_current: bool,
    /// MUST be false: a changed boundary cannot be hidden on a recovered state.
    pub boundary_change_hidden: bool,
}

/// The governed continuity-notice view every surface reads verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityNoticeView {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Stable view id.
    pub view_id: String,
    /// Stable notice id.
    pub notice_id: String,
    /// Evaluation instant used to derive refresh age.
    pub as_of: String,
    /// Notice kind.
    pub notice_kind: NoticeKindClass,
    /// Notice-kind label.
    pub notice_kind_label: String,
    /// Coarse category.
    pub category: NoticeCategoryClass,
    /// Category label.
    pub category_label: String,
    /// Plan class.
    pub plan_class: PlanClass,
    /// Plan-class label.
    pub plan_class_label: String,
    /// Title.
    pub title: String,
    /// Summary.
    pub summary: String,
    /// Creation time (UTC).
    pub created_at: String,
    /// Last-update time (UTC).
    pub updated_at: String,
    /// Schedule.
    pub schedule: NoticeSchedule,
    /// Affected scope.
    pub affected_scope: AffectedScope,
    /// Boundary change.
    pub boundary_change: BoundaryChange,
    /// Blocked / queued / preserved write rows.
    pub blocked_writes: Vec<BlockedWriteRow>,
    /// Successful hosted mutations.
    pub succeeded_hosted_mutations: Vec<HostedMutationRow>,
    /// Local-safe continuity.
    pub local_continuity: LocalContinuity,
    /// Lifecycle.
    pub lifecycle: NoticeLifecycle,
    /// Derived honest effective freshness.
    pub effective_freshness: EffectiveFreshnessClass,
    /// Effective-freshness label.
    pub effective_freshness_label: String,
    /// Whether the effective freshness was downgraded from current.
    pub freshness_downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<DowngradeReasonClass>,
    /// Whether a boundary change is unresolved.
    pub boundary_change_unresolved: bool,
    /// The single honesty bit the chrome reads.
    pub honesty_marker_present: bool,
    /// Summary counters.
    pub summary_counts: ContinuityNoticeSummary,
    /// Display copy + invariant flags.
    pub display_copy: DisplayCopy,
    /// Canonical durable history object ref.
    pub history_ref: String,
    /// Canonical durable support-export object ref.
    pub support_export_ref: String,
    /// Opaque evidence refs.
    pub evidence_refs: Vec<String>,
    /// Repo-relative narrative refs.
    pub narrative_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Build errors
// ---------------------------------------------------------------------------

/// Reason a [`ContinuityNoticeView`] could not be built from its inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewBuildError {
    /// `view_id` was empty.
    EmptyViewId,
    /// `notice_id` was empty.
    EmptyNoticeId,
    /// `as_of` was empty.
    AsOfEmpty,
    /// `title` was empty or too long.
    InvalidTitle,
    /// A reviewable sentence was empty or too long.
    InvalidSentence(String),
    /// `history_ref` is not a canonical durable object ref.
    HistoryRefNotCanonical(String),
    /// `support_export_ref` is not a canonical durable object ref.
    SupportExportRefNotCanonical(String),
    /// A boundary axis appears more than once.
    DuplicateBoundaryAxis(String),
    /// At least one boundary axis row is required.
    BoundaryAxesEmpty,
    /// A changed/unknown axis must carry a current ref.
    ChangedAxisMissingCurrentRef(String),
    /// `boundary_change_required` is true but no axis is changed/unknown.
    BoundaryChangeWithoutChangedAxis,
    /// A changed/unknown axis exists but `boundary_change_required` is false.
    ChangedAxisWithoutBoundaryChange(String),
    /// A preserved (queued / local-draft) write lacks a queue / intent ref.
    PreservedWriteMissingQueueRef(String),
    /// A duplicate blocked-write action class.
    DuplicateBlockedWrite(String),
    /// At least one retained local-safe capability is required.
    RetainedCapabilitiesEmpty,
    /// At least one narrative ref is required.
    NarrativeRefsEmpty,
}

impl std::fmt::Display for ViewBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyViewId => write!(f, "view_id must not be empty"),
            Self::EmptyNoticeId => write!(f, "notice_id must not be empty"),
            Self::AsOfEmpty => write!(f, "as_of must not be empty"),
            Self::InvalidTitle => write!(f, "title must be 1..={MAX_TITLE_CHARS} chars"),
            Self::InvalidSentence(what) => {
                write!(f, "{what} must be a 1..={MAX_SENTENCE_CHARS}-char sentence")
            }
            Self::HistoryRefNotCanonical(r) => {
                write!(f, "history_ref {r:?} is not a canonical durable object ref")
            }
            Self::SupportExportRefNotCanonical(r) => write!(
                f,
                "support_export_ref {r:?} is not a canonical durable object ref",
            ),
            Self::DuplicateBoundaryAxis(a) => write!(f, "duplicate boundary axis: {a}"),
            Self::BoundaryAxesEmpty => write!(f, "boundary_change.axes must not be empty"),
            Self::ChangedAxisMissingCurrentRef(a) => write!(
                f,
                "boundary axis {a} is changed/unknown but carries no current_ref",
            ),
            Self::BoundaryChangeWithoutChangedAxis => write!(
                f,
                "boundary_change_required is true but no axis is changed or unknown",
            ),
            Self::ChangedAxisWithoutBoundaryChange(a) => write!(
                f,
                "boundary axis {a} is changed/unknown but boundary_change_required is false",
            ),
            Self::PreservedWriteMissingQueueRef(a) => write!(
                f,
                "preserved (queued/local-draft) write {a} must carry a queue_or_intent_ref",
            ),
            Self::DuplicateBlockedWrite(a) => write!(f, "duplicate blocked-write action: {a}"),
            Self::RetainedCapabilitiesEmpty => write!(
                f,
                "local_continuity.retained_local_safe_capabilities must not be empty",
            ),
            Self::NarrativeRefsEmpty => write!(f, "narrative_refs must not be empty"),
        }
    }
}

impl std::error::Error for ViewBuildError {}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

fn check_sentence(value: &str, what: &str) -> Result<(), ViewBuildError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > MAX_SENTENCE_CHARS {
        return Err(ViewBuildError::InvalidSentence(what.to_owned()));
    }
    Ok(())
}

impl ContinuityNoticeView {
    /// Build a continuity-notice view from typed inputs.
    ///
    /// `as_of` is the chrome's "now", used to derive the refresh-age bucket and
    /// therefore the no-silent-current downgrade.
    pub fn build(
        input: ContinuityNoticeInput,
        as_of: impl Into<String>,
    ) -> Result<Self, ViewBuildError> {
        let as_of = as_of.into();
        if input.view_id.trim().is_empty() {
            return Err(ViewBuildError::EmptyViewId);
        }
        if input.notice_id.trim().is_empty() {
            return Err(ViewBuildError::EmptyNoticeId);
        }
        if as_of.trim().is_empty() {
            return Err(ViewBuildError::AsOfEmpty);
        }
        let title = input.title.trim();
        if title.is_empty() || title.chars().count() > MAX_TITLE_CHARS {
            return Err(ViewBuildError::InvalidTitle);
        }
        check_sentence(&input.summary, "summary")?;
        check_sentence(&input.affected_scope.scope_summary, "scope_summary")?;
        check_sentence(&input.boundary_change.summary, "boundary summary")?;
        check_sentence(
            &input.local_continuity.continuity_summary,
            "continuity_summary",
        )?;

        if !is_canonical_object_ref(&input.history_ref) {
            return Err(ViewBuildError::HistoryRefNotCanonical(input.history_ref));
        }
        if !is_canonical_object_ref(&input.support_export_ref) {
            return Err(ViewBuildError::SupportExportRefNotCanonical(
                input.support_export_ref,
            ));
        }

        // ---- Boundary change ------------------------------------------------
        if input.boundary_change.axes.is_empty() {
            return Err(ViewBuildError::BoundaryAxesEmpty);
        }
        let mut seen_axes = std::collections::BTreeSet::new();
        let mut changed_axis_count = 0u32;
        let mut unknown_axis_count = 0u32;
        for axis in &input.boundary_change.axes {
            if !seen_axes.insert(axis.axis_class) {
                return Err(ViewBuildError::DuplicateBoundaryAxis(
                    axis.axis_class.as_str().to_owned(),
                ));
            }
            check_sentence(&axis.summary, "boundary axis summary")?;
            if axis.axis_state_class.is_meaningful_change() {
                if axis.current_ref.as_deref().map(is_canonical_object_ref) != Some(true) {
                    return Err(ViewBuildError::ChangedAxisMissingCurrentRef(
                        axis.axis_class.as_str().to_owned(),
                    ));
                }
                if !input.boundary_change.boundary_change_required {
                    return Err(ViewBuildError::ChangedAxisWithoutBoundaryChange(
                        axis.axis_class.as_str().to_owned(),
                    ));
                }
                match axis.axis_state_class {
                    BoundaryAxisStateClass::Changed => changed_axis_count += 1,
                    BoundaryAxisStateClass::UnknownRecheckRequired => unknown_axis_count += 1,
                    _ => {}
                }
            }
        }
        if input.boundary_change.boundary_change_required
            && changed_axis_count == 0
            && unknown_axis_count == 0
        {
            return Err(ViewBuildError::BoundaryChangeWithoutChangedAxis);
        }
        let boundary_change_unresolved = input.boundary_change.boundary_change_required
            && !input.boundary_change.review_completed
            && (changed_axis_count > 0 || unknown_axis_count > 0);

        let axes: Vec<BoundaryAxisRow> = input
            .boundary_change
            .axes
            .iter()
            .map(|a| BoundaryAxisRow {
                axis_class: a.axis_class,
                axis_label: a.axis_class.label().to_owned(),
                axis_state_class: a.axis_state_class,
                axis_state_label: a.axis_state_class.label().to_owned(),
                previous_ref: a.previous_ref.clone(),
                current_ref: a.current_ref.clone(),
                summary: a.summary.trim().to_owned(),
            })
            .collect();
        let boundary_change = BoundaryChange {
            boundary_change_required: input.boundary_change.boundary_change_required,
            review_completed: input.boundary_change.review_completed,
            boundary_change_unresolved,
            changed_axis_count,
            unknown_axis_count,
            axes,
            summary: input.boundary_change.summary.trim().to_owned(),
        };

        // ---- Blocked writes -------------------------------------------------
        let mut seen_writes = std::collections::BTreeSet::new();
        let mut blocked_writes = Vec::with_capacity(input.blocked_writes.len());
        let mut queued_publish_later_count = 0u32;
        let mut local_draft_preserved_count = 0u32;
        let mut blocked_no_safe_retry_count = 0u32;
        let mut requires_manual_rerun_count = 0u32;
        for w in &input.blocked_writes {
            if !seen_writes.insert(w.action_class) {
                return Err(ViewBuildError::DuplicateBlockedWrite(
                    w.action_class.as_str().to_owned(),
                ));
            }
            check_sentence(&w.note, "blocked-write note")?;
            let intent_preserved = w.continuity_posture.is_preserved();
            if w.continuity_posture.requires_queue_ref()
                && w.queue_or_intent_ref
                    .as_deref()
                    .map(is_canonical_object_ref)
                    != Some(true)
            {
                return Err(ViewBuildError::PreservedWriteMissingQueueRef(
                    w.action_class.as_str().to_owned(),
                ));
            }
            match w.continuity_posture {
                WriteContinuityPostureClass::QueuedPublishLater => queued_publish_later_count += 1,
                WriteContinuityPostureClass::LocalDraftPreserved => {
                    local_draft_preserved_count += 1
                }
                WriteContinuityPostureClass::BlockedNoSafeRetry => blocked_no_safe_retry_count += 1,
                WriteContinuityPostureClass::RequiresManualRerun => {
                    requires_manual_rerun_count += 1
                }
                _ => {}
            }
            blocked_writes.push(BlockedWriteRow {
                action_class: w.action_class,
                block_state_class: w.block_state_class,
                continuity_posture: w.continuity_posture,
                continuity_posture_label: w.continuity_posture.label().to_owned(),
                safer_guidance: w.safer_guidance,
                safer_guidance_label: w.safer_guidance.label().to_owned(),
                queue_or_intent_ref: w.queue_or_intent_ref.clone(),
                idempotency_key_present: w.idempotency_key_present,
                intent_preserved,
                resume_trigger: w.resume_trigger,
                note: w.note.trim().to_owned(),
            });
        }
        blocked_writes.sort_by(|a, b| a.action_class.cmp(&b.action_class));

        // ---- Succeeded hosted mutations ------------------------------------
        let mut succeeded = Vec::with_capacity(input.succeeded_hosted_mutations.len());
        for m in &input.succeeded_hosted_mutations {
            check_sentence(&m.note, "hosted-mutation note")?;
            succeeded.push(HostedMutationRow {
                action_class: m.action_class,
                result_ref: m.result_ref.clone(),
                completed_at: m.completed_at.clone(),
                note: m.note.trim().to_owned(),
            });
        }
        succeeded.sort_by(|a, b| a.action_class.cmp(&b.action_class));

        // ---- Local continuity ----------------------------------------------
        if input
            .local_continuity
            .retained_local_safe_capabilities
            .is_empty()
        {
            return Err(ViewBuildError::RetainedCapabilitiesEmpty);
        }
        for cap in &input.local_continuity.retained_local_safe_capabilities {
            check_sentence(cap, "retained local-safe capability")?;
        }
        let local_continuity = LocalContinuity {
            local_core_status: input.local_continuity.local_core_status,
            local_core_status_label: input.local_continuity.local_core_status.label().to_owned(),
            retained_local_safe_capabilities: input
                .local_continuity
                .retained_local_safe_capabilities
                .iter()
                .map(|s| s.trim().to_owned())
                .collect(),
            continue_local_guidance_required: input
                .local_continuity
                .continue_local_guidance_required,
            continuity_summary: input.local_continuity.continuity_summary.trim().to_owned(),
        };

        if input.narrative_refs.is_empty() {
            return Err(ViewBuildError::NarrativeRefsEmpty);
        }

        // ---- Schedule + freshness derivation -------------------------------
        let refresh_age = match input.schedule.latest_refresh_at.as_deref() {
            Some(at) => derive_refresh_age(at, &as_of),
            None => RefreshAgeClass::Never,
        };
        let (effective_freshness, downgrade_reasons) =
            derive_effective_freshness(input.lifecycle.freshness_class, refresh_age);
        let freshness_downgraded = !effective_freshness.is_current();

        let schedule = NoticeSchedule {
            time_basis: input.schedule.time_basis,
            time_basis_label: input.schedule.time_basis.as_str().to_owned(),
            starts_at: input.schedule.starts_at.clone(),
            expected_or_actual_ends_at: input.schedule.expected_or_actual_ends_at.clone(),
            completed_at: input.schedule.completed_at.clone(),
            timezone_id: input.schedule.timezone_id.clone(),
            utc_offset_at_start: input.schedule.utc_offset_at_start.clone(),
            latest_refresh_at: input.schedule.latest_refresh_at.clone(),
            refresh_age,
            refresh_age_label: refresh_age.label().to_owned(),
        };

        let lifecycle = NoticeLifecycle {
            freshness_class: input.lifecycle.freshness_class,
            freshness_label: input.lifecycle.freshness_class.label().to_owned(),
            supersedes_id: input.lifecycle.supersedes_id.clone(),
            superseded_by_id: input.lifecycle.superseded_by_id.clone(),
            retained_until_at: input.lifecycle.retained_until_at.clone(),
            history_refs: input.lifecycle.history_refs.clone(),
        };

        let summary_counts = ContinuityNoticeSummary {
            blocked_write_count: blocked_writes.len() as u32,
            queued_publish_later_count,
            local_draft_preserved_count,
            preserved_intent_count: queued_publish_later_count + local_draft_preserved_count,
            blocked_no_safe_retry_count,
            requires_manual_rerun_count,
            succeeded_hosted_mutation_count: succeeded.len() as u32,
            changed_boundary_axis_count: changed_axis_count,
            unknown_boundary_axis_count: unknown_axis_count,
        };

        let honesty_marker_present = freshness_downgraded || boundary_change_unresolved;

        let kind = input.notice_kind;
        let affected_scope = AffectedScope {
            deployment_profiles: input.affected_scope.deployment_profiles.clone(),
            tenant_refs: input.affected_scope.tenant_refs.clone(),
            region_refs: input.affected_scope.region_refs.clone(),
            residency_scope_classes: input.affected_scope.residency_scope_classes.clone(),
            service_classes: input.affected_scope.service_classes.clone(),
            scope_summary: input.affected_scope.scope_summary.trim().to_owned(),
        };

        let display_copy = build_display_copy(
            kind,
            &input.summary,
            &schedule,
            &affected_scope,
            &boundary_change,
            &summary_counts,
            &local_continuity,
            effective_freshness,
            &downgrade_reasons,
        );

        Ok(Self {
            record_kind: CONTINUITY_NOTICE_VIEW_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_NOTICE_VIEW_SCHEMA_VERSION,
            notice: CONTINUITY_NOTICE_NOTICE.to_owned(),
            view_id: input.view_id.clone(),
            notice_id: input.notice_id.clone(),
            as_of,
            notice_kind: kind,
            notice_kind_label: kind.label().to_owned(),
            category: kind.category(),
            category_label: kind.category().label().to_owned(),
            plan_class: kind.plan_class(),
            plan_class_label: kind.plan_class().label().to_owned(),
            title: title.to_owned(),
            summary: input.summary.trim().to_owned(),
            created_at: input.created_at.clone(),
            updated_at: input.updated_at.clone(),
            schedule,
            affected_scope,
            boundary_change,
            blocked_writes,
            succeeded_hosted_mutations: succeeded,
            local_continuity,
            lifecycle,
            effective_freshness,
            effective_freshness_label: effective_freshness.label().to_owned(),
            freshness_downgraded,
            downgrade_reasons,
            boundary_change_unresolved,
            honesty_marker_present,
            summary_counts,
            display_copy,
            history_ref: input.history_ref.clone(),
            support_export_ref: input.support_export_ref.clone(),
            evidence_refs: input.evidence_refs.clone(),
            narrative_refs: input.narrative_refs.clone(),
        })
    }

    /// Deterministic plaintext block for support exports and reviewer previews.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Maintenance & failover continuity notice\n");
        out.push_str(&format!("View: {}\n", self.view_id));
        out.push_str(&format!("Notice: {}\n", self.notice_id));
        out.push_str(&format!(
            "Kind: {} ({}) | Category: {} | Plan: {}\n",
            self.notice_kind.label(),
            self.notice_kind.as_str(),
            self.category.as_str(),
            self.plan_class.as_str(),
        ));
        out.push_str(&format!("As of: {}\n", self.as_of));
        out.push_str(&format!(
            "Freshness: {} ({}) declared={}\n",
            self.effective_freshness.label(),
            self.effective_freshness.as_str(),
            self.lifecycle.freshness_class.as_str(),
        ));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        if let Some(label) = &self.display_copy.stale_label {
            out.push_str(&format!("Stale: {label}\n"));
        }
        out.push_str(&format!(
            "Window: {} -> {} {} ({})\n",
            self.schedule.starts_at,
            self.schedule
                .expected_or_actual_ends_at
                .as_deref()
                .unwrap_or("(open)"),
            self.schedule.timezone_id,
            self.schedule.utc_offset_at_start,
        ));
        out.push_str(&format!(
            "Refresh: {} ({})\n",
            self.schedule
                .latest_refresh_at
                .as_deref()
                .unwrap_or("(none)"),
            self.schedule.refresh_age.as_str(),
        ));
        out.push_str(&format!("Scope: {}\n", self.affected_scope.scope_summary));
        out.push('\n');
        out.push_str(&format!(
            "Blocked writes ({}): queued_publish_later={}, local_draft={}, no_safe_retry={}, manual_rerun={}\n",
            self.summary_counts.blocked_write_count,
            self.summary_counts.queued_publish_later_count,
            self.summary_counts.local_draft_preserved_count,
            self.summary_counts.blocked_no_safe_retry_count,
            self.summary_counts.requires_manual_rerun_count,
        ));
        for w in &self.blocked_writes {
            let preserved = if w.intent_preserved {
                "preserved"
            } else {
                "not-preserved"
            };
            out.push_str(&format!(
                "- {} [{preserved}] posture={} block={} guidance={} resume={} -> {}\n",
                w.action_class.as_str(),
                w.continuity_posture.as_str(),
                w.block_state_class.as_str(),
                w.safer_guidance.as_str(),
                w.resume_trigger.as_str(),
                w.queue_or_intent_ref.as_deref().unwrap_or("(none)"),
            ));
        }
        out.push('\n');
        out.push_str(&format!(
            "Succeeded hosted mutations ({}):\n",
            self.summary_counts.succeeded_hosted_mutation_count,
        ));
        for m in &self.succeeded_hosted_mutations {
            out.push_str(&format!(
                "- {} -> {} ({})\n",
                m.action_class.as_str(),
                m.result_ref,
                m.completed_at,
            ));
        }
        out.push('\n');
        out.push_str(&format!(
            "Boundary change: required={}, unresolved={}, changed_axes={}, unknown_axes={}\n",
            self.boundary_change.boundary_change_required,
            self.boundary_change.boundary_change_unresolved,
            self.boundary_change.changed_axis_count,
            self.boundary_change.unknown_axis_count,
        ));
        for a in &self.boundary_change.axes {
            out.push_str(&format!(
                "- {} {} prev={} cur={}\n",
                a.axis_class.as_str(),
                a.axis_state_class.as_str(),
                a.previous_ref.as_deref().unwrap_or("(none)"),
                a.current_ref.as_deref().unwrap_or("(none)"),
            ));
        }
        out.push('\n');
        out.push_str(&format!(
            "Local continuity: {} | continue_local_guidance={}\n",
            self.local_continuity.local_core_status.as_str(),
            self.local_continuity.continue_local_guidance_required,
        ));
        for cap in &self.local_continuity.retained_local_safe_capabilities {
            out.push_str(&format!("- local-safe: {cap}\n"));
        }
        out.push('\n');
        out.push_str(&format!("History: {}\n", self.history_ref));
        out.push_str(&format!("Support export: {}\n", self.support_export_ref));
        out
    }
}

#[allow(clippy::too_many_arguments)]
fn build_display_copy(
    kind: NoticeKindClass,
    summary: &str,
    schedule: &NoticeSchedule,
    scope: &AffectedScope,
    boundary: &BoundaryChange,
    counts: &ContinuityNoticeSummary,
    local: &LocalContinuity,
    effective_freshness: EffectiveFreshnessClass,
    downgrade_reasons: &[DowngradeReasonClass],
) -> DisplayCopy {
    let primary_status_line = format!("{}: {}", kind.label(), summary.trim());
    let schedule_line = format!(
        "{} starts {} (ends {}) {} {}",
        kind.label(),
        schedule.starts_at,
        schedule
            .expected_or_actual_ends_at
            .as_deref()
            .unwrap_or("open"),
        schedule.timezone_id,
        schedule.utc_offset_at_start,
    );
    let scope_line = scope.scope_summary.clone();
    let blocked_writes_line = format!(
        "{} write classes affected this window.",
        counts.blocked_write_count,
    );
    let queued_preserved_line = format!(
        "{} intents preserved ({} publish-later queued, {} local drafts); they survive the window.",
        counts.preserved_intent_count,
        counts.queued_publish_later_count,
        counts.local_draft_preserved_count,
    );
    let succeeded_line = format!(
        "{} hosted mutations already landed and are listed separately from queued work.",
        counts.succeeded_hosted_mutation_count,
    );
    let local_continuity_line = local.continuity_summary.clone();
    let boundary_change_line = if boundary.boundary_change_required {
        format!(
            "Boundary change: {} changed, {} need recheck — {}",
            boundary.changed_axis_count, boundary.unknown_axis_count, boundary.summary,
        )
    } else {
        "No tenant / region / endpoint boundary change.".to_owned()
    };
    let freshness_line = if effective_freshness.is_current() {
        "This notice is current.".to_owned()
    } else {
        let reasons: Vec<&str> = downgrade_reasons.iter().map(|r| r.label()).collect();
        format!(
            "This notice is {} — {}.",
            effective_freshness.label(),
            reasons.join("; "),
        )
    };
    let follow_up_line = if counts.blocked_no_safe_retry_count > 0 {
        "Some writes have no safe retry — export or escalate before the window.".to_owned()
    } else if counts.requires_manual_rerun_count > 0 {
        "Some writes need a manual rerun after the window.".to_owned()
    } else if counts.preserved_intent_count > 0 {
        "Queued and local-draft work will replay when the window resumes.".to_owned()
    } else {
        "Continue local work; managed writes resume when the window ends.".to_owned()
    };
    let stale_label = if effective_freshness.is_current() {
        None
    } else {
        Some(format!(
            "This continuity notice is {} — do not read it as current operational truth; re-check live service health.",
            effective_freshness.label(),
        ))
    };

    DisplayCopy {
        primary_status_line,
        schedule_line,
        scope_line,
        blocked_writes_line,
        queued_preserved_line,
        succeeded_line,
        local_continuity_line,
        boundary_change_line,
        freshness_line,
        follow_up_line,
        stale_label,
        all_work_broken_implied: false,
        incident_language_for_planned_used: false,
        generic_degraded_banner_used: false,
        queued_and_succeeded_collapsed: false,
        stale_presented_as_current: false,
        boundary_change_hidden: false,
    }
}

/// Derive the honest effective freshness and the downgrade reasons.
fn derive_effective_freshness(
    declared: FreshnessClass,
    refresh_age: RefreshAgeClass,
) -> (EffectiveFreshnessClass, Vec<DowngradeReasonClass>) {
    match declared {
        FreshnessClass::ActiveCurrent => {
            if refresh_age.is_current() {
                (EffectiveFreshnessClass::Current, Vec::new())
            } else {
                (
                    EffectiveFreshnessClass::RefreshStale,
                    vec![DowngradeReasonClass::RefreshExpired],
                )
            }
        }
        FreshnessClass::SupersededStale => (
            EffectiveFreshnessClass::SupersededStale,
            vec![DowngradeReasonClass::NoticeSuperseded],
        ),
        FreshnessClass::CompletedHistorical => (
            EffectiveFreshnessClass::CompletedHistorical,
            vec![DowngradeReasonClass::WindowCompleted],
        ),
        FreshnessClass::ImportedHistorical => (
            EffectiveFreshnessClass::ImportedHistorical,
            vec![DowngradeReasonClass::ImportedOffline],
        ),
    }
}

fn derive_refresh_age(refresh_at: &str, as_of: &str) -> RefreshAgeClass {
    let last = match parse_timestamp_minutes(refresh_at) {
        Some(v) => v,
        None => return RefreshAgeClass::Never,
    };
    let now = match parse_timestamp_minutes(as_of) {
        Some(v) => v,
        None => return RefreshAgeClass::Never,
    };
    if now < last {
        return RefreshAgeClass::Never;
    }
    let delta = now - last;
    if delta <= 5 {
        RefreshAgeClass::Fresh
    } else if delta <= 60 {
        RefreshAgeClass::Recent
    } else if delta <= 60 * 24 {
        RefreshAgeClass::Stale
    } else {
        RefreshAgeClass::VeryStale
    }
}

fn parse_timestamp_minutes(input: &str) -> Option<i64> {
    let bytes = input.as_bytes();
    if bytes.len() < 16 {
        return None;
    }
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    if bytes[10] != b'T' && bytes[10] != b' ' {
        return None;
    }
    if bytes[13] != b':' {
        return None;
    }
    let year: i64 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    let month: u32 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    let day: u32 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    let hour: u32 = std::str::from_utf8(&bytes[11..13]).ok()?.parse().ok()?;
    let minute: u32 = std::str::from_utf8(&bytes[14..16]).ok()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 {
        return None;
    }
    let day_number = days_from_civil(year, month, day);
    Some(day_number * 24 * 60 + i64::from(hour) * 60 + i64::from(minute))
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let m_i = m as i64;
    let doy = (153 * (if m_i > 2 { m_i - 3 } else { m_i + 9 }) + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_continuity() -> LocalContinuityInput {
        LocalContinuityInput {
            local_core_status: LocalCoreStatusClass::LocalCoreUnaffected,
            retained_local_safe_capabilities: vec![
                "Editing, saving, and local search keep working.".to_owned(),
            ],
            continue_local_guidance_required: true,
            continuity_summary: "Local work continues throughout the window.".to_owned(),
        }
    }

    fn base_input() -> ContinuityNoticeInput {
        ContinuityNoticeInput {
            view_id: "continuity_notice_view:test".to_owned(),
            notice_id: "notice.test".to_owned(),
            notice_kind: NoticeKindClass::ReadOnlyWindow,
            title: "Read-only window".to_owned(),
            summary: "Hosted writes pause; reads and local work continue.".to_owned(),
            created_at: "2026-05-20T08:00:00Z".to_owned(),
            updated_at: "2026-05-20T11:58:00Z".to_owned(),
            schedule: ScheduleInput {
                time_basis: TimeBasisClass::ScheduledExact,
                starts_at: "2026-05-20T12:00:00Z".to_owned(),
                expected_or_actual_ends_at: Some("2026-05-20T13:00:00Z".to_owned()),
                completed_at: None,
                timezone_id: "Europe/Berlin".to_owned(),
                utc_offset_at_start: "+02:00".to_owned(),
                latest_refresh_at: Some("2026-05-20T11:58".to_owned()),
            },
            affected_scope: ScopeInput {
                deployment_profiles: vec![DeploymentProfileClass::ManagedCloud],
                tenant_refs: vec!["tenant.ref.a".to_owned()],
                region_refs: vec!["region.ref.eu".to_owned()],
                residency_scope_classes: vec![ResidencyScopeClass::CustomerRegionPinned],
                service_classes: vec![ServiceClass::ProviderReviewService],
                scope_summary: "Managed-cloud provider review in the EU region.".to_owned(),
            },
            boundary_change: BoundaryChangeInput {
                boundary_change_required: false,
                review_completed: false,
                axes: vec![BoundaryAxisInput {
                    axis_class: BoundaryAxisClass::Tenant,
                    axis_state_class: BoundaryAxisStateClass::Unchanged,
                    previous_ref: None,
                    current_ref: None,
                    summary: "Tenant unchanged.".to_owned(),
                }],
                summary: "No boundary change.".to_owned(),
            },
            blocked_writes: vec![BlockedWriteInput {
                action_class: ManagedActionClass::ManagedReviewCommentPublish,
                block_state_class: BlockStateClass::BlockedReadOnly,
                continuity_posture: WriteContinuityPostureClass::QueuedPublishLater,
                safer_guidance: SaferThanRetryGuidanceClass::RetrySafeWhenResumed,
                queue_or_intent_ref: Some("aureline://publish_later_queue/q-1".to_owned()),
                idempotency_key_present: true,
                resume_trigger: ResumeTriggerClass::WindowEnds,
                note: "Review comments queue for publish-later.".to_owned(),
            }],
            succeeded_hosted_mutations: vec![HostedMutationInput {
                action_class: ManagedActionClass::ManagedReviewApproval,
                result_ref: "aureline://change_review/cr-1".to_owned(),
                completed_at: "2026-05-20T11:50:00Z".to_owned(),
                note: "Approval landed before the window.".to_owned(),
            }],
            local_continuity: local_continuity(),
            lifecycle: LifecycleInput {
                freshness_class: FreshnessClass::ActiveCurrent,
                supersedes_id: None,
                superseded_by_id: None,
                retained_until_at: None,
                history_refs: vec![],
            },
            history_ref: "aureline://continuity_notice_history/notice.test".to_owned(),
            support_export_ref: "aureline://support_export/notice.test".to_owned(),
            evidence_refs: vec!["evidence.ref.test".to_owned()],
            narrative_refs: vec!["docs/ops/m3/maintenance_failover_truth.md".to_owned()],
        }
    }

    #[test]
    fn current_active_notice_has_no_honesty_marker() {
        let view = ContinuityNoticeView::build(base_input(), "2026-05-20T12:00").unwrap();
        assert_eq!(view.effective_freshness, EffectiveFreshnessClass::Current);
        assert!(!view.freshness_downgraded);
        assert!(!view.honesty_marker_present);
        assert!(view.display_copy.stale_label.is_none());
        assert_eq!(view.category, NoticeCategoryClass::Drain);
        assert_eq!(view.summary_counts.queued_publish_later_count, 1);
        assert_eq!(view.summary_counts.preserved_intent_count, 1);
        assert_eq!(view.summary_counts.succeeded_hosted_mutation_count, 1);
    }

    #[test]
    fn stale_refresh_downgrades_active_notice() {
        let mut input = base_input();
        input.schedule.latest_refresh_at = Some("2026-05-20T06:00".to_owned());
        let view = ContinuityNoticeView::build(input, "2026-05-20T12:00").unwrap();
        assert_eq!(
            view.effective_freshness,
            EffectiveFreshnessClass::RefreshStale
        );
        assert!(view.freshness_downgraded);
        assert!(view.honesty_marker_present);
        assert_eq!(
            view.downgrade_reasons,
            vec![DowngradeReasonClass::RefreshExpired]
        );
        assert!(view.display_copy.stale_label.is_some());
    }

    #[test]
    fn superseded_notice_cannot_read_as_current() {
        let mut input = base_input();
        input.lifecycle.freshness_class = FreshnessClass::SupersededStale;
        input.lifecycle.superseded_by_id = Some("notice.test.v2".to_owned());
        // Even with a fresh refresh, a superseded notice downgrades.
        let view = ContinuityNoticeView::build(input, "2026-05-20T12:00").unwrap();
        assert_eq!(
            view.effective_freshness,
            EffectiveFreshnessClass::SupersededStale
        );
        assert!(view.honesty_marker_present);
        assert!(view.display_copy.stale_label.is_some());
    }

    #[test]
    fn changed_boundary_marks_unresolved_and_requires_current_ref() {
        let mut input = base_input();
        input.notice_kind = NoticeKindClass::RegionalFailover;
        input.boundary_change = BoundaryChangeInput {
            boundary_change_required: true,
            review_completed: false,
            axes: vec![BoundaryAxisInput {
                axis_class: BoundaryAxisClass::Region,
                axis_state_class: BoundaryAxisStateClass::Changed,
                previous_ref: Some("region.ref.eu".to_owned()),
                current_ref: Some("aureline://region/eu-2".to_owned()),
                summary: "Region moved to the failover region.".to_owned(),
            }],
            summary: "Region boundary changed.".to_owned(),
        };
        let view = ContinuityNoticeView::build(input, "2026-05-20T12:00").unwrap();
        assert!(view.boundary_change_unresolved);
        assert!(view.honesty_marker_present);
        assert_eq!(view.summary_counts.changed_boundary_axis_count, 1);
        assert_eq!(view.plan_class, PlanClass::Emergency);
        assert_eq!(view.category, NoticeCategoryClass::Failover);
    }

    #[test]
    fn changed_axis_without_current_ref_is_rejected() {
        let mut input = base_input();
        input.boundary_change = BoundaryChangeInput {
            boundary_change_required: true,
            review_completed: false,
            axes: vec![BoundaryAxisInput {
                axis_class: BoundaryAxisClass::Tenant,
                axis_state_class: BoundaryAxisStateClass::Changed,
                previous_ref: Some("tenant.ref.a".to_owned()),
                current_ref: None,
                summary: "Tenant changed but no current ref.".to_owned(),
            }],
            summary: "Tenant boundary changed.".to_owned(),
        };
        let err = ContinuityNoticeView::build(input, "2026-05-20T12:00").unwrap_err();
        assert!(matches!(
            err,
            ViewBuildError::ChangedAxisMissingCurrentRef(_)
        ));
    }

    #[test]
    fn preserved_write_requires_queue_ref() {
        let mut input = base_input();
        input.blocked_writes[0].queue_or_intent_ref = None;
        let err = ContinuityNoticeView::build(input, "2026-05-20T12:00").unwrap_err();
        assert!(matches!(
            err,
            ViewBuildError::PreservedWriteMissingQueueRef(_)
        ));
    }

    #[test]
    fn completed_notice_keeps_boundary_visible() {
        let mut input = base_input();
        input.notice_kind = NoticeKindClass::PostEventReconciliation;
        input.lifecycle.freshness_class = FreshnessClass::CompletedHistorical;
        input.boundary_change = BoundaryChangeInput {
            boundary_change_required: true,
            review_completed: true,
            axes: vec![BoundaryAxisInput {
                axis_class: BoundaryAxisClass::Tenant,
                axis_state_class: BoundaryAxisStateClass::Changed,
                previous_ref: Some("tenant.ref.a".to_owned()),
                current_ref: Some("aureline://tenant/b".to_owned()),
                summary: "Tenant moved.".to_owned(),
            }],
            summary: "Tenant boundary changed and was reviewed.".to_owned(),
        };
        let view = ContinuityNoticeView::build(input, "2026-05-20T12:00").unwrap();
        // Review completed -> not unresolved, but the changed identity stays
        // visible and freshness is historical (honesty marker still on).
        assert!(!view.boundary_change_unresolved);
        assert_eq!(view.boundary_change.changed_axis_count, 1);
        assert!(!view.display_copy.boundary_change_hidden);
        assert_eq!(
            view.effective_freshness,
            EffectiveFreshnessClass::CompletedHistorical
        );
        assert!(view.honesty_marker_present);
    }

    #[test]
    fn roundtrips_through_json() {
        let view = ContinuityNoticeView::build(base_input(), "2026-05-20T12:00").unwrap();
        let json = serde_json::to_string_pretty(&view).unwrap();
        let back: ContinuityNoticeView = serde_json::from_str(&json).unwrap();
        assert_eq!(view, back);
    }
}
