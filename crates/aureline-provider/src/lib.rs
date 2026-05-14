//! Connected-provider registry alpha and publish-later continuity records.
//!
//! This crate owns the first consuming implementation for the connected-provider
//! registry alpha. It keeps code-host, issue, and CI/check provider descriptors
//! on one typed contract, projects the publish-later queue through explicit
//! freshness and dependency truth, and exposes an export-safe support projection
//! for reviewer and diagnostic surfaces.

#![doc(html_root_url = "https://docs.rs/aureline-provider/0.0.0")]

pub mod publish_later;
pub mod registry;

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
