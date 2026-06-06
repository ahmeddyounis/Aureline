//! Fixture-driven coverage for stable hybrid retrieval governance.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_hybrid_retrieval_inspector_packet, EmbeddingIndexStateClass,
    RetrievalConsumerSurface, RetrievalGovernanceTrack, RetrievalInspectorFindingKind,
    RetrievalInspectorPacket, RetrievalInspectorPacketInput, RetrievalLaneClass,
    RetrievalLocalityClass, RetrievalPromotionState, RetrievalQueryClass,
    HYBRID_RETRIEVAL_STABLE_ARTIFACT_DOC_REF, HYBRID_RETRIEVAL_STABLE_DOC_REF,
    HYBRID_RETRIEVAL_STABLE_FIXTURE_DIR, HYBRID_RETRIEVAL_STABLE_PACKET_REF,
    HYBRID_RETRIEVAL_STABLE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StableFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: RetrievalInspectorPacketInput,
    expect: ExpectedStableFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedStableFixture {
    promotion_state: String,
    declared_lane_tokens: Vec<String>,
    support_export_safe: bool,
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

fn load_fixture() -> StableFixture {
    let path = repo_root()
        .join(HYBRID_RETRIEVAL_STABLE_FIXTURE_DIR)
        .join("baseline_stable.json");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn materialized_baseline() -> RetrievalInspectorPacket {
    RetrievalInspectorPacket::materialize(load_fixture().input)
}

fn finding_tokens(packet: &RetrievalInspectorPacket) -> BTreeSet<&'static str> {
    packet
        .validation_findings
        .iter()
        .map(|finding| finding.finding_kind.as_str())
        .collect()
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(HYBRID_RETRIEVAL_STABLE_SCHEMA_REF);
    assert_exists(HYBRID_RETRIEVAL_STABLE_DOC_REF);
    assert_exists(HYBRID_RETRIEVAL_STABLE_ARTIFACT_DOC_REF);
    assert_exists(HYBRID_RETRIEVAL_STABLE_FIXTURE_DIR);
    assert_exists(HYBRID_RETRIEVAL_STABLE_PACKET_REF);
}

#[test]
fn stable_fixture_materializes_promotable_packet() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.record_kind,
        "hybrid_retrieval_inspector_stable_case"
    );
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.case_name, "baseline_stable");
    assert!(
        fixture
            .scenario
            .contains("lexical, structural, graph, embedding, and fusion"),
        "fixture scenario should name the stable lane families"
    );

    let packet = RetrievalInspectorPacket::materialize(fixture.input);
    assert_eq!(packet.governance_track, RetrievalGovernanceTrack::Stable);
    assert_eq!(packet.query_class, RetrievalQueryClass::Mixed);
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state
    );
    assert_eq!(
        packet.declared_lane_tokens(),
        fixture
            .expect
            .declared_lane_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.validation_findings.len(),
        fixture.expect.validation_finding_count
    );
    assert_eq!(packet.promotion_state, RetrievalPromotionState::Promotable);
    for surface in [
        RetrievalConsumerSurface::SearchResults,
        RetrievalConsumerSurface::AiContext,
        RetrievalConsumerSurface::ReviewWorkspace,
        RetrievalConsumerSurface::DocsHelp,
        RetrievalConsumerSurface::SupportExport,
    ] {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} projection",
            surface.as_str()
        );
    }

    let export = packet.support_export(
        "support-export:retrieval-inspector:m4:stable",
        "2026-06-06T20:00:02Z",
    );
    assert_eq!(export.is_export_safe(), fixture.expect.support_export_safe);
}

#[test]
fn checked_in_stable_artifact_matches_fixture_packet() {
    let expected_packet = materialized_baseline();
    let path = repo_root().join(HYBRID_RETRIEVAL_STABLE_PACKET_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("artifact {path:?} must read: {err}"));
    let artifact_packet: RetrievalInspectorPacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("artifact {path:?} must parse: {err}"));

    assert_eq!(artifact_packet, expected_packet);
    assert!(artifact_packet.validate().is_empty());
    assert!(current_stable_hybrid_retrieval_inspector_packet().is_ok());
}

