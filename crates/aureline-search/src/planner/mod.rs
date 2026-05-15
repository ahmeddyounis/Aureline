//! Shared lexical, structural, cached, and graph-backed search planner.
//!
//! The alpha planner is the first common contract for quick open, file
//! search, and symbol search. It chooses the usable data path for each
//! surface, records unavailable higher-authority paths, fuses duplicate rows
//! by canonical target, and emits a `Why this result?` explanation without
//! claiming graph or language certainty when those lanes are still warming.

use std::collections::{BTreeMap, BTreeSet};

use aureline_docs::{
    CitationAnchorAvailability, DocsFreshnessClass as DocsIndexFreshnessClass,
    DocsSearchIndexEntry, DocsSearchQueryResult, VersionMatchState as DocsIndexVersionMatchState,
};
use serde::{Deserialize, Serialize};

use crate::counts::ScopeCandidateTruthRecord;
use crate::lexical::{LexicalSearchResults, ReadinessClass};
use crate::query_session::{SearchQuerySession, SearchSurface};
use crate::result_id::build_planned_result_id;
use crate::results::RankingReasonClass;

/// Planner version recorded in alpha planner passes.
pub const SEARCH_PLANNER_ALPHA_VERSION: &str = "search-planner-alpha";

const QUICK_OPEN_PRIORITY: [PlannerDataPath; 4] = [
    PlannerDataPath::Cached,
    PlannerDataPath::Lexical,
    PlannerDataPath::Structural,
    PlannerDataPath::GraphBacked,
];
const FILE_SEARCH_PRIORITY: [PlannerDataPath; 2] =
    [PlannerDataPath::Lexical, PlannerDataPath::Cached];
const SYMBOL_SEARCH_PRIORITY: [PlannerDataPath; 4] = [
    PlannerDataPath::GraphBacked,
    PlannerDataPath::Structural,
    PlannerDataPath::Cached,
    PlannerDataPath::Lexical,
];
const DOCS_SEARCH_PRIORITY: [PlannerDataPath; 4] = [
    PlannerDataPath::Docs,
    PlannerDataPath::Cached,
    PlannerDataPath::GraphBacked,
    PlannerDataPath::Lexical,
];

/// Data path that can answer a planner-backed search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerDataPath {
    /// Filename, path, or text rows from the lexical index.
    Lexical,
    /// Syntax, outline, or file-local symbol rows from structural analysis.
    Structural,
    /// Recent, hot, or persisted rows served from a cache.
    Cached,
    /// Semantic graph rows with graph/provider evidence.
    GraphBacked,
    /// Documentation, help, citation-anchor, or docs-pack rows.
    Docs,
}

impl PlannerDataPath {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lexical => "lexical",
            Self::Structural => "structural",
            Self::Cached => "cached",
            Self::GraphBacked => "graph_backed",
            Self::Docs => "docs",
        }
    }

    /// Short human-readable badge for row/source attribution.
    pub const fn badge(self) -> &'static str {
        match self {
            Self::Lexical => "Lexical",
            Self::Structural => "Structural",
            Self::Cached => "Cached",
            Self::GraphBacked => "Graph",
            Self::Docs => "Docs",
        }
    }
}

/// Readiness state for one planner data path or fused result set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerPathReadiness {
    /// The path is fully ready for the declared scope.
    Ready,
    /// Hot-set rows are ready while broader indexing continues.
    HotSetReady,
    /// The path has rows, but declared-scope coverage is incomplete.
    Partial,
    /// The path is warming and may have limited rows.
    Warming,
    /// The path is serving stale or cached rows with disclosure.
    Stale,
    /// The path cannot currently answer.
    Unavailable,
    /// The path exists but is outside the active workset or policy scope.
    OutOfScope,
}

impl PlannerPathReadiness {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::HotSetReady => "hot_set_ready",
            Self::Partial => "partial",
            Self::Warming => "warming",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// Projects lexical readiness into planner readiness.
    pub const fn from_readiness_class(readiness: ReadinessClass) -> Self {
        match readiness {
            ReadinessClass::Ready => Self::Ready,
            ReadinessClass::HotSetReady => Self::HotSetReady,
            ReadinessClass::Warming => Self::Warming,
            ReadinessClass::Partial => Self::Partial,
            ReadinessClass::Stale => Self::Stale,
            ReadinessClass::Unavailable => Self::Unavailable,
            ReadinessClass::OutOfScope => Self::OutOfScope,
        }
    }

    const fn is_unavailable_like(self) -> bool {
        matches!(self, Self::Unavailable | Self::OutOfScope)
    }
}

/// Freshness class attached to one planner data-path snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerFreshnessClass {
    /// Snapshot was captured from live local state.
    AuthoritativeLive,
    /// Snapshot is warm cached state within the accepted freshness window.
    WarmCached,
    /// Snapshot is stale but still useful with disclosure.
    StaleCached,
    /// Snapshot was imported from a mirrored or provider-owned source.
    Imported,
    /// Freshness is unknown and must be treated as degraded.
    Unknown,
}

impl PlannerFreshnessClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::StaleCached => "stale_cached",
            Self::Imported => "imported",
            Self::Unknown => "unknown",
        }
    }
}

/// Reason a higher-authority data path could not answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerUnavailableReason {
    /// Graph index is warming and cannot prove the result yet.
    GraphWarming,
    /// Graph index or provider is unavailable.
    GraphUnavailable,
    /// Structural or language provider is unavailable.
    LanguageUnavailable,
    /// Lexical index is unavailable.
    LexicalIndexUnavailable,
    /// Cache has no rows for this query or scope.
    CacheMiss,
    /// Policy or trust posture blocked this path.
    PolicyLimited,
    /// Active workset or sparse scope excludes this path.
    OutsideScope,
    /// Documentation or help index cannot currently answer.
    DocsUnavailable,
}

impl PlannerUnavailableReason {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphWarming => "graph_warming",
            Self::GraphUnavailable => "graph_unavailable",
            Self::LanguageUnavailable => "language_unavailable",
            Self::LexicalIndexUnavailable => "lexical_index_unavailable",
            Self::CacheMiss => "cache_miss",
            Self::PolicyLimited => "policy_limited",
            Self::OutsideScope => "outside_scope",
            Self::DocsUnavailable => "docs_unavailable",
        }
    }
}

/// Target shape surfaced by the planner result set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerTargetKind {
    /// File or path target.
    File,
    /// Text match inside a file.
    TextMatch,
    /// Symbol, route, type, member, or structural target.
    Symbol,
    /// Documentation, help, or citation-anchor target.
    DocsAnchor,
}

impl PlannerTargetKind {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::TextMatch => "text_match",
            Self::Symbol => "symbol",
            Self::DocsAnchor => "docs_anchor",
        }
    }
}

/// Ranking reason emitted by one planner data path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerRankingReason {
    /// Target name exactly matched the query.
    ExactNameMatch,
    /// Lexical basename or title matched exactly.
    LexicalExactMatch,
    /// Lexical basename or title matched by prefix.
    LexicalPrefixMatch,
    /// Lexical basename or title matched by substring.
    LexicalSubstringMatch,
    /// Lexical path matched when the basename did not.
    LexicalPathMatch,
    /// Text content matched the query.
    TextMatch,
    /// Recent file or recent place signal boosted the row.
    RecentFileBias,
    /// Recent edit signal boosted the row.
    RecentEditBias,
    /// Hot-set selection boosted the row.
    HotSetBias,
    /// Structural symbol/name match answered the query.
    StructuralSymbolMatch,
    /// Structural fallback answered while graph proof is pending.
    StructuralFallback,
    /// Symbol kind prior boosted the row.
    SymbolKindPrior,
    /// Graph provided an exact symbol/entity answer.
    GraphExactSymbol,
    /// Graph neighborhood or relation boosted the row.
    GraphNeighbourhoodHop,
    /// Cached snapshot answered the query.
    CachedSnapshotHit,
    /// Result carries a generated-artifact caveat.
    GeneratedArtifactDeprioritized,
    /// Result was returned from a partial or warming source.
    PartialIndex,
    /// Graph path was unavailable and fallback was used.
    GraphUnavailable,
    /// Language or structural path was unavailable and fallback was used.
    LanguageUnavailable,
    /// Documentation anchor matched the query.
    DocsAnchorMatch,
    /// Documentation row was bound to the requested symbol or command.
    DocsSymbolLinkedReference,
    /// Project documentation precedence affected the row order.
    DocsSourcePrecedence,
    /// Citation anchors are available for inspection.
    CitationAvailable,
    /// Citation anchors are missing and must be disclosed.
    CitationMissing,
    /// A stale example or snippet signal affected the row.
    StaleExampleSignal,
}

