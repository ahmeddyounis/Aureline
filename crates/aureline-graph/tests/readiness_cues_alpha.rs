use std::path::Path;

use aureline_graph::{
    GraphCueSurface, GraphFactCuePacket, GraphFactTruthLane, GraphQueryRequest, GraphStore,
};
use aureline_graph_proto::{
    all_scenarios, Freshness, FreshnessFrame, StaleReason, Visibility, WorkspaceGraph,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    required_truth_lanes: Vec<String>,
    surface_expectations: Vec<SurfaceExpectation>,
    evidence_packet_required_fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SurfaceExpectation {
    surface: String,
    required_truth_lane: String,
    graph_backed: bool,
}

#[test]
fn fact_cue_fixture_covers_acceptance_lanes() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "graph_fact_cue_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let packets = vec![
        exact_local_packet(GraphCueSurface::Navigation),
        partial_packet(GraphCueSurface::AiContext),
        imported_packet(GraphCueSurface::Review),
        inferred_packet(GraphCueSurface::AiContext),
        stale_packet(GraphCueSurface::SupportExport),
        waiting_packet(GraphCueSurface::Navigation),
        fallback_packet(GraphCueSurface::AiContext),
    ];
    let mut lane_tokens = packets
        .iter()
        .flat_map(|packet| {
            packet
                .truth_lanes
                .iter()
                .map(|lane| lane.as_str().to_owned())
        })
        .collect::<Vec<_>>();
    lane_tokens.sort();
    lane_tokens.dedup();

    for required in fixture.required_truth_lanes {
        assert!(
            lane_tokens.contains(&required),
            "missing required truth lane {required}; got {lane_tokens:?}"
        );
    }

    for expectation in fixture.surface_expectations {
        let packet = packets
            .iter()
            .find(|packet| packet.consumer_surface.as_str() == expectation.surface)
            .unwrap_or_else(|| panic!("missing surface {}", expectation.surface));
        assert_eq!(
            packet.has_graph_backed_cues(),
            expectation.graph_backed,
            "graph-backed mismatch for {}",
            expectation.surface
        );
        assert!(
            packet
                .truth_lanes
                .iter()
                .any(|lane| lane.as_str() == expectation.required_truth_lane),
            "surface {} missing lane {} in {:?}",
            expectation.surface,
            expectation.required_truth_lane,
            packet.truth_lanes
        );
    }
}

#[test]
fn graph_surfaces_receive_cues_without_raw_debug_data() {
    let packet = imported_packet(GraphCueSurface::Navigation);

    assert!(packet.requires_visible_cues());
    assert!(packet.has_graph_backed_cues());
    assert_eq!(packet.readiness, "stale");
    assert!(packet
        .truth_lanes
        .contains(&GraphFactTruthLane::ImportedGraphFact));
    assert!(packet.cues.iter().all(|cue| cue.graph_ref.is_some()));
    assert!(packet.cues.iter().all(|cue| cue
        .export_labels
        .contains(&"imported_graph_fact".to_owned())));

    let rendered = serde_json::to_string(&packet).expect("packet serializes");
    for forbidden in ["freshness_frame", "scope_refs", "source_anchors"] {
        assert!(
            !rendered.contains(forbidden),
            "surface cue export leaked raw debug field {forbidden}: {rendered}"
        );
    }
}

#[test]
fn ai_context_and_review_distinguish_graph_from_fallback_search() {
    let ai_graph = exact_local_packet(GraphCueSurface::AiContext);
    let ai_fallback = fallback_packet(GraphCueSurface::AiContext);
    let review_seed = imported_packet(GraphCueSurface::Review);

    assert!(ai_graph.has_graph_backed_cues());
    assert_eq!(
        ai_graph.truth_lanes,
        vec![GraphFactTruthLane::ExactLocalGraphFact]
    );
    assert!(!ai_fallback.has_graph_backed_cues());
    assert_eq!(
        ai_fallback.truth_lanes,
        vec![GraphFactTruthLane::FallbackSearchFact]
    );
    assert_eq!(review_seed.consumer_surface.as_str(), "review");
    assert!(review_seed
        .truth_lanes
        .contains(&GraphFactTruthLane::ImportedGraphFact));
}

