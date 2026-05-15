//! Alpha graph query-family records.
//!
//! These records are intentionally small and reuse the graph seed vocabulary
//! directly. Search, navigation, support, and future public graph surfaces can
//! consume the same envelope without minting private graph row models.

use std::collections::BTreeMap;

use aureline_graph_proto::{
    ConfidenceLevel, EdgeClass, EdgeEvidenceState, Freshness, FreshnessFrame, GraphEdge, GraphNode,
    NodeBody, NodeClass, QueryFamilyTag, WorksetScopeRef,
};
use serde::{Deserialize, Serialize};

use crate::journey_budget::{BudgetUnit, JourneyId, LedgerRollup};

/// Schema version for the alpha graph query-family runtime records.
pub const GRAPH_QUERY_FAMILY_ALPHA_VERSION: u32 = 1;

/// Row-level result partiality vocabulary shared by graph and search results.
///
/// The class travels with query outputs so a warming, partial, stale, or
/// unavailable result keeps its caveat after sorting, pagination, deduping, or
/// projection through search and navigation surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultPartialityClass {
    /// Result came from a ready provider; no caveat is required.
    Authoritative,
    /// Result came from a provider that is still warming up.
    Warming,
    /// Result came from a provider with rows but incomplete coverage.
    Partial,
    /// Result came from a cached or stale snapshot.
    Stale,
    /// Result came from a provider that cannot currently answer.
    Unavailable,
}

impl ResultPartialityClass {
    /// Stable token used in records, fixtures, and snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
        }
    }

    /// Short label suitable for a row chip or badge.
    pub const fn row_badge(self) -> &'static str {
        match self {
            Self::Authoritative => "Authoritative",
            Self::Warming => "Warming",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Returns true when the result carries a visible caveat.
    pub const fn is_partial(self) -> bool {
        !matches!(self, Self::Authoritative)
    }
}

/// Stable alpha query classes supported by the runtime graph store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphAlphaQueryClass {
    /// Resolves a symbol label or qualified path to graph symbol nodes.
    SymbolLookup,
    /// Walks import edges around a subject file or symbol.
    ImportNeighborhood,
    /// Resolves ownership edges and owner nodes for a subject node.
    OwnershipLookup,
    /// Returns impact edges when present without claiming full impact depth.
    ImpactSeed,
    /// Returns explainer citation edges when present without producing prose.
    ExplainerCitationSeed,
}

impl GraphAlphaQueryClass {
    /// Returns the stable schema token for this query class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymbolLookup => "symbol_lookup",
            Self::ImportNeighborhood => "import_neighborhood",
            Self::OwnershipLookup => "ownership_lookup",
            Self::ImpactSeed => "impact_seed",
            Self::ExplainerCitationSeed => "explainer_citation_seed",
        }
    }

    /// Returns the published query-family tag this alpha query serves.
    pub const fn query_family_tag(self) -> QueryFamilyTag {
        match self {
            Self::SymbolLookup => QueryFamilyTag::SymbolJump,
            Self::ImportNeighborhood => QueryFamilyTag::SemanticCodeSearch,
            Self::OwnershipLookup => QueryFamilyTag::OwnershipLookup,
            Self::ImpactSeed => QueryFamilyTag::ImpactExplorer,
            Self::ExplainerCitationSeed => QueryFamilyTag::CitedExplainerWalk,
        }
    }
}

/// Readiness state for an alpha graph query envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphQueryReadiness {
    /// The graph rows are current for the declared scope.
    Ready,
    /// The query returned useful hot-set rows before full graph warm-up.
    HotSetReady,
    /// The query returned rows but declared-scope coverage is incomplete.
    Partial,
    /// The graph lane is warming and rows may be incomplete.
    Warming,
    /// The query returned stale or cached rows with disclosure.
    Stale,
    /// The graph lane cannot currently answer.
    Unavailable,
    /// The requested subject is outside the active scope.
    OutOfScope,
}

impl GraphQueryReadiness {
    /// Returns the stable schema token for this readiness state.
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
}

