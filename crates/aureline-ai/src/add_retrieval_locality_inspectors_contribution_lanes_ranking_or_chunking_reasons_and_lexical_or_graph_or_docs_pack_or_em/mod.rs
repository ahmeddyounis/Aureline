//! Retrieval-locality inspectors with labeled contribution lanes,
//! ranking-or-chunking reasons, and lexical/graph/docs-pack/embedding/
//! provider-overlay labeling across search, docs recall, and AI context packs.
//!
//! Where the frozen recall matrix
//! ([`crate::freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix`])
//! qualifies a whole surface, this module inspects a single produced recall
//! result — one search response, one docs-recall pane, or one assembled AI
//! context pack — and explains *how it was built*. Each
//! [`ContributionLaneRow`] names which lane contributed (lexical, graph,
//! docs-pack, embedding, or provider-overlay), whether it contributed cleanly
//! or degraded, why each candidate was selected (a ranking *or* chunking
//! reason), where the data lived ([`RetrievalLocalityClass`]), and which
//! retrieval generation produced it.
//!
//! The packet keeps in-product labels and exported labels identical: a
//! [`RetrievalInspectorSurfaceRow`] records the hidden-scope count, the degraded
//! lanes, and the provider-overlay posture exactly as the user saw them, and
//! [`RetrievalLocalityInspectorPacket::validate`] refuses to let any of those
//! disclosures drift. A surface that hides scope or carries a degraded lane may
//! not claim [`CompletenessClass::Complete`]; a contributing provider-overlay
//! lane must be disclosed by the surface's overlay posture; a stale or
//! recomputing lane must show as degraded rather than masquerade as current; and
//! any surface a support export or replay packet can reach must preserve its
//! lane and locality labels.
//!
//! Raw query bodies, document bodies, raw embeddings, raw provider payloads,
//! credentials, exact scores, and exact token or cost amounts stay outside the
//! support boundary — only labels, classes, and counts cross it.
//!
//! The boundary schema is
//! [`schemas/ai/add-retrieval-locality-inspectors-contribution-lanes-ranking-or-chunking-reasons-and-lexical-or-graph-or-docs-pack-or-em.schema.json`](../../../../schemas/ai/add-retrieval-locality-inspectors-contribution-lanes-ranking-or-chunking-reasons-and-lexical-or-graph-or-docs-pack-or-em.schema.json).
//! The contract doc is
//! [`docs/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em.md`](../../../../docs/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/`](../../../../fixtures/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RetrievalLocalityInspectorPacket`].
pub const RETRIEVAL_LOCALITY_INSPECTOR_RECORD_KIND: &str = "retrieval_locality_inspector";

/// Schema version for retrieval-locality inspector records.
pub const RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF: &str =
    "schemas/ai/add-retrieval-locality-inspectors-contribution-lanes-ranking-or-chunking-reasons-and-lexical-or-graph-or-docs-pack-or-em.schema.json";

/// Repo-relative path of the contract doc.
pub const RETRIEVAL_LOCALITY_INSPECTOR_DOC_REF: &str =
    "docs/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em.md";

/// Repo-relative path of the frozen result-identity / ranking contract.
pub const RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF: &str =
    "docs/search/result_identity_and_ranking.md";

/// Repo-relative path of the frozen search-explainability contract.
pub const RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF: &str =
    "docs/search/search_explainability_contract.md";

/// Repo-relative path of the frozen AI context-assembly contract.
pub const RETRIEVAL_LOCALITY_INSPECTOR_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen spend/route receipt contract (provider overlay).
pub const RETRIEVAL_LOCALITY_INSPECTOR_SPEND_RECEIPT_CONTRACT_REF: &str =
    "docs/ai/spend_and_route_receipt_contract.md";

/// Repo-relative path of the upstream frozen recall matrix this inspector projects.
pub const RETRIEVAL_LOCALITY_INSPECTOR_RECALL_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const RETRIEVAL_LOCALITY_INSPECTOR_FIXTURE_DIR: &str =
    "fixtures/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em";

/// Repo-relative path of the checked support-export artifact.
pub const RETRIEVAL_LOCALITY_INSPECTOR_ARTIFACT_REF: &str =
    "artifacts/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RETRIEVAL_LOCALITY_INSPECTOR_SUMMARY_REF: &str =
    "artifacts/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em.md";

