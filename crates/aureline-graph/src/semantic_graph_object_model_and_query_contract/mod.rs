//! Stable semantic-graph object model and query-family contract.
//!
//! This module is the graph-owned stable contract for semantic workspace
//! graph objects, query-family handles, invalidation decisions, surface
//! bindings, topology reuse, and support reconstruction. It is deliberately
//! metadata-only: object bodies, raw source, raw query text, provider payloads,
//! credentials, and private inference prompts stay outside this packet.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SemanticGraphContractPacket`].
pub const SEMANTIC_GRAPH_CONTRACT_PACKET_RECORD_KIND: &str =
    "semantic_graph_object_model_and_query_contract_packet";

/// Stable record-kind tag for [`SemanticGraphContractSupportExport`].
pub const SEMANTIC_GRAPH_CONTRACT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "semantic_graph_object_model_and_query_contract_support_export";

/// Integer schema version for the stable semantic-graph contract packet.
pub const SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the reviewer doc.
pub const SEMANTIC_GRAPH_CONTRACT_DOC_REF: &str =
    "docs/graph/m4/semantic-graph-object-model-and-query-contract.md";

/// Repo-relative path of the human-readable release artifact.
pub const SEMANTIC_GRAPH_CONTRACT_ARTIFACT_DOC_REF: &str =
    "artifacts/graph/m4/semantic-graph-object-model-and-query-contract.md";

/// Repo-relative path of the stable boundary schema.
pub const SEMANTIC_GRAPH_CONTRACT_SCHEMA_REF: &str =
    "schemas/graph/semantic-workspace-graph.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const SEMANTIC_GRAPH_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/graph/m4/semantic-graph-object-model-and-query-contract";

/// Repo-relative path of the checked-in stable contract packet.
pub const SEMANTIC_GRAPH_CONTRACT_PACKET_ARTIFACT_REF: &str =
    "artifacts/graph/m4/semantic-graph-object-model-and-query-contract.json";

/// Stable semantic-graph object classes shared by graph-backed consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableGraphObjectKind {
    /// Workspace, root, or named workset boundary object.
    WorkspaceRootWorkset,
    /// File or document object.
    FileDocument,
    /// Symbol, API, route, or schema object.
    SymbolApi,
    /// Relationship edge object.
    RelationshipEdge,
    /// Docs, ADR, runbook, ownership, or knowledge-pack object.
    DocsKnowledge,
    /// Operational artifact such as build, runtime, review, support, or generated artifact.
    OperationalArtifact,
}

impl StableGraphObjectKind {
    /// Every required stable graph object kind in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::WorkspaceRootWorkset,
        Self::FileDocument,
        Self::SymbolApi,
        Self::RelationshipEdge,
        Self::DocsKnowledge,
        Self::OperationalArtifact,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRootWorkset => "workspace_root_workset",
            Self::FileDocument => "file_document",
            Self::SymbolApi => "symbol_api",
            Self::RelationshipEdge => "relationship_edge",
            Self::DocsKnowledge => "docs_knowledge",
            Self::OperationalArtifact => "operational_artifact",
        }
    }
}

/// Stable query-family vocabulary exposed to consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StableQueryFamily {
    /// Resolve a stable object or label.
    Lookup,
    /// Traverse directly adjacent graph objects.
    Neighborhood,
    /// Explain why a graph-backed answer or relationship exists.
    ExplainWhy,
    /// Return impact candidates and impact reasons for a subject.
    Impact,
    /// Resolve owners, policy authorities, or responsibility relations.
    Ownership,
    /// Compare graph objects or query answers across two snapshots.
    DiffBetweenSnapshots,
    /// Return the evidence chain for an object, edge, or answer.
    PathToEvidence,
}

impl StableQueryFamily {
    /// Every stable query family in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::Lookup,
        Self::Neighborhood,
        Self::ExplainWhy,
        Self::Impact,
        Self::Ownership,
        Self::DiffBetweenSnapshots,
        Self::PathToEvidence,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lookup => "lookup",
            Self::Neighborhood => "neighborhood",
            Self::ExplainWhy => "explain-why",
            Self::Impact => "impact",
            Self::Ownership => "ownership",
            Self::DiffBetweenSnapshots => "diff-between-snapshots",
            Self::PathToEvidence => "path-to-evidence",
        }
    }
}

/// Shared freshness vocabulary for graph-backed product lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphFreshnessState {
    /// Current and complete for the declared scope.
    Live,
    /// Known stale and rendered with a stale disclosure.
    Stale,
    /// The declared scope is still warming.
    Warming,
    /// The declared scope is intentionally or operationally partial.
    PartialScope,
    /// Answer came from a cached snapshot.
    Cached,
    /// Provider or graph lane cannot currently answer.
    ProviderUnavailable,
}

impl GraphFreshnessState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
            Self::Warming => "warming",
            Self::PartialScope => "partial_scope",
            Self::Cached => "cached",
            Self::ProviderUnavailable => "provider_unavailable",
        }
    }

    /// True when this state requires a visible caveat.
    pub const fn requires_visible_truth_label(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Shared graph confidence tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphConfidenceTier {
    /// Active buffer, lexical shard, or direct path evidence only.
    ImmediateLexical,
    /// Current parsed structure.
    Structural,
    /// Warm language/build semantic graph.
    Semantic,
    /// Corroborated by build, test, debug, runtime, or support evidence.
    VerifiedRuntime,
    /// Human-confirmed docs, owner, ADR, review, or support annotation.
    UserCurated,
    /// Inferred or heuristic relation that must remain labeled.
    Inferred,
    /// Confidence is intentionally withheld.
    Withheld,
}

impl GraphConfidenceTier {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmediateLexical => "immediate_lexical",
            Self::Structural => "structural",
            Self::Semantic => "semantic",
            Self::VerifiedRuntime => "verified_runtime",
            Self::UserCurated => "user_curated",
            Self::Inferred => "inferred",
            Self::Withheld => "withheld",
        }
    }

    /// True when this confidence tier requires an inference disclosure.
    pub const fn requires_inference_label(self) -> bool {
        matches!(self, Self::Inferred | Self::Withheld)
    }
}

/// Shared visibility scope model across search, review, docs, AI, topology, and support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphVisibilityScope {
    /// Current root only.
    CurrentRoot,
    /// Selected named workset.
    SelectedWorkset,
    /// Full loaded workspace.
    FullWorkspace,
    /// Remote cache or imported mirror.
    RemoteCache,
    /// Outside the current loaded scope.
    OutsideCurrentScope,
    /// Policy, trust, entitlement, or locality-limited view.
    PolicyLimited,
}

