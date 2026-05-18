use aureline_graph::{GraphQueryRequest, GraphStore, WorksetScopeDescriptor};
use aureline_graph_proto::{
    all_scenarios, EdgeClass, Visibility, WorksetScopeClass, WorksetScopeRef, WorkspaceGraph,
};
use aureline_graph_ui::{
    CodebaseExplainerSurface, GraphDisclosureState, ImpactConfidenceFamily, ImpactExplorerSurface,
    ImpactReasonClass, ScopeVocabularyClass, TopologySurface,
};

#[test]
fn topology_surface_preserves_canvas_list_table_parity_and_scope_truth() {
    let (store, packet) = impact_packet_with_out_of_scope_edge();
    let topology = TopologySurface::from_impact_packet(&packet);

    topology
        .validate_table_parity()
        .expect("canvas nodes and edges must have table alternates");
    assert_eq!(topology.scope_class, ScopeVocabularyClass::CurrentRepo);
    assert!(topology
        .freshness_provenance_strip
        .disclosures
        .contains(&GraphDisclosureState::OutsideCurrentScope));
    assert!(topology
        .relation_legend
        .iter()
        .any(|entry| entry.relation_class.as_str() == "heuristic_summary_relation"));
    assert!(!topology.open_evidence_actions.is_empty());
    assert!(!topology.export_actions.is_empty());

    let json = serde_json::to_string(&topology).expect("topology surface serializes");
    let decoded: TopologySurface =
        serde_json::from_str(&json).expect("topology surface parses back");
    assert_eq!(decoded.canvas_edges.len(), topology.edge_table_rows.len());
    assert!(store.edge(&decoded.canvas_edges[0].edge_id).is_some());
}

#[test]
fn impact_explorer_rows_use_controlled_reason_and_scope_vocabularies() {
    let (store, packet) = impact_packet_with_out_of_scope_edge();
    let impact = ImpactExplorerSurface::from_packet(&store, &packet);

    let required_reason_tokens = ImpactReasonClass::all()
        .iter()
        .map(|reason| reason.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        required_reason_tokens,
        vec![
            "exact_edge",
            "shared_target",
            "ownership_rule",
            "generated_linkage",
            "heuristic_similarity",
            "policy_coupling",
        ]
    );
    let scope_labels = ScopeVocabularyClass::all()
        .iter()
        .map(|scope| scope.label())
        .collect::<Vec<_>>();
    assert_eq!(
        scope_labels,
        vec![
            "Current repo",
            "Selected workset",
            "Full workspace",
            "Remote cache",
            "Outside current scope",
        ]
    );

    let row = impact.rows.first().expect("impact row is visible");
    assert_eq!(row.reason_class, ImpactReasonClass::HeuristicSimilarity);
    assert_eq!(row.reason_family, ImpactConfidenceFamily::Heuristic);
    assert_eq!(
        row.loaded_scope_note.scope_class,
        ScopeVocabularyClass::CurrentRepo
    );
    assert!(row.reason_note.contains("Transitive"));
    assert!(impact.hidden_out_of_scope_count > 0);
    assert!(impact
        .batch_actions
        .iter()
        .any(|action| action.as_str() == "request_widen_scope"));
}

#[test]
fn cited_codebase_explainer_preserves_source_labels_citations_and_omissions() {
    let (_store, packet) = impact_packet_with_out_of_scope_edge();
    let explainer = CodebaseExplainerSurface::from_impact_packet(&packet);

    explainer
        .validate_cited_claims()
        .expect("non-trivial explainer claims must cite evidence");
    assert_eq!(explainer.generated_vs_curated_label.as_str(), "generated");
    assert!(explainer
        .scope_disclosures
        .contains(&GraphDisclosureState::OutsideCurrentScope));
    assert!(explainer
        .claims
        .iter()
        .all(|claim| !claim.citations.is_empty()));
    assert!(explainer.export_packets.iter().any(|packet| {
        packet.export_format == "json_explainer_snapshot"
            && packet.preserves_citations
            && packet.preserves_omissions
            && packet.preserves_source_labels
    }));

    let json = serde_json::to_string(&explainer).expect("explainer surface serializes");
    let decoded: CodebaseExplainerSurface =
        serde_json::from_str(&json).expect("explainer surface parses back");
    assert_eq!(
        decoded.claims[0].citations.len(),
        explainer.claims[0].citations.len()
    );
}

fn impact_packet_with_out_of_scope_edge() -> (GraphStore, aureline_graph::ImpactExplainerPacket) {
    let graph = provider_graph_with_out_of_scope_impact();
    let store = GraphStore::persist_snapshot(graph).expect("graph fixture must validate");
    let request = GraphQueryRequest::impact_seed(
        "q:impact_explainer:ui_surface",
        "ws:aureline",
        "node:provider:issue_42",
    )
    .with_scope_ids(["scope:root:0"]);
    let scope = WorksetScopeDescriptor::local_sparse(
        "scope:root:0",
        "current_root",
        ["repo:aureline"],
        1,
        store.node_count(),
        store.edge_count() - 1,
        1,
    );
    let packet = store
        .build_impact_explainer_packet(request, scope)
        .expect("impact request builds an explainer packet");
    (store, packet)
}

fn provider_graph_with_out_of_scope_impact() -> WorkspaceGraph {
    let mut graph = all_scenarios()
        .into_iter()
        .find(|scenario| scenario.label == "provider_resources_and_citations")
        .expect("provider scenario exists")
        .graph;
    let outside_scope = WorksetScopeRef {
        scope_class: WorksetScopeClass::SparseSlice,
        scope_id: "scope:sparse:outside_impact".to_owned(),
        visibility: Visibility::PartialVisible,
    };
    let mut outside_edge = graph
        .edges
        .iter()
        .find(|edge| edge.edge_id == "edge:impacts:issue_to_server_file")
        .expect("provider scenario carries an impact edge")
        .clone();
    outside_edge.edge_id = "edge:impacts:issue_to_adr_outside_scope".to_owned();
    outside_edge.to_node_id = "node:doc:adr_0007".to_owned();
    outside_edge.scope_refs = vec![outside_scope.clone()];
    assert_eq!(outside_edge.edge_class, EdgeClass::Impacts);
    graph.scope_refs.push(outside_scope);
    graph.edges.push(outside_edge);
    graph
}
