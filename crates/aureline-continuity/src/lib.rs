//! Connectivity state, deferred-intent, and reconciliation contracts.
//!
//! This crate owns the stable continuity model shared by managed, provider,
//! request-workspace, remote, shell, service-health, diagnostics, and support
//! export surfaces. It models connectivity as an explicit state machine and
//! requires every networked command to declare queueability and replay safety
//! before offline or reconnect behavior can be admitted.

#![doc(html_root_url = "https://docs.rs/aureline-continuity/0.0.0")]

pub mod connectivity_state_and_deferred_intent;
pub mod m5_backup_restore_failover_packets;
pub mod m5_continuity_certification;
pub mod m5_continuity_freshness_slo;
pub mod m5_control_plane_vs_data_plane_outage;
pub mod m5_key_mode_and_storage_posture;
pub mod m5_locality_descriptors_and_tenant_cards;
pub mod m5_locality_tenant_keymode_and_drill_matrix;
pub mod m5_mirror_airgap_continuity_packets;
pub mod m5_operator_support_continuity_summary;
pub mod m5_restore_from_backup_reviews;

pub use connectivity_state_and_deferred_intent::{
    admit_deferred_intent, audit_connectivity_continuity_page, replay_decision,
    seeded_connectivity_continuity_page, validate_connectivity_continuity_page, ActorIdentity,
    AuthScopeSnapshot, CommandQueueabilityDeclaration, ConnectivityBadge, ConnectivityCardAction,
    ConnectivityContinuityDefect, ConnectivityContinuityDefectKind, ConnectivityContinuityPage,
    ConnectivityState, ConnectivityStateCard, DeferredIntent, DeferredIntentAction,
    DeferredIntentState, DriftDimension, DriftRevalidationSnapshot, ExpiryPolicy,
    IdempotencyKeyReceipt, IdempotencyKeyReceiptOutcome, IdempotencyKeyShape, LocalSafePromise,
    NetworkCommandDeclaration, OfflineReadClass, QueueAdmissionDecision, QueueAdmissionOutcome,
    QueueabilityClass, ReconciliationDecision, ReconciliationDisposition, ReconciliationOwnerClass,
    ReconciliationPacket, ReconciliationReviewSheet, ReplayOutcome, ReplayPrerequisite,
    ReplayPrerequisiteClass, ReplayPrerequisiteState, ReplayRevalidationInput, ReplaySafetyClass,
    SensitivePayloadPosture, ServiceFamily, StaleLabelSemantics, SupportExportOutcomeRow,
    SupportExportPacket, TargetIdentity, CONNECTIVITY_BADGE_RECORD_KIND,
    CONNECTIVITY_CARD_RECORD_KIND, CONNECTIVITY_CONTINUITY_ARTIFACT_REF,
    CONNECTIVITY_CONTINUITY_DEFECT_RECORD_KIND, CONNECTIVITY_CONTINUITY_DOC_REF,
    CONNECTIVITY_CONTINUITY_PAGE_RECORD_KIND, CONNECTIVITY_CONTINUITY_SCHEMA_REF,
    CONNECTIVITY_CONTINUITY_SCHEMA_VERSION, CONNECTIVITY_CONTINUITY_SHARED_CONTRACT_REF,
    DEFERRED_INTENT_RECORD_KIND, IDEMPOTENCY_KEY_RECEIPT_RECORD_KIND,
    NETWORK_COMMAND_DECLARATION_RECORD_KIND, RECONCILIATION_PACKET_RECORD_KIND,
    RECONCILIATION_REVIEW_SHEET_RECORD_KIND, SUPPORT_EXPORT_PACKET_RECORD_KIND,
};

