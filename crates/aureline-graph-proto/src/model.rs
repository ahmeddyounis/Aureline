//! In-memory semantic-workspace-graph model.
//!
//! Mirrors the record shapes the schema encodes. The validator
//! enforces the eleven identity / label rules; consumers construct
//! these values and hand them off to `validate_graph`.

use crate::vocab::{
    AnchorKind, CitationClass, ConfidenceLevel, EdgeClass, EdgeEvidenceState, EnvironmentClass,
    Freshness, ImpactReasonClass, InvalidationProducerTag, MissingReason, NodeClass,
    ProvenanceClass, QueryFamilyTag, ReachabilityState, ShardAffinityTag, SourceClass,
    StaleReason, SymbolVisibility, TopologyKind, TrustState, Visibility, WarmingProgressHint,
    WorksetScopeClass,
};

/// ADR-0006 five-layer filesystem-identity record. The graph seed
/// references this shape verbatim rather than minting a private
/// identity family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesystemIdentity {
    pub presentation_path: String,
    pub logical_workspace_identity: String,
    pub canonical_filesystem_object: String,
    pub alias_set: Vec<String>,
    pub save_target_token: String,
}

/// One workset / scope ref. Every node and edge lists at least one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorksetScopeRef {
    pub scope_class: WorksetScopeClass,
    pub scope_id: String,
    pub visibility: Visibility,
}

/// One source-anchor slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAnchor {
    pub anchor_kind: AnchorKind,
    pub anchor_ref: String,
    pub line_range: Option<String>,
}

/// Provenance stamp carried by every node and edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceStamp {
    pub source_class: SourceClass,
    pub provenance_class: ProvenanceClass,
    pub producer_ref: Option<String>,
    pub producer_version: Option<String>,
    pub recorded_at: String,
    pub imported_bundle_ref: Option<String>,
    pub replay_capture_ref: Option<String>,
    pub support_ref: Option<String>,
}

/// Per-record freshness frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshnessFrame {
    pub freshness: Freshness,
    pub recorded_at: String,
    pub stale_reason: Option<StaleReason>,
    pub cache_key_ref: Option<String>,
    pub warming_progress_hint: Option<WarmingProgressHint>,
}

/// Graph-level confidence-rollup slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfidenceRollup {
    pub rolled_up_level: ConfidenceLevel,
    pub source_confidences: Vec<ConfidenceLevel>,
    pub rollup_note: Option<String>,
}

/// Impact-reason slot for impact explorers and review packs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpactReason {
    pub reason_class: ImpactReasonClass,
    pub note: Option<String>,
    pub mutation_journal_ref: Option<String>,
    pub review_ref: Option<String>,
}

/// Explainer-citation slot for cited-explainer overlays and AI
/// context. `citation_ref` reuses the graph node id family rather
/// than minting a private pointer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplainerCitation {
    pub citation_class: CitationClass,
    pub citation_ref: String,
    pub line_range: Option<String>,
    pub confidence_level: ConfidenceLevel,
}

/// Topology-edge slot. Topology maps reuse graph edges with this
/// slot rather than minting a private topology record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyEdgeSlot {
    pub topology_kind: TopologyKind,
    pub environment_class: Option<EnvironmentClass>,
    pub deployment_tag: Option<String>,
}

/// Class-specific node body. The `node_class` on `GraphNode` MUST
/// match exactly one variant here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeBody {
    File {
        filesystem_identity: FilesystemIdentity,
        media_class: Option<String>,
        language_id: Option<String>,
        large_file_mode: bool,
    },
    Directory {
        filesystem_identity: FilesystemIdentity,
        role: Option<String>,
    },
    Symbol {
        symbol_kind: String,
        declared_in_file_node_id: String,
        qualified_path: String,
        visibility: Option<SymbolVisibility>,
    },
    Doc {
        doc_kind: String,
        doc_ref: String,
        anchor_filesystem_identity: Option<FilesystemIdentity>,
    },
    Ownership {
        ownership_kind: String,
        ownership_ref: String,
        display_label: Option<String>,
        codeowners_rule_ref: Option<String>,
    },
    Topology {
        topology_kind: String,
        topology_ref: String,
        environment_class: Option<EnvironmentClass>,
    },
    ProviderResource {
        provider_kind: String,
        provider_ref: String,
        resource_handle: String,
        reachability_state: Option<ReachabilityState>,
    },
    GeneratedArtifact {
        lineage_record_ref: String,
        filesystem_identity: Option<FilesystemIdentity>,
        generation_class: String,
        drift_state: String,
    },
    ImportedRoot {
        import_kind: String,
        import_ref: String,
        filesystem_identity: Option<FilesystemIdentity>,
        trust_state: Option<TrustState>,
    },
    WorksetScope {
        scope_ref: WorksetScopeRef,
        display_label: Option<String>,
    },
    PolicyView {
        underlying_scope_id: String,
        policy_ref: String,
        hidden_member_count: u64,
    },
    MissingAnchor {
        expected_node_class: NodeClass,
        missing_reason: MissingReason,
        last_known_ref: Option<String>,
    },
}

