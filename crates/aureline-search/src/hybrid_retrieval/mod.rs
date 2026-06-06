//! Hybrid retrieval inspector and embedding-index truth packets.
//!
//! This module is the search-owned contract for beta hybrid retrieval rows.
//! It joins lexical, vector, and graph contributions into one export-safe
//! packet with explicit locality, readiness, retrieval epoch, embedder identity,
//! fallback policy, and consumer projections. The packet is intentionally
//! metadata-only: it carries no raw vectors, raw source bodies, provider
//! payloads, secrets, or private numeric rank weights.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::planner::{
    PlannedSearchResult, PlannerContribution, PlannerDataPath, PlannerFreshnessClass,
    PlannerPathReadiness, PlannerRankingReason, PlannerTargetKind, SearchPlannerOutput,
};

/// Stable record-kind tag for [`RetrievalInspectorPacket`].
pub const RETRIEVAL_INSPECTOR_RECORD_KIND: &str = "retrieval_inspector_beta_packet";

/// Stable record-kind tag for [`RetrievalInspectorSupportExport`].
pub const RETRIEVAL_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "retrieval_inspector_support_export";

/// Integer schema version for retrieval-inspector beta packets.
pub const RETRIEVAL_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the retrieval-inspector boundary schema.
pub const RETRIEVAL_INSPECTOR_SCHEMA_REF: &str = "schemas/search/retrieval_inspector.schema.json";

/// Repo-relative path of the stable hybrid-retrieval inspector schema.
pub const HYBRID_RETRIEVAL_STABLE_SCHEMA_REF: &str =
    "schemas/search/hybrid-retrieval-inspector.schema.json";

/// Repo-relative path of the hybrid-retrieval reviewer doc.
pub const HYBRID_RETRIEVAL_BETA_DOC_REF: &str = "docs/search/m3/hybrid_retrieval_beta.md";

/// Repo-relative path of the stable hybrid-retrieval reviewer doc.
pub const HYBRID_RETRIEVAL_STABLE_DOC_REF: &str =
    "docs/search/m4/stabilize-hybrid-retrieval-epochs-and-locality.md";

/// Repo-relative path of the human-readable stable reviewer artifact.
pub const HYBRID_RETRIEVAL_STABLE_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/stabilize-hybrid-retrieval-epochs-and-locality.md";

/// Repo-relative path of the canonical checked-in beta packet artifact.
pub const HYBRID_RETRIEVAL_BETA_PACKET_REF: &str =
    "artifacts/search/m3/hybrid_retrieval_beta_packet.json";

/// Repo-relative path of the canonical checked-in stable packet artifact.
pub const HYBRID_RETRIEVAL_STABLE_PACKET_REF: &str =
    "artifacts/search/m4/hybrid_retrieval_inspector_packet.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const HYBRID_RETRIEVAL_BETA_FIXTURE_DIR: &str = "fixtures/search/hybrid_retrieval_beta";

/// Repo-relative path of the protected stable fixture corpus directory.
pub const HYBRID_RETRIEVAL_STABLE_FIXTURE_DIR: &str =
    "fixtures/search/m4/stabilize-hybrid-retrieval-epochs-and-locality";

/// Stability profile for retrieval-inspector validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalGovernanceTrack {
    /// Beta compatibility profile used by existing M3 retrieval packets.
    Beta,
    /// Stable profile that enforces M4 semantic-recall invariants.
    Stable,
}

impl RetrievalGovernanceTrack {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }
}

impl Default for RetrievalGovernanceTrack {
    fn default() -> Self {
        Self::Beta
    }
}

/// Query class used to select latency and retrieval-lane posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalQueryClass {
    /// Query was not classified by the producer.
    Unclassified,
    /// Exact path, filename, text, or command lookup.
    Exact,
    /// Symbol, outline, type, dependency, or graph-structure lookup.
    Structural,
    /// Conceptual lookup where embedding recall may add candidates.
    Conceptual,
    /// Mixed query that intentionally combines exact, structural, and semantic lanes.
    Mixed,
}

impl RetrievalQueryClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unclassified => "unclassified",
            Self::Exact => "exact",
            Self::Structural => "structural",
            Self::Conceptual => "conceptual",
            Self::Mixed => "mixed",
        }
    }
}

impl Default for RetrievalQueryClass {
    fn default() -> Self {
        Self::Unclassified
    }
}

/// Search, AI, review, docs, or support surface that consumes a retrieval packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalConsumerSurface {
    /// Search result pane or result inspector.
    SearchResults,
    /// AI context picker or context inspector.
    AiContext,
    /// Review workspace or review-assist evidence lane.
    ReviewWorkspace,
    /// Docs/help surface explaining retrieval state.
    DocsHelp,
    /// Support export preview or generated bundle.
    SupportExport,
}

impl RetrievalConsumerSurface {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResults => "search_results",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Closed lane vocabulary for hybrid retrieval contributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalLaneClass {
    /// Exact text, path, filename, or regex recall.
    Lexical,
    /// Structural recall from symbols, outlines, anchors, or syntax indexes.
    Structural,
    /// Embedding or vector recall with an embedder identity and retrieval epoch.
    Vector,
    /// Embedding recall with an embedder identity and retrieval epoch.
    Embedding,
    /// Semantic graph recall or graph-neighborhood expansion.
    Graph,
    /// Fused row assembled from more than one lane.
    Fused,
}

impl RetrievalLaneClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lexical => "lexical",
            Self::Structural => "structural",
            Self::Vector => "vector",
            Self::Embedding => "embedding",
            Self::Graph => "graph",
            Self::Fused => "fused",
        }
    }

    fn from_planner_path(path: PlannerDataPath) -> Self {
        match path {
            PlannerDataPath::Lexical => Self::Lexical,
            PlannerDataPath::GraphBacked => Self::Graph,
            PlannerDataPath::Structural | PlannerDataPath::Docs => Self::Structural,
            PlannerDataPath::Cached => Self::Fused,
        }
    }
}

/// Locality vocabulary disclosed for every retrieval lane and contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalLocalityClass {
    /// Data was retrieved from the current local workspace.
    LocalWorkspace,
    /// Data was retrieved from a local disposable cache.
    LocalCache,
    /// Data was retrieved from a signed mirrored or offline pack.
    MirroredPack,
    /// Data was retrieved from a tenant-scoped managed index.
    ManagedTenantScoped,
    /// Data was retrieved from a provider or remote index.
    ProviderRemote,
}

impl RetrievalLocalityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::LocalCache => "local_cache",
            Self::MirroredPack => "mirrored_pack",
            Self::ManagedTenantScoped => "managed_tenant_scoped",
            Self::ProviderRemote => "provider_remote",
        }
    }

    /// Returns true when this locality crosses the local-first boundary.
    pub const fn is_remote(self) -> bool {
        matches!(self, Self::ManagedTenantScoped | Self::ProviderRemote)
    }
}