pub use m5_backup_restore_failover_packets::{
    audit_backup_restore_failover_page, seeded_backup_restore_failover_input,
    seeded_backup_restore_failover_page, validate_backup_restore_failover_page,
    BackupRestoreFailoverDefect, BackupRestoreFailoverDescriptor, BackupRestoreFailoverInput,
    BackupRestoreFailoverOutcome, BackupRestoreFailoverPacketEntry, BackupRestoreFailoverPage,
    BackupRestoreFailoverSummary, BackupRestoreFailoverSupportExport,
    BackupRestoreFailoverSurfaceProjection, ClaimCoverageClass, ClaimCoverageRow, DrillEvidence,
    DrillPacketRegistry, PacketNarrowReasonClass, PacketSurfaceClass, RestoreOperationClass,
    RestoreScope, ScopeExercisedClass, BACKUP_RESTORE_FAILOVER_ARTIFACT_REF,
    BACKUP_RESTORE_FAILOVER_DEFECT_RECORD_KIND, BACKUP_RESTORE_FAILOVER_DESCRIPTOR_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_DOC_REF, BACKUP_RESTORE_FAILOVER_OUTCOME_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_PAGE_RECORD_KIND, BACKUP_RESTORE_FAILOVER_SCHEMA_REF,
    BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION, BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF,
    BACKUP_RESTORE_FAILOVER_SUMMARY_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_SURFACE_PROJECTION_RECORD_KIND, CLAIM_COVERAGE_ROW_RECORD_KIND,
    DRILL_PACKET_REGISTRY_RECORD_KIND,
};

pub use m5_control_plane_vs_data_plane_outage::{
    audit_service_outage_taxonomy_page, seeded_service_outage_taxonomy_input,
    seeded_service_outage_taxonomy_page, validate_service_outage_taxonomy_page,
    DegradedFallbackClass, ImpairmentSeverityClass, LocalCoreContinuity, OptionalServiceFamily,
    OutageDegradedStateClass, OutageEvidenceStateClass, OutageNarrowReasonClass,
    OutageSurfaceClass, OutageTaxonomyDefect, ServiceOutageDescriptor, ServiceOutageEntry,
    ServiceOutageOutcome, ServiceOutageSurfaceProjection, ServiceOutageTaxonomyInput,
    ServiceOutageTaxonomyPage, ServiceOutageTaxonomySummary, ServiceOutageTaxonomySupportExport,
    OUTAGE_SURFACE_PROJECTION_RECORD_KIND, OUTAGE_TAXONOMY_ARTIFACT_REF,
    OUTAGE_TAXONOMY_DEFECT_RECORD_KIND, OUTAGE_TAXONOMY_DOC_REF, OUTAGE_TAXONOMY_PAGE_RECORD_KIND,
    OUTAGE_TAXONOMY_SCHEMA_REF, OUTAGE_TAXONOMY_SCHEMA_VERSION,
    OUTAGE_TAXONOMY_SHARED_CONTRACT_REF, OUTAGE_TAXONOMY_SUMMARY_RECORD_KIND,
    OUTAGE_TAXONOMY_SUPPORT_EXPORT_RECORD_KIND, SERVICE_OUTAGE_DESCRIPTOR_RECORD_KIND,
    SERVICE_OUTAGE_OUTCOME_RECORD_KIND,
};

pub use m5_locality_descriptors_and_tenant_cards::{
    audit_locality_tenant_card_page, seeded_locality_tenant_card_page,
    seeded_locality_tenant_input, validate_locality_tenant_card_page, LocalityDescriptor,
    LocalitySurfaceClass, LocalitySurfaceProjection, LocalityTenantCardPage, LocalityTenantDefect,
    LocalityTenantEntry, LocalityTenantInput, LocalityTenantNarrowReasonClass,
    LocalityTenantRowOutcome, LocalityTenantSummary, LocalityTenantSupportExport, RegionPinClass,
    RegionPinHonorState, RetentionClass, TenantBoundaryCard, TenantIsolationClass,
    LOCALITY_DESCRIPTOR_RECORD_KIND, LOCALITY_SURFACE_PROJECTION_RECORD_KIND,
    LOCALITY_TENANT_ARTIFACT_REF, LOCALITY_TENANT_DEFECT_RECORD_KIND, LOCALITY_TENANT_DOC_REF,
    LOCALITY_TENANT_PAGE_RECORD_KIND, LOCALITY_TENANT_ROW_OUTCOME_RECORD_KIND,
    LOCALITY_TENANT_SCHEMA_REF, LOCALITY_TENANT_SCHEMA_VERSION,
    LOCALITY_TENANT_SHARED_CONTRACT_REF, LOCALITY_TENANT_SUMMARY_RECORD_KIND,
    LOCALITY_TENANT_SUPPORT_EXPORT_RECORD_KIND, TENANT_BOUNDARY_CARD_RECORD_KIND,
};