/// Partial-truth cause attached to a query envelope or result row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphPartialTruthCause {
    /// A warming graph row contributed to the result.
    Warming,
    /// A stale graph row contributed to the result.
    Stale,
    /// An imported graph row contributed to the result.
    Imported,
    /// A replayed graph row contributed to the result.
    Replayed,
    /// A low or unknown confidence row contributed to the result.
    LowConfidence,
    /// At least one result row is only partially visible in the active scope.
    PartialScope,
    /// A missing-anchor node or edge contributed to the result.
    MissingAnchor,
    /// A policy-view row contributed to the result.
    PolicyHidden,
    /// A derived or inferred row contributed to the result.
    Derived,
}

/// Typed downgrade reason attached to a graph query envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphQueryDowngradeReason {
    /// The metadata-only journey budget rejected additional query rows.
    JourneyBudgetOverrun,
}

impl GraphQueryDowngradeReason {
    /// Returns the stable schema token for this downgrade reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JourneyBudgetOverrun => "journey_budget_overrun",
        }
    }
}

impl GraphPartialTruthCause {
    /// Returns the stable schema token for this partial-truth cause.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Warming => "warming",
            Self::Stale => "stale",
            Self::Imported => "imported",
            Self::Replayed => "replayed",
            Self::LowConfidence => "low_confidence",
            Self::PartialScope => "partial_scope",
            Self::MissingAnchor => "missing_anchor",
            Self::PolicyHidden => "policy_hidden",
            Self::Derived => "derived",
        }
    }
}

/// Row class emitted by the alpha graph query runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphQueryRowClass {
    /// A row referencing a graph node.
    Node,
    /// A row referencing a graph edge.
    Edge,
    /// A row that explicitly represents an unresolved reference.
    MissingAnchor,
    /// A row that represents a policy-limited projection.
    PolicyHidden,
}

impl GraphQueryRowClass {
    /// Returns the stable schema token for this row class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Edge => "edge",
            Self::MissingAnchor => "missing_anchor",
            Self::PolicyHidden => "policy_hidden",
        }
    }
}

/// Descriptor for one alpha graph query class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphQueryFamilyDescriptor {
    /// Integer schema version for this descriptor.
    pub schema_version: u32,
    /// Stable alpha query class.
    pub query_class: GraphAlphaQueryClass,
    /// Published graph query-family tag served by this alpha query.
    pub query_family_tag: QueryFamilyTag,
    /// Node classes this query may return.
    pub allowed_node_classes: Vec<NodeClass>,
    /// Edge classes this query may return.
    pub allowed_edge_classes: Vec<EdgeClass>,
    /// Minimum freshness behavior expected by consumers.
    pub expected_freshness_behavior: &'static str,
    /// Confidence behavior expected by consumers.
    pub expected_confidence_behavior: &'static str,
}

impl GraphQueryFamilyDescriptor {
    /// Returns the descriptor table for every alpha query class.
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
                query_class: GraphAlphaQueryClass::SymbolLookup,
                query_family_tag: QueryFamilyTag::SymbolJump,
                allowed_node_classes: vec![NodeClass::SymbolNode],
                allowed_edge_classes: vec![EdgeClass::DefinesSymbol, EdgeClass::ReferencesSymbol],
                expected_freshness_behavior:
                    "prefer_authoritative; disclose warming, stale, imported, or replayed rows",
                expected_confidence_behavior:
                    "high and medium may navigate; low and unknown must render a confidence cue",
            },
            Self {
                schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
                query_class: GraphAlphaQueryClass::ImportNeighborhood,
                query_family_tag: QueryFamilyTag::SemanticCodeSearch,
                allowed_node_classes: vec![
                    NodeClass::FileNode,
                    NodeClass::DirectoryNode,
                    NodeClass::SymbolNode,
                    NodeClass::MissingAnchorNode,
                ],
                allowed_edge_classes: vec![EdgeClass::ImportsModule, EdgeClass::MissingAnchorFor],
                expected_freshness_behavior:
                    "authoritative when current; partial and stale imports must stay disclosed",
                expected_confidence_behavior:
                    "direct parser evidence is high; inferred import evidence must render derived",
            },
            Self {
                schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
                query_class: GraphAlphaQueryClass::OwnershipLookup,
                query_family_tag: QueryFamilyTag::OwnershipLookup,
                allowed_node_classes: vec![
                    NodeClass::FileNode,
                    NodeClass::DirectoryNode,
                    NodeClass::SymbolNode,
                    NodeClass::OwnershipNode,
                    NodeClass::MissingAnchorNode,
                ],
                allowed_edge_classes: vec![EdgeClass::OwnedBy, EdgeClass::MissingAnchorFor],
                expected_freshness_behavior:
                    "codeowner-derived authority is current only for the declared graph epoch",
                expected_confidence_behavior:
                    "ownership rows may be medium but must keep source anchors visible",
            },
            Self {
                schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
                query_class: GraphAlphaQueryClass::ImpactSeed,
                query_family_tag: QueryFamilyTag::ImpactExplorer,
                allowed_node_classes: vec![NodeClass::FileNode, NodeClass::SymbolNode],
                allowed_edge_classes: vec![EdgeClass::Impacts],
                expected_freshness_behavior:
                    "alpha returns only stored impact edges and never claims full transitive depth",
                expected_confidence_behavior:
                    "inferred impact rows must stay derived or heuristic in downstream surfaces",
            },
            Self {
                schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
                query_class: GraphAlphaQueryClass::ExplainerCitationSeed,
                query_family_tag: QueryFamilyTag::CitedExplainerWalk,
                allowed_node_classes: vec![NodeClass::DocNode, NodeClass::SymbolNode],
                allowed_edge_classes: vec![EdgeClass::Cites, EdgeClass::Explains],
                expected_freshness_behavior:
                    "alpha returns citation anchors only; stale citations require disclosure",
                expected_confidence_behavior:
                    "explainers may cite medium confidence evidence but must not upgrade it",
            },
        ]
    }

    /// Returns the descriptor for a single alpha query class.
    pub fn for_query_class(query_class: GraphAlphaQueryClass) -> Self {
        Self::all()
            .into_iter()
            .find(|descriptor| descriptor.query_class == query_class)
            .expect("descriptor table must cover every alpha graph query class")
    }
}