/// Readiness state attached to a retrieval lane or row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalReadinessClass {
    /// Lane is ready for the declared scope.
    Ready,
    /// Hot-set rows are ready while broader indexing continues.
    HotSetReady,
    /// Lane is warming and may return incomplete rows.
    Warming,
    /// Lane has rows but incomplete declared-scope coverage.
    Partial,
    /// Lane is known stale and must be labeled.
    Stale,
    /// Lane cannot answer.
    Unavailable,
    /// Lane is outside the active scope or workset.
    OutOfScope,
}

impl RetrievalReadinessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::HotSetReady => "hot_set_ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// Projects planner readiness into retrieval-inspector readiness.
    pub const fn from_planner_readiness(readiness: PlannerPathReadiness) -> Self {
        match readiness {
            PlannerPathReadiness::Ready => Self::Ready,
            PlannerPathReadiness::HotSetReady => Self::HotSetReady,
            PlannerPathReadiness::Warming => Self::Warming,
            PlannerPathReadiness::Partial => Self::Partial,
            PlannerPathReadiness::Stale => Self::Stale,
            PlannerPathReadiness::Unavailable => Self::Unavailable,
            PlannerPathReadiness::OutOfScope => Self::OutOfScope,
        }
    }

    fn requires_visible_caveat(self) -> bool {
        matches!(
            self,
            Self::HotSetReady | Self::Warming | Self::Partial | Self::Stale | Self::OutOfScope
        )
    }
}

/// Freshness state attached to a retrieval lane or contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalFreshnessClass {
    /// Source was captured from live local state.
    AuthoritativeLive,
    /// Source is a warm cache within an accepted freshness window.
    WarmCached,
    /// Source is stale but still inspectable.
    StaleCached,
    /// Source was imported from a mirrored pack or provider-owned source.
    Imported,
    /// Freshness could not be verified.
    Unknown,
}

impl RetrievalFreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::StaleCached => "stale_cached",
            Self::Imported => "imported",
            Self::Unknown => "unknown",
        }
    }

    /// Projects planner freshness into retrieval-inspector freshness.
    pub const fn from_planner_freshness(freshness: PlannerFreshnessClass) -> Self {
        match freshness {
            PlannerFreshnessClass::AuthoritativeLive => Self::AuthoritativeLive,
            PlannerFreshnessClass::WarmCached => Self::WarmCached,
            PlannerFreshnessClass::StaleCached => Self::StaleCached,
            PlannerFreshnessClass::Imported => Self::Imported,
            PlannerFreshnessClass::Unknown => Self::Unknown,
        }
    }
}

/// Embedding-index state disclosed by vector retrieval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingIndexStateClass {
    /// Embedding index is ready for the declared scope.
    Ready,
    /// Embedding index is warming.
    Warming,
    /// Embedding index is partial but labeled.
    Partial,
    /// Embedding index is stale and must rebuild or disclose stale use.
    Stale,
    /// Embedder, tokenizer, chunker, or trust boundary changed.
    IncompatibleEpoch,
    /// Policy blocks the embedding lane.
    PolicyBlocked,
    /// Embedding lane cannot answer.
    Unavailable,
}

impl EmbeddingIndexStateClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::IncompatibleEpoch => "incompatible_epoch",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Local-first routing posture for a retrieval packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalRoutePolicyClass {
    /// Retrieval is constrained to local lanes only.
    LocalOnly,
    /// Local lanes answered and no remote fallback was used.
    LocalFirstUsed,
    /// Local-first policy used a labeled remote fallback.
    LocalFirstRemoteFallback,
    /// Policy explicitly selected a remote or managed route.
    RemoteAllowedByPolicy,
    /// Policy blocked the remote route.
    RemoteBlockedByPolicy,
    /// No lane can currently answer.
    NoUsableLane,
}

impl RetrievalRoutePolicyClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::LocalFirstUsed => "local_first_used",
            Self::LocalFirstRemoteFallback => "local_first_remote_fallback",
            Self::RemoteAllowedByPolicy => "remote_allowed_by_policy",
            Self::RemoteBlockedByPolicy => "remote_blocked_by_policy",
            Self::NoUsableLane => "no_usable_lane",
        }
    }
}

/// Reason local-first retrieval used or withheld a fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalFallbackReasonClass {
    /// No fallback was required.
    None,
    /// Local vector index was unavailable.
    LocalVectorIndexUnavailable,
    /// Local vector index was denied by policy.
    LocalVectorPolicyDenied,
    /// Local vector index epoch was incompatible.
    LocalVectorEpochMismatch,
    /// Graph lane was unavailable.
    GraphUnavailable,
    /// Graph lane was warming.
    GraphWarming,
    /// Lexical seed rows were the only safe answer.
    LexicalSeedOnly,
    /// Remote recall was allowed by policy.
    RemoteAllowedByPolicy,
    /// Remote recall was blocked by policy.
    RemoteBlockedByPolicy,
}

impl RetrievalFallbackReasonClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::LocalVectorIndexUnavailable => "local_vector_index_unavailable",
            Self::LocalVectorPolicyDenied => "local_vector_policy_denied",
            Self::LocalVectorEpochMismatch => "local_vector_epoch_mismatch",
            Self::GraphUnavailable => "graph_unavailable",
            Self::GraphWarming => "graph_warming",
            Self::LexicalSeedOnly => "lexical_seed_only",
            Self::RemoteAllowedByPolicy => "remote_allowed_by_policy",
            Self::RemoteBlockedByPolicy => "remote_blocked_by_policy",
        }
    }
}

/// Ranking or matching reason for one retrieval contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalReasonClass {
    /// Exact lexical name or title match.
    LexicalExactMatch,
    /// Prefix lexical name or title match.
    LexicalPrefixMatch,
    /// Substring lexical name or title match.
    LexicalSubstringMatch,
    /// Lexical path match.
    LexicalPathMatch,
    /// Vector semantic similarity promoted the candidate.
    VectorSemanticSimilarity,
    /// Graph neighborhood expansion promoted the candidate.
    GraphExpansion,
    /// Graph exact symbol/entity match promoted the candidate.
    GraphExactEntity,
    /// Recentness signal boosted the candidate.
    RecencyBoost,
    /// Partial-index state affected the row.
    PartialIndex,
    /// Local fallback affected the row.
    LocalFallback,
    /// Remote route affected the row.
    RemoteRoute,
    /// Policy limit affected the row.
    PolicyLimited,
}

