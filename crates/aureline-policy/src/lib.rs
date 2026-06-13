//! Policy simulation, authority-ticket, remembered-decision, and deployment
//! residency contracts.
//!
//! This crate owns the beta policy-governance object model consumed by shell,
//! support, admin, and offline-file control paths. It does not evaluate a full
//! policy language; it provides typed preview records, bounded exception and
//! waiver rows, authority-ticket lineage for privileged actions,
//! remembered-decision drift checks, metadata-safe support exports, and stable
//! deployment-residency proof packets.

#![doc(html_root_url = "https://docs.rs/aureline-policy/0.0.0")]

pub mod add_launch_inspector_and_command_runtime_explain_sheets_that_answer_where_this_runs_why_this_toolchain_what_it_can_acces;
pub mod authority;
pub mod deployment_profile_continuity_truth;
pub mod finalize_backup_restore_failover_and_local_core_continuity;
pub mod finalize_managed_workspace_lifecycle_truth;
pub mod finalize_open_vs_paid_boundary_and_offboarding;
pub mod finalize_signed_policy_bundle_offline_entitlement_and_mirror;
pub mod finalize_the_secret_broker_handle_only_delegated_and;
pub mod freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix;
pub mod harden_enterprise_network_proxy_pac_manual_system_proxy;
pub mod harden_identity_and_admin_support_export_parity_audit;
pub mod harden_os_keychain_and_trust_store_integration_trust;
pub mod implement_approval_ticket_issuance_deny_reason_packets_replay_nonce_or_expiry_enforcement_and_local_first_verification_f;
pub mod implement_execution_surface_classes_sandbox_profile_descriptors_and_unsupported_or_stricter_profile_truth;
pub mod m5_exception_expiry;
pub mod policy_simulation_and_expiry;
pub mod publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits;
pub mod records_policy_governance_snapshot;
pub mod runtime_authority_issuers;
pub mod ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e;
pub mod ship_child_envelope_derivation_nested_launch_narrowing_handle_only_secret_projection_and_no_ambient_privilege_enforcemen;
pub mod simulation;
pub mod stabilize_approval_ticket_audit_and_target_identity_lineage;
pub mod stabilize_deployment_and_residency_truth;
pub mod stabilize_effective_policy_remembered_decision_waiver_expiry_and;
pub mod stabilize_organization_admin_provisioning_and_seat_lifecycle_truth;
pub mod stabilize_transport_policy_proxy_resolution_trust_store_and_mirror_route;

pub use authority::{
    audit_authority_ticket_page, seeded_authority_ticket_page, validate_authority_ticket_page,
    AuthorityActorBinding, AuthorityActorClass, AuthorityEvaluationOutcome, AuthorityGuardrails,
    AuthorityIssuerClass, AuthorityLineage, AuthorityRequestOriginClass, AuthorityRevocationHook,
    AuthorityRevocationState, AuthoritySandboxBinding, AuthoritySideEffectClass,
    AuthoritySourceProof, AuthoritySourceProofClass, AuthorityTargetClass, AuthorityTargetIdentity,
    AuthorityTicketClass, AuthorityTicketDefect, AuthorityTicketDefectKind, AuthorityTicketPage,
    AuthorityTicketRecord, AuthorityTicketSpendAttempt, AuthorityTicketSummary,
    AuthorityTicketSupportExport, AuthorityUsePosture, CredentialConsumerIdentity,
    CredentialProjectionMode, CredentialProjectionRecord, CredentialReferenceClass,
    RememberedAuthorityRule, RootAuthorityChangeClass, RootAuthorityChangeRecord,
    AUTHORITY_TICKET_DEFECT_RECORD_KIND, AUTHORITY_TICKET_PAGE_RECORD_KIND,
    AUTHORITY_TICKET_RECORD_KIND, AUTHORITY_TICKET_SCHEMA_VERSION,
    AUTHORITY_TICKET_SHARED_CONTRACT_REF, AUTHORITY_TICKET_SOURCE_MATRIX_REF,
    AUTHORITY_TICKET_SPEND_ATTEMPT_RECORD_KIND, AUTHORITY_TICKET_SUMMARY_RECORD_KIND,
    AUTHORITY_TICKET_SUPPORT_EXPORT_RECORD_KIND, CREDENTIAL_PROJECTION_RECORD_KIND,
    ROOT_AUTHORITY_CHANGE_RECORD_KIND,
};

pub use runtime_authority_issuers::{
    audit_runtime_authority_issuer_page, seeded_runtime_authority_issuer_page,
    validate_runtime_authority_issuer_page, AuthoritySourceClass, IssuerBoundaryDecision,
    IssuerBoundaryDecisionClass, IssuerBoundaryRejectionReason, IssuerBoundaryRequest,
    RememberedDecisionRule, RequestingSurfaceClass, RequestingSurfaceRecord,
    RuntimeAuthorityIssuerDefect, RuntimeAuthorityIssuerDefectKind, RuntimeAuthorityIssuerPage,
    RuntimeAuthorityIssuerRecord, RuntimeAuthorityIssuerSummary, RuntimeAuthorityLineagePacket,
    RuntimeAuthorityLineageRow, ISSUER_BOUNDARY_DECISION_RECORD_KIND,
    ISSUER_BOUNDARY_REQUEST_RECORD_KIND, REMEMBERED_DECISION_RULE_RECORD_KIND,
    REQUESTING_SURFACE_RECORD_KIND, RUNTIME_AUTHORITY_ISSUER_DEFECT_RECORD_KIND,
    RUNTIME_AUTHORITY_ISSUER_PAGE_RECORD_KIND, RUNTIME_AUTHORITY_ISSUER_RECORD_KIND,
    RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION, RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF,
    RUNTIME_AUTHORITY_ISSUER_SOURCE_MATRIX_REF, RUNTIME_AUTHORITY_ISSUER_SUMMARY_RECORD_KIND,
    RUNTIME_AUTHORITY_LINEAGE_PACKET_RECORD_KIND,
};

