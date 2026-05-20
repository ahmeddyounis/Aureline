//! Docs-node identity and citation evidence primitives.
//!
//! This crate owns the bounded alpha records that let docs/help rows,
//! graph explainers, onboarding packs, support exports, and AI evidence
//! packets preserve one citation vocabulary. The records carry stable ids,
//! pack revisions, locale/fallback state, freshness, locality, exact-anchor
//! availability, and inference/confidence labels. They intentionally do not
//! carry raw document bodies, raw source files, raw URLs, or prompt text.

#![doc(html_root_url = "https://docs.rs/aureline-docs/0.0.0")]

pub mod citations;
pub mod evidence_model;
pub mod index;
pub mod locale_overlay;
pub mod maintenance;
pub mod pack;

pub use citations::{
    CitationAnchorAlpha, CitationAnchorAlphaInput, CitationAnchorAvailability,
    CitationConfidenceClass, CitationDrawerEvidenceView, CitationDrawerEvidenceViewInput,
    CitationDrawerRow, CitationEvidenceExport, CitationEvidenceExportInput,
    CitationInferenceMarker, CitationLocalityClass, CitationSourceClass, CitationTruthViolation,
    DocsFreshnessClass, DocsNodeIdentity, DocsNodeIdentityInput, DocsNodeKind, DocsScopeClass,
    HelpPackItemEvidence, LocaleOverlayState, SourcePrecedenceClass, VersionMatchState,
    CITATION_ANCHOR_ALPHA_RECORD_KIND, CITATION_DRAWER_ALPHA_RECORD_KIND,
    CITATION_EVIDENCE_EXPORT_ALPHA_RECORD_KIND, DOCS_CITATION_ALPHA_SCHEMA_VERSION,
    DOCS_NODE_ALPHA_RECORD_KIND,
};
pub use evidence_model::{
    DocsDerivedClaimKind, DocsDerivedExplanation, DocsDerivedExplanationClaim,
    DocsDerivedExplanationInput, DocsDerivedExplanationKind, DocsEvidenceModelViolation,
    DocsExampleValidationClass, DocsExternalOpenFallback, DocsExternalOpenState,
    DocsKnowledgeObjectKind, DocsKnowledgeSourceStrip, DocsKnowledgeSurfaceEvidencePacket,
    DocsKnowledgeSurfaceEvidencePacketInput, DocsKnowledgeSurfaceKind,
    DocsKnowledgeSurfaceProjection, DocsKnowledgeSurfaceProjectionInput, DocsMirrorOfflinePosture,
    DocsNodeProvenance, DocsNodeProvenanceInput, DocsTruthDowngrade, DocsTruthLabelClass,
    DOCS_DERIVED_EXPLANATION_RECORD_KIND, DOCS_KNOWLEDGE_SURFACE_EVIDENCE_PACKET_RECORD_KIND,
    DOCS_KNOWLEDGE_SURFACE_PROJECTION_RECORD_KIND, DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION,
    DOCS_NODE_PROVENANCE_RECORD_KIND,
};
pub use index::{
    DocsSearchIndex, DocsSearchIndexEntry, DocsSearchQueryResult,
    DOCS_SEARCH_INDEX_ENTRY_RECORD_KIND, DOCS_SEARCH_INDEX_RECORD_KIND,
    DOCS_SEARCH_INDEX_SCHEMA_VERSION, DOCS_SEARCH_QUERY_RESULT_RECORD_KIND,
    DOCS_SEARCH_RESULT_KIND_TOKEN,
};
pub use locale_overlay::{
    seeded_translated_pack_locale_overlay_contract,
    seeded_translated_pack_locale_overlay_support_export,
    seeded_translated_pack_locale_overlay_surface_projection,
    validate_seeded_translated_pack_locale_overlay, LocaleOverlayBadgeClass,
    LocaleOverlayContract, LocaleOverlayCoverage, LocaleOverlayCoverageState,
    LocaleOverlayFinding, LocaleOverlayMirrorOfflinePosture, LocaleOverlayPackKind,
    LocaleOverlayRecord, LocaleOverlaySkewState, LocaleOverlaySourceLanguageAction,
    LocaleOverlaySupportExport, LocaleOverlaySupportExportPolicy, LocaleOverlaySupportRow,
    LocaleOverlaySurfaceProjection, LocaleOverlaySurfaceRow, LOCALE_OVERLAY_CONTRACT_RECORD_KIND,
    LOCALE_OVERLAY_FIXTURE_REF, LOCALE_OVERLAY_RECORD_KIND, LOCALE_OVERLAY_SCHEMA_REF,
    LOCALE_OVERLAY_SCHEMA_VERSION, LOCALE_OVERLAY_SUPPORT_EXPORT_FIXTURE_REF,
    LOCALE_OVERLAY_SUPPORT_EXPORT_RECORD_KIND, LOCALE_OVERLAY_SURFACE_FIXTURE_REF,
    LOCALE_OVERLAY_SURFACE_PROJECTION_RECORD_KIND, OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL,
    TRANSLATED_PACK_LOCALE_OVERLAY_CONTRACT_ID, TRANSLATED_PACK_LOCALE_OVERLAY_VERSION_REF,
};
pub use maintenance::{
    seeded_docs_preview_and_maintenance_contract,
    seeded_docs_preview_and_maintenance_review_packet,
    seeded_docs_preview_and_maintenance_surface_projection,
    validate_seeded_docs_preview_and_maintenance, DocsArtifactKind, DocsAudienceScope,
    DocsExampleFindingRow, DocsExampleValidationMode, DocsFindingClass, DocsFindingDetectionState,
    DocsFindingSuppression, DocsFindingSuppressionState, DocsHandoffBanner, DocsMaintenanceAction,
    DocsMaintenanceContract,
    DocsMaintenanceCoverage, DocsMaintenanceFinding, DocsMaintenanceReviewPacket, DocsMaintenanceRow,
    DocsMaintenanceSurfaceProjection, DocsPreviewHeader, DocsPreviewMode,
    DocsPreviewSanitizationState, DocsPublishBoundaryState, DocsPublishScope,
    DocsSourceVersionBadge, DocsSuggestionApplyPosture, DocsSuggestionCard, DocsSuggestionTrigger,
    DOCS_EXAMPLE_FINDING_ROW_RECORD_KIND, DOCS_MAINTENANCE_CONTRACT_RECORD_KIND,
    DOCS_MAINTENANCE_REVIEW_PACKET_RECORD_KIND, DOCS_MAINTENANCE_ROW_RECORD_KIND,
    DOCS_MAINTENANCE_ROW_SCHEMA_REF, DOCS_MAINTENANCE_SCHEMA_VERSION,
    DOCS_MAINTENANCE_SURFACE_PROJECTION_RECORD_KIND, DOCS_PREVIEW_AND_MAINTENANCE_CONTRACT_ID,
    DOCS_PREVIEW_AND_MAINTENANCE_VERSION_REF, DOCS_PREVIEW_HEADER_RECORD_KIND,
    DOCS_SUGGESTION_CARD_RECORD_KIND, DOCS_SUGGESTION_CARD_SCHEMA_REF,
};
pub use pack::{
    DocsPack, DocsPackLoadError, DocsPackNode, DocsPackSourceTruth, DOCS_PACK_ALPHA_RECORD_KIND,
    DOCS_PACK_ALPHA_SCHEMA_VERSION,
};