impl PlannerRankingReason {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactNameMatch => "exact_name_match",
            Self::LexicalExactMatch => "lexical_exact_match",
            Self::LexicalPrefixMatch => "lexical_prefix_match",
            Self::LexicalSubstringMatch => "lexical_substring_match",
            Self::LexicalPathMatch => "lexical_path_match",
            Self::TextMatch => "text_match",
            Self::RecentFileBias => "recent_file_bias",
            Self::RecentEditBias => "recent_edit_bias",
            Self::HotSetBias => "hot_set_bias",
            Self::StructuralSymbolMatch => "structural_symbol_match",
            Self::StructuralFallback => "structural_fallback",
            Self::SymbolKindPrior => "symbol_kind_prior",
            Self::GraphExactSymbol => "graph_exact_symbol",
            Self::GraphNeighbourhoodHop => "graph_neighbourhood_hop",
            Self::CachedSnapshotHit => "cached_snapshot_hit",
            Self::GeneratedArtifactDeprioritized => "generated_artifact_deprioritized",
            Self::PartialIndex => "partial_index",
            Self::GraphUnavailable => "graph_unavailable",
            Self::LanguageUnavailable => "language_unavailable",
            Self::DocsAnchorMatch => "docs_anchor_match",
            Self::DocsSymbolLinkedReference => "docs_symbol_linked_reference",
            Self::DocsSourcePrecedence => "docs_source_precedence",
            Self::CitationAvailable => "citation_available",
            Self::CitationMissing => "citation_missing",
            Self::StaleExampleSignal => "stale_example_signal",
        }
    }

    const fn rank(self) -> u8 {
        match self {
            Self::ExactNameMatch | Self::LexicalExactMatch | Self::GraphExactSymbol => 0,
            Self::LexicalPrefixMatch | Self::StructuralSymbolMatch => 1,
            Self::LexicalSubstringMatch | Self::TextMatch => 2,
            Self::LexicalPathMatch | Self::GraphNeighbourhoodHop => 3,
            Self::RecentEditBias | Self::RecentFileBias | Self::HotSetBias => 4,
            Self::StructuralFallback | Self::CachedSnapshotHit | Self::SymbolKindPrior => 5,
            Self::GeneratedArtifactDeprioritized | Self::PartialIndex => 6,
            Self::GraphUnavailable | Self::LanguageUnavailable => 7,
            Self::DocsAnchorMatch => 0,
            Self::DocsSymbolLinkedReference => 1,
            Self::DocsSourcePrecedence | Self::CitationAvailable => 4,
            Self::CitationMissing | Self::StaleExampleSignal => 6,
        }
    }
}

/// Truth class for one fused planner result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerResultTruthClass {
    /// Exact local result from a ready authoritative path.
    Exact,
    /// Imported or docs-pack fact with source and freshness disclosure.
    Imported,
    /// Useful but not project-wide authoritative result.
    Heuristic,
    /// Cached result served with freshness disclosure.
    Cached,
    /// Graph-backed result served from semantic graph evidence.
    GraphBacked,
    /// Fused result with multiple contributing path classes.
    Hybrid,
}

impl PlannerResultTruthClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
            Self::Cached => "cached",
            Self::GraphBacked => "graph_backed",
            Self::Hybrid => "hybrid",
        }
    }
}

/// Planner decision for one data-path snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlannerPathDecisionClass {
    /// Snapshot is the primary selected answering path.
    SelectedPrimary,
    /// Snapshot answers because a higher-authority path is unavailable.
    SelectedFallback,
    /// Snapshot contributes evidence behind an already-selected primary row.
    SelectedSupplementary,
    /// Snapshot could not answer and that fact must be disclosed.
    UnavailableDisclosed,
    /// Snapshot was eligible but produced no rows.
    SkippedNoRows,
    /// Snapshot does not belong to the active search surface.
    SkippedSurfaceIneligible,
}

impl PlannerPathDecisionClass {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedPrimary => "selected_primary",
            Self::SelectedFallback => "selected_fallback",
            Self::SelectedSupplementary => "selected_supplementary",
            Self::UnavailableDisclosed => "unavailable_disclosed",
            Self::SkippedNoRows => "skipped_no_rows",
            Self::SkippedSurfaceIneligible => "skipped_surface_ineligible",
        }
    }

    /// True when this decision contributes rows to the result set.
    pub const fn is_selected(self) -> bool {
        matches!(
            self,
            Self::SelectedPrimary | Self::SelectedFallback | Self::SelectedSupplementary
        )
    }

    const fn rank(self) -> u8 {
        match self {
            Self::SelectedPrimary => 0,
            Self::SelectedFallback => 1,
            Self::SelectedSupplementary => 2,
            Self::UnavailableDisclosed => 3,
            Self::SkippedNoRows => 4,
            Self::SkippedSurfaceIneligible => 5,
        }
    }
}

/// Fallback state recorded on each planner pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticFallbackState {
    /// Semantic graph was not relevant for the active surface/pass.
    SemanticNotApplicable,
    /// Graph-backed data answered the selected result set.
    GraphBacked,
    /// Graph was unavailable and structural fallback answered.
    GraphUnavailableStructuralFallback,
    /// Graph was unavailable and lexical fallback answered.
    GraphUnavailableLexicalOnly,
    /// Graph was unavailable and cached fallback answered.
    GraphUnavailableCachedOnly,
    /// Language/structural data was unavailable and lexical fallback answered.
    LanguageUnavailableLexicalOnly,
    /// Lexical index was unavailable and cached fallback answered.
    CachedFallbackLexicalUnavailable,
    /// No data path could answer.
    NoUsablePath,
}

impl SemanticFallbackState {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticNotApplicable => "semantic_not_applicable",
            Self::GraphBacked => "graph_backed",
            Self::GraphUnavailableStructuralFallback => "graph_unavailable_structural_fallback",
            Self::GraphUnavailableLexicalOnly => "graph_unavailable_lexical_only",
            Self::GraphUnavailableCachedOnly => "graph_unavailable_cached_only",
            Self::LanguageUnavailableLexicalOnly => "language_unavailable_lexical_only",
            Self::CachedFallbackLexicalUnavailable => "cached_fallback_lexical_unavailable",
            Self::NoUsablePath => "no_usable_path",
        }
    }
}

/// One candidate row from a planner data-path snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerCandidate {
    /// Stable row id inside the source data-path snapshot.
    pub candidate_id: String,
    /// Canonical target id used to fuse rows from multiple data paths.
    pub canonical_id: String,
    /// Target class represented by this candidate.
    pub target_kind: PlannerTargetKind,
    /// Primary display title for row/debug snapshots.
    pub title: String,
    /// Workspace-relative path, when the row points into a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    /// Stable symbol ref, when the row points at a symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_ref: Option<String>,
    /// Ordered ranking reasons contributed by this candidate.
    #[serde(default)]
    pub ranking_reasons: Vec<PlannerRankingReason>,
    /// Candidate-specific partial-truth causes.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Scope truth reused by graph-backed and AI context candidate surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_truth: Option<ScopeCandidateTruthRecord>,
}

