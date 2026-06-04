//! Fixture-driven coverage for the stable docs source/result/pack/citation contract.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_source_result_pack_and_citation_packet,
    seeded_stable_docs_source_result_pack_and_citation_input, StableDocsConsumerSurface,
    StableDocsFindingKind, StableDocsPackDetailSheetKind, StableDocsPromotionState,
    StableDocsSourceResultPackCitationInput, StableDocsSourceResultPackCitationPacket,
    STABLE_DOCS_CONTRACT_ARTIFACT_DOC_REF, STABLE_DOCS_CONTRACT_ARTIFACT_REF,
    STABLE_DOCS_CONTRACT_DOC_REF, STABLE_DOCS_CONTRACT_FIXTURE_DIR,
    STABLE_DOCS_CONTRACT_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StableDocsFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: StableDocsSourceResultPackCitationInput,
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

fn load_fixture(file_name: &str) -> StableDocsFixture {
    let path = repo_root()
        .join(STABLE_DOCS_CONTRACT_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(fixture.record_kind, "stable_docs_contract_case");
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = StableDocsSourceResultPackCitationPacket::materialize(fixture.input);
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
fn doc_fixture_schema_and_artifact_exist_on_disk() {
    assert_exists(STABLE_DOCS_CONTRACT_DOC_REF);
    assert_exists(STABLE_DOCS_CONTRACT_ARTIFACT_DOC_REF);
    assert_exists(STABLE_DOCS_CONTRACT_SCHEMA_REF);
    assert_exists(STABLE_DOCS_CONTRACT_FIXTURE_DIR);
    assert_exists(STABLE_DOCS_CONTRACT_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn source_result_freshness_drift_fixture_blocks_stable() {
    assert_fixture_matches("source_result_freshness_drift_blocks_stable.json");
}

#[test]
fn citation_set_bundles_raw_pack_fixture_blocks_stable() {
    assert_fixture_matches("citation_set_bundles_raw_pack_blocks_stable.json");
}

#[test]
fn pack_detail_sheet_hides_actions_fixture_blocks_stable() {
    assert_fixture_matches("pack_detail_sheet_hides_actions_blocks_stable.json");
}

#[test]
fn citation_drawer_drops_inference_marker_fixture_blocks_stable() {
    assert_fixture_matches("citation_drawer_drops_inference_marker_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_precedence_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_precedence_blocks_stable.json");
}

#[test]
fn checked_in_packet_covers_required_surfaces_and_detail_sheets() {
    let packet = current_stable_docs_source_result_pack_and_citation_packet()
        .expect("stable docs packet validates");
    assert_eq!(packet.promotion_state, StableDocsPromotionState::Stable);
    assert!(packet.validate().is_empty());

    for surface in StableDocsConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} projection",
            surface.as_str()
        );
    }

    let detail_sheet_kinds: BTreeSet<&str> = packet
        .pack_detail_sheets
        .iter()
        .map(|sheet| sheet.sheet_kind.as_str())
        .collect();
    for required in StableDocsPackDetailSheetKind::REQUIRED {
        assert!(
            detail_sheet_kinds.contains(required.as_str()),
            "stable packet must cover {} detail sheet",
            required.as_str()
        );
    }
}

#[test]
fn artifact_file_matches_seeded_packet() {
    let path = repo_root().join(STABLE_DOCS_CONTRACT_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: StableDocsSourceResultPackCitationPacket = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed = StableDocsSourceResultPackCitationPacket::materialize(
        seeded_stable_docs_source_result_pack_and_citation_input(),
    );
    assert_eq!(
        from_file, from_seed,
        "checked-in stable docs packet drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_stable_docs_contract -- packet > artifacts/docs/stable_docs_source_result_pack_and_citation.json`",
    );
}

#[test]
fn support_export_preserves_packet_safely() {
    let packet = current_stable_docs_source_result_pack_and_citation_packet()
        .expect("stable docs packet validates");
    let export = packet.support_export(
        "support-export:stable_docs_contract:test",
        "2026-06-04T16:30:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.export_packet_id_ref, packet.packet_id);
    assert_eq!(export.export_packet, packet);
}

#[test]
fn closed_tokens_are_pinned() {
    assert_eq!(
        StableDocsFindingKind::SourceResultTruthMismatch.as_str(),
        "source_result_truth_mismatch"
    );
    assert_eq!(
        StableDocsFindingKind::CitationDrawerParityDropped.as_str(),
        "citation_drawer_parity_dropped"
    );
    assert_eq!(
        StableDocsFindingKind::SourcePrecedenceCoverageMissing.as_str(),
        "source_precedence_coverage_missing"
    );
}
