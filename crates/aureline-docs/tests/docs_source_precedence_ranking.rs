//! Fixture-driven coverage for the docs-source precedence/ranking parity packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_precedence_ranking_export, current_stable_docs_precedence_ranking_packet,
    seeded_stable_docs_precedence_ranking_input, DocsPrecedenceRankingPacket,
    DocsPrecedenceRankingPacketInput, DocsPrecedenceRankingPromotionState,
    DocsPrecedenceRankingSupportExport, DocsSourceLane, RankExplanationSurface,
    DOCS_PRECEDENCE_RANKING_ARTIFACT_REF, DOCS_PRECEDENCE_RANKING_DOC_REF,
    DOCS_PRECEDENCE_RANKING_FIXTURE_DIR, DOCS_PRECEDENCE_RANKING_SCHEMA_REF,
    DOCS_PRECEDENCE_RANKING_SUMMARY_REF,
};
use serde::Deserialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:docs_source_precedence_and_ranking_parity:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

#[derive(Debug, Deserialize)]
struct RankingFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsPrecedenceRankingPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
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

fn load_fixture(file_name: &str) -> RankingFixture {
    let path = repo_root()
        .join(DOCS_PRECEDENCE_RANKING_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind,
        "docs_source_precedence_and_ranking_parity_case"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DocsPrecedenceRankingPacket::materialize(fixture.input);
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}; findings: {:?}",
        fixture.case_name,
        fixture.expect.promotion_state,
        packet.promotion_state,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );

    if !fixture.expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for expected in &fixture.expect.expected_finding_kinds {
            assert!(
                observed.contains(expected.as_str()),
                "fixture {} expected finding {expected}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn doc_schema_artifact_and_fixtures_exist_on_disk() {
    assert_exists(DOCS_PRECEDENCE_RANKING_DOC_REF);
    assert_exists(DOCS_PRECEDENCE_RANKING_SCHEMA_REF);
    assert_exists(DOCS_PRECEDENCE_RANKING_ARTIFACT_REF);
    assert_exists(DOCS_PRECEDENCE_RANKING_SUMMARY_REF);
    assert_exists(DOCS_PRECEDENCE_RANKING_FIXTURE_DIR);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn source_lane_flattened_fixture_blocks_stable() {
    assert_fixture_matches("source_lane_flattened_blocks_stable.json");
}

#[test]
fn project_masquerades_as_vendor_fixture_blocks_stable() {
    assert_fixture_matches("project_masquerades_as_vendor_blocks_stable.json");
}

#[test]
fn unexplained_rank_inversion_fixture_blocks_stable() {
    assert_fixture_matches("unexplained_rank_inversion_blocks_stable.json");
}

#[test]
fn outrank_without_visible_alternative_fixture_blocks_stable() {
    assert_fixture_matches("outrank_without_visible_alternative_blocks_stable.json");
}

#[test]
fn derived_ranked_as_primary_fixture_blocks_stable() {
    assert_fixture_matches("derived_ranked_as_primary_blocks_stable.json");
}

#[test]
fn reason_class_mismatch_fixture_blocks_stable() {
    assert_fixture_matches("reason_class_mismatch_blocks_stable.json");
}

#[test]
fn hidden_ranking_model_fixture_blocks_stable() {
    assert_fixture_matches("hidden_ranking_model_blocks_stable.json");
}

#[test]
fn offline_unavailable_reason_missing_fixture_blocks_stable() {
    assert_fixture_matches("offline_unavailable_reason_missing_blocks_stable.json");
}

#[test]
fn missing_rank_explanation_surface_fixture_blocks_stable() {
    assert_fixture_matches("missing_rank_explanation_surface_blocks_stable.json");
}

#[test]
fn air_gapped_candidate_fixture_narrows_below_stable() {
    assert_fixture_matches("air_gapped_candidate_narrows_below_stable.json");
}

#[test]
fn checked_in_packet_keeps_seven_lanes_and_covers_every_surface() {
    let packet =
        current_stable_docs_precedence_ranking_packet().expect("stable ranking packet validates");
    assert_eq!(
        packet.promotion_state,
        DocsPrecedenceRankingPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    let lanes: BTreeSet<DocsSourceLane> = packet.lanes_present().into_iter().collect();
    for lane in DocsSourceLane::ALL {
        assert!(
            lanes.contains(&lane),
            "stable packet must keep the {} lane distinguishable",
            lane.as_str()
        );
    }

    let surfaces: BTreeSet<RankExplanationSurface> =
        packet.covered_surfaces().into_iter().collect();
    for surface in RankExplanationSurface::REQUIRED {
        assert!(
            surfaces.contains(&surface),
            "stable packet must explain the ranking on the {} surface",
            surface.as_str()
        );
    }

    // Support export reconstructs every ranking set (full coverage).
    for surface in RankExplanationSurface::FULL_COVERAGE {
        let covered: BTreeSet<&str> = packet
            .surface_projections
            .iter()
            .filter(|projection| projection.surface == surface)
            .map(|projection| projection.ranking_set_ref.as_str())
            .collect();
        for ranking_set in &packet.ranking_sets {
            assert!(
                covered.contains(ranking_set.subject_id.as_str()),
                "{} must reconstruct ranking set {}",
                surface.as_str(),
                ranking_set.subject_id
            );
        }
    }
}

#[test]
fn checked_in_support_export_matches_seed() {
    let path = repo_root().join(DOCS_PRECEDENCE_RANKING_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: DocsPrecedenceRankingSupportExport = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed =
        DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input())
            .support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    assert_eq!(
        from_file, from_seed,
        "checked-in support export drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- support-export > {}`",
        DOCS_PRECEDENCE_RANKING_ARTIFACT_REF,
    );
}

#[test]
fn checked_in_support_export_is_export_safe() {
    let export =
        current_stable_docs_precedence_ranking_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert!(export.export_packet.validate().is_empty());
}