/// Request consumed by [`crate::GraphStore`] query methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphQueryRequest {
    /// Stable request id copied onto the response envelope.
    pub query_request_id: String,
    /// Alpha query class to execute.
    pub query_class: GraphAlphaQueryClass,
    /// Workspace id the caller expects to query.
    pub workspace_id: String,
    /// Optional normalized label token for symbol or label lookup.
    pub label_token: Option<String>,
    /// Optional subject node id for neighborhood queries.
    pub subject_node_id: Option<String>,
    /// Optional scope ids constraining the query.
    pub scope_ids: Vec<String>,
    /// Optional metadata-only budget limits for this query journey.
    pub journey_budget_limits: BTreeMap<BudgetUnit, u64>,
}

impl GraphQueryRequest {
    /// Builds a symbol lookup request for a normalized label token.
    pub fn symbol_lookup(
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        label_token: impl Into<String>,
    ) -> Self {
        Self {
            query_request_id: query_request_id.into(),
            query_class: GraphAlphaQueryClass::SymbolLookup,
            workspace_id: workspace_id.into(),
            label_token: Some(label_token.into()),
            subject_node_id: None,
            scope_ids: Vec::new(),
            journey_budget_limits: BTreeMap::new(),
        }
    }

    /// Builds an import-neighborhood request for a graph node id.
    pub fn import_neighborhood(
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        subject_node_id: impl Into<String>,
    ) -> Self {
        Self::for_subject(
            query_request_id,
            workspace_id,
            GraphAlphaQueryClass::ImportNeighborhood,
            subject_node_id,
        )
    }

    /// Builds an ownership lookup request for a graph node id.
    pub fn ownership_lookup(
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        subject_node_id: impl Into<String>,
    ) -> Self {
        Self::for_subject(
            query_request_id,
            workspace_id,
            GraphAlphaQueryClass::OwnershipLookup,
            subject_node_id,
        )
    }

    /// Builds an impact seed request for a graph node id.
    pub fn impact_seed(
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        subject_node_id: impl Into<String>,
    ) -> Self {
        Self::for_subject(
            query_request_id,
            workspace_id,
            GraphAlphaQueryClass::ImpactSeed,
            subject_node_id,
        )
    }

    /// Builds an explainer-citation seed request for a graph node id.
    pub fn explainer_citation_seed(
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        subject_node_id: impl Into<String>,
    ) -> Self {
        Self::for_subject(
            query_request_id,
            workspace_id,
            GraphAlphaQueryClass::ExplainerCitationSeed,
            subject_node_id,
        )
    }

