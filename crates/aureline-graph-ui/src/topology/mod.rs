//! Topology-map projections with list and table parity.
//!
//! Topology renderers consume [`TopologySurface`] so the canvas, list alternate,
//! table alternate, exports, and support packets all use the same node ids,
//! edge ids, scope disclosure, relation legend, and freshness strip.

use std::collections::BTreeSet;

use aureline_graph::{
    EdgeEvidenceState, ImpactExplainerPacket, OpenDetailActionClass, TopologyEdgeProjection,
};
use serde::{Deserialize, Serialize};

/// Schema version for graph topology surface projections.
pub const TOPOLOGY_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for topology surfaces.
pub const TOPOLOGY_SURFACE_RECORD_KIND: &str = "topology_surface_record";

/// Scope vocabulary shared by topology, impact, AI context, review hints, and exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeVocabularyClass {
    /// The active result is bounded to the current repository.
    CurrentRepo,
    /// The active result is bounded to a selected workset.
    SelectedWorkset,
    /// The active result covers the full workspace.
    FullWorkspace,
    /// The active result came from a remote cache and must stay labeled as such.
    RemoteCache,
    /// The result is known to be outside the current scope.
    OutsideCurrentScope,
}

impl ScopeVocabularyClass {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepo => "current_repo",
            Self::SelectedWorkset => "selected_workset",
            Self::FullWorkspace => "full_workspace",
            Self::RemoteCache => "remote_cache",
            Self::OutsideCurrentScope => "outside_current_scope",
        }
    }

    /// Returns the user-facing label mandated by the UI vocabulary.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CurrentRepo => "Current repo",
            Self::SelectedWorkset => "Selected workset",
            Self::FullWorkspace => "Full workspace",
            Self::RemoteCache => "Remote cache",
            Self::OutsideCurrentScope => "Outside current scope",
        }
    }

    /// Maps graph and workspace scope tokens onto the shared UI vocabulary.
    pub fn from_graph_scope_token(token: &str) -> Self {
        match token {
            "current_repo" | "current_root" => Self::CurrentRepo,
            "full_workspace" => Self::FullWorkspace,
            "remote_cache" | "remote_agent" => Self::RemoteCache,
            "outside_current_scope" | "out_of_scope" => Self::OutsideCurrentScope,
            "selected_workset"
            | "named_workset"
            | "sparse_slice"
            | "policy_limited_view"
            | "review_workspace"
            | "companion_surface" => Self::SelectedWorkset,
            _ => Self::SelectedWorkset,
        }
    }

    /// Returns every token in the shared vocabulary.
    pub const fn all() -> &'static [Self] {
        &[
            Self::CurrentRepo,
            Self::SelectedWorkset,
            Self::FullWorkspace,
            Self::RemoteCache,
            Self::OutsideCurrentScope,
        ]
    }
}

/// Partiality or omission state that must survive UI and export paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphDisclosureState {
    /// The graph query is explicitly partial.
    PartialGraph,
    /// An imported fact contributed to the result.
    ImportedFact,
    /// A parser-only fallback contributed to the result.
    ParserOnlyFallback,
    /// Policy hid part of the graph.
    PolicyHidden,
    /// The result came from a remote cache.
    RemoteCache,
    /// Matching graph evidence exists outside the active scope.
    OutsideCurrentScope,
    /// A generated or heuristic relation contributed to the result.
    GeneratedOrHeuristic,
}

impl GraphDisclosureState {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialGraph => "partial_graph",
            Self::ImportedFact => "imported_fact",
            Self::ParserOnlyFallback => "parser_only_fallback",
            Self::PolicyHidden => "policy_hidden",
            Self::RemoteCache => "remote_cache",
            Self::OutsideCurrentScope => "outside_current_scope",
            Self::GeneratedOrHeuristic => "generated_or_heuristic",
        }
    }
}

/// Relation class rendered by the topology legend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationLegendClass {
    /// Direct structural relations such as imports, declarations, and calls.
    ExactStructuralEdge,
    /// Build, runtime, deployment, or hosting relations.
    BuildRuntimeRelation,
    /// Ownership, role, or policy controller relations.
    OwnershipPolicyRelation,
    /// Coverage or test relations.
    CoverageTestRelation,
    /// Generated or derived artifact relations.
    GeneratedDerivedRelation,
    /// Heuristic or summary relations that must not render as exact.
    HeuristicSummaryRelation,
}

