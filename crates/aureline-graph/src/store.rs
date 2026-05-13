//! Validated graph snapshot storage and query execution.

use std::collections::{BTreeMap, BTreeSet};

use aureline_graph_proto::{
    validate_graph, EdgeClass, GraphEdge, GraphNode, NodeBody, NodeClass, QueryFamilyTag,
    ValidationError, WorkspaceGraph,
};

use crate::query::{
    GraphAlphaQueryClass, GraphQueryEnvelope, GraphQueryFamilyDescriptor, GraphQueryRequest,
    GraphQueryRow,
};

/// Error returned when a graph snapshot cannot be persisted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphStoreError {
    /// The canonical graph seed validator rejected the snapshot.
    ValidationFailed(Vec<ValidationError>),
}

/// Stores one validated semantic graph snapshot for alpha query execution.
#[derive(Debug, Clone)]
pub struct GraphStore {
    workspace_graph_id: String,
    workspace_id: String,
    recorded_at: String,
    nodes: BTreeMap<String, GraphNode>,
    edges: BTreeMap<String, GraphEdge>,
}

impl GraphStore {
    /// Persists a validated workspace graph snapshot into the alpha store.
    pub fn persist_snapshot(graph: WorkspaceGraph) -> Result<Self, GraphStoreError> {
        let (errors, _hooks) = validate_graph(&graph);
        if !errors.is_empty() {
            return Err(GraphStoreError::ValidationFailed(errors));
        }

        Ok(Self {
            workspace_graph_id: graph.workspace_graph_id,
            workspace_id: graph.workspace_id,
            recorded_at: graph.recorded_at,
            nodes: graph
                .nodes
                .into_iter()
                .map(|node| (node.node_id.clone(), node))
                .collect(),
            edges: graph
                .edges
                .into_iter()
                .map(|edge| (edge.edge_id.clone(), edge))
                .collect(),
        })
    }

    /// Alias for [`GraphStore::persist_snapshot`] for call sites that name the graph input.
    pub fn from_workspace_graph(graph: WorkspaceGraph) -> Result<Self, GraphStoreError> {
        Self::persist_snapshot(graph)
    }

    /// Returns the stored workspace graph id.
    pub fn workspace_graph_id(&self) -> &str {
        &self.workspace_graph_id
    }

    /// Returns the stored workspace id.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Returns the graph snapshot timestamp used for alpha envelopes.
    pub fn recorded_at(&self) -> &str {
        &self.recorded_at
    }

    /// Returns the number of stored graph nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of stored graph edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns a stored node by canonical graph node id.
    pub fn node(&self, node_id: &str) -> Option<&GraphNode> {
        self.nodes.get(node_id)
    }

    /// Returns a stored edge by canonical graph edge id.
    pub fn edge(&self, edge_id: &str) -> Option<&GraphEdge> {
        self.edges.get(edge_id)
    }

    /// Returns the alpha query-family descriptor table.
    pub fn query_family_descriptors() -> Vec<GraphQueryFamilyDescriptor> {
        GraphQueryFamilyDescriptor::all()
    }

    /// Executes one alpha graph query request against the stored snapshot.
    pub fn query(&self, request: GraphQueryRequest) -> GraphQueryEnvelope {
        if request.workspace_id != self.workspace_id {
            return self.unavailable_envelope(&request);
        }

        let rows = match request.query_class {
            GraphAlphaQueryClass::SymbolLookup => self.symbol_lookup_rows(&request),
            GraphAlphaQueryClass::ImportNeighborhood => self.edge_neighborhood_rows(
                &request,
                EdgeClass::ImportsModule,
                QueryFamilyTag::SemanticCodeSearch,
            ),
            GraphAlphaQueryClass::OwnershipLookup => self.ownership_lookup_rows(&request),
            GraphAlphaQueryClass::ImpactSeed => self.edge_neighborhood_rows(
                &request,
                EdgeClass::Impacts,
                QueryFamilyTag::ImpactExplorer,
            ),
            GraphAlphaQueryClass::ExplainerCitationSeed => self.explainer_citation_rows(&request),
        };

        GraphQueryEnvelope::from_rows(
            self.envelope_id(&request),
            &request,
            self.workspace_graph_id.clone(),
            self.recorded_at.clone(),
            rows,
        )
    }