impl RetrievalReasonClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalExactMatch => "lexical_exact_match",
            Self::LexicalPrefixMatch => "lexical_prefix_match",
            Self::LexicalSubstringMatch => "lexical_substring_match",
            Self::LexicalPathMatch => "lexical_path_match",
            Self::VectorSemanticSimilarity => "vector_semantic_similarity",
            Self::GraphExpansion => "graph_expansion",
            Self::GraphExactEntity => "graph_exact_entity",
            Self::RecencyBoost => "recency_boost",
            Self::PartialIndex => "partial_index",
            Self::LocalFallback => "local_fallback",
            Self::RemoteRoute => "remote_route",
            Self::PolicyLimited => "policy_limited",
        }
    }

    fn from_planner_reason(reason: PlannerRankingReason) -> Self {
        match reason {
            PlannerRankingReason::ExactNameMatch | PlannerRankingReason::LexicalExactMatch => {
                Self::LexicalExactMatch
            }
            PlannerRankingReason::LexicalPrefixMatch => Self::LexicalPrefixMatch,
            PlannerRankingReason::LexicalSubstringMatch | PlannerRankingReason::TextMatch => {
                Self::LexicalSubstringMatch
            }
            PlannerRankingReason::LexicalPathMatch => Self::LexicalPathMatch,
            PlannerRankingReason::GraphExactSymbol => Self::GraphExactEntity,
            PlannerRankingReason::GraphNeighbourhoodHop => Self::GraphExpansion,
            PlannerRankingReason::RecentFileBias
            | PlannerRankingReason::RecentEditBias
            | PlannerRankingReason::HotSetBias => Self::RecencyBoost,
            PlannerRankingReason::PartialIndex
            | PlannerRankingReason::GeneratedArtifactDeprioritized
            | PlannerRankingReason::CitationMissing
            | PlannerRankingReason::StaleExampleSignal => Self::PartialIndex,
            PlannerRankingReason::GraphUnavailable | PlannerRankingReason::LanguageUnavailable => {
                Self::LocalFallback
            }
            PlannerRankingReason::StructuralSymbolMatch
            | PlannerRankingReason::StructuralFallback
            | PlannerRankingReason::SymbolKindPrior
            | PlannerRankingReason::CachedSnapshotHit
            | PlannerRankingReason::DocsAnchorMatch
            | PlannerRankingReason::DocsSymbolLinkedReference
            | PlannerRankingReason::DocsSourcePrecedence
            | PlannerRankingReason::CitationAvailable => Self::GraphExpansion,
        }
    }
}

/// Role one lane played in a fused retrieval row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalContributionRole {
    /// Lane answered the row as primary truth source.
    Primary,
    /// Lane supplied supplementary ranking or provenance evidence.
    Supplementary,
    /// Lane answered because a stronger lane was unavailable.
    Fallback,
    /// Lane was considered but withheld by policy, scope, or readiness.
    Withheld,
}

impl RetrievalContributionRole {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Supplementary => "supplementary",
            Self::Fallback => "fallback",
            Self::Withheld => "withheld",
        }
    }
}

/// Type of target a retrieval contribution points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalAnchorKind {
    /// Workspace file or path anchor.
    File,
    /// Workspace symbol anchor.
    Symbol,
    /// Documentation or help anchor.
    Docs,
    /// Runtime, incident, or generated artifact anchor.
    Artifact,
    /// Review note or review evidence anchor.
    ReviewNote,
    /// Semantic graph node or edge anchor.
    GraphNode,
}

impl RetrievalAnchorKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Docs => "docs",
            Self::Artifact => "artifact",
            Self::ReviewNote => "review_note",
            Self::GraphNode => "graph_node",
        }
    }

    fn from_planner_target(target: PlannerTargetKind) -> Self {
        match target {
            PlannerTargetKind::File | PlannerTargetKind::TextMatch => Self::File,
            PlannerTargetKind::Symbol => Self::Symbol,
            PlannerTargetKind::DocsAnchor => Self::Docs,
        }
    }
}

/// Promotion state derived from retrieval-inspector validation findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalPromotionState {
    /// Packet is eligible for the claimed beta row.
    Promotable,
    /// Packet is usable but requires review before promotion.
    NeedsReview,
    /// Packet blocks promotion until corrected.
    Blocked,
}

impl RetrievalPromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promotable => "promotable",
            Self::NeedsReview => "needs_review",
            Self::Blocked => "blocked",
        }
    }
}

/// Severity attached to one retrieval-inspector validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalInspectorFindingSeverity {
    /// Informational finding that does not affect promotion.
    Info,
    /// Reviewable finding that narrows promotion confidence.
    Warning,
    /// Finding that blocks promotion.
    Blocker,
}

impl RetrievalInspectorFindingSeverity {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocker => "blocker",
        }
    }
}

/// Closed finding vocabulary for retrieval-inspector validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalInspectorFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen beta schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Lexical, vector, or graph contribution is missing from the packet.
    MissingHybridLane,
    /// Vector contribution lacks an embedding manifest or embedder identity.
    MissingEmbeddingManifest,
    /// Remote or managed retrieval was not visibly disclosed.
    UnlabelledRemoteRoute,
    /// Partial, warming, hot-set, or stale index state lacks a cause.
    UnlabelledPartialIndex,
    /// Search, AI, or support projection does not preserve the packet.
    MissingExportProjection,
    /// Packet would allow mutation without live target re-resolution.
    MutatingActionWithoutLiveResolution,
    /// Stable packet is missing a required lane or lane snapshot.
    MissingStableLane,
    /// Stable packet did not classify the query.
    MissingQueryClassification,
    /// Embedding index or contribution uses an invalidated epoch.
    EmbeddingEpochInvalidated,
    /// One visible row mixes embedding generations.
    MixedGenerationRecall,
    /// Managed or remote embedding recall omitted tenant, region, or policy boundary.
    TenantBoundaryUndisclosed,
    /// Policy-hidden omissions were collapsed or lacked a disclosure ref.
    PolicyHiddenOmissionUndisclosed,
    /// Mirrored or signed-pack embeddings lack signature or compatibility disclosure.
    SignedPackCompatibilityUndisclosed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl RetrievalInspectorFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingHybridLane => "missing_hybrid_lane",
            Self::MissingEmbeddingManifest => "missing_embedding_manifest",
            Self::UnlabelledRemoteRoute => "unlabelled_remote_route",
            Self::UnlabelledPartialIndex => "unlabelled_partial_index",
            Self::MissingExportProjection => "missing_export_projection",
            Self::MutatingActionWithoutLiveResolution => "mutating_action_without_live_resolution",
            Self::MissingStableLane => "missing_stable_lane",
            Self::MissingQueryClassification => "missing_query_classification",
            Self::EmbeddingEpochInvalidated => "embedding_epoch_invalidated",
            Self::MixedGenerationRecall => "mixed_generation_recall",
            Self::TenantBoundaryUndisclosed => "tenant_boundary_undisclosed",
            Self::PolicyHiddenOmissionUndisclosed => "policy_hidden_omission_undisclosed",
            Self::SignedPackCompatibilityUndisclosed => "signed_pack_compatibility_undisclosed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the retrieval inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalInspectorFinding {
    /// Closed finding kind.
    pub finding_kind: RetrievalInspectorFindingKind,
    /// Finding severity.
    pub severity: RetrievalInspectorFindingSeverity,
    /// Short support-safe explanation.
    pub summary: String,
}