/// A recall-producing surface this inspector explains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalInspectorSurface {
    /// Workspace search results.
    Search,
    /// Docs and in-app browser recall pane.
    DocsRecall,
    /// Assembled AI context pack handed to a model.
    AiContextPack,
}

impl RetrievalInspectorSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::Search, Self::DocsRecall, Self::AiContextPack];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::DocsRecall => "docs_recall",
            Self::AiContextPack => "ai_context_pack",
        }
    }
}

/// Lane that contributed candidates to a recall result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributionLaneClass {
    /// Lexical / keyword lane.
    LexicalKeyword,
    /// Graph-traversal lane over the workspace knowledge graph.
    GraphTraversal,
    /// Docs-pack lane drawing from indexed documentation packs.
    DocsPack,
    /// Embedding-vector (semantic) lane.
    EmbeddingVector,
    /// Provider-overlay lane that merges results from a remote provider.
    ProviderOverlay,
}

impl ContributionLaneClass {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LexicalKeyword,
        Self::GraphTraversal,
        Self::DocsPack,
        Self::EmbeddingVector,
        Self::ProviderOverlay,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalKeyword => "lexical_keyword",
            Self::GraphTraversal => "graph_traversal",
            Self::DocsPack => "docs_pack",
            Self::EmbeddingVector => "embedding_vector",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Whether and how a contribution lane took part in a recall result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributionState {
    /// Lane contributed candidates cleanly.
    Contributed,
    /// Lane contributed but is degraded (stale, partial, or recomputing).
    Degraded,
    /// Lane ran but returned nothing.
    Empty,
    /// Lane was suppressed by policy, scope, or budget.
    Suppressed,
}

impl ContributionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Contributed => "contributed",
            Self::Degraded => "degraded",
            Self::Empty => "empty",
            Self::Suppressed => "suppressed",
        }
    }

    /// Whether the lane actually put candidates into the result.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Contributed | Self::Degraded)
    }
}

/// Whether a lane explains its candidates by ranking, by chunking, or by overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionReasonKind {
    /// Candidates were ranked and selected by a score.
    Ranking,
    /// Candidates were chunked from larger sources.
    Chunking,
    /// Candidates were merged from a remote provider overlay.
    Overlay,
    /// Lane did not contribute, so no selection reason applies.
    NotApplicable,
}

impl SelectionReasonKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ranking => "ranking",
            Self::Chunking => "chunking",
            Self::Overlay => "overlay",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Concrete ranking-or-chunking reason a lane used to pick its candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionReasonClass {
    /// Ranked by semantic (embedding) similarity.
    RankBySemanticSimilarity,
    /// Ranked by a lexical relevance score.
    RankByLexicalScore,
    /// Ranked by proximity in the workspace graph.
    RankByGraphProximity,
    /// Ranked by hybrid fusion of multiple lanes.
    RankByHybridFusion,
    /// Chunked along semantic boundaries.
    ChunkBySemanticBoundary,
    /// Chunked along document structure (headings, sections).
    ChunkByDocStructure,
    /// Chunked into fixed-size windows.
    ChunkByFixedWindow,
    /// Merged from a disclosed remote provider overlay.
    ProviderOverlayMerge,
    /// No selection reason applies; the lane did not contribute.
    NotApplicable,
}

impl SelectionReasonClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RankBySemanticSimilarity => "rank_by_semantic_similarity",
            Self::RankByLexicalScore => "rank_by_lexical_score",
            Self::RankByGraphProximity => "rank_by_graph_proximity",
            Self::RankByHybridFusion => "rank_by_hybrid_fusion",
            Self::ChunkBySemanticBoundary => "chunk_by_semantic_boundary",
            Self::ChunkByDocStructure => "chunk_by_doc_structure",
            Self::ChunkByFixedWindow => "chunk_by_fixed_window",
            Self::ProviderOverlayMerge => "provider_overlay_merge",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// The [`SelectionReasonKind`] this concrete reason belongs to.
    pub const fn kind(self) -> SelectionReasonKind {
        match self {
            Self::RankBySemanticSimilarity
            | Self::RankByLexicalScore
            | Self::RankByGraphProximity
            | Self::RankByHybridFusion => SelectionReasonKind::Ranking,
            Self::ChunkBySemanticBoundary
            | Self::ChunkByDocStructure
            | Self::ChunkByFixedWindow => SelectionReasonKind::Chunking,
            Self::ProviderOverlayMerge => SelectionReasonKind::Overlay,
            Self::NotApplicable => SelectionReasonKind::NotApplicable,
        }
    }
}