impl NodeBody {
    /// The node class the body is valid for.
    pub fn expected_node_class(&self) -> NodeClass {
        match self {
            Self::File { .. } => NodeClass::FileNode,
            Self::Directory { .. } => NodeClass::DirectoryNode,
            Self::Symbol { .. } => NodeClass::SymbolNode,
            Self::Doc { .. } => NodeClass::DocNode,
            Self::Ownership { .. } => NodeClass::OwnershipNode,
            Self::Topology { .. } => NodeClass::TopologyNode,
            Self::ProviderResource { .. } => NodeClass::ProviderResourceNode,
            Self::GeneratedArtifact { .. } => NodeClass::GeneratedArtifactNode,
            Self::ImportedRoot { .. } => NodeClass::ImportedRootNode,
            Self::WorksetScope { .. } => NodeClass::WorksetScopeNode,
            Self::PolicyView { .. } => NodeClass::PolicyViewNode,
            Self::MissingAnchor { .. } => NodeClass::MissingAnchorNode,
        }
    }
}

/// Edge evidence / provenance / freshness / confidence bundle. Split
/// from `GraphEdge` so that tests can roll a freshness frame without
/// re-stating the endpoint id pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeEvidence {
    pub evidence_state: EdgeEvidenceState,
    pub provenance_stamp: ProvenanceStamp,
    pub freshness_frame: FreshnessFrame,
    pub confidence_level: ConfidenceLevel,
    pub confidence_rollup: Option<ConfidenceRollup>,
}

/// Thin typed wrapper for the edge class. Keeping edge_class and
/// body separate avoids ambiguity on edges that do not carry a body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EdgeBody {
    pub topology_edge_slot: Option<TopologyEdgeSlot>,
    pub impact_reasons: Vec<ImpactReason>,
    pub explainer_citations: Vec<ExplainerCitation>,
}

/// One graph node record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphNode {
    pub node_id: String,
    pub node_class: NodeClass,
    pub workspace_id: String,
    pub node_body: NodeBody,
    pub display_label: Option<String>,
    pub provenance_stamp: ProvenanceStamp,
    pub freshness_frame: FreshnessFrame,
    pub confidence_level: ConfidenceLevel,
    pub confidence_rollup: Option<ConfidenceRollup>,
    pub query_family_tags: Vec<QueryFamilyTag>,
    pub shard_affinity_tags: Vec<ShardAffinityTag>,
    pub invalidation_producer_tags: Vec<InvalidationProducerTag>,
    pub scope_refs: Vec<WorksetScopeRef>,
    pub source_anchors: Vec<SourceAnchor>,
    pub impact_reasons: Vec<ImpactReason>,
    pub explainer_citations: Vec<ExplainerCitation>,
}

/// One graph edge record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEdge {
    pub edge_id: String,
    pub edge_class: EdgeClass,
    pub workspace_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub evidence: EdgeEvidence,
    pub body: EdgeBody,
    pub query_family_tags: Vec<QueryFamilyTag>,
    pub shard_affinity_tags: Vec<ShardAffinityTag>,
    pub invalidation_producer_tags: Vec<InvalidationProducerTag>,
    pub scope_refs: Vec<WorksetScopeRef>,
    pub source_anchors: Vec<SourceAnchor>,
}

/// One workspace-graph snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceGraph {
    pub workspace_graph_id: String,
    pub workspace_id: String,
    pub recorded_at: String,
    pub producer_ref: Option<String>,
    pub producer_version: Option<String>,
    pub scope_refs: Vec<WorksetScopeRef>,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub notes: Vec<String>,
}
