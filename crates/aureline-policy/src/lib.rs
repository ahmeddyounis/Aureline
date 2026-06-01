//! Policy simulation, authority-ticket, and remembered-decision contracts.
//!
//! This crate owns the beta policy-governance object model consumed by shell,
//! support, admin, and offline-file control paths. It does not evaluate a full
//! policy language; it provides typed preview records, bounded exception and
//! waiver rows, authority-ticket lineage for privileged actions,
//! remembered-decision drift checks, and metadata-safe support exports.

#![doc(html_root_url = "https://docs.rs/aureline-policy/0.0.0")]

pub mod authority;
pub mod finalize_signed_policy_bundle_offline_entitlement_and_mirror;
pub mod runtime_authority_issuers;
pub mod simulation;
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