/// Where the data a lane read physically lived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalLocalityClass {
    /// Data stayed on the local device only.
    LocalDeviceOnly,
    /// Data is workspace-scoped, no cross-workspace recall.
    WorkspaceLocal,
    /// Data is tenant-scoped and region-pinned, no cross-tenant recall.
    TenantRegionPinned,
    /// Data is managed-hosted and region-pinned.
    ManagedHostedRegionPinned,
    /// Data was fetched from a remote provider.
    ProviderRemote,
}

impl RetrievalLocalityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::WorkspaceLocal => "workspace_local",
            Self::TenantRegionPinned => "tenant_region_pinned",
            Self::ManagedHostedRegionPinned => "managed_hosted_region_pinned",
            Self::ProviderRemote => "provider_remote",
        }
    }

    /// Whether this locality involves leaving the local workspace boundary.
    pub const fn is_remote_or_managed(self) -> bool {
        matches!(self, Self::ManagedHostedRegionPinned | Self::ProviderRemote)
    }
}

/// Retrieval generation that produced a lane's candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalGenerationState {
    /// Lane is on the current generation.
    Current,
    /// Lane is recomputing onto the current generation.
    Recomputing,
    /// Lane is on a stale prior generation.
    Stale,
    /// Lane mixes generations, but the mix is explicitly labeled.
    MixedGenerationLabeled,
}

impl RetrievalGenerationState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Recomputing => "recomputing",
            Self::Stale => "stale",
            Self::MixedGenerationLabeled => "mixed_generation_labeled",
        }
    }

    /// Whether this generation, if a lane contributed it, must surface as
    /// degraded rather than as a clean current contribution.
    pub const fn requires_degraded(self) -> bool {
        matches!(self, Self::Recomputing | Self::Stale)
    }
}

/// One labeled contribution lane in a recall result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributionLaneRow {
    /// Which lane this row describes.
    pub lane: ContributionLaneClass,
    /// Whether and how the lane contributed.
    pub state: ContributionState,
    /// Whether the lane explains by ranking, chunking, or overlay.
    pub selection_reason_kind: SelectionReasonKind,
    /// The concrete ranking-or-chunking reason.
    pub selection_reason: SelectionReasonClass,
    /// Where the lane's data lived.
    pub locality: RetrievalLocalityClass,
    /// Retrieval generation that produced the lane's candidates.
    pub generation_state: RetrievalGenerationState,
    /// Review-safe summary of what the lane contributed and why.
    pub reason_summary: String,
}

impl ContributionLaneRow {
    /// Whether this lane row labels itself coherently.
    ///
    /// The concrete reason must match its declared kind; an inactive lane must
    /// declare a [`SelectionReasonClass::NotApplicable`] reason while an active
    /// lane must not; a provider-overlay lane that contributes must use an
    /// overlay reason and a remote or managed locality, and no other lane may
    /// claim an overlay reason.
    pub fn is_coherent(&self) -> bool {
        if self.reason_summary.trim().is_empty() {
            return false;
        }
        if self.selection_reason.kind() != self.selection_reason_kind {
            return false;
        }
        let is_not_applicable = self.selection_reason == SelectionReasonClass::NotApplicable;
        if self.state.is_active() == is_not_applicable {
            // Active lanes need a real reason; inactive lanes must not carry one.
            return false;
        }
        match self.lane {
            ContributionLaneClass::ProviderOverlay => {
                if self.state.is_active()
                    && (self.selection_reason_kind != SelectionReasonKind::Overlay
                        || !self.locality.is_remote_or_managed())
                {
                    return false;
                }
            }
            _ => {
                if self.selection_reason_kind == SelectionReasonKind::Overlay {
                    return false;
                }
            }
        }
        true
    }

    /// Whether the lane's generation is presented honestly: a stale or
    /// recomputing generation must show as [`ContributionState::Degraded`].
    pub fn generation_is_honest(&self) -> bool {
        if self.generation_state.requires_degraded() {
            self.state == ContributionState::Degraded
        } else {
            true
        }
    }
}

/// Disclosed completeness of a recall result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessClass {
    /// Every in-scope candidate was considered; nothing was hidden or degraded.
    Complete,
    /// Some in-scope candidates are hidden (scope, permission, or budget).
    PartialHiddenScope,
    /// The result is a degraded subset because one or more lanes degraded.
    DegradedSubset,
}