impl GraphVisibilityScope {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRoot => "current_root",
            Self::SelectedWorkset => "selected_workset",
            Self::FullWorkspace => "full_workspace",
            Self::RemoteCache => "remote_cache",
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// Shared retention class for stable graph objects and query handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphRetentionClass {
    /// Durable workspace graph object.
    DurableWorkspace,
    /// Session-scoped graph object.
    SessionScoped,
    /// Exportable captured snapshot.
    ExportableSnapshot,
    /// Managed replicated view.
    ManagedReplica,
    /// Ephemeral projection not persisted as graph truth.
    EphemeralProjection,
    /// Retention is withheld by policy.
    Withheld,
}

impl GraphRetentionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableWorkspace => "durable_workspace",
            Self::SessionScoped => "session_scoped",
            Self::ExportableSnapshot => "exportable_snapshot",
            Self::ManagedReplica => "managed_replica",
            Self::EphemeralProjection => "ephemeral_projection",
            Self::Withheld => "withheld",
        }
    }
}

/// Producer identity attached to every stable graph object and query handle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphProducerIdentity {
    /// Stable producer id.
    pub producer_id: String,
    /// Producer kind or lane.
    pub producer_kind: String,
    /// Producer version that minted the object or query handle.
    pub producer_version: String,
}

impl GraphProducerIdentity {
    fn is_valid(&self) -> bool {
        !self.producer_id.trim().is_empty()
            && !self.producer_kind.trim().is_empty()
            && !self.producer_version.trim().is_empty()
    }
}

/// Stable graph object shared across claimed stable graph-backed lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableGraphObject {
    /// Stable object id.
    pub object_id: String,
    /// Stable object kind.
    pub object_kind: StableGraphObjectKind,
    /// Object schema version.
    pub schema_version: u32,
    /// Producer identity.
    pub producer: GraphProducerIdentity,
    /// Timestamp at which freshness was evaluated.
    pub freshness_timestamp: String,
    /// Shared freshness state.
    pub freshness: GraphFreshnessState,
    /// Shared confidence tier.
    pub confidence: GraphConfidenceTier,
    /// Shared visibility scope.
    pub visibility_scope: GraphVisibilityScope,
    /// Shared retention class.
    pub retention_class: GraphRetentionClass,
    /// Provenance refs safe for support/export.
    #[serde(default)]
    pub provenance_refs: Vec<String>,
    /// Evidence refs that reconstruct the object without raw private payloads.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Source object id for relationship-edge objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_object_id: Option<String>,
    /// Target object id for relationship-edge objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_object_id: Option<String>,
    /// Truth label shown when freshness, confidence, scope, or provider state is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_truth_label: Option<String>,
}

impl StableGraphObject {
    fn has_required_metadata(&self) -> bool {
        !self.object_id.trim().is_empty()
            && self.schema_version > 0
            && self.producer.is_valid()
            && !self.freshness_timestamp.trim().is_empty()
            && !self.provenance_refs.is_empty()
            && !self.evidence_refs.is_empty()
    }

    fn has_relationship_endpoints(&self) -> bool {
        !matches!(self.object_kind, StableGraphObjectKind::RelationshipEdge)
            || (self
                .source_object_id
                .as_deref()
                .map(|id| !id.trim().is_empty())
                .unwrap_or(false)
                && self
                    .target_object_id
                    .as_deref()
                    .map(|id| !id.trim().is_empty())
                    .unwrap_or(false))
    }

    fn has_required_truth_label(&self) -> bool {
        let needs_label = self.freshness.requires_visible_truth_label()
            || self.confidence.requires_inference_label()
            || matches!(
                self.visibility_scope,
                GraphVisibilityScope::OutsideCurrentScope | GraphVisibilityScope::PolicyLimited
            );
        !needs_label
            || self
                .visible_truth_label
                .as_deref()
                .map(|label| !label.trim().is_empty())
                .unwrap_or(false)
    }
}

/// Stable query handle shared by graph-backed consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableGraphQueryHandle {
    /// Stable query handle id.
    pub query_handle_id: String,
    /// Stable query family.
    pub query_family: StableQueryFamily,
    /// Inspectable scope ref.
    pub scope_ref: String,
    /// Snapshot epoch that the answer was drawn from.
    pub snapshot_epoch: String,
    /// Delta sequence for subscription or materialized-view parity.
    pub delta_sequence: u64,
    /// Producer identity.
    pub producer: GraphProducerIdentity,
    /// Shared freshness state.
    pub freshness: GraphFreshnessState,
    /// Shared confidence tier.
    pub confidence: GraphConfidenceTier,
    /// Object refs present in the answer.
    #[serde(default)]
    pub object_refs: Vec<String>,
    /// Evidence refs that reconstruct the answer.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Truth label shown when query answer is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_truth_label: Option<String>,
}

impl StableGraphQueryHandle {
    fn is_valid(&self) -> bool {
        !self.query_handle_id.trim().is_empty()
            && !self.scope_ref.trim().is_empty()
            && !self.snapshot_epoch.trim().is_empty()
            && self.producer.is_valid()
            && !self.object_refs.is_empty()
            && !self.evidence_refs.is_empty()
    }

    fn has_required_truth_label(&self) -> bool {
        let needs_label = self.freshness.requires_visible_truth_label()
            || self.confidence.requires_inference_label();
        !needs_label
            || self
                .visible_truth_label
                .as_deref()
                .map(|label| !label.trim().is_empty())
                .unwrap_or(false)
    }
}

/// Bounded invalidation classes for graph refresh behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphInvalidationClass {
    /// No invalidation was needed.
    NoInvalidationNeeded,
    /// Only the smallest affected subgraph was invalidated.
    SmallestSubgraph,
    /// Full rebuild caused by schema boundary.
    FullRebuildSchemaBoundary,
    /// Full rebuild caused by producer-version boundary.
    FullRebuildProducerVersionBoundary,
    /// Full rebuild caused by workspace-epoch boundary.
    FullRebuildWorkspaceEpochBoundary,
    /// Disallowed arbitrary full-graph rebuild.
    ArbitraryFullGraphRebuild,
}

