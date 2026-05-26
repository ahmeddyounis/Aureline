//! Fixture-driven coverage for the stable docs/code semantic-recall
//! boundary truth packet shared by the M4 stable lane and the v1.x
//! preview lane.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_semantic_recall_boundary_truth_packet, SemanticRecallBoundaryConsumerSurface,
    SemanticRecallBoundaryFindingKind, SemanticRecallBoundaryLaneClass,
    SemanticRecallBoundaryPromotionState, SemanticRecallBoundarySurfaceTrack,
    SemanticRecallBoundaryTruthPacket, SemanticRecallBoundaryTruthPacketInput,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_ARTIFACT_DOC_REF, SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_FIXTURE_DIR, SEMANTIC_RECALL_BOUNDARY_TRUTH_MILESTONE_DOC_REF,
    SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_ARTIFACT_REF, SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SemanticRecallBoundaryFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SemanticRecallBoundaryTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    lane_tokens: Vec<String>,
    surface_track_tokens: Vec<String>,
    locality_tokens: Vec<String>,
    epoch_state_tokens: Vec<String>,
    pack_signature_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> SemanticRecallBoundaryFixture {
    let path = repo_root()
        .join(SEMANTIC_RECALL_BOUNDARY_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "semantic_recall_boundary_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = SemanticRecallBoundaryTruthPacket::materialize(fixture.input.clone());
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
        packet.surface_track_tokens(),
        expect
            .surface_track_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} surface-track tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.locality_tokens(),
        expect
            .locality_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} locality tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.epoch_state_tokens(),
        expect
            .epoch_state_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} epoch-state tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.pack_signature_tokens(),
        expect
            .pack_signature_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} pack-signature tokens drifted",
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
    assert_exists(SEMANTIC_RECALL_BOUNDARY_TRUTH_SCHEMA_REF);
    assert_exists(SEMANTIC_RECALL_BOUNDARY_TRUTH_DOC_REF);
    assert_exists(SEMANTIC_RECALL_BOUNDARY_TRUTH_MILESTONE_DOC_REF);
    assert_exists(SEMANTIC_RECALL_BOUNDARY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(SEMANTIC_RECALL_BOUNDARY_TRUTH_FIXTURE_DIR);
    assert_exists(SEMANTIC_RECALL_BOUNDARY_TRUTH_PACKET_ARTIFACT_REF);
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
fn unsigned_mirrored_pack_fixture_blocks_stable() {
    assert_fixture_matches("unsigned_mirrored_pack_blocks_stable.json");
}

#[test]
fn mixed_generation_recall_fixture_blocks_stable() {
    assert_fixture_matches("mixed_generation_recall_blocks_stable.json");
}

#[test]
fn policy_omissions_undisclosed_fixture_blocks_stable() {
    assert_fixture_matches("policy_omissions_undisclosed_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane_and_track() {
    let packet = current_stable_semantic_recall_boundary_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        SemanticRecallBoundaryPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required_lane in SemanticRecallBoundaryLaneClass::REQUIRED {
        assert!(
            packet
                .rows
                .iter()
                .any(|row| row.recall_lane_class == required_lane),
            "stable packet must include row for lane class {}",
            required_lane.as_str()
        );
    }
    for required_track in SemanticRecallBoundarySurfaceTrack::REQUIRED {
        assert!(
            packet
                .rows
                .iter()
                .any(|row| row.surface_track == required_track),
            "stable packet must include row for surface track {}",
            required_track.as_str()
        );
    }
    for surface in SemanticRecallBoundaryConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_semantic_recall_boundary_tokens_are_pinned() {
    assert_eq!(
        SemanticRecallBoundaryLaneClass::DocsSemanticRecall.as_str(),
        "docs_semantic_recall"
    );
    assert_eq!(
        SemanticRecallBoundarySurfaceTrack::V1xPreview.as_str(),
        "v1x_preview"
    );
    assert_eq!(
        SemanticRecallBoundaryFindingKind::UnlabeledManagedOrVectorMatch.as_str(),
        "unlabeled_managed_or_vector_match"
    );
    assert_eq!(
        SemanticRecallBoundaryFindingKind::EpochMismatchPresentedAsCurrent.as_str(),
        "epoch_mismatch_presented_as_current"
    );
    assert_eq!(
        SemanticRecallBoundaryPromotionState::BlocksStable.as_str(),
        "blocks_stable"
    );
}
