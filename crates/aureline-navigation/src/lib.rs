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
//!
//! [`hierarchy_views`] turns the hierarchy edge into a typed view and export model:
//! it builds call, type, override, and ownership hierarchy views that group edges by
//! a direct/transitive/inferred/runtime-observed legend, separate current-scope from
//! captured-scope counts, name every hidden or missing scope explicitly, preserve
//! provider attribution, freshness, and confidence, expose competing roots and a
//! disambiguation path before a jump when the root is ambiguous, carry stable
//! open/peek/split/expand/export actions across the hierarchy view, graph overlay,
//! search panel, docs link, and keyboard routes, and project to review, support, AI,
//! graph, and docs consumers without flattening a hierarchy into one opaque tree.
//!
//! [`related_object_navigation`] turns the related-object relation into a typed,
//! source-attributed panel and export model: it builds route, component, test, doc,
//! owner, and generated-artifact links grouped by a graph-derived/framework-derived/
//! curated/runtime-derived source legend, carries each link's fallback mode, freshness,
//! proof, and scope so a framework guess never poses as a graph-proven fact, separates
//! current-scope from captured-scope counts, names the anchor context the panel was
//! invoked from and whether it supports stable relation anchors so notebook, diff,
//! docs-linked, and generated-artifact contexts reuse the same relation semantics and
//! unsupported parity is labeled honestly, exposes competing links and a disambiguation
//! path before a jump, carries stable open/peek/split/reveal/export actions across every
//! route, and projects to review, support, AI, graph, and docs consumers without
//! flattening into generic smart links.
//!
//! [`relation_continuity`] turns peek, temporary reveal, open-in-split, back/forward
//! history, and recent-location entries into a relation-aware, replay-safe support/export
//! packet: each entry preserves its relation kind, origin surface, return anchor, and
//! current-versus-captured target truth; a remapped, drifted, missing-target,
//! scope-unavailable, or archived entry keeps its drift state, reason, and recovery
//! choices visible and never silently jumps to a nearby guess; every entry and rename-
//! preview-evidence row names its evidence class so a grep fallback is never replayed as
//! semantic and carries a replay-safe target id; and every consumer surface preserves that
//! truth without retargeting or exporting code bodies — so symbol navigation and rename
//! evidence survive replay, drift, and return-context restoration.
//!
//! [`rename_preview`] turns the rename-preview set into a typed, governed preview-and-
//! apply model: it groups rename candidates into the editable set and the held set
//! (blocked, conflict, generated, read-only, partial-scope) by a fixed precedence so a
//! held candidate is never folded into the editable count, separates change-versus-held
//! and current-versus-captured counts, keeps every omitted candidate visible with its
//! omission reason and label, names whether evidence is semantic, framework-derived,
//! runtime-observed, imported, or a lexical fallback so a grep match is never renamed as
//! certainty, enforces an inspect-before-mutate apply gate that always blocks a blind
//! apply and binds an undo checkpoint, projects the frozen rename-preview-set object, and
//! reaches review, support, AI, graph, docs, and editor consumers without flattening a
//! broad rename into one generic apply action.
//!
//! [`relation_navigation_qualification`] turns that object model and its sibling
//! lanes into a claim-governance certification: it names every certified relation-
//! navigation family (definition/declaration/implementation target-kind honesty,
//! references/access-kind truth, hierarchy proof classes, related-object attribution,
//! rename-preview completeness, and continuity/replay fidelity), publishes one
//! qualification row per claimed search/graph/docs/editor surface, derives every
//! claim state purely from its proof state and freshness so a stale or failing proof
//! narrows or withdraws the affected claim automatically, emits explicit release
//! evidence rows for each family, and projects to the About, Help, search/navigation,
//! support, compatibility, release-truth, and public-truth surfaces that consume the
//! same qualification state instead of restating relation-navigation quality by hand.

#![doc(html_root_url = "https://docs.rs/aureline-navigation/0.0.0")]

pub mod bookmark_history_and_drift_continuity;
pub mod hierarchy_views;
pub mod m5_relation_navigation;
pub mod reference_panes;
pub mod related_object_navigation;
pub mod relation_continuity;
pub mod relation_navigation_qualification;
pub mod relation_resolution;
pub mod rename_preview;
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