impl PlannerCandidate {
    /// Builds a file candidate with a canonical workspace-file id.
    pub fn file(
        candidate_id: impl Into<String>,
        relative_path: impl Into<String>,
        ranking_reasons: Vec<PlannerRankingReason>,
    ) -> Self {
        let relative_path = relative_path.into();
        Self {
            candidate_id: candidate_id.into(),
            canonical_id: format!("workspace:file:{relative_path}"),
            target_kind: PlannerTargetKind::File,
            title: title_for_path(&relative_path),
            relative_path: Some(relative_path),
            symbol_ref: None,
            ranking_reasons,
            partial_truth_causes: Vec::new(),
            scope_truth: None,
        }
    }

    /// Builds a symbol candidate with a caller-provided canonical id.
    pub fn symbol(
        candidate_id: impl Into<String>,
        canonical_id: impl Into<String>,
        title: impl Into<String>,
        relative_path: impl Into<String>,
        symbol_ref: impl Into<String>,
        ranking_reasons: Vec<PlannerRankingReason>,
    ) -> Self {
        Self {
            candidate_id: candidate_id.into(),
            canonical_id: canonical_id.into(),
            target_kind: PlannerTargetKind::Symbol,
            title: title.into(),
            relative_path: Some(relative_path.into()),
            symbol_ref: Some(symbol_ref.into()),
            ranking_reasons,
            partial_truth_causes: Vec::new(),
            scope_truth: None,
        }
    }

    /// Attach shared scope truth to this candidate.
    pub fn with_scope_truth(mut self, scope_truth: ScopeCandidateTruthRecord) -> Self {
        self.scope_truth = Some(scope_truth);
        self
    }
}

/// Snapshot of one data path considered by the planner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerPathSnapshot {
    /// Data path represented by this snapshot.
    pub path_kind: PlannerDataPath,
    /// Stable snapshot id for support/export joins.
    pub snapshot_id: String,
    /// Readiness state for this path at capture time.
    pub readiness: PlannerPathReadiness,
    /// Freshness class for this path at capture time.
    pub freshness: PlannerFreshnessClass,
    /// Index epoch used by this path, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_epoch: Option<String>,
    /// Graph epoch used by this path, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Reason this path could not answer, when degraded or unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<PlannerUnavailableReason>,
    /// Path-level partial-truth causes.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Candidate rows this path produced for the query.
    #[serde(default)]
    pub rows: Vec<PlannerCandidate>,
}

impl PlannerPathSnapshot {
    /// Builds a lexical planner snapshot from existing lexical search results.
    pub fn from_lexical_results(
        snapshot_id: impl Into<String>,
        freshness: PlannerFreshnessClass,
        results: &LexicalSearchResults,
    ) -> Self {
        let rows = results
            .groups
            .iter()
            .flat_map(|group| group.items.iter())
            .map(|row| {
                let relative_path = row.relative_path.clone();
                PlannerCandidate {
                    candidate_id: row.identity.result_id.clone(),
                    canonical_id: format!("workspace:file:{relative_path}"),
                    target_kind: PlannerTargetKind::File,
                    title: title_for_path(&relative_path),
                    relative_path: Some(relative_path),
                    symbol_ref: None,
                    ranking_reasons: row
                        .identity
                        .ranking_reasons
                        .iter()
                        .map(|reason| map_lexical_reason(*reason))
                        .collect(),
                    partial_truth_causes: Vec::new(),
                    scope_truth: None,
                }
            })
            .collect();

        Self {
            path_kind: PlannerDataPath::Lexical,
            snapshot_id: snapshot_id.into(),
            readiness: PlannerPathReadiness::from_readiness_class(results.readiness),
            freshness,
            index_epoch: None,
            graph_epoch: None,
            unavailable_reason: None,
            partial_truth_causes: results.partial_truth_causes.clone(),
            rows,
        }
    }

    /// Builds a graph-backed planner snapshot from an alpha graph query envelope.
    pub fn from_graph_query_envelope(
        snapshot_id: impl Into<String>,
        envelope: &aureline_graph::GraphQueryEnvelope,
    ) -> Self {
        let rows = envelope
            .rows
            .iter()
            .filter_map(candidate_from_graph_row)
            .collect();

        Self {
            path_kind: PlannerDataPath::GraphBacked,
            snapshot_id: snapshot_id.into(),
            readiness: readiness_from_graph_envelope(envelope),
            freshness: freshness_from_graph_envelope(envelope),
            index_epoch: None,
            graph_epoch: Some(envelope.workspace_graph_id.clone()),
            unavailable_reason: match envelope.readiness {
                aureline_graph::GraphQueryReadiness::Unavailable => {
                    Some(PlannerUnavailableReason::GraphUnavailable)
                }
                aureline_graph::GraphQueryReadiness::Warming if envelope.rows.is_empty() => {
                    Some(PlannerUnavailableReason::GraphWarming)
                }
                aureline_graph::GraphQueryReadiness::OutOfScope => {
                    Some(PlannerUnavailableReason::OutsideScope)
                }
                _ => None,
            },
            partial_truth_causes: envelope
                .partial_truth_causes
                .iter()
                .map(|cause| cause.as_str().to_string())
                .collect(),
            rows,
        }
    }

    /// Builds a structural planner snapshot from a language symbol snapshot.
    pub fn from_symbol_snapshot(
        snapshot_id: impl Into<String>,
        snapshot: &aureline_language::SymbolSnapshotRecord,
    ) -> Self {
        let rows = snapshot
            .symbols
            .iter()
            .map(candidate_from_symbol_record)
            .collect();
        let readiness = readiness_from_symbol_snapshot(snapshot);

        Self {
            path_kind: PlannerDataPath::Structural,
            snapshot_id: snapshot_id.into(),
            readiness,
            freshness: freshness_from_symbol_snapshot(snapshot),
            index_epoch: Some(snapshot.state.symbol_epoch_ref.clone()),
            graph_epoch: None,
            unavailable_reason: if snapshot.state.search_consumable {
                None
            } else {
                Some(PlannerUnavailableReason::LanguageUnavailable)
            },
            partial_truth_causes: snapshot.state.partial_truth_causes.clone(),
            rows,
        }
    }

    /// Builds a docs planner snapshot from a docs-pack search index query.
    pub fn from_docs_search_query_result(
        snapshot_id: impl Into<String>,
        query_result: &DocsSearchQueryResult,
    ) -> Self {
        let rows = query_result
            .entries
            .iter()
            .map(candidate_from_docs_search_entry)
            .collect();

        Self {
            path_kind: PlannerDataPath::Docs,
            snapshot_id: snapshot_id.into(),
            readiness: if query_result.partial_index {
                PlannerPathReadiness::Partial
            } else {
                PlannerPathReadiness::Ready
            },
            freshness: freshness_from_docs_search_entries(&query_result.entries),
            index_epoch: Some(query_result.index_epoch.clone()),
            graph_epoch: None,
            unavailable_reason: None,
            partial_truth_causes: query_result.partial_truth_causes.clone(),
            rows,
        }
    }

    fn can_answer(&self) -> bool {
        !self.rows.is_empty()
            && !self.readiness.is_unavailable_like()
            && self.unavailable_reason.is_none()
    }

    fn is_disclosed_unavailable(&self) -> bool {
        self.readiness.is_unavailable_like()
            || self.unavailable_reason.is_some()
            || (self.rows.is_empty() && self.readiness == PlannerPathReadiness::Warming)
    }
}

/// Inputs required to run one shared planner pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPlannerInputs {
    /// Query session that opened this planner pass.
    pub query_session: SearchQuerySession,
    /// Stable planner pass id.
    pub planner_pass_id: String,
    /// Stable result set id minted for this pass.
    pub result_set_id: String,
    /// Planner version to record on the pass and session.
    #[serde(default = "default_planner_version")]
    pub planner_version: String,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
    /// Data-path snapshots considered by the planner.
    #[serde(default)]
    pub path_snapshots: Vec<PlannerPathSnapshot>,
}

