//! Workspace search foundations.
//!
//! This crate is the canonical home for the workspace lexical-search shell
//! and the first hot-set indexing scheduler: the runtime path that backs
//! filename- and path-search rows in the live shell while richer graph lanes
//! warm in the background.
//!
//! The vocabulary here is intentionally narrow:
//!
//! - [`lexical::SourceClass`] names the lane that produced a row
//!   (filename vs. path) so downstream surfaces never imply semantic depth.
//! - [`lexical::ReadinessClass`] names whether the lane is ready,
//!   hot-set-ready, warming, partial, or unavailable — sourced from the
//!   upstream workspace lifecycle
//!   ([`aureline_workspace::WorkspaceReadinessInputs`]) and the readiness
//!   labels published by the reactive store
//!   ([`aureline_reactive_state::ReadinessLabel`]). Surfaces MUST surface the
//!   same token; they MUST NOT collapse `warming` and `partial` into a generic
//!   "loading" badge.
//! - [`hot_set::HotSetPlan`] records why a file or symbol is hot, which cold
//!   paths were deferred, and which fallback was used when hot inputs were not
//!   available.
//! - [`hybrid_retrieval::RetrievalInspectorPacket`] joins lexical, vector, and
//!   graph retrieval contributions with locality, readiness, embedder epoch,
//!   local-first fallback, and support/AI export projections.
//! - [`planner::SearchPlannerAlpha`] chooses lexical, structural, cached, or
//!   graph-backed paths for quick open, file search, symbol search, and docs
//!   search while preserving explicit fallback explanations.
//! - [`docs_linking::DocsLinkedSearchProjection`] binds symbols and commands
//!   to documentation anchors while carrying source, version, freshness,
//!   citation, and stale-example evidence on the same planned result IDs.
//! - [`readiness::IndexedLaneState`] projects warming, partial, cached, stale,
//!   and paused indexed-data states into one vocabulary for status chrome,
//!   result panes, graph/docs disclosures, and support artifacts.
//! - [`lexical::ScopeClass`] mirrors the
//!   [`aureline_workspace::ScopeClass`] so the search shell projects scope
//!   chips through the same vocabulary the workset surface uses.
//! - [`counts::SearchScopeCountsRecord`] distinguishes visible, loaded,
//!   all-matching, and hidden rows, and [`counts::ScopeCandidateTruthRecord`]
//!   carries the same scope truth onto graph-backed and AI context candidates.
//! - [`collections::CollectionViewAlphaRecord`] carries typed filters,
//!   saved-view refs, scope counters, stable selection state, and
//!   batch-review sheets for dense search, review, and package lanes.
//! - [`remap::DeepLinkRemapPacket`] records old/new target identity, active
//!   workset or slice, confidence, and recovery actions when a deep link,
//!   bookmark, or history row drifts.
//! - [`session_ledger::QuerySessionLedgerRecord`] records query sessions,
//!   saved-query privacy projections, and export-safe support/docs packets
//!   without cloning the planner or result identity model.
//! - [`aureline_navigation::target_model`] is the shared semantic target
//!   model for definitions, references, hierarchy edges, rename previews, and
//!   continuity refs used when search rows promote into navigation.
//!
//! Higher layers (the shell `search_shell` module) convert this vocabulary
//! into chrome and persistable diagnostics; this crate only owns the
//! identity, ranking, and partiality truth.

#![doc(html_root_url = "https://docs.rs/aureline-search/0.0.0")]

pub mod collection_portability_truth;
pub mod collections;
pub mod counts;
pub mod deep_link_navigation_truth;
pub mod docs_linking;
pub mod hot_set;
pub mod hybrid_retrieval;
pub mod index_scheduler;
pub mod infrastructure_intelligence;
pub mod lexical;
pub mod monorepo_hot_set_truth;
pub mod planner;
pub mod query_artifacts;
pub mod query_session;
pub mod quick_open_latency_truth;
pub mod ranking_reason;
pub mod readiness;
pub mod remap;
pub mod result_id;
pub mod result_truth_packet;
pub mod results;
pub mod scope;
pub mod search_benchmark_corpus_truth;
pub mod session_ledger;

