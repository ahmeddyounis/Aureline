//! Release engineering contracts shared by UI, headless, support, and audit flows.
//!
//! This crate owns release-object types that must stay independent of any
//! single renderer, CI script, or support export. The first module is the
//! release-center object model: release candidates, version-bump proposals,
//! publish targets, artifact bundles, promotion steps, and scoped
//! rollback/revocation records.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod release_center_model;

pub use release_center_model::{
    ArtifactBundleCard, ArtifactFamilyClass, ArtifactGraphConsistency, ArtifactPayloadRefs,
    AuthSourceClass, BlastRadiusClass, BreakGlassDisclosure, BreakGlassStateClass,
    CompatibilityImpactClass, CompatibilityNote, ContinuityClass, ContinuityNote,
    DryRunAvailabilityClass, DryRunDisclosure, EvidenceFreshnessClass, EvidenceRef,
    ImmutableDigest, PromotionEventClass, PromotionReadiness, PromotionStage,
    PromotionTimelineStep, PublishTargetClass, PublishTargetDescriptor, ReleaseCandidate,
    ReleaseCenterHeadlessPlan, ReleaseCenterModelValidationReport, ReleaseCenterModelViolation,
    ReleaseCenterObjectIdentityIndex, ReleaseCenterObjectModel, ReleaseCenterSupportAuditExport,
    ReleaseCenterUiState, RollbackOrRevocationKind, RollbackOrRevocationRecord, RolloutRing,
    SemanticChangeClass, SignatureStateClass, TargetMutabilityClass, TargetVisibilityClass,
    VersionBumpProposal, RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND,
    RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION,
};
