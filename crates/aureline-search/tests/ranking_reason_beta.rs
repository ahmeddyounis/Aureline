//! Fixture-driven coverage for ranking-reason and operator-truth beta packets.

use std::path::{Path, PathBuf};

use aureline_search::{
    current_beta_search_operator_truth_packet, PartialIndexDrillState, RankingReasonSignal,
    SearchOperatorConsumerSurface, SearchOperatorDowngradeState, SearchOperatorPromotionState,
    SearchOperatorTruthFindingKind, SearchOperatorTruthPacket, SearchOperatorTruthPacketInput,
    SEARCH_OPERATOR_TRUTH_DOC_REF, SEARCH_OPERATOR_TRUTH_FIXTURE_DIR,
    SEARCH_OPERATOR_TRUTH_PACKET_ARTIFACT_REF, SEARCH_OPERATOR_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OperatorTruthFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SearchOperatorTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    ranking_reason_tokens: Vec<String>,
    covered_drill_state_tokens: Vec<String>,
    support_export_safe: bool,
    ai_projection_present: bool,
    review_projection_present: bool,
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

fn load_fixture() -> OperatorTruthFixture {
    let path = repo_root()
        .join(SEARCH_OPERATOR_TRUTH_FIXTURE_DIR)
        .join("graph_ai_operator_truth.json");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(SEARCH_OPERATOR_TRUTH_SCHEMA_REF);
    assert_exists(SEARCH_OPERATOR_TRUTH_DOC_REF);
    assert_exists(SEARCH_OPERATOR_TRUTH_FIXTURE_DIR);
    assert_exists(SEARCH_OPERATOR_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn fixture_materializes_reviewable_operator_truth_packet() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "search_operator_truth_beta_case");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.case_name, "graph_ai_operator_truth");
    assert!(
        fixture.scenario.contains("ranking reasons"),
        "fixture scenario should explain the ranking-reason lane"
    );

    let packet = SearchOperatorTruthPacket::materialize(fixture.input);
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state
    );
    assert_eq!(packet.rows.len(), fixture.expect.row_count);
    assert_eq!(
        packet.validate().len(),
        fixture.expect.validation_finding_count
    );
    assert_eq!(
        packet.ranking_reason_tokens(),
        fixture
            .expect
            .ranking_reason_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.partial_index_drill.covered_state_tokens(),
        fixture
            .expect
            .covered_drill_state_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.has_projection_for(SearchOperatorConsumerSurface::AiContext),
        fixture.expect.ai_projection_present
    );
    assert_eq!(
        packet.has_projection_for(SearchOperatorConsumerSurface::ReviewWorkspace),
        fixture.expect.review_projection_present
    );
    assert!(packet.has_projection_for(SearchOperatorConsumerSurface::GraphOverlay));
    assert!(packet.partial_index_drill.is_reusable());
    assert!(packet.partial_index_drill.all_degraded_rows_downgraded());

    let export = packet.support_export(
        "support-export:search-operator-truth:payments",
        "2026-05-17T15:00:08Z",
    );
    assert_eq!(export.is_export_safe(), fixture.expect.support_export_safe);
}

#[test]
fn checked_in_artifact_matches_materialized_fixture_packet() {
    let fixture = load_fixture();
    let expected_packet = SearchOperatorTruthPacket::materialize(fixture.input);
    let artifact_packet =
        current_beta_search_operator_truth_packet().expect("checked-in packet validates");

    assert_eq!(artifact_packet, expected_packet);
    assert_eq!(
        artifact_packet.promotion_state,
        SearchOperatorPromotionState::NeedsReview
    );
    assert!(artifact_packet.validate().is_empty());
}

#[test]
fn stale_or_failing_drill_rows_must_downgrade_affected_rows() {
    let mut fixture = load_fixture();
    fixture.input.partial_index_drill.rows[2].downgrade_state = SearchOperatorDowngradeState::None;
    fixture.input.partial_index_drill.rows[2]
        .blocked_actions
        .clear();
    let packet = SearchOperatorTruthPacket::materialize(fixture.input);

    assert_eq!(
        packet.promotion_state,
        SearchOperatorPromotionState::Blocked
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == SearchOperatorTruthFindingKind::PartialIndexRowNotDowngraded
    }));
}

#[test]
fn missing_ai_or_review_projection_blocks_promotion() {
    let mut fixture = load_fixture();
    fixture.input.consumer_projections.retain(|projection| {
        projection.consumer_surface != SearchOperatorConsumerSurface::AiContext
    });
    let packet = SearchOperatorTruthPacket::materialize(fixture.input);

    assert_eq!(
        packet.promotion_state,
        SearchOperatorPromotionState::Blocked
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == SearchOperatorTruthFindingKind::MissingConsumerProjection
    }));
}

#[test]
fn closed_operator_truth_tokens_are_pinned() {
    assert_eq!(
        SearchOperatorConsumerSurface::AiContext.as_str(),
        "ai_context"
    );
    assert_eq!(
        SearchOperatorConsumerSurface::ReviewWorkspace.as_str(),
        "review_workspace"
    );
    assert_eq!(RankingReasonSignal::PartialIndex.as_str(), "partial_index");
    assert_eq!(RankingReasonSignal::StaleShard.as_str(), "stale_shard");
    assert_eq!(PartialIndexDrillState::Failing.as_str(), "failing");
    assert_eq!(
        SearchOperatorDowngradeState::RedBlocksBetaPromotion.as_str(),
        "red_blocks_beta_promotion"
    );
}