/// Output of one shared planner pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPlannerOutput {
    /// Query session with the selected readiness and epochs projected onto it.
    pub query_session: SearchQuerySession,
    /// Explainable planner-pass record.
    pub planner_pass: PlannerPassRecord,
    /// Fused result set produced by the planner pass.
    pub result_set: PlannedResultSet,
}

/// Explainable planner-pass record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerPassRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable planner-pass identity.
    pub planner_pass_id: String,
    /// Query-session id this pass answered.
    pub query_session_id_ref: String,
    /// Result-set id minted by this pass.
    pub result_set_id: String,
    /// Planner version used for this pass.
    pub planner_version: String,
    /// Surface family answered by this pass.
    pub surface: SearchSurface,
    /// Readiness state of the fused result set.
    pub readiness_state: PlannerPathReadiness,
    /// Fallback state for graph/semantic/language lanes.
    pub semantic_fallback_state: SemanticFallbackState,
    /// Data-path decisions in surface-priority order.
    pub path_decisions: Vec<PlannerPathDecision>,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
}

impl PlannerPassRecord {
    /// Stable record-kind tag carried in serialized planner passes.
    pub const RECORD_KIND: &'static str = "search_planner_pass";
}

/// Decision emitted for one data-path snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerPathDecision {
    /// Data path considered by this decision.
    pub path_kind: PlannerDataPath,
    /// Snapshot id considered by this decision.
    pub snapshot_id: String,
    /// Decision class selected for this path.
    pub decision_class: PlannerPathDecisionClass,
    /// Readiness state observed for this path.
    pub readiness: PlannerPathReadiness,
    /// Freshness class observed for this path.
    pub freshness: PlannerFreshnessClass,
    /// Reason this path could not answer, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<PlannerUnavailableReason>,
    /// Number of candidate rows available on this path.
    pub candidate_count: usize,
    /// Path-level partial-truth causes.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
}

/// Result-set record emitted by one planner pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedResultSet {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable result-set identity.
    pub result_set_id: String,
    /// Query-session id this result set belongs to.
    pub query_session_id_ref: String,
    /// Planner-pass id that produced this result set.
    pub planner_pass_id_ref: String,
    /// Surface family answered by this result set.
    pub surface: SearchSurface,
    /// Readiness state of the fused result set.
    pub readiness_state: PlannerPathReadiness,
    /// Fused rows in stable rank order.
    pub rows: Vec<PlannedSearchResult>,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
}

impl PlannedResultSet {
    /// Stable record-kind tag carried in serialized result sets.
    pub const RECORD_KIND: &'static str = "planned_search_result_set";
}

/// One fused planner result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedSearchResult {
    /// Stable planner result id.
    pub result_id: String,
    /// Canonical target id used for dedupe/fusion.
    pub canonical_id: String,
    /// Target kind represented by this row.
    pub target_kind: PlannerTargetKind,
    /// Primary row title.
    pub title: String,
    /// Workspace-relative path, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    /// Stable symbol ref, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_ref: Option<String>,
    /// Data path that answered the row.
    pub answered_by: PlannerDataPath,
    /// Decision role of the answering path.
    pub answer_role: PlannerPathDecisionClass,
    /// Truth class for this fused row.
    pub truth_class: PlannerResultTruthClass,
    /// Readiness state attached to the answering path.
    pub readiness_state: PlannerPathReadiness,
    /// Ordered ranking reasons visible in debug/support views.
    pub ranking_reasons: Vec<PlannerRankingReason>,
    /// Partial-truth causes attached to this row.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Shared scope truth for this fused row, when a contributing candidate
    /// crossed or disclosed a scope boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_truth: Option<ScopeCandidateTruthRecord>,
    /// Per-path contributions that built this row.
    pub contributions: Vec<PlannerContribution>,
    /// Human-readable explanation packet for `Why this result?`.
    pub explanation: PlannerResultExplanation,
}

impl PlannedSearchResult {
    /// Returns stable ranking-reason tokens.
    pub fn ranking_reason_tokens(&self) -> Vec<&'static str> {
        self.ranking_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect()
    }

    /// Returns stable partial-truth cause tokens.
    pub fn partial_truth_cause_tokens(&self) -> Vec<&str> {
        self.partial_truth_causes
            .iter()
            .map(String::as_str)
            .collect()
    }
}

/// Contribution from one data path to a fused result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerContribution {
    /// Data path that contributed this row.
    pub path_kind: PlannerDataPath,
    /// Source snapshot id for this contribution.
    pub snapshot_id: String,
    /// Source candidate id inside the snapshot.
    pub candidate_id: String,
    /// Decision role assigned to the contributing path.
    pub role: PlannerPathDecisionClass,
    /// Readiness state for the contributing path.
    pub readiness: PlannerPathReadiness,
    /// Ranking reasons contributed by this path.
    pub ranking_reasons: Vec<PlannerRankingReason>,
    /// Partial-truth causes contributed by this path.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Shared scope truth contributed by this path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_truth: Option<ScopeCandidateTruthRecord>,
}

/// Explanation packet for one planner result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerResultExplanation {
    /// Short summary suitable for a support or debug detail sheet.
    pub summary: String,
    /// Data path that answered the row.
    pub selected_path: PlannerDataPath,
    /// True when graph or language readiness forced a lower-authority path.
    pub degraded_by_missing_graph_or_language: bool,
    /// Fallback reason when the selected path is not the highest authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<PlannerUnavailableReason>,
}

/// Stateless alpha search planner.
#[derive(Debug, Default, Clone, Copy)]
pub struct SearchPlannerAlpha;

impl SearchPlannerAlpha {
    /// Runs one planner pass and returns the updated query session plus result set.
    pub fn plan(inputs: SearchPlannerInputs) -> SearchPlannerOutput {
        let decisions = build_path_decisions(inputs.query_session.surface, &inputs.path_snapshots);
        let mut result_set = build_result_set(&inputs, &decisions);
        let readiness_state = readiness_for_result_set(&result_set.rows, &decisions);
        result_set.readiness_state = readiness_state;
        let semantic_fallback_state = semantic_fallback_state(&result_set.rows, &decisions);

        let mut query_session = inputs.query_session.clone();
        query_session.planner_version = inputs.planner_version.clone();
        query_session.readiness_state = readiness_state.as_str().to_string();
        query_session.index_epoch = first_selected_index_epoch(&inputs.path_snapshots, &decisions);
        query_session.graph_epoch = first_selected_graph_epoch(&inputs.path_snapshots, &decisions);

        let planner_pass = PlannerPassRecord {
            record_kind: PlannerPassRecord::RECORD_KIND.to_string(),
            schema_version: 1,
            planner_pass_id: inputs.planner_pass_id,
            query_session_id_ref: query_session.query_session_id.clone(),
            result_set_id: inputs.result_set_id,
            planner_version: inputs.planner_version,
            surface: query_session.surface,
            readiness_state,
            semantic_fallback_state,
            path_decisions: decisions,
            observed_at: inputs.observed_at,
        };

        SearchPlannerOutput {
            query_session,
            planner_pass,
            result_set,
        }
    }
}

#[derive(Debug)]
struct PlannedSearchResultAccumulator {
    result: PlannedSearchResult,
    contribution_paths: BTreeSet<PlannerDataPath>,
}

impl PlannedSearchResultAccumulator {
    fn add_contribution(
        &mut self,
        snapshot: &PlannerPathSnapshot,
        candidate: &PlannerCandidate,
        decision: &PlannerPathDecision,
    ) {
        let contribution_path_inserted = self.contribution_paths.insert(snapshot.path_kind);
        append_unique_reasons(&mut self.result.ranking_reasons, &candidate.ranking_reasons);
        append_unique_strings(
            &mut self.result.partial_truth_causes,
            &snapshot.partial_truth_causes,
        );
        append_unique_strings(
            &mut self.result.partial_truth_causes,
            &candidate.partial_truth_causes,
        );
        self.result.contributions.push(build_contribution(
            snapshot,
            candidate,
            decision.decision_class,
        ));
        if self.result.scope_truth.is_none() {
            self.result.scope_truth = candidate.scope_truth.clone();
        }
        if contribution_path_inserted && self.contribution_paths.len() > 1 {
            self.result.truth_class = PlannerResultTruthClass::Hybrid;
        }
    }
}