impl RelationLegendClass {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactStructuralEdge => "exact_structural_edge",
            Self::BuildRuntimeRelation => "build_runtime_relation",
            Self::OwnershipPolicyRelation => "ownership_policy_relation",
            Self::CoverageTestRelation => "coverage_test_relation",
            Self::GeneratedDerivedRelation => "generated_derived_relation",
            Self::HeuristicSummaryRelation => "heuristic_summary_relation",
        }
    }

    /// Classifies an edge by edge class and evidence state.
    pub fn from_edge(edge: &TopologyEdgeProjection) -> Self {
        if matches!(
            edge.evidence_state.as_str(),
            "inferred_relation" | "stale_relation" | "missing_anchor"
        ) {
            return Self::HeuristicSummaryRelation;
        }
        match edge.edge_class.as_str() {
            "owned_by" | "scoped_by" => Self::OwnershipPolicyRelation,
            "generated_from" | "produces_artifact" | "consumes_artifact" => {
                Self::GeneratedDerivedRelation
            }
            "deployed_to" | "runs_in" | "hosted_by" | "depends_on" => Self::BuildRuntimeRelation,
            "impacts" if edge.evidence_state == EdgeEvidenceState::InferredRelation.as_str() => {
                Self::HeuristicSummaryRelation
            }
            _ => Self::ExactStructuralEdge,
        }
    }
}

/// Selection state shared by visual and table projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionState {
    /// The row is not selected.
    NotSelected,
    /// The row is selected in the current surface state.
    Selected,
}

/// One action exposed by topology, impact, or explainer surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAction {
    /// Stable action id.
    pub action_id: String,
    /// Stable action class token.
    pub action_class: String,
    /// Graph object, surface record, or export packet this action opens.
    pub subject_ref: String,
    /// Whether the action preserves the active scope boundary.
    pub preserves_scope: bool,
}

/// One relation legend row visible beside the map and in exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationLegendEntry {
    /// Legend class being disclosed.
    pub relation_class: RelationLegendClass,
    /// Edge classes covered by this legend row.
    pub edge_classes: Vec<String>,
    /// Human-readable disclosure text.
    pub description: String,
}

/// One filter state entry copied to alternates and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceFilterState {
    /// Stable filter id.
    pub filter_id: String,
    /// Filter class token.
    pub filter_class: String,
    /// Whether the filter is engaged.
    pub engaged: bool,
    /// Selected values, when the filter is engaged.
    pub selected_values: Vec<String>,
}

/// Freshness, provenance, and scope strip rendered above graph surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphFreshnessProvenanceStrip {
    /// Workspace graph epoch or snapshot id.
    pub graph_epoch: String,
    /// Shared scope vocabulary class.
    pub scope_class: ScopeVocabularyClass,
    /// Active scope id copied from the graph or workset object.
    pub scope_id: String,
    /// Readiness token copied from the graph packet.
    pub readiness: String,
    /// Provenance summary text safe for support packets.
    pub provenance_note: String,
    /// Explicit partiality and omission disclosures.
    pub disclosures: Vec<GraphDisclosureState>,
}

/// One visible topology node and its list/table alternate row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyNodeRow {
    /// Canonical graph node id.
    pub node_id: String,
    /// Node class token copied from the graph object.
    pub node_class: String,
    /// Redaction-aware display label.
    pub display_label: String,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Selection state shared with the canvas.
    pub selection_state: SelectionState,
}

/// One visible topology edge and its table alternate row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEdgeRow {
    /// Canonical graph edge id.
    pub edge_id: String,
    /// Edge class token copied from the graph object.
    pub edge_class: String,
    /// Canonical source node id.
    pub from_node_id: String,
    /// Canonical target node id.
    pub to_node_id: String,
    /// Legend relation class shown for this edge.
    pub relation_class: RelationLegendClass,
    /// Edge evidence state copied from the graph object.
    pub evidence_state: String,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Selection state shared with the canvas.
    pub selection_state: SelectionState,
}