pub use m5_locality_tenant_keymode_and_drill_matrix::{
    audit_continuity_claim_matrix_page, seeded_continuity_claim_matrix_input,
    seeded_continuity_claim_matrix_page, validate_continuity_claim_matrix_page,
    ClaimSurfaceVisibility, ContinuityClaimDefect, ContinuityClaimMatrixInput,
    ContinuityClaimMatrixPage, ContinuityClaimMatrixSummary, ContinuityClaimMatrixSupportExport,
    ContinuityClaimNarrowReasonClass, ContinuityClaimQualificationClass, ContinuityClaimRow,
    ContinuityClaimRowOutcome, ContinuityDrill, ContinuityLaneClass, ContinuityPacketFamilyClass,
    ContinuityProfileClass, DrillCadenceClass, DrillEvidenceStateClass, DrillScheduleEntry,
    KeyModeClass, LocalityClass, LocalityPosture, PartialLossClass, PlaneImpairmentClass,
    RestoreFailoverHostingClass, RestoreIdentityClass, TenantScopeClass,
    CONTINUITY_CLAIM_DEFECT_RECORD_KIND, CONTINUITY_CLAIM_MATRIX_ARTIFACT_REF,
    CONTINUITY_CLAIM_MATRIX_DOC_REF, CONTINUITY_CLAIM_MATRIX_PAGE_RECORD_KIND,
    CONTINUITY_CLAIM_MATRIX_SCHEMA_REF, CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION,
    CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF, CONTINUITY_CLAIM_MATRIX_SUMMARY_RECORD_KIND,
    CONTINUITY_CLAIM_MATRIX_SUPPORT_EXPORT_RECORD_KIND, CONTINUITY_CLAIM_ROW_OUTCOME_RECORD_KIND,
};

pub use m5_continuity_freshness_slo::{
    audit_continuity_freshness_slo_dashboard, seeded_continuity_freshness_slo_dashboard,
    seeded_continuity_freshness_slo_input, validate_continuity_freshness_slo_dashboard,
    ContinuityFreshnessDefect, ContinuityFreshnessDefectKind, ContinuityFreshnessRow,
    ContinuityFreshnessRowOutcome, ContinuityFreshnessRowState, ContinuityFreshnessSlo,
    ContinuityFreshnessSloDashboard, ContinuityFreshnessSloInput, ContinuityFreshnessSloState,
    ContinuityFreshnessSloSummary, ContinuityFreshnessSloSupportExport, ContinuityPromotionVerdict,
    ContinuityProofPacket, ContinuityRerunPath, ContinuityStopAction, ContinuityStopReason,
    ContinuityStopRule, RerunAutomationClass, CONTINUITY_FRESHNESS_DEFECT_RECORD_KIND,
    CONTINUITY_FRESHNESS_ROW_OUTCOME_RECORD_KIND, CONTINUITY_FRESHNESS_SLO_ARTIFACT_REF,
    CONTINUITY_FRESHNESS_SLO_DASHBOARD_RECORD_KIND, CONTINUITY_FRESHNESS_SLO_DOC_REF,
    CONTINUITY_FRESHNESS_SLO_SCHEMA_REF, CONTINUITY_FRESHNESS_SLO_SCHEMA_VERSION,
    CONTINUITY_FRESHNESS_SLO_SHARED_CONTRACT_REF, CONTINUITY_FRESHNESS_SLO_SUMMARY_RECORD_KIND,
    CONTINUITY_FRESHNESS_SLO_SUPPORT_EXPORT_RECORD_KIND,
};