fn build_path_decisions(
    surface: SearchSurface,
    snapshots: &[PlannerPathSnapshot],
) -> Vec<PlannerPathDecision> {
    let priority = priority_for_surface(surface);
    let mut decisions = Vec::new();
    let mut selected_any = false;
    let mut higher_authority_unavailable = false;
    let mut consumed = BTreeSet::new();

    for path_kind in priority {
        for (index, snapshot) in snapshots.iter().enumerate() {
            if snapshot.path_kind != *path_kind {
                continue;
            }
            consumed.insert(index);
            let decision_class = if snapshot.can_answer() {
                if selected_any {
                    PlannerPathDecisionClass::SelectedSupplementary
                } else {
                    selected_any = true;
                    if higher_authority_unavailable {
                        PlannerPathDecisionClass::SelectedFallback
                    } else {
                        PlannerPathDecisionClass::SelectedPrimary
                    }
                }
            } else if snapshot.is_disclosed_unavailable() {
                higher_authority_unavailable = true;
                PlannerPathDecisionClass::UnavailableDisclosed
            } else {
                PlannerPathDecisionClass::SkippedNoRows
            };
            decisions.push(decision_from_snapshot(snapshot, decision_class));
        }
    }

    for (index, snapshot) in snapshots.iter().enumerate() {
        if consumed.contains(&index) {
            continue;
        }
        decisions.push(decision_from_snapshot(
            snapshot,
            PlannerPathDecisionClass::SkippedSurfaceIneligible,
        ));
    }

    decisions.sort_by(|a, b| {
        priority_index(surface, a.path_kind)
            .cmp(&priority_index(surface, b.path_kind))
            .then_with(|| a.decision_class.rank().cmp(&b.decision_class.rank()))
            .then_with(|| a.snapshot_id.cmp(&b.snapshot_id))
    });
    decisions
}

fn build_result_set(
    inputs: &SearchPlannerInputs,
    decisions: &[PlannerPathDecision],
) -> PlannedResultSet {
    let decision_by_snapshot: BTreeMap<&str, &PlannerPathDecision> = decisions
        .iter()
        .map(|decision| (decision.snapshot_id.as_str(), decision))
        .collect();
    let mut by_canonical: BTreeMap<String, PlannedSearchResultAccumulator> = BTreeMap::new();
    let mut snapshots = inputs.path_snapshots.iter().collect::<Vec<_>>();
    snapshots.sort_by(|a, b| {
        let a_decision = decision_by_snapshot.get(a.snapshot_id.as_str());
        let b_decision = decision_by_snapshot.get(b.snapshot_id.as_str());
        a_decision
            .map(|d| d.decision_class.rank())
            .unwrap_or(u8::MAX)
            .cmp(
                &b_decision
                    .map(|d| d.decision_class.rank())
                    .unwrap_or(u8::MAX),
            )
            .then_with(|| {
                priority_index(inputs.query_session.surface, a.path_kind)
                    .cmp(&priority_index(inputs.query_session.surface, b.path_kind))
            })
            .then_with(|| a.snapshot_id.cmp(&b.snapshot_id))
    });

    for snapshot in snapshots {
        let Some(decision) = decision_by_snapshot.get(snapshot.snapshot_id.as_str()) else {
            continue;
        };
        if !decision.decision_class.is_selected() {
            continue;
        }
        for candidate in &snapshot.rows {
            by_canonical
                .entry(candidate.canonical_id.clone())
                .and_modify(|accumulator| {
                    accumulator.add_contribution(snapshot, candidate, decision);
                })
                .or_insert_with(|| {
                    let contribution =
                        build_contribution(snapshot, candidate, decision.decision_class);
                    let mut partial_truth_causes = snapshot.partial_truth_causes.clone();
                    append_unique_strings(
                        &mut partial_truth_causes,
                        &candidate.partial_truth_causes,
                    );
                    let truth_class = truth_class_for(snapshot.path_kind, snapshot.readiness);
                    let fallback_reason = fallback_reason_for(decisions);
                    let degraded = decision.decision_class
                        == PlannerPathDecisionClass::SelectedFallback
                        && fallback_reason.is_some_and(is_graph_or_language_unavailable);
                    let result = PlannedSearchResult {
                        result_id: build_planned_result_id(
                            inputs.query_session.surface,
                            &candidate.canonical_id,
                        ),
                        canonical_id: candidate.canonical_id.clone(),
                        target_kind: candidate.target_kind,
                        title: candidate.title.clone(),
                        relative_path: candidate.relative_path.clone(),
                        symbol_ref: candidate.symbol_ref.clone(),
                        answered_by: snapshot.path_kind,
                        answer_role: decision.decision_class,
                        truth_class,
                        readiness_state: snapshot.readiness,
                        ranking_reasons: candidate.ranking_reasons.clone(),
                        partial_truth_causes,
                        scope_truth: candidate.scope_truth.clone(),
                        contributions: vec![contribution],
                        explanation: PlannerResultExplanation {
                            summary: explanation_summary(
                                snapshot.path_kind,
                                decision.decision_class,
                                snapshot.readiness,
                            ),
                            selected_path: snapshot.path_kind,
                            degraded_by_missing_graph_or_language: degraded,
                            fallback_reason,
                        },
                    };
                    let mut contribution_paths = BTreeSet::new();
                    contribution_paths.insert(snapshot.path_kind);
                    PlannedSearchResultAccumulator {
                        result,
                        contribution_paths,
                    }
                });
        }
    }

    let mut rows = by_canonical
        .into_values()
        .map(|accumulator| accumulator.result)
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        a.answer_role
            .rank()
            .cmp(&b.answer_role.rank())
            .then_with(|| {
                priority_index(inputs.query_session.surface, a.answered_by)
                    .cmp(&priority_index(inputs.query_session.surface, b.answered_by))
            })
            .then_with(|| primary_reason_rank(a).cmp(&primary_reason_rank(b)))
            .then_with(|| {
                a.title
                    .to_ascii_lowercase()
                    .cmp(&b.title.to_ascii_lowercase())
            })
            .then_with(|| a.canonical_id.cmp(&b.canonical_id))
    });

    PlannedResultSet {
        record_kind: PlannedResultSet::RECORD_KIND.to_string(),
        schema_version: 1,
        result_set_id: inputs.result_set_id.clone(),
        query_session_id_ref: inputs.query_session.query_session_id.clone(),
        planner_pass_id_ref: inputs.planner_pass_id.clone(),
        surface: inputs.query_session.surface,
        readiness_state: PlannerPathReadiness::Unavailable,
        rows,
        observed_at: inputs.observed_at.clone(),
    }
}

fn build_contribution(
    snapshot: &PlannerPathSnapshot,
    candidate: &PlannerCandidate,
    role: PlannerPathDecisionClass,
) -> PlannerContribution {
    let mut partial_truth_causes = snapshot.partial_truth_causes.clone();
    append_unique_strings(&mut partial_truth_causes, &candidate.partial_truth_causes);
    PlannerContribution {
        path_kind: snapshot.path_kind,
        snapshot_id: snapshot.snapshot_id.clone(),
        candidate_id: candidate.candidate_id.clone(),
        role,
        readiness: snapshot.readiness,
        ranking_reasons: candidate.ranking_reasons.clone(),
        partial_truth_causes,
        scope_truth: candidate.scope_truth.clone(),
    }
}

