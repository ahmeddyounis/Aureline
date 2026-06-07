//! Scheduled maintenance, read-only/drain windows, tenant migration, failover,
//! and publish-later/local-draft continuity for managed and provider-linked surfaces.
//!
//! This crate owns the typed service-health continuity records that make planned
//! maintenance and managed continuity truthful. It lands explicit scheduled/
//! read-only/drain/migration/failover state models, exact-time notices, blocked-
//! write disclosure, and publish-later or local-draft continuity across provider-
//! backed and shared-session surfaces.
//!
//! The model reuses the provider deferred-intent and reconciliation vocabulary
//! ([`aureline_provider::publish_later::QueueState`],
//! [`aureline_provider::reconciliation::ReconciliationResult`],
//! [`aureline_provider::reconciliation::ProviderDriftClass`]) so drain-window
//! local-draft continuity does not become a one-off special case with different
//! lifecycle language.

#![doc(html_root_url = "https://docs.rs/aureline-service-health/0.0.0")]

pub mod finalize_service_health_destination_truth;
pub mod service_health_feed;
pub mod stabilize_maintenance_and_drain_windows;

pub use finalize_service_health_destination_truth::{
    canonical_service_health_destination_truth_descriptor, BuildIdentityDescriptor,
    ContinuityDrill, ContinuityDrillScenario, DescriptorFreshnessState, DestinationDescriptor,
    DestinationTrustClass, DestinationTrustClassManifest, FreshnessDescriptor, PublicProofSurface,
    ServiceContractState, ServiceHealthDestinationCoverage, ServiceHealthDestinationFinding,
    ServiceHealthDestinationFindingSeverity, ServiceHealthDestinationSupportExport,
    ServiceHealthDestinationTruthDescriptor, ServiceHealthDestinationValidationReport,
    ServiceHealthTruthCard, SupportSaveLaterContract,
    SERVICE_HEALTH_DESTINATION_CANONICAL_DESCRIPTOR_REF, SERVICE_HEALTH_DESTINATION_RECORD_KIND,
    SERVICE_HEALTH_DESTINATION_SCHEMA_REF, SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION,
    SERVICE_HEALTH_DESTINATION_SHARED_CONTRACT_REF,
    SERVICE_HEALTH_DESTINATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use service_health_feed::{
    canonical_service_health_feed, ServiceHealthContractState, ServiceHealthFeed,
    ServiceHealthFeedCoverage, ServiceHealthFeedFinding, ServiceHealthFeedFindingSeverity,
    ServiceHealthFeedItem, ServiceHealthFeedSupportExport, ServiceHealthFeedValidationReport,
    ServiceHealthFreshness, ServiceHealthOutageScope, ServiceHealthSourceClass,
    ServiceHealthSurface, ServiceHealthSurfaceBinding, SERVICE_HEALTH_FEED_CANONICAL_FIXTURE_REF,
    SERVICE_HEALTH_FEED_ITEM_RECORD_KIND, SERVICE_HEALTH_FEED_RECORD_KIND,
    SERVICE_HEALTH_FEED_SCHEMA_REF, SERVICE_HEALTH_FEED_SCHEMA_VERSION,
    SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF, SERVICE_HEALTH_FEED_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_maintenance_and_drain_windows::{
    AffectedSurfaceClass, BlockedWriteClass, BlockedWriteDisclosure, DeferOptionClass,
    ExactTimeWindow, LocalSafeAction, LocalSafeActionClass, MaintenanceNoticeKind,
    MaintenanceWindowState, MaintenanceWindowStateRecord, MaintenanceWindowStateSummary,
    PostWindowReconciliationResult, PostWindowReconciliationState, PostWindowReconciliationSummary,
    RevalidationDimension, RevalidationDimensionResult, ScheduledMaintenanceNotice,
    ScheduledMaintenanceNoticeSummary, ServiceHealthContinuityContractRefs,
    ServiceHealthContinuityCoverage, ServiceHealthContinuityFinding,
    ServiceHealthContinuityFindingSeverity, ServiceHealthContinuityFixtureMetadata,
    ServiceHealthContinuityPage, ServiceHealthContinuitySupportExport,
    ServiceHealthContinuityValidationReport, StaleNoticeDowngradeClass, StaleNoticeDowngradeRule,
    StaleNoticeDowngradeRuleSummary, BLOCKED_WRITE_DISCLOSURE_RECORD_KIND,
    MAINTENANCE_WINDOW_STATE_RECORD_KIND, POST_WINDOW_RECONCILIATION_RESULT_RECORD_KIND,
    SCHEDULED_MAINTENANCE_NOTICE_RECORD_KIND, SERVICE_HEALTH_CONTINUITY_PAGE_RECORD_KIND,
    SERVICE_HEALTH_CONTINUITY_SCHEMA_VERSION, SERVICE_HEALTH_CONTINUITY_SHARED_CONTRACT_REF,
    SERVICE_HEALTH_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND,
    SERVICE_HEALTH_CONTINUITY_VALIDATION_REPORT_RECORD_KIND,
    STALE_NOTICE_DOWNGRADE_RULE_RECORD_KIND,
};