impl CompletenessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::PartialHiddenScope => "partial_hidden_scope",
            Self::DegradedSubset => "degraded_subset",
        }
    }

    /// Whether this class asserts a complete, undegraded result.
    pub const fn implies_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Provider-overlay posture disclosed for a recall surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderOverlayPosture {
    /// No provider overlay is configured for this surface.
    NoOverlay,
    /// Recall stays local; a provider overlay exists but did not contribute.
    LocalOnlyNoOverlay,
    /// A provider overlay contributed and is disclosed.
    OverlayDisclosed,
    /// A provider overlay contributed but is degraded.
    OverlayDegraded,
}

impl ProviderOverlayPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOverlay => "no_overlay",
            Self::LocalOnlyNoOverlay => "local_only_no_overlay",
            Self::OverlayDisclosed => "overlay_disclosed",
            Self::OverlayDegraded => "overlay_degraded",
        }
    }

    /// Whether this posture discloses an active provider overlay.
    pub const fn discloses_active_overlay(self) -> bool {
        matches!(self, Self::OverlayDisclosed | Self::OverlayDegraded)
    }
}

/// Consumer surface that must project an inspector row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorConsumerSurface {
    /// Search results UI.
    SearchResults,
    /// Docs or in-app browser companion.
    DocsCompanion,
    /// Composer context-pack inspector.
    ComposerContextPack,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Support / export packet.
    SupportExport,
    /// Replay packet.
    ReplayPacket,
}

impl InspectorConsumerSurface {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResults => "search_results",
            Self::DocsCompanion => "docs_companion",
            Self::ComposerContextPack => "composer_context_pack",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::ReplayPacket => "replay_packet",
        }
    }

    /// Whether this consumer must preserve the in-product lane and locality
    /// labels (support export and replay both must).
    pub const fn requires_label_parity(self) -> bool {
        matches!(self, Self::SupportExport | Self::ReplayPacket)
    }
}

/// Trigger that can narrow an inspector surface below its disclosed posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorDowngradeTrigger {
    /// A contribution lane degraded.
    LaneDegraded,
    /// Mixed embedding generations were detected without labeling.
    MixedGenerationUnlabeled,
    /// Hidden in-scope candidates were not disclosed.
    HiddenScopeUndisclosed,
    /// A provider overlay contributed without disclosure.
    ProviderOverlayUndisclosed,
    /// Exported or replayed labels drifted from in-product labels.
    ReplayLabelDrift,
    /// A required locality posture is unavailable on this build.
    LocalityUnavailable,
    /// Proof packet has gone stale.
    ProofStale,
}

impl InspectorDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LaneDegraded,
        Self::MixedGenerationUnlabeled,
        Self::HiddenScopeUndisclosed,
        Self::ProviderOverlayUndisclosed,
        Self::ReplayLabelDrift,
        Self::LocalityUnavailable,
        Self::ProofStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaneDegraded => "lane_degraded",
            Self::MixedGenerationUnlabeled => "mixed_generation_unlabeled",
            Self::HiddenScopeUndisclosed => "hidden_scope_undisclosed",
            Self::ProviderOverlayUndisclosed => "provider_overlay_undisclosed",
            Self::ReplayLabelDrift => "replay_label_drift",
            Self::LocalityUnavailable => "locality_unavailable",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One inspected recall surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalInspectorSurfaceRow {
    /// Which recall surface this row inspects.
    pub surface: RetrievalInspectorSurface,
    /// Review-safe description of the inspected result.
    pub scope_summary: String,
    /// Labeled contribution lanes.
    pub contribution_lanes: Vec<ContributionLaneRow>,
    /// Count of in-scope candidates that were hidden from the result.
    pub hidden_scope_count: u32,
    /// Lanes the surface reports as degraded.
    pub degraded_lanes: Vec<ContributionLaneClass>,
    /// Disclosed provider-overlay posture.
    pub provider_overlay_posture: ProviderOverlayPosture,
    /// Disclosed completeness of the result.
    pub completeness_claim: CompletenessClass,
    /// Whether exported / replayed labels match in-product labels.
    pub replay_label_parity: bool,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<InspectorDowngradeTrigger>,
    /// Source contract refs this surface projects against.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<InspectorConsumerSurface>,
}

