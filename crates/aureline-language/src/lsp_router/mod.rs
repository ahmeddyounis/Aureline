//! LSP router records and launch-language routing decisions.
//!
//! This module owns the first bounded language-router implementation for
//! launch-language LSP lanes. It emits the existing router decision and
//! provider-status packet shapes, and it keeps LSP host health, syntax fallback,
//! locality, restart-budget, and degraded-state labels visible to downstream
//! shell, diagnostics, and support consumers.

mod records;
mod router;

pub use records::{
    CapabilityClass, CompletenessClass, CoordinateTranslationRequirementClass, DecisionOutcome,
    DegradedStateClass, EpochBinding, EpochRoleClass, FallbackClass, FaultDomainId, FreshnessClass,
    HealthState, LaneClass, LanguageServerHostIdentity, LanguageServerHostStatus, LocalityClass,
    PlacementPreferenceClass, PrecedenceBand, ProviderFamily, ProviderKind, ProviderPolicyContext,
    ProviderRoleClass, ProviderStackRow, ProviderStatusRowRecord, RedactionClass,
    RequestedAuthorityFloorClass, RouterDecisionRecord, RouterDecisionSchemaVersion,
    RouterRequestContext, RouterTrustState, RoutingContext, ScopeClaimClass, ScopeLimitClass,
    SupportClass, SurfaceClass, SurfaceReport, SurfaceSupportClass, SurfaceSupportClassRow,
    ROUTER_DECISION_SCHEMA_VERSION,
};
pub use router::{LspRouter, RouterRequest, WorkspaceLocalRouterRequest};
