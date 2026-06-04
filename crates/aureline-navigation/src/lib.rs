//! Typed navigation target contracts shared by language, graph, search, shell, AI, review, and support surfaces.
//!
//! The crate owns the beta [`target_model`] vocabulary for definitions,
//! declarations, implementations, references, hierarchy edges, rename-preview
//! candidate sets, disambiguation sets, and breadcrumb/bookmark continuity
//! projections. Consumers should project their local provider records into
//! these types before rendering UI, serving CLI/headless output, assembling AI
//! context, or exporting support/review evidence.

#![doc(html_root_url = "https://docs.rs/aureline-navigation/0.0.0")]

pub mod bookmark_history_and_drift_continuity;
pub mod target_model;

pub use bookmark_history_and_drift_continuity::{
    validate_navigation_continuity_packet, BreadcrumbTrail, DurableNavigationAnchor,
    NavigationConsumerProjection, NavigationContinuityArtifact, NavigationContinuityArtifactKind,
    NavigationContinuityError, NavigationContinuityFinding, NavigationContinuityFindingKind,
    NavigationContinuityFindingSeverity, NavigationContinuityPacket, NavigationContinuitySurface,
    NavigationDriftState, NavigationHistoryEntry, NavigationMark, NavigationScopeRef,
    NavigationSourceRef, OutlineSnapshot, PeekContext, RestoreNavigationArtifact,
    RestoreNavigationPacket, StableAnchorRemap, BOOKMARK_HISTORY_CONTINUITY_ARTIFACT_REF,
    BOOKMARK_HISTORY_CONTINUITY_DOC_REF, BOOKMARK_HISTORY_CONTINUITY_FIXTURE_DIR,
    BOOKMARK_HISTORY_CONTINUITY_PACKET_RECORD_KIND, BOOKMARK_HISTORY_CONTINUITY_SCHEMA_REF,
    BOOKMARK_HISTORY_CONTINUITY_SCHEMA_VERSION, REQUIRED_CONTINUITY_SURFACES,
    REQUIRED_DRIFT_STATES,
};

pub use target_model::{
    current_navigation_target_fidelity_corpus, current_navigation_target_fidelity_fixture_refs,
    load_navigation_target_fidelity_case, AccessKind, AmbiguityClass, ConsumerProjection,
    ConsumerSurface, ContinuityArtifactKind, ContinuityState, DowngradeReason,
    ExportRedactionClass, FreshnessClass, GeneratedOrExternalState, HierarchyEdge,
    HierarchyEdgeKind, NavigationConfidence, NavigationDisambiguationSet, NavigationPromotionState,
    NavigationTarget, NavigationTargetCountSummary, NavigationTargetFidelityCase,
    NavigationTargetFidelityCorpus, NavigationTargetFidelityCorpusEntry,
    NavigationTargetFidelityEvaluator, NavigationTargetFidelityReferences,
    NavigationTargetFidelityReport, NavigationTargetFidelityReportRow,
    NavigationTargetFidelityViolation, NavigationTargetModelError, NavigationTargetModelVersion,
    NavigationTargetRef, ProofClass, ProviderClass, ReferenceOccurrence, RelationKind,
    RenameApplyPosture, RenamePreviewSet, ScopeCompleteness, TargetContinuityRef,
    NAVIGATION_TARGET_BETA_CONTRACT_DOC_REF, NAVIGATION_TARGET_FIDELITY_CASE_RECORD_KIND,
    NAVIGATION_TARGET_FIDELITY_CORPUS_DIR, NAVIGATION_TARGET_FIDELITY_REPORT_RECORD_KIND,
    NAVIGATION_TARGET_FIDELITY_REPORT_REF, NAVIGATION_TARGET_SCHEMA_REF,
    NAVIGATION_TARGET_SCHEMA_VERSION, REQUIRED_CONSUMER_SURFACES, REQUIRED_RELATION_KINDS,
};