pub use simulation::{
    audit_policy_simulation_beta_page, revalidate_remembered_decision,
    seeded_policy_simulation_beta_page, simulate_policy_change,
    validate_policy_simulation_beta_page, ActionFamilyClass, ActorPersonaClass, ActorRef,
    AffectedPolicySurface, DashboardBucketClass, DegradedModeClass, EnvironmentBinding,
    ExceptionKindClass, ExceptionalAuthorityRecord, ExceptionalAuthorityStatusClass,
    MemoryStateClass, PolicyChangeClass, PolicyContextSnapshot, PolicySimulationBetaDefect,
    PolicySimulationBetaDefectKind, PolicySimulationBetaPage, PolicySimulationRecord,
    PolicySimulationRequest, PolicySimulationSummary, PolicySimulationSupportExport,
    PolicyStateAtActionTime, ProtectedPathChangeClass, RememberedDecisionDriftReason,
    RememberedDecisionDriftSnapshot, RememberedDecisionRecord, RenewalPathClass,
    RevocationPathClass, ScopeKind, ScopeRef, SubjectRef, TimeHorizon,
    POLICY_SIMULATION_AFFECTED_SURFACE_RECORD_KIND, POLICY_SIMULATION_BETA_DEFECT_RECORD_KIND,
    POLICY_SIMULATION_BETA_PAGE_RECORD_KIND, POLICY_SIMULATION_BETA_SCHEMA_VERSION,
    POLICY_SIMULATION_EXCEPTION_RECORD_KIND, POLICY_SIMULATION_RECORD_KIND,
    POLICY_SIMULATION_REMEMBERED_DECISION_RECORD_KIND, POLICY_SIMULATION_SHARED_CONTRACT_REF,
    POLICY_SIMULATION_STATE_AT_ACTION_RECORD_KIND, POLICY_SIMULATION_SUMMARY_RECORD_KIND,
    POLICY_SIMULATION_SUPPORT_EXPORT_RECORD_KIND,
};

pub use stabilize_effective_policy_remembered_decision_waiver_expiry_and::{
    audit_effective_policy_stabilize_page, seeded_effective_policy_stabilize_page,
    validate_effective_policy_stabilize_page, EffectivePolicyStabilizeDefect,
    EffectivePolicyStabilizeNarrowReasonClass, EffectivePolicyStabilizePage,
    EffectivePolicyStabilizeQualificationClass, EffectivePolicyStabilizeRow,
    EffectivePolicyStabilizeSummary, EffectivePolicyStabilizeSupportExport,
    EFFECTIVE_POLICY_STABILIZE_ARTIFACT_REF, EFFECTIVE_POLICY_STABILIZE_DEFECT_RECORD_KIND,
    EFFECTIVE_POLICY_STABILIZE_DOC_REF, EFFECTIVE_POLICY_STABILIZE_PAGE_RECORD_KIND,
    EFFECTIVE_POLICY_STABILIZE_ROW_RECORD_KIND, EFFECTIVE_POLICY_STABILIZE_SCHEMA_VERSION,
    EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF,
    EFFECTIVE_POLICY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND,
};

pub use policy_simulation_and_expiry::{
    audit_policy_simulation_and_expiry_page, seeded_policy_simulation_and_expiry_page,
    validate_policy_simulation_and_expiry_page, ApprovalHistoryRow, ExceptionPreviewSheet,
    ExpiryBanner, ExpirySubjectClass, PolicyDiffImpactSummary, PolicySimulationAndExpiryDefect,
    PolicySimulationAndExpiryDefectKind, PolicySimulationAndExpiryPage,
    PolicySimulationAndExpiryQualificationClass, PolicySimulationAndExpirySummary,
    PolicySimulationAndExpirySupportExport, PolicySimulationExceptionExpiryReviewPacket,
    PolicySimulationProjectionSurfaceClass, PolicySimulationView, ReapprovalTriggerClass,
    APPROVAL_HISTORY_ROW_RECORD_KIND, EXCEPTION_PREVIEW_SHEET_RECORD_KIND,
    EXPIRY_BANNER_RECORD_KIND, POLICY_DIFF_IMPACT_SUMMARY_RECORD_KIND,
    POLICY_SIMULATION_AND_EXPIRY_ARTIFACT_REF, POLICY_SIMULATION_AND_EXPIRY_DEFECT_RECORD_KIND,
    POLICY_SIMULATION_AND_EXPIRY_DOC_REF, POLICY_SIMULATION_AND_EXPIRY_PAGE_RECORD_KIND,
    POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION, POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF,
    POLICY_SIMULATION_AND_EXPIRY_SUPPORT_EXPORT_RECORD_KIND,
    POLICY_SIMULATION_EXCEPTION_EXPIRY_REVIEW_PACKET_RECORD_KIND,
    POLICY_SIMULATION_VIEW_RECORD_KIND,
};

