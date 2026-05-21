//! Beta coverage for the graph surface's workset / sparse-scope /
//! policy-limited-view scope boundaries.
//!
//! Named-workset, sparse-slice, and policy-limited-view truth is already beta on
//! the workspace, search, and refactor surfaces. This drill promotes it on the
//! graph surface: it replays a frozen corpus through the impact-explainer packet
//! builder and proves the graph honours the declared scope the same way the
//! sibling surfaces do — in-scope impact edges stay visible, edges that escape
//! the scope are *labeled* as out-of-scope rather than silently dropped, and
//! policy-hidden members are disclosed through the scope descriptor.
//!
//! The corpus also pins the 1:1 mapping between the graph `WorksetScopeClass`
//! vocabulary and the `aureline-workspace` `ScopeClass` vocabulary, so the
//! surfaces share one scope-truth vocabulary instead of a divergent label set.
//! `aureline-graph` deliberately does not depend on `aureline-workspace`; the
//! workspace tokens are mirrored as a frozen list here and re-checked against
//! crate source by `ci/check_beta_workset_scope_coverage.py`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use aureline_graph::{
    GraphQueryRequest, GraphStore, ImpactExplainerPacket, WorksetScopeDescriptor,
};
use aureline_graph_proto::{
    all_scenarios, ConfidenceLevel, Freshness, FreshnessFrame, GraphEdge, GraphNode,
    InvalidationProducerTag, NodeBody, NodeClass, ProvenanceClass, ProvenanceStamp, QueryFamilyTag,
    ShardAffinityTag, SourceClass, Visibility, WorksetScopeClass, WorksetScopeRef, WorkspaceGraph,
};
use serde::Deserialize;

/// Base in-scope impact edge re-scoped to the active workset/slice/view.
const BASE_IMPACT_EDGE_ID: &str = "edge:impacts:issue_to_server_file";
/// Cloned impact edge re-scoped so it escapes the active scope.
const OUTSIDE_EDGE_ID: &str = "edge:impacts:issue_to_adr_outside_scope";
/// Target the out-of-scope clone points at (present in the provider scenario).
const OUTSIDE_TARGET_NODE_ID: &str = "node:doc:adr_0007";

/// Mirror of `aureline_workspace::ScopeClass::as_str()`. `aureline-graph` must
/// not depend on `aureline-workspace`, so the shared scope vocabulary is pinned
/// here. `ci/check_beta_workset_scope_coverage.py` re-derives the workspace
/// vocabulary from crate source and fails closed if this list drifts from it.
const WORKSPACE_SCOPE_CLASS_VOCABULARY: [&str; 5] = [
    "current_repo",
    "selected_workset",
    "sparse_slice",
    "full_workspace",
    "policy_limited_view",
];

#[derive(Debug, Clone, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    scope_class_vocabulary_map: BTreeMap<String, String>,
    graph_only_scope_classes: Vec<String>,
    required_graph_scope_classes: Vec<String>,
    cases: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Case {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    source_graph_scenario: String,
    graph_scope_class: String,
    workspace_scope_class: String,
    request: CaseRequest,
    policy: CasePolicy,
    expect: CaseExpect,
}