impl RetrievalInspectorFinding {
    fn new(
        finding_kind: RetrievalInspectorFindingKind,
        severity: RetrievalInspectorFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Embedding-index manifest carried by vector retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddingIndexManifest {
    /// Stable manifest id.
    pub manifest_id: String,
    /// Workspace identity ref used to isolate derived vectors.
    pub workspace_id_ref: String,
    /// Snapshot or revision ref used to derive the index.
    pub snapshot_ref: String,
    /// Retrieval epoch for this embedder/chunker/trust-boundary generation.
    pub retrieval_epoch: String,
    /// Embedder model identity.
    pub embedder_model_id: String,
    /// Embedder model version or digest.
    pub embedder_model_version: String,
    /// Tokenizer identity used by the embedder.
    pub tokenizer_id: String,
    /// Chunker strategy identity.
    pub chunker_id: String,
    /// Trust boundary that scopes this index.
    pub trust_boundary_ref: String,
    /// Locality of this embedding index.
    pub locality: RetrievalLocalityClass,
    /// Current state of this embedding index.
    pub state: EmbeddingIndexStateClass,
    /// Policy scope that admitted or denied this index.
    pub policy_scope_ref: String,
    /// Tenant scope for managed or remote embedding indexes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_scope_ref: Option<String>,
    /// Region or residency ref for managed or remote embedding indexes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_ref: Option<String>,
    /// Retention policy used when deriving and caching embeddings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_policy_id: Option<String>,
    /// Compatibility or downgrade disclosure for precomputed embedding packs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_ref: Option<String>,
    /// Signed pack ref when the index came from a mirrored pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_pack_ref: Option<String>,
}

impl EmbeddingIndexManifest {
    fn matches_contribution(&self, contribution: &RetrievalContribution) -> bool {
        contribution
            .retrieval_epoch
            .as_deref()
            .map_or(false, |epoch| epoch == self.retrieval_epoch)
            && contribution
                .embedder_model_id
                .as_deref()
                .map_or(false, |model| model == self.embedder_model_id)
            && contribution
                .embedder_model_version
                .as_deref()
                .map_or(true, |version| version == self.embedder_model_version)
            && contribution
                .tokenizer_id
                .as_deref()
                .map_or(true, |tokenizer| tokenizer == self.tokenizer_id)
            && contribution
                .chunker_id
                .as_deref()
                .map_or(true, |chunker| chunker == self.chunker_id)
            && contribution
                .trust_boundary_ref
                .as_deref()
                .map_or(true, |boundary| boundary == self.trust_boundary_ref)
            && contribution
                .retention_policy_id
                .as_deref()
                .map_or(true, |retention| {
                    self.retention_policy_id
                        .as_deref()
                        .map_or(false, |manifest_retention| retention == manifest_retention)
                })
    }

    fn epoch_key(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.retrieval_epoch,
            self.embedder_model_id,
            self.embedder_model_version,
            self.tokenizer_id,
            self.chunker_id,
            self.retention_policy_id.as_deref().unwrap_or("")
        )
    }

    fn is_invalidated_for_stable(&self) -> bool {
        matches!(
            self.state,
            EmbeddingIndexStateClass::Stale
                | EmbeddingIndexStateClass::IncompatibleEpoch
                | EmbeddingIndexStateClass::Unavailable
        )
    }

    fn requires_tenant_boundary(&self) -> bool {
        matches!(
            self.locality,
            RetrievalLocalityClass::ManagedTenantScoped | RetrievalLocalityClass::ProviderRemote
        )
    }

    fn has_tenant_boundary(&self) -> bool {
        self.tenant_scope_ref
            .as_deref()
            .map_or(false, |value| !value.trim().is_empty())
            && self
                .region_ref
                .as_deref()
                .map_or(false, |value| !value.trim().is_empty())
            && !self.policy_scope_ref.trim().is_empty()
    }

    fn requires_pack_compatibility(&self) -> bool {
        matches!(self.locality, RetrievalLocalityClass::MirroredPack)
    }

    fn has_pack_compatibility(&self) -> bool {
        self.signed_pack_ref
            .as_deref()
            .map_or(false, |value| !value.trim().is_empty())
            && self
                .compatibility_ref
                .as_deref()
                .map_or(false, |value| !value.trim().is_empty())
    }
}

/// Local-first policy disclosure attached to a retrieval packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalFirstPolicyDisclosure {
    /// Stable policy ref used for route decisions.
    pub policy_ref: String,
    /// Route policy class observed for this packet.
    pub route_policy: RetrievalRoutePolicyClass,
    /// Preferred locality before fallbacks were considered.
    pub preferred_locality: RetrievalLocalityClass,
    /// Locality that actually served the selected context.
    pub active_locality: RetrievalLocalityClass,
    /// Fallback reason when local-first routing degraded or withheld a lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<RetrievalFallbackReasonClass>,
    /// True when remote or managed routing is visible to consumers.
    pub remote_route_disclosed: bool,
    /// True only when the retrieval path may write through a provider.
    pub provider_write_allowed: bool,
    /// Support-safe explanation label shown in inspector/export surfaces.
    pub explanation_label: String,
}

impl LocalFirstPolicyDisclosure {
    fn requires_fallback_reason(&self) -> bool {
        matches!(
            self.route_policy,
            RetrievalRoutePolicyClass::LocalFirstRemoteFallback
                | RetrievalRoutePolicyClass::RemoteAllowedByPolicy
                | RetrievalRoutePolicyClass::RemoteBlockedByPolicy
                | RetrievalRoutePolicyClass::NoUsableLane
        )
    }
}

/// Snapshot of one retrieval lane considered by the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLaneSnapshot {
    /// Stable lane id.
    pub lane_id: String,
    /// Lane class.
    pub lane_class: RetrievalLaneClass,
    /// Source snapshot or producer id.
    pub source_snapshot_id: String,
    /// Locality of this lane.
    pub locality: RetrievalLocalityClass,
    /// Readiness of this lane.
    pub readiness: RetrievalReadinessClass,
    /// Freshness of this lane.
    pub freshness: RetrievalFreshnessClass,
    /// Retrieval epoch when the lane uses indexed derived data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_epoch: Option<String>,
    /// Embedder model id when this is a vector lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedder_model_id: Option<String>,
    /// Graph epoch when this is a graph lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Policy scope used to admit or deny the lane.
    pub policy_scope_ref: String,
    /// Route policy observed for this lane.
    pub route_policy: RetrievalRoutePolicyClass,
    /// Fallback reason when this lane degraded or was withheld.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<RetrievalFallbackReasonClass>,
    /// Number of candidate rows available on this lane.
    pub candidate_count: u32,
    /// Path-level partial-truth causes.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
}