pub use m5_exception_expiry::{
    seeded_m5_exception_expiry_packet, ApprovalEvent, ApprovalEventClass,
    ApprovalHistoryRow as M5ApprovalHistoryRow, AuthorityDimension, ExceptionRequestSheet,
    ExceptionScopeBinding, ExpiryBanner as M5ExpiryBanner, ExpiryState, M5ExceptionExpiryPacket,
    M5ExceptionExpiryRow, M5ExceptionExpiryViolation, ObservedContext,
    RememberedDecisionRevalidation, RevalidationOutcome, M5_APPROVAL_HISTORY_ROW_RECORD_KIND,
    M5_EXCEPTION_EXPIRY_ARTIFACT_REF, M5_EXCEPTION_EXPIRY_DOC_REF,
    M5_EXCEPTION_EXPIRY_RECORDS_CONTRACT_REF, M5_EXCEPTION_EXPIRY_RECORD_KIND,
    M5_EXCEPTION_EXPIRY_SCHEMA_VERSION, M5_EXCEPTION_EXPIRY_SHARED_CONTRACT_REF,
    M5_EXCEPTION_REQUEST_SHEET_RECORD_KIND, M5_EXPIRY_BANNER_RECORD_KIND,
    M5_REMEMBERED_DECISION_REVALIDATION_RECORD_KIND,
};

pub use records_policy_governance_snapshot::{
    seeded_records_policy_governance_snapshot, PolicyGovernanceCoverageRow, PolicyGovernanceFamily,
    PolicyGovernanceScopeSnapshot, PolicyGovernanceSnapshotDefect,
    PolicyGovernanceSnapshotDefectKind, RECORDS_POLICY_GOVERNANCE_SCOPE_ROW_RECORD_KIND,
    RECORDS_POLICY_GOVERNANCE_SNAPSHOT_RECORD_KIND,
    RECORDS_POLICY_GOVERNANCE_SNAPSHOT_SCHEMA_VERSION,
};

pub use harden_enterprise_network_proxy_pac_manual_system_proxy::{
    audit_harden_enterprise_network_proxy_page, seeded_harden_enterprise_network_proxy_page,
    validate_harden_enterprise_network_proxy_page, BootstrapCredentialDeclaration,
    BootstrapCredentialKind, HardenEnterpriseNetworkProxyDefect,
    HardenEnterpriseNetworkProxyNarrowReasonClass, HardenEnterpriseNetworkProxyPage,
    HardenEnterpriseNetworkProxyQualificationClass, HardenEnterpriseNetworkProxyRow,
    HardenEnterpriseNetworkProxySummary, HardenEnterpriseNetworkProxySupportExport,
    ProxyPrecedenceClass, ProxyRouteClass, ProxySelectorReasonClass, RouteClientCertPostureClass,
    TlsVerificationPostureClass, HARDEN_ENTERPRISE_NETWORK_PROXY_ARTIFACT_REF,
    HARDEN_ENTERPRISE_NETWORK_PROXY_DEFECT_RECORD_KIND, HARDEN_ENTERPRISE_NETWORK_PROXY_DOC_REF,
    HARDEN_ENTERPRISE_NETWORK_PROXY_PAGE_RECORD_KIND,
    HARDEN_ENTERPRISE_NETWORK_PROXY_ROW_RECORD_KIND,
    HARDEN_ENTERPRISE_NETWORK_PROXY_SCHEMA_VERSION,
    HARDEN_ENTERPRISE_NETWORK_PROXY_SHARED_CONTRACT_REF,
    HARDEN_ENTERPRISE_NETWORK_PROXY_SUMMARY_RECORD_KIND,
    HARDEN_ENTERPRISE_NETWORK_PROXY_SUPPORT_EXPORT_RECORD_KIND, NETWORK_TRUST_BETA_CONTRACT_REF,
};

pub use finalize_signed_policy_bundle_offline_entitlement_and_mirror::{
    audit_finalize_signed_policy_bundle_page, seeded_finalize_signed_policy_bundle_page,
    validate_finalize_signed_policy_bundle_page, BundleDeliverySourceClass, BundleEnvelopeReview,
    BundleImportFlowClass, BundleKindClass, BundleLifecycleAuditEvent, BundleLifecycleEventClass,
    ExpiryGuidanceClass, FinalizeSignedPolicyBundleDefect,
    FinalizeSignedPolicyBundleNarrowReasonClass, FinalizeSignedPolicyBundlePage,
    FinalizeSignedPolicyBundleQualificationClass, FinalizeSignedPolicyBundleRow,
    FinalizeSignedPolicyBundleSummary, FinalizeSignedPolicyBundleSupportExport, GracePostureClass,
    OfflineGraceState, PolicyBundleSimulationPacket, PolicyEpochState,
    PrivilegedOperationPostureClass, OFFLINE_ENTITLEMENT_VERIFIER_CONTRACT_REF,
    SIGNED_POLICY_BUNDLE_FINALIZE_ARTIFACT_REF, SIGNED_POLICY_BUNDLE_FINALIZE_DEFECT_RECORD_KIND,
    SIGNED_POLICY_BUNDLE_FINALIZE_DOC_REF,
    SIGNED_POLICY_BUNDLE_FINALIZE_LIFECYCLE_EVENT_RECORD_KIND,
    SIGNED_POLICY_BUNDLE_FINALIZE_PAGE_RECORD_KIND, SIGNED_POLICY_BUNDLE_FINALIZE_ROW_RECORD_KIND,
    SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
    SIGNED_POLICY_BUNDLE_FINALIZE_SHARED_CONTRACT_REF,
    SIGNED_POLICY_BUNDLE_FINALIZE_SUMMARY_RECORD_KIND,
    SIGNED_POLICY_BUNDLE_FINALIZE_SUPPORT_EXPORT_RECORD_KIND,
};