    fn unavailable_envelope(&self, request: &GraphQueryRequest) -> GraphQueryEnvelope {
        GraphQueryEnvelope::unavailable(
            self.envelope_id(request),
            request,
            self.workspace_graph_id.clone(),
            self.recorded_at.clone(),
        )
    }

    fn symbol_lookup_rows(&self, request: &GraphQueryRequest) -> Vec<GraphQueryRow> {
        let label_token = request
            .label_token
            .as_deref()
            .map(normalize_label)
            .unwrap_or_default();
        let mut scored = self
            .nodes
            .values()
            .filter(|node| node.node_class == NodeClass::SymbolNode)
            .filter(|node| node.query_family_tags.contains(&QueryFamilyTag::SymbolJump))
            .filter(|node| self.node_in_scope(node, request))
            .filter_map(|node| symbol_match_score(node, &label_token).map(|score| (score, node)))
            .collect::<Vec<_>>();
        scored.sort_by(|(left_score, left), (right_score, right)| {
            left_score
                .cmp(right_score)
                .then_with(|| left.node_id.cmp(&right.node_id))
        });

        scored
            .into_iter()
            .enumerate()
            .map(|(row_index, (_score, node))| self.node_row(row_index, node))
            .collect()
    }

    fn edge_neighborhood_rows(
        &self,
        request: &GraphQueryRequest,
        edge_class: EdgeClass,
        family_tag: QueryFamilyTag,
    ) -> Vec<GraphQueryRow> {
        let mut row_sources = Vec::new();
        let mut node_ids = BTreeSet::new();
        for edge in self.edges.values() {
            if edge.edge_class != edge_class
                || !edge.query_family_tags.contains(&family_tag)
                || !self.edge_in_scope(edge, request)
                || !edge_matches_subject(edge, request.subject_node_id.as_deref())
            {
                continue;
            }
            row_sources.push(RowSource::Edge(edge.edge_id.clone()));
            node_ids.insert(edge.from_node_id.clone());
            node_ids.insert(edge.to_node_id.clone());
        }
        for node_id in node_ids {
            row_sources.push(RowSource::Node(node_id));
        }
        self.rows_from_sources(row_sources)
    }

    fn ownership_lookup_rows(&self, request: &GraphQueryRequest) -> Vec<GraphQueryRow> {
        let mut row_sources = Vec::new();
        let mut owner_node_ids = BTreeSet::new();
        for edge in self.edges.values() {
            if edge.edge_class != EdgeClass::OwnedBy
                || !edge
                    .query_family_tags
                    .contains(&QueryFamilyTag::OwnershipLookup)
                || !self.edge_in_scope(edge, request)
                || !edge_matches_subject(edge, request.subject_node_id.as_deref())
            {
                continue;
            }
            row_sources.push(RowSource::Edge(edge.edge_id.clone()));
            owner_node_ids.insert(edge.to_node_id.clone());
        }
        for node_id in owner_node_ids {
            row_sources.push(RowSource::Node(node_id));
        }
        self.rows_from_sources(row_sources)
    }

    fn explainer_citation_rows(&self, request: &GraphQueryRequest) -> Vec<GraphQueryRow> {
        let mut row_sources = Vec::new();
        let mut cited_node_ids = BTreeSet::new();
        for edge in self.edges.values() {
            if !matches!(edge.edge_class, EdgeClass::Cites | EdgeClass::Explains)
                || !edge
                    .query_family_tags
                    .contains(&QueryFamilyTag::CitedExplainerWalk)
                || !self.edge_in_scope(edge, request)
                || !edge_matches_subject(edge, request.subject_node_id.as_deref())
            {
                continue;
            }
            row_sources.push(RowSource::Edge(edge.edge_id.clone()));
            cited_node_ids.insert(edge.to_node_id.clone());
        }
        for node_id in cited_node_ids {
            row_sources.push(RowSource::Node(node_id));
        }
        self.rows_from_sources(row_sources)
    }

