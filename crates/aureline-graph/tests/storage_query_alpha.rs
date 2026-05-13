use std::path::Path;

use aureline_graph::{
    EdgeClass, GraphAlphaQueryClass, GraphQueryRequest, GraphQueryRowClass, GraphStore, NodeClass,
};
use aureline_graph_proto::{
    all_scenarios, AnchorKind, ConfidenceLevel, EdgeBody, EdgeEvidence, Freshness, FreshnessFrame,
    GraphEdge, GraphNode, InvalidationProducerTag, NodeBody, ProvenanceClass, ProvenanceStamp,
    QueryFamilyTag, ShardAffinityTag, SourceAnchor, SourceClass, WorkspaceGraph,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    expected_descriptor_classes: Vec<String>,
    queries: Vec<QueryExpectation>,
}

#[derive(Debug, Deserialize)]
struct QueryExpectation {
    query_class: String,
    request_id: String,
    subject_node_id: Option<String>,
    label_token: Option<String>,
    expected_readiness: String,
    expected_row_count: usize,
    expected_row_refs: Vec<String>,
}

#[test]
fn graph_store_queries_launch_wedge_symbols_imports_and_ownership() {
    let fixture = load_fixture("launch_wedge_symbols_imports_ownership.json");
    assert_eq!(fixture.record_kind, "graph_query_family_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let descriptors = GraphStore::query_family_descriptors();
    let descriptor_tokens = descriptors
        .iter()
        .map(|descriptor| descriptor.query_class.as_str().to_string())
        .collect::<Vec<_>>();
    assert_eq!(descriptor_tokens, fixture.expected_descriptor_classes);

    let store = GraphStore::persist_snapshot(graph_with_import_edge())
        .expect("graph fixture must validate before persistence");
    assert_eq!(store.workspace_id(), "ws:aureline");
    assert!(store.node_count() >= 7);
    assert!(store.edge_count() >= 6);

    for expected in fixture.queries {
        let request = request_from_expectation(&expected);
        let envelope = store.query(request);
        assert_eq!(envelope.schema_version, 1);
        assert_eq!(envelope.query_class.as_str(), expected.query_class);
        assert_eq!(envelope.readiness.as_str(), expected.expected_readiness);
        assert_eq!(envelope.rows.len(), expected.expected_row_count);
        assert_eq!(row_refs(&envelope.rows), expected.expected_row_refs);
    }
}

#[test]
fn graph_store_rejects_invalid_snapshots_before_querying() {
    let mut graph = graph_with_import_edge();
    let duplicate = graph.nodes[0].clone();
    graph.nodes.push(duplicate);

    let error = GraphStore::persist_snapshot(graph)
        .expect_err("duplicate graph ids must fail canonical validation");
    let aureline_graph::GraphStoreError::ValidationFailed(errors) = error;
    assert!(
        errors.iter().any(|error| matches!(
            error,
            aureline_graph_proto::ValidationError::DuplicateNodeId { .. }
        )),
        "expected duplicate node id validation failure, got {errors:?}"
    );
}

#[test]
fn graph_rows_preserve_truth_for_search_and_navigation_consumers() {
    let store = GraphStore::persist_snapshot(graph_with_import_edge())
        .expect("graph fixture must validate before persistence");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:nav:symbol:greet",
        "ws:aureline",
        "greet",
    ));

    assert!(!envelope.requires_partial_truth_disclosure());
    let row = envelope
        .rows
        .first()
        .expect("symbol lookup returns one row");
    assert_eq!(row.row_class, GraphQueryRowClass::Node);
    assert_eq!(row.node_class, Some(NodeClass::SymbolNode));
    assert_eq!(row.node_id.as_deref(), Some("node:symbol:greet_fn"));
    assert_eq!(row.symbol_ref.as_deref(), Some("aureline::greet"));
    assert_eq!(row.relative_path.as_deref(), Some("src/lib.rs"));
    assert_eq!(row.confidence_level, ConfidenceLevel::High);
    assert_eq!(row.freshness_frame.freshness, Freshness::Authoritative);
}

fn load_fixture(name: &str) -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/graph/query_family_alpha")
        .join(name);
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn request_from_expectation(expected: &QueryExpectation) -> GraphQueryRequest {
    match expected.query_class.as_str() {
        "symbol_lookup" => GraphQueryRequest::symbol_lookup(
            expected.request_id.clone(),
            "ws:aureline",
            expected
                .label_token
                .clone()
                .expect("symbol fixture carries a label token"),
        ),
        "import_neighborhood" => GraphQueryRequest::import_neighborhood(
            expected.request_id.clone(),
            "ws:aureline",
            expected
                .subject_node_id
                .clone()
                .expect("import fixture carries a subject node"),
        ),
        "ownership_lookup" => GraphQueryRequest::ownership_lookup(
            expected.request_id.clone(),
            "ws:aureline",
            expected
                .subject_node_id
                .clone()
                .expect("ownership fixture carries a subject node"),
        ),
        other => panic!("unexpected fixture query class {other}"),
    }
}