impl GraphInvalidationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoInvalidationNeeded => "no_invalidation_needed",
            Self::SmallestSubgraph => "smallest_subgraph",
            Self::FullRebuildSchemaBoundary => "full_rebuild_schema_boundary",
            Self::FullRebuildProducerVersionBoundary => "full_rebuild_producer_version_boundary",
            Self::FullRebuildWorkspaceEpochBoundary => "full_rebuild_workspace_epoch_boundary",
            Self::ArbitraryFullGraphRebuild => "arbitrary_full_graph_rebuild",
        }
    }

    /// True when this invalidation is an allowed full-rebuild boundary.
    pub const fn is_allowed_full_rebuild_boundary(self) -> bool {
        matches!(
            self,
            Self::FullRebuildSchemaBoundary
                | Self::FullRebuildProducerVersionBoundary
                | Self::FullRebuildWorkspaceEpochBoundary
        )
    }
}

/// One invalidation event attached to the stable contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphInvalidationEvent {
    /// Stable invalidation event id.
    pub event_id: String,
    /// Closed invalidation class.
    pub invalidation_class: GraphInvalidationClass,
    /// Object refs affected by a smallest-subgraph invalidation.
    #[serde(default)]
    pub affected_object_refs: Vec<String>,
    /// Producer version before the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_producer_version: Option<String>,
    /// Producer version after the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_producer_version: Option<String>,
    /// Schema version before the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_schema_version: Option<u32>,
    /// Schema version after the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_schema_version: Option<u32>,
    /// Workspace epoch before the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_workspace_epoch: Option<String>,
    /// Workspace epoch after the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_workspace_epoch: Option<String>,
    /// Support-safe visible reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_reason: Option<String>,
    /// True when the event is visible to graph-backed consumers and exports.
    pub visible_to_consumers: bool,
}

impl GraphInvalidationEvent {
    fn is_valid(&self) -> bool {
        if self.event_id.trim().is_empty() || !self.visible_to_consumers {
            return false;
        }
        match self.invalidation_class {
            GraphInvalidationClass::NoInvalidationNeeded => true,
            GraphInvalidationClass::SmallestSubgraph => !self.affected_object_refs.is_empty(),
            GraphInvalidationClass::FullRebuildSchemaBoundary => {
                self.previous_schema_version.is_some()
                    && self.next_schema_version.is_some()
                    && self.has_visible_reason()
            }
            GraphInvalidationClass::FullRebuildProducerVersionBoundary => {
                self.previous_producer_version
                    .as_deref()
                    .map(|value| !value.trim().is_empty())
                    .unwrap_or(false)
                    && self
                        .next_producer_version
                        .as_deref()
                        .map(|value| !value.trim().is_empty())
                        .unwrap_or(false)
                    && self.has_visible_reason()
            }
            GraphInvalidationClass::FullRebuildWorkspaceEpochBoundary => {
                self.previous_workspace_epoch
                    .as_deref()
                    .map(|value| !value.trim().is_empty())
                    .unwrap_or(false)
                    && self
                        .next_workspace_epoch
                        .as_deref()
                        .map(|value| !value.trim().is_empty())
                        .unwrap_or(false)
                    && self.has_visible_reason()
            }
            GraphInvalidationClass::ArbitraryFullGraphRebuild => false,
        }
    }

    fn has_visible_reason(&self) -> bool {
        self.visible_reason
            .as_deref()
            .map(|reason| !reason.trim().is_empty())
            .unwrap_or(false)
    }
}

/// Consumer surface that must bind graph-linked actions to stable ids or query handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphConsumerSurface {
    /// Search result and ranking surfaces.
    Search,
    /// Review and impact-review surfaces.
    Review,
    /// Docs and citation surfaces.
    Docs,
    /// Navigation and breadcrumbs.
    Navigation,
    /// AI context and inspector surfaces.
    AiContext,
    /// Onboarding tours and explainers.
    OnboardingTour,
    /// Topology map, table, and list surfaces.
    TopologyMap,
    /// Support export bundle.
    SupportExport,
}

impl GraphConsumerSurface {
    /// Every required graph-backed consumer surface in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::Search,
        Self::Review,
        Self::Docs,
        Self::Navigation,
        Self::AiContext,
        Self::OnboardingTour,
        Self::TopologyMap,
        Self::SupportExport,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Review => "review",
            Self::Docs => "docs",
            Self::Navigation => "navigation",
            Self::AiContext => "ai_context",
            Self::OnboardingTour => "onboarding_tour",
            Self::TopologyMap => "topology_map",
            Self::SupportExport => "support_export",
        }
    }
}

/// Stable-claim posture for one graph-backed consumer action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphStableClaimLevel {
    /// Consumer may claim stable graph-backed behavior.
    Stable,
    /// Consumer is automatically narrowed below stable until migrated.
    NarrowedBelowStable,
}

impl GraphStableClaimLevel {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
        }
    }
}

/// Binding proving a graph-linked action carries stable object ids or query handles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphConsumerActionBinding {
    /// Consumer surface.
    pub consumer_surface: GraphConsumerSurface,
    /// Stable action ref.
    pub action_ref: String,
    /// Object ids embedded in the action.
    #[serde(default)]
    pub object_id_refs: Vec<String>,
    /// Query handles embedded in the action.
    #[serde(default)]
    pub query_handle_refs: Vec<String>,
    /// True when stable handles are embedded for reconstruction.
    pub embeds_stable_handles: bool,
    /// True when the surface still uses private graph objects.
    pub uses_private_graph_shape: bool,
    /// Hidden richer query family if one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_richer_query_family: Option<String>,
    /// True when inferred text or relations are presented without labels.
    pub unlabeled_inference_present: bool,
    /// Product claim level for this surface.
    pub stable_claim_level: GraphStableClaimLevel,
}

impl GraphConsumerActionBinding {
    fn has_reconstructable_binding(&self) -> bool {
        !self.action_ref.trim().is_empty()
            && self.embeds_stable_handles
            && (!self.object_id_refs.is_empty() || !self.query_handle_refs.is_empty())
    }

    fn requires_narrowed_claim(&self) -> bool {
        self.uses_private_graph_shape
            || self.hidden_richer_query_family.is_some()
            || self.unlabeled_inference_present
    }
}