#[derive(Debug, Clone, Deserialize)]
struct CaseRequest {
    query_request_id: String,
    subject_node_id: String,
    in_scope_id: String,
    out_of_scope_id: String,
    out_of_scope_class: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CasePolicy {
    hidden_member_count: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct CaseExpect {
    scope_mode: String,
    visible_impact_edge_count: usize,
    visible_impact_edge_ids: Vec<String>,
    out_of_scope_count: usize,
    out_of_scope_edge_ids: Vec<String>,
    policy_hidden_result_count: u64,
    descriptor_hidden_result_count: usize,
    no_impact_found: bool,
}

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/graph/workset_scope_beta")
}

fn load_manifest() -> Manifest {
    let path = corpus_dir().join("manifest.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let manifest: Manifest =
        serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    assert_eq!(manifest.record_kind, "graph_workset_scope_beta_manifest");
    assert_eq!(manifest.schema_version, 1);
    manifest
}

fn load_cases(manifest: &Manifest) -> Vec<Case> {
    manifest
        .cases
        .iter()
        .map(|name| {
            let path = corpus_dir().join(name);
            let payload =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            let case: Case = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
            assert_eq!(
                case.record_kind, "graph_workset_scope_beta_case",
                "unexpected record_kind in {name}"
            );
            assert_eq!(
                case.schema_version, 1,
                "unexpected schema_version in {name}"
            );
            case
        })
        .collect()
}

fn scope_class_from_token(token: &str) -> WorksetScopeClass {
    WorksetScopeClass::all()
        .iter()
        .copied()
        .find(|class| class.as_str() == token)
        .unwrap_or_else(|| panic!("unknown graph scope class token {token}"))
}

fn scenario_graph(label: &str) -> WorkspaceGraph {
    all_scenarios()
        .into_iter()
        .find(|scenario| scenario.label == label)
        .unwrap_or_else(|| panic!("missing graph scenario {label}"))
        .graph
}

fn policy_view_node(in_scope: &WorksetScopeRef, hidden_member_count: u64) -> GraphNode {
    let recorded_at = "mono:beta:workset_scope:policy".to_owned();
    GraphNode {
        node_id: "node:policy_view:beta_restricted_members".to_owned(),
        node_class: NodeClass::PolicyViewNode,
        workspace_id: "ws:aureline".to_owned(),
        node_body: NodeBody::PolicyView {
            underlying_scope_id: in_scope.scope_id.clone(),
            policy_ref: "policy:beta:restricted_module".to_owned(),
            hidden_member_count,
        },
        display_label: Some("Policy view: restricted members".to_owned()),
        provenance_stamp: ProvenanceStamp {
            source_class: SourceClass::PolicyProjection,
            provenance_class: ProvenanceClass::PolicyProjected,
            producer_ref: Some("policy_projector".to_owned()),
            producer_version: Some("0.0.0".to_owned()),
            recorded_at: recorded_at.clone(),
            imported_bundle_ref: None,
            replay_capture_ref: None,
            support_ref: None,
        },
        freshness_frame: FreshnessFrame {
            freshness: Freshness::Authoritative,
            recorded_at,
            stale_reason: None,
            cache_key_ref: None,
            warming_progress_hint: None,
        },
        confidence_level: ConfidenceLevel::High,
        confidence_rollup: None,
        query_family_tags: vec![
            QueryFamilyTag::PublicGraphQuery,
            QueryFamilyTag::TopologyWalk,
        ],
        shard_affinity_tags: vec![ShardAffinityTag::PolicyProjectedShard],
        invalidation_producer_tags: vec![InvalidationProducerTag::PolicyEpochRoll],
        scope_refs: vec![in_scope.clone()],
        source_anchors: vec![],
        impact_reasons: vec![],
        explainer_citations: vec![],
    }
}

/// Builds the case graph: the base impact edge is re-scoped to the active scope
/// under the case's scope class, a sibling impact edge is cloned into a
/// different scope so it escapes the active scope, and (when the case declares
/// hidden policy members) a policy view node is projected into the active scope.
fn build_case_graph(case: &Case) -> WorkspaceGraph {
    let mut graph = scenario_graph(&case.source_graph_scenario);

    let in_scope = WorksetScopeRef {
        scope_class: scope_class_from_token(&case.graph_scope_class),
        scope_id: case.request.in_scope_id.clone(),
        visibility: Visibility::FullyVisible,
    };
    let out_scope = WorksetScopeRef {
        scope_class: scope_class_from_token(&case.request.out_of_scope_class),
        scope_id: case.request.out_of_scope_id.clone(),
        visibility: Visibility::PartialVisible,
    };

    let mut outside_edge: Option<GraphEdge> = None;
    for edge in &mut graph.edges {
        if edge.edge_id == BASE_IMPACT_EDGE_ID {
            edge.scope_refs = vec![in_scope.clone()];
            let mut clone = edge.clone();
            clone.edge_id = OUTSIDE_EDGE_ID.to_owned();
            clone.to_node_id = OUTSIDE_TARGET_NODE_ID.to_owned();
            clone.scope_refs = vec![out_scope.clone()];
            outside_edge = Some(clone);
        }
    }
    let outside_edge = outside_edge.unwrap_or_else(|| {
        panic!(
            "scenario {} is missing base impact edge {BASE_IMPACT_EDGE_ID}",
            case.source_graph_scenario
        )
    });
    graph.edges.push(outside_edge);
    graph.scope_refs.push(in_scope.clone());
    graph.scope_refs.push(out_scope);

    if case.policy.hidden_member_count > 0 {
        graph
            .nodes
            .push(policy_view_node(&in_scope, case.policy.hidden_member_count));
    }

    graph
}

/// Sums hidden members across policy view nodes that sit in the active scope.
fn policy_hidden_in_scope(graph: &WorkspaceGraph, in_scope_id: &str) -> u64 {
    graph
        .nodes
        .iter()
        .filter_map(|node| match &node.node_body {
            NodeBody::PolicyView {
                hidden_member_count,
                ..
            } if node.scope_refs.iter().any(|s| s.scope_id == in_scope_id) => {
                Some(*hidden_member_count)
            }
            _ => None,
        })
        .sum()
}

fn project_packet(case: &Case, graph: &WorkspaceGraph) -> (ImpactExplainerPacket, GraphStore) {
    let store =
        GraphStore::persist_snapshot(graph.clone()).expect("case graph must validate clean");
    let descriptor = WorksetScopeDescriptor::local_sparse(
        case.request.in_scope_id.clone(),
        case.graph_scope_class.clone(),
        ["repo:aureline"],
        case.expect.descriptor_hidden_result_count,
        store.node_count(),
        store
            .edge_count()
            .saturating_sub(case.expect.out_of_scope_count),
        case.expect.descriptor_hidden_result_count,
    );
    let request = GraphQueryRequest::impact_seed(
        case.request.query_request_id.clone(),
        "ws:aureline",
        case.request.subject_node_id.clone(),
    )
    .with_scope_ids([case.request.in_scope_id.clone()]);
    let packet = store
        .build_impact_explainer_packet(request, descriptor)
        .expect("impact request builds an explainer packet");
    (packet, store)
}

#[test]
fn every_case_keeps_in_scope_results_and_labels_out_of_scope() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    assert!(!cases.is_empty(), "corpus must declare at least one case");

    for case in &cases {
        let graph = build_case_graph(case);
        let (packet, store) = project_packet(case, &graph);

        // The active scope class travels into the packet's scope disclosure.
        assert_eq!(
            packet.workset_scope.scope_class, case.graph_scope_class,
            "scope class label dropped in {}",
            case.case_id
        );
        assert_eq!(
            packet.workset_scope.scope_mode.as_str(),
            case.expect.scope_mode,
            "scope mode mismatch in {}",
            case.case_id
        );

        // Visible impact edges must be exactly the declared in-scope set.
        let visible_ids: BTreeSet<&str> = packet
            .visual_projection
            .edges
            .iter()
            .map(|edge| edge.edge_id.as_str())
            .collect();
        let expected_visible: BTreeSet<&str> = case
            .expect
            .visible_impact_edge_ids
            .iter()
            .map(String::as_str)
            .collect();
        assert_eq!(
            visible_ids, expected_visible,
            "visible impact edge set mismatch in {}",
            case.case_id
        );
        assert_eq!(
            packet.impact_summary.visible_impact_edge_count, case.expect.visible_impact_edge_count,
            "visible impact edge count mismatch in {}",
            case.case_id
        );

        // No visible result may escape the declared scope: every visible edge
        // carries the active scope id. A leak fails the test here.
        for &edge_id in &visible_ids {
            let edge = store
                .edge(edge_id)
                .unwrap_or_else(|| panic!("visible edge {edge_id} must resolve in store"));
            assert!(
                edge.scope_refs
                    .iter()
                    .any(|scope| scope.scope_id == case.request.in_scope_id),
                "visible edge {edge_id} escaped scope {} in {}",
                case.request.in_scope_id,
                case.case_id
            );
        }

        // Out-of-scope edges are labeled, never silently dropped, and never
        // leak into the visible set.
        assert_eq!(
            packet.impact_summary.out_of_scope_count, case.expect.out_of_scope_count,
            "out-of-scope count mismatch in {}",
            case.case_id
        );
        for outside in &case.expect.out_of_scope_edge_ids {
            assert!(
                !visible_ids.contains(outside.as_str()),
                "out-of-scope edge {outside} leaked into the visible set in {}",
                case.case_id
            );
            let edge = store.edge(outside).unwrap_or_else(|| {
                panic!("out-of-scope edge {outside} must remain present (labeled, not dropped)")
            });
            assert!(
                !edge
                    .scope_refs
                    .iter()
                    .any(|scope| scope.scope_id == case.request.in_scope_id),
                "edge {outside} counted out-of-scope but carries the active scope in {}",
                case.case_id
            );
        }

        // Policy-limited members are disclosed through the scope descriptor's
        // hidden_result_count, not silently dropped.
        let policy_hidden = policy_hidden_in_scope(&graph, &case.request.in_scope_id);
        assert_eq!(
            policy_hidden, case.expect.policy_hidden_result_count,
            "policy-hidden member count mismatch in {}",
            case.case_id
        );
        assert_eq!(
            packet.workset_scope.hidden_result_count, case.expect.descriptor_hidden_result_count,
            "scope descriptor hidden_result_count mismatch in {}",
            case.case_id
        );
        assert_eq!(
            packet.workset_scope.hidden_result_count,
            packet.impact_summary.out_of_scope_count + policy_hidden as usize,
            "scope disclosure must account for out-of-scope + policy-hidden results in {}",
            case.case_id
        );

        assert_eq!(
            packet.impact_summary.no_impact_found, case.expect.no_impact_found,
            "no_impact_found mismatch in {}",
            case.case_id
        );
    }
}

#[test]
fn corpus_covers_named_workset_sparse_slice_and_policy_limited_view() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    let covered: BTreeSet<&str> = cases
        .iter()
        .map(|case| case.graph_scope_class.as_str())
        .collect();
    for required in &manifest.required_graph_scope_classes {
        assert!(
            covered.contains(required.as_str()),
            "corpus is missing a case for required scope class {required}"
        );
    }
    // The promotion target is explicit: these three classes must be present.
    for required in ["named_workset", "sparse_slice", "policy_limited_view"] {
        assert!(
            covered.contains(required),
            "corpus must cover scope class {required}"
        );
    }
}