#[test]
fn stable_packet_blocks_missing_query_classification() {
    let mut fixture = load_fixture();
    fixture.input.query_class = RetrievalQueryClass::Unclassified;
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(finding_tokens(&packet)
        .contains(RetrievalInspectorFindingKind::MissingQueryClassification.as_str()));
}

#[test]
fn stable_packet_blocks_invalidated_embedding_epoch() {
    let mut fixture = load_fixture();
    fixture.input.embedding_indexes[0].state = EmbeddingIndexStateClass::IncompatibleEpoch;
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(finding_tokens(&packet)
        .contains(RetrievalInspectorFindingKind::EmbeddingEpochInvalidated.as_str()));
}

#[test]
fn stable_packet_blocks_mixed_generation_embedding_rows() {
    let mut fixture = load_fixture();
    let mut next_manifest = fixture.input.embedding_indexes[0].clone();
    next_manifest.manifest_id = "embedding-index:workspace:next-generation".to_owned();
    next_manifest.retrieval_epoch = "retrieval-epoch:aureline:2026-06-06:v5".to_owned();
    next_manifest.embedder_model_version = "model-digest:code-doc-local:2026-06-05".to_owned();
    fixture.input.embedding_indexes.push(next_manifest);

    let mut next_contribution = fixture.input.rows[0].contributions[2].clone();
    next_contribution.retrieval_epoch = Some("retrieval-epoch:aureline:2026-06-06:v5".to_owned());
    next_contribution.embedder_model_version =
        Some("model-digest:code-doc-local:2026-06-05".to_owned());
    fixture.input.rows[0].contributions.push(next_contribution);
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(finding_tokens(&packet)
        .contains(RetrievalInspectorFindingKind::MixedGenerationRecall.as_str()));
}

#[test]
fn stable_packet_blocks_managed_embedding_without_tenant_boundary() {
    let mut fixture = load_fixture();
    fixture.input.embedding_indexes[0].locality = RetrievalLocalityClass::ManagedTenantScoped;
    fixture.input.embedding_indexes[0].tenant_scope_ref = None;
    fixture.input.embedding_indexes[0].region_ref = None;
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(finding_tokens(&packet)
        .contains(RetrievalInspectorFindingKind::TenantBoundaryUndisclosed.as_str()));
}

#[test]
fn stable_packet_blocks_unsigned_mirrored_embedding_pack() {
    let mut fixture = load_fixture();
    fixture.input.embedding_indexes[1].signed_pack_ref = None;
    fixture.input.embedding_indexes[1].compatibility_ref = None;
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(finding_tokens(&packet)
        .contains(RetrievalInspectorFindingKind::SignedPackCompatibilityUndisclosed.as_str()));
}

#[test]
fn stable_packet_blocks_policy_hidden_omission_without_disclosure() {
    let mut fixture = load_fixture();
    fixture.input.omissions[0].disclosure_ref = None;
    let packet = RetrievalInspectorPacket::materialize(fixture.input);

    assert_eq!(packet.promotion_state, RetrievalPromotionState::Blocked);
    assert!(finding_tokens(&packet)
        .contains(RetrievalInspectorFindingKind::PolicyHiddenOmissionUndisclosed.as_str()));
}

#[test]
fn closed_stable_tokens_are_pinned() {
    assert_eq!(RetrievalGovernanceTrack::Stable.as_str(), "stable");
    assert_eq!(RetrievalQueryClass::Exact.as_str(), "exact");
    assert_eq!(RetrievalQueryClass::Structural.as_str(), "structural");
    assert_eq!(RetrievalQueryClass::Conceptual.as_str(), "conceptual");
    assert_eq!(RetrievalQueryClass::Mixed.as_str(), "mixed");
    assert_eq!(RetrievalLaneClass::Structural.as_str(), "structural");
    assert_eq!(RetrievalLaneClass::Embedding.as_str(), "embedding");
}