/// Shared topology and fallback reuse proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphTopologyReuseProof {
    /// Stable proof id.
    pub proof_id: String,
    /// True when canvas nodes reuse stable object ids.
    pub canvas_reuses_node_id_space: bool,
    /// True when edge rows reuse stable relationship object ids.
    pub canvas_reuses_edge_id_space: bool,
    /// True when table/list fallback preserves stable ids.
    pub table_fallback_reuses_same_ids: bool,
    /// True when breadcrumbs preserve stable ids.
    pub breadcrumbs_reuse_same_ids: bool,
    /// True when AI inspectors preserve stable ids and query handles.
    pub ai_inspector_reuses_same_ids: bool,
    /// True when onboarding tours preserve stable ids and query handles.
    pub onboarding_tour_reuses_same_ids: bool,
    /// True when review explainers preserve stable ids and query handles.
    pub review_explainer_reuses_same_ids: bool,
    /// True when support packets preserve stable ids and query handles.
    pub support_export_reuses_same_ids: bool,
    /// True when every projection uses the shared scope vocabulary.
    pub uses_shared_scope_model: bool,
    /// True when every projection uses the shared freshness vocabulary.
    pub uses_shared_freshness_vocabulary: bool,
    /// True when every projection uses the shared confidence vocabulary.
    pub uses_shared_confidence_vocabulary: bool,
}

impl GraphTopologyReuseProof {
    fn is_valid(&self) -> bool {
        !self.proof_id.trim().is_empty()
            && self.canvas_reuses_node_id_space
            && self.canvas_reuses_edge_id_space
            && self.table_fallback_reuses_same_ids
            && self.breadcrumbs_reuse_same_ids
            && self.ai_inspector_reuses_same_ids
            && self.onboarding_tour_reuses_same_ids
            && self.review_explainer_reuses_same_ids
            && self.support_export_reuses_same_ids
            && self.uses_shared_scope_model
            && self.uses_shared_freshness_vocabulary
            && self.uses_shared_confidence_vocabulary
    }
}

/// Support/export projection that can reconstruct graph-backed answers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphSupportReconstructionProjection {
    /// Stable projection id.
    pub projection_id: String,
    /// Packet id preserved by the support export.
    pub packet_id_ref: String,
    /// Object refs present in the export.
    #[serde(default)]
    pub object_id_refs: Vec<String>,
    /// Query handles present in the export.
    #[serde(default)]
    pub query_handle_refs: Vec<String>,
    /// Query families present in the export.
    #[serde(default)]
    pub query_families: Vec<StableQueryFamily>,
    /// Invalidation event refs present in the export.
    #[serde(default)]
    pub invalidation_event_refs: Vec<String>,
    /// True when the exact graph objects can be reconstructed.
    pub reconstructs_exact_graph_objects: bool,
    /// True when query families and handles can be reconstructed.
    pub reconstructs_query_families: bool,
    /// True when evidence refs are preserved.
    pub preserves_evidence_links: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials or authority are excluded.
    pub ambient_authority_excluded: bool,
}

impl GraphSupportReconstructionProjection {
    fn is_valid_for(&self, packet_id: &str) -> bool {
        !self.projection_id.trim().is_empty()
            && self.packet_id_ref == packet_id
            && !self.object_id_refs.is_empty()
            && !self.query_handle_refs.is_empty()
            && !self.query_families.is_empty()
            && !self.invalidation_event_refs.is_empty()
            && self.reconstructs_exact_graph_objects
            && self.reconstructs_query_families
            && self.preserves_evidence_links
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
    }
}

/// Constructor input for [`SemanticGraphContractPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGraphContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Stable workspace id.
    pub workspace_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Stable query-family vocabulary carried by the packet.
    #[serde(default)]
    pub stable_query_families: Vec<StableQueryFamily>,
    /// Stable graph objects.
    #[serde(default)]
    pub objects: Vec<StableGraphObject>,
    /// Stable query handles.
    #[serde(default)]
    pub query_handles: Vec<StableGraphQueryHandle>,
    /// Invalidation events.
    #[serde(default)]
    pub invalidation_events: Vec<GraphInvalidationEvent>,
    /// Consumer action bindings.
    #[serde(default)]
    pub consumer_action_bindings: Vec<GraphConsumerActionBinding>,
    /// Topology reuse proof.
    pub topology_reuse: GraphTopologyReuseProof,
    /// Support reconstruction projection.
    pub support_reconstruction: GraphSupportReconstructionProjection,
    /// Source contract refs used to build this packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Stable promotion state for the semantic-graph contract packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticGraphContractPromotionState {
    /// Packet certifies the stable graph contract.
    Stable,
    /// Packet is publishable only with narrowed product claims.
    NarrowedBelowStable,
    /// Packet has a blocker and cannot certify stable graph-backed behavior.
    BlocksStable,
}

impl SemanticGraphContractPromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one semantic-graph contract finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticGraphContractFindingSeverity {
    /// Reviewable finding that narrows a claim below stable.
    Warning,
    /// Blocker that prevents stable certification.
    Blocker,
}

/// Closed finding vocabulary for [`SemanticGraphContractPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticGraphContractFindingKind {
    /// Record kind does not match the stable packet.
    WrongRecordKind,
    /// Schema version does not match the stable packet.
    WrongSchemaVersion,
    /// Packet id, workspace id, or generated timestamp is empty.
    MissingPacketIdentity,
    /// Required graph object kind is absent.
    MissingRequiredObjectKind,
    /// Object metadata is incomplete.
    MissingObjectMetadata,
    /// Relationship-edge object lacks source or target object id.
    RelationshipEdgeMissingEndpoints,
    /// Degraded object lacks a visible truth label.
    MissingObjectTruthLabel,
    /// Stable query-family vocabulary differs from the closed stable list.
    QueryFamilyVocabularyDrift,
    /// Query handle metadata is incomplete.
    MissingQueryHandleMetadata,
    /// Degraded query handle lacks a visible truth label.
    MissingQueryTruthLabel,
    /// Query handle references an unknown object id.
    QueryHandleReferencesUnknownObject,
    /// Invalidation event is incomplete.
    InvalidInvalidationEvent,
    /// Arbitrary full-graph rebuild was attempted.
    ArbitraryFullGraphRebuild,
    /// Required consumer surface binding is absent.
    MissingConsumerBinding,
    /// Consumer binding does not carry stable object ids or query handles.
    ConsumerBindingNotReconstructable,
    /// Consumer still uses private graph shape or hidden query families.
    ConsumerClaimMustBeNarrowed,
    /// Consumer was not narrowed despite private graph truth.
    ConsumerNarrowingMissing,
    /// Topology/list/breadcrumb/AI/review/support reuse proof is incomplete.
    TopologyReuseIncomplete,
    /// Support export cannot reconstruct exact objects, query handles, or evidence.
    SupportReconstructionIncomplete,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl SemanticGraphContractFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingRequiredObjectKind => "missing_required_object_kind",
            Self::MissingObjectMetadata => "missing_object_metadata",
            Self::RelationshipEdgeMissingEndpoints => "relationship_edge_missing_endpoints",
            Self::MissingObjectTruthLabel => "missing_object_truth_label",
            Self::QueryFamilyVocabularyDrift => "query_family_vocabulary_drift",
            Self::MissingQueryHandleMetadata => "missing_query_handle_metadata",
            Self::MissingQueryTruthLabel => "missing_query_truth_label",
            Self::QueryHandleReferencesUnknownObject => "query_handle_references_unknown_object",
            Self::InvalidInvalidationEvent => "invalid_invalidation_event",
            Self::ArbitraryFullGraphRebuild => "arbitrary_full_graph_rebuild",
            Self::MissingConsumerBinding => "missing_consumer_binding",
            Self::ConsumerBindingNotReconstructable => "consumer_binding_not_reconstructable",
            Self::ConsumerClaimMustBeNarrowed => "consumer_claim_must_be_narrowed",
            Self::ConsumerNarrowingMissing => "consumer_narrowing_missing",
            Self::TopologyReuseIncomplete => "topology_reuse_incomplete",
            Self::SupportReconstructionIncomplete => "support_reconstruction_incomplete",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the semantic-graph contract validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGraphContractValidationFinding {
    /// Closed finding kind.
    pub finding_kind: SemanticGraphContractFindingKind,
    /// Finding severity.
    pub severity: SemanticGraphContractFindingSeverity,
    /// Support-safe summary.
    pub summary: String,
}