#[test]
fn fact_labels_survive_evidence_packet_export() {
    let fixture = load_fixture();
    let packet = imported_packet(GraphCueSurface::SupportExport);
    let value = serde_json::to_value(&packet).expect("packet serializes to evidence value");

    assert_eq!(value["record_kind"], "graph_fact_cue_packet");
    assert_eq!(value["export_preserves_fact_labels"], true);
    for field in fixture.evidence_packet_required_fields {
        assert!(
            value["cues"][0].get(&field).is_some(),
            "missing exported field {field} in {value}"
        );
    }
    assert_eq!(value["cues"][0]["truth_lane"], "imported_graph_fact");
    let labels = value["cues"][0]["export_labels"]
        .as_array()
        .expect("export labels are an array");
    assert!(labels
        .iter()
        .any(|label| label.as_str() == Some("imported_graph_fact")));
}

fn exact_local_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    let store = GraphStore::persist_snapshot(scenario_graph("local_root_workspace"))
        .expect("local graph validates");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:cue:local:greet",
        "ws:aureline",
        "greet",
    ));
    GraphFactCuePacket::from_graph_query_envelope("packet:cue:local:greet", surface, &envelope)
}

fn imported_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    let store = GraphStore::persist_snapshot(scenario_graph("imported_root_vendor_drop"))
        .expect("imported graph validates");
    let envelope = store.query(GraphQueryRequest::ownership_lookup(
        "q:cue:imported:owner",
        "ws:aureline",
        "node:file:vendor_acme_lib_rs",
    ));
    GraphFactCuePacket::from_graph_query_envelope("packet:cue:imported:owner", surface, &envelope)
}

fn partial_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    let mut graph = scenario_graph("local_root_workspace");
    let node = graph
        .nodes
        .iter_mut()
        .find(|node| node.node_id == "node:symbol:greet_fn")
        .expect("symbol node exists");
    for scope in &mut node.scope_refs {
        scope.visibility = Visibility::PartialVisible;
    }
    let store = GraphStore::persist_snapshot(graph).expect("partial graph validates");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:cue:partial:greet",
        "ws:aureline",
        "greet",
    ));
    GraphFactCuePacket::from_graph_query_envelope("packet:cue:partial:greet", surface, &envelope)
}

fn inferred_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    let store = GraphStore::persist_snapshot(scenario_graph("provider_resources_and_citations"))
        .expect("provider graph validates");
    let envelope = store.query(GraphQueryRequest::impact_seed(
        "q:cue:inferred:impact",
        "ws:aureline",
        "node:provider:issue_42",
    ));
    GraphFactCuePacket::from_graph_query_envelope("packet:cue:inferred:impact", surface, &envelope)
}

fn stale_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    let store =
        GraphStore::persist_snapshot(stale_symbol_graph()).expect("stale symbol graph validates");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:cue:stale:greet",
        "ws:aureline",
        "greet",
    ));
    GraphFactCuePacket::from_graph_query_envelope("packet:cue:stale:greet", surface, &envelope)
}

fn waiting_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    let store = GraphStore::persist_snapshot(scenario_graph("local_root_workspace"))
        .expect("local graph validates");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:cue:waiting:greet",
        "ws:missing",
        "greet",
    ));
    GraphFactCuePacket::from_graph_query_envelope("packet:cue:waiting:greet", surface, &envelope)
}

fn fallback_packet(surface: GraphCueSurface) -> GraphFactCuePacket {
    GraphFactCuePacket::from_fallback_search(
        "packet:cue:fallback:greet",
        surface,
        "q:cue:fallback:greet",
        "ws:aureline",
        "search:result:greet",
        "partial",
        "mono:graph:cue:0001",
    )
}

fn stale_symbol_graph() -> WorkspaceGraph {
    let mut graph = scenario_graph("local_root_workspace");
    let node = graph
        .nodes
        .iter_mut()
        .find(|node| node.node_id == "node:symbol:greet_fn")
        .expect("symbol node exists");
    node.freshness_frame = FreshnessFrame {
        freshness: Freshness::Stale,
        recorded_at: "mono:graph:cue:stale:0001".to_owned(),
        stale_reason: Some(StaleReason::UpstreamInputStale),
        cache_key_ref: None,
        warming_progress_hint: None,
    };
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
        .join("../../fixtures/graph/imported_fact_cues/readiness_and_fact_cues.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