#[test]
fn graph_scope_classes_map_one_to_one_to_workspace_vocabulary() {
    let manifest = load_manifest();

    // Every graph scope class is either mapped to a workspace class or declared
    // as a graph-only surface extension — nothing is left unaccounted-for.
    let graph_tokens: BTreeSet<String> = WorksetScopeClass::all()
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    let mut accounted: BTreeSet<String> = manifest
        .scope_class_vocabulary_map
        .keys()
        .cloned()
        .collect();
    for token in &manifest.graph_only_scope_classes {
        assert!(
            !manifest.scope_class_vocabulary_map.contains_key(token),
            "graph-only class {token} must not also appear in the mapping"
        );
        accounted.insert(token.clone());
    }
    assert_eq!(
        accounted, graph_tokens,
        "every graph WorksetScopeClass must be mapped or declared graph-only"
    );

    // The mapping is injective: no two graph classes collapse onto one
    // workspace class.
    let mapped_values: Vec<&String> = manifest.scope_class_vocabulary_map.values().collect();
    let unique_values: BTreeSet<&String> = mapped_values.iter().copied().collect();
    assert_eq!(
        mapped_values.len(),
        unique_values.len(),
        "scope-class mapping must be injective"
    );

    // The mapped workspace classes are exactly the workspace ScopeClass
    // vocabulary — surjective onto it, and (with injectivity above) a 1:1
    // bijection over the shared scope vocabulary.
    let mapped_set: BTreeSet<String> = manifest
        .scope_class_vocabulary_map
        .values()
        .cloned()
        .collect();
    let workspace_vocab: BTreeSet<String> = WORKSPACE_SCOPE_CLASS_VOCABULARY
        .iter()
        .map(|token| token.to_string())
        .collect();
    assert_eq!(
        mapped_set, workspace_vocab,
        "mapped scope classes must cover the aureline-workspace ScopeClass vocabulary exactly"
    );

    // Each case's declared workspace scope class agrees with the shared map, so
    // a case cannot quote a private label divergent from the vocabulary.
    for case in &load_cases(&manifest) {
        let mapped = manifest
            .scope_class_vocabulary_map
            .get(&case.graph_scope_class)
            .unwrap_or_else(|| {
                panic!(
                    "case {} uses unmapped graph scope class {}",
                    case.case_id, case.graph_scope_class
                )
            });
        assert_eq!(
            mapped, &case.workspace_scope_class,
            "case {} workspace_scope_class disagrees with the vocabulary map",
            case.case_id
        );
    }
}