impl RetrievalInspectorSurfaceRow {
    /// The lanes whose [`ContributionState`] is [`ContributionState::Degraded`].
    fn degraded_states(&self) -> BTreeSet<ContributionLaneClass> {
        self.contribution_lanes
            .iter()
            .filter(|row| row.state == ContributionState::Degraded)
            .map(|row| row.lane)
            .collect()
    }

    /// Whether the reported [`Self::degraded_lanes`] exactly match the lanes
    /// whose state is degraded — disclosure can neither hide nor invent one.
    pub fn degraded_lanes_consistent(&self) -> bool {
        let reported: BTreeSet<ContributionLaneClass> =
            self.degraded_lanes.iter().copied().collect();
        reported == self.degraded_states()
    }

    /// Whether the provider-overlay posture matches the lane reality: an active
    /// overlay lane is disclosed, and a disclosed overlay has an active lane.
    pub fn overlay_disclosure_consistent(&self) -> bool {
        let has_active_overlay = self
            .contribution_lanes
            .iter()
            .any(|row| row.lane == ContributionLaneClass::ProviderOverlay && row.state.is_active());
        has_active_overlay == self.provider_overlay_posture.discloses_active_overlay()
    }

    /// Whether the completeness claim is honest: a surface that hides scope or
    /// carries a degraded lane may not claim [`CompletenessClass::Complete`].
    pub fn completeness_is_honest(&self) -> bool {
        let hides_or_degrades = self.hidden_scope_count > 0 || !self.degraded_lanes.is_empty();
        !(hides_or_degrades && self.completeness_claim.implies_complete())
    }

    /// Whether every lane is coherent and presents its generation honestly.
    pub fn lanes_are_coherent(&self) -> bool {
        self.contribution_lanes
            .iter()
            .all(|row| row.is_coherent() && row.generation_is_honest())
    }

    /// Whether this surface preserves labels for every consumer that requires
    /// parity (support export and replay).
    pub fn replay_parity_satisfied(&self) -> bool {
        if self
            .consumer_surfaces
            .iter()
            .any(|consumer| consumer.requires_label_parity())
        {
            self.replay_label_parity
        } else {
            true
        }
    }