    /// Adds scope ids that bound this request.
    pub fn with_scope_ids(
        mut self,
        scope_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.scope_ids = scope_ids.into_iter().map(Into::into).collect();
        self
    }

    /// Adds or replaces one metadata-only journey-budget limit.
    pub fn with_journey_budget_limit(mut self, unit: BudgetUnit, limit: u64) -> Self {
        self.journey_budget_limits.insert(unit, limit);
        self
    }

    /// Returns the deterministic journey id for this query's semantic inputs.
    pub fn journey_id(&self) -> JourneyId {
        query_journey_id(self)
    }

    fn for_subject(
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        query_class: GraphAlphaQueryClass,
        subject_node_id: impl Into<String>,
    ) -> Self {
        Self {
            query_request_id: query_request_id.into(),
            query_class,
            workspace_id: workspace_id.into(),
            label_token: None,
            subject_node_id: Some(subject_node_id.into()),
            scope_ids: Vec::new(),
            journey_budget_limits: BTreeMap::new(),
        }
    }
}

/// One row emitted by an alpha graph query envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphQueryRow {
    /// Stable zero-based index inside the envelope.
    pub row_index: usize,
    /// Row class used by consumers to pick rendering posture.
    pub row_class: GraphQueryRowClass,
    /// Referenced graph node id, when this row points at a node.
    pub node_id: Option<String>,
    /// Referenced graph edge id, when this row points at an edge.
    pub edge_id: Option<String>,
    /// Node class copied from the canonical graph node.
    pub node_class: Option<NodeClass>,
    /// Edge class copied from the canonical graph edge.
    pub edge_class: Option<EdgeClass>,
    /// Human-readable display label safe for row titles.
    pub display_label: String,
    /// Workspace-relative path when a row can navigate to a file.
    pub relative_path: Option<String>,
    /// Stable symbol ref when a row can navigate to a symbol.
    pub symbol_ref: Option<String>,
    /// Freshness frame copied from the canonical node or edge.
    pub freshness_frame: FreshnessFrame,
    /// Confidence level copied from the canonical node or edge.
    pub confidence_level: ConfidenceLevel,
    /// Evidence state copied from canonical graph edges.
    pub evidence_state: Option<EdgeEvidenceState>,
    /// Scope refs copied from the canonical node or edge.
    pub scope_refs: Vec<WorksetScopeRef>,
    /// Partial-truth causes visible to consumers.
    pub partial_truth_causes: Vec<GraphPartialTruthCause>,
}

impl GraphQueryRow {
    /// Builds a query row from a graph node.
    pub fn from_node(row_index: usize, node: &GraphNode) -> Self {
        let row_class = match node.node_class {
            NodeClass::MissingAnchorNode => GraphQueryRowClass::MissingAnchor,
            NodeClass::PolicyViewNode => GraphQueryRowClass::PolicyHidden,
            _ => GraphQueryRowClass::Node,
        };
        let partial_truth_causes = partial_truth_causes_for_node(node);
        Self {
            row_index,
            row_class,
            node_id: Some(node.node_id.clone()),
            edge_id: None,
            node_class: Some(node.node_class),
            edge_class: None,
            display_label: display_label_for_node(node),
            relative_path: relative_path_for_node(node),
            symbol_ref: symbol_ref_for_node(node),
            freshness_frame: node.freshness_frame.clone(),
            confidence_level: node.confidence_level,
            evidence_state: None,
            scope_refs: node.scope_refs.clone(),
            partial_truth_causes,
        }
    }

    /// Builds a query row from a graph edge.
    pub fn from_edge(row_index: usize, edge: &GraphEdge) -> Self {
        let row_class = match edge.evidence.evidence_state {
            EdgeEvidenceState::MissingAnchor => GraphQueryRowClass::MissingAnchor,
            _ => GraphQueryRowClass::Edge,
        };
        let partial_truth_causes = partial_truth_causes_for_edge(edge);
        Self {
            row_index,
            row_class,
            node_id: None,
            edge_id: Some(edge.edge_id.clone()),
            node_class: None,
            edge_class: Some(edge.edge_class),
            display_label: display_label_for_edge(edge),
            relative_path: None,
            symbol_ref: None,
            freshness_frame: edge.evidence.freshness_frame.clone(),
            confidence_level: edge.evidence.confidence_level,
            evidence_state: Some(edge.evidence.evidence_state),
            scope_refs: edge.scope_refs.clone(),
            partial_truth_causes,
        }
    }

