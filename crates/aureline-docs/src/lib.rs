//! Docs-node identity and citation evidence primitives.
//!
//! This crate owns the bounded alpha records that let docs/help rows,
//! graph explainers, onboarding packs, support exports, and AI evidence
//! packets preserve one citation vocabulary. The records carry stable ids,
//! pack revisions, locale/fallback state, freshness, locality, exact-anchor
//! availability, and inference/confidence labels. They intentionally do not
//! carry raw document bodies, raw source files, raw URLs, or prompt text.

#![doc(html_root_url = "https://docs.rs/aureline-docs/0.0.0")]

pub mod add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa;
pub mod add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels;
pub mod citations;
pub mod docs_browser_truth_packet;
pub mod docs_maintenance_and_stale_example_governance;
pub mod docs_pack_truth_packet;
pub mod evidence_model;
pub mod freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract;
pub mod implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export;
pub mod implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings;
pub mod implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu;
pub mod index;
pub mod locale_overlay;
pub mod maintenance;
pub mod pack;
pub mod semantic_recall_boundary_truth_packet;
pub mod ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links;
pub mod ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling;
pub mod ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history;
pub mod stable_docs_contract;

pub use add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa::{
    current_stable_light_remote_edit_export,
    packet_to_input as light_remote_edit_packet_to_input,
    seeded_stable_light_remote_edit_input, ApplyPosture, AuthorityGrant, AuthorityScope,
    BaseStateKind, CapturedVsLive as LightRemoteEditCapturedVsLive, EditConfidence, EditFreshness,
    EditIntent, EditIntentKind, EditLocality, EditSourceClass, EditTrustClass, EditVersionMatch,
    LightRemoteEditArtifactError, LightRemoteEditChipSet, LightRemoteEditConsumerProjection,
    LightRemoteEditConsumerSurface, LightRemoteEditDegradation, LightRemoteEditDegradationClass,
    LightRemoteEditExport, LightRemoteEditExportRow, LightRemoteEditExportScope,
    LightRemoteEditFindingKind, LightRemoteEditFindingSeverity, LightRemoteEditPromotionState,
    LightRemoteEditScope, LightRemoteEditSupportExport, LightRemoteEditSurface,
    LightRemoteEditSurfacesPacket, LightRemoteEditSurfacesPacketInput,
    LightRemoteEditValidationFinding, ReturnPath as LightRemoteEditReturnPath,
    ReturnPathKind as LightRemoteEditReturnPathKind, StaleStateDisclosure,
    LIGHT_REMOTE_EDIT_ARTIFACT_REF, LIGHT_REMOTE_EDIT_DOC_REF, LIGHT_REMOTE_EDIT_FIXTURE_DIR,
    LIGHT_REMOTE_EDIT_RECORD_KIND, LIGHT_REMOTE_EDIT_SCHEMA_REF, LIGHT_REMOTE_EDIT_SCHEMA_VERSION,
    LIGHT_REMOTE_EDIT_SUMMARY_REF, LIGHT_REMOTE_EDIT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels::{
    current_stable_codebase_understanding_cards_export,
    packet_to_input as codebase_understanding_cards_packet_to_input,
    seeded_stable_codebase_understanding_cards_input, CardEvidence, CardProvenance,
    CodebaseUnderstandingCardsArtifactError, CodebaseUnderstandingCardsPacket,
    CodebaseUnderstandingCardsPacketInput, CodebaseUnderstandingCardsSupportExport,
    EvidenceDerivation, EvidenceExportRow, EvidenceExportScope, EvidenceSubjectKind, OwnerRef,
    OwnershipBasis, TopologyEdgeKind, TopologyEdgeRef, UnderstandingCard, UnderstandingCardKind,
    UnderstandingChipSet, UnderstandingConfidence, UnderstandingConsumerProjection,
    UnderstandingConsumerSurface, UnderstandingDegradation, UnderstandingDegradationClass,
    UnderstandingEvidenceExport, UnderstandingFindingKind, UnderstandingFindingSeverity,
    UnderstandingFreshness, UnderstandingLocality, UnderstandingPromotionState,
    UnderstandingSourceClass, UnderstandingValidationFinding, UnderstandingVersionMatch,
    UNDERSTANDING_CARDS_ARTIFACT_REF, UNDERSTANDING_CARDS_DOC_REF, UNDERSTANDING_CARDS_FIXTURE_DIR,
    UNDERSTANDING_CARDS_RECORD_KIND, UNDERSTANDING_CARDS_SCHEMA_REF,
    UNDERSTANDING_CARDS_SCHEMA_VERSION, UNDERSTANDING_CARDS_SUMMARY_REF,
    UNDERSTANDING_CARDS_SUPPORT_EXPORT_RECORD_KIND,
};
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
pub use docs_browser_truth_packet::{
    current_stable_docs_browser_truth_packet, seeded_stable_docs_browser_truth_packet_input,
    DocsBrowserCapturedVsLive, DocsBrowserCitationAnchor, DocsBrowserConsumerProjection,
    DocsBrowserConsumerSurface, DocsBrowserFindingKind, DocsBrowserFindingSeverity,
    DocsBrowserFreshnessState, DocsBrowserHandoffCapability, DocsBrowserPromotionState,
    DocsBrowserResultObject, DocsBrowserSourceClass, DocsBrowserSourceDescriptor,
    DocsBrowserSymbolFlow, DocsBrowserSymbolFlowStep, DocsBrowserSymbolLinkClass,
    DocsBrowserSymbolRef, DocsBrowserTrustClass, DocsBrowserTruthArtifactError,
    DocsBrowserTruthPacket, DocsBrowserTruthPacketInput, DocsBrowserTruthSupportExport,
    DocsBrowserValidationFinding, DocsBrowserVersionMatchState,
    DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_DOC_REF, DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_REF,
    DOCS_BROWSER_TRUTH_PACKET_DOC_REF, DOCS_BROWSER_TRUTH_PACKET_FIXTURE_DIR,
    DOCS_BROWSER_TRUTH_PACKET_MILESTONE_DOC_REF, DOCS_BROWSER_TRUTH_PACKET_RECORD_KIND,
    DOCS_BROWSER_TRUTH_PACKET_SCHEMA_REF, DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
    DOCS_BROWSER_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND,
};
pub use docs_maintenance_and_stale_example_governance::{
    current_docs_maintenance_and_stale_example_governance_packet,
    seeded_docs_maintenance_and_stale_example_governance_input, DocsActiveContentState,
    DocsMaintenanceArtifactClass, DocsMaintenanceGovernanceArtifactError,
    DocsMaintenanceGovernanceFinding, DocsMaintenanceGovernanceFindingKind,
    DocsMaintenanceGovernancePacket, DocsMaintenanceGovernancePacketInput,
    DocsMaintenanceGovernanceProjection, DocsMaintenanceGovernancePromotionState,
    DocsMaintenanceGovernanceSupportExport, DocsMaintenanceGovernanceSurface,
    DocsMaintenancePacket, DocsMirrorBrowserHandoffPosture, DocsRenderConfig,
    DocsRenderSecurityProfile, DocsShareExportPosture, DocsSuggestionObject, DocsValidationOutcome,
    DocsValidationResult, StaleExampleGovernanceFinding,
    DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_DOC_REF, DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF,
    DOCS_MAINTENANCE_GOVERNANCE_DOC_REF, DOCS_MAINTENANCE_GOVERNANCE_FIXTURE_DIR,
    DOCS_MAINTENANCE_GOVERNANCE_RECORD_KIND, DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_REF,
    DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
    DOCS_MAINTENANCE_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use docs_pack_truth_packet::{
    current_stable_docs_pack_truth_packet, seeded_stable_docs_pack_truth_packet_input,
    CitationSetExport, DocsPackChannel, DocsPackConsumerProjection, DocsPackConsumerSurface,
    DocsPackFindingKind, DocsPackFindingSeverity, DocsPackLocalAvailability, DocsPackManifest,
    DocsPackMirrorLineage, DocsPackMirrorState, DocsPackPinState, DocsPackPromotionState,
    DocsPackPublishableState, DocsPackRefreshState, DocsPackSignatureStatus, DocsPackSignerClass,
    DocsPackSigningBlock, DocsPackSourceClass, DocsPackTruthArtifactError, DocsPackTruthPacket,
    DocsPackTruthPacketInput, DocsPackTruthSupportExport, DocsPackValidationFinding,
    DocsPackVersionRange, DocsRenderMode, DocsValidationResultClass, StaleExampleFinding,
    StaleExampleFindingClass, StaleExampleSuppression, DOCS_PACK_TRUTH_PACKET_ARTIFACT_DOC_REF,
    DOCS_PACK_TRUTH_PACKET_ARTIFACT_REF, DOCS_PACK_TRUTH_PACKET_DOC_REF,
    DOCS_PACK_TRUTH_PACKET_FIXTURE_DIR, DOCS_PACK_TRUTH_PACKET_MILESTONE_DOC_REF,
    DOCS_PACK_TRUTH_PACKET_RECORD_KIND, DOCS_PACK_TRUTH_PACKET_SCHEMA_REF,
    DOCS_PACK_TRUTH_PACKET_SCHEMA_VERSION, DOCS_PACK_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND,
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
pub use freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract::{
    current_stable_m5_docs_and_code_recall_matrix_export, M5DocsRecallConsumerSurface,
    M5DocsRecallDowngradeTrigger, M5DocsRecallEvidenceRequirement, M5DocsRecallLane,
    M5DocsRecallMatrixArtifactError, M5DocsRecallMatrixConsumerProjection, M5DocsRecallMatrixLaneRow,
    M5DocsRecallMatrixPacket, M5DocsRecallMatrixPacketInput, M5DocsRecallMatrixProofFreshness,
    M5DocsRecallMatrixTrustReview, M5DocsRecallMatrixViolation, M5DocsRecallQualificationClass,
    M5DocsRecallRollbackPosture, M5_DOCS_RECALL_MATRIX_ARTIFACT_REF,
    M5_DOCS_RECALL_MATRIX_BROWSER_SURFACE_CONTRACT_REF,
    M5_DOCS_RECALL_MATRIX_CODE_EXPLAINER_CONTRACT_REF, M5_DOCS_RECALL_MATRIX_DOCS_RECALL_CONTRACT_REF,
    M5_DOCS_RECALL_MATRIX_DOC_REF, M5_DOCS_RECALL_MATRIX_FIXTURE_DIR,
    M5_DOCS_RECALL_MATRIX_RECORD_KIND, M5_DOCS_RECALL_MATRIX_RETRIEVAL_DEBUG_CONTRACT_REF,
    M5_DOCS_RECALL_MATRIX_SCHEMA_REF, M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION,
    M5_DOCS_RECALL_MATRIX_SUMMARY_REF,
};
pub use implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export::{
    current_stable_semantic_recall_ledger_export,
    packet_to_input as semantic_recall_ledger_packet_to_input,
    seeded_stable_semantic_recall_ledger_input, DerivationClass, ProvenanceExportScope,
    QueryRefinementRelation, RankingSignal, RankingSignalKind, RecallDegradation,
    RecallDegradationClass, ResultProvenance, SemanticRecallChipSet, SemanticRecallConfidence,
    SemanticRecallConsumerProjection, SemanticRecallConsumerSurface, SemanticRecallFindingKind,
    SemanticRecallFindingSeverity, SemanticRecallFreshness, SemanticRecallLedgerArtifactError,
    SemanticRecallLedgerEntry, SemanticRecallLedgerPacket, SemanticRecallLedgerPacketInput,
    SemanticRecallLedgerSupportExport, SemanticRecallLocality, SemanticRecallPromotionState,
    SemanticRecallProvenanceExport, SemanticRecallProvenanceRow, SemanticRecallQuerySessionLedger,
    SemanticRecallResultRow, SemanticRecallSourceClass, SemanticRecallSubjectKind,
    SemanticRecallSubjectScope, SemanticRecallValidationFinding, SemanticRecallVersionMatch,
    SignalContributionClass, SEMANTIC_RECALL_LEDGER_ARTIFACT_REF, SEMANTIC_RECALL_LEDGER_DOC_REF,
    SEMANTIC_RECALL_LEDGER_FIXTURE_DIR, SEMANTIC_RECALL_LEDGER_RECORD_KIND,
    SEMANTIC_RECALL_LEDGER_SCHEMA_REF, SEMANTIC_RECALL_LEDGER_SCHEMA_VERSION,
    SEMANTIC_RECALL_LEDGER_SUMMARY_REF, SEMANTIC_RECALL_LEDGER_SUPPORT_EXPORT_RECORD_KIND,
};
pub use implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings::{
    current_stable_docs_pack_recall_export, packet_to_input,
    seeded_stable_docs_pack_recall_input, DocsPackRecallArtifactError, DocsPackRecallChipSet,
    DocsPackRecallConfidence, DocsPackRecallConsumerProjection, DocsPackRecallConsumerSurface,
    DocsPackRecallFindingKind, DocsPackRecallFindingSeverity, DocsPackRecallFreshness,
    DocsPackRecallLocality, DocsPackRecallMirrorAwareness, DocsPackRecallPacket,
    DocsPackRecallPacketInput, DocsPackRecallPromotionState, DocsPackRecallResultRow,
    DocsPackRecallSourceClass, DocsPackRecallStaleFinding, DocsPackRecallStaleFindingClass,
    DocsPackRecallSupportExport, DocsPackRecallValidationFinding, DocsPackRecallVersionMatch,
    DOCS_PACK_RECALL_ARTIFACT_REF, DOCS_PACK_RECALL_DOC_REF, DOCS_PACK_RECALL_FIXTURE_DIR,
    DOCS_PACK_RECALL_RECORD_KIND, DOCS_PACK_RECALL_SCHEMA_REF, DOCS_PACK_RECALL_SCHEMA_VERSION,
    DOCS_PACK_RECALL_SUMMARY_REF, DOCS_PACK_RECALL_SUPPORT_EXPORT_RECORD_KIND,
};
pub use implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu::{
    current_stable_scoped_browser_export,
    packet_to_input as scoped_browser_packet_to_input, seeded_stable_scoped_browser_input,
    CapturedVsLive, HandoffCapability, HandoffReason, HandoffReasonKind, ReturnPath, ReturnPathKind,
    ScopedBrowserArtifactError, ScopedBrowserChipSet, ScopedBrowserConfidence,
    ScopedBrowserConsumerProjection, ScopedBrowserConsumerSurface, ScopedBrowserDegradation,
    ScopedBrowserDegradationClass, ScopedBrowserExport, ScopedBrowserExportRow,
    ScopedBrowserExportScope, ScopedBrowserFindingKind, ScopedBrowserFindingSeverity,
    ScopedBrowserFreshness, ScopedBrowserLocality, ScopedBrowserPromotionState, ScopedBrowserScope,
    ScopedBrowserSourceClass, ScopedBrowserSupportExport, ScopedBrowserSurface,
    ScopedBrowserSurfacesPacket, ScopedBrowserSurfacesPacketInput, ScopedBrowserTrustClass,
    ScopedBrowserValidationFinding, ScopedBrowserVersionMatch, SCOPED_BROWSER_ARTIFACT_REF,
    SCOPED_BROWSER_DOC_REF, SCOPED_BROWSER_FIXTURE_DIR, SCOPED_BROWSER_RECORD_KIND,
    SCOPED_BROWSER_SCHEMA_REF, SCOPED_BROWSER_SCHEMA_VERSION, SCOPED_BROWSER_SUMMARY_REF,
    SCOPED_BROWSER_SUPPORT_EXPORT_RECORD_KIND,
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
    validate_seeded_translated_pack_locale_overlay, LocaleOverlayBadgeClass, LocaleOverlayContract,
    LocaleOverlayCoverage, LocaleOverlayCoverageState, LocaleOverlayFinding,
    LocaleOverlayMirrorOfflinePosture, LocaleOverlayPackKind, LocaleOverlayRecord,
    LocaleOverlaySkewState, LocaleOverlaySourceLanguageAction, LocaleOverlaySupportExport,
    LocaleOverlaySupportExportPolicy, LocaleOverlaySupportRow, LocaleOverlaySurfaceProjection,
    LocaleOverlaySurfaceRow, LOCALE_OVERLAY_CONTRACT_RECORD_KIND, LOCALE_OVERLAY_FIXTURE_REF,
    LOCALE_OVERLAY_RECORD_KIND, LOCALE_OVERLAY_SCHEMA_REF, LOCALE_OVERLAY_SCHEMA_VERSION,
    LOCALE_OVERLAY_SUPPORT_EXPORT_FIXTURE_REF, LOCALE_OVERLAY_SUPPORT_EXPORT_RECORD_KIND,
    LOCALE_OVERLAY_SURFACE_FIXTURE_REF, LOCALE_OVERLAY_SURFACE_PROJECTION_RECORD_KIND,
    OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL, TRANSLATED_PACK_LOCALE_OVERLAY_CONTRACT_ID,
    TRANSLATED_PACK_LOCALE_OVERLAY_VERSION_REF,
};
pub use maintenance::{
    seeded_docs_preview_and_maintenance_contract,
    seeded_docs_preview_and_maintenance_review_packet,
    seeded_docs_preview_and_maintenance_surface_projection,
    validate_seeded_docs_preview_and_maintenance, DocsArtifactKind, DocsAudienceScope,
    DocsExampleFindingRow, DocsExampleValidationMode, DocsFindingClass, DocsFindingDetectionState,
    DocsFindingSuppression, DocsFindingSuppressionState, DocsHandoffBanner, DocsMaintenanceAction,
    DocsMaintenanceContract, DocsMaintenanceCoverage, DocsMaintenanceFinding,
    DocsMaintenanceReviewPacket, DocsMaintenanceRow, DocsMaintenanceSurfaceProjection,
    DocsPreviewHeader, DocsPreviewMode, DocsPreviewSanitizationState, DocsPublishBoundaryState,
    DocsPublishScope, DocsSourceVersionBadge, DocsSuggestionApplyPosture, DocsSuggestionCard,
    DocsSuggestionTrigger, DOCS_EXAMPLE_FINDING_ROW_RECORD_KIND,
    DOCS_MAINTENANCE_CONTRACT_RECORD_KIND, DOCS_MAINTENANCE_REVIEW_PACKET_RECORD_KIND,
    DOCS_MAINTENANCE_ROW_RECORD_KIND, DOCS_MAINTENANCE_ROW_SCHEMA_REF,
    DOCS_MAINTENANCE_SCHEMA_VERSION, DOCS_MAINTENANCE_SURFACE_PROJECTION_RECORD_KIND,
    DOCS_PREVIEW_AND_MAINTENANCE_CONTRACT_ID, DOCS_PREVIEW_AND_MAINTENANCE_VERSION_REF,
    DOCS_PREVIEW_HEADER_RECORD_KIND, DOCS_SUGGESTION_CARD_RECORD_KIND,
    DOCS_SUGGESTION_CARD_SCHEMA_REF,
};
pub use pack::{
    DocsPack, DocsPackLoadError, DocsPackNode, DocsPackSourceTruth, DOCS_PACK_ALPHA_RECORD_KIND,
    DOCS_PACK_ALPHA_SCHEMA_VERSION,
};
pub use semantic_recall_boundary_truth_packet::{
    current_stable_semantic_recall_boundary_truth_packet,
    ConfidenceClass as SemanticRecallBoundaryConfidenceClass,
    ConsumerSurface as SemanticRecallBoundaryConsumerSurface,
    DowngradeState as SemanticRecallBoundaryDowngradeState, EmbedderIdentity,
    FindingKind as SemanticRecallBoundaryFindingKind,
    FindingSeverity as SemanticRecallBoundaryFindingSeverity, LaneParticipation,
    LocalityClass as SemanticRecallBoundaryLocalityClass, PackSignature, PackSignatureState,
    PromotionState as SemanticRecallBoundaryPromotionState,
    RecallLaneClass as SemanticRecallBoundaryLaneClass,
    RetrievalEpochState as SemanticRecallBoundaryEpochState,
    SemanticRecallBoundaryConsumerProjection, SemanticRecallBoundaryRow,
    SemanticRecallBoundaryTruthArtifactError, SemanticRecallBoundaryTruthPacket,
    SemanticRecallBoundaryTruthPacketInput, SemanticRecallBoundaryTruthSupportExport,
    SurfaceTrack as SemanticRecallBoundarySurfaceTrack,
    ValidationFinding as SemanticRecallBoundaryValidationFinding,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_ARTIFACT_DOC_REF, SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_FIXTURE_DIR, SEMANTIC_RECALL_BOUNDARY_TRUTH_MILESTONE_DOC_REF,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_ARTIFACT_REF,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_RECORD_KIND, SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_REF,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_VERSION,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links::{
    current_stable_docs_search_link_export, packet_to_input as docs_search_link_packet_to_input,
    seeded_stable_docs_search_link_input, DocsSearchLinkAnchorKind,
    DocsSearchLinkArtifactError, DocsSearchLinkBrowserHandoffReason, DocsSearchLinkChipSet,
    DocsSearchLinkCodeAnchor, DocsSearchLinkConsumerProjection, DocsSearchLinkConsumerSurface,
    DocsSearchLinkDeepLink, DocsSearchLinkDisclosure, DocsSearchLinkDisclosureClass,
    DocsSearchLinkFindingKind, DocsSearchLinkFindingSeverity, DocsSearchLinkFreshness,
    DocsSearchLinkPacket, DocsSearchLinkPacketInput, DocsSearchLinkProjectVendorCue,
    DocsSearchLinkPromotionState, DocsSearchLinkRepairHook, DocsSearchLinkRepairHookKind,
    DocsSearchLinkResolutionClass, DocsSearchLinkResultKind, DocsSearchLinkResultRow,
    DocsSearchLinkReuseState, DocsSearchLinkSourceClass, DocsSearchLinkSubjectKind,
    DocsSearchLinkSupportExport, DocsSearchLinkSymbolCard, DocsSearchLinkValidationFinding,
    DocsSearchLinkVersionMatch, DOCS_SEARCH_LINK_ARTIFACT_REF, DOCS_SEARCH_LINK_DOC_REF,
    DOCS_SEARCH_LINK_FIXTURE_DIR, DOCS_SEARCH_LINK_RECORD_KIND, DOCS_SEARCH_LINK_SCHEMA_REF,
    DOCS_SEARCH_LINK_SCHEMA_VERSION, DOCS_SEARCH_LINK_SUMMARY_REF,
    DOCS_SEARCH_LINK_SUPPORT_EXPORT_RECORD_KIND,
    DOCS_SEARCH_LINK_SYMBOL_REFERENCE_CONTRACT_REF, DOCS_SEARCH_LINK_VALIDATION_MANIFEST_REF,
};
pub use ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling::{
    current_stable_retrieval_debug_export,
    packet_to_input as retrieval_debug_packet_to_input, seeded_stable_retrieval_debug_input,
    RankingSignal as RetrievalRankingSignal, RankingSignalKind as RetrievalRankingSignalKind,
    RetrievalChipSet, RetrievalConfidence,
    RetrievalConsumerProjection, RetrievalConsumerSurface, RetrievalDebugArtifactError,
    RetrievalDebugEntry, RetrievalDebugExport, RetrievalDebugExportRow, RetrievalDebugPacket,
    RetrievalDebugPacketInput, RetrievalDebugSupportExport, RetrievalDegradation,
    RetrievalDegradationClass, RetrievalDerivationLabel, RetrievalExportScope, RetrievalFindingKind,
    RetrievalFindingSeverity, RetrievalFreshness, RetrievalLane, RetrievalLocality,
    RetrievalPromotionState, RetrievalSourceClass, RetrievalSubjectKind, RetrievalValidationFinding,
    RetrievalVersionMatch, SignalContribution, RETRIEVAL_DEBUG_ARTIFACT_REF, RETRIEVAL_DEBUG_DOC_REF,
    RETRIEVAL_DEBUG_FIXTURE_DIR, RETRIEVAL_DEBUG_RECORD_KIND, RETRIEVAL_DEBUG_SCHEMA_REF,
    RETRIEVAL_DEBUG_SCHEMA_VERSION, RETRIEVAL_DEBUG_SUMMARY_REF,
    RETRIEVAL_DEBUG_SUPPORT_EXPORT_RECORD_KIND,
};
pub use ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history::{
    current_stable_saved_query_privacy_export,
    packet_to_input as saved_query_privacy_packet_to_input,
    seeded_stable_saved_query_privacy_input, CapturedVsLive as SavedQueryCapturedVsLive,
    QueryChipSet, QueryConfidence, QueryEntryKind, QueryFreshness, QueryLocality, QueryPrivacyClass,
    QueryRedactionClass, QuerySourceClass, QueryTrustClass, QueryVersionMatch, RetentionDisclosure,
    RetentionPosture, SavedQueryConsumerProjection, SavedQueryConsumerSurface, SavedQueryDegradation,
    SavedQueryDegradationClass, SavedQueryEntry, SavedQueryExportRow, SavedQueryExportScope,
    SavedQueryFindingKind, SavedQueryFindingSeverity, SavedQueryHistoryExport,
    SavedQueryPrivacyArtifactError, SavedQueryPrivacyPacket, SavedQueryPrivacyPacketInput,
    SavedQueryPrivacySupportExport, SavedQueryPromotionState, SavedQueryValidationFinding,
    SharePosture, SupportExportSafety, Visibility, VisibilityGrant,
    SAVED_QUERY_PRIVACY_ARTIFACT_REF, SAVED_QUERY_PRIVACY_DOC_REF, SAVED_QUERY_PRIVACY_FIXTURE_DIR,
    SAVED_QUERY_PRIVACY_RECORD_KIND, SAVED_QUERY_PRIVACY_SCHEMA_REF,
    SAVED_QUERY_PRIVACY_SCHEMA_VERSION, SAVED_QUERY_PRIVACY_SUMMARY_REF,
    SAVED_QUERY_PRIVACY_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stable_docs_contract::{
    current_stable_docs_source_result_pack_and_citation_packet,
    seeded_stable_docs_source_result_pack_and_citation_input, StableCitationDrawerParity,
    StableDerivedCitationSet, StableDocsConsumerProjection, StableDocsConsumerSurface,
    StableDocsContractArtifactError, StableDocsFindingKind, StableDocsFindingSeverity,
    StableDocsPackDetailSheet, StableDocsPackDetailSheetKind, StableDocsPromotionState,
    StableDocsResultObject, StableDocsSourceDescriptor, StableDocsSourceResultPackCitationInput,
    StableDocsSourceResultPackCitationPacket, StableDocsSupportExport, StableDocsSupportTrustClass,
    StableDocsValidationFinding, StableExportPosture, StablePackActionSet,
    STABLE_DOCS_CONTRACT_ARTIFACT_DOC_REF, STABLE_DOCS_CONTRACT_ARTIFACT_REF,
    STABLE_DOCS_CONTRACT_DOC_REF, STABLE_DOCS_CONTRACT_FIXTURE_DIR,
    STABLE_DOCS_CONTRACT_RECORD_KIND, STABLE_DOCS_CONTRACT_SCHEMA_REF,
    STABLE_DOCS_CONTRACT_SCHEMA_VERSION, STABLE_DOCS_CONTRACT_SUPPORT_EXPORT_RECORD_KIND,
};
