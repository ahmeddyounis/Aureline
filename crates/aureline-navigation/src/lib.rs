//! Typed navigation target contracts shared by language, graph, search, shell, AI, review, and support surfaces.
//!
//! The crate owns the beta [`target_model`] vocabulary for definitions,
//! declarations, implementations, references, hierarchy edges, rename-preview
//! candidate sets, disambiguation sets, and breadcrumb/bookmark continuity
//! projections. Consumers should project their local provider records into
//! these types before rendering UI, serving CLI/headless output, assembling AI
//! context, or exporting support/review evidence.

#![doc(html_root_url = "https://docs.rs/aureline-navigation/0.0.0")]

pub mod target_model;

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