    fn rows_from_sources(&self, row_sources: Vec<RowSource>) -> Vec<GraphQueryRow> {
        row_sources
            .into_iter()
            .filter_map(|source| match source {
                RowSource::Node(node_id) => self.nodes.get(&node_id).map(RowTarget::Node),
                RowSource::Edge(edge_id) => self.edges.get(&edge_id).map(RowTarget::Edge),
            })
            .enumerate()
            .map(|(row_index, target)| match target {
                RowTarget::Node(node) => self.node_row(row_index, node),
                RowTarget::Edge(edge) => GraphQueryRow::from_edge(row_index, edge),
            })
            .collect()
    }

    fn node_row(&self, row_index: usize, node: &GraphNode) -> GraphQueryRow {
        let mut row = GraphQueryRow::from_node(row_index, node);
        if let NodeBody::Symbol {
            declared_in_file_node_id,
            ..
        } = &node.node_body
        {
            row.relative_path = self
                .nodes
                .get(declared_in_file_node_id)
                .and_then(relative_path_for_file_like_node)
                .or_else(|| Some(declared_in_file_node_id.clone()));
        }
        row
    }

    fn node_in_scope(&self, node: &GraphNode, request: &GraphQueryRequest) -> bool {
        request.scope_ids.is_empty()
            || node
                .scope_refs
                .iter()
                .any(|scope| request.scope_ids.contains(&scope.scope_id))
    }

    fn edge_in_scope(&self, edge: &GraphEdge, request: &GraphQueryRequest) -> bool {
        request.scope_ids.is_empty()
            || edge
                .scope_refs
                .iter()
                .any(|scope| request.scope_ids.contains(&scope.scope_id))
    }

    fn envelope_id(&self, request: &GraphQueryRequest) -> String {
        format!(
            "env:{}:{}",
            request.query_class.as_str(),
            request.query_request_id
        )
    }
}

#[derive(Debug)]
enum RowSource {
    Node(String),
    Edge(String),
}

#[derive(Debug)]
enum RowTarget<'a> {
    Node(&'a GraphNode),
    Edge(&'a GraphEdge),
}

fn symbol_match_score(node: &GraphNode, normalized_label_token: &str) -> Option<u8> {
    let labels = symbol_labels(node);
    if labels
        .iter()
        .any(|label| normalize_label(label) == normalized_label_token)
    {
        return Some(0);
    }
    if labels.iter().any(|label| {
        normalize_label(label)
            .rsplit("::")
            .next()
            .is_some_and(|last| last == normalized_label_token)
    }) {
        return Some(1);
    }
    if labels
        .iter()
        .any(|label| normalize_label(label).contains(normalized_label_token))
    {
        return Some(2);
    }
    None
}

fn symbol_labels(node: &GraphNode) -> Vec<&str> {
    let mut labels = Vec::new();
    if let Some(label) = &node.display_label {
        labels.push(label.as_str());
    }
    if let NodeBody::Symbol { qualified_path, .. } = &node.node_body {
        labels.push(qualified_path.as_str());
    }
    labels
}

fn edge_matches_subject(edge: &GraphEdge, subject_node_id: Option<&str>) -> bool {
    match subject_node_id {
        Some(subject) => edge.from_node_id == subject || edge.to_node_id == subject,
        None => true,
    }
}

fn relative_path_for_file_like_node(node: &GraphNode) -> Option<String> {
    match &node.node_body {
        NodeBody::File {
            filesystem_identity,
            ..
        }
        | NodeBody::Directory {
            filesystem_identity,
            ..
        } => Some(trim_workspace_prefix(
            &filesystem_identity.presentation_path,
        )),
        NodeBody::Doc {
            anchor_filesystem_identity,
            ..
        } => anchor_filesystem_identity
            .as_ref()
            .map(|identity| trim_workspace_prefix(&identity.presentation_path)),
        _ => None,
    }
}

fn normalize_label(label: &str) -> String {
    label.trim().to_ascii_lowercase()
}

fn trim_workspace_prefix(path: &str) -> String {
    path.strip_prefix("/workspace/aureline/")
        .or_else(|| path.strip_prefix("/workspace/"))
        .unwrap_or(path)
        .to_string()
}
