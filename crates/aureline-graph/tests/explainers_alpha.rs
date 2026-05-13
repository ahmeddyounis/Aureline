use std::path::Path;

use aureline_graph::{
    ExplainerSourceKind, GraphQueryRequest, GraphStore, ImpactEdgeEvidenceClass,
    WorksetScopeDescriptor,
};
use aureline_graph_proto::{all_scenarios, EdgeClass, WorksetScopeClass, WorksetScopeRef};
use aureline_graph_proto::{Visibility, WorkspaceGraph};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    request: FixtureRequest,
    expected_packet: ExpectedPacket,
}

#[derive(Debug, Deserialize)]
struct FixtureRequest {
    query_request_id: String,
    subject_node_id: String,
    scope_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedPacket {
    record_kind: String,
    readiness: String,
    evidence_card_source: String,
    visual_presentation_mode: String,
    fallback_presentation_mode: String,
    visible_impact_edge_count: usize,
    out_of_scope_count: usize,
    exact_edge_count: usize,
    heuristic_edge_count: usize,
    cited_file_count: usize,
    required_actions: Vec<String>,
}

#[test]
fn impact_explainer_packet_preserves_evidence_scope_and_fallback_identity() {
    let fixture = load_fixture();
    let graph = provider_graph_with_out_of_scope_impact();
    let store = GraphStore::persist_snapshot(graph).expect("graph fixture must validate");
    let request = GraphQueryRequest::impact_seed(
        fixture.request.query_request_id,
        "ws:aureline",
        fixture.request.subject_node_id,
    )
    .with_scope_ids(fixture.request.scope_ids);
    let packet = store
        .build_impact_explainer_packet(request, sparse_root_scope(&store))
        .expect("impact request builds an explainer packet");

    assert_eq!(fixture.record_kind, "graph_impact_explainer_alpha_case");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(packet.record_kind, fixture.expected_packet.record_kind);
    assert_eq!(packet.readiness, fixture.expected_packet.readiness);
    assert_eq!(
        packet.evidence_card.generated_vs_curated_source.as_str(),
        fixture.expected_packet.evidence_card_source
    );
    assert_eq!(
        packet.visual_projection.presentation_mode,
        fixture.expected_packet.visual_presentation_mode
    );
    assert_eq!(
        packet.non_canvas_fallback.presentation_mode,
        fixture.expected_packet.fallback_presentation_mode
    );
    assert_eq!(
        packet.impact_summary.visible_impact_edge_count,
        fixture.expected_packet.visible_impact_edge_count
    );
    assert_eq!(
        packet.impact_summary.out_of_scope_count,
        fixture.expected_packet.out_of_scope_count
    );
    assert_eq!(
        packet.impact_summary.exact_edge_count,
        fixture.expected_packet.exact_edge_count
    );
    assert_eq!(
        packet.impact_summary.heuristic_edge_count,
        fixture.expected_packet.heuristic_edge_count
    );
    assert_eq!(
        packet.evidence_card.cited_file_count,
        fixture.expected_packet.cited_file_count
    );

    let visual_edge = packet
        .visual_projection
        .edges
        .first()
        .expect("visual projection carries an impact edge");
    let fallback_row = packet
        .non_canvas_fallback
        .rows
        .first()
        .expect("fallback table carries the same impact edge");
    assert_eq!(visual_edge.edge_id, fallback_row.edge_id);
    assert_eq!(visual_edge.from_node_id, fallback_row.from_node_id);
    assert_eq!(visual_edge.to_node_id, fallback_row.to_node_id);
    assert_eq!(visual_edge.freshness, fallback_row.freshness);
    assert_eq!(visual_edge.confidence, fallback_row.confidence);
    assert_eq!(
        visual_edge.edge_evidence_class,
        ImpactEdgeEvidenceClass::Heuristic
    );

    assert_eq!(packet.workset_scope.scope_id, "scope:root:0");
    assert_eq!(packet.workset_scope.scope_mode.as_str(), "sparse");
    assert_eq!(packet.workset_scope.hidden_result_count, 1);
    assert!(packet
        .coverage_claim
        .contains("full transitive repo impact is not claimed"));

    let action_tokens = packet
        .evidence_card
        .open_detail_actions
        .iter()
        .map(|action| action.action_class.as_str().to_owned())
        .collect::<Vec<_>>();
    for required in fixture.expected_packet.required_actions {
        assert!(
            action_tokens.contains(&required),
            "missing action {required}; got {action_tokens:?}"
        );
    }
    assert!(packet
        .evidence_card
        .citations
        .iter()
        .any(|citation| citation.source_kind == ExplainerSourceKind::Generated));
}

#[test]
fn impact_explainer_discloses_out_of_scope_when_visible_result_is_empty() {
    let graph = provider_graph_with_out_of_scope_impact();
    let store = GraphStore::persist_snapshot(graph).expect("graph fixture must validate");
    let request = GraphQueryRequest::impact_seed(
        "q:impact_explainer:outside_only",
        "ws:aureline",
        "node:provider:issue_42",
    )
    .with_scope_ids(["scope:missing_visible_slice"]);
    let packet = store
        .build_impact_explainer_packet(request, missing_visible_slice_scope(&store))
        .expect("impact request builds an explainer packet");

    assert!(packet.impact_summary.no_impact_found);
    assert_eq!(packet.impact_summary.visible_impact_edge_count, 0);
    assert_eq!(packet.impact_summary.out_of_scope_count, 2);
    assert_eq!(
        packet.impact_summary.no_impact_reason.as_deref(),
        Some("impact_edges_exist_outside_active_scope")
    );
}

#[test]
fn journey_budget_ledger_maps_explainer_to_navigation_budgets() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/perf/journey_budget_ledger_alpha.yaml");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let value: serde_yaml::Value =
        serde_yaml::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));

    assert_eq!(value["schema_version"].as_i64(), Some(1));
    assert_eq!(
        value["ledger_id"].as_str(),
        Some("aureline.journey_budget_ledger_alpha")
    );
    let rows = value["budget_rows"]
        .as_sequence()
        .expect("budget_rows is a sequence");
    let explainer_row = rows
        .iter()
        .find(|row| row["lane"].as_str() == Some("graph_impact_explainer"))
        .expect("graph impact explainer budget row exists");
    let budget_refs = explainer_row["protected_budget_refs"]
        .as_sequence()
        .expect("budget refs are listed")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(budget_refs.contains(&"budget.path.command_palette.open"));
    assert!(budget_refs.contains(&"budget.path.editor.placeholder_open"));
    assert_eq!(
        explainer_row["packet_ref"].as_str(),
        Some("fixtures/graph/topology_explainer_alpha/impact_explainer_packet_alpha.json")
    );
}

fn sparse_root_scope(store: &GraphStore) -> WorksetScopeDescriptor {
    WorksetScopeDescriptor::local_sparse(
        "scope:root:0",
        "current_root",
        ["repo:aureline"],
        1,
        store.node_count(),
        store.edge_count() - 1,
        1,
    )
}

fn missing_visible_slice_scope(store: &GraphStore) -> WorksetScopeDescriptor {
    WorksetScopeDescriptor::local_sparse(
        "scope:missing_visible_slice",
        "sparse_slice",
        ["repo:aureline"],
        2,
        store.node_count(),
        store.edge_count() - 2,
        2,
    )
}

fn provider_graph_with_out_of_scope_impact() -> WorkspaceGraph {
    let mut graph = scenario_graph("provider_resources_and_citations");
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

fn scenario_graph(label: &str) -> WorkspaceGraph {
    all_scenarios()
        .into_iter()
        .find(|scenario| scenario.label == label)
        .unwrap_or_else(|| panic!("missing graph scenario {label}"))
        .graph
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/graph/topology_explainer_alpha/impact_explainer_packet_alpha.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