impl RetrievalLaneSnapshot {
    /// Builds a retrieval lane snapshot from an existing planner path snapshot.
    pub fn from_planner_path_snapshot(
        lane_id: impl Into<String>,
        snapshot: &crate::planner::PlannerPathSnapshot,
        locality: RetrievalLocalityClass,
        policy_scope_ref: impl Into<String>,
        route_policy: RetrievalRoutePolicyClass,
    ) -> Self {
        Self {
            lane_id: lane_id.into(),
            lane_class: RetrievalLaneClass::from_planner_path(snapshot.path_kind),
            source_snapshot_id: snapshot.snapshot_id.clone(),
            locality,
            readiness: RetrievalReadinessClass::from_planner_readiness(snapshot.readiness),
            freshness: RetrievalFreshnessClass::from_planner_freshness(snapshot.freshness),
            retrieval_epoch: snapshot.index_epoch.clone(),
            embedder_model_id: None,
            graph_epoch: snapshot.graph_epoch.clone(),
            policy_scope_ref: policy_scope_ref.into(),
            route_policy,
            fallback_reason: None,
            candidate_count: snapshot.rows.len() as u32,
            partial_truth_causes: snapshot.partial_truth_causes.clone(),
        }
    }
}

/// Metadata-safe target anchor for one retrieval contribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalProvenanceAnchor {
    /// Target kind.
    pub anchor_kind: RetrievalAnchorKind,
    /// Opaque target ref, such as a file, symbol, doc, artifact, or graph node.
    pub target_ref: String,
    /// Chunk, range, node, or edge ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_or_node_ref: Option<String>,
}

/// One lane contribution to a retrieval-inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalContribution {
    /// Lane id this contribution came from.
    pub lane_id_ref: String,
    /// Lane class.
    pub lane_class: RetrievalLaneClass,
    /// Role this lane played in the fused row.
    pub contribution_role: RetrievalContributionRole,
    /// Locality of this contribution.
    pub locality: RetrievalLocalityClass,
    /// Readiness of this contribution.
    pub readiness: RetrievalReadinessClass,
    /// Freshness of this contribution.
    pub freshness: RetrievalFreshnessClass,
    /// Metadata-safe source anchor.
    pub provenance: RetrievalProvenanceAnchor,
    /// Ranking or matching reasons contributed by this lane.
    #[serde(default)]
    pub ranking_reasons: Vec<RetrievalReasonClass>,
    /// Retrieval epoch when indexed data contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_epoch: Option<String>,
    /// Embedder model id when vector recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedder_model_id: Option<String>,
    /// Embedder model version or digest when embedding recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedder_model_version: Option<String>,
    /// Tokenizer id when embedding recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer_id: Option<String>,
    /// Chunking strategy id when embedding recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunker_id: Option<String>,
    /// Trust boundary ref when embedding recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_boundary_ref: Option<String>,
    /// Retention policy id when embedding recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_policy_id: Option<String>,
    /// Graph epoch when graph recall contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Partial-truth causes specific to this contribution.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
}

impl RetrievalContribution {
    fn uses_embedding_recall(&self) -> bool {
        matches!(
            self.lane_class,
            RetrievalLaneClass::Vector | RetrievalLaneClass::Embedding
        )
    }
}

/// One lane or candidate class omitted from the visible retrieval packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLaneOmission {
    /// Stable omission id.
    pub omission_id: String,
    /// Omitted lane class.
    pub omitted_lane_class: RetrievalLaneClass,
    /// Support-safe reason the lane was omitted.
    pub omission_reason: String,
    /// True when policy hid the omitted candidates.
    pub policy_hidden: bool,
    /// Disclosure ref shown to consumers when policy hides candidates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
}

/// One fused row visible in the retrieval inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalInspectorRow {
    /// Stable row id for the inspector packet.
    pub row_id: String,
    /// Canonical target id after fusion/deduplication.
    pub canonical_id: String,
    /// User-visible label safe for support and docs.
    pub display_title: String,
    /// Search result or AI context row ref that rendered this row.
    pub rendered_result_ref: String,
    /// Selected lane class for the row.
    pub selected_lane_class: RetrievalLaneClass,
    /// Locality that answered the row.
    pub selected_locality: RetrievalLocalityClass,
    /// Row readiness.
    pub readiness: RetrievalReadinessClass,
    /// Per-lane contributions that built the row.
    #[serde(default)]
    pub contributions: Vec<RetrievalContribution>,
    /// Short explanation shown by the retrieval inspector.
    pub explanation: String,
    /// True when mutations must re-resolve targets against live state.
    pub mutating_actions_require_live_resolution: bool,
}

impl RetrievalInspectorRow {
    /// Builds a metadata-only inspector row from a planned search result.
    pub fn from_planned_result(
        result: &PlannedSearchResult,
        default_locality: RetrievalLocalityClass,
    ) -> Self {
        let selected_lane_class = RetrievalLaneClass::from_planner_path(result.answered_by);
        let readiness = RetrievalReadinessClass::from_planner_readiness(result.readiness_state);
        let contributions = result
            .contributions
            .iter()
            .map(|contribution| {
                contribution_from_planner_result(contribution, result, default_locality)
            })
            .collect();

        Self {
            row_id: result.result_id.clone(),
            canonical_id: result.canonical_id.clone(),
            display_title: result.title.clone(),
            rendered_result_ref: result.result_id.clone(),
            selected_lane_class,
            selected_locality: default_locality,
            readiness,
            contributions,
            explanation: result.explanation.summary.clone(),
            mutating_actions_require_live_resolution: true,
        }
    }
}

/// Consumer projection proving a surface reads the same retrieval packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: RetrievalConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by this projection.
    pub retrieval_packet_id_ref: String,
    /// Absolute or monotonic timestamp for the projection.
    pub rendered_at: String,
    /// True when this projection preserves the packet id instead of reminting truth.
    pub preserves_same_packet: bool,
    /// True when raw private material is excluded from the projection.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials or authority are excluded.
    pub ambient_authority_excluded: bool,
}

