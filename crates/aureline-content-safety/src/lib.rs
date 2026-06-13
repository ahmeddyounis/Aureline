//! Shared suspicious-content detector and representation-labeled transfer
//! records for safe preview.
//!
//! This crate provides one shared detector contract that every preview-capable
//! surface can map to: editor content, save-review, docs/help pages, install
//! review, support export, and so on. It does not invent a parallel evidence
//! vocabulary — record kinds and field names follow the boundary schemas in
//! `/schemas/security/trust_class.schema.json` and
//! `/schemas/security/text_representation_policy.schema.json` so a single
//! detector outcome can be consumed by any later preview surface without
//! re-parsing the raw content independently.
//!
//! Scope:
//!
//! - Detect bidi control codepoints, invisible/zero-width formatting, and
//!   mixed-script confusable identifiers in plain UTF-8 text.
//! - Build [`SuspiciousContentCaseRecord`] and
//!   [`SurfaceTrustResolutionRecord`] payloads that match the boundary
//!   schemas verbatim.
//! - Build representation-labeled [`RepresentationTransferRecord`] payloads
//!   so copy/export affordances can be surfaced as raw vs escaped instead of
//!   a generic "Copy" or "Export".
//! - Provide an `escape_for_safe_inspection` helper that turns suspicious
//!   codepoints into stable `\u{XXXX}` escapes so an escaped-copy path is
//!   reachable wherever the detector finds something.
//!
//! Out of scope (deferred to later lanes):
//!
//! - Whole-script confusable scoring across full Unicode tables.
//! - Sandboxing / iframe / process boundaries for `IsolatedRemoteActive`.
//! - Final UI chrome for warning badges and trust-class badges.
//! - Raw-vs-rendered divergence detection on rendered HTML/Markdown trees;
//!   only token-level findings on plain text are produced here.

#![doc(html_root_url = "https://docs.rs/aureline-content-safety/0.0.0")]

pub mod content_integrity;
pub mod detector;
pub mod freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix;
pub mod m5_suspicious_text_detector_parity;
pub mod records;
pub mod representation_copy_export;
pub mod representation_labels;
pub mod stable_safe_preview_trust;
pub mod suspicious_content;
pub mod suspicious_text;
pub mod transfer;