/// Disclosure attached to an aggregated cluster before it can be trusted as complete.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyClusterDisclosure {
    /// Stable cluster id.
    pub cluster_id: String,
    /// Count of graph entities represented by the cluster.
    pub count: usize,
    /// Category or family represented by the cluster.
    pub category: String,
    /// Omitted graph classes, if any.
    pub omitted_classes: Vec<String>,
    /// Scope note rendered with the cluster.
    pub scope_note: String,
    /// Freshness note rendered with the cluster.
    pub freshness_note: String,
}

/// Exportable topology-map surface with canvas/list/table parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologySurface {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable surface id.
    pub surface_id: String,
    /// Workspace id this surface describes.
    pub workspace_id: String,
    /// Workspace graph id used to build the surface.
    pub workspace_graph_id: String,
    /// Shared scope vocabulary class.
    pub scope_class: ScopeVocabularyClass,
    /// Nodes visible on the canvas.
    pub canvas_nodes: Vec<TopologyNodeRow>,
    /// Edges visible on the canvas.
    pub canvas_edges: Vec<TopologyEdgeRow>,
    /// Equivalent node list/table rows.
    pub node_table_rows: Vec<TopologyNodeRow>,
    /// Equivalent edge table rows.
    pub edge_table_rows: Vec<TopologyEdgeRow>,
    /// Relation legend rows rendered beside the map.
    pub relation_legend: Vec<RelationLegendEntry>,
    /// Filter state shared by the map and alternates.
    pub filter_state: Vec<SurfaceFilterState>,
    /// Freshness, provenance, and scope strip.
    pub freshness_provenance_strip: GraphFreshnessProvenanceStrip,
    /// Open-source actions for cited nodes.
    pub open_source_actions: Vec<SurfaceAction>,
    /// Open-evidence actions for graph relations.
    pub open_evidence_actions: Vec<SurfaceAction>,
    /// Export actions for parity packets.
    pub export_actions: Vec<SurfaceAction>,
    /// Cluster disclosures for aggregated map nodes.
    pub cluster_disclosures: Vec<TopologyClusterDisclosure>,
}

impl TopologySurface {
    /// Builds a topology surface from an impact explainer packet.
    pub fn from_impact_packet(packet: &ImpactExplainerPacket) -> Self {
        let scope_class =
            ScopeVocabularyClass::from_graph_scope_token(&packet.workset_scope.scope_class);
        let canvas_nodes = packet
            .visual_projection
            .nodes
            .iter()
            .map(|node| TopologyNodeRow {
                node_id: node.node_id.clone(),
                node_class: node.node_class.clone(),
                display_label: node.display_label.clone(),
                freshness: node.freshness.clone(),
                confidence: node.confidence.clone(),
                selection_state: selection_for_node(packet, &node.node_id),
            })
            .collect::<Vec<_>>();
        let canvas_edges = packet
            .visual_projection
            .edges
            .iter()
            .map(edge_row)
            .collect::<Vec<_>>();
        let relation_legend = relation_legend(&canvas_edges);
        let actions = packet
            .evidence_card
            .open_detail_actions
            .iter()
            .map(|action| SurfaceAction {
                action_id: action.action_id.clone(),
                action_class: action.action_class.as_str().to_owned(),
                subject_ref: action.subject_ref.clone(),
                preserves_scope: action.preserves_scope,
            })
            .collect::<Vec<_>>();
        let open_source_actions = actions
            .iter()
            .filter(|action| {
                action.action_class == OpenDetailActionClass::OpenSourceAtAnchor.as_str()
            })
            .cloned()
            .collect::<Vec<_>>();
        let open_evidence_actions = actions
            .iter()
            .filter(|action| {
                action.action_class == OpenDetailActionClass::InspectGraphRelation.as_str()
                    || action.action_class == OpenDetailActionClass::OpenTopologyTable.as_str()
            })
            .cloned()
            .collect::<Vec<_>>();
        let export_actions = actions
            .iter()
            .filter(|action| {
                action.action_class == OpenDetailActionClass::ExportEvidenceCard.as_str()
            })
            .cloned()
            .collect::<Vec<_>>();

        Self {
            record_kind: TOPOLOGY_SURFACE_RECORD_KIND.to_owned(),
            schema_version: TOPOLOGY_SURFACE_SCHEMA_VERSION,
            surface_id: format!("surface:topology:{}", packet.query_request_id),
            workspace_id: packet.workspace_id.clone(),
            workspace_graph_id: packet.workspace_graph_id.clone(),
            scope_class,
            node_table_rows: canvas_nodes.clone(),
            edge_table_rows: canvas_edges.clone(),
            canvas_nodes,
            canvas_edges,
            relation_legend,
            filter_state: vec![SurfaceFilterState {
                filter_id: format!("filter:{}:scope", packet.query_request_id),
                filter_class: "scope_visibility_filter".to_owned(),
                engaged: true,
                selected_values: vec![scope_class.as_str().to_owned()],
            }],
            freshness_provenance_strip: GraphFreshnessProvenanceStrip {
                graph_epoch: packet.workspace_graph_id.clone(),
                scope_class,
                scope_id: packet.workset_scope.scope_id.clone(),
                readiness: packet.readiness.clone(),
                provenance_note: packet.coverage_claim.clone(),
                disclosures: disclosure_states(packet),
            },
            open_source_actions,
            open_evidence_actions,
            export_actions,
            cluster_disclosures: cluster_disclosures(packet),
        }
    }