/// Constructor input for [`RetrievalInspectorPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalInspectorPacketInput {
    /// Validation profile requested by the producer.
    #[serde(default)]
    pub governance_track: RetrievalGovernanceTrack,
    /// Query class that selected the retrieval lane mix.
    #[serde(default)]
    pub query_class: RetrievalQueryClass,
    /// Stable packet id.
    pub packet_id: String,
    /// Query session id that produced the rows.
    pub query_session_id_ref: String,
    /// Planner pass id that produced the rows.
    pub planner_pass_id_ref: String,
    /// Result set id that produced the rows.
    pub result_set_id_ref: String,
    /// Primary consumer surface for this packet.
    pub consumer_surface: RetrievalConsumerSurface,
    /// Capture timestamp.
    pub captured_at: String,
    /// Local-first policy disclosure.
    pub local_first_policy: LocalFirstPolicyDisclosure,
    /// Embedding indexes referenced by vector contributions.
    #[serde(default)]
    pub embedding_indexes: Vec<EmbeddingIndexManifest>,
    /// Lane snapshots considered by the retrieval inspector.
    #[serde(default)]
    pub lane_snapshots: Vec<RetrievalLaneSnapshot>,
    /// Omitted lanes, withheld candidate classes, and policy-hidden classes.
    #[serde(default)]
    pub omissions: Vec<RetrievalLaneOmission>,
    /// Fused inspector rows.
    #[serde(default)]
    pub rows: Vec<RetrievalInspectorRow>,
    /// Consumer projections that preserve the same packet.
    #[serde(default)]
    pub consumer_projections: Vec<RetrievalConsumerProjection>,
}

/// Hybrid retrieval packet shown in product and exported by consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalInspectorPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Validation profile requested by the producer.
    #[serde(default)]
    pub governance_track: RetrievalGovernanceTrack,
    /// Query class that selected the retrieval lane mix.
    #[serde(default)]
    pub query_class: RetrievalQueryClass,
    /// Stable packet id.
    pub packet_id: String,
    /// Query session id that produced the rows.
    pub query_session_id_ref: String,
    /// Planner pass id that produced the rows.
    pub planner_pass_id_ref: String,
    /// Result set id that produced the rows.
    pub result_set_id_ref: String,
    /// Primary consumer surface for this packet.
    pub consumer_surface: RetrievalConsumerSurface,
    /// Capture timestamp.
    pub captured_at: String,
    /// Local-first policy disclosure.
    pub local_first_policy: LocalFirstPolicyDisclosure,
    /// Embedding indexes referenced by vector contributions.
    #[serde(default)]
    pub embedding_indexes: Vec<EmbeddingIndexManifest>,
    /// Lane snapshots considered by the retrieval inspector.
    #[serde(default)]
    pub lane_snapshots: Vec<RetrievalLaneSnapshot>,
    /// Omitted lanes, withheld candidate classes, and policy-hidden classes.
    #[serde(default)]
    pub omissions: Vec<RetrievalLaneOmission>,
    /// Fused inspector rows.
    #[serde(default)]
    pub rows: Vec<RetrievalInspectorRow>,
    /// Consumer projections that preserve the same packet.
    #[serde(default)]
    pub consumer_projections: Vec<RetrievalConsumerProjection>,
    /// Derived promotion state for claimed beta rows.
    pub promotion_state: RetrievalPromotionState,
    /// Validation findings captured when the packet was materialized.
    #[serde(default)]
    pub validation_findings: Vec<RetrievalInspectorFinding>,
}