    /// Returns the canonical id a consumer should use for dedupe/fusion.
    pub fn canonical_id(&self) -> Option<&str> {
        self.node_id.as_deref().or(self.edge_id.as_deref())
    }
}

/// Versioned response envelope emitted by the alpha graph query runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphQueryEnvelope {
    /// Integer schema version for this envelope.
    pub schema_version: u32,
    /// Stable envelope id for support/export joins.
    pub envelope_id: String,
    /// Request id this envelope answers.
    pub query_request_id: String,
    /// Alpha query class used to produce this envelope.
    pub query_class: GraphAlphaQueryClass,
    /// Published query-family tag served by this envelope.
    pub query_family_tag: QueryFamilyTag,
    /// Workspace graph snapshot id used to answer.
    pub workspace_graph_id: String,
    /// Workspace id used to answer.
    pub workspace_id: String,
    /// Monotonic or fixture timestamp for export parity.
    pub emitted_at: String,
    /// Readiness state of this envelope.
    pub readiness: GraphQueryReadiness,
    /// Row-level partiality token consumers reuse for result chrome and exports.
    pub result_partiality_class: ResultPartialityClass,
    /// Typed reasons this envelope was downgraded after query execution began.
    pub downgrade_reasons: Vec<GraphQueryDowngradeReason>,
    /// Metadata-only budget rollup for this query journey.
    pub journey_budget_rollup: LedgerRollup,
    /// Highest-risk partial-truth causes for this envelope.
    pub partial_truth_causes: Vec<GraphPartialTruthCause>,
    /// Ordered query rows.
    pub rows: Vec<GraphQueryRow>,
}

impl GraphQueryEnvelope {
    /// Builds a response envelope and derives readiness from its rows.
    pub fn from_rows(
        envelope_id: impl Into<String>,
        request: &GraphQueryRequest,
        workspace_graph_id: impl Into<String>,
        emitted_at: impl Into<String>,
        rows: Vec<GraphQueryRow>,
        journey_budget_rollup: LedgerRollup,
    ) -> Self {
        let partial_truth_causes = envelope_partial_truth_causes(&rows);
        let downgraded_for_budget = journey_budget_rollup.exceeded_budget();
        let readiness = if downgraded_for_budget {
            GraphQueryReadiness::Partial
        } else {
            readiness_for_rows(&rows, &partial_truth_causes)
        };
        let result_partiality_class = result_partiality_for_readiness(readiness);
        let downgrade_reasons = if downgraded_for_budget {
            vec![GraphQueryDowngradeReason::JourneyBudgetOverrun]
        } else {
            Vec::new()
        };
        Self {
            schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
            envelope_id: envelope_id.into(),
            query_request_id: request.query_request_id.clone(),
            query_class: request.query_class,
            query_family_tag: request.query_class.query_family_tag(),
            workspace_graph_id: workspace_graph_id.into(),
            workspace_id: request.workspace_id.clone(),
            emitted_at: emitted_at.into(),
            readiness,
            result_partiality_class,
            downgrade_reasons,
            journey_budget_rollup,
            partial_truth_causes,
            rows,
        }
    }