pub use finalize_the_secret_broker_handle_only_delegated_and::{
    audit_finalize_secret_broker_rows, seeded_finalize_secret_broker_page,
    validate_finalize_secret_broker_page, CredentialRotationEventClass, CredentialRotationState,
    FinalizeSecretBrokerDefect, FinalizeSecretBrokerNarrowReasonClass, FinalizeSecretBrokerPage,
    FinalizeSecretBrokerQualificationClass, FinalizeSecretBrokerRow, FinalizeSecretBrokerSummary,
    FinalizeSecretBrokerSupportExport, RememberedApprovalRow, SecretBrokerFlowClass,
    SecretBrokerHandleClass, FINALIZE_SECRET_BROKER_ARTIFACT_REF,
    FINALIZE_SECRET_BROKER_DEFECT_RECORD_KIND, FINALIZE_SECRET_BROKER_DOC_REF,
    FINALIZE_SECRET_BROKER_PAGE_RECORD_KIND,
    FINALIZE_SECRET_BROKER_REMEMBERED_APPROVAL_RECORD_KIND, FINALIZE_SECRET_BROKER_ROW_RECORD_KIND,
    FINALIZE_SECRET_BROKER_SCHEMA_VERSION, FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF,
    FINALIZE_SECRET_BROKER_SUMMARY_RECORD_KIND, FINALIZE_SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND,
    SECRET_BROKER_BETA_CONTRACT_REF,
};

pub use harden_identity_and_admin_support_export_parity_audit::{
    audit_harden_identity_admin_page, seeded_harden_identity_admin_page,
    validate_harden_identity_admin_page, AdminActionLineage, HardenIdentityAdminDefect,
    HardenIdentityAdminNarrowReasonClass, HardenIdentityAdminPage,
    HardenIdentityAdminQualificationClass, HardenIdentityAdminSummary,
    HardenIdentityAdminSupportExport, IdentityAdminProvisioningClass, IdentityAdminRow,
    IdentityAdminRowClass, IdentityAdminSyncFreshnessClass, LocalTenantScopeClass,
    ProvisioningFailureKind, HARDEN_IDENTITY_ADMIN_ARTIFACT_REF,
    HARDEN_IDENTITY_ADMIN_DEFECT_RECORD_KIND, HARDEN_IDENTITY_ADMIN_DOC_REF,
    HARDEN_IDENTITY_ADMIN_PAGE_RECORD_KIND, HARDEN_IDENTITY_ADMIN_ROW_RECORD_KIND,
    HARDEN_IDENTITY_ADMIN_SCHEMA_VERSION, HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF,
    HARDEN_IDENTITY_ADMIN_SUMMARY_RECORD_KIND, HARDEN_IDENTITY_ADMIN_SUPPORT_EXPORT_RECORD_KIND,
};

pub use harden_os_keychain_and_trust_store_integration_trust::{
    audit_harden_os_keychain_trust_store_page, seeded_harden_os_keychain_trust_store_page,
    validate_harden_os_keychain_trust_store_page, HardenOsKeychainTrustStoreDefect,
    HardenOsKeychainTrustStoreNarrowReasonClass, HardenOsKeychainTrustStorePage,
    HardenOsKeychainTrustStoreQualificationClass, HardenOsKeychainTrustStoreSummary,
    HardenOsKeychainTrustStoreSupportExport, SessionImpactClass, TrustStoreChangeAttributionClass,
    TrustStoreChangeClass, TrustStoreChangeEvent, TrustStoreLayerClass, TrustStoreLayerHealthClass,
    TrustStoreLayerRow, TrustStoreRepairActionClass, HARDEN_OS_KEYCHAIN_TRUST_STORE_ARTIFACT_REF,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_CHANGE_EVENT_RECORD_KIND,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_DEFECT_RECORD_KIND, HARDEN_OS_KEYCHAIN_TRUST_STORE_DOC_REF,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_PAGE_RECORD_KIND,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_ROW_RECORD_KIND, HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_SUMMARY_RECORD_KIND,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_SUPPORT_EXPORT_RECORD_KIND,
};

pub use publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits::{
    audit_enterprise_docs_matrices_known_limits_page,
    seeded_enterprise_docs_matrices_known_limits_page,
    validate_enterprise_docs_matrices_known_limits_page, DocsCompletenessClass, DocsDeclaration,
    EnterpriseDocsMatricesKnownLimitsDefect, EnterpriseDocsMatricesKnownLimitsNarrowReasonClass,
    EnterpriseDocsMatricesKnownLimitsPage, EnterpriseDocsMatricesKnownLimitsQualificationClass,
    EnterpriseDocsMatricesKnownLimitsRow, EnterpriseDocsMatricesKnownLimitsSummary,
    EnterpriseDocsMatricesKnownLimitsSupportExport, KnownLimitCompletenessClass,
    KnownLimitsDeclaration, MatrixCompletenessClass, MatrixDeclaration, ProofCurrencyClass,
    ProofCurrencyDeclaration, ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_ARTIFACT_REF,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_DEFECT_RECORD_KIND,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_DOC_REF,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_PAGE_RECORD_KIND,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_ROW_RECORD_KIND,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SCHEMA_VERSION,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SHARED_CONTRACT_REF,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SUMMARY_RECORD_KIND,
    ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SUPPORT_EXPORT_RECORD_KIND,
};