impl SemanticGraphContractValidationFinding {
    fn new(
        finding_kind: SemanticGraphContractFindingKind,
        severity: SemanticGraphContractFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Stable semantic-graph object/query contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGraphContractPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Stable query families.
    #[serde(default)]
    pub stable_query_families: Vec<StableQueryFamily>,
    /// Stable graph objects.
    #[serde(default)]
    pub objects: Vec<StableGraphObject>,
    /// Stable query handles.
    #[serde(default)]
    pub query_handles: Vec<StableGraphQueryHandle>,
    /// Invalidation events.
    #[serde(default)]
    pub invalidation_events: Vec<GraphInvalidationEvent>,
    /// Consumer action bindings.
    #[serde(default)]
    pub consumer_action_bindings: Vec<GraphConsumerActionBinding>,
    /// Topology reuse proof.
    pub topology_reuse: GraphTopologyReuseProof,
    /// Support reconstruction projection.
    pub support_reconstruction: GraphSupportReconstructionProjection,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: SemanticGraphContractPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<SemanticGraphContractValidationFinding>,
}

impl SemanticGraphContractPacket {
    /// Materialize a packet and record derived validation findings.
    pub fn materialize(input: SemanticGraphContractPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SEMANTIC_GRAPH_CONTRACT_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workspace_id: input.workspace_id,
            generated_at: input.generated_at,
            stable_query_families: input.stable_query_families,
            objects: input.objects,
            query_handles: input.query_handles,
            invalidation_events: input.invalidation_events,
            consumer_action_bindings: input.consumer_action_bindings,
            topology_reuse: input.topology_reuse,
            support_reconstruction: input.support_reconstruction,
            source_contract_refs: input.source_contract_refs,
            promotion_state: SemanticGraphContractPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validate the packet against the stable graph contract.
    pub fn validate(&self) -> Vec<SemanticGraphContractValidationFinding> {
        self.derived_findings(true)
    }

    /// True when this packet has no blocker finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == SemanticGraphContractFindingSeverity::Blocker)
    }

    /// Returns the object kind tokens carried by this packet.
    pub fn object_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for object in &self.objects {
            set.insert(object.object_kind);
        }
        set.into_iter().map(StableGraphObjectKind::as_str).collect()
    }