    /// Builds an unavailable envelope for a request the store cannot answer.
    pub fn unavailable(
        envelope_id: impl Into<String>,
        request: &GraphQueryRequest,
        workspace_graph_id: impl Into<String>,
        emitted_at: impl Into<String>,
        journey_budget_rollup: LedgerRollup,
    ) -> Self {
        Self {
            schema_version: GRAPH_QUERY_FAMILY_ALPHA_VERSION,
            envelope_id: envelope_id.into(),
            query_request_id: request.query_request_id.clone(),
            query_class: request.query_class,
            query_family_tag: request.query_class.query_family_tag(),
            workspace_graph_id: workspace_graph_id.into(),
            workspace_id: request.workspace_id.clone(),
            emitted_at: emitted_at.into(),
            readiness: GraphQueryReadiness::Unavailable,
            result_partiality_class: ResultPartialityClass::Unavailable,
            downgrade_reasons: Vec::new(),
            journey_budget_rollup,
            partial_truth_causes: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Returns true when consumers must render a partial-truth disclosure.
    pub fn requires_partial_truth_disclosure(&self) -> bool {
        self.readiness != GraphQueryReadiness::Ready || !self.partial_truth_causes.is_empty()
    }
}

/// Projects graph query readiness onto the shared result-partiality vocabulary.
pub const fn result_partiality_for_readiness(
    readiness: GraphQueryReadiness,
) -> ResultPartialityClass {
    match readiness {
        GraphQueryReadiness::Ready => ResultPartialityClass::Authoritative,
        GraphQueryReadiness::HotSetReady | GraphQueryReadiness::Partial => {
            ResultPartialityClass::Partial
        }
        GraphQueryReadiness::Warming => ResultPartialityClass::Warming,
        GraphQueryReadiness::Stale => ResultPartialityClass::Stale,
        GraphQueryReadiness::Unavailable | GraphQueryReadiness::OutOfScope => {
            ResultPartialityClass::Unavailable
        }
    }
}

fn readiness_for_rows(
    rows: &[GraphQueryRow],
    partial_truth_causes: &[GraphPartialTruthCause],
) -> GraphQueryReadiness {
    if rows.is_empty() {
        return GraphQueryReadiness::Ready;
    }
    if partial_truth_causes.contains(&GraphPartialTruthCause::Warming) {
        return GraphQueryReadiness::Warming;
    }
    if partial_truth_causes.contains(&GraphPartialTruthCause::PartialScope) {
        return GraphQueryReadiness::Partial;
    }
    if partial_truth_causes.contains(&GraphPartialTruthCause::Stale)
        || partial_truth_causes.contains(&GraphPartialTruthCause::Imported)
        || partial_truth_causes.contains(&GraphPartialTruthCause::Replayed)
    {
        return GraphQueryReadiness::Stale;
    }
    GraphQueryReadiness::Ready
}

fn envelope_partial_truth_causes(rows: &[GraphQueryRow]) -> Vec<GraphPartialTruthCause> {
    let mut causes = Vec::new();
    for row in rows {
        for cause in &row.partial_truth_causes {
            if !causes.contains(cause) {
                causes.push(*cause);
            }
        }
    }
    causes.sort();
    causes
}

fn partial_truth_causes_for_node(node: &GraphNode) -> Vec<GraphPartialTruthCause> {
    let mut causes = partial_truth_causes_for_freshness(node.freshness_frame.freshness);
    append_confidence_cause(&mut causes, node.confidence_level);
    match node.node_class {
        NodeClass::MissingAnchorNode => {
            append_cause(&mut causes, GraphPartialTruthCause::MissingAnchor)
        }
        NodeClass::PolicyViewNode => {
            append_cause(&mut causes, GraphPartialTruthCause::PolicyHidden)
        }
        _ => {}
    }
    if node
        .scope_refs
        .iter()
        .any(|scope| scope.visibility.as_str() != "fully_visible")
    {
        append_cause(&mut causes, GraphPartialTruthCause::PartialScope);
    }
    causes
}

fn partial_truth_causes_for_edge(edge: &GraphEdge) -> Vec<GraphPartialTruthCause> {
    let mut causes = partial_truth_causes_for_freshness(edge.evidence.freshness_frame.freshness);
    append_confidence_cause(&mut causes, edge.evidence.confidence_level);
    match edge.evidence.evidence_state {
        EdgeEvidenceState::InferredRelation => {
            append_cause(&mut causes, GraphPartialTruthCause::Derived);
        }
        EdgeEvidenceState::StaleRelation => {
            append_cause(&mut causes, GraphPartialTruthCause::Stale);
        }
        EdgeEvidenceState::MissingAnchor => {
            append_cause(&mut causes, GraphPartialTruthCause::MissingAnchor);
        }
        EdgeEvidenceState::DirectEvidence | EdgeEvidenceState::ImportedEvidence => {}
    }
    if edge
        .scope_refs
        .iter()
        .any(|scope| scope.visibility.as_str() != "fully_visible")
    {
        append_cause(&mut causes, GraphPartialTruthCause::PartialScope);
    }
    causes
}

fn partial_truth_causes_for_freshness(freshness: Freshness) -> Vec<GraphPartialTruthCause> {
    match freshness {
        Freshness::Authoritative => Vec::new(),
        Freshness::Warming => vec![GraphPartialTruthCause::Warming],
        Freshness::Cached | Freshness::Stale => vec![GraphPartialTruthCause::Stale],
        Freshness::Replayed => vec![GraphPartialTruthCause::Replayed],
        Freshness::Imported => vec![GraphPartialTruthCause::Imported],
    }
}

fn append_confidence_cause(causes: &mut Vec<GraphPartialTruthCause>, confidence: ConfidenceLevel) {
    if matches!(confidence, ConfidenceLevel::Low | ConfidenceLevel::Unknown) {
        append_cause(causes, GraphPartialTruthCause::LowConfidence);
    }
}

fn append_cause(causes: &mut Vec<GraphPartialTruthCause>, cause: GraphPartialTruthCause) {
    if !causes.contains(&cause) {
        causes.push(cause);
    }
}

fn query_journey_id(request: &GraphQueryRequest) -> JourneyId {
    let label_token = request
        .label_token
        .as_deref()
        .map(normalize_journey_component)
        .unwrap_or_else(|| "-".to_owned());
    let subject_node_id = request.subject_node_id.as_deref().unwrap_or("-").to_owned();
    let mut scope_ids = request.scope_ids.clone();
    scope_ids.sort();
    scope_ids.dedup();
    let scope_key = if scope_ids.is_empty() {
        "-".to_owned()
    } else {
        scope_ids.join(",")
    };

    JourneyId::new(format!(
        "journey:graph_query:{}:{}:{}:{}:{}",
        stable_component("class", request.query_class.as_str()),
        stable_component("workspace", &request.workspace_id),
        stable_component("label", &label_token),
        stable_component("subject", &subject_node_id),
        stable_component("scopes", &scope_key)
    ))
}

fn normalize_journey_component(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn stable_component(name: &str, value: &str) -> String {
    format!("{name}#{}:{value}", value.len())
}

fn display_label_for_node(node: &GraphNode) -> String {
    if let Some(label) = &node.display_label {
        return label.clone();
    }
    match &node.node_body {
        NodeBody::File {
            filesystem_identity,
            ..
        }
        | NodeBody::Directory {
            filesystem_identity,
            ..
        } => filesystem_identity.presentation_path.clone(),
        NodeBody::Symbol { qualified_path, .. } => qualified_path.clone(),
        NodeBody::Doc { doc_ref, .. } => doc_ref.clone(),
        NodeBody::Ownership {
            display_label,
            ownership_ref,
            ..
        } => display_label
            .clone()
            .unwrap_or_else(|| ownership_ref.clone()),
        NodeBody::Topology { topology_ref, .. } => topology_ref.clone(),
        NodeBody::ProviderResource { provider_ref, .. } => provider_ref.clone(),
        NodeBody::GeneratedArtifact {
            lineage_record_ref, ..
        } => lineage_record_ref.clone(),
        NodeBody::ImportedRoot { import_ref, .. } => import_ref.clone(),
        NodeBody::WorksetScope {
            display_label,
            scope_ref,
        } => display_label
            .clone()
            .unwrap_or_else(|| scope_ref.scope_id.clone()),
        NodeBody::PolicyView { policy_ref, .. } => policy_ref.clone(),
        NodeBody::MissingAnchor { last_known_ref, .. } => last_known_ref
            .clone()
            .unwrap_or_else(|| node.node_id.clone()),
    }
}

fn display_label_for_edge(edge: &GraphEdge) -> String {
    format!(
        "{} {} {}",
        edge.from_node_id,
        edge.edge_class.as_str(),
        edge.to_node_id
    )
}

fn relative_path_for_node(node: &GraphNode) -> Option<String> {
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
        NodeBody::Symbol {
            declared_in_file_node_id,
            ..
        } => Some(declared_in_file_node_id.clone()),
        NodeBody::Doc {
            anchor_filesystem_identity,
            ..
        } => anchor_filesystem_identity
            .as_ref()
            .map(|identity| trim_workspace_prefix(&identity.presentation_path)),
        _ => None,
    }
}

fn symbol_ref_for_node(node: &GraphNode) -> Option<String> {
    match &node.node_body {
        NodeBody::Symbol { qualified_path, .. } => Some(qualified_path.clone()),
        _ => None,
    }
}

fn trim_workspace_prefix(path: &str) -> String {
    path.strip_prefix("/workspace/aureline/")
        .or_else(|| path.strip_prefix("/workspace/"))
        .unwrap_or(path)
        .to_string()
}