pub use stabilize_deployment_and_residency_truth::{
    audit_deployment_residency_stabilize_page, seeded_deployment_residency_input,
    seeded_deployment_residency_stabilize_page, validate_deployment_residency_stabilize_page,
    DeploymentProfileClass, DeploymentResidencyInput, DeploymentResidencyPlaneStrip,
    DeploymentResidencyProfileRow, DeploymentResidencyStabilizeDefect,
    DeploymentResidencyStabilizeNarrowReasonClass, DeploymentResidencyStabilizePage,
    DeploymentResidencyStabilizeQualificationClass, DeploymentResidencyStabilizeRow,
    DeploymentResidencyStabilizeSummary, DeploymentResidencyStabilizeSupportExport,
    MirrorOfflineStateClass, TenantOrgScopeClass, DEPLOYMENT_RESIDENCY_STABILIZE_ARTIFACT_REF,
    DEPLOYMENT_RESIDENCY_STABILIZE_DEFECT_RECORD_KIND, DEPLOYMENT_RESIDENCY_STABILIZE_DOC_REF,
    DEPLOYMENT_RESIDENCY_STABILIZE_PAGE_RECORD_KIND,
    DEPLOYMENT_RESIDENCY_STABILIZE_ROW_RECORD_KIND, DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION,
    DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF,
    DEPLOYMENT_RESIDENCY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND,
};

pub use deployment_profile_continuity_truth::{
    audit_deployment_profile_continuity_page, seeded_deployment_profile_continuity_input,
    seeded_deployment_profile_continuity_page, validate_deployment_profile_continuity_page,
    ContinuitySurfaceVisibility, DeploymentProfileContinuityDefect,
    DeploymentProfileContinuityInput, DeploymentProfileContinuityNarrowReasonClass,
    DeploymentProfileContinuityPage, DeploymentProfileContinuityQualificationClass,
    DeploymentProfileContinuitySummary, DeploymentProfileContinuitySupportExport,
    DeploymentServiceFact, FactFamilyClass, FreshnessStateClass, HostingPostureClass,
    LocalSafeFallbackCard, LocalSafeStateClass, MirrorFreshnessCard, PlaneClass,
    ResidualDependencyClass, ResidualDependencyRow, SignerContinuityClass, SurfaceReuseRow,
    DEPLOYMENT_PROFILE_CONTINUITY_ARTIFACT_REF, DEPLOYMENT_PROFILE_CONTINUITY_DEFECT_RECORD_KIND,
    DEPLOYMENT_PROFILE_CONTINUITY_DOC_REF, DEPLOYMENT_PROFILE_CONTINUITY_PAGE_RECORD_KIND,
    DEPLOYMENT_PROFILE_CONTINUITY_SCHEMA_VERSION,
    DEPLOYMENT_PROFILE_CONTINUITY_SHARED_CONTRACT_REF,
    DEPLOYMENT_PROFILE_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND,
};

pub use stabilize_organization_admin_provisioning_and_seat_lifecycle_truth::{
    audit_organization_admin_truth_page, seeded_organization_admin_truth_page,
    validate_organization_admin_truth_page, AdminFailureKind, AdminLifecycleActionLineage,
    AdminSyncFreshnessClass, DirectoryProviderCard, EntitlementImpactClass, LifecycleFlowClass,
    LifecycleImpactPreview, OrganizationAdminTruthDefect, OrganizationAdminTruthNarrowReasonClass,
    OrganizationAdminTruthPage, OrganizationAdminTruthQualificationClass,
    OrganizationAdminTruthSummary, OrganizationAdminTruthSupportExport, OrganizationDeploymentMode,
    OrganizationOverviewCard, OrganizationProvisioningClass, ProviderStateClass,
    RolloutRingAuditRow, RolloutRingClass, RolloutRingStateClass, SeatClass, SeatLifecycleRow,
    SeatLifecycleStateClass, SeatSummary, SurfaceVisibility, DIRECTORY_PROVIDER_CARD_RECORD_KIND,
    LIFECYCLE_IMPACT_PREVIEW_RECORD_KIND, ORGANIZATION_ADMIN_TRUTH_ARTIFACT_REF,
    ORGANIZATION_ADMIN_TRUTH_DEFECT_RECORD_KIND, ORGANIZATION_ADMIN_TRUTH_DOC_REF,
    ORGANIZATION_ADMIN_TRUTH_PAGE_RECORD_KIND, ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
    ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF,
    ORGANIZATION_ADMIN_TRUTH_SUPPORT_EXPORT_RECORD_KIND, ORGANIZATION_OVERVIEW_CARD_RECORD_KIND,
    ROLLOUT_RING_AUDIT_ROW_RECORD_KIND, SEAT_LIFECYCLE_ROW_RECORD_KIND,
};