pub use m5_continuity_certification::{
    audit_continuity_certification_report, seeded_continuity_certification_input,
    seeded_continuity_certification_report, validate_continuity_certification_report,
    CertificationDefectKind, CertificationDimension, CertificationEvidence,
    CertificationEvidenceState, CertificationNarrowReasonClass, CertificationSourceRefs,
    CertifiedRow, CertifiedRowOutcome, ContinuityCertificationDefect, ContinuityCertificationInput,
    ContinuityCertificationReport, ContinuityCertificationSummary,
    ContinuityCertificationSupportExport, RowCertificationVerdict,
    CERTIFIED_ROW_OUTCOME_RECORD_KIND, CONTINUITY_CERTIFICATION_ARTIFACT_REF,
    CONTINUITY_CERTIFICATION_DEFECT_RECORD_KIND, CONTINUITY_CERTIFICATION_DOC_REF,
    CONTINUITY_CERTIFICATION_REPORT_RECORD_KIND, CONTINUITY_CERTIFICATION_SCHEMA_REF,
    CONTINUITY_CERTIFICATION_SCHEMA_VERSION, CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF,
    CONTINUITY_CERTIFICATION_SUMMARY_RECORD_KIND,
    CONTINUITY_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND,
};

pub use m5_key_mode_and_storage_posture::{
    audit_key_mode_storage_posture_page, seeded_key_mode_storage_posture_input,
    seeded_key_mode_storage_posture_page, validate_key_mode_storage_posture_page,
    DegradedStateClass, KeyAvailabilityState, KeyEvidenceStateClass, KeyModeDescriptor,
    KeyModeStorageEntry, KeyModeStoragePostureInput, KeyModeStoragePosturePage,
    KeyModeStoragePostureSummary, KeyModeStoragePostureSupportExport, KeyPostureDefect,
    KeyPostureNarrowReasonClass, KeyPostureRowOutcome, KeyPostureSurfaceClass,
    KeyPostureSurfaceProjection, StorageEncryptionClass, StoragePostureDescriptor, StoreLockState,
    TrustRootPostureClass, KEY_MODE_DESCRIPTOR_RECORD_KIND, KEY_POSTURE_ARTIFACT_REF,
    KEY_POSTURE_DEFECT_RECORD_KIND, KEY_POSTURE_DOC_REF, KEY_POSTURE_PAGE_RECORD_KIND,
    KEY_POSTURE_ROW_OUTCOME_RECORD_KIND, KEY_POSTURE_SCHEMA_REF, KEY_POSTURE_SCHEMA_VERSION,
    KEY_POSTURE_SHARED_CONTRACT_REF, KEY_POSTURE_SUMMARY_RECORD_KIND,
    KEY_POSTURE_SUPPORT_EXPORT_RECORD_KIND, KEY_POSTURE_SURFACE_PROJECTION_RECORD_KIND,
    STORAGE_POSTURE_DESCRIPTOR_RECORD_KIND,
};

pub use m5_restore_from_backup_reviews::{
    audit_restore_review_page, seeded_restore_review_input, seeded_restore_review_page,
    validate_restore_review_page, AffectedSliceClass, CompareExportParity, ReplayFence,
    ReplayFenceStateClass, ReplayPostureClass, RestoreArtifactFamilyClass, RestoreFidelityClass,
    RestoreIdentitySummary, RestoreLaneClass, RestoreReviewCoverageRow, RestoreReviewDefect,
    RestoreReviewDescriptor, RestoreReviewEntry, RestoreReviewInput,
    RestoreReviewNarrowReasonClass, RestoreReviewOutcome, RestoreReviewPage, RestoreReviewRegistry,
    RestoreReviewSummary, RestoreReviewSupportExport, RestoreReviewSurfaceProjection,
    ReviewCoverageClass, ReviewSurfaceClass, RESTORE_REVIEW_ARTIFACT_REF,
    RESTORE_REVIEW_COVERAGE_ROW_RECORD_KIND, RESTORE_REVIEW_DEFECT_RECORD_KIND,
    RESTORE_REVIEW_DESCRIPTOR_RECORD_KIND, RESTORE_REVIEW_DOC_REF,
    RESTORE_REVIEW_OUTCOME_RECORD_KIND, RESTORE_REVIEW_PAGE_RECORD_KIND,
    RESTORE_REVIEW_REGISTRY_RECORD_KIND, RESTORE_REVIEW_SCHEMA_REF, RESTORE_REVIEW_SCHEMA_VERSION,
    RESTORE_REVIEW_SHARED_CONTRACT_REF, RESTORE_REVIEW_SUMMARY_RECORD_KIND,
    RESTORE_REVIEW_SUPPORT_EXPORT_RECORD_KIND, RESTORE_REVIEW_SURFACE_PROJECTION_RECORD_KIND,
};