fn decision_from_snapshot(
    snapshot: &PlannerPathSnapshot,
    decision_class: PlannerPathDecisionClass,
) -> PlannerPathDecision {
    PlannerPathDecision {
        path_kind: snapshot.path_kind,
        snapshot_id: snapshot.snapshot_id.clone(),
        decision_class,
        readiness: snapshot.readiness,
        freshness: snapshot.freshness,
        unavailable_reason: snapshot.unavailable_reason,
        candidate_count: snapshot.rows.len(),
        partial_truth_causes: snapshot.partial_truth_causes.clone(),
    }
}

fn readiness_for_result_set(
    rows: &[PlannedSearchResult],
    decisions: &[PlannerPathDecision],
) -> PlannerPathReadiness {
    if rows.is_empty() {
        if decisions.iter().any(|decision| {
            decision.decision_class == PlannerPathDecisionClass::UnavailableDisclosed
        }) {
            return PlannerPathReadiness::Unavailable;
        }
        return PlannerPathReadiness::Warming;
    }

    let mut readiness = PlannerPathReadiness::Ready;
    for row in rows {
        readiness = merge_readiness(readiness, row.readiness_state);
    }
    readiness
}

fn merge_readiness(
    current: PlannerPathReadiness,
    next: PlannerPathReadiness,
) -> PlannerPathReadiness {
    let current_rank = readiness_rank(current);
    let next_rank = readiness_rank(next);
    if next_rank > current_rank {
        next
    } else {
        current
    }
}

fn readiness_rank(readiness: PlannerPathReadiness) -> u8 {
    match readiness {
        PlannerPathReadiness::Ready => 0,
        PlannerPathReadiness::HotSetReady => 1,
        PlannerPathReadiness::Stale => 2,
        PlannerPathReadiness::Partial => 3,
        PlannerPathReadiness::Warming => 4,
        PlannerPathReadiness::OutOfScope => 5,
        PlannerPathReadiness::Unavailable => 6,
    }
}

fn semantic_fallback_state(
    rows: &[PlannedSearchResult],
    decisions: &[PlannerPathDecision],
) -> SemanticFallbackState {
    if rows.is_empty() {
        return SemanticFallbackState::NoUsablePath;
    }
    if rows
        .iter()
        .any(|row| row.answered_by == PlannerDataPath::GraphBacked)
    {
        return SemanticFallbackState::GraphBacked;
    }

    let graph_unavailable = decisions.iter().any(|decision| {
        decision.path_kind == PlannerDataPath::GraphBacked
            && decision.decision_class == PlannerPathDecisionClass::UnavailableDisclosed
    });
    let language_unavailable = decisions.iter().any(|decision| {
        decision.path_kind == PlannerDataPath::Structural
            && decision.decision_class == PlannerPathDecisionClass::UnavailableDisclosed
    });
    let lexical_unavailable = decisions.iter().any(|decision| {
        decision.path_kind == PlannerDataPath::Lexical
            && decision.decision_class == PlannerPathDecisionClass::UnavailableDisclosed
    });

    if graph_unavailable
        && rows
            .iter()
            .any(|row| row.answered_by == PlannerDataPath::Structural)
    {
        return SemanticFallbackState::GraphUnavailableStructuralFallback;
    }
    if graph_unavailable
        && rows
            .iter()
            .any(|row| row.answered_by == PlannerDataPath::Lexical)
    {
        return SemanticFallbackState::GraphUnavailableLexicalOnly;
    }
    if graph_unavailable
        && rows
            .iter()
            .any(|row| row.answered_by == PlannerDataPath::Cached)
    {
        return SemanticFallbackState::GraphUnavailableCachedOnly;
    }
    if language_unavailable
        && rows
            .iter()
            .any(|row| row.answered_by == PlannerDataPath::Lexical)
    {
        return SemanticFallbackState::LanguageUnavailableLexicalOnly;
    }
    if lexical_unavailable
        && rows
            .iter()
            .any(|row| row.answered_by == PlannerDataPath::Cached)
    {
        return SemanticFallbackState::CachedFallbackLexicalUnavailable;
    }

    SemanticFallbackState::SemanticNotApplicable
}

fn fallback_reason_for(decisions: &[PlannerPathDecision]) -> Option<PlannerUnavailableReason> {
    decisions
        .iter()
        .find(|decision| decision.decision_class == PlannerPathDecisionClass::UnavailableDisclosed)
        .and_then(|decision| decision.unavailable_reason)
}

fn is_graph_or_language_unavailable(reason: PlannerUnavailableReason) -> bool {
    matches!(
        reason,
        PlannerUnavailableReason::GraphWarming
            | PlannerUnavailableReason::GraphUnavailable
            | PlannerUnavailableReason::LanguageUnavailable
    )
}

fn first_selected_index_epoch(
    snapshots: &[PlannerPathSnapshot],
    decisions: &[PlannerPathDecision],
) -> Option<String> {
    first_selected_epoch(snapshots, decisions, |snapshot| {
        snapshot.index_epoch.clone()
    })
}

fn first_selected_graph_epoch(
    snapshots: &[PlannerPathSnapshot],
    decisions: &[PlannerPathDecision],
) -> Option<String> {
    first_selected_epoch(snapshots, decisions, |snapshot| {
        snapshot.graph_epoch.clone()
    })
}

fn first_selected_epoch(
    snapshots: &[PlannerPathSnapshot],
    decisions: &[PlannerPathDecision],
    epoch: impl Fn(&PlannerPathSnapshot) -> Option<String>,
) -> Option<String> {
    decisions
        .iter()
        .filter(|decision| decision.decision_class.is_selected())
        .find_map(|decision| {
            snapshots
                .iter()
                .find(|snapshot| snapshot.snapshot_id == decision.snapshot_id)
                .and_then(&epoch)
        })
}

fn truth_class_for(
    path_kind: PlannerDataPath,
    readiness: PlannerPathReadiness,
) -> PlannerResultTruthClass {
    match path_kind {
        PlannerDataPath::GraphBacked => PlannerResultTruthClass::GraphBacked,
        PlannerDataPath::Docs => PlannerResultTruthClass::Imported,
        PlannerDataPath::Cached => PlannerResultTruthClass::Cached,
        PlannerDataPath::Structural => PlannerResultTruthClass::Heuristic,
        PlannerDataPath::Lexical => {
            if readiness == PlannerPathReadiness::Ready {
                PlannerResultTruthClass::Exact
            } else {
                PlannerResultTruthClass::Heuristic
            }
        }
    }
}

fn explanation_summary(
    path_kind: PlannerDataPath,
    decision_class: PlannerPathDecisionClass,
    readiness: PlannerPathReadiness,
) -> String {
    let role = match decision_class {
        PlannerPathDecisionClass::SelectedPrimary => "selected primary",
        PlannerPathDecisionClass::SelectedFallback => "selected fallback",
        PlannerPathDecisionClass::SelectedSupplementary => "supplementary",
        PlannerPathDecisionClass::UnavailableDisclosed
        | PlannerPathDecisionClass::SkippedNoRows
        | PlannerPathDecisionClass::SkippedSurfaceIneligible => "not selected",
    };
    format!(
        "{} path answered as {role} with {} readiness.",
        path_kind.badge(),
        readiness.as_str()
    )
}

fn append_unique_reasons(
    target: &mut Vec<PlannerRankingReason>,
    incoming: &[PlannerRankingReason],
) {
    for reason in incoming {
        if !target.contains(reason) {
            target.push(*reason);
        }
    }
}

fn append_unique_strings(target: &mut Vec<String>, incoming: &[String]) {
    for value in incoming {
        if !target.contains(value) {
            target.push(value.clone());
        }
    }
}

fn priority_for_surface(surface: SearchSurface) -> &'static [PlannerDataPath] {
    match surface {
        SearchSurface::QuickOpen => &QUICK_OPEN_PRIORITY,
        SearchSurface::FileSearch => &FILE_SEARCH_PRIORITY,
        SearchSurface::SymbolSearch => &SYMBOL_SEARCH_PRIORITY,
        SearchSurface::DocsSearch => &DOCS_SEARCH_PRIORITY,
    }
}

