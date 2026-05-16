//! Connected-provider registry alpha, publish-later continuity, approval-ticket
//! records, and account-scope beta page.
//!
//! This crate owns the first consuming implementation for connected-provider
//! alpha authority. It keeps code-host, issue, and CI/check provider descriptors
//! on one typed contract, projects the publish-later queue through explicit
//! freshness and dependency truth, binds high-risk provider/helper mutations
//! to export-safe approval-ticket or reviewed-scope lineage, and (since the
//! account-scope beta) separates connected-account, installation-grant, and
//! delegated-credential authority on provider-linked rows while resolving the
//! effective scope and surfacing scope-drift events that force reapproval or
//! downgrade.

#![doc(html_root_url = "https://docs.rs/aureline-provider/0.0.0")]

pub use aureline_auth::{KeyMode, RegionMode, ResidencyMode};

pub mod account_scope;
pub mod approval_tickets;
pub mod publish_later;
pub mod registry;
pub mod route_resolution;

pub use account_scope::{
    audit_account_scope_beta_page, seeded_account_scope_beta_page,
    validate_account_scope_beta_page, AccountAuthSourceClass, AccountLifecycleStateClass,
    AccountScopeBetaDefect, AccountScopeBetaDefectKind, AccountScopeBetaPage,
    AccountScopeBetaProfileClass, AccountScopeBetaSummary, AccountScopeBetaSupportExport,
    ActingIdentityClass, AuthorityDecisionClass, AuthorityDowngradeClass, ConnectedAccountRow,
    ConnectedAccountSubject, DelegatedCredentialLifecycleStateClass, DelegatedCredentialRow,
    EffectiveScopeResolutionRow, GrantResolutionReasonClass, InstallationGrantLifecycleStateClass,
    InstallationGrantRow, ProviderHostBinding, ProviderTargetIdentity, ReapprovalRouteClass,
    RequestedActionClass, ScopeDriftEvent, ScopeDriftTriggerClass,
    ACCOUNT_SCOPE_BETA_CONNECTED_ACCOUNT_ROW_RECORD_KIND, ACCOUNT_SCOPE_BETA_DEFECT_RECORD_KIND,
    ACCOUNT_SCOPE_BETA_DELEGATED_CREDENTIAL_ROW_RECORD_KIND,
    ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND,
    ACCOUNT_SCOPE_BETA_INSTALLATION_GRANT_ROW_RECORD_KIND, ACCOUNT_SCOPE_BETA_PAGE_RECORD_KIND,
    ACCOUNT_SCOPE_BETA_SCHEMA_VERSION, ACCOUNT_SCOPE_BETA_SCOPE_DRIFT_EVENT_RECORD_KIND,
    ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF, ACCOUNT_SCOPE_BETA_SOURCE_MATRIX_REF,
    ACCOUNT_SCOPE_BETA_SUMMARY_RECORD_KIND, ACCOUNT_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use approval_tickets::{
    ApprovalActorClass, ApprovalActorLineageEntry, ApprovalActorScope, ApprovalAuthSourceClass,
    ApprovalAuthorityKind, ApprovalIssuerClass, ApprovalRequestOriginClass,
    ApprovalSideEffectClass, ApprovalTargetClass, ApprovalTargetIdentity,
    ApprovalTicketAlphaCoverage, ApprovalTicketAlphaPacket, ApprovalTicketAlphaRecord,
    ApprovalTicketAlphaValidationFinding, ApprovalTicketAlphaValidationReport,
    ApprovalTicketContractRefs, ApprovalTicketLineageSummary, ApprovalTicketSpendAttempt,
    ApprovalTicketSpendSummary, ApprovalTicketSupportAdminPacket, ApprovalTicketUsePosture,
    HighRiskActionClass, MutationAuthorityBinding, MutationAuthorityRequirement,
    MutationAuthoritySummary, NativeReapprovalRoute, ReviewedScopeAlphaRecord,
    TicketEvaluationOutcome, APPROVAL_TICKET_ALPHA_PACKET_RECORD_KIND,
    APPROVAL_TICKET_ALPHA_RECORD_KIND, APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
    APPROVAL_TICKET_ALPHA_VALIDATION_REPORT_RECORD_KIND, APPROVAL_TICKET_SPEND_ATTEMPT_RECORD_KIND,
    APPROVAL_TICKET_SUPPORT_ADMIN_PACKET_RECORD_KIND, MUTATION_AUTHORITY_BINDING_RECORD_KIND,
    REVIEWED_SCOPE_ALPHA_RECORD_KIND,
};
pub use publish_later::{
    DependencyClass, DependencyState, ExportSafetyClass, PublishLaterQueueAlphaItem,
    QueueActionKind, QueueDependency, QueueNextSafeActionClass, QueueState, ReauthRequirementClass,
    RescopeRequirementClass,
};
pub use route_resolution::{
    audit_route_resolution_beta_page, seeded_route_resolution_beta_page,
    validate_route_resolution_beta_page, AuthorityTruthPanel, AuthorityTruthState,
    BrowserHandoffPanel, BrowserHandoffReasonClass, FallbackDescriptor, GrantDescriptor,
    LaneClass, RouteActionClass, RouteChoiceClass, RouteDegradedStateClass, RouteDescriptor,
    RouteFreshness, RouteOwnerClass, RouteOwnerDescriptor, RouteResolutionBetaDefect,
    RouteResolutionBetaDefectKind, RouteResolutionBetaPage, RouteResolutionBetaSummary,
    RouteResolutionBetaSupportExport, RouteResolutionRow,
    ROUTE_RESOLUTION_BETA_AUTHORITY_TRUTH_PANEL_RECORD_KIND,
    ROUTE_RESOLUTION_BETA_BROWSER_HANDOFF_PANEL_RECORD_KIND,
    ROUTE_RESOLUTION_BETA_DEFECT_RECORD_KIND, ROUTE_RESOLUTION_BETA_PAGE_RECORD_KIND,
    ROUTE_RESOLUTION_BETA_ROW_RECORD_KIND, ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
    ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF, ROUTE_RESOLUTION_BETA_SOURCE_MATRIX_REF,
    ROUTE_RESOLUTION_BETA_SUMMARY_RECORD_KIND, ROUTE_RESOLUTION_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use registry::{
    validate_provider_capability_lifecycle_claim, ActorScope, ArtifactTrustClass,
    ClaimedProviderSurface, ConnectedProviderAlphaPacket, ConnectedProviderDescriptor,
    ConnectedProviderRegistryPacket, ContractRefs, FindingSeverity, FreshnessLabel, FreshnessTruth,
    LocalTruthAuthorityClass, MutationSurfaceState, PipelineOverlayDescriptor, PipelineOverlayKind,
    ProviderActorClass, ProviderAlphaCoverage, ProviderAlphaSupportExport,
    ProviderAlphaValidationFinding, ProviderAlphaValidationReport, ProviderAuthSourceClass,
    ProviderFallbackMode, ProviderFamily, ProviderFixtureMetadata, ProviderObjectKind,
    ProviderRouteOriginLabel, ProviderSource, ProviderSourceClass, ProviderSurfaceClass,
    ProviderTruthSourceClass, RedactionClass, RunControlClass, RunControlDescriptor,
    RunControlMutationMode, StaleTargetRiskClass, SurfaceActionDescriptor, TargetRef,
    UpstreamMutationScopeClass, CONNECTED_PROVIDER_ALPHA_PACKET_RECORD_KIND,
    CONNECTED_PROVIDER_DESCRIPTOR_RECORD_KIND, CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
    PIPELINE_OVERLAY_DESCRIPTOR_RECORD_KIND, PROVIDER_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
    PROVIDER_ALPHA_VALIDATION_REPORT_RECORD_KIND, PROVIDER_SURFACE_CLAIM_RECORD_KIND,
    RUN_CONTROL_DESCRIPTOR_RECORD_KIND,
};
