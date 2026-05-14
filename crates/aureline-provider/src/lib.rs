//! Connected-provider registry alpha, publish-later continuity, and approval-ticket records.
//!
//! This crate owns the first consuming implementation for connected-provider
//! alpha authority. It keeps code-host, issue, and CI/check provider descriptors
//! on one typed contract, projects the publish-later queue through explicit
//! freshness and dependency truth, and binds high-risk provider/helper mutations
//! to export-safe approval-ticket or reviewed-scope lineage.

#![doc(html_root_url = "https://docs.rs/aureline-provider/0.0.0")]

pub mod approval_tickets;
pub mod publish_later;
pub mod registry;

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
pub use registry::{
    ActorScope, ArtifactTrustClass, ClaimedProviderSurface, ConnectedProviderAlphaPacket,
    ConnectedProviderDescriptor, ConnectedProviderRegistryPacket, ContractRefs, FindingSeverity,
    FreshnessLabel, FreshnessTruth, LocalTruthAuthorityClass, MutationSurfaceState,
    PipelineOverlayDescriptor, PipelineOverlayKind, ProviderActorClass, ProviderAlphaCoverage,
    ProviderAlphaSupportExport, ProviderAlphaValidationFinding, ProviderAlphaValidationReport,
    ProviderAuthSourceClass, ProviderFallbackMode, ProviderFamily, ProviderFixtureMetadata,
    ProviderObjectKind, ProviderSource, ProviderSourceClass, ProviderSurfaceClass,
    ProviderTruthSourceClass, RedactionClass, RunControlClass, RunControlDescriptor,
    RunControlMutationMode, StaleTargetRiskClass, SurfaceActionDescriptor, TargetRef,
    UpstreamMutationScopeClass, CONNECTED_PROVIDER_ALPHA_PACKET_RECORD_KIND,
    CONNECTED_PROVIDER_DESCRIPTOR_RECORD_KIND, CONNECTED_PROVIDER_REGISTRY_SCHEMA_VERSION,
    PIPELINE_OVERLAY_DESCRIPTOR_RECORD_KIND, PROVIDER_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
    PROVIDER_ALPHA_VALIDATION_REPORT_RECORD_KIND, PROVIDER_SURFACE_CLAIM_RECORD_KIND,
    RUN_CONTROL_DESCRIPTOR_RECORD_KIND,
};