    /// Returns the stable query-family tokens carried by this packet.
    pub fn query_family_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for family in &self.stable_query_families {
            set.insert(*family);
        }
        set.into_iter().map(StableQueryFamily::as_str).collect()
    }

    /// Returns true when a consumer surface has a reconstructable binding.
    pub fn has_binding_for(&self, surface: GraphConsumerSurface) -> bool {
        self.consumer_action_bindings.iter().any(|binding| {
            binding.consumer_surface == surface && binding.has_reconstructable_binding()
        })
    }

    /// Build a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SemanticGraphContractSupportExport {
        SemanticGraphContractSupportExport {
            record_kind: SEMANTIC_GRAPH_CONTRACT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<SemanticGraphContractValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != SEMANTIC_GRAPH_CONTRACT_PACKET_RECORD_KIND {
            findings.push(SemanticGraphContractValidationFinding::new(
                SemanticGraphContractFindingKind::WrongRecordKind,
                SemanticGraphContractFindingSeverity::Blocker,
                "semantic-graph contract packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION {
            findings.push(SemanticGraphContractValidationFinding::new(
                SemanticGraphContractFindingKind::WrongSchemaVersion,
                SemanticGraphContractFindingSeverity::Blocker,
                "semantic-graph contract packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workspace_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(SemanticGraphContractValidationFinding::new(
                SemanticGraphContractFindingKind::MissingPacketIdentity,
                SemanticGraphContractFindingSeverity::Blocker,
                "packet id, workspace id, and generated timestamp are required",
            ));
        }

        let actual_families: BTreeSet<StableQueryFamily> =
            self.stable_query_families.iter().copied().collect();
        let required_families: BTreeSet<StableQueryFamily> =
            StableQueryFamily::REQUIRED.into_iter().collect();
        if actual_families != required_families
            || self.stable_query_families.len() != StableQueryFamily::REQUIRED.len()
        {
            findings.push(SemanticGraphContractValidationFinding::new(
                SemanticGraphContractFindingKind::QueryFamilyVocabularyDrift,
                SemanticGraphContractFindingSeverity::Blocker,
                "stable query families must be exactly lookup, neighborhood, explain-why, impact, ownership, diff-between-snapshots, and path-to-evidence",
            ));
        }

        let object_ids: BTreeSet<&str> = self
            .objects
            .iter()
            .map(|object| object.object_id.as_str())
            .collect();
        let query_handle_ids: BTreeSet<&str> = self
            .query_handles
            .iter()
            .map(|query| query.query_handle_id.as_str())
            .collect();
        let object_kinds: BTreeSet<StableGraphObjectKind> = self
            .objects
            .iter()
            .map(|object| object.object_kind)
            .collect();
        for required_kind in StableGraphObjectKind::REQUIRED {
            if !object_kinds.contains(&required_kind) {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::MissingRequiredObjectKind,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!("packet is missing object kind {}", required_kind.as_str()),
                ));
            }
        }

        for object in &self.objects {
            if !object.has_required_metadata() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::MissingObjectMetadata,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!("object {} is missing stable metadata", object.object_id),
                ));
            }
            if !object.has_relationship_endpoints() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::RelationshipEdgeMissingEndpoints,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "relationship-edge object {} lacks source or target object id",
                        object.object_id
                    ),
                ));
            }
            if !object.has_required_truth_label() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::MissingObjectTruthLabel,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "object {} lacks a required visible truth label",
                        object.object_id
                    ),
                ));
            }
        }

        for query in &self.query_handles {
            if !query.is_valid() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::MissingQueryHandleMetadata,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "query handle {} is missing stable metadata",
                        query.query_handle_id
                    ),
                ));
            }
            if !query.has_required_truth_label() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::MissingQueryTruthLabel,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "query handle {} lacks a required visible truth label",
                        query.query_handle_id
                    ),
                ));
            }
            for object_ref in &query.object_refs {
                if !object_ids.contains(object_ref.as_str()) {
                    findings.push(SemanticGraphContractValidationFinding::new(
                        SemanticGraphContractFindingKind::QueryHandleReferencesUnknownObject,
                        SemanticGraphContractFindingSeverity::Blocker,
                        format!(
                            "query handle {} references unknown object {}",
                            query.query_handle_id, object_ref
                        ),
                    ));
                }
            }
        }

        for event in &self.invalidation_events {
            if matches!(
                event.invalidation_class,
                GraphInvalidationClass::ArbitraryFullGraphRebuild
            ) {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::ArbitraryFullGraphRebuild,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "invalidation event {} attempts arbitrary full rebuild",
                        event.event_id
                    ),
                ));
            }
            if !event.is_valid() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::InvalidInvalidationEvent,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "invalidation event {} is incomplete for {}",
                        event.event_id,
                        event.invalidation_class.as_str()
                    ),
                ));
            }
        }

        for required_surface in GraphConsumerSurface::REQUIRED {
            if !self.has_binding_for(required_surface) {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::MissingConsumerBinding,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a reconstructable {} binding",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for binding in &self.consumer_action_bindings {
            if !binding.has_reconstructable_binding() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::ConsumerBindingNotReconstructable,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!(
                        "consumer binding {} does not embed stable ids or query handles",
                        binding.action_ref
                    ),
                ));
            }
            for object_ref in &binding.object_id_refs {
                if !object_ids.contains(object_ref.as_str()) {
                    findings.push(SemanticGraphContractValidationFinding::new(
                        SemanticGraphContractFindingKind::ConsumerBindingNotReconstructable,
                        SemanticGraphContractFindingSeverity::Blocker,
                        format!(
                            "consumer binding {} references unknown object {}",
                            binding.action_ref, object_ref
                        ),
                    ));
                }
            }
            for query_ref in &binding.query_handle_refs {
                if !query_handle_ids.contains(query_ref.as_str()) {
                    findings.push(SemanticGraphContractValidationFinding::new(
                        SemanticGraphContractFindingKind::ConsumerBindingNotReconstructable,
                        SemanticGraphContractFindingSeverity::Blocker,
                        format!(
                            "consumer binding {} references unknown query handle {}",
                            binding.action_ref, query_ref
                        ),
                    ));
                }
            }
            if binding.requires_narrowed_claim() {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::ConsumerClaimMustBeNarrowed,
                    SemanticGraphContractFindingSeverity::Warning,
                    format!(
                        "consumer binding {} uses private graph truth or unlabeled inference",
                        binding.action_ref
                    ),
                ));
                if binding.stable_claim_level != GraphStableClaimLevel::NarrowedBelowStable {
                    findings.push(SemanticGraphContractValidationFinding::new(
                        SemanticGraphContractFindingKind::ConsumerNarrowingMissing,
                        SemanticGraphContractFindingSeverity::Blocker,
                        format!(
                            "consumer binding {} must be narrowed below stable",
                            binding.action_ref
                        ),
                    ));
                }
            }
        }

        if !self.topology_reuse.is_valid() {
            findings.push(SemanticGraphContractValidationFinding::new(
                SemanticGraphContractFindingKind::TopologyReuseIncomplete,
                SemanticGraphContractFindingSeverity::Blocker,
                "topology, table/list fallback, breadcrumbs, AI, review, onboarding, and support must reuse one id/scope/freshness/confidence model",
            ));
        }
        if !self
            .support_reconstruction
            .is_valid_for(self.packet_id.as_str())
        {
            findings.push(SemanticGraphContractValidationFinding::new(
                SemanticGraphContractFindingKind::SupportReconstructionIncomplete,
                SemanticGraphContractFindingSeverity::Blocker,
                "support reconstruction must preserve exact objects, query handles, invalidation events, and evidence links",
            ));
        }
        for object_ref in &self.support_reconstruction.object_id_refs {
            if !object_ids.contains(object_ref.as_str()) {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::SupportReconstructionIncomplete,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!("support reconstruction references unknown object {object_ref}"),
                ));
            }
        }
        for query_ref in &self.support_reconstruction.query_handle_refs {
            if !query_handle_ids.contains(query_ref.as_str()) {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::SupportReconstructionIncomplete,
                    SemanticGraphContractFindingSeverity::Blocker,
                    format!("support reconstruction references unknown query handle {query_ref}"),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != SemanticGraphContractFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(SemanticGraphContractValidationFinding::new(
                    SemanticGraphContractFindingKind::PromotionStateMismatch,
                    SemanticGraphContractFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[SemanticGraphContractValidationFinding],
) -> SemanticGraphContractPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == SemanticGraphContractFindingSeverity::Blocker)
    {
        SemanticGraphContractPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == SemanticGraphContractFindingSeverity::Warning)
    {
        SemanticGraphContractPromotionState::NarrowedBelowStable
    } else {
        SemanticGraphContractPromotionState::Stable
    }
}

/// Support-export wrapper preserving the product graph contract packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGraphContractSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials or authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: SemanticGraphContractPacket,
}

impl SemanticGraphContractSupportExport {
    /// True when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEMANTIC_GRAPH_CONTRACT_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable semantic-graph contract packet.
#[derive(Debug)]
pub enum SemanticGraphContractArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<SemanticGraphContractValidationFinding>),
}

