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
//!
//! [`relation_resolution`] turns that object model into a no-silent-aliasing
//! resolver: given a Go to Definition / Declaration / Implementation command and
//! the candidate targets providers returned, it resolves to a single distinct
//! relation kind, opens a disambiguation set instead of guessing when multiple
//! candidates could change behavior, and never relabels one relation kind as
//! another — it discloses a fallback or reports the command unavailable, and
//! records a replayable explanation so support and debug packets can reconstruct
//! which relation kind was navigated and why.
//!
//! [`reference_panes`] turns the reference occurrence into a typed pane and export
//! model: it groups occurrences by access kind, separates current-scope from
//! captured-scope counts, names whether each group's evidence is semantic,
//! framework-derived, runtime-observed, imported, or a lexical fallback, keeps
//! generated/external/test-only labels visible, exposes stable open/peek/split/
//! export actions identically across the references pane, search panel, docs links,
//! and keyboard routes, and projects to review, support, AI, and graph consumers
//! without flattening a reference set into generic search hits.

#![doc(html_root_url = "https://docs.rs/aureline-navigation/0.0.0")]

pub mod bookmark_history_and_drift_continuity;
pub mod m5_relation_navigation;
pub mod reference_panes;
pub mod relation_resolution;
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

pub use reference_panes::{
    build_reference_pane, reference_panes_lines, reference_panes_set, ActionRoute, HistoryEffect,
    PaneActionAffordance, PaneActionKind, ReferenceEvidenceClass, ReferenceGroup, ReferenceLabel,
    ReferencePane, ReferencePaneInput, ReferencePaneInvariant, ReferencePaneProjection,
    ReferencePaneScenario, ReferencePaneSet, ReferencePaneValidationError, ReferenceScopeCounts,
    REFERENCE_ACCESS_KIND_ORDER, REFERENCE_PANES_ARTIFACT_REF, REFERENCE_PANES_AS_OF,
    REFERENCE_PANES_DOC_REF, REFERENCE_PANES_FIXTURE_REF, REFERENCE_PANES_FREEZE_GATE_REF,
    REFERENCE_PANES_RECORD_KIND, REFERENCE_PANES_SCHEMA_REF, REFERENCE_PANES_SCHEMA_VERSION,
    REFERENCE_PANES_SET_ID,
};

pub use relation_resolution::{
    relation_resolution_lines, relation_resolution_set, resolve_navigation, AliasingPosture,
    NavigationCommand, NavigationRequest, NavigationResolution, ProviderReach,
    RelationResolutionInvariant, RelationResolutionScenario, RelationResolutionSet,
    RelationResolutionValidationError, RelationResolvedTarget, ResolutionDisposition,
    RELATION_RESOLUTION_ARTIFACT_REF, RELATION_RESOLUTION_AS_OF, RELATION_RESOLUTION_DOC_REF,
    RELATION_RESOLUTION_FIXTURE_REF, RELATION_RESOLUTION_FREEZE_GATE_REF,
    RELATION_RESOLUTION_RECORD_KIND, RELATION_RESOLUTION_SCHEMA_REF,
    RELATION_RESOLUTION_SCHEMA_VERSION, RELATION_RESOLUTION_SET_ID,
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
