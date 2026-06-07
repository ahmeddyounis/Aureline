//! Fixture-driven coverage for the stable semantic graph object model,
//! query-family vocabulary, invalidation, topology reuse, and support export.

use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_semantic_graph_contract_packet, seeded_semantic_graph_contract_input,
    GraphConsumerSurface, GraphInvalidationClass, GraphInvalidationEvent,
    SemanticGraphContractFindingKind, SemanticGraphContractPacket,
    SemanticGraphContractPromotionState, SemanticGraphStableClaimLevel, StableQueryFamily,
    SEMANTIC_GRAPH_CONTRACT_ARTIFACT_DOC_REF, SEMANTIC_GRAPH_CONTRACT_DOC_REF,
    SEMANTIC_GRAPH_CONTRACT_FIXTURE_DIR, SEMANTIC_GRAPH_CONTRACT_PACKET_ARTIFACT_REF,
    SEMANTIC_GRAPH_CONTRACT_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ContractFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    packet_ref: String,
    expected: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    object_kind_tokens: Vec<String>,
    query_family_tokens: Vec<String>,
    required_consumer_surfaces: Vec<String>,
    support_export_safe: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> ContractFixture {
    let path = repo_root()
        .join(SEMANTIC_GRAPH_CONTRACT_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

#[test]
fn docs_schema_fixture_and_artifact_exist_on_disk() {
    assert_exists(SEMANTIC_GRAPH_CONTRACT_DOC_REF);
    assert_exists(SEMANTIC_GRAPH_CONTRACT_ARTIFACT_DOC_REF);
    assert_exists(SEMANTIC_GRAPH_CONTRACT_SCHEMA_REF);
    assert_exists(SEMANTIC_GRAPH_CONTRACT_FIXTURE_DIR);
    assert_exists(SEMANTIC_GRAPH_CONTRACT_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_packet_matches_baseline_fixture() {
    let fixture = load_fixture("baseline_stable.json");
    assert_eq!(fixture.record_kind, "semantic_graph_contract_fixture");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.case_name, "baseline_stable");
    assert_eq!(
        fixture.packet_ref,
        SEMANTIC_GRAPH_CONTRACT_PACKET_ARTIFACT_REF
    );

    let packet =
        current_stable_semantic_graph_contract_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expected.promotion_state
    );
    assert!(packet.validate().is_empty());
    assert_eq!(
        packet.object_kind_tokens(),
        fixture
            .expected
            .object_kind_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.query_family_tokens(),
        fixture
            .expected
            .query_family_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );

    for surface in GraphConsumerSurface::REQUIRED {
        assert!(
            fixture
                .expected
                .required_consumer_surfaces
                .iter()
                .any(|token| token == surface.as_str()),
            "fixture must name required surface {}",
            surface.as_str()
        );
        assert!(
            packet.has_binding_for(surface),
            "packet must preserve reconstructable binding for {}",
            surface.as_str()
        );
    }

    let export = packet.support_export(
        "support-export:semantic-graph-contract:baseline",
        "2026-06-07T00:36:00Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expected.support_export_safe
    );
}

#[test]
fn seeded_packet_matches_checked_in_contract_shape() {
    let seeded = SemanticGraphContractPacket::materialize(seeded_semantic_graph_contract_input());
    let checked_in =
        current_stable_semantic_graph_contract_packet().expect("checked-in packet validates");
    assert_eq!(seeded.promotion_state, checked_in.promotion_state);
    assert_eq!(seeded.object_kind_tokens(), checked_in.object_kind_tokens());
    assert_eq!(
        seeded.query_family_tokens(),
        checked_in.query_family_tokens()
    );
    assert_eq!(seeded.validate(), checked_in.validate());
}

#[test]
fn query_family_vocabulary_is_closed_to_stable_contract() {
    let mut input = seeded_semantic_graph_contract_input();
    input.stable_query_families.pop();
    let packet = SemanticGraphContractPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SemanticGraphContractPromotionState::BlocksStable
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == SemanticGraphContractFindingKind::QueryFamilyVocabularyDrift
    }));

    assert_eq!(StableQueryFamily::ExplainWhy.as_str(), "explain-why");
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
fn arbitrary_full_graph_rebuild_is_visible_blocker() {
    let mut input = seeded_semantic_graph_contract_input();
    input.invalidation_events.push(GraphInvalidationEvent {
        event_id: "invalidation:bad:arbitrary-full-rebuild".to_owned(),
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
fn private_consumer_graph_truth_must_be_narrowed_below_stable() {
    let mut input = seeded_semantic_graph_contract_input();
    input.consumer_action_bindings[0].uses_private_graph_shape = true;
    input.consumer_action_bindings[0].stable_claim_level =
        SemanticGraphStableClaimLevel::NarrowedBelowStable;

    let packet = SemanticGraphContractPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SemanticGraphContractPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == SemanticGraphContractFindingKind::ConsumerClaimMustBeNarrowed
    }));
}