pub use collection_portability_truth::{
    current_stable_collection_portability_truth_packet, CollectionPortabilityColumnPreset,
    CollectionPortabilityConsumerProjection, CollectionPortabilityConsumerSurface,
    CollectionPortabilityFindingKind, CollectionPortabilityFindingSeverity,
    CollectionPortabilityPromotionState, CollectionPortabilityReopenState,
    CollectionPortabilityRow, CollectionPortabilityTruthArtifactError,
    CollectionPortabilityTruthPacket, CollectionPortabilityTruthPacketInput,
    CollectionPortabilityTruthSupportExport, CollectionPortabilityValidationFinding,
    COLLECTION_PORTABILITY_TRUTH_ARTIFACT_DOC_REF, COLLECTION_PORTABILITY_TRUTH_DOC_REF,
    COLLECTION_PORTABILITY_TRUTH_FIXTURE_DIR, COLLECTION_PORTABILITY_TRUTH_PACKET_ARTIFACT_REF,
    COLLECTION_PORTABILITY_TRUTH_PACKET_RECORD_KIND, COLLECTION_PORTABILITY_TRUTH_SCHEMA_REF,
    COLLECTION_PORTABILITY_TRUTH_SCHEMA_VERSION,
    COLLECTION_PORTABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use collections::{
    consumed_scope_counts_schema_version, BatchActionClass, BatchAftermathSummary,
    BatchExecutionOriginClass, BatchMemberDisposition, BatchReviewMember, BatchReviewSheet,
    CollectionCountStatus, CollectionCountTerm, CollectionCountValue, CollectionFilterAst,
    CollectionFilterChip, CollectionFilterClause, CollectionFilterExpression,
    CollectionFilterLiteral, CollectionFilterOperator, CollectionFilterSourceClass,
    CollectionScopeCounters, CollectionSelectionState, CollectionSortKey, CollectionSurfaceFamily,
    CollectionValidationFinding, CollectionValidationFindingKind, CollectionViewAlphaRecord,
    FilterLiteralMaterialClass, FilterRoundTripState, SavedCollectionView,
    SavedViewFallbackBehavior, SavedViewOwnerScope, SavedViewPrivacyClass,
    SearchCollectionViewInputs, SelectionScopeClass, StableCollectionItemRef,
    BATCH_REVIEW_SHEET_RECORD_KIND, COLLECTION_FILTER_AST_RECORD_KIND,
    COLLECTION_SELECTION_STATE_RECORD_KIND, COLLECTION_VIEW_ALPHA_RECORD_KIND,
    COLLECTION_VIEW_ALPHA_SCHEMA_VERSION, FILTER_AST_ALPHA_SCHEMA_VERSION,
    SAVED_COLLECTION_VIEW_RECORD_KIND, SAVED_VIEW_ALPHA_SCHEMA_VERSION,
};
pub use deep_link_navigation_truth::{
    current_stable_deep_link_navigation_truth_packet, DeepLinkNavigationTruthArtifactError,
    DeepLinkNavigationTruthConsumerProjection, DeepLinkNavigationTruthConsumerSurface,
    DeepLinkNavigationTruthFindingKind, DeepLinkNavigationTruthFindingSeverity,
    DeepLinkNavigationTruthPacket, DeepLinkNavigationTruthPacketInput,
    DeepLinkNavigationTruthPromotionState, DeepLinkNavigationTruthRow,
    DeepLinkNavigationTruthSupportExport, DeepLinkNavigationTruthValidationFinding,
    DEEP_LINK_NAVIGATION_TRUTH_ARTIFACT_DOC_REF, DEEP_LINK_NAVIGATION_TRUTH_DOC_REF,
    DEEP_LINK_NAVIGATION_TRUTH_FIXTURE_DIR, DEEP_LINK_NAVIGATION_TRUTH_PACKET_ARTIFACT_REF,
    DEEP_LINK_NAVIGATION_TRUTH_PACKET_RECORD_KIND, DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_REF,
    DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_VERSION,
    DEEP_LINK_NAVIGATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use counts::{
    HiddenScopeDisclosure, HiddenScopeReason, ScopeCandidateTruthRecord, ScopeTruthLabel,
    ScopeTruthSurface, ScopeWarningKind, ScopeWarningRecord, SearchNoResultsState,
    SearchScopeCountsClass, SearchScopeCountsInputs, SearchScopeCountsRecord,
    SCOPE_TRUTH_COUNTS_SCHEMA_VERSION,
};
pub use docs_linking::{
    DocsCitationAvailability, DocsCitationDrawerHook, DocsDerivedReuseState, DocsDocKind,
    DocsEvidenceAction, DocsEvidenceActionKind, DocsEvidenceState, DocsExactAnchor,
    DocsFreshnessClass, DocsLinkResolutionClass, DocsLinkedReference, DocsLinkedSearchInputs,
    DocsLinkedSearchProjection, DocsLinkedSearchResult, DocsLinkingSupportExport,
    DocsLinkingSupportRow, DocsLinkingValidationFinding, DocsLocalityClass,
    DocsMissingAnchorDowngrade, DocsProjectVendorTruthCue, DocsPublishBoundaryState,
    DocsSourceClass, DocsStaleDetectionState, DocsStaleExampleSignal, DocsSubjectKind,
    DocsSuggestionCard, DocsSuggestionClass, DocsTriggerClass, DocsValidationFreshness,
    DocsVersionMatchState, DOCS_LINKING_ALPHA_SCHEMA_VERSION,
};
pub use hot_set::{
    HotSetCandidate, HotSetExplanation, HotSetFallback, HotSetFallbackReason, HotSetInputClass,
    HotSetPartialTruthCause, HotSetPlan, HotSetPlanEntry, HotSetPlanInputs, HotSetPlanner,
    HotSetResponsiveness, HotSetTarget, HotSetTargetKind, SearchReadinessState,
    DEFAULT_MAX_HOT_SET_TARGETS,
};
pub use hybrid_retrieval::{
    EmbeddingIndexManifest, EmbeddingIndexStateClass, LocalFirstPolicyDisclosure,
    RetrievalAnchorKind, RetrievalConsumerProjection, RetrievalConsumerSurface,
    RetrievalContribution, RetrievalContributionRole, RetrievalFallbackReasonClass,
    RetrievalFreshnessClass, RetrievalInspectorFinding, RetrievalInspectorFindingKind,
    RetrievalInspectorFindingSeverity, RetrievalInspectorPacket, RetrievalInspectorPacketInput,
    RetrievalInspectorRow, RetrievalInspectorSupportExport, RetrievalLaneClass,
    RetrievalLaneSnapshot, RetrievalLocalityClass, RetrievalPromotionState,
    RetrievalProvenanceAnchor, RetrievalReadinessClass, RetrievalReasonClass,
    RetrievalRoutePolicyClass, HYBRID_RETRIEVAL_BETA_DOC_REF, HYBRID_RETRIEVAL_BETA_FIXTURE_DIR,
    HYBRID_RETRIEVAL_BETA_PACKET_REF, RETRIEVAL_INSPECTOR_RECORD_KIND,
    RETRIEVAL_INSPECTOR_SCHEMA_REF, RETRIEVAL_INSPECTOR_SCHEMA_VERSION,
    RETRIEVAL_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND,
};
pub use index_scheduler::{
    FirstUsefulNavigationSnapshot, IndexSchedulerAlpha, IndexSchedulerInputs, IndexSchedulerOutput,
    ScheduledQuickOpenSnapshot,
};
pub use infrastructure_intelligence::{
    project_infrastructure_relationships_for_search, InfrastructureIntelligenceAlphaPage,
    InfrastructureSearchProjection, InfrastructureSearchResultRow,
};
pub use lexical::{
    LexicalIndexInputs, LexicalIndexState, LexicalQuery, LexicalSearchResults, LexicalShell,
    LexicalShellSnapshot, MatchKind, ReadinessClass, ResultGroup, ResultRow, ScopeClass,
    SourceClass, MAX_RESULTS_PER_GROUP,
};

pub use monorepo_hot_set_truth::{
    current_stable_monorepo_hot_set_truth_packet, GracefulDegradationClass,
    HotSetCoverageEstimate, IndexingLaneClass, MonorepoArchetypeClass,
    MonorepoConsumerProjection, MonorepoConsumerSurface, MonorepoHotSetTruthArtifactError,
    MonorepoHotSetTruthPacket, MonorepoHotSetTruthPacketInput, MonorepoHotSetTruthSupportExport,
    MonorepoTruthFindingKind, MonorepoTruthFindingSeverity, MonorepoTruthPromotionState,
    MonorepoTruthRow, MonorepoTruthValidationFinding, ResponsivenessInvariants,
    WarmingTransition, MONOREPO_HOT_SET_TRUTH_ARTIFACT_DOC_REF, MONOREPO_HOT_SET_TRUTH_DOC_REF,
    MONOREPO_HOT_SET_TRUTH_FIXTURE_DIR, MONOREPO_HOT_SET_TRUTH_PACKET_ARTIFACT_REF,
    MONOREPO_HOT_SET_TRUTH_PACKET_RECORD_KIND, MONOREPO_HOT_SET_TRUTH_SCHEMA_REF,
    MONOREPO_HOT_SET_TRUTH_SCHEMA_VERSION, MONOREPO_HOT_SET_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use planner::{
    PlannedResultSet, PlannedSearchResult, PlannerCandidate, PlannerContribution, PlannerDataPath,
    PlannerFreshnessClass, PlannerPassRecord, PlannerPathDecision, PlannerPathDecisionClass,
    PlannerPathReadiness, PlannerPathSnapshot, PlannerRankingReason, PlannerResultExplanation,
    PlannerResultTruthClass, PlannerTargetKind, PlannerUnavailableReason, SearchPlannerAlpha,
    SearchPlannerInputs, SearchPlannerOutput, SemanticFallbackState, SEARCH_PLANNER_ALPHA_VERSION,
};

pub use query_session::{
    stable_query_hash, QueryTextMode, SearchQuerySession, SearchSurface,
    SEARCH_QUERY_SESSION_SCHEMA_VERSION,
};

pub use quick_open_latency_truth::{
    current_stable_quick_open_latency_truth_packet, CertifiedArchetypeClass,
    LatencyBudget, LatencyConsumerProjection, LatencyConsumerSurface, LatencyFindingKind,
    LatencyFindingSeverity, LatencyObservation, LatencyPromotionState, LatencySurface,
    LatencyTruthRow, LatencyValidationFinding, PartialIndexTruthClass,
    QuickOpenLatencyTruthArtifactError, QuickOpenLatencyTruthPacket,
    QuickOpenLatencyTruthPacketInput, QuickOpenLatencyTruthSupportExport,
    SessionReadinessState, SessionReadinessTransition,
    QUICK_OPEN_LATENCY_TRUTH_ARTIFACT_DOC_REF, QUICK_OPEN_LATENCY_TRUTH_DOC_REF,
    QUICK_OPEN_LATENCY_TRUTH_FIXTURE_DIR, QUICK_OPEN_LATENCY_TRUTH_PACKET_ARTIFACT_REF,
    QUICK_OPEN_LATENCY_TRUTH_PACKET_RECORD_KIND, QUICK_OPEN_LATENCY_TRUTH_SCHEMA_REF,
    QUICK_OPEN_LATENCY_TRUTH_SCHEMA_VERSION,
    QUICK_OPEN_LATENCY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use query_artifacts::{
    QueryHistoryEntry, SavedQuery, ScopePackBinding, SearchArtifactMaterializationInput,
    SearchArtifactMigrationState, SearchArtifactSet, SearchArtifactValidationFinding,
    SearchArtifactValidationFindingKind, SearchCollectionSnapshot, SearchDeepLink,
    SearchRedactionProfile, SearchResultSemantics, SearchRetentionMode,
    SearchRetentionWideningBasis, SearchScopeHonestyState, SearchSyncClass,
    QUERY_HISTORY_ENTRY_RECORD_KIND, QUERY_HISTORY_SCHEMA_REF, SAVED_QUERY_EXPORT_PRIVACY_DOC_REF,
    SAVED_QUERY_PRIVACY_FIXTURE_DIR, SAVED_QUERY_RECORD_KIND, SAVED_QUERY_SCHEMA_REF,
    SCOPE_PACK_BINDING_RECORD_KIND, SEARCH_COLLECTION_SNAPSHOT_RECORD_KIND,
    SEARCH_DEEP_LINK_RECORD_KIND, SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF,
    SEARCH_QUERY_ARTIFACT_SCHEMA_VERSION,
};

pub use ranking_reason::{
    current_beta_search_operator_truth_packet, PartialIndexDrillPacket, PartialIndexDrillRow,
    PartialIndexDrillState, RankingReasonSignal, SearchOperatorConsumerSurface,
    SearchOperatorDowngradeState, SearchOperatorPromotionState, SearchOperatorTruthArtifactError,
    SearchOperatorTruthFinding, SearchOperatorTruthFindingKind, SearchOperatorTruthFindingSeverity,
    SearchOperatorTruthPacket, SearchOperatorTruthPacketInput, SearchOperatorTruthProjection,
    SearchOperatorTruthRow, SearchOperatorTruthSupportExport,
    PARTIAL_INDEX_DRILL_PACKET_RECORD_KIND, SEARCH_OPERATOR_TRUTH_DOC_REF,
    SEARCH_OPERATOR_TRUTH_FIXTURE_DIR, SEARCH_OPERATOR_TRUTH_PACKET_ARTIFACT_REF,
    SEARCH_OPERATOR_TRUTH_PACKET_RECORD_KIND, SEARCH_OPERATOR_TRUTH_SCHEMA_REF,
    SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION, SEARCH_OPERATOR_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use readiness::{
    IndexedLaneKind, IndexedLaneState, IndexedLaneStateInput, IndexedLaneSupportRow,
    IndexedStateClass, IndexedStateReason, IndexedStateSupportArtifact,
    INDEXED_LANE_STATE_SCHEMA_VERSION, INDEXED_STATE_SUPPORT_ARTIFACT_SCHEMA_VERSION,
};

pub use remap::{
    DeepLinkDriftState, DeepLinkRemapOutcome, DeepLinkRemapPacket, DeepLinkRemapPacketError,
    DeepLinkRemapRecordKind, RemapConfidence, RemapConfidenceClass, RemapEvidenceClass,
    RemapFailureReason, RemapScopePacket, RemapTarget, RemapTargetField, RemapTargetKind,
    DEEP_LINK_REMAP_PACKET_SCHEMA_VERSION,
};

pub use result_id::{
    build_lexical_result_id, build_planned_result_id, build_surface_result_id,
    normalize_result_id_part, StableResultKind, LEXICAL_RESULT_ID_SCHEME,
};

pub use result_truth_packet::{
    current_stable_search_result_truth_packet, ActionFallbackModeClass, CapturedVsLiveClass,
    ConfidenceClass, DedupeContributor, FactLabelClass, FreshnessClass, HistoryPolicyClass,
    RankingReason, RankingSignalClass, ResultKindClass, ScopeCounters, SearchActionBinding,
    SearchResultRef, SearchResultTruthArtifactError, SearchResultTruthConsumerProjection,
    SearchResultTruthConsumerSurface, SearchResultTruthFindingKind,
    SearchResultTruthFindingSeverity, SearchResultTruthPacket, SearchResultTruthPacketInput,
    SearchResultTruthPacketSupportExport, SearchResultTruthPromotionState, SearchResultTruthRow,
    SearchResultTruthValidationFinding, SourceStratumClass, TieBreakClass,
    SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_DOC_REF, SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_REF,
    SEARCH_RESULT_TRUTH_PACKET_DOC_REF, SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR,
    SEARCH_RESULT_TRUTH_PACKET_RECORD_KIND, SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
    SEARCH_RESULT_TRUTH_PACKET_SCHEMA_VERSION,
    SEARCH_RESULT_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND,
};

pub use results::{
    build_lexical_identity, derive_lexical_ranking_reasons, derive_partiality_class,
    project_lexical_partiality, RankingReasonClass, ResultIdentity, ResultPartialityClass,
};

pub use scope::{
    glob_matches_relative_path, ScopeFilterOutcome, ScopePatternKind, ScopePatternRecord,
    ScopePresentationState, WorkspaceSearchScope, WorkspaceSearchScopeMetadata,
};

pub use search_benchmark_corpus_truth::{
    current_stable_search_benchmark_corpus_truth_packet, BenchmarkCorpusClass,
    BenchmarkCorpusDefinition, CertifiedArchetypePackRow, CorpusConfidenceClass,
    CorpusConsumerProjection, CorpusConsumerSurface, CorpusFindingKind, CorpusFindingSeverity,
    CorpusPromotionState, CorpusValidationFinding, EvaluationDowngradeState, ProvenanceClass,
    QueryPackClass, RankingMetricClass, RankingMetricObservation, RetentionPolicyClass,
    SearchBenchmarkCorpusTruthArtifactError, SearchBenchmarkCorpusTruthPacket,
    SearchBenchmarkCorpusTruthPacketInput, SearchBenchmarkCorpusTruthSupportExport,
    SEARCH_BENCHMARK_CORPUS_TRUTH_ARTIFACT_DOC_REF, SEARCH_BENCHMARK_CORPUS_TRUTH_DOC_REF,
    SEARCH_BENCHMARK_CORPUS_TRUTH_FIXTURE_DIR,
    SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_ARTIFACT_REF,
    SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_RECORD_KIND, SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_REF,
    SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_VERSION,
    SEARCH_BENCHMARK_CORPUS_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use session_ledger::{
    QueryMaterialDisposition, QuerySessionLedgerEntry, QuerySessionLedgerRecord,
    SavedQueryDenialReason, SavedQueryPrivacyClass, SavedQueryRecord, SavedQueryRecordInputs,
    SavedQueryReopenContext, SavedQueryReopenProjection, SavedQueryReopenState,
    SavedQuerySharePolicy, SavedQuerySourceClass, SavedQueryValidationFinding,
    SavedQueryValidationFindingKind, SearchExportDestination, SearchExportError,
    SearchExportPacket, SearchExportPacketInputs, SearchPacketCountSummary,
    SearchPacketRedactionState, SAVED_QUERY_ALPHA_SCHEMA_VERSION,
};

pub use aureline_navigation::target_model as navigation_target_model;
pub use aureline_workspace::{GeneratedArtifactClass, LineageFreshnessClass, LineageHintRecord};