impl fmt::Display for SemanticGraphContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "semantic-graph contract packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "semantic-graph contract packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SemanticGraphContractArtifactError {}

/// Returns the checked-in stable semantic-graph contract packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_semantic_graph_contract_packet(
) -> Result<SemanticGraphContractPacket, SemanticGraphContractArtifactError> {
    let packet: SemanticGraphContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/graph/m4/semantic-graph-object-model-and-query-contract.json"
    )))
    .map_err(SemanticGraphContractArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SemanticGraphContractArtifactError::Validation(findings))
    }
}

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn producer(id: &str, kind: &str) -> GraphProducerIdentity {
    GraphProducerIdentity {
        producer_id: id.to_owned(),
        producer_kind: kind.to_owned(),
        producer_version: "2026.06.07".to_owned(),
    }
}

fn object(
    object_id: &str,
    object_kind: StableGraphObjectKind,
    freshness: GraphFreshnessState,
    confidence: GraphConfidenceTier,
    visibility_scope: GraphVisibilityScope,
    retention_class: GraphRetentionClass,
) -> StableGraphObject {
    StableGraphObject {
        object_id: object_id.to_owned(),
        object_kind,
        schema_version: SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION,
        producer: producer(
            "producer:graph.semantic-contract",
            "semantic_graph_contract",
        ),
        freshness_timestamp: "2026-06-07T00:35:00Z".to_owned(),
        freshness,
        confidence,
        visibility_scope,
        retention_class,
        provenance_refs: refs(&["provenance:graph.semantic-contract.seed"]),
        evidence_refs: refs(&["evidence:graph.semantic-contract.fixture"]),
        source_object_id: None,
        target_object_id: None,
        visible_truth_label: freshness
            .requires_visible_truth_label()
            .then(|| freshness.as_str().to_owned()),
    }
}

fn binding(
    surface: GraphConsumerSurface,
    object_refs: &[&str],
    query_refs: &[&str],
) -> GraphConsumerActionBinding {
    GraphConsumerActionBinding {
        consumer_surface: surface,
        action_ref: format!("action:graph-contract:{}", surface.as_str()),
        object_id_refs: refs(object_refs),
        query_handle_refs: refs(query_refs),
        embeds_stable_handles: true,
        uses_private_graph_shape: false,
        hidden_richer_query_family: None,
        unlabeled_inference_present: false,
        stable_claim_level: GraphStableClaimLevel::Stable,
    }
}

