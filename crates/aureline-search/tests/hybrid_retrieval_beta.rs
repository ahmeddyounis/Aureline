//! Fixture-driven coverage for the hybrid retrieval inspector beta.
//!
//! The drill proves the schema, docs, fixture, and checked-in artifact exist,
//! then verifies that lexical, vector, and graph contributions keep locality,
//! readiness, embedder epoch, local-first fallback, and export parity visible.

use std::path::{Path, PathBuf};

use aureline_search::{
    RetrievalConsumerSurface, RetrievalInspectorFindingKind, RetrievalInspectorPacket,
    RetrievalInspectorPacketInput, RetrievalLaneClass, RetrievalPromotionState,
    HYBRID_RETRIEVAL_BETA_DOC_REF, HYBRID_RETRIEVAL_BETA_FIXTURE_DIR,
    HYBRID_RETRIEVAL_BETA_PACKET_REF, RETRIEVAL_INSPECTOR_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: RetrievalInspectorPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    lane_tokens: Vec<String>,
    remote_route_disclosed: bool,
    support_export_safe: bool,
    ai_projection_present: bool,
    validation_finding_count: usize,
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

fn load_fixture() -> CaseFixture {
    let path = repo_root()
        .join(HYBRID_RETRIEVAL_BETA_FIXTURE_DIR)
        .join("local_first_hybrid_inspector.json");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(RETRIEVAL_INSPECTOR_SCHEMA_REF);
    assert_exists(HYBRID_RETRIEVAL_BETA_DOC_REF);
    assert_exists(HYBRID_RETRIEVAL_BETA_PACKET_REF);
    assert_exists(HYBRID_RETRIEVAL_BETA_FIXTURE_DIR);
}

#[test]
fn fixture_materializes_promotable_hybrid_packet() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "hybrid_retrieval_beta_case");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.case_name, "local_first_hybrid_inspector");
    assert!(
        fixture.scenario.contains("lexical, vector, and graph"),
        "fixture scenario should explain the protected lane mix"
    );

    let packet = RetrievalInspectorPacket::materialize(fixture.input);
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state
    );
    assert_eq!(
        packet.contributing_lane_tokens(),
        fixture
            .expect
            .lane_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.local_first_policy.remote_route_disclosed,
        fixture.expect.remote_route_disclosed
    );
    assert_eq!(
        packet.validate().len(),
        fixture.expect.validation_finding_count
    );
    assert_eq!(packet.promotion_state, RetrievalPromotionState::Promotable);
    assert!(packet.has_projection_for(RetrievalConsumerSurface::SearchResults));
    assert_eq!(
        packet.has_projection_for(RetrievalConsumerSurface::AiContext),
        fixture.expect.ai_projection_present
    );
    assert!(packet.has_projection_for(RetrievalConsumerSurface::SupportExport));

    let export = packet.support_export(
        "support-export:retrieval-inspector:m3:0001",
        "2026-05-17T12:21:00Z",
    );
    assert_eq!(export.is_export_safe(), fixture.expect.support_export_safe);
    assert_eq!(export.inspector_packet, packet);
}

#[test]
fn checked_in_artifact_matches_materialized_fixture_packet() {
    let fixture = load_fixture();
    let expected_packet = RetrievalInspectorPacket::materialize(fixture.input);
    let path = repo_root().join(HYBRID_RETRIEVAL_BETA_PACKET_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("artifact {path:?} must read: {err}"));
    let artifact_packet: RetrievalInspectorPacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("artifact {path:?} must parse: {err}"));

    assert_eq!(artifact_packet, expected_packet);
    assert!(artifact_packet.validate().is_empty());
    assert!(artifact_packet.is_promotable());
}

#[test]
fn undisclosed_remote_route_blocks_promotion() {
    let mut fixture = load_fixture();
    fixture.input.local_first_policy.remote_route_disclosed = false;
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == RetrievalInspectorFindingKind::UnlabelledRemoteRoute
    }));
}

#[test]
fn vector_contribution_requires_matching_embedding_manifest() {
    let mut fixture = load_fixture();
    fixture.input.embedding_indexes.clear();
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == RetrievalInspectorFindingKind::MissingEmbeddingManifest
    }));
}

#[test]
fn closed_lane_tokens_are_pinned() {
    assert_eq!(RetrievalLaneClass::Lexical.as_str(), "lexical");
    assert_eq!(RetrievalLaneClass::Vector.as_str(), "vector");
    assert_eq!(RetrievalLaneClass::Graph.as_str(), "graph");
    assert_eq!(RetrievalLaneClass::Fused.as_str(), "fused");
    assert_eq!(RetrievalConsumerSurface::AiContext.as_str(), "ai_context");
    assert_eq!(
        RetrievalConsumerSurface::SupportExport.as_str(),
        "support_export"
    );
}
