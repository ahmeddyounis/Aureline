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

pub mod authority;
pub mod finalize_backup_restore_failover_and_local_core_continuity;
pub mod finalize_open_vs_paid_boundary_and_offboarding;
pub mod finalize_signed_policy_bundle_offline_entitlement_and_mirror;
pub mod finalize_the_secret_broker_handle_only_delegated_and;
pub mod harden_enterprise_network_proxy_pac_manual_system_proxy;
pub mod harden_os_keychain_and_trust_store_integration_trust;
pub mod runtime_authority_issuers;
pub mod simulation;
pub mod stabilize_approval_ticket_audit_and_target_identity_lineage;
pub mod stabilize_deployment_and_residency_truth;
pub mod stabilize_effective_policy_remembered_decision_waiver_expiry_and;

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
    validate_finalize_signed_policy_bundle_page, BundleImportFlowClass, BundleKindClass,
    FinalizeSignedPolicyBundleDefect, FinalizeSignedPolicyBundleNarrowReasonClass,
    FinalizeSignedPolicyBundlePage, FinalizeSignedPolicyBundleQualificationClass,
    FinalizeSignedPolicyBundleRow, FinalizeSignedPolicyBundleSummary,
    FinalizeSignedPolicyBundleSupportExport, GracePostureClass, OfflineGraceState,
    PolicyBundleSimulationPacket, PolicyEpochState,
    OFFLINE_ENTITLEMENT_VERIFIER_CONTRACT_REF,
    SIGNED_POLICY_BUNDLE_FINALIZE_ARTIFACT_REF, SIGNED_POLICY_BUNDLE_FINALIZE_DEFECT_RECORD_KIND,
    SIGNED_POLICY_BUNDLE_FINALIZE_DOC_REF, SIGNED_POLICY_BUNDLE_FINALIZE_PAGE_RECORD_KIND,
    SIGNED_POLICY_BUNDLE_FINALIZE_ROW_RECORD_KIND, SIGNED_POLICY_BUNDLE_FINALIZE_SCHEMA_VERSION,
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
    FINALIZE_SECRET_BROKER_PAGE_RECORD_KIND, FINALIZE_SECRET_BROKER_REMEMBERED_APPROVAL_RECORD_KIND,
    FINALIZE_SECRET_BROKER_ROW_RECORD_KIND, FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
    FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF, FINALIZE_SECRET_BROKER_SUMMARY_RECORD_KIND,
    FINALIZE_SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND, SECRET_BROKER_BETA_CONTRACT_REF,
};

pub use harden_os_keychain_and_trust_store_integration_trust::{
    audit_harden_os_keychain_trust_store_page, seeded_harden_os_keychain_trust_store_page,
    validate_harden_os_keychain_trust_store_page, HardenOsKeychainTrustStoreDefect,
    HardenOsKeychainTrustStoreNarrowReasonClass, HardenOsKeychainTrustStorePage,
    HardenOsKeychainTrustStoreQualificationClass, HardenOsKeychainTrustStoreSummary,
    HardenOsKeychainTrustStoreSupportExport, SessionImpactClass, TrustStoreChangeAttributionClass,
    TrustStoreChangeClass, TrustStoreChangeEvent, TrustStoreLayerClass, TrustStoreLayerHealthClass,
    TrustStoreLayerRow, TrustStoreRepairActionClass,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_ARTIFACT_REF, HARDEN_OS_KEYCHAIN_TRUST_STORE_CHANGE_EVENT_RECORD_KIND,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_DEFECT_RECORD_KIND, HARDEN_OS_KEYCHAIN_TRUST_STORE_DOC_REF,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_PAGE_RECORD_KIND, HARDEN_OS_KEYCHAIN_TRUST_STORE_ROW_RECORD_KIND,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION, HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_SUMMARY_RECORD_KIND,
    HARDEN_OS_KEYCHAIN_TRUST_STORE_SUPPORT_EXPORT_RECORD_KIND,
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
    DEPLOYMENT_RESIDENCY_STABILIZE_ROW_RECORD_KIND,
    DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION,
    DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF,
    DEPLOYMENT_RESIDENCY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND,
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

pub use finalize_open_vs_paid_boundary_and_offboarding::{
    audit_open_vs_paid_boundary_page, seeded_open_vs_paid_boundary_input,
    seeded_open_vs_paid_boundary_page, validate_open_vs_paid_boundary_page,
    CapabilityBoundaryClass, CapabilityBoundaryInputRow, CapabilityFamilyClass,
    ExportRetentionClass, GraceWindowStateClass, OffboardingOutcomeClass,
    OffboardingPacket, OpenVsPaidBoundaryDefect, OpenVsPaidBoundaryInput,
    OpenVsPaidBoundaryNarrowReasonClass, OpenVsPaidBoundaryPage,
    OpenVsPaidBoundaryQualificationClass, OpenVsPaidBoundaryRow,
    OpenVsPaidBoundarySummary, OpenVsPaidBoundarySupportExport, UsageExportAvailabilityClass,
    UsageExportPacket, OFFBOARDING_PACKET_RECORD_KIND, OPEN_VS_PAID_BOUNDARY_ARTIFACT_REF,
    OPEN_VS_PAID_BOUNDARY_DEFECT_RECORD_KIND, OPEN_VS_PAID_BOUNDARY_DOC_REF,
    OPEN_VS_PAID_BOUNDARY_PAGE_RECORD_KIND, OPEN_VS_PAID_BOUNDARY_ROW_RECORD_KIND,
    OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION, OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF,
    OPEN_VS_PAID_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND, USAGE_EXPORT_PACKET_RECORD_KIND,
};
