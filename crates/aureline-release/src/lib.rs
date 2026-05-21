//! Release engineering contracts shared by UI, headless, support, and audit flows.
//!
//! This crate owns release-object types that must stay independent of any
//! single renderer, CI script, or support export. The first module is the
//! release-center object model: release candidates, version-bump proposals,
//! publish targets, artifact bundles, promotion steps, and scoped
//! rollback/revocation records. The correction-train module formalizes the
//! shared correction-train, hotfix, and backport packet form on top of the
//! same rollback and release-candidate refs.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod correction_train;
pub mod release_center_model;

pub use correction_train::{
    BackportDecision, BackportMatrixRow, CorrectionEvidence, CorrectionItem, CorrectionRisk,
    CorrectionScope, CorrectionTrainPacket, CorrectionTrainViolation, CorrectionTriage,
    PacketTemplates, ReleaseNotesRefs, SupportProjection, TargetChannelUpdate, TriageLane,
    CORRECTION_TRAIN_PACKET_RECORD_KIND, CORRECTION_TRAIN_PACKET_SCHEMA_VERSION,
    SECURITY_OR_TRUST_ISSUE_CLASSES, SHARED_PACKET_FORM_TERMS, SUPPORTED_LINE_CLASSES,
};

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
