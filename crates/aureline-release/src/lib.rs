//! Release engineering contracts shared by UI, headless, support, and audit flows.
//!
//! This crate owns release-object types that must stay independent of any
//! single renderer, CI script, or support export. The first module is the
//! release-center object model: release candidates, version-bump proposals,
//! publish targets, artifact bundles, promotion steps, and scoped
//! rollback/revocation records. The correction-train module formalizes the
//! shared correction-train, hotfix, and backport packet form on top of the
//! same rollback and release-candidate refs. The stable-claim-matrix module
//! freezes the stable claim matrix, launch cutline, qualification rows, and
//! shiproom stop rules that decide which surfaces may publish as Stable. The
//! support-class-ledger module is the publication layer on top of that matrix:
//! it publishes the v1.0 support-class assignments, the certified-archetype
//! manifest, and the downgrade automation that narrows a published support
//! class when its backing thins out. The stable-qualification-matrix module
//! finalizes the per-lane qualification rows (desktop, remote/helper,
//! ecosystem, state/schema, provider, accessibility) that ground those claims
//! and, for every cross-binary or cross-service boundary, publishes the
//! mixed-version section — negotiated fields, supported skew window, upgrade and
//! rollback order, and unsupported-state behavior — that decides whether the
//! boundary may inherit a Stable mixed-version claim or is coordinated-upgrade-only.
//! The stable-claim-manifest module is the publication layer that binds all
//! three of those records together: it assigns each published subject one
//! canonical lifecycle label, names the backing claim row, qualification rows, and
//! support-class entry that label depends on, and attaches a packet-freshness SLO
//! so a subject whose proof packet has breached its SLO narrows below the launch
//! cutline automatically before publication.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod correction_train;
pub mod release_center_model;
pub mod stable_claim_manifest;
pub mod stable_claim_matrix;
pub mod stable_qualification_matrix;
pub mod support_class_ledger;

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

pub use stable_claim_manifest::{
    current_stable_claim_manifest, FreshnessSlo, FreshnessSloState, ManifestEntry,
    ManifestExportProjection, ManifestExportRow, ManifestPublicationRecord, ManifestState,
    NarrowingReason, ProofPacket, PublicationAction, PublicationRule, StableClaimManifest,
    StableClaimManifestSummary, StableClaimManifestViolation, STABLE_CLAIM_MANIFEST_JSON,
    STABLE_CLAIM_MANIFEST_PATH, STABLE_CLAIM_MANIFEST_RECORD_KIND,
    STABLE_CLAIM_MANIFEST_SCHEMA_VERSION,
};

pub use stable_claim_matrix::{
    current_stable_claim_matrix, DowngradeReason, LaunchCutline, OwnerSignoff, PromotionDecision,
    PromotionDecisionRecord, QualificationEvidence, QualificationState, QualificationWaiver,
    ShiproomStopRule, StableClaimExportProjection, StableClaimExportRow, StableClaimLevel,
    StableClaimMatrix, StableClaimMatrixSummary, StableClaimMatrixViolation, StableClaimRow,
    StopAction, STABLE_CLAIM_MATRIX_JSON, STABLE_CLAIM_MATRIX_PATH,
    STABLE_CLAIM_MATRIX_RECORD_KIND, STABLE_CLAIM_MATRIX_SCHEMA_VERSION,
};

pub use stable_qualification_matrix::{
    current_stable_qualification_matrix, BoundaryFamily,
    DowngradeReason as QualificationDowngradeReason, DowngradeRule as QualificationDowngradeRule,
    MixedVersionPosture, MixedVersionSection, OrderRecord, OutOfWindowPosture,
    PromotionDecisionRecord as QualificationPromotionDecisionRecord, QualificationAction,
    QualificationExportProjection, QualificationExportRow, QualificationRow, QualificationRowScope,
    SkewWindow, StableQualificationMatrix, StableQualificationMatrixSummary,
    StableQualificationMatrixViolation, UnsupportedStateBehavior, STABLE_QUALIFICATION_MATRIX_JSON,
    STABLE_QUALIFICATION_MATRIX_PATH, STABLE_QUALIFICATION_MATRIX_RECORD_KIND,
    STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
};

pub use support_class_ledger::{
    current_support_class_ledger, ArchetypeCertification, CertificationStatus, CertifiedArchetype,
    CertifiedCutline, DowngradeAction, DowngradeReason as LedgerDowngradeReason, DowngradeRule,
    EvidencePathClass, LedgerOwnerSignoff, LedgerState, LedgerWaiver, PublicationDecision,
    PublicationDecisionRecord as SupportPublicationDecisionRecord, SupportClass, SupportClassEntry,
    SupportClassExportProjection, SupportClassExportRow, SupportClassLedger,
    SupportClassLedgerSummary, SupportClassLedgerViolation, SupportEvidence,
    SUPPORT_CLASS_LEDGER_JSON, SUPPORT_CLASS_LEDGER_PATH, SUPPORT_CLASS_LEDGER_RECORD_KIND,
    SUPPORT_CLASS_LEDGER_SCHEMA_VERSION,
};
