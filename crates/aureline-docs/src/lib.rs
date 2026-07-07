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
pub mod add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context;
pub mod add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels;
pub mod add_version_freshness_vocabulary_and_stale_example_broken_link_findings;
pub mod authoring;
pub mod certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface;
pub mod certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims;
pub mod citations;
pub mod docs_browser_truth_packet;
pub mod docs_maintenance_and_stale_example_governance;
pub mod docs_pack_truth_packet;
pub mod evidence_model;
pub mod freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract;
pub mod freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix;
pub mod freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix;
pub mod freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix;
pub mod implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor;
pub mod implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports;
pub mod implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export;
pub mod implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes;
pub mod implement_docs_result_rows_and_source_or_version_badges_with_result_kind_provider_version_scope_and_freshness_truth;
pub mod implement_docs_search_bars_and_scope_switchers_with_corpus_provider_and_cached_live_state_truth;
pub mod implement_docs_symbol_linked_reference_cards_with_code_anchor_and_exact_nearby_project_or_keyword_fallback_truth;
pub mod implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings;
pub mod implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu;
pub mod index;
pub mod locale_overlay;
pub mod m5_docs_authoring_certification;
pub mod maintenance;
pub mod pack;
pub mod semantic_recall_boundary_truth_packet;
pub mod ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity;
pub mod ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links;
pub mod ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling;
pub mod ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history;
pub mod stable_docs_contract;
pub mod stable_docs_source_and_result_object_reuse_across_consumer_surfaces;

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
pub use add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context::{
    current_stable_docs_precedence_ranking_export, current_stable_docs_precedence_ranking_packet,
    seeded_stable_docs_precedence_ranking_input, DocsPrecedenceRankingArtifactError,
    DocsPrecedenceRankingFindingKind, DocsPrecedenceRankingFindingSeverity,
    DocsPrecedenceRankingPacket, DocsPrecedenceRankingPacketInput,
    DocsPrecedenceRankingPromotionState, DocsPrecedenceRankingSupportExport,
    DocsPrecedenceRankingValidationFinding, DocsRankingSet, DocsSourceLane, PrecedenceReason,
    RankExplanationProjection, RankExplanationSurface, RankSubjectKind, RankedDocsCandidate,
    DOCS_PRECEDENCE_RANKING_ARTIFACT_REF, DOCS_PRECEDENCE_RANKING_DOC_REF,
    DOCS_PRECEDENCE_RANKING_FIXTURE_DIR, DOCS_PRECEDENCE_RANKING_MATRIX_CONTRACT_REF,
    DOCS_PRECEDENCE_RANKING_RECORD_KIND, DOCS_PRECEDENCE_RANKING_SCHEMA_REF,
    DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION, DOCS_PRECEDENCE_RANKING_SOURCE_RESULT_CONTRACT_REF,
    DOCS_PRECEDENCE_RANKING_SUMMARY_REF, DOCS_PRECEDENCE_RANKING_SUPPORT_EXPORT_RECORD_KIND,
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
pub use add_version_freshness_vocabulary_and_stale_example_broken_link_findings::{
    current_stable_docs_version_freshness_export, current_stable_docs_version_freshness_packet,
    seeded_stable_docs_version_freshness_input, DocsVersionFreshnessArtifactError,
    DocsVersionFreshnessCard, DocsVersionFreshnessConfidence,
    DocsVersionFreshnessConsumerProjection, DocsVersionFreshnessConsumerSurface,
    DocsVersionFreshnessDisclosure, DocsVersionFreshnessFinding, DocsVersionFreshnessFindingActions,
    DocsVersionFreshnessFindingClass, DocsVersionFreshnessFindingSeverity,
    DocsVersionFreshnessPacket, DocsVersionFreshnessPacketInput, DocsVersionFreshnessPromotionState,
    DocsVersionFreshnessState, DocsVersionFreshnessSubjectKind, DocsVersionFreshnessSuppressionState,
    DocsVersionFreshnessSupportExport, DocsVersionFreshnessValidationFinding,
    DocsVersionFreshnessValidationKind, DocsVersionFreshnessValidationSeverity,
    DOCS_VERSION_FRESHNESS_ARTIFACT_REF, DOCS_VERSION_FRESHNESS_DOC_REF,
    DOCS_VERSION_FRESHNESS_FIXTURE_DIR, DOCS_VERSION_FRESHNESS_RECORD_KIND,
    DOCS_VERSION_FRESHNESS_SCHEMA_REF, DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
    DOCS_VERSION_FRESHNESS_SUMMARY_REF, DOCS_VERSION_FRESHNESS_SUPPORT_EXPORT_RECORD_KIND,
};
pub use authoring::evidence_handoff::{
    current_stable_docs_evidence_handoff_export,
    packet_to_input as docs_evidence_handoff_packet_to_input,
    seeded_stable_docs_evidence_handoff_input, DocsChangeKind, DocsChangeSubject,
    DocsEvidenceHandoffArtifactError, DocsEvidenceHandoffExport, DocsEvidenceHandoffPacket,
    DocsEvidenceHandoffPacketInput, DocsEvidenceHandoffSupportExport, EvidenceBinding,
    EvidenceHandoffEntry, EvidenceHandoffExportRow, EvidenceKind, EvidenceLocality,
    EvidenceProvenance, EvidenceRedactionState, EvidenceReopenHandle, EvidenceScope,
    EvidenceVersionMatch, EvidenceFreshness, HandoffConsumerProjection, HandoffConsumerSurface,
    HandoffDegradation, HandoffDegradationClass, HandoffExportScope, HandoffFinding,
    HandoffFindingKind, HandoffFindingSeverity, HandoffPromotionState, MirrorOfflinePosture,
    DOCS_EVIDENCE_HANDOFF_ARTIFACT_REF, DOCS_EVIDENCE_HANDOFF_DOC_REF,
    DOCS_EVIDENCE_HANDOFF_FIXTURE_DIR, DOCS_EVIDENCE_HANDOFF_RECORD_KIND,
    DOCS_EVIDENCE_HANDOFF_SCHEMA_REF, DOCS_EVIDENCE_HANDOFF_SCHEMA_VERSION,
    DOCS_EVIDENCE_HANDOFF_SUMMARY_REF, DOCS_EVIDENCE_HANDOFF_SUPPORT_EXPORT_RECORD_KIND,
};
pub use authoring::markdown_workspace::{
    current_stable_markdown_authoring_workspace_export, seeded_mode_commands,
    seeded_recovery_command, seeded_stable_markdown_authoring_workspace,
    seeded_stable_markdown_authoring_workspace_input, BrowserHandoffAvailability,
    MarkdownAuthoringWorkspace, MarkdownAuthoringWorkspaceInput, MarkdownWorkspaceArtifactError,
    MarkdownWorkspaceViolation, RenderCapability, WorkspaceAnchor, WorkspaceAnchorKind,
    WorkspaceModeCommand, WorkspaceRecoveryCommand, WorkspaceRenderCapabilities,
    MARKDOWN_WORKSPACE_ARTIFACT_REF, MARKDOWN_WORKSPACE_DOC_REF, MARKDOWN_WORKSPACE_FIXTURE_DIR,
    MARKDOWN_WORKSPACE_RECORD_KIND, MARKDOWN_WORKSPACE_SCHEMA_REF,
    MARKDOWN_WORKSPACE_SCHEMA_VERSION, MARKDOWN_WORKSPACE_SUMMARY_REF, MODE_RENDERED_COMMAND_ID,
    MODE_SOURCE_COMMAND_ID, MODE_SPLIT_COMMAND_ID, RECOVER_SOURCE_COMMAND_ID,
};
pub use authoring::suggestion_panel::{
    current_stable_docs_suggestion_panel_export, packet_to_input as docs_suggestion_panel_packet_to_input,
    seeded_stable_docs_suggestion_panel_input, DocsSuggestionPanelArtifactError,
    DocsSuggestionPanelExport, DocsSuggestionPanelPacket, DocsSuggestionPanelPacketInput,
    DocsSuggestionPanelSupportExport, PanelActionSet, PanelApplyPosture, PanelChipSet,
    PanelConfidence, PanelConsumerProjection, PanelConsumerSurface, PanelDegradation,
    PanelDegradationClass, PanelDisposition, PanelDispositionState, PanelEvidenceProvenance,
    PanelExportScope, PanelFindingKind, PanelFindingSeverity, PanelFreshness, PanelLocality,
    PanelProposal, PanelProposalKind, PanelPromotionState, PanelSuggestion,
    PanelSuggestionExportRow, PanelSuggestionTarget, PanelTargetKind, PanelTrigger,
    PanelTriggerSource, PanelValidationFinding, PanelVersionMatch,
    DOCS_SUGGESTION_PANEL_ARTIFACT_REF, DOCS_SUGGESTION_PANEL_DOC_REF,
    DOCS_SUGGESTION_PANEL_FIXTURE_DIR, DOCS_SUGGESTION_PANEL_RECORD_KIND,
    DOCS_SUGGESTION_PANEL_SCHEMA_REF, DOCS_SUGGESTION_PANEL_SCHEMA_VERSION,
    DOCS_SUGGESTION_PANEL_SUMMARY_REF, DOCS_SUGGESTION_PANEL_SUPPORT_EXPORT_RECORD_KIND,
};
pub use authoring::validation_report::{
    current_stable_docs_validation_report_export,
    packet_to_input as docs_validation_report_packet_to_input,
    seeded_stable_docs_validation_report_input, DocsValidationReportArtifactError,
    DocsValidationReportExport, DocsValidationReportPacket, DocsValidationReportPacketInput,
    DocsValidationReportSupportExport, ValidationActionSet, ValidationChipSet,
    ValidationConsumerProjection, ValidationConsumerSurface, ValidationDegradation,
    ValidationDegradationClass, ValidationEvidenceProvenance, ValidationExportScope,
    ValidationFinding, ValidationFindingKind, ValidationFindingSeverity, ValidationFreshness,
    ValidationLocality, ValidationMode, ValidationOutcome, ValidationProducer,
    ValidationPromotionState, ValidationReportExportRow, ValidationReportRow, ValidationScope,
    ValidationSubject, ValidationSubjectKind, ValidationSuppression, ValidationSuppressionState,
    ValidationVersionMatch, ValidatorKind, DOCS_VALIDATION_REPORT_ARTIFACT_REF,
    DOCS_VALIDATION_REPORT_DOC_REF, DOCS_VALIDATION_REPORT_FIXTURE_DIR,
    DOCS_VALIDATION_REPORT_RECORD_KIND, DOCS_VALIDATION_REPORT_SCHEMA_REF,
    DOCS_VALIDATION_REPORT_SCHEMA_VERSION, DOCS_VALIDATION_REPORT_SUMMARY_REF,
    DOCS_VALIDATION_REPORT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use authoring::release_docs_surface::{
    seeded_release_docs_maintenance_contract, seeded_release_docs_review_packet,
    seeded_release_docs_surface_projection, validate_seeded_release_docs_maintenance,
    ReleaseDocsCompareEntry, ReleaseDocsCompareKind, ReleaseDocsCoverage, ReleaseDocsEvidenceScope,
    ReleaseDocsFinding, ReleaseDocsIntegrationAnchor, ReleaseDocsIntegrationTarget,
    ReleaseDocsMaintenanceContract, ReleaseDocsMaintenanceSurface, ReleaseDocsReviewPacket,
    ReleaseDocsSurfaceProjection, OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL,
    RELEASE_DOCS_MAINTENANCE_CONTRACT_ID, RELEASE_DOCS_MAINTENANCE_CONTRACT_RECORD_KIND,
    RELEASE_DOCS_MAINTENANCE_DOC_REF, RELEASE_DOCS_MAINTENANCE_FIXTURE_DIR,
    RELEASE_DOCS_MAINTENANCE_SCHEMA_REF, RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
    RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND, RELEASE_DOCS_MAINTENANCE_VERSION_REF,
    RELEASE_DOCS_REVIEW_PACKET_RECORD_KIND, RELEASE_DOCS_SURFACE_PROJECTION_RECORD_KIND,
    REOPEN_RELEASE_DOCS_COMPARE_ACTION_LABEL, REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
};
pub use authoring::safe_rendered_preview::{
    current_stable_rendered_preview_boundary_export, not_applicable_capability_boundaries,
    seeded_capability_boundaries,
    seeded_recovery_command as seeded_rendered_preview_recovery_command,
    seeded_stable_rendered_preview_boundary, seeded_stable_rendered_preview_boundary_input,
    AccessibilityParity, CapabilityAuthority, CapabilityRequestState, PreviewBoundaryViolation,
    PreviewCapabilityBoundary, PreviewCapabilityKind, PreviewRenderPosture, PreviewSurfaceOwner,
    RenderedPreviewBoundary, RenderedPreviewBoundaryArtifactError, RenderedPreviewBoundaryInput,
    OPEN_SOURCE_ACTION_REF as RENDERED_PREVIEW_OPEN_SOURCE_ACTION_REF,
    RECOVER_SOURCE_COMMAND_ID as RENDERED_PREVIEW_RECOVER_SOURCE_COMMAND_ID,
    RENDERED_PREVIEW_BOUNDARY_ARTIFACT_REF, RENDERED_PREVIEW_BOUNDARY_DOC_REF,
    RENDERED_PREVIEW_BOUNDARY_FIXTURE_DIR, RENDERED_PREVIEW_BOUNDARY_RECORD_KIND,
    RENDERED_PREVIEW_BOUNDARY_SCHEMA_REF, RENDERED_PREVIEW_BOUNDARY_SCHEMA_VERSION,
    RENDERED_PREVIEW_BOUNDARY_SUMMARY_REF,
};
pub use certify_docs_browser_semantic_recall_and_codebase_understanding_rows_and_narrow_any_underqualified_surface::{
    current_stable_certification_export, seeded_stable_certification_input,
    CertificationArtifactError, CertificationCompatibilityReport, CertificationConsumerProjection,
    CertificationConsumerSurface, CertificationDowngradeAction, CertificationDowngradeRule,
    CertificationDowngradeTrigger, CertificationPacket, CertificationPacketInput,
    CertificationProofFreshness, CertificationQualificationClass, CertificationTrustReview,
    CertificationVerdict, CertificationViolation, CertifiedSurfaceLane, CertifiedSurfaceRow,
    CERTIFICATION_ARTIFACT_REF, CERTIFICATION_DOC_REF, CERTIFICATION_FIXTURE_DIR,
    CERTIFICATION_RECORD_KIND, CERTIFICATION_SCHEMA_REF, CERTIFICATION_SCHEMA_VERSION,
    CERTIFICATION_SUMMARY_REF,
};
pub use certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims::{
    current_stable_docs_claim_certification_export, seeded_stable_docs_claim_certification_input,
    CertifiedDocsProfile, DocsClaimCertificationArtifactError, DocsClaimCertificationPacket,
    DocsClaimCertificationPacketInput, DocsClaimCertificationViolation,
    DocsClaimCompatibilityReport, DocsClaimConsumerProjection, DocsClaimConsumerSurface,
    DocsClaimDowngradeAction, DocsClaimDowngradeRule, DocsClaimDowngradeTrigger,
    DocsClaimProofFreshness, DocsClaimQualificationClass, DocsClaimTrustReview, DocsClaimVerdict,
    DocsEvidenceClass, DocsProfileQualificationRow, DOCS_CLAIM_CERTIFICATION_ARTIFACT_REF,
    DOCS_CLAIM_CERTIFICATION_DOC_REF, DOCS_CLAIM_CERTIFICATION_FIXTURE_DIR,
    DOCS_CLAIM_CERTIFICATION_RECORD_KIND, DOCS_CLAIM_CERTIFICATION_SCHEMA_REF,
    DOCS_CLAIM_CERTIFICATION_SCHEMA_VERSION, DOCS_CLAIM_CERTIFICATION_SUMMARY_REF,
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
    DocsInfraTruthLayer, DocsInfrastructureLineage, DocsKnowledgeObjectKind,
    DocsKnowledgeSourceStrip, DocsKnowledgeSurfaceEvidencePacket,
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
pub use freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix::{
    current_stable_m5_docs_contracts_matrix_export, DocsContractBrowserHandoffPrivacyConsequence,
    DocsContractBrowserHandoffReason, DocsContractFreshnessState, DocsContractLocaleMatch,
    DocsContractMirrorOfflinePosture, DocsContractSourceClass, DocsContractTrustClass,
    DocsContractVersionMatchState, M5DocsContractStateVocabulary,
    M5DocsContractsConsumerProjection, M5DocsContractsConsumerSurface,
    M5DocsContractsDowngradeTrigger, M5DocsContractsEvidenceRequirement,
    M5DocsContractsMatrixArtifactError, M5DocsContractsMatrixPacket,
    M5DocsContractsMatrixPacketInput, M5DocsContractsMatrixViolation,
    M5DocsContractsProofFreshness, M5DocsContractsQualificationClass,
    M5DocsContractsReleasePosture, M5DocsContractsRollbackPosture, M5DocsContractsTrustReview,
    M5DocsContractsVocabularySet, M5DocsObjectKind, M5DocsObjectRow,
    M5_DOCS_CONTRACTS_BROWSER_HANDOFF_CONTRACT_REF, M5_DOCS_CONTRACTS_DERIVED_EXPLANATION_CONTRACT_REF,
    M5_DOCS_CONTRACTS_DOCS_BROWSER_CONTRACT_REF, M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF,
    M5_DOCS_CONTRACTS_MATRIX_DOC_REF, M5_DOCS_CONTRACTS_MATRIX_FIXTURE_DIR,
    M5_DOCS_CONTRACTS_MATRIX_RECORD_KIND, M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF,
    M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION, M5_DOCS_CONTRACTS_MATRIX_SUMMARY_REF,
    M5_DOCS_CONTRACTS_PACK_MANIFEST_CONTRACT_REF,
    M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF,
};
pub use freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix::{
    current_stable_m5_markdown_authoring_matrix_export, M5AuthoringConsumerSurface,
    M5AuthoringDowngradeTrigger, M5AuthoringEvidenceHandoffScope, M5AuthoringEvidenceRequirement,
    M5AuthoringMatrixArtifactError, M5AuthoringMatrixConsumerProjection, M5AuthoringMatrixLaneRow,
    M5AuthoringMatrixPacket, M5AuthoringMatrixPacketInput, M5AuthoringMatrixProofFreshness,
    M5AuthoringMatrixReleasePosture, M5AuthoringMatrixTrustReview, M5AuthoringMatrixViolation,
    M5AuthoringPreviewSafetyClass, M5AuthoringQualificationClass, M5AuthoringRollbackPosture,
    M5AuthoringSuggestionTrigger, M5AuthoringSurface, M5AuthoringValidationState,
    M5AuthoringWorkspaceMode, M5_AUTHORING_MATRIX_ARTIFACT_REF,
    M5_AUTHORING_MATRIX_BROWSER_HANDOFF_CONTRACT_REF, M5_AUTHORING_MATRIX_DOCS_PACK_CONTRACT_REF,
    M5_AUTHORING_MATRIX_DOC_REF, M5_AUTHORING_MATRIX_FIXTURE_DIR,
    M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF, M5_AUTHORING_MATRIX_RECORD_KIND,
    M5_AUTHORING_MATRIX_SCHEMA_REF, M5_AUTHORING_MATRIX_SCHEMA_VERSION,
    M5_AUTHORING_MATRIX_SUGGESTION_CONTRACT_REF, M5_AUTHORING_MATRIX_SUMMARY_REF,
};
pub use implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor::{
    current_stable_browser_handoff_export, current_stable_browser_handoff_packet,
    seeded_stable_browser_handoff_input, BrowserHandoff, BrowserHandoffArtifactError,
    BrowserHandoffPacket, BrowserHandoffPacketInput, BrowserHandoffPromotionState,
    BrowserHandoffConsumerProjection, BrowserHandoffConsumerSurface, BrowserHandoffSupportExport,
    BrowserHandoffValidationFinding, BrowserHandoffValidationKind,
    BrowserHandoffValidationSeverity, HandoffDestinationClass, HandoffPolicyPosture,
    HandoffSourceSurface, ReturnAnchor,
    ReturnAnchorKind, SharedContext, BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF,
    BROWSER_HANDOFF_OBJECTS_DOC_REF, BROWSER_HANDOFF_OBJECTS_FIXTURE_DIR,
    BROWSER_HANDOFF_OBJECTS_INTEGRATION_CONTRACT_REF, BROWSER_HANDOFF_OBJECTS_MATRIX_CONTRACT_REF,
    BROWSER_HANDOFF_OBJECTS_RECORD_KIND, BROWSER_HANDOFF_OBJECTS_SCHEMA_REF,
    BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION, BROWSER_HANDOFF_OBJECTS_SUMMARY_REF,
    BROWSER_HANDOFF_OBJECTS_SUPPORT_EXPORT_RECORD_KIND,
};
pub use implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports::{
    current_stable_derived_explanation_citation_export,
    current_stable_derived_explanation_citation_packet,
    seeded_stable_derived_explanation_citation_input, CitationBasis, CitationConsumerProjection,
    CitationRedactionState, CitedDocRef, CitedFileRef, CitedSymbolRef,
    DerivationTool, DerivedExplanationCitationArtifactError, DerivedExplanationCitationPacket,
    DerivedExplanationCitationPacketInput, DerivedExplanationCitationPromotionState,
    DerivedExplanationCitationSet, DerivedExplanationCitationSupportExport,
    DerivedExplanationCitationValidationFinding, DerivedExplanationCitationValidationKind,
    DerivedExplanationCitationValidationSeverity, DerivedExplanationSurface, GraphEpochRef,
    InferenceConfidence, InferenceLabel, DERIVED_EXPLANATION_CITATION_ARTIFACT_REF,
    DERIVED_EXPLANATION_CITATION_DOC_REF, DERIVED_EXPLANATION_CITATION_FIXTURE_DIR,
    DERIVED_EXPLANATION_CITATION_MATRIX_CONTRACT_REF, DERIVED_EXPLANATION_CITATION_RECORD_KIND,
    DERIVED_EXPLANATION_CITATION_SCHEMA_REF, DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION,
    DERIVED_EXPLANATION_CITATION_SUMMARY_REF,
    DERIVED_EXPLANATION_CITATION_SUPPORT_EXPORT_RECORD_KIND,
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
pub use implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes::{
    current_stable_docs_authoring_review_export,
    packet_to_input as docs_authoring_review_packet_to_input,
    seeded_stable_docs_authoring_review_input, AuthoringSuggestion,
    CapturedVsLive as DocsAuthoringReviewCapturedVsLive, DocsAuthoringReviewArtifactError,
    DocsAuthoringReviewExport, DocsAuthoringReviewExportRow, DocsAuthoringReviewPacket,
    DocsAuthoringReviewPacketInput, DocsAuthoringReviewSupportExport, DocsReviewChipSet,
    DocsReviewConfidence, DocsReviewConsumerProjection, DocsReviewConsumerSurface,
    DocsReviewDegradation, DocsReviewDegradationClass, DocsReviewExportScope, DocsReviewFindingKind,
    DocsReviewFindingSeverity, DocsReviewFreshness, DocsReviewItem, DocsReviewItemKind,
    DocsReviewLocality, DocsReviewPromotionState, DocsReviewSourceClass, DocsReviewTrustClass,
    DocsReviewValidationFinding, DocsReviewVersionMatch, ReviewFindingClass, StaleReviewVerdict,
    SuggestionApplyPosture, SuggestionTrigger, DOCS_AUTHORING_REVIEW_ARTIFACT_REF,
    DOCS_AUTHORING_REVIEW_DOC_REF, DOCS_AUTHORING_REVIEW_FIXTURE_DIR,
    DOCS_AUTHORING_REVIEW_RECORD_KIND, DOCS_AUTHORING_REVIEW_SCHEMA_REF,
    DOCS_AUTHORING_REVIEW_SCHEMA_VERSION, DOCS_AUTHORING_REVIEW_SUMMARY_REF,
    DOCS_AUTHORING_REVIEW_SUPPORT_EXPORT_RECORD_KIND,
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
pub use m5_docs_authoring_certification::{
    certify_profile_row, current_stable_docs_authoring_cert_report, full_surface_coverage,
    seeded_stable_docs_authoring_cert_input, seeded_stable_docs_authoring_cert_report,
    CertCompatibilityReport, CertConsumerProjection, CertDowngradeAction, CertDowngradeRule,
    CertDowngradeTrigger, CertFreshnessState, CertProofFreshness, CertQualificationClass,
    CertTrustReview, CertVerdict, CertViolation, DocsAuthoringCertArtifactError,
    DocsAuthoringCertIndex, DocsAuthoringCertReport, DocsAuthoringCertReportInput,
    DocsAuthoringCertSurface, DocsAuthoringProfile, DocsAuthoringProfileRow, ProfileRowInput,
    ProfileSurfaceCoverage, WaiverAndDowngradeLog, WaiverLogEntry, WaiverLogEntryKind,
    DOCS_AUTHORING_CERT_ARTIFACT_REF, DOCS_AUTHORING_CERT_DOC_REF, DOCS_AUTHORING_CERT_FIXTURE_DIR,
    DOCS_AUTHORING_CERT_RECORD_KIND, DOCS_AUTHORING_CERT_SCHEMA_REF,
    DOCS_AUTHORING_CERT_SCHEMA_VERSION, DOCS_AUTHORING_CERT_SUMMARY_REF,
    DOCS_AUTHORING_WAIVER_LOG_RECORD_KIND, DOCS_AUTHORING_WAIVER_LOG_REF,
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
pub use ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity::{
    current_stable_docs_pack_manager_export, current_stable_docs_pack_manager_packet,
    seeded_stable_docs_pack_manager_input, DocsPackImportExportContinuity, DocsPackImportOrigin,
    DocsPackLifecycleFlow, DocsPackManagerAction, DocsPackManagerActionAvailability,
    DocsPackManagerActionState, DocsPackManagerArtifactError, DocsPackManagerFindingKind,
    DocsPackManagerFindingSeverity, DocsPackManagerPacket, DocsPackManagerPacketInput,
    DocsPackManagerProfile, DocsPackManagerProfileProjection, DocsPackManagerPromotionState,
    DocsPackManagerRow, DocsPackManagerSupportExport, DocsPackManagerValidationFinding,
    DOCS_PACK_MANAGER_ARTIFACT_REF, DOCS_PACK_MANAGER_DOC_REF, DOCS_PACK_MANAGER_FIXTURE_DIR,
    DOCS_PACK_MANAGER_RECORD_KIND, DOCS_PACK_MANAGER_SCHEMA_REF, DOCS_PACK_MANAGER_SCHEMA_VERSION,
    DOCS_PACK_MANAGER_SUMMARY_REF, DOCS_PACK_MANAGER_SUPPORT_EXPORT_RECORD_KIND,
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
pub use stable_docs_source_and_result_object_reuse_across_consumer_surfaces::{
    current_stable_docs_source_result_reuse_export,
    current_stable_docs_source_result_reuse_packet, seeded_stable_docs_source_result_reuse_input,
    DocsObjectConsumerSurface, DocsObjectFindingKind, DocsObjectFindingSeverity,
    DocsObjectPromotionState, DocsObjectReuseArtifactError, DocsObjectReusePacket,
    DocsObjectReusePacketInput, DocsObjectReuseSupportExport, DocsObjectSurfaceProjection,
    DocsObjectTrustClass, DocsObjectValidationFinding, DocsResult, DocsSnippetMeta,
    DocsSourceDescriptor, DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF, DOCS_SOURCE_RESULT_REUSE_DOC_REF,
    DOCS_SOURCE_RESULT_REUSE_FIXTURE_DIR, DOCS_SOURCE_RESULT_REUSE_RECORD_KIND,
    DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF, DOCS_SOURCE_RESULT_REUSE_SCHEMA_VERSION,
    DOCS_SOURCE_RESULT_REUSE_SUMMARY_REF, DOCS_SOURCE_RESULT_REUSE_SUPPORT_EXPORT_RECORD_KIND,
};