pub use hierarchy_views::{
    build_hierarchy_view, edge_legend, hierarchy_views_lines, hierarchy_views_set,
    HierarchyActionAffordance, HierarchyActionKind, HierarchyActionRoute, HierarchyAmbiguityState,
    HierarchyDirection, HierarchyEdgeCounts, HierarchyEdgeLegend, HierarchyHistoryEffect,
    HierarchyLabel, HierarchyScopeGap, HierarchyTier, HierarchyView, HierarchyViewInput,
    HierarchyViewInvariant, HierarchyViewKind, HierarchyViewProjection, HierarchyViewScenario,
    HierarchyViewSet, HierarchyViewValidationError, HIERARCHY_LEGEND_ORDER,
    HIERARCHY_VIEWS_ARTIFACT_REF, HIERARCHY_VIEWS_AS_OF, HIERARCHY_VIEWS_DOC_REF,
    HIERARCHY_VIEWS_FIXTURE_REF, HIERARCHY_VIEWS_FREEZE_GATE_REF, HIERARCHY_VIEWS_RECORD_KIND,
    HIERARCHY_VIEWS_SCHEMA_REF, HIERARCHY_VIEWS_SCHEMA_VERSION, HIERARCHY_VIEWS_SET_ID,
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

pub use related_object_navigation::{
    build_related_object_panel, related_object_navigation_lines, related_object_navigation_set,
    AnchorParity, RelatedObjectActionAffordance, RelatedObjectActionKind, RelatedObjectActionRoute,
    RelatedObjectAnchorContext, RelatedObjectCounts, RelatedObjectDisambiguation,
    RelatedObjectFallbackMode, RelatedObjectGroup, RelatedObjectHeadline,
    RelatedObjectHistoryEffect, RelatedObjectInvariant, RelatedObjectKind, RelatedObjectLabel,
    RelatedObjectLink, RelatedObjectNavigationSet, RelatedObjectPanel, RelatedObjectPanelInput,
    RelatedObjectProjection, RelatedObjectScenario, RelatedObjectSourceClass,
    RelatedObjectValidationError, RELATED_OBJECT_NAV_ARTIFACT_REF, RELATED_OBJECT_NAV_AS_OF,
    RELATED_OBJECT_NAV_DOC_REF, RELATED_OBJECT_NAV_FIXTURE_REF, RELATED_OBJECT_NAV_FREEZE_GATE_REF,
    RELATED_OBJECT_NAV_RECORD_KIND, RELATED_OBJECT_NAV_SCHEMA_REF,
    RELATED_OBJECT_NAV_SCHEMA_VERSION, RELATED_OBJECT_NAV_SET_ID, RELATED_OBJECT_SOURCE_ORDER,
};

pub use rename_preview::{
    build_rename_preview, rename_preview_lines, rename_preview_set, GovernedRenamePreview,
    RenameApplyGate, RenameApplyPrecondition, RenameCandidate, RenameCandidateCounts,
    RenameCandidateGroup, RenameCandidateGroupKind, RenameCandidateLabel, RenameEvidenceClass,
    RenameOmissionReason, RenamePreviewGovernanceSet, RenamePreviewInput, RenamePreviewInvariant,
    RenamePreviewProjection, RenamePreviewScenario, RenamePreviewValidationError,
    RENAME_GROUP_ORDER, RENAME_PREVIEW_ARTIFACT_REF, RENAME_PREVIEW_AS_OF, RENAME_PREVIEW_DOC_REF,
    RENAME_PREVIEW_FIXTURE_REF, RENAME_PREVIEW_FREEZE_GATE_REF, RENAME_PREVIEW_RECORD_KIND,
    RENAME_PREVIEW_SCHEMA_REF, RENAME_PREVIEW_SCHEMA_VERSION, RENAME_PREVIEW_SET_ID,
};

pub use relation_continuity::{
    build_relation_continuity_packet, relation_continuity_lines, relation_continuity_set,
    RelationContinuityCounts, RelationContinuityEvidenceClass, RelationContinuityInput,
    RelationContinuityInvariant, RelationContinuityLabel, RelationContinuityPacket,
    RelationContinuityProjection, RelationContinuityScenario, RelationContinuitySet,
    RelationContinuityValidationError, RelationNavEntryInput, RelationNavEntryKind,
    RelationNavigationEntry, RelationRecoveryChoice, RelationTargetSnapshot, RenamePreviewEvidence,
    RenamePreviewEvidenceInput, ReturnAnchor, RELATION_CONTINUITY_ARTIFACT_REF,
    RELATION_CONTINUITY_AS_OF, RELATION_CONTINUITY_DOC_REF, RELATION_CONTINUITY_DRIFT_STATES,
    RELATION_CONTINUITY_FIXTURE_REF, RELATION_CONTINUITY_FREEZE_GATE_REF,
    RELATION_CONTINUITY_RECORD_KIND, RELATION_CONTINUITY_SCHEMA_REF,
    RELATION_CONTINUITY_SCHEMA_VERSION, RELATION_CONTINUITY_SET_ID, RELATION_NAV_ENTRY_ORDER,
};

pub use relation_navigation_qualification::{
    certify, default_qualification_input, narrow_claim, relation_navigation_qualification,
    relation_navigation_qualification_lines, ClaimState, ClaimedSurface, FamilyProofPosture,
    ProofFreshness, ProofState, QualificationConsumer, QualificationConsumerProjection,
    RelationNavQualificationCertification, RelationNavQualificationFamily,
    RelationNavQualificationFamilyEntry, RelationNavQualificationInput,
    RelationNavQualificationInvariant, RelationNavQualificationRow,
    RelationNavQualificationValidationError, ReleaseEvidenceRow,
    RELATION_NAV_QUALIFICATION_ARTIFACT_REF, RELATION_NAV_QUALIFICATION_AS_OF,
    RELATION_NAV_QUALIFICATION_CERTIFICATION_ID, RELATION_NAV_QUALIFICATION_DOC_REF,
    RELATION_NAV_QUALIFICATION_FIXTURE_REF, RELATION_NAV_QUALIFICATION_FREEZE_GATE_REF,
    RELATION_NAV_QUALIFICATION_RECORD_KIND, RELATION_NAV_QUALIFICATION_SCHEMA_REF,
    RELATION_NAV_QUALIFICATION_SCHEMA_VERSION,
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