/// Returns the seeded stable packet input used by docs, artifacts, and tests.
pub fn seeded_semantic_graph_contract_input() -> SemanticGraphContractPacketInput {
    let mut objects = vec![
        object(
            "graph-object:workspace:root-workset",
            StableGraphObjectKind::WorkspaceRootWorkset,
            GraphFreshnessState::Live,
            GraphConfidenceTier::Semantic,
            GraphVisibilityScope::SelectedWorkset,
            GraphRetentionClass::DurableWorkspace,
        ),
        object(
            "graph-object:file:src-lib",
            StableGraphObjectKind::FileDocument,
            GraphFreshnessState::Live,
            GraphConfidenceTier::Structural,
            GraphVisibilityScope::SelectedWorkset,
            GraphRetentionClass::DurableWorkspace,
        ),
        object(
            "graph-object:symbol:api-graph-contract",
            StableGraphObjectKind::SymbolApi,
            GraphFreshnessState::Live,
            GraphConfidenceTier::Semantic,
            GraphVisibilityScope::SelectedWorkset,
            GraphRetentionClass::DurableWorkspace,
        ),
        object(
            "graph-object:docs:contract",
            StableGraphObjectKind::DocsKnowledge,
            GraphFreshnessState::Cached,
            GraphConfidenceTier::UserCurated,
            GraphVisibilityScope::SelectedWorkset,
            GraphRetentionClass::ExportableSnapshot,
        ),
        object(
            "graph-object:artifact:support-export",
            StableGraphObjectKind::OperationalArtifact,
            GraphFreshnessState::PartialScope,
            GraphConfidenceTier::VerifiedRuntime,
            GraphVisibilityScope::SelectedWorkset,
            GraphRetentionClass::ExportableSnapshot,
        ),
    ];
    let mut edge = object(
        "graph-object:edge:file-defines-symbol",
        StableGraphObjectKind::RelationshipEdge,
        GraphFreshnessState::Live,
        GraphConfidenceTier::Semantic,
        GraphVisibilityScope::SelectedWorkset,
        GraphRetentionClass::DurableWorkspace,
    );
    edge.source_object_id = Some("graph-object:file:src-lib".to_owned());
    edge.target_object_id = Some("graph-object:symbol:api-graph-contract".to_owned());
    objects.push(edge);

    let query_handles = StableQueryFamily::REQUIRED
        .iter()
        .map(|family| StableGraphQueryHandle {
            query_handle_id: format!("graph-query:{}", family.as_str()),
            query_family: *family,
            scope_ref: "scope:selected-workset:stable-graph-contract".to_owned(),
            snapshot_epoch: "graph-epoch:workspace:2026-06-07T00:35:00Z".to_owned(),
            delta_sequence: 42,
            producer: producer(
                "producer:graph.query-family-contract",
                "graph_query_contract",
            ),
            freshness: if matches!(
                family,
                StableQueryFamily::PathToEvidence | StableQueryFamily::DiffBetweenSnapshots
            ) {
                GraphFreshnessState::Cached
            } else {
                GraphFreshnessState::Live
            },
            confidence: GraphConfidenceTier::Semantic,
            object_refs: refs(&[
                "graph-object:workspace:root-workset",
                "graph-object:file:src-lib",
                "graph-object:symbol:api-graph-contract",
                "graph-object:edge:file-defines-symbol",
            ]),
            evidence_refs: refs(&["evidence:graph.query-family-contract.fixture"]),
            visible_truth_label: matches!(
                family,
                StableQueryFamily::PathToEvidence | StableQueryFamily::DiffBetweenSnapshots
            )
            .then(|| "cached".to_owned()),
        })
        .collect::<Vec<_>>();

    SemanticGraphContractPacketInput {
        packet_id: "packet:graph.semantic-contract:stable".to_owned(),
        workspace_id: "workspace:aureline:contract-fixture".to_owned(),
        generated_at: "2026-06-07T00:35:00Z".to_owned(),
        stable_query_families: StableQueryFamily::REQUIRED.to_vec(),
        objects,
        query_handles,
        invalidation_events: vec![
            GraphInvalidationEvent {
                event_id: "invalidation:subgraph:file-src-lib".to_owned(),
                invalidation_class: GraphInvalidationClass::SmallestSubgraph,
                affected_object_refs: refs(&[
                    "graph-object:file:src-lib",
                    "graph-object:symbol:api-graph-contract",
                    "graph-object:edge:file-defines-symbol",
                ]),
                previous_producer_version: None,
                next_producer_version: None,
                previous_schema_version: None,
                next_schema_version: None,
                previous_workspace_epoch: None,
                next_workspace_epoch: None,
                visible_reason: Some(
                    "file edit invalidated only the affected file/symbol edge".to_owned(),
                ),
                visible_to_consumers: true,
            },
            GraphInvalidationEvent {
                event_id: "invalidation:producer-version:graph-contract".to_owned(),
                invalidation_class: GraphInvalidationClass::FullRebuildProducerVersionBoundary,
                affected_object_refs: Vec::new(),
                previous_producer_version: Some("2026.06.06".to_owned()),
                next_producer_version: Some("2026.06.07".to_owned()),
                previous_schema_version: None,
                next_schema_version: None,
                previous_workspace_epoch: None,
                next_workspace_epoch: None,
                visible_reason: Some(
                    "producer-version boundary required a visible full rebuild".to_owned(),
                ),
                visible_to_consumers: true,
            },
        ],
        consumer_action_bindings: GraphConsumerSurface::REQUIRED
            .iter()
            .map(|surface| {
                binding(
                    *surface,
                    &["graph-object:symbol:api-graph-contract"],
                    &["graph-query:lookup", "graph-query:path-to-evidence"],
                )
            })
            .collect(),
        topology_reuse: GraphTopologyReuseProof {
            proof_id: "topology-reuse:graph.semantic-contract".to_owned(),
            canvas_reuses_node_id_space: true,
            canvas_reuses_edge_id_space: true,
            table_fallback_reuses_same_ids: true,
            breadcrumbs_reuse_same_ids: true,
            ai_inspector_reuses_same_ids: true,
            onboarding_tour_reuses_same_ids: true,
            review_explainer_reuses_same_ids: true,
            support_export_reuses_same_ids: true,
            uses_shared_scope_model: true,
            uses_shared_freshness_vocabulary: true,
            uses_shared_confidence_vocabulary: true,
        },
        support_reconstruction: GraphSupportReconstructionProjection {
            projection_id: "support-reconstruction:graph.semantic-contract".to_owned(),
            packet_id_ref: "packet:graph.semantic-contract:stable".to_owned(),
            object_id_refs: refs(&[
                "graph-object:workspace:root-workset",
                "graph-object:file:src-lib",
                "graph-object:symbol:api-graph-contract",
                "graph-object:edge:file-defines-symbol",
                "graph-object:docs:contract",
                "graph-object:artifact:support-export",
            ]),
            query_handle_refs: StableQueryFamily::REQUIRED
                .iter()
                .map(|family| format!("graph-query:{}", family.as_str()))
                .collect(),
            query_families: StableQueryFamily::REQUIRED.to_vec(),
            invalidation_event_refs: refs(&[
                "invalidation:subgraph:file-src-lib",
                "invalidation:producer-version:graph-contract",
            ]),
            reconstructs_exact_graph_objects: true,
            reconstructs_query_families: true,
            preserves_evidence_links: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        },
        source_contract_refs: refs(&[
            SEMANTIC_GRAPH_CONTRACT_DOC_REF,
            SEMANTIC_GRAPH_CONTRACT_ARTIFACT_DOC_REF,
            SEMANTIC_GRAPH_CONTRACT_SCHEMA_REF,
            SEMANTIC_GRAPH_CONTRACT_FIXTURE_DIR,
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_query_family_tokens_are_pinned() {
        assert_eq!(StableQueryFamily::Lookup.as_str(), "lookup");
        assert_eq!(StableQueryFamily::Neighborhood.as_str(), "neighborhood");
        assert_eq!(StableQueryFamily::ExplainWhy.as_str(), "explain-why");
        assert_eq!(StableQueryFamily::Impact.as_str(), "impact");
        assert_eq!(StableQueryFamily::Ownership.as_str(), "ownership");
        assert_eq!(
            StableQueryFamily::DiffBetweenSnapshots.as_str(),
            "diff-between-snapshots"
        );
        assert_eq!(
            StableQueryFamily::PathToEvidence.as_str(),
            "path-to-evidence"
        );
    }

    #[test]
    fn seeded_packet_certifies_stable() {
        let packet =
            SemanticGraphContractPacket::materialize(seeded_semantic_graph_contract_input());
        assert_eq!(
            packet.promotion_state,
            SemanticGraphContractPromotionState::Stable,
            "unexpected findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert_eq!(
            packet.query_family_tokens(),
            vec![
                "lookup",
                "neighborhood",
                "explain-why",
                "impact",
                "ownership",
                "diff-between-snapshots",
                "path-to-evidence"
            ]
        );
    }

    #[test]
    fn arbitrary_full_rebuild_blocks_stable() {
        let mut input = seeded_semantic_graph_contract_input();
        input.invalidation_events.push(GraphInvalidationEvent {
            event_id: "invalidation:bad:arbitrary".to_owned(),
            invalidation_class: GraphInvalidationClass::ArbitraryFullGraphRebuild,
            affected_object_refs: Vec::new(),
            previous_producer_version: None,
            next_producer_version: None,
            previous_schema_version: None,
            next_schema_version: None,
            previous_workspace_epoch: None,
            next_workspace_epoch: None,
            visible_reason: Some("ordinary freshness churn".to_owned()),
            visible_to_consumers: true,
        });
        let packet = SemanticGraphContractPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            SemanticGraphContractPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == SemanticGraphContractFindingKind::ArbitraryFullGraphRebuild
        }));
    }

    #[test]
    fn private_consumer_shape_is_narrowed_below_stable() {
        let mut input = seeded_semantic_graph_contract_input();
        input.consumer_action_bindings[0].uses_private_graph_shape = true;
        input.consumer_action_bindings[0].stable_claim_level =
            GraphStableClaimLevel::NarrowedBelowStable;
        let packet = SemanticGraphContractPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            SemanticGraphContractPromotionState::NarrowedBelowStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == SemanticGraphContractFindingKind::ConsumerClaimMustBeNarrowed
        }));
    }
}