pub use finalize_backup_restore_failover_and_local_core_continuity::{
    audit_backup_restore_failover_page, seeded_backup_restore_failover_page,
    validate_backup_restore_failover_page, BackupDeclaration, BackupRestoreFailoverDefect,
    BackupRestoreFailoverNarrowReasonClass, BackupRestoreFailoverPage,
    BackupRestoreFailoverQualificationClass, BackupRestoreFailoverRow,
    BackupRestoreFailoverSummary, BackupRestoreFailoverSupportExport, BackupStateClass,
    EnterpriseProfileClass, FailoverBehaviorClass, FailoverContinuityDeclaration,
    LocalCoreContinuityPostureClass, RestoreTestDeclaration, RestoreTestPostureClass,
    BACKUP_RESTORE_FAILOVER_ARTIFACT_REF, BACKUP_RESTORE_FAILOVER_DEFECT_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_DOC_REF, BACKUP_RESTORE_FAILOVER_PAGE_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_ROW_RECORD_KIND, BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
    BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF, BACKUP_RESTORE_FAILOVER_SUMMARY_RECORD_KIND,
    BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND,
};

pub use stabilize_approval_ticket_audit_and_target_identity_lineage::{
    audit_stabilize_approval_ticket_page, seeded_stabilize_approval_ticket_page,
    validate_stabilize_approval_ticket_page, StabilizeApprovalTicketDefect,
    StabilizeApprovalTicketNarrowReasonClass, StabilizeApprovalTicketPage,
    StabilizeApprovalTicketQualificationClass, StabilizeApprovalTicketRow,
    StabilizeApprovalTicketSummary, StabilizeApprovalTicketSupportExport,
    APPROVAL_TICKET_BETA_CONTRACT_REF, STABILIZE_APPROVAL_TICKET_ARTIFACT_REF,
    STABILIZE_APPROVAL_TICKET_DEFECT_RECORD_KIND, STABILIZE_APPROVAL_TICKET_DOC_REF,
    STABILIZE_APPROVAL_TICKET_PAGE_RECORD_KIND, STABILIZE_APPROVAL_TICKET_ROW_RECORD_KIND,
    STABILIZE_APPROVAL_TICKET_SCHEMA_VERSION, STABILIZE_APPROVAL_TICKET_SHARED_CONTRACT_REF,
    STABILIZE_APPROVAL_TICKET_SUPPORT_EXPORT_RECORD_KIND,
};

pub use freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix::{
    current_stable_m5_runtime_authority_matrix_export,
    frozen_stable_m5_runtime_authority_matrix_packet, M5ApprovalTicketPosture, M5CapabilityClass,
    M5DegradedFallback, M5ExecutingSurface, M5RuntimeAuthorityConsumerSurface,
    M5RuntimeAuthorityDowngradeTrigger, M5RuntimeAuthorityMatrixArtifactError,
    M5RuntimeAuthorityMatrixConsumerProjection, M5RuntimeAuthorityMatrixPacket,
    M5RuntimeAuthorityMatrixPacketInput, M5RuntimeAuthorityMatrixProofFreshness,
    M5RuntimeAuthorityMatrixSurfaceRow, M5RuntimeAuthorityMatrixTrustReview,
    M5RuntimeAuthorityMatrixViolation, M5RuntimeAuthorityQualificationClass, M5SandboxProfile,
    M5SecretScope, M5UnsupportedProfileBehavior,
    M5_RUNTIME_AUTHORITY_MATRIX_APPROVAL_TICKET_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_ARTIFACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_AUTHORITY_TICKET_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_DOC_REF, M5_RUNTIME_AUTHORITY_MATRIX_FIXTURE_DIR,
    M5_RUNTIME_AUTHORITY_MATRIX_ISSUER_CONTRACT_REF, M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID,
    M5_RUNTIME_AUTHORITY_MATRIX_RECORD_KIND, M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SCHEMA_VERSION,
    M5_RUNTIME_AUTHORITY_MATRIX_SECRET_BOUNDARY_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SECRET_HANDLE_CONTRACT_REF,
    M5_RUNTIME_AUTHORITY_MATRIX_SUMMARY_REF,
};

pub use implement_execution_surface_classes_sandbox_profile_descriptors_and_unsupported_or_stricter_profile_truth::{
    current_stable_m5_execution_surface_resolution_export,
    frozen_stable_m5_execution_surface_resolution_packet, resolve_surface_on_platform,
    M5ExecutionBackendClass, M5ExecutionLaunchPath, M5ExecutionLaunchPathBinding,
    M5ExecutionPlatform, M5ExecutionSurfaceResolutionArtifactError,
    M5ExecutionSurfaceResolutionConsumerProjection, M5ExecutionSurfaceResolutionPacket,
    M5ExecutionSurfaceResolutionPacketInput, M5ExecutionSurfaceResolutionTrustReview,
    M5ExecutionSurfaceResolutionViolation, M5PlatformResolution, M5ProfileResolutionStatus,
    M5ResolvedSurfaceRow, M5SandboxProfileDescriptor,
    M5_EXECUTION_SURFACE_RESOLUTION_ARTIFACT_REF, M5_EXECUTION_SURFACE_RESOLUTION_DOC_REF,
    M5_EXECUTION_SURFACE_RESOLUTION_FIXTURE_DIR, M5_EXECUTION_SURFACE_RESOLUTION_PACKET_ID,
    M5_EXECUTION_SURFACE_RESOLUTION_RECORD_KIND, M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_REF,
    M5_EXECUTION_SURFACE_RESOLUTION_SCHEMA_VERSION, M5_EXECUTION_SURFACE_RESOLUTION_SUMMARY_REF,
    M5_SANDBOX_PROFILE_DESCRIPTOR_VERSION,
};