pub use m5_mirror_airgap_continuity_packets::{
    audit_mirror_airgap_page, seeded_mirror_airgap_input, seeded_mirror_airgap_page,
    validate_mirror_airgap_page, AdvisoryRevocationSourceClass, ConnectivityPostureClass,
    MirrorAirgapDefect, MirrorAirgapDescriptor, MirrorAirgapInput, MirrorAirgapNarrowReasonClass,
    MirrorAirgapOutcome, MirrorAirgapPacketEntry, MirrorAirgapPage, MirrorAirgapSummary,
    MirrorAirgapSupportExport, MirrorAirgapSurfaceProjection, MirrorFreshness,
    MirrorFreshnessStateClass, OfflineContinuityRegistry, OfflineCoverageClass, OfflineCoverageRow,
    OfflineExchangeClass, OfflineSurfaceClass, PublicFallbackPolicyClass, TrustRootContinuity,
    TrustRootRenewalClass, MIRROR_AIRGAP_ARTIFACT_REF, MIRROR_AIRGAP_DEFECT_RECORD_KIND,
    MIRROR_AIRGAP_DESCRIPTOR_RECORD_KIND, MIRROR_AIRGAP_DOC_REF, MIRROR_AIRGAP_OUTCOME_RECORD_KIND,
    MIRROR_AIRGAP_PAGE_RECORD_KIND, MIRROR_AIRGAP_SCHEMA_REF, MIRROR_AIRGAP_SCHEMA_VERSION,
    MIRROR_AIRGAP_SHARED_CONTRACT_REF, MIRROR_AIRGAP_SUMMARY_RECORD_KIND,
    MIRROR_AIRGAP_SUPPORT_EXPORT_RECORD_KIND, MIRROR_AIRGAP_SURFACE_PROJECTION_RECORD_KIND,
    OFFLINE_CONTINUITY_REGISTRY_RECORD_KIND, OFFLINE_COVERAGE_ROW_RECORD_KIND,
};

pub use m5_operator_support_continuity_summary::{
    audit_operator_support_continuity_page, seeded_operator_support_continuity_input,
    seeded_operator_support_continuity_page, validate_operator_support_continuity_page,
    AffectedOutageLabel, ContinuityRowSummary, ContinuityRowSummaryOutcome,
    ContinuitySummarySurfaceCoverage, LocalityKeyTenantPosture, OperatorSupportContinuityDefect,
    OperatorSupportContinuityInput, OperatorSupportContinuityPage,
    OperatorSupportContinuitySummary, OperatorSupportContinuitySupportExport, SummaryEvidence,
    SummaryNarrowReasonClass, SummaryRedaction, CONTINUITY_ROW_SUMMARY_OUTCOME_RECORD_KIND,
    CONTINUITY_ROW_SUMMARY_RECORD_KIND, OPERATOR_SUPPORT_CONTINUITY_ARTIFACT_REF,
    OPERATOR_SUPPORT_CONTINUITY_DEFECT_RECORD_KIND, OPERATOR_SUPPORT_CONTINUITY_DOC_REF,
    OPERATOR_SUPPORT_CONTINUITY_PAGE_RECORD_KIND, OPERATOR_SUPPORT_CONTINUITY_SCHEMA_REF,
    OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION, OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF,
    OPERATOR_SUPPORT_CONTINUITY_SUMMARY_RECORD_KIND,
    OPERATOR_SUPPORT_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND,
};