impl RetrievalInspectorPacket {
    /// Materializes a retrieval packet and records derived validation findings.
    pub fn materialize(input: RetrievalInspectorPacketInput) -> Self {
        let mut packet = Self {
            record_kind: RETRIEVAL_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: RETRIEVAL_INSPECTOR_SCHEMA_VERSION,
            governance_track: input.governance_track,
            query_class: input.query_class,
            packet_id: input.packet_id,
            query_session_id_ref: input.query_session_id_ref,
            planner_pass_id_ref: input.planner_pass_id_ref,
            result_set_id_ref: input.result_set_id_ref,
            consumer_surface: input.consumer_surface,
            captured_at: input.captured_at,
            local_first_policy: input.local_first_policy,
            embedding_indexes: input.embedding_indexes,
            lane_snapshots: input.lane_snapshots,
            omissions: input.omissions,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            promotion_state: RetrievalPromotionState::Promotable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Builds retrieval-inspector input from an existing planner output.
    ///
    /// Planner-derived input carries lexical and graph contributions that the
    /// planner already knows about. Callers can append vector lane snapshots,
    /// embedding manifests, and vector contributions before calling
    /// [`Self::materialize`] when embedding recall contributed.
    pub fn input_from_planner_output(
        packet_id: impl Into<String>,
        consumer_surface: RetrievalConsumerSurface,
        captured_at: impl Into<String>,
        planner_output: &SearchPlannerOutput,
        local_first_policy: LocalFirstPolicyDisclosure,
        default_locality: RetrievalLocalityClass,
    ) -> RetrievalInspectorPacketInput {
        RetrievalInspectorPacketInput {
            governance_track: RetrievalGovernanceTrack::Beta,
            query_class: RetrievalQueryClass::Unclassified,
            packet_id: packet_id.into(),
            query_session_id_ref: planner_output.query_session.query_session_id.clone(),
            planner_pass_id_ref: planner_output.planner_pass.planner_pass_id.clone(),
            result_set_id_ref: planner_output.result_set.result_set_id.clone(),
            consumer_surface,
            captured_at: captured_at.into(),
            local_first_policy,
            embedding_indexes: Vec::new(),
            lane_snapshots: Vec::new(),
            omissions: Vec::new(),
            rows: planner_output
                .result_set
                .rows
                .iter()
                .map(|row| RetrievalInspectorRow::from_planned_result(row, default_locality))
                .collect(),
            consumer_projections: Vec::new(),
        }
    }

    /// Builds a retrieval packet from an existing planner output.
    ///
    /// This is useful for planner-only inspectability. Claimed hybrid beta
    /// rows still need vector lane snapshots, embedding manifests, and export
    /// projections before they can validate as promotable.
    pub fn from_planner_output(
        packet_id: impl Into<String>,
        consumer_surface: RetrievalConsumerSurface,
        captured_at: impl Into<String>,
        planner_output: &SearchPlannerOutput,
        local_first_policy: LocalFirstPolicyDisclosure,
        default_locality: RetrievalLocalityClass,
    ) -> Self {
        Self::materialize(Self::input_from_planner_output(
            packet_id,
            consumer_surface,
            captured_at,
            planner_output,
            local_first_policy,
            default_locality,
        ))
    }

    /// Re-validates the packet against the beta retrieval-inspector invariants.
    pub fn validate(&self) -> Vec<RetrievalInspectorFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_promotable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == RetrievalInspectorFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: RetrievalConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.retrieval_packet_id_ref == self.packet_id
                && projection.preserves_same_packet
                && projection.raw_private_material_excluded
                && projection.ambient_authority_excluded
        })
    }

    /// Returns the lane-class tokens that contributed to visible rows.
    pub fn contributing_lane_tokens(&self) -> Vec<&'static str> {
        let mut lanes = BTreeSet::new();
        for row in &self.rows {
            for contribution in &row.contributions {
                lanes.insert(contribution.lane_class);
            }
        }
        lanes.into_iter().map(RetrievalLaneClass::as_str).collect()
    }

    /// Returns lane-class tokens declared by stable lane snapshots and selected rows.
    pub fn declared_lane_tokens(&self) -> Vec<&'static str> {
        let mut lanes = BTreeSet::new();
        for lane in &self.lane_snapshots {
            lanes.insert(lane.lane_class);
        }
        for row in &self.rows {
            lanes.insert(row.selected_lane_class);
        }
        lanes.into_iter().map(RetrievalLaneClass::as_str).collect()
    }

    /// Builds a support export that embeds the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RetrievalInspectorSupportExport {
        RetrievalInspectorSupportExport {
            record_kind: RETRIEVAL_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RETRIEVAL_INSPECTOR_SCHEMA_VERSION,
            export_id: export_id.into(),
            retrieval_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            inspector_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<RetrievalInspectorFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != RETRIEVAL_INSPECTOR_RECORD_KIND {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::WrongRecordKind,
                RetrievalInspectorFindingSeverity::Blocker,
                "retrieval inspector packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != RETRIEVAL_INSPECTOR_SCHEMA_VERSION {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::WrongSchemaVersion,
                RetrievalInspectorFindingSeverity::Blocker,
                "retrieval inspector packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.query_session_id_ref.trim().is_empty()
            || self.planner_pass_id_ref.trim().is_empty()
            || self.result_set_id_ref.trim().is_empty()
        {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::MissingIdentity,
                RetrievalInspectorFindingSeverity::Blocker,
                "packet, query-session, planner-pass, and result-set refs are required",
            ));
        }

        for required_lane in [
            RetrievalLaneClass::Lexical,
            RetrievalLaneClass::Vector,
            RetrievalLaneClass::Graph,
        ] {
            let lane_present = self.rows.iter().any(|row| {
                row.contributions
                    .iter()
                    .any(|contribution| match required_lane {
                        RetrievalLaneClass::Vector => contribution.uses_embedding_recall(),
                        _ => contribution.lane_class == required_lane,
                    })
            });
            if !lane_present {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::MissingHybridLane,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "hybrid retrieval packet is missing a {} contribution",
                        required_lane.as_str()
                    ),
                ));
            }
        }

        if self.governance_track == RetrievalGovernanceTrack::Stable {
            self.push_stable_findings(&mut findings);
        }

        if self.local_first_policy.provider_write_allowed {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::MutatingActionWithoutLiveResolution,
                RetrievalInspectorFindingSeverity::Blocker,
                "retrieval packets must not permit provider writes",
            ));
        }
        if self.local_first_policy.active_locality.is_remote()
            && !self.local_first_policy.remote_route_disclosed
        {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::UnlabelledRemoteRoute,
                RetrievalInspectorFindingSeverity::Blocker,
                "remote or managed retrieval must be visibly disclosed",
            ));
        }
        if self.local_first_policy.requires_fallback_reason()
            && self.local_first_policy.fallback_reason.is_none()
        {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::UnlabelledRemoteRoute,
                RetrievalInspectorFindingSeverity::Warning,
                "fallback or remote routing requires a visible fallback reason",
            ));
        }

        for lane in &self.lane_snapshots {
            if lane.readiness.requires_visible_caveat() && lane.partial_truth_causes.is_empty() {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::UnlabelledPartialIndex,
                    RetrievalInspectorFindingSeverity::Warning,
                    format!("lane {} is degraded without a visible cause", lane.lane_id),
                ));
            }
        }

        for row in &self.rows {
            if !row.mutating_actions_require_live_resolution {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::MutatingActionWithoutLiveResolution,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "row {} can be used for mutation without live re-resolution",
                        row.row_id
                    ),
                ));
            }
            for contribution in &row.contributions {
                if contribution.locality.is_remote()
                    && !self.local_first_policy.remote_route_disclosed
                {
                    findings.push(RetrievalInspectorFinding::new(
                        RetrievalInspectorFindingKind::UnlabelledRemoteRoute,
                        RetrievalInspectorFindingSeverity::Blocker,
                        format!(
                            "remote contribution {} is missing route disclosure",
                            contribution.lane_id_ref
                        ),
                    ));
                }
                if contribution.readiness.requires_visible_caveat()
                    && contribution.partial_truth_causes.is_empty()
                {
                    findings.push(RetrievalInspectorFinding::new(
                        RetrievalInspectorFindingKind::UnlabelledPartialIndex,
                        RetrievalInspectorFindingSeverity::Warning,
                        format!(
                            "contribution {} is degraded without a visible cause",
                            contribution.lane_id_ref
                        ),
                    ));
                }
                if contribution.uses_embedding_recall()
                    && self.matching_embedding_manifest(contribution).is_none()
                {
                    findings.push(RetrievalInspectorFinding::new(
                        RetrievalInspectorFindingKind::MissingEmbeddingManifest,
                        RetrievalInspectorFindingSeverity::Blocker,
                        format!(
                            "vector contribution {} lacks a matching embedding manifest",
                            contribution.lane_id_ref
                        ),
                    ));
                }
            }
        }

        for required_surface in [
            self.consumer_surface,
            RetrievalConsumerSurface::SupportExport,
        ] {
            if !self.has_projection_for(required_surface) {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::MissingExportProjection,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != RetrievalInspectorFindingKind::PromotionStateMismatch
            });
            let derived_promotion = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived_promotion {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::PromotionStateMismatch,
                    RetrievalInspectorFindingSeverity::Blocker,
                    "stored promotion state does not match derived validation findings",
                ));
            }
        }

        findings
    }

    fn push_stable_findings(&self, findings: &mut Vec<RetrievalInspectorFinding>) {
        if self.query_class == RetrievalQueryClass::Unclassified {
            findings.push(RetrievalInspectorFinding::new(
                RetrievalInspectorFindingKind::MissingQueryClassification,
                RetrievalInspectorFindingSeverity::Blocker,
                "stable retrieval packets must classify queries as exact, structural, conceptual, or mixed",
            ));
        }

        let declared_lanes: BTreeSet<RetrievalLaneClass> = self
            .lane_snapshots
            .iter()
            .map(|lane| lane.lane_class)
            .chain(self.rows.iter().map(|row| row.selected_lane_class))
            .chain(self.rows.iter().flat_map(|row| {
                row.contributions
                    .iter()
                    .map(|contribution| contribution.lane_class)
            }))
            .collect();
        for required_lane in [
            RetrievalLaneClass::Lexical,
            RetrievalLaneClass::Structural,
            RetrievalLaneClass::Graph,
            RetrievalLaneClass::Embedding,
            RetrievalLaneClass::Fused,
        ] {
            if !declared_lanes.contains(&required_lane) {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::MissingStableLane,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "stable retrieval packet is missing the {} lane",
                        required_lane.as_str()
                    ),
                ));
            }
        }

        for manifest in &self.embedding_indexes {
            if manifest.is_invalidated_for_stable() {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::EmbeddingEpochInvalidated,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "embedding manifest {} cannot publish as current stable recall",
                        manifest.manifest_id
                    ),
                ));
            }
            if manifest.requires_tenant_boundary() && !manifest.has_tenant_boundary() {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::TenantBoundaryUndisclosed,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "managed embedding manifest {} must disclose tenant, region, and policy boundary",
                        manifest.manifest_id
                    ),
                ));
            }
            if manifest.requires_pack_compatibility() && !manifest.has_pack_compatibility() {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::SignedPackCompatibilityUndisclosed,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "mirrored embedding manifest {} must disclose signature and compatibility refs",
                        manifest.manifest_id
                    ),
                ));
            }
        }

        for omission in &self.omissions {
            if omission.policy_hidden
                && omission
                    .disclosure_ref
                    .as_deref()
                    .map_or(true, |value| value.trim().is_empty())
            {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::PolicyHiddenOmissionUndisclosed,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "policy-hidden omission {} lacks a disclosure ref",
                        omission.omission_id
                    ),
                ));
            }
        }

        for row in &self.rows {
            let mut embedding_epoch_keys = BTreeSet::new();
            for contribution in &row.contributions {
                if contribution.uses_embedding_recall() {
                    if let Some(manifest) = self.matching_embedding_manifest(contribution) {
                        embedding_epoch_keys.insert(manifest.epoch_key());
                    }
                }
            }
            if embedding_epoch_keys.len() > 1 {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::MixedGenerationRecall,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!("row {} mixes embedding generations", row.row_id),
                ));
            }
        }

        for required_surface in [
            RetrievalConsumerSurface::SearchResults,
            RetrievalConsumerSurface::AiContext,
            RetrievalConsumerSurface::ReviewWorkspace,
            RetrievalConsumerSurface::DocsHelp,
            RetrievalConsumerSurface::SupportExport,
        ] {
            if !self.has_projection_for(required_surface) {
                findings.push(RetrievalInspectorFinding::new(
                    RetrievalInspectorFindingKind::MissingExportProjection,
                    RetrievalInspectorFindingSeverity::Blocker,
                    format!(
                        "stable packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
    }

    fn matching_embedding_manifest(
        &self,
        contribution: &RetrievalContribution,
    ) -> Option<&EmbeddingIndexManifest> {
        self.embedding_indexes
            .iter()
            .find(|manifest| manifest.matches_contribution(contribution))
    }
}

/// Support-export wrapper that preserves the product retrieval packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalInspectorSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Retrieval packet id preserved by this export.
    pub retrieval_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials or authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact retrieval packet shown in product.
    pub inspector_packet: RetrievalInspectorPacket,
}

impl RetrievalInspectorSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == RETRIEVAL_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == RETRIEVAL_INSPECTOR_SCHEMA_VERSION
            && self.retrieval_packet_id_ref == self.inspector_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.inspector_packet.validate().is_empty()
    }
}

