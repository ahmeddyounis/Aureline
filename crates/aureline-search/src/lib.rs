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
//! - [`remap::DeepLinkRemapPacket`] records old/new target identity, active
//!   workset or slice, confidence, and recovery actions when a deep link,
//!   bookmark, or history row drifts.
//!
//! Higher layers (the shell `search_shell` module) convert this vocabulary
//! into chrome and persistable diagnostics; this crate only owns the
//! identity, ranking, and partiality truth.

#![doc(html_root_url = "https://docs.rs/aureline-search/0.0.0")]

pub mod counts;
pub mod docs_linking;
pub mod hot_set;
pub mod index_scheduler;
pub mod lexical;
pub mod planner;
pub mod query_session;
pub mod readiness;
pub mod remap;
pub mod result_id;
pub mod results;
pub mod scope;

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
pub use index_scheduler::{
    FirstUsefulNavigationSnapshot, IndexSchedulerAlpha, IndexSchedulerInputs, IndexSchedulerOutput,
    ScheduledQuickOpenSnapshot,
};
pub use lexical::{
    LexicalIndexInputs, LexicalIndexState, LexicalQuery, LexicalSearchResults, LexicalShell,
    LexicalShellSnapshot, MatchKind, ReadinessClass, ResultGroup, ResultRow, ScopeClass,
    SourceClass, MAX_RESULTS_PER_GROUP,
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

pub use results::{
    build_lexical_identity, derive_lexical_ranking_reasons, derive_partiality_class,
    project_lexical_partiality, RankingReasonClass, ResultIdentity, ResultPartialityClass,
};

pub use scope::{
    glob_matches_relative_path, ScopeFilterOutcome, ScopePatternKind, ScopePatternRecord,
    ScopePresentationState, WorkspaceSearchScope, WorkspaceSearchScopeMetadata,
};

pub use aureline_workspace::{GeneratedArtifactClass, LineageFreshnessClass, LineageHintRecord};
