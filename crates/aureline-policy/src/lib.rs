//! Policy simulation, exception lifecycle, and remembered-decision contracts.
//!
//! This crate owns the beta policy-governance object model consumed by shell,
//! support, admin, and offline-file control paths. It does not evaluate a full
//! policy language; it provides typed preview records, bounded exception and
//! waiver rows, remembered-decision drift checks, and metadata-safe support
//! exports.

#![doc(html_root_url = "https://docs.rs/aureline-policy/0.0.0")]

pub mod simulation;

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