pub use add_launch_inspector_and_command_runtime_explain_sheets_that_answer_where_this_runs_why_this_toolchain_what_it_can_acces::{
    current_stable_m5_launch_inspector_export, frozen_stable_m5_launch_inspector_packet,
    M5ExplainDegradation, M5ExplainDegradationReason, M5ExplainStatus, M5LaunchExplainSheet,
    M5LaunchInspectorArtifactError, M5LaunchInspectorConsumerProjection, M5LaunchInspectorPacket,
    M5LaunchInspectorPacketInput, M5LaunchInspectorProofFreshness, M5LaunchInspectorTrustReview,
    M5LaunchInspectorViolation, M5LaunchRoute, M5ToolchainSelectionReason, M5WhatItCanAccess,
    M5WhereItRuns, M5WhoApprovedIt, M5WhyThisToolchain, M5_LAUNCH_INSPECTOR_ARTIFACT_REF,
    M5_LAUNCH_INSPECTOR_DOC_REF, M5_LAUNCH_INSPECTOR_FIXTURE_DIR, M5_LAUNCH_INSPECTOR_PACKET_ID,
    M5_LAUNCH_INSPECTOR_RECORD_KIND, M5_LAUNCH_INSPECTOR_SCHEMA_REF,
    M5_LAUNCH_INSPECTOR_SCHEMA_VERSION, M5_LAUNCH_INSPECTOR_SUMMARY_REF,
};

pub use implement_approval_ticket_issuance_deny_reason_packets_replay_nonce_or_expiry_enforcement_and_local_first_verification_f::{
    build_ledger_packet, current_stable_m5_approval_ticket_ledger_export, denied_tickets,
    frozen_stable_m5_approval_ticket_ledger_packet, valid_tickets, M5ApprovalTicket,
    M5ApprovalTicketConsumerProjection, M5ApprovalTicketLedgerArtifactError,
    M5ApprovalTicketLedgerPacket, M5ApprovalTicketLedgerPacketInput,
    M5ApprovalTicketLedgerViolation, M5ApprovalTicketProofFreshness, M5ApprovalTicketTrustReview,
    M5LocalFirstVerification, M5LocalFirstVerificationMethod, M5ReplayProtection, M5TicketActionClass,
    M5TicketActor, M5TicketBinding, M5TicketDenyDimension, M5TicketDenyReason,
    M5TicketIssuanceLineage, M5TicketTarget, M5TicketValidity, M5TicketVerificationState,
    M5_APPROVAL_TICKET_LEDGER_ARTIFACT_REF, M5_APPROVAL_TICKET_LEDGER_DOC_REF,
    M5_APPROVAL_TICKET_LEDGER_FIXTURE_DIR, M5_APPROVAL_TICKET_LEDGER_PACKET_ID,
    M5_APPROVAL_TICKET_LEDGER_RECORD_KIND, M5_APPROVAL_TICKET_LEDGER_SCHEMA_REF,
    M5_APPROVAL_TICKET_LEDGER_SCHEMA_VERSION, M5_APPROVAL_TICKET_LEDGER_SUMMARY_REF,
};

pub use ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e::{
    current_stable_m5_capability_envelope_export, frozen_stable_m5_capability_envelope_packet,
    M5AllowedScopeEntry, M5AllowedScopeKind, M5CapabilityEnvelope,
    M5CapabilityEnvelopeArtifactError, M5CapabilityEnvelopeConsumerProjection,
    M5CapabilityEnvelopePacket, M5CapabilityEnvelopePacketInput,
    M5CapabilityEnvelopeProofFreshness, M5CapabilityEnvelopeTrustReview,
    M5CapabilityEnvelopeViolation, M5EnvelopeActor, M5EnvelopeActorClass,
    M5EnvelopeAuditLineage, M5EnvelopeExpiry, M5EnvelopeIssuerClass, M5EnvelopeTarget,
    M5EnvelopeTargetClass, M5PolicyEpochBinding, M5ScopeAccessMode, M5SecretHandleRef,
    M5_CAPABILITY_ENVELOPE_ARTIFACT_REF, M5_CAPABILITY_ENVELOPE_DOC_REF,
    M5_CAPABILITY_ENVELOPE_FIXTURE_DIR, M5_CAPABILITY_ENVELOPE_PACKET_ID,
    M5_CAPABILITY_ENVELOPE_RECORD_KIND, M5_CAPABILITY_ENVELOPE_SCHEMA_REF,
    M5_CAPABILITY_ENVELOPE_SCHEMA_VERSION, M5_CAPABILITY_ENVELOPE_SUMMARY_REF,
};

pub use ship_child_envelope_derivation_nested_launch_narrowing_handle_only_secret_projection_and_no_ambient_privilege_enforcemen::{
    build_derivation_packet, current_stable_m5_child_envelope_derivation_export,
    frozen_stable_m5_child_envelope_derivation_packet, narrowed_derivations, nominal_derivations,
    M5AmbientEnvironmentPosture, M5ChildEnvelope, M5ChildEnvelopeDerivation,
    M5ChildEnvelopeDerivationArtifactError, M5ChildEnvelopeDerivationConsumerProjection,
    M5ChildEnvelopeDerivationPacket, M5ChildEnvelopeDerivationPacketInput,
    M5ChildEnvelopeDerivationProofFreshness, M5ChildEnvelopeDerivationTrustReview,
    M5ChildEnvelopeDerivationViolation, M5DerivationActor, M5DerivationLineage,
    M5DerivationNarrowingDimension, M5EnforcementBackendStatus, M5NestedLaunchLane,
    M5ParentAuthoritySnapshot, M5_CHILD_ENVELOPE_DERIVATION_ARTIFACT_REF,
    M5_CHILD_ENVELOPE_DERIVATION_DOC_REF, M5_CHILD_ENVELOPE_DERIVATION_FIXTURE_DIR,
    M5_CHILD_ENVELOPE_DERIVATION_PACKET_ID, M5_CHILD_ENVELOPE_DERIVATION_RECORD_KIND,
    M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_REF, M5_CHILD_ENVELOPE_DERIVATION_SCHEMA_VERSION,
    M5_CHILD_ENVELOPE_DERIVATION_SUMMARY_REF,
};