pub use content_integrity::{
    project_content_integrity_warnings, project_content_integrity_warnings_from_detection,
    warnings_cover_required_surfaces, ContentIntegritySurfaceKind, ContentIntegrityWarningRecord,
    CONTENT_INTEGRITY_REQUIRED_SURFACES, CONTENT_INTEGRITY_WARNING_RECORD_KIND,
    CONTENT_INTEGRITY_WARNING_SCHEMA_VERSION,
};
pub use detector::{
    detect_suspicious_content, escape_for_safe_inspection, has_suspicious_content,
    DetectorOutcomeClass, SuspiciousContentDetection, SuspiciousFinding,
};
pub use freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix::{
    current_stable_m5_content_integrity_matrix_export,
    frozen_stable_m5_content_integrity_matrix_packet, M5ContentIntegrityActiveContentPolicy,
    M5ContentIntegrityArtifactFamily, M5ContentIntegrityConsumerSurface,
    M5ContentIntegrityCopyExportRepresentation, M5ContentIntegrityDisplayMode,
    M5ContentIntegrityDowngradeTrigger, M5ContentIntegrityEvidenceRequirement,
    M5ContentIntegrityMatrixArtifactError, M5ContentIntegrityMatrixConsumerProjection,
    M5ContentIntegrityMatrixFamilyRow, M5ContentIntegrityMatrixPacket,
    M5ContentIntegrityMatrixPacketInput, M5ContentIntegrityMatrixProofFreshness,
    M5ContentIntegrityMatrixTrustReview, M5ContentIntegrityMatrixViolation,
    M5ContentIntegrityQualificationClass, M5ContentIntegrityRawRenderedPosture,
    M5ContentIntegritySafePreviewMode, M5ContentIntegrityTrustClass,
    M5_CONTENT_INTEGRITY_MATRIX_ARTIFACT_REF, M5_CONTENT_INTEGRITY_MATRIX_DOC_REF,
    M5_CONTENT_INTEGRITY_MATRIX_FIXTURE_DIR, M5_CONTENT_INTEGRITY_MATRIX_PACKET_ID,
    M5_CONTENT_INTEGRITY_MATRIX_RECORD_KIND,
    M5_CONTENT_INTEGRITY_MATRIX_REPRESENTATION_EXPORT_CONTRACT_REF,
    M5_CONTENT_INTEGRITY_MATRIX_SAFE_PREVIEW_TRUST_CONTRACT_REF,
    M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_REF, M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_VERSION,
    M5_CONTENT_INTEGRITY_MATRIX_SUMMARY_REF,
    M5_CONTENT_INTEGRITY_MATRIX_TEXT_REPRESENTATION_CONTRACT_REF,
    M5_CONTENT_INTEGRITY_MATRIX_TRUST_CLASS_CONTRACT_REF,
};
pub use m5_suspicious_text_detector_parity::{
    current_m5_suspicious_text_parity_export, frozen_m5_suspicious_text_parity_packet,
    project_m5_suspicious_text_parity, M5SuspiciousTextDisplayMode,
    M5SuspiciousTextParityExportError, M5SuspiciousTextParityPacket, M5SuspiciousTextParitySeed,
    M5SuspiciousTextParityViolation, M5SuspiciousTextSupportAdminExport, M5SuspiciousTextSurface,
    M5SuspiciousTextSurfaceProjection, M5SuspiciousTextThreatClass, M5SuspiciousTextThreatCue,
    M5SuspiciousTextThreatCueSummary, M5SuspiciousTextThreatSeverity,
    M5SuspiciousTextTransferChoice, M5SuspiciousTextWarning,
    M5_SUSPICIOUS_TEXT_PARITY_ARTIFACT_REF, M5_SUSPICIOUS_TEXT_PARITY_DOC_REF,
    M5_SUSPICIOUS_TEXT_PARITY_FIXTURE_DIR, M5_SUSPICIOUS_TEXT_PARITY_PACKET_ID,
    M5_SUSPICIOUS_TEXT_PARITY_RECORD_KIND, M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_REF,
    M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_VERSION, M5_SUSPICIOUS_TEXT_SUPPORT_ADMIN_RECORD_KIND,
};
pub use records::{
    LabelExamples, SurfaceFamily, SurfaceTrustResolutionRecord, SuspiciousContentCaseRecord,
    SuspiciousContentClass, SuspiciousContentFindingRecord, TrustClass,
};
pub use representation_copy_export::{
    CopyExportActionKind, CopyExportLabelClass, CopyPayloadMode, InteractionPolicyContext,
    InteractionRepresentationClass, InteractionSafetyCopyExportRecord, ProtectedCopySurfaceKind,
    RepresentationCopyExportActionInput, RepresentationCopyExportActionProjection,
    RepresentationCopyExportAlphaPacket, RepresentationCopyExportCase,
    RepresentationCopyExportSurfaceInput, RepresentationCopyExportSurfaceProjection,
    RepresentationCopyExportValidationReport, RepresentationCopyExportViolation,
    INTERACTION_SAFETY_COPY_EXPORT_RECORD_KIND, INTERACTION_SAFETY_SCHEMA_VERSION,
    PROTECTED_COPY_EXPORT_SURFACES, REPRESENTATION_COPY_EXPORT_ALPHA_PACKET_RECORD_KIND,
    REPRESENTATION_COPY_EXPORT_ALPHA_SCHEMA_VERSION,
    REPRESENTATION_COPY_EXPORT_VALIDATION_REPORT_RECORD_KIND,
};
pub use representation_labels::{
    LabeledRepresentationClass, RepresentationActionKind, RepresentationActionPosture,
    RepresentationExportRecord, RepresentationLabelsActionInput,
    RepresentationLabelsActionProjection, RepresentationLabelsBetaCase,
    RepresentationLabelsBetaGateStatus, RepresentationLabelsBetaPacket,
    RepresentationLabelsBetaValidationReport, RepresentationLabelsBetaViolation,
    RepresentationLabelsSurfaceInput, RepresentationLabelsSurfaceRow, RepresentationOmissionReason,
    RepresentationOriginClass, RepresentationScopeClass, RepresentationSurfaceKind,
    RepresentationTransformKind, RiskyContentClass, REPRESENTATION_EXPORT_RECORD_KIND,
    REPRESENTATION_EXPORT_SCHEMA_REF, REPRESENTATION_LABELS_BETA_DOC_REF,
    REPRESENTATION_LABELS_BETA_FIXTURE_DIR, REPRESENTATION_LABELS_BETA_PACKET_RECORD_KIND,
    REPRESENTATION_LABELS_BETA_SCHEMA_VERSION,
    REPRESENTATION_LABELS_BETA_VALIDATION_REPORT_RECORD_KIND,
    REPRESENTATION_LABELS_REQUIRED_ACTIONS, REPRESENTATION_LABELS_REQUIRED_CONTENT_CLASSES,
    REPRESENTATION_LABELS_REQUIRED_REPRESENTATIONS, REPRESENTATION_LABELS_REQUIRED_SURFACES,
};
pub use stable_safe_preview_trust::{
    stable_safe_preview_trust_packet, OriginBoundaryClass, SafePreviewAllowedBehavior,
    SafePreviewConsumerSurface, SafePreviewDowngradeState, SafePreviewDowngradeTrigger,
    SafePreviewSurfaceMatrixRow, SafePreviewTransferCase, SafePreviewTransferCaseKind,
    SafePreviewTrustClassContract, StableSafePreviewTrustGateStatus, StableSafePreviewTrustPacket,
    StableSafePreviewTrustValidationReport, StableSafePreviewTrustViolation, SurfaceQualification,
    TrustEvidenceCarrier, VisibleTrustCue, REQUIRED_STABLE_CONSUMER_SURFACES,
    REQUIRED_TRANSFER_CASE_KINDS, REQUIRED_TRUST_EVIDENCE_CARRIERS,
    STABLE_SAFE_PREVIEW_SHARED_CONTRACT_REF, STABLE_SAFE_PREVIEW_TRUST_DOC_REF,
    STABLE_SAFE_PREVIEW_TRUST_FIXTURE_DIR, STABLE_SAFE_PREVIEW_TRUST_PACKET_RECORD_KIND,
    STABLE_SAFE_PREVIEW_TRUST_SCHEMA_REF, STABLE_SAFE_PREVIEW_TRUST_SCHEMA_VERSION,
    STABLE_SAFE_PREVIEW_TRUST_VALIDATION_REPORT_RECORD_KIND,
};
pub use suspicious_content::{
    current_content_integrity_beta_packet, ContentIntegrityBetaArtifactError,
    ContentIntegrityBetaCase, ContentIntegrityBetaGateStatus, ContentIntegrityBetaPacket,
    ContentIntegrityBetaSurfaceInput, ContentIntegrityBetaSurfaceKind,
    ContentIntegrityBetaSurfaceRow, ContentIntegrityBetaValidationReport,
    ContentIntegrityBetaViolation, ContentIntegrityOperatorTruthControls,
    ContentIntegrityRepresentationChoice, ContentIntegrityRepresentationChoiceInput,
    ContentIntegrityRepresentationClass, ContentIntegrityTransferKind,
    CONTENT_INTEGRITY_BETA_DOC_REF, CONTENT_INTEGRITY_BETA_FIXTURE_DIR,
    CONTENT_INTEGRITY_BETA_PACKET_RECORD_KIND, CONTENT_INTEGRITY_BETA_PACKET_REF,
    CONTENT_INTEGRITY_BETA_REQUIRED_SURFACES, CONTENT_INTEGRITY_BETA_SCHEMA_VERSION,
    CONTENT_INTEGRITY_BETA_VALIDATION_REPORT_RECORD_KIND,
    CONTENT_INTEGRITY_REQUIRED_REPRESENTATIONS,
};
pub use suspicious_text::{
    project_suspicious_text_core_surfaces, SuspiciousTextAnchor, SuspiciousTextLocationKind,
    SuspiciousTextProjectionSeed, SuspiciousTextSafeExport, SuspiciousTextSurfaceKind,
    SuspiciousTextSurfacePacket, SuspiciousTextSurfaceProjection, SuspiciousTextTransferChoice,
    SuspiciousTextWarning, SuspiciousTextWarningAction, CORE_SOURCE_SURFACES,
    SUSPICIOUS_TEXT_SURFACE_PACKET_SCHEMA_VERSION,
};
pub use transfer::{
    BodyPosture, RepresentationActionId, RepresentationClass, RepresentationTransferRecord,
};