fn priority_index(surface: SearchSurface, path_kind: PlannerDataPath) -> usize {
    priority_for_surface(surface)
        .iter()
        .position(|candidate| *candidate == path_kind)
        .unwrap_or(usize::MAX)
}

fn primary_reason_rank(row: &PlannedSearchResult) -> u8 {
    row.ranking_reasons
        .first()
        .map(|reason| reason.rank())
        .unwrap_or(u8::MAX)
}

fn title_for_path(relative_path: &str) -> String {
    relative_path
        .rsplit_once('/')
        .map(|(_, basename)| basename)
        .unwrap_or(relative_path)
        .to_string()
}

fn map_lexical_reason(reason: RankingReasonClass) -> PlannerRankingReason {
    match reason {
        RankingReasonClass::ExactBasenameMatch => PlannerRankingReason::LexicalExactMatch,
        RankingReasonClass::PrefixBasenameMatch => PlannerRankingReason::LexicalPrefixMatch,
        RankingReasonClass::SubstringBasenameMatch => PlannerRankingReason::LexicalSubstringMatch,
        RankingReasonClass::SubstringPathMatch => PlannerRankingReason::LexicalPathMatch,
        RankingReasonClass::GeneratedArtifactDeprioritized => {
            PlannerRankingReason::GeneratedArtifactDeprioritized
        }
        RankingReasonClass::PartialCoverageCaveat => PlannerRankingReason::PartialIndex,
    }
}

fn candidate_from_graph_row(row: &aureline_graph::GraphQueryRow) -> Option<PlannerCandidate> {
    let canonical_id = row.canonical_id()?.to_string();
    let target_kind = match row.node_class {
        Some(aureline_graph::NodeClass::SymbolNode) => PlannerTargetKind::Symbol,
        Some(
            aureline_graph::NodeClass::FileNode
            | aureline_graph::NodeClass::DirectoryNode
            | aureline_graph::NodeClass::DocNode,
        ) => PlannerTargetKind::File,
        _ if row.edge_id.is_some() => PlannerTargetKind::Symbol,
        _ => PlannerTargetKind::File,
    };
    let mut ranking_reasons = if row.edge_id.is_some() {
        vec![PlannerRankingReason::GraphNeighbourhoodHop]
    } else if row.node_class == Some(aureline_graph::NodeClass::SymbolNode) {
        vec![PlannerRankingReason::GraphExactSymbol]
    } else {
        vec![PlannerRankingReason::GraphNeighbourhoodHop]
    };
    let partial_truth_causes = row
        .partial_truth_causes
        .iter()
        .map(|cause| cause.as_str().to_string())
        .collect::<Vec<_>>();
    if !partial_truth_causes.is_empty() {
        ranking_reasons.push(PlannerRankingReason::PartialIndex);
    }

    Some(PlannerCandidate {
        candidate_id: format!("{}:{}", row.row_class.as_str(), row.row_index),
        canonical_id,
        target_kind,
        title: row.display_label.clone(),
        relative_path: row.relative_path.clone(),
        symbol_ref: row.symbol_ref.clone(),
        ranking_reasons,
        partial_truth_causes,
        scope_truth: None,
    })
}

fn readiness_from_graph_envelope(
    envelope: &aureline_graph::GraphQueryEnvelope,
) -> PlannerPathReadiness {
    match envelope.readiness {
        aureline_graph::GraphQueryReadiness::Ready => PlannerPathReadiness::Ready,
        aureline_graph::GraphQueryReadiness::HotSetReady => PlannerPathReadiness::HotSetReady,
        aureline_graph::GraphQueryReadiness::Partial => PlannerPathReadiness::Partial,
        aureline_graph::GraphQueryReadiness::Warming => PlannerPathReadiness::Warming,
        aureline_graph::GraphQueryReadiness::Stale => PlannerPathReadiness::Stale,
        aureline_graph::GraphQueryReadiness::Unavailable => PlannerPathReadiness::Unavailable,
        aureline_graph::GraphQueryReadiness::OutOfScope => PlannerPathReadiness::OutOfScope,
    }
}

fn freshness_from_graph_envelope(
    envelope: &aureline_graph::GraphQueryEnvelope,
) -> PlannerFreshnessClass {
    if envelope
        .partial_truth_causes
        .contains(&aureline_graph::GraphPartialTruthCause::Imported)
    {
        return PlannerFreshnessClass::Imported;
    }
    if envelope.partial_truth_causes.iter().any(|cause| {
        matches!(
            cause,
            aureline_graph::GraphPartialTruthCause::Stale
                | aureline_graph::GraphPartialTruthCause::Replayed
        )
    }) {
        return PlannerFreshnessClass::StaleCached;
    }
    if envelope
        .partial_truth_causes
        .contains(&aureline_graph::GraphPartialTruthCause::Warming)
    {
        return PlannerFreshnessClass::Unknown;
    }
    PlannerFreshnessClass::AuthoritativeLive
}

fn candidate_from_symbol_record(symbol: &aureline_language::SymbolRecord) -> PlannerCandidate {
    let mut ranking_reasons = vec![
        PlannerRankingReason::StructuralSymbolMatch,
        PlannerRankingReason::StructuralFallback,
        PlannerRankingReason::SymbolKindPrior,
    ];
    if !symbol.partial_truth_causes.is_empty() {
        ranking_reasons.push(PlannerRankingReason::PartialIndex);
    }

    PlannerCandidate {
        candidate_id: symbol.symbol_id.clone(),
        canonical_id: symbol.stable_symbol_ref.clone(),
        target_kind: PlannerTargetKind::Symbol,
        title: symbol.name.clone(),
        relative_path: Some(symbol.workspace_relative_path.clone()),
        symbol_ref: Some(symbol.stable_symbol_ref.clone()),
        ranking_reasons,
        partial_truth_causes: symbol.partial_truth_causes.clone(),
        scope_truth: None,
    }
}

fn candidate_from_docs_search_entry(entry: &DocsSearchIndexEntry) -> PlannerCandidate {
    let mut ranking_reasons = vec![PlannerRankingReason::DocsAnchorMatch];
    match entry.docs_node.citation_availability {
        CitationAnchorAvailability::ExactAnchorAvailable => {
            ranking_reasons.push(PlannerRankingReason::CitationAvailable);
        }
        CitationAnchorAvailability::NotCitationBearing => {}
        CitationAnchorAvailability::AnchorUnavailableDisclosed
        | CitationAnchorAvailability::HiddenByPolicy
        | CitationAnchorAvailability::OmittedByPolicy => {
            ranking_reasons.push(PlannerRankingReason::CitationMissing);
        }
    }
    let partial_truth_causes = entry.partial_truth_causes();
    if !partial_truth_causes.is_empty() {
        ranking_reasons.push(PlannerRankingReason::PartialIndex);
    }

    PlannerCandidate {
        candidate_id: entry.canonical_ref.clone(),
        canonical_id: entry.canonical_ref.clone(),
        target_kind: PlannerTargetKind::DocsAnchor,
        title: entry.title.clone(),
        relative_path: None,
        symbol_ref: Some(entry.exact_reopen_ref.clone()),
        ranking_reasons,
        partial_truth_causes,
        scope_truth: None,
    }
}

fn freshness_from_docs_search_entries(entries: &[DocsSearchIndexEntry]) -> PlannerFreshnessClass {
    if entries.is_empty() {
        return PlannerFreshnessClass::Unknown;
    }
    if entries
        .iter()
        .any(|entry| entry.docs_node.freshness_class == DocsIndexFreshnessClass::Unverified)
    {
        return PlannerFreshnessClass::Unknown;
    }
    if entries.iter().any(|entry| {
        matches!(
            entry.docs_node.freshness_class,
            DocsIndexFreshnessClass::DegradedCached | DocsIndexFreshnessClass::Stale
        ) || entry.docs_node.version_match_state
            == DocsIndexVersionMatchState::IncompatibleDriftDetected
    }) {
        return PlannerFreshnessClass::StaleCached;
    }
    if entries
        .iter()
        .any(|entry| entry.docs_node.freshness_class == DocsIndexFreshnessClass::WarmCached)
    {
        return PlannerFreshnessClass::WarmCached;
    }
    PlannerFreshnessClass::AuthoritativeLive
}

