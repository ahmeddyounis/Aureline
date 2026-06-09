//! Shared service-health feed contract for desktop, CLI/headless, Help/About,
//! diagnostics, support export, release notes, migration notices, and handoff
//! packets.
//!
//! The product already has multiple service-health-adjacent surfaces: the shell
//! aggregator, Help/About destination truth, diagnostics-center summaries,
//! maintenance and failover notices, and support/export packets. This module
//! freezes the one metadata-safe feed object those surfaces are expected to
//! consume or project into so outage scope, freshness, source provenance, and
//! local-only continuity cannot drift per surface.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable schema version for shared service-health feed objects.
pub const SERVICE_HEALTH_FEED_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ServiceHealthFeed`].
pub const SERVICE_HEALTH_FEED_RECORD_KIND: &str = "service_health_feed_record";

/// Stable record-kind tag for [`ServiceHealthFeedItem`].
pub const SERVICE_HEALTH_FEED_ITEM_RECORD_KIND: &str = "service_health_feed_item_record";

/// Stable record-kind tag for [`ServiceHealthFeedSupportExport`].
pub const SERVICE_HEALTH_FEED_SUPPORT_EXPORT_RECORD_KIND: &str =
    "service_health_feed_support_export_record";

/// Stable shared-contract ref quoted by every participating surface.
pub const SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF: &str = "service_health_feed:v1";

/// Stable schema ref for the shared feed contract.
pub const SERVICE_HEALTH_FEED_SCHEMA_REF: &str = "schemas/help/service-health-feed.schema.json";

/// Stable ref for the canonical checked-in feed fixture.
pub const SERVICE_HEALTH_FEED_CANONICAL_FIXTURE_REF: &str =
    "fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/canonical_feed.json";

/// Contract-state vocabulary shared across service-health surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthContractState {
    /// Live checks are current and the service family is behaving as claimed.
    Ready,
    /// The family is usable with reduced capacity or partial impact.
    Degraded,
    /// Managed reachability is absent, but a productive local path remains.
    LocalOnly,
    /// The rendered status comes from visibly stale data.
    Stale,
    /// The observed payload or route does not match the admitted contract.
    ContractMismatch,
    /// Policy prevents the family from operating or publishing.
    PolicyBlocked,
    /// The family is unavailable for the named workflows.
    Unavailable,
    /// A planned window is scheduled but not yet active.
    Scheduled,
    /// Reads remain available while writes are blocked.
    ReadOnly,
    /// New writes are paused while existing work drains.
    Drain,
    /// A tenant, endpoint, or runtime migration is underway.
    Migration,
    /// Failover is active.
    Failover,
    /// Post-window reconciliation is in progress.
    Reconciling,
    /// The incident or maintenance window resolved, but the surface keeps the
    /// record visible for copy/export continuity.
    Resolved,
}

impl ServiceHealthContractState {
    /// Every stable token, in canonical order.
    pub const ALL: [Self; 14] = [
        Self::Ready,
        Self::Degraded,
        Self::LocalOnly,
        Self::Stale,
        Self::ContractMismatch,
        Self::PolicyBlocked,
        Self::Unavailable,
        Self::Scheduled,
        Self::ReadOnly,
        Self::Drain,
        Self::Migration,
        Self::Failover,
        Self::Reconciling,
        Self::Resolved,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::LocalOnly => "local_only",
            Self::Stale => "stale",
            Self::ContractMismatch => "contract_mismatch",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
            Self::Scheduled => "scheduled",
            Self::ReadOnly => "read_only",
            Self::Drain => "drain",
            Self::Migration => "migration",
            Self::Failover => "failover",
            Self::Reconciling => "reconciling",
            Self::Resolved => "resolved",
        }
    }

    /// Returns true when the state preserves an explicitly healthy or
    /// post-incident lane for unaffected work.
    pub const fn preserves_healthy_lane(self) -> bool {
        matches!(self, Self::Ready | Self::Resolved)
    }
}

/// Provenance class describing where the currently rendered state came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthSourceClass {
    /// A current live probe or poll produced the state.
    LivePolling,
    /// The surface is rendering cached data.
    CachedData,
    /// The surface is rendering a mirrored or relayed notice.
    MirroredNotice,
    /// The surface is rendering an offline bundle or installed pack.
    OfflineBundle,
}

impl ServiceHealthSourceClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LivePolling => "live_polling",
            Self::CachedData => "cached_data",
            Self::MirroredNotice => "mirrored_notice",
            Self::OfflineBundle => "offline_bundle",
        }
    }

    /// Whether the source class must not imply live reachability.
    pub const fn forbids_live_reachability(self) -> bool {
        !matches!(self, Self::LivePolling)
    }
}

/// Scoped outage posture for one feed item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthOutageScope {
    /// No outage is currently in effect for this family.
    None,
    /// One family is affected while neighboring families remain healthy.
    SingleService,
    /// Several families are affected, but unaffected families stay visibly
    /// healthy.
    PartialService,
    /// The affected workflows span the full product boundary.
    FullProduct,
    /// The state is a planned maintenance or continuity window.
    MaintenanceWindow,
}

impl ServiceHealthOutageScope {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SingleService => "single_service",
            Self::PartialService => "partial_service",
            Self::FullProduct => "full_product",
            Self::MaintenanceWindow => "maintenance_window",
        }
    }
}

/// Surface class that participates in the shared feed contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthSurface {
    /// Desktop shell chrome.
    DesktopUi,
    /// CLI/headless output.
    CliHeadless,
    /// About surface.
    About,
    /// Help surface.
    Help,
    /// Service-health panel or banner.
    ServiceHealth,
    /// Diagnostics center or inspector summary.
    Diagnostics,
    /// Support export or bundle preview.
    SupportExport,
    /// Release notes or release center copy.
    ReleaseNotes,
    /// Migration notice or wizard copy.
    MigrationNotice,
    /// Issue-report template or report builder.
    IssueReportTemplate,
    /// Community or support handoff chooser.
    CommunityHandoff,
}

impl ServiceHealthSurface {
    /// Surfaces this contract is required to cover.
    pub const REQUIRED: [Self; 11] = [
        Self::DesktopUi,
        Self::CliHeadless,
        Self::About,
        Self::Help,
        Self::ServiceHealth,
        Self::Diagnostics,
        Self::SupportExport,
        Self::ReleaseNotes,
        Self::MigrationNotice,
        Self::IssueReportTemplate,
        Self::CommunityHandoff,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::About => "about",
            Self::Help => "help",
            Self::ServiceHealth => "service_health",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::ReleaseNotes => "release_notes",
            Self::MigrationNotice => "migration_notice",
            Self::IssueReportTemplate => "issue_report_template",
            Self::CommunityHandoff => "community_handoff",
        }
    }
}

/// Freshness block shared across feed items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFreshness {
    /// Stable freshness ref.
    pub freshness_ref: String,
    /// Where the rendered status came from.
    pub source_class: ServiceHealthSourceClass,
    /// UTC timestamp the source was last checked or mirrored.
    pub last_checked_at: String,
    /// UTC timestamp after which the status must be treated as stale.
    pub stale_after: String,
    /// User-visible freshness label.
    pub visible_freshness_label: String,
    /// Whether the surface may claim live reachability.
    pub live_reachability_claim_allowed: bool,
}

/// One shared service-health feed item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFeedItem {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable item id.
    pub item_id: String,
    /// Stable service-family token.
    pub service_family: String,
    /// Stable boundary-class token.
    pub boundary_class: String,
    /// Contract state carried by every surface.
    pub contract_state: ServiceHealthContractState,
    /// Scoped outage posture.
    pub outage_scope: ServiceHealthOutageScope,
    /// Workflows affected by this state.
    pub affected_workflows: Vec<String>,
    /// Explicitly unaffected workflows that remain healthy or usable.
    pub unaffected_workflows: Vec<String>,
    /// Reviewer-facing summary for copy/export parity.
    pub summary: String,
    /// Shared freshness block.
    pub freshness: ServiceHealthFreshness,
    /// Stable diagnostics or repair action refs.
    pub diagnostics_actions: Vec<String>,
    /// Local-only continuity note shown across surfaces.
    pub local_only_continuity_note: String,
    /// Surfaces expected to render this item verbatim.
    pub surfaced_on: Vec<ServiceHealthSurface>,
}

impl ServiceHealthFeedItem {
    /// Returns the item ref surfaces quote when linking this item.
    pub fn item_ref(&self) -> String {
        self.item_id.clone()
    }

    /// Returns true when the item advertises a cached or offline state.
    pub const fn has_non_live_source(&self) -> bool {
        self.freshness.source_class.forbids_live_reachability()
    }
}

/// Binding proving one surface consumes the shared feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthSurfaceBinding {
    /// Surface consuming the feed.
    pub surface: ServiceHealthSurface,
    /// Shared feed ref.
    pub feed_ref: String,
    /// Feed item refs rendered by the surface.
    pub item_refs: Vec<String>,
    /// Whether the surface consumes the shared feed instead of hand-authoring
    /// surface-local copy.
    pub consumes_shared_feed: bool,
    /// Whether last-checked time is visible.
    pub last_checked_visible: bool,
    /// Whether freshness / stale labeling is visible.
    pub freshness_visible: bool,
    /// Whether the local-only continuity note is visible.
    pub local_only_continuity_visible: bool,
    /// Whether the surface may overclaim live reachability.
    pub may_overclaim_live_reachability: bool,
    /// Whether the rendered state is copyable/exportable without screenshots.
    pub copyable_exportable: bool,
}

/// Top-level shared service-health feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFeed {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable feed id.
    pub feed_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Item rows.
    pub items: Vec<ServiceHealthFeedItem>,
    /// Surface bindings proving shared consumption.
    pub surface_bindings: Vec<ServiceHealthSurfaceBinding>,
}

impl ServiceHealthFeed {
    /// Returns the closed contract-state vocabulary as stable tokens.
    pub fn contract_state_vocabulary(&self) -> Vec<String> {
        ServiceHealthContractState::ALL
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect()
    }

    /// Validates the feed against shared-surface invariants.
    pub fn validate(&self) -> ServiceHealthFeedValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Builds the metadata-safe support/export projection.
    pub fn support_export_projection(&self) -> ServiceHealthFeedSupportExport {
        ServiceHealthFeedSupportExport {
            record_kind: SERVICE_HEALTH_FEED_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
            feed_id: self.feed_id.clone(),
            shared_contract_ref: self.shared_contract_ref.clone(),
            contract_state_vocabulary: self.contract_state_vocabulary(),
            items: self.items.clone(),
            surface_count: self.surface_bindings.len(),
        }
    }
}

/// Coverage observed during validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFeedCoverage {
    /// Contract states currently present in the feed.
    pub contract_states: BTreeSet<ServiceHealthContractState>,
    /// Source classes currently present in the feed.
    pub source_classes: BTreeSet<ServiceHealthSourceClass>,
    /// Outage scopes currently present in the feed.
    pub outage_scopes: BTreeSet<ServiceHealthOutageScope>,
    /// Surfaces covered by feed bindings.
    pub surfaces: BTreeSet<ServiceHealthSurface>,
}

/// Finding severity for feed validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthFeedFindingSeverity {
    /// Error that blocks the feed.
    Error,
    /// Warning a reviewer should inspect.
    Warning,
}

/// One feed validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFeedFinding {
    /// Finding severity.
    pub severity: ServiceHealthFeedFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Reviewer-facing finding message.
    pub message: String,
}

/// Validation report for one shared feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFeedValidationReport {
    /// Feed id under validation.
    pub feed_id: String,
    /// Whether the feed passed validation.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: ServiceHealthFeedCoverage,
    /// Findings emitted while validating.
    pub findings: Vec<ServiceHealthFeedFinding>,
}

/// Metadata-safe support/export projection for a shared feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthFeedSupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Feed id.
    pub feed_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Contract-state vocabulary.
    pub contract_state_vocabulary: Vec<String>,
    /// Shared feed items included in the export.
    pub items: Vec<ServiceHealthFeedItem>,
    /// Number of bound surfaces.
    pub surface_count: usize,
}

/// Returns the canonical feed fixture used by the stable lane.
pub fn canonical_service_health_feed() -> ServiceHealthFeed {
    let all_surfaces = ServiceHealthSurface::REQUIRED.to_vec();
    let items = vec![
        item(
            "feed:local_core",
            "local_core",
            "local_only",
            ServiceHealthContractState::Ready,
            ServiceHealthOutageScope::None,
            &[],
            &["editing", "save", "search", "git", "diagnostics"],
            "Local core workflows remain healthy.",
            freshness(
                "freshness:live-status-feed",
                ServiceHealthSourceClass::LivePolling,
                "2026-06-07T18:20:00Z",
                "2026-06-07T18:35:00Z",
                "Live status checked 2026-06-07T18:20:00Z",
                true,
            ),
            &["diagnostics.open.local_core"],
            "Core local work does not require managed service reachability.",
            &all_surfaces,
        ),
        item(
            "feed:ai_assist",
            "ai_assist",
            "vendor_managed",
            ServiceHealthContractState::Degraded,
            ServiceHealthOutageScope::SingleService,
            &["ai_completion", "ai_chat"],
            &["editing", "search", "git", "diagnostics"],
            "Managed AI is slow, but unaffected local workflows remain healthy.",
            freshness(
                "freshness:live-ai-provider",
                ServiceHealthSourceClass::LivePolling,
                "2026-06-07T18:22:00Z",
                "2026-06-07T18:37:00Z",
                "Live provider health checked 2026-06-07T18:22:00Z",
                true,
            ),
            &["diagnostics.open.ai_provider", "diagnostics.retry.ai_provider"],
            "Use local editing and diagnostics while managed AI retries.",
            &all_surfaces,
        ),
        item(
            "feed:workspace_sync",
            "workspace_sync",
            "local_with_remote_required",
            ServiceHealthContractState::LocalOnly,
            ServiceHealthOutageScope::SingleService,
            &["workspace_sync", "external_publish"],
            &["editing", "review_drafts", "search"],
            "Managed sync is paused; local work and queued drafts remain available.",
            freshness(
                "freshness:sync-live",
                ServiceHealthSourceClass::LivePolling,
                "2026-06-07T18:21:00Z",
                "2026-06-07T18:36:00Z",
                "Live sync check reports local-only continuity",
                true,
            ),
            &["diagnostics.open.workspace_sync", "diagnostics.review.publish_later"],
            "Save locally now and publish after reconnect.",
            &all_surfaces,
        ),
        item(
            "feed:docs_knowledge",
            "docs_knowledge",
            "local_with_remote_optional",
            ServiceHealthContractState::Stale,
            ServiceHealthOutageScope::SingleService,
            &["remote_docs_browse"],
            &["installed_help", "local_docs_search"],
            "Remote docs freshness is stale; installed help and local docs remain usable.",
            freshness(
                "freshness:offline-docs-pack",
                ServiceHealthSourceClass::OfflineBundle,
                "2026-06-06T08:00:00Z",
                "2026-06-13T08:00:00Z",
                "Offline docs pack; live docs service not checked",
                false,
            ),
            &["diagnostics.open.docs_pack"],
            "Installed help pages and local docs search continue from the offline pack.",
            &all_surfaces,
        ),
        item(
            "feed:release_feed",
            "release_feed",
            "public_status",
            ServiceHealthContractState::ContractMismatch,
            ServiceHealthOutageScope::SingleService,
            &["release_notes_live_refresh"],
            &["checked_in_release_notes", "support_export"],
            "Live release-feed payload mismatches the admitted contract; mirrored notes remain inspectable.",
            freshness(
                "freshness:mirrored-release-notes",
                ServiceHealthSourceClass::MirroredNotice,
                "2026-06-07T17:50:00Z",
                "2026-06-14T17:50:00Z",
                "Mirrored release notes shown; live contract mismatch remains visible",
                false,
            ),
            &["diagnostics.open.release_feed"],
            "Use checked-in release notes and mirrored notices until the live feed contract is repaired.",
            &all_surfaces,
        ),
        item(
            "feed:telemetry_upload",
            "telemetry_upload",
            "hosted_optional",
            ServiceHealthContractState::PolicyBlocked,
            ServiceHealthOutageScope::SingleService,
            &["telemetry_upload"],
            &["local_crash_capture", "support_export"],
            "Telemetry upload is policy-blocked; local crash capture and support export remain available.",
            freshness(
                "freshness:telemetry-policy",
                ServiceHealthSourceClass::LivePolling,
                "2026-06-07T18:15:00Z",
                "2026-06-07T18:30:00Z",
                "Live policy state checked 2026-06-07T18:15:00Z",
                true,
            ),
            &["diagnostics.open.telemetry_policy"],
            "Support packets save locally and leave only through explicit submit.",
            &all_surfaces,
        ),
        item(
            "feed:marketplace",
            "marketplace",
            "hosted_optional",
            ServiceHealthContractState::Unavailable,
            ServiceHealthOutageScope::PartialService,
            &["marketplace_browse", "extension_install"],
            &["installed_extensions", "local_workspace_work"],
            "Marketplace is unavailable, but installed extensions and local work remain healthy.",
            freshness(
                "freshness:marketplace-live",
                ServiceHealthSourceClass::LivePolling,
                "2026-06-07T18:17:00Z",
                "2026-06-07T18:32:00Z",
                "Live marketplace outage checked 2026-06-07T18:17:00Z",
                true,
            ),
            &["diagnostics.open.marketplace"],
            "Existing installed extensions remain usable locally.",
            &all_surfaces,
        ),
        item(
            "feed:scheduled-maintenance",
            "managed_sync_window",
            "remote_managed",
            ServiceHealthContractState::Scheduled,
            ServiceHealthOutageScope::MaintenanceWindow,
            &["managed_sync_window_start"],
            &["editing", "draft_review"],
            "A scheduled maintenance window is announced with explicit local continuity.",
            freshness(
                "freshness:maintenance-notice",
                ServiceHealthSourceClass::MirroredNotice,
                "2026-06-07T16:00:00Z",
                "2026-06-08T16:00:00Z",
                "Mirrored maintenance notice refreshed 2026-06-07T16:00:00Z",
                false,
            ),
            &["diagnostics.open.maintenance_notice"],
            "Local work can continue before the window starts.",
            &all_surfaces,
        ),
        item(
            "feed:read-only-window",
            "provider_review",
            "remote_managed",
            ServiceHealthContractState::ReadOnly,
            ServiceHealthOutageScope::MaintenanceWindow,
            &["review_publish", "comment_publish"],
            &["local_review", "draft_capture"],
            "Review surfaces are read-only during the maintenance window.",
            freshness(
                "freshness:read-only-notice",
                ServiceHealthSourceClass::MirroredNotice,
                "2026-06-07T18:05:00Z",
                "2026-06-08T18:05:00Z",
                "Mirrored read-only notice refreshed 2026-06-07T18:05:00Z",
                false,
            ),
            &["diagnostics.open.review_window", "diagnostics.review.publish_later"],
            "Draft comments remain local and copyable until publish resumes.",
            &all_surfaces,
        ),
        item(
            "feed:drain-window",
            "publish_queue",
            "remote_managed",
            ServiceHealthContractState::Drain,
            ServiceHealthOutageScope::MaintenanceWindow,
            &["external_publish"],
            &["local_draft_authoring", "support_export"],
            "New remote publishes are paused while queued work drains.",
            freshness(
                "freshness:drain-notice",
                ServiceHealthSourceClass::MirroredNotice,
                "2026-06-07T18:06:00Z",
                "2026-06-08T18:06:00Z",
                "Mirrored drain notice refreshed 2026-06-07T18:06:00Z",
                false,
            ),
            &["diagnostics.review.publish_later", "diagnostics.open.drain_window"],
            "Save locally now and let queued publishes reconcile after the drain window.",
            &all_surfaces,
        ),
        item(
            "feed:migration-window",
            "remote_runtime_migration",
            "remote_managed",
            ServiceHealthContractState::Migration,
            ServiceHealthOutageScope::MaintenanceWindow,
            &["remote_attach", "remote_shell"],
            &["local_workspace_work", "local_diagnostics"],
            "A runtime migration is underway; local work remains available.",
            freshness(
                "freshness:migration-notice",
                ServiceHealthSourceClass::MirroredNotice,
                "2026-06-07T18:07:00Z",
                "2026-06-08T18:07:00Z",
                "Mirrored migration notice refreshed 2026-06-07T18:07:00Z",
                false,
            ),
            &["diagnostics.open.migration_window"],
            "Continue locally while remote runtime migration completes.",
            &all_surfaces,
        ),
        item(
            "feed:failover-window",
            "status_failover",
            "remote_managed",
            ServiceHealthContractState::Failover,
            ServiceHealthOutageScope::MaintenanceWindow,
            &["managed_status_refresh"],
            &["local_workspace_work", "support_export"],
            "Failover is active; locally cached facts remain copyable and exportable.",
            freshness(
                "freshness:failover-notice",
                ServiceHealthSourceClass::CachedData,
                "2026-06-07T18:08:00Z",
                "2026-06-07T18:23:00Z",
                "Cached failover notice from 2026-06-07T18:08:00Z",
                false,
            ),
            &["diagnostics.open.failover_window"],
            "Use cached facts locally until the failover settles.",
            &all_surfaces,
        ),
        item(
            "feed:reconciling-queue",
            "publish_reconciliation",
            "remote_managed",
            ServiceHealthContractState::Reconciling,
            ServiceHealthOutageScope::MaintenanceWindow,
            &["queued_publish_replay"],
            &["draft_review", "support_export"],
            "Queued work is reconciling after the managed window.",
            freshness(
                "freshness:reconciling-status",
                ServiceHealthSourceClass::LivePolling,
                "2026-06-07T18:18:00Z",
                "2026-06-07T18:33:00Z",
                "Live reconciliation status checked 2026-06-07T18:18:00Z",
                true,
            ),
            &["diagnostics.open.reconciliation_sheet"],
            "Queued local drafts remain reviewable until reconciliation completes.",
            &all_surfaces,
        ),
        item(
            "feed:resolved-incident",
            "release_status_resolution",
            "public_status",
            ServiceHealthContractState::Resolved,
            ServiceHealthOutageScope::MaintenanceWindow,
            &[],
            &["release_notes", "support_export", "help"],
            "The prior release-feed incident resolved and remains exportable for continuity.",
            freshness(
                "freshness:resolved-notice",
                ServiceHealthSourceClass::MirroredNotice,
                "2026-06-07T18:10:00Z",
                "2026-06-14T18:10:00Z",
                "Resolved notice mirrored 2026-06-07T18:10:00Z",
                false,
            ),
            &["diagnostics.open.resolved_notice"],
            "The resolved state remains copyable without requiring screenshots.",
            &all_surfaces,
        ),
    ];

    let item_refs = items
        .iter()
        .map(ServiceHealthFeedItem::item_ref)
        .collect::<Vec<_>>();
    ServiceHealthFeed {
        record_kind: SERVICE_HEALTH_FEED_RECORD_KIND.to_owned(),
        schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
        feed_id: "service_health_feed:stable.shared".to_owned(),
        shared_contract_ref: SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF.to_owned(),
        schema_ref: SERVICE_HEALTH_FEED_SCHEMA_REF.to_owned(),
        items,
        surface_bindings: ServiceHealthSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| ServiceHealthSurfaceBinding {
                surface,
                feed_ref: SERVICE_HEALTH_FEED_CANONICAL_FIXTURE_REF.to_owned(),
                item_refs: item_refs.clone(),
                consumes_shared_feed: true,
                last_checked_visible: true,
                freshness_visible: true,
                local_only_continuity_visible: true,
                may_overclaim_live_reachability: false,
                copyable_exportable: true,
            })
            .collect(),
    }
}

fn item(
    item_id: &str,
    service_family: &str,
    boundary_class: &str,
    contract_state: ServiceHealthContractState,
    outage_scope: ServiceHealthOutageScope,
    affected_workflows: &[&str],
    unaffected_workflows: &[&str],
    summary: &str,
    freshness: ServiceHealthFreshness,
    diagnostics_actions: &[&str],
    local_only_continuity_note: &str,
    surfaced_on: &[ServiceHealthSurface],
) -> ServiceHealthFeedItem {
    ServiceHealthFeedItem {
        schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
        record_kind: SERVICE_HEALTH_FEED_ITEM_RECORD_KIND.to_owned(),
        item_id: item_id.to_owned(),
        service_family: service_family.to_owned(),
        boundary_class: boundary_class.to_owned(),
        contract_state,
        outage_scope,
        affected_workflows: affected_workflows.iter().map(|v| (*v).to_owned()).collect(),
        unaffected_workflows: unaffected_workflows
            .iter()
            .map(|v| (*v).to_owned())
            .collect(),
        summary: summary.to_owned(),
        freshness,
        diagnostics_actions: diagnostics_actions
            .iter()
            .map(|v| (*v).to_owned())
            .collect(),
        local_only_continuity_note: local_only_continuity_note.to_owned(),
        surfaced_on: surfaced_on.to_vec(),
    }
}

fn freshness(
    freshness_ref: &str,
    source_class: ServiceHealthSourceClass,
    last_checked_at: &str,
    stale_after: &str,
    visible_freshness_label: &str,
    live_reachability_claim_allowed: bool,
) -> ServiceHealthFreshness {
    ServiceHealthFreshness {
        freshness_ref: freshness_ref.to_owned(),
        source_class,
        last_checked_at: last_checked_at.to_owned(),
        stale_after: stale_after.to_owned(),
        visible_freshness_label: visible_freshness_label.to_owned(),
        live_reachability_claim_allowed,
    }
}

struct Validator<'a> {
    feed: &'a ServiceHealthFeed,
    findings: Vec<ServiceHealthFeedFinding>,
    coverage: ServiceHealthFeedCoverage,
}

impl<'a> Validator<'a> {
    fn new(feed: &'a ServiceHealthFeed) -> Self {
        Self {
            feed,
            findings: Vec::new(),
            coverage: ServiceHealthFeedCoverage::default(),
        }
    }

    fn run(&mut self) {
        self.validate_envelope();
        self.validate_items();
        self.validate_surface_bindings();
    }

    fn finish(self) -> ServiceHealthFeedValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ServiceHealthFeedFindingSeverity::Error);
        ServiceHealthFeedValidationReport {
            feed_id: self.feed.feed_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn error(&mut self, check_id: &str, message: impl Into<String>) {
        self.findings.push(ServiceHealthFeedFinding {
            severity: ServiceHealthFeedFindingSeverity::Error,
            check_id: check_id.to_owned(),
            message: message.into(),
        });
    }

    fn validate_envelope(&mut self) {
        if self.feed.record_kind != SERVICE_HEALTH_FEED_RECORD_KIND {
            self.error("feed.record_kind", "feed record_kind is not canonical");
        }
        if self.feed.schema_version != SERVICE_HEALTH_FEED_SCHEMA_VERSION {
            self.error("feed.schema_version", "feed schema_version is unsupported");
        }
        if self.feed.shared_contract_ref != SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF {
            self.error(
                "feed.shared_contract_ref",
                "feed shared_contract_ref drifted from the stable contract",
            );
        }
        if self.feed.schema_ref != SERVICE_HEALTH_FEED_SCHEMA_REF {
            self.error("feed.schema_ref", "feed schema_ref drifted");
        }
        if self.feed.items.is_empty() {
            self.error("feed.items.empty", "feed must carry at least one item");
        }
    }

    fn validate_items(&mut self) {
        for item in &self.feed.items {
            self.coverage.contract_states.insert(item.contract_state);
            self.coverage
                .source_classes
                .insert(item.freshness.source_class);
            self.coverage.outage_scopes.insert(item.outage_scope);

            if item.item_id.trim().is_empty()
                || item.service_family.trim().is_empty()
                || item.boundary_class.trim().is_empty()
                || item.summary.trim().is_empty()
                || item.freshness.last_checked_at.trim().is_empty()
                || item.freshness.visible_freshness_label.trim().is_empty()
                || item.local_only_continuity_note.trim().is_empty()
            {
                self.error(
                    "feed.item.required_fields",
                    format!(
                        "item {} is missing required summary or identity fields",
                        item.item_id
                    ),
                );
            }

            if item.diagnostics_actions.is_empty() {
                self.error(
                    "feed.item.diagnostics_actions",
                    format!(
                        "item {} must expose at least one diagnostics action",
                        item.item_id
                    ),
                );
            }

            if item.contract_state == ServiceHealthContractState::Ready
                && !item.affected_workflows.is_empty()
            {
                self.error(
                    "feed.item.ready_affected_workflows",
                    format!(
                        "ready item {} must not carry affected workflows",
                        item.item_id
                    ),
                );
            }

            if item.outage_scope == ServiceHealthOutageScope::PartialService
                && item.unaffected_workflows.is_empty()
            {
                self.error(
                    "feed.item.partial_scope_requires_unaffected",
                    format!(
                        "partial-service item {} must name unaffected workflows",
                        item.item_id
                    ),
                );
            }

            if item.freshness.source_class.forbids_live_reachability()
                && item.freshness.live_reachability_claim_allowed
            {
                self.error(
                    "feed.item.non_live_claim",
                    format!(
                        "item {} may not claim live reachability from {:?}",
                        item.item_id, item.freshness.source_class
                    ),
                );
            }
        }

        if !self
            .feed
            .items
            .iter()
            .any(|item| item.contract_state.preserves_healthy_lane())
        {
            self.error(
                "feed.coverage.healthy_lane",
                "feed must preserve at least one explicitly healthy or resolved lane",
            );
        }
    }

    fn validate_surface_bindings(&mut self) {
        for binding in &self.feed.surface_bindings {
            self.coverage.surfaces.insert(binding.surface);

            if !binding.consumes_shared_feed {
                self.error(
                    "feed.binding.shared_feed_required",
                    format!(
                        "surface {} does not consume the shared feed",
                        binding.surface.as_str()
                    ),
                );
            }
            if !binding.last_checked_visible || !binding.freshness_visible {
                self.error(
                    "feed.binding.freshness_visible",
                    format!(
                        "surface {} must keep last_checked and freshness visible",
                        binding.surface.as_str()
                    ),
                );
            }
            if !binding.local_only_continuity_visible {
                self.error(
                    "feed.binding.local_continuity",
                    format!(
                        "surface {} must keep local-only continuity visible",
                        binding.surface.as_str()
                    ),
                );
            }
            if binding.may_overclaim_live_reachability {
                self.error(
                    "feed.binding.no_overclaim",
                    format!(
                        "surface {} may not overclaim live reachability",
                        binding.surface.as_str()
                    ),
                );
            }
            if !binding.copyable_exportable {
                self.error(
                    "feed.binding.copyable_exportable",
                    format!(
                        "surface {} must remain copyable/exportable without screenshots",
                        binding.surface.as_str()
                    ),
                );
            }
        }
    }
}