    /// Verifies every visible canvas node and edge has an equivalent table row.
    pub fn validate_table_parity(&self) -> Result<(), SurfaceParityError> {
        let canvas_node_ids = self
            .canvas_nodes
            .iter()
            .map(|node| node.node_id.as_str())
            .collect::<BTreeSet<_>>();
        let table_node_ids = self
            .node_table_rows
            .iter()
            .map(|node| node.node_id.as_str())
            .collect::<BTreeSet<_>>();
        if canvas_node_ids != table_node_ids {
            return Err(SurfaceParityError::NodeIdentityMismatch);
        }

        let canvas_edge_ids = self
            .canvas_edges
            .iter()
            .map(|edge| edge.edge_id.as_str())
            .collect::<BTreeSet<_>>();
        let table_edge_ids = self
            .edge_table_rows
            .iter()
            .map(|edge| edge.edge_id.as_str())
            .collect::<BTreeSet<_>>();
        if canvas_edge_ids != table_edge_ids {
            return Err(SurfaceParityError::EdgeIdentityMismatch);
        }

        for canvas_row in &self.canvas_nodes {
            let table_row = self
                .node_table_rows
                .iter()
                .find(|row| row.node_id == canvas_row.node_id)
                .ok_or(SurfaceParityError::NodeIdentityMismatch)?;
            if table_row.freshness != canvas_row.freshness
                || table_row.confidence != canvas_row.confidence
                || table_row.selection_state != canvas_row.selection_state
            {
                return Err(SurfaceParityError::NodeTruthMismatch {
                    node_id: canvas_row.node_id.clone(),
                });
            }
        }

        for canvas_row in &self.canvas_edges {
            let table_row = self
                .edge_table_rows
                .iter()
                .find(|row| row.edge_id == canvas_row.edge_id)
                .ok_or(SurfaceParityError::EdgeIdentityMismatch)?;
            if table_row.freshness != canvas_row.freshness
                || table_row.confidence != canvas_row.confidence
                || table_row.selection_state != canvas_row.selection_state
                || table_row.relation_class != canvas_row.relation_class
            {
                return Err(SurfaceParityError::EdgeTruthMismatch {
                    edge_id: canvas_row.edge_id.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Error returned when a topology surface loses canvas/table parity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceParityError {
    /// Canvas node ids differ from table node ids.
    NodeIdentityMismatch,
    /// Canvas edge ids differ from table edge ids.
    EdgeIdentityMismatch,
    /// A node row has matching identity but mismatched truth fields.
    NodeTruthMismatch {
        /// Node id with mismatched fields.
        node_id: String,
    },
    /// An edge row has matching identity but mismatched truth fields.
    EdgeTruthMismatch {
        /// Edge id with mismatched fields.
        edge_id: String,
    },
}

fn edge_row(edge: &TopologyEdgeProjection) -> TopologyEdgeRow {
    TopologyEdgeRow {
        edge_id: edge.edge_id.clone(),
        edge_class: edge.edge_class.clone(),
        from_node_id: edge.from_node_id.clone(),
        to_node_id: edge.to_node_id.clone(),
        relation_class: RelationLegendClass::from_edge(edge),
        evidence_state: edge.evidence_state.clone(),
        freshness: edge.freshness.clone(),
        confidence: edge.confidence.clone(),
        selection_state: SelectionState::NotSelected,
    }
}

fn selection_for_node(packet: &ImpactExplainerPacket, node_id: &str) -> SelectionState {
    if node_id == packet.subject_node_id {
        SelectionState::Selected
    } else {
        SelectionState::NotSelected
    }
}

fn relation_legend(edges: &[TopologyEdgeRow]) -> Vec<RelationLegendEntry> {
    let mut classes = BTreeSet::new();
    for edge in edges {
        classes.insert(edge.relation_class);
    }
    classes
        .into_iter()
        .map(|relation_class| {
            let edge_classes = edges
                .iter()
                .filter(|edge| edge.relation_class == relation_class)
                .map(|edge| edge.edge_class.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            RelationLegendEntry {
                relation_class,
                edge_classes,
                description: relation_description(relation_class).to_owned(),
            }
        })
        .collect()
}

fn relation_description(relation_class: RelationLegendClass) -> &'static str {
    match relation_class {
        RelationLegendClass::ExactStructuralEdge => {
            "Exact structural relation with source and target evidence."
        }
        RelationLegendClass::BuildRuntimeRelation => {
            "Build, runtime, deployment, or hosting relation with freshness."
        }
        RelationLegendClass::OwnershipPolicyRelation => {
            "Ownership, role, or policy relation with authority source."
        }
        RelationLegendClass::CoverageTestRelation => {
            "Coverage or test relation with exactness disclosure."
        }
        RelationLegendClass::GeneratedDerivedRelation => {
            "Generated or derived relation with generator or lineage source."
        }
        RelationLegendClass::HeuristicSummaryRelation => {
            "Heuristic or summary relation that must not be treated as exact."
        }
    }
}

fn disclosure_states(packet: &ImpactExplainerPacket) -> Vec<GraphDisclosureState> {
    let mut states = BTreeSet::new();
    if matches!(
        packet.readiness.as_str(),
        "partial" | "warming" | "hot_set_ready" | "stale"
    ) || packet.workset_scope.index_coverage.not_loaded_count > 0
    {
        states.insert(GraphDisclosureState::PartialGraph);
    }
    if packet.impact_summary.out_of_scope_count > 0 || packet.workset_scope.hidden_result_count > 0
    {
        states.insert(GraphDisclosureState::OutsideCurrentScope);
    }
    for cause in &packet.partial_truth_causes {
        match cause.as_str() {
            "imported" | "replayed" => {
                states.insert(GraphDisclosureState::ImportedFact);
            }
            "policy_hidden" => {
                states.insert(GraphDisclosureState::PolicyHidden);
            }
            "derived" | "low_confidence" | "missing_anchor" => {
                states.insert(GraphDisclosureState::GeneratedOrHeuristic);
            }
            "partial_scope" => {
                states.insert(GraphDisclosureState::PartialGraph);
            }
            _ => {}
        }
    }
    if states.is_empty() {
        states.insert(GraphDisclosureState::PartialGraph);
    }
    states.into_iter().collect()
}

fn cluster_disclosures(packet: &ImpactExplainerPacket) -> Vec<TopologyClusterDisclosure> {
    if packet.impact_summary.out_of_scope_count == 0 {
        return Vec::new();
    }
    vec![TopologyClusterDisclosure {
        cluster_id: format!("cluster:{}:outside_scope", packet.query_request_id),
        count: packet.impact_summary.out_of_scope_count,
        category: "outside_current_scope".to_owned(),
        omitted_classes: vec!["impact_edge".to_owned()],
        scope_note: "Additional impact edges exist outside the active scope.".to_owned(),
        freshness_note: packet.readiness.clone(),
    }]
}
