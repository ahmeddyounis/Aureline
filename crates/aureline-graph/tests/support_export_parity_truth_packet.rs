//! Fixture-driven coverage for the stable support-export parity,
//! query-session/search-export, retrieval-debug, and operator-truth
//! inspector truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_support_export_parity_truth_packet, SupportExportParityConsumerSurface,
    SupportExportParityFindingKind, SupportExportParityLaneClass,
    SupportExportParityPromotionState, SupportExportParityTruthPacket,
    SupportExportParityTruthPacketInput, SUPPORT_EXPORT_PARITY_TRUTH_ARTIFACT_DOC_REF,
    SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF, SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR,
    SUPPORT_EXPORT_PARITY_TRUTH_PACKET_ARTIFACT_REF,
    SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SupportExportParityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SupportExportParityTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    lane_tokens: Vec<String>,
    export_packet_class_tokens: Vec<String>,
    redaction_tokens: Vec<String>,
    live_vs_captured_tokens: Vec<String>,
    downgrade_tokens: Vec<String>,
    support_export_safe: bool,
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

fn load_fixture(file_name: &str) -> SupportExportParityFixture {
    let path = repo_root()
        .join(SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "support_export_parity_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = SupportExportParityTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.lane_tokens(),
        expect
            .lane_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} lane tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.export_packet_class_tokens(),
        expect
            .export_packet_class_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} export-packet-class tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.redaction_tokens(),
        expect
            .redaction_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} redaction tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.live_vs_captured_tokens(),
        expect
            .live_vs_captured_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} live-vs-captured tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.downgrade_tokens(),
        expect
            .downgrade_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} downgrade tokens drifted",
        fixture.case_name
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_REF);
    assert_exists(SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF);
    assert_exists(SUPPORT_EXPORT_PARITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR);
    assert_exists(SUPPORT_EXPORT_PARITY_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn raw_query_text_leak_fixture_blocks_stable() {
    assert_fixture_matches("raw_query_text_leak_blocks_stable.json");
}

#[test]
fn projection_drops_redaction_fixture_blocks_stable() {
    assert_fixture_matches("projection_drops_redaction_blocks_stable.json");
}

#[test]
fn operator_truth_missing_reconstruction_fixture_blocks_stable() {
    assert_fixture_matches("operator_truth_missing_reconstruction_blocks_stable.json");
}

#[test]
fn deep_link_freezes_certainty_fixture_blocks_stable() {
    assert_fixture_matches("deep_link_freezes_certainty_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_support_export_parity_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        SupportExportParityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in SupportExportParityLaneClass::REQUIRED {
        assert!(
            packet
                .rows
                .iter()
                .any(|row| row.lane_class == required),
            "stable packet must include row for lane class {}",
            required.as_str()
        );
    }
    for surface in SupportExportParityConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_support_export_parity_tokens_are_pinned() {
    assert_eq!(
        SupportExportParityLaneClass::SearchExport.as_str(),
        "search_export"
    );
    assert_eq!(
        SupportExportParityLaneClass::OperatorTruthInspector.as_str(),
        "operator_truth_inspector"
    );
    assert_eq!(
        SupportExportParityFindingKind::OperatorTruthMissingReconstruction.as_str(),
        "operator_truth_missing_reconstruction"
    );
    assert_eq!(
        SupportExportParityFindingKind::DeepLinkFreezesRecipientCertainty.as_str(),
        "deep_link_freezes_recipient_certainty"
    );
    assert_eq!(
        SupportExportParityPromotionState::BlocksStable.as_str(),
        "blocks_stable"
    );
}