    /// Effective completeness after honest narrowing.
    ///
    /// A surface that claims [`CompletenessClass::Complete`] while hiding scope
    /// or carrying a degraded lane narrows to [`CompletenessClass::DegradedSubset`]
    /// (when a lane degraded) or [`CompletenessClass::PartialHiddenScope`].
    pub fn effective_completeness(&self) -> CompletenessClass {
        if self.completeness_is_honest() {
            return self.completeness_claim;
        }
        if !self.degraded_lanes.is_empty() {
            CompletenessClass::DegradedSubset
        } else {
            CompletenessClass::PartialHiddenScope
        }
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLocalityInspectorGuardrails {
    /// No cross-workspace recall happens by default.
    pub no_cross_workspace_recall_by_default: bool,
    /// No cross-tenant recall happens by default.
    pub no_cross_tenant_recall_by_default: bool,
    /// Mixed generations are labeled and never masquerade as current.
    pub mixed_generation_labeled_never_masquerades: bool,
    /// Degraded lanes are never presented as a complete result.
    pub degraded_lanes_never_implied_complete: bool,
    /// A contributing provider overlay is always disclosed.
    pub provider_overlay_always_disclosed: bool,
    /// Replay and support exports preserve lane and locality labels.
    pub replay_preserves_lane_and_locality_labels: bool,
    /// Hidden in-scope counts are disclosed, not silently dropped.
    pub hidden_scope_counts_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLocalityInspectorConsumerProjection {
    /// Search labels every contribution lane.
    pub search_labels_all_contribution_lanes: bool,
    /// Docs recall labels lanes and locality.
    pub docs_recall_labels_lanes_and_locality: bool,
    /// AI context pack labels lanes with ranking-or-chunking reasons.
    pub context_pack_labels_lanes_and_ranking_or_chunking: bool,
    /// Diagnostics shows hidden-scope counts and degraded lanes.
    pub diagnostics_shows_hidden_scope_and_degraded: bool,
    /// Support export preserves the labels the user saw.
    pub support_export_preserves_labels: bool,
    /// Replay packets preserve the labels the user saw.
    pub replay_preserves_labels: bool,
    /// Surfaces below complete are visibly labeled, never presented as complete.
    pub unqualified_completeness_labeled: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLocalityInspectorProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RetrievalLocalityInspectorPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievalLocalityInspectorPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable inspector label.
    pub inspector_label: String,
    /// Inspected surface rows.
    pub surface_rows: Vec<RetrievalInspectorSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: RetrievalLocalityInspectorGuardrails,
    /// Consumer projection block.
    pub consumer_projection: RetrievalLocalityInspectorConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RetrievalLocalityInspectorProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe retrieval-locality inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLocalityInspectorPacket {
    /// Record kind; must equal [`RETRIEVAL_LOCALITY_INSPECTOR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable inspector label.
    pub inspector_label: String,
    /// Inspected surface rows.
    pub surface_rows: Vec<RetrievalInspectorSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: RetrievalLocalityInspectorGuardrails,
    /// Consumer projection block.
    pub consumer_projection: RetrievalLocalityInspectorConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RetrievalLocalityInspectorProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RetrievalLocalityInspectorPacket {
    /// Builds a retrieval-locality inspector packet from surface input.
    pub fn new(input: RetrievalLocalityInspectorPacketInput) -> Self {
        Self {
            record_kind: RETRIEVAL_LOCALITY_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_VERSION,
            packet_id: input.packet_id,
            inspector_label: input.inspector_label,
            surface_rows: input.surface_rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the retrieval-locality inspector invariants.
    pub fn validate(&self) -> Vec<RetrievalLocalityInspectorViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RETRIEVAL_LOCALITY_INSPECTOR_RECORD_KIND {
            violations.push(RetrievalLocalityInspectorViolation::WrongRecordKind);
        }
        if self.schema_version != RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_VERSION {
            violations.push(RetrievalLocalityInspectorViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.inspector_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RetrievalLocalityInspectorViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("retrieval locality inspector packet serializes"),
        ) {
            violations.push(RetrievalLocalityInspectorViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("retrieval locality inspector packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Retrieval Locality Inspector\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.inspector_label));
        out.push_str(&format!("- Surfaces: {}\n", self.surface_rows.len()));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: completeness `{}`\n",
                row.surface.as_str(),
                row.completeness_claim.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Hidden scope: {}\n", row.hidden_scope_count));
            out.push_str(&format!(
                "  - Provider overlay: `{}`\n",
                row.provider_overlay_posture.as_str()
            ));
            out.push_str("  - Lanes:\n");
            for lane in &row.contribution_lanes {
                out.push_str(&format!(
                    "    - `{}`: {} / {} / locality `{}` / gen `{}`\n",
                    lane.lane.as_str(),
                    lane.state.as_str(),
                    lane.selection_reason.as_str(),
                    lane.locality.as_str(),
                    lane.generation_state.as_str()
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in inspector export.
#[derive(Debug)]
pub enum RetrievalLocalityInspectorArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RetrievalLocalityInspectorViolation>),
}

impl fmt::Display for RetrievalLocalityInspectorArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "retrieval locality inspector export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "retrieval locality inspector export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RetrievalLocalityInspectorArtifactError {}

/// Validation failures emitted by [`RetrievalLocalityInspectorPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetrievalLocalityInspectorViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required surface is missing from the inspector.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A contribution lane labels itself incoherently.
    LaneLabelingIncoherent,
    /// Reported degraded lanes do not match degraded lane states.
    DegradedLanesInconsistent,
    /// A stale or recomputing generation masquerades as a clean contribution.
    MixedGenerationMasquerades,
    /// A contributing provider overlay is not disclosed by the overlay posture.
    ProviderOverlayUndisclosed,
    /// A surface that hides scope or degrades a lane claims completeness.
    HiddenScopeImpliesComplete,
    /// A surface a support export or replay can reach drops label parity.
    ReplayLabelParityMissing,
    /// A surface has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RetrievalLocalityInspectorViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::LaneLabelingIncoherent => "lane_labeling_incoherent",
            Self::DegradedLanesInconsistent => "degraded_lanes_inconsistent",
            Self::MixedGenerationMasquerades => "mixed_generation_masquerades",
            Self::ProviderOverlayUndisclosed => "provider_overlay_undisclosed",
            Self::HiddenScopeImpliesComplete => "hidden_scope_implies_complete",
            Self::ReplayLabelParityMissing => "replay_label_parity_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable inspector export.
pub fn current_stable_retrieval_locality_inspector_export(
) -> Result<RetrievalLocalityInspectorPacket, RetrievalLocalityInspectorArtifactError> {
    let packet: RetrievalLocalityInspectorPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em/support_export.json"
    )))
    .map_err(RetrievalLocalityInspectorArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RetrievalLocalityInspectorArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &RetrievalLocalityInspectorPacket,
    violations: &mut Vec<RetrievalLocalityInspectorViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF,
        RETRIEVAL_LOCALITY_INSPECTOR_DOC_REF,
        RETRIEVAL_LOCALITY_INSPECTOR_RANKING_CONTRACT_REF,
        RETRIEVAL_LOCALITY_INSPECTOR_EXPLAINABILITY_CONTRACT_REF,
        RETRIEVAL_LOCALITY_INSPECTOR_CONTEXT_ASSEMBLY_CONTRACT_REF,
        RETRIEVAL_LOCALITY_INSPECTOR_SPEND_RECEIPT_CONTRACT_REF,
        RETRIEVAL_LOCALITY_INSPECTOR_RECALL_MATRIX_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RetrievalLocalityInspectorViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surface_rows(
    packet: &RetrievalLocalityInspectorPacket,
    violations: &mut Vec<RetrievalLocalityInspectorViolation>,
) {
    let present: BTreeSet<RetrievalInspectorSurface> =
        packet.surface_rows.iter().map(|row| row.surface).collect();
    for required in RetrievalInspectorSurface::ALL {
        if !present.contains(&required) {
            violations.push(RetrievalLocalityInspectorViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.contribution_lanes.is_empty()
        {
            violations.push(RetrievalLocalityInspectorViolation::SurfaceRowIncomplete);
        }
        if !row
            .contribution_lanes
            .iter()
            .all(ContributionLaneRow::is_coherent)
        {
            violations.push(RetrievalLocalityInspectorViolation::LaneLabelingIncoherent);
        }
        if !row
            .contribution_lanes
            .iter()
            .all(ContributionLaneRow::generation_is_honest)
        {
            violations.push(RetrievalLocalityInspectorViolation::MixedGenerationMasquerades);
        }
        if !row.degraded_lanes_consistent() {
            violations.push(RetrievalLocalityInspectorViolation::DegradedLanesInconsistent);
        }
        if !row.overlay_disclosure_consistent() {
            violations.push(RetrievalLocalityInspectorViolation::ProviderOverlayUndisclosed);
        }
        if !row.completeness_is_honest() {
            violations.push(RetrievalLocalityInspectorViolation::HiddenScopeImpliesComplete);
        }
        if !row.replay_parity_satisfied() {
            violations.push(RetrievalLocalityInspectorViolation::ReplayLabelParityMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(RetrievalLocalityInspectorViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(RetrievalLocalityInspectorViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_guardrails(
    packet: &RetrievalLocalityInspectorPacket,
    violations: &mut Vec<RetrievalLocalityInspectorViolation>,
) {
    let guardrails = &packet.guardrails;
    let ok = guardrails.no_cross_workspace_recall_by_default
        && guardrails.no_cross_tenant_recall_by_default
        && guardrails.mixed_generation_labeled_never_masquerades
        && guardrails.degraded_lanes_never_implied_complete
        && guardrails.provider_overlay_always_disclosed
        && guardrails.replay_preserves_lane_and_locality_labels
        && guardrails.hidden_scope_counts_disclosed;
    if !ok {
        violations.push(RetrievalLocalityInspectorViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &RetrievalLocalityInspectorPacket,
    violations: &mut Vec<RetrievalLocalityInspectorViolation>,
) {
    let projection = &packet.consumer_projection;
    let ok = projection.search_labels_all_contribution_lanes
        && projection.docs_recall_labels_lanes_and_locality
        && projection.context_pack_labels_lanes_and_ranking_or_chunking
        && projection.diagnostics_shows_hidden_scope_and_degraded
        && projection.support_export_preserves_labels
        && projection.replay_preserves_labels
        && projection.unqualified_completeness_labeled;
    if !ok {
        violations.push(RetrievalLocalityInspectorViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &RetrievalLocalityInspectorPacket,
    violations: &mut Vec<RetrievalLocalityInspectorViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(RetrievalLocalityInspectorViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