fn readiness_from_symbol_snapshot(
    snapshot: &aureline_language::SymbolSnapshotRecord,
) -> PlannerPathReadiness {
    match snapshot.state.completeness_class {
        aureline_language::SymbolSnapshotCompletenessClass::CompleteCurrentFile => {
            PlannerPathReadiness::Ready
        }
        aureline_language::SymbolSnapshotCompletenessClass::PartialParserErrors => {
            PlannerPathReadiness::Partial
        }
        aureline_language::SymbolSnapshotCompletenessClass::UnavailableNoStructure => {
            PlannerPathReadiness::Unavailable
        }
    }
}

fn freshness_from_symbol_snapshot(
    snapshot: &aureline_language::SymbolSnapshotRecord,
) -> PlannerFreshnessClass {
    match snapshot.state.parse_freshness_class {
        aureline_language::ParseFreshnessClass::CurrentBufferVersion => {
            PlannerFreshnessClass::AuthoritativeLive
        }
        aureline_language::ParseFreshnessClass::WarmCached => PlannerFreshnessClass::WarmCached,
        aureline_language::ParseFreshnessClass::StaleBufferVersion
        | aureline_language::ParseFreshnessClass::StaleGrammarVersion => {
            PlannerFreshnessClass::StaleCached
        }
        aureline_language::ParseFreshnessClass::UnverifiedImported => {
            PlannerFreshnessClass::Unknown
        }
    }
}

fn default_planner_version() -> String {
    SEARCH_PLANNER_ALPHA_VERSION.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counts::{
        HiddenScopeDisclosure, ScopeCandidateTruthRecord, ScopeTruthSurface, SearchNoResultsState,
        SearchScopeCountsInputs, SearchScopeCountsRecord,
    };
    use crate::lexical::scope::ScopeClass;

    fn session(surface: SearchSurface) -> SearchQuerySession {
        SearchQuerySession::for_local_text(
            "search:session:test",
            surface,
            "main",
            ScopeClass::CurrentRepo,
            "Current repo",
            SEARCH_PLANNER_ALPHA_VERSION,
            "warming",
            "mono:1",
        )
    }

    fn outside_scope_truth() -> ScopeCandidateTruthRecord {
        let counts = SearchScopeCountsRecord::derive(SearchScopeCountsInputs {
            visible_rows: 0,
            loaded_rows: Some(0),
            all_matching_rows: Some(1),
            hidden_by_current_scope_rows: 1,
            hidden_by_policy_rows: 0,
            hidden_by_remote_cache_rows: 0,
            readiness_is_ready: true,
        });
        let hidden =
            HiddenScopeDisclosure::derive("Selected workset · Editor core", &counts, None, false);
        ScopeCandidateTruthRecord::new(
            ScopeTruthSurface::GraphCandidate,
            "Selected workset · Editor core",
            "selected_workset",
            Some("scope:editor_core".to_string()),
            Some("sparse".to_string()),
            Some("repo:payments-api".to_string()),
            "authoritative_live",
            true,
            false,
            counts,
            SearchNoResultsState::NoResultsInThisWorkset,
            hidden,
            vec![],
        )
    }

    #[test]
    fn symbol_search_uses_structural_fallback_when_graph_is_unavailable() {
        let inputs = SearchPlannerInputs {
            query_session: session(SearchSurface::SymbolSearch),
            planner_pass_id: "search:planner:test".to_string(),
            result_set_id: "search:result_set:test".to_string(),
            planner_version: SEARCH_PLANNER_ALPHA_VERSION.to_string(),
            observed_at: "mono:1".to_string(),
            path_snapshots: vec![
                PlannerPathSnapshot {
                    path_kind: PlannerDataPath::GraphBacked,
                    snapshot_id: "search:snapshot:graph".to_string(),
                    readiness: PlannerPathReadiness::Warming,
                    freshness: PlannerFreshnessClass::Unknown,
                    index_epoch: None,
                    graph_epoch: Some("graph:epoch:1".to_string()),
                    unavailable_reason: Some(PlannerUnavailableReason::GraphWarming),
                    partial_truth_causes: vec!["indexing_in_progress".to_string()],
                    rows: Vec::new(),
                },
                PlannerPathSnapshot {
                    path_kind: PlannerDataPath::Structural,
                    snapshot_id: "search:snapshot:structural".to_string(),
                    readiness: PlannerPathReadiness::Partial,
                    freshness: PlannerFreshnessClass::AuthoritativeLive,
                    index_epoch: Some("struct:epoch:1".to_string()),
                    graph_epoch: None,
                    unavailable_reason: None,
                    partial_truth_causes: vec!["graph_pending".to_string()],
                    rows: vec![PlannerCandidate::symbol(
                        "candidate:structural",
                        "symbol:main",
                        "main",
                        "src/main.rs",
                        "rust:fn:main",
                        vec![
                            PlannerRankingReason::StructuralSymbolMatch,
                            PlannerRankingReason::StructuralFallback,
                        ],
                    )],
                },
            ],
        };

        let output = SearchPlannerAlpha::plan(inputs);
        assert_eq!(
            output.planner_pass.semantic_fallback_state,
            SemanticFallbackState::GraphUnavailableStructuralFallback
        );
        assert_eq!(
            output.result_set.rows[0].answered_by,
            PlannerDataPath::Structural
        );
        assert_eq!(
            output.result_set.rows[0].answer_role,
            PlannerPathDecisionClass::SelectedFallback
        );
    }

    #[test]
    fn graph_candidate_preserves_shared_scope_truth() {
        let truth = outside_scope_truth();
        let inputs = SearchPlannerInputs {
            query_session: session(SearchSurface::SymbolSearch),
            planner_pass_id: "search:planner:scope_truth".to_string(),
            result_set_id: "search:result_set:scope_truth".to_string(),
            planner_version: SEARCH_PLANNER_ALPHA_VERSION.to_string(),
            observed_at: "mono:2".to_string(),
            path_snapshots: vec![PlannerPathSnapshot {
                path_kind: PlannerDataPath::GraphBacked,
                snapshot_id: "search:snapshot:graph:scope_truth".to_string(),
                readiness: PlannerPathReadiness::Ready,
                freshness: PlannerFreshnessClass::AuthoritativeLive,
                index_epoch: None,
                graph_epoch: Some("graph:epoch:scope_truth".to_string()),
                unavailable_reason: None,
                partial_truth_causes: Vec::new(),
                rows: vec![PlannerCandidate::symbol(
                    "candidate:graph:outside_scope",
                    "symbol:route:handler",
                    "RouteHandler",
                    "services/payments/src/routes.rs",
                    "rust:fn:RouteHandler",
                    vec![PlannerRankingReason::GraphExactSymbol],
                )
                .with_scope_truth(truth.clone())],
            }],
        };

        let output = SearchPlannerAlpha::plan(inputs);
        let row_truth = output.result_set.rows[0]
            .scope_truth
            .as_ref()
            .expect("graph row must preserve shared scope truth");
        assert_eq!(row_truth.surface_token, "graph_candidate");
        assert_eq!(row_truth.candidate_scope_label, "Outside current scope");
        assert_eq!(
            row_truth.repo_or_module_ref.as_deref(),
            Some("repo:payments-api")
        );
        assert_eq!(
            row_truth.counts.hidden_by_current_scope_rows,
            truth.counts.hidden_by_current_scope_rows
        );
        assert_eq!(
            output.result_set.rows[0].contributions[0]
                .scope_truth
                .as_ref()
                .expect("contribution must preserve shared scope truth")
                .active_scope_label,
            "Selected workset · Editor core"
        );
    }
}