fn row_refs(rows: &[aureline_graph::GraphQueryRow]) -> Vec<String> {
    rows.iter()
        .map(|row| {
            row.node_id
                .clone()
                .or_else(|| row.edge_id.clone())
                .expect("query row carries a graph ref")
        })
        .collect()
}

fn graph_with_import_edge() -> WorkspaceGraph {
    let mut graph = all_scenarios()
        .into_iter()
        .find(|scenario| scenario.label == "local_root_workspace")
        .expect("local graph scenario exists")
        .graph;
    let scope_ref = graph.scope_refs[0].clone();

    graph.nodes.push(GraphNode {
        node_id: "node:file:dep_rs".into(),
        node_class: NodeClass::FileNode,
        workspace_id: "ws:aureline".into(),
        node_body: NodeBody::File {
            filesystem_identity: aureline_graph_proto::FilesystemIdentity {
                presentation_path: "/workspace/aureline/src/dep.rs".into(),
                logical_workspace_identity: "ws:aureline/root:0/src/dep.rs".into(),
                canonical_filesystem_object: "fs:vol:1:file:1044:gen:1".into(),
                alias_set: vec![],
                save_target_token: "sv:ws:aureline:root:0:file:1044:gen:1".into(),
            },
            media_class: Some("text_source".into()),
            language_id: Some("rust".into()),
            large_file_mode: false,
        },
        display_label: Some("src/dep.rs".into()),
        provenance_stamp: authoritative_stamp(1, 20, SourceClass::WorkspaceFilesystem),
        freshness_frame: authoritative(1, 20),
        confidence_level: ConfidenceLevel::High,
        confidence_rollup: None,
        query_family_tags: vec![
            QueryFamilyTag::SemanticCodeSearch,
            QueryFamilyTag::PublicGraphQuery,
        ],
        shard_affinity_tags: vec![ShardAffinityTag::WorkspaceRootLocal],
        invalidation_producer_tags: vec![InvalidationProducerTag::WorkspaceVfsWriter],
        scope_refs: vec![scope_ref.clone()],
        source_anchors: vec![SourceAnchor {
            anchor_kind: AnchorKind::FilesystemIdentity,
            anchor_ref: "fs:vol:1:file:1044:gen:1".into(),
            line_range: None,
        }],
        impact_reasons: vec![],
        explainer_citations: vec![],
    });

    graph.edges.push(GraphEdge {
        edge_id: "edge:imports:lib_to_dep".into(),
        edge_class: EdgeClass::ImportsModule,
        workspace_id: "ws:aureline".into(),
        from_node_id: "node:file:lib_rs".into(),
        to_node_id: "node:file:dep_rs".into(),
        evidence: EdgeEvidence {
            evidence_state: aureline_graph_proto::EdgeEvidenceState::DirectEvidence,
            provenance_stamp: authoritative_stamp(1, 21, SourceClass::SymbolResolver),
            freshness_frame: authoritative(1, 21),
            confidence_level: ConfidenceLevel::High,
            confidence_rollup: None,
        },
        body: EdgeBody::default(),
        query_family_tags: vec![QueryFamilyTag::SemanticCodeSearch],
        shard_affinity_tags: vec![ShardAffinityTag::SymbolCacheShard],
        invalidation_producer_tags: vec![InvalidationProducerTag::SymbolResolverRebuild],
        scope_refs: vec![scope_ref],
        source_anchors: vec![SourceAnchor {
            anchor_kind: AnchorKind::SymbolDefinitionSite,
            anchor_ref: "node:file:lib_rs".into(),
            line_range: Some("4:4".into()),
        }],
    });

    graph
}

fn authoritative(scenario: u8, tick: u16) -> FreshnessFrame {
    FreshnessFrame {
        freshness: Freshness::Authoritative,
        recorded_at: format!("mono:graph:{scenario:04}:00:00:00.{tick:04}"),
        stale_reason: None,
        cache_key_ref: None,
        warming_progress_hint: None,
    }
}

fn authoritative_stamp(scenario: u8, tick: u16, source_class: SourceClass) -> ProvenanceStamp {
    ProvenanceStamp {
        source_class,
        provenance_class: ProvenanceClass::AuthoritativeProducer,
        producer_ref: Some(source_class.as_str().to_string()),
        producer_version: Some("0.0.0".to_string()),
        recorded_at: format!("mono:graph:{scenario:04}:00:00:00.{tick:04}"),
        imported_bundle_ref: None,
        replay_capture_ref: None,
        support_ref: None,
    }
}

#[test]
fn descriptor_tokens_are_stable() {
    let descriptors = GraphStore::query_family_descriptors();
    assert_eq!(
        descriptors
            .iter()
            .map(|descriptor| descriptor.query_class)
            .collect::<Vec<_>>(),
        vec![
            GraphAlphaQueryClass::SymbolLookup,
            GraphAlphaQueryClass::ImportNeighborhood,
            GraphAlphaQueryClass::OwnershipLookup,
            GraphAlphaQueryClass::ImpactSeed,
            GraphAlphaQueryClass::ExplainerCitationSeed,
        ]
    );
}