fn contribution_from_planner_result(
    contribution: &PlannerContribution,
    result: &PlannedSearchResult,
    default_locality: RetrievalLocalityClass,
) -> RetrievalContribution {
    RetrievalContribution {
        lane_id_ref: contribution.snapshot_id.clone(),
        lane_class: RetrievalLaneClass::from_planner_path(contribution.path_kind),
        contribution_role: role_from_planner_decision(contribution.role),
        locality: default_locality,
        readiness: RetrievalReadinessClass::from_planner_readiness(contribution.readiness),
        freshness: RetrievalFreshnessClass::Unknown,
        provenance: RetrievalProvenanceAnchor {
            anchor_kind: RetrievalAnchorKind::from_planner_target(result.target_kind),
            target_ref: result.canonical_id.clone(),
            chunk_or_node_ref: result
                .symbol_ref
                .clone()
                .or_else(|| result.relative_path.clone()),
        },
        ranking_reasons: contribution
            .ranking_reasons
            .iter()
            .map(|reason| RetrievalReasonClass::from_planner_reason(*reason))
            .collect(),
        retrieval_epoch: None,
        embedder_model_id: None,
        embedder_model_version: None,
        tokenizer_id: None,
        chunker_id: None,
        trust_boundary_ref: None,
        retention_policy_id: None,
        graph_epoch: None,
        partial_truth_causes: contribution.partial_truth_causes.clone(),
    }
}

fn role_from_planner_decision(
    role: crate::planner::PlannerPathDecisionClass,
) -> RetrievalContributionRole {
    match role {
        crate::planner::PlannerPathDecisionClass::SelectedPrimary => {
            RetrievalContributionRole::Primary
        }
        crate::planner::PlannerPathDecisionClass::SelectedFallback => {
            RetrievalContributionRole::Fallback
        }
        crate::planner::PlannerPathDecisionClass::SelectedSupplementary => {
            RetrievalContributionRole::Supplementary
        }
        crate::planner::PlannerPathDecisionClass::UnavailableDisclosed
        | crate::planner::PlannerPathDecisionClass::SkippedNoRows
        | crate::planner::PlannerPathDecisionClass::SkippedSurfaceIneligible => {
            RetrievalContributionRole::Withheld
        }
    }
}

fn promotion_state_for_findings(findings: &[RetrievalInspectorFinding]) -> RetrievalPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == RetrievalInspectorFindingSeverity::Blocker)
    {
        RetrievalPromotionState::Blocked
    } else if findings
        .iter()
        .any(|finding| finding.severity == RetrievalInspectorFindingSeverity::Warning)
    {
        RetrievalPromotionState::NeedsReview
    } else {
        RetrievalPromotionState::Promotable
    }
}

/// Errors emitted when reading the checked-in stable hybrid-retrieval packet.
#[derive(Debug)]
pub enum HybridRetrievalStableArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<RetrievalInspectorFinding>),
}

impl fmt::Display for HybridRetrievalStableArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "hybrid retrieval packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "hybrid retrieval packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for HybridRetrievalStableArtifactError {}

/// Returns the checked-in stable hybrid-retrieval inspector packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_hybrid_retrieval_inspector_packet(
) -> Result<RetrievalInspectorPacket, HybridRetrievalStableArtifactError> {
    let packet: RetrievalInspectorPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/hybrid_retrieval_inspector_packet.json"
    )))
    .map_err(HybridRetrievalStableArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(HybridRetrievalStableArtifactError::Validation(findings))
    }
}