pub use finalize_managed_workspace_lifecycle_truth::{
    audit_finalize_managed_workspace_lifecycle_truth_page,
    seeded_finalize_managed_workspace_lifecycle_truth_page,
    seeded_managed_workspace_lifecycle_input,
    validate_finalize_managed_workspace_lifecycle_truth_page,
    FinalizeManagedWorkspaceLifecycleNarrowReasonClass,
    FinalizeManagedWorkspaceLifecycleQualificationClass,
    FinalizeManagedWorkspaceLifecycleTruthDefect, FinalizeManagedWorkspaceLifecycleTruthPage,
    FinalizeManagedWorkspaceLifecycleTruthRow, FinalizeManagedWorkspaceLifecycleTruthSummary,
    FinalizeManagedWorkspaceLifecycleTruthSupportExport, ManagedDestructiveOperationClass,
    ManagedFallbackPathClass, ManagedJoinModeClass, ManagedPersistenceClass,
    ManagedProvisioningEvent, ManagedProvisioningStateClass, ManagedRebuildPlan,
    ManagedSecretModelClass, ManagedShareHandoffToken, ManagedSuspendResumeCheckpoint,
    ManagedWorkspaceDescriptor, ManagedWorkspaceLifecycleInputRow,
    ManagedWorkspaceLifecycleTruthInput, FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_ARTIFACT_REF,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_DEFECT_RECORD_KIND,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_DOC_REF,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_PAGE_RECORD_KIND,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_ROW_RECORD_KIND,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SCHEMA_VERSION,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SHARED_CONTRACT_REF,
    FINALIZE_MANAGED_WORKSPACE_LIFECYCLE_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use stabilize_transport_policy_proxy_resolution_trust_store_and_mirror_route::{
    audit_transport_policy_inspector_page, seeded_transport_policy_inspector_page,
    validate_transport_policy_inspector_page, EgressDecisionClass, EndpointClass,
    HandshakeOutcomeClass, MirrorRouteState, NetworkEventRecord, PlaneStatusClass,
    ProxyResolutionStep, RouteSourceClass, TransportPolicyInspectorDefect,
    TransportPolicyInspectorNarrowReasonClass, TransportPolicyInspectorPage,
    TransportPolicyInspectorQualificationClass, TransportPolicyInspectorSummary,
    TransportPolicyInspectorSupportExport, TransportPolicyRecord, TransportTrustLayerClass,
    TrustLayerSnapshot, NETWORK_EVENT_RECORD_KIND, TRANSPORT_POLICY_INSPECTOR_ARTIFACT_REF,
    TRANSPORT_POLICY_INSPECTOR_DEFECT_RECORD_KIND, TRANSPORT_POLICY_INSPECTOR_DOC_REF,
    TRANSPORT_POLICY_INSPECTOR_PAGE_RECORD_KIND, TRANSPORT_POLICY_INSPECTOR_SCHEMA_VERSION,
    TRANSPORT_POLICY_INSPECTOR_SHARED_CONTRACT_REF,
    TRANSPORT_POLICY_INSPECTOR_STABLE_PROOF_INDEX_REF,
    TRANSPORT_POLICY_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND, TRANSPORT_POLICY_RECORD_KIND,
};

pub use finalize_open_vs_paid_boundary_and_offboarding::{
    audit_open_vs_paid_boundary_page, seeded_open_vs_paid_boundary_input,
    seeded_open_vs_paid_boundary_page, validate_open_vs_paid_boundary_page,
    CapabilityBoundaryClass, CapabilityBoundaryInputRow, CapabilityFamilyClass,
    ExportRetentionClass, GraceWindowStateClass, OffboardingOutcomeClass, OffboardingPacket,
    OpenVsPaidBoundaryDefect, OpenVsPaidBoundaryInput, OpenVsPaidBoundaryNarrowReasonClass,
    OpenVsPaidBoundaryPage, OpenVsPaidBoundaryQualificationClass, OpenVsPaidBoundaryRow,
    OpenVsPaidBoundarySummary, OpenVsPaidBoundarySupportExport, UsageExportAvailabilityClass,
    UsageExportPacket, OFFBOARDING_PACKET_RECORD_KIND, OPEN_VS_PAID_BOUNDARY_ARTIFACT_REF,
    OPEN_VS_PAID_BOUNDARY_DEFECT_RECORD_KIND, OPEN_VS_PAID_BOUNDARY_DOC_REF,
    OPEN_VS_PAID_BOUNDARY_PAGE_RECORD_KIND, OPEN_VS_PAID_BOUNDARY_ROW_RECORD_KIND,
    OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION, OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF,
    OPEN_VS_PAID_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND, USAGE_EXPORT_PACKET_RECORD_KIND,
};
