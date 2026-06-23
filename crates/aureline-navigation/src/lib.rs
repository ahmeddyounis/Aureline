//! Typed navigation target contracts shared by language, graph, search, shell, AI, review, and support surfaces.
//!
//! The crate owns the beta [`target_model`] vocabulary for definitions,
//! declarations, implementations, references, hierarchy edges, rename-preview
//! candidate sets, disambiguation sets, and breadcrumb/bookmark continuity
//! projections. Consumers should project their local provider records into
//! these types before rendering UI, serving CLI/headless output, assembling AI
//! context, or exporting support/review evidence.
//!
//! [`m5_relation_navigation`] freezes the governance matrix over that object
//! model: it names every governed relation-navigation object family (navigation
//! target, reference occurrence, hierarchy edge, related-object relation,
//! rename-preview set, and the relation/fallback vocabulary), pins one controlled
//! vocabulary across relation kinds, proof classes, access kinds, ambiguity,
//! freshness, partiality, generated/runtime labels, and rename omission reasons,
//! maps each object to the proof packet that keeps it current, and states the
//! invariants every search, graph, docs, and editor surface must hold.

#![doc(html_root_url = "https://docs.rs/aureline-navigation/0.0.0")]

pub mod bookmark_history_and_drift_continuity;
pub mod m5_relation_navigation;
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

pub use m5_relation_navigation::{
    relation_navigation_lines, relation_navigation_matrix, RelationNavConsumer,
    RelationNavFieldDef, RelationNavInvariant, RelationNavMatrixValidationError,
    RelationNavObjectClass, RelationNavObjectEntry, RelationNavRedactionClass,
    RelationNavSharedVocabulary, RelationNavStateClass, RelationNavStateTerm, RelationNavTokenDef,
    RelationNavVocabulary, RelationNavigationMatrix, M5_RELATION_NAVIGATION_AS_OF,
    M5_RELATION_NAVIGATION_FREEZE_GATE_REF, M5_RELATION_NAVIGATION_MATRIX_ID,
    M5_RELATION_NAVIGATION_RECORD_KIND, M5_RELATION_NAVIGATION_SCHEMA_REF,
    M5_RELATION_NAVIGATION_SCHEMA_VERSION,
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
