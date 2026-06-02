//! Fixture-driven coverage for the stable monorepo hot-set indexing,
//! warming-state, and graceful-degradation truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_monorepo_hot_set_truth_packet, GracefulDegradationClass, IndexingLaneClass,
    MonorepoArchetypeClass, MonorepoConsumerSurface, MonorepoHotSetTruthPacket,
    MonorepoHotSetTruthPacketInput, MonorepoTruthFindingKind, MonorepoTruthPromotionState,
    SearchReadinessState, MONOREPO_HOT_SET_TRUTH_ARTIFACT_DOC_REF, MONOREPO_HOT_SET_TRUTH_DOC_REF,
    MONOREPO_HOT_SET_TRUTH_FIXTURE_DIR, MONOREPO_HOT_SET_TRUTH_PACKET_ARTIFACT_REF,
    MONOREPO_HOT_SET_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MonorepoTruthFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: MonorepoHotSetTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    archetype_tokens: Vec<String>,
    lane_tokens: Vec<String>,
    degradation_tokens: Vec<String>,
    readiness_state_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> MonorepoTruthFixture {
    let path = repo_root()
        .join(MONOREPO_HOT_SET_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "monorepo_hot_set_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = MonorepoHotSetTruthPacket::materialize(fixture.input.clone());
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
        packet.archetype_tokens(),
        expect
            .archetype_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.lane_tokens(),
        expect
            .lane_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.degradation_tokens(),
        expect
            .degradation_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.readiness_state_tokens(),
        expect
            .readiness_state_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
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
    assert_exists(MONOREPO_HOT_SET_TRUTH_SCHEMA_REF);
    assert_exists(MONOREPO_HOT_SET_TRUTH_DOC_REF);
    assert_exists(MONOREPO_HOT_SET_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(MONOREPO_HOT_SET_TRUTH_FIXTURE_DIR);
    assert_exists(MONOREPO_HOT_SET_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn degradation_unlabeled_fixture_blocks_stable() {
    assert_fixture_matches("degradation_unlabeled_blocks_stable.json");
}

#[test]
fn missing_warming_transition_fixture_blocks_stable() {
    assert_fixture_matches("missing_warming_transition_blocks_stable.json");
}

#[test]
fn edit_input_blocked_fixture_blocks_stable() {
    assert_fixture_matches("edit_input_blocked_blocks_stable.json");
}

#[test]
fn projection_drops_degradation_fixture_blocks_stable() {
    assert_fixture_matches("projection_drops_degradation_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_certifies_every_archetype_lane() {
    let packet =
        current_stable_monorepo_hot_set_truth_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, MonorepoTruthPromotionState::Stable);
    assert!(packet.validate().is_empty());

    assert_eq!(
        packet.archetype_tokens().len(),
        MonorepoArchetypeClass::ALL.len()
    );
    assert_eq!(packet.lane_tokens().len(), IndexingLaneClass::ALL.len());
    for archetype in MonorepoArchetypeClass::ALL {
        for lane in IndexingLaneClass::ALL {
            assert!(
                packet
                    .rows
                    .iter()
                    .any(|row| row.archetype == archetype && row.lane == lane),
                "certified archetype {} × lane {} must have a row in the stable packet",
                archetype.as_str(),
                lane.as_str()
            );
        }
    }
    for surface in MonorepoConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_monorepo_truth_tokens_are_pinned() {
    assert_eq!(
        MonorepoArchetypeClass::VeryLargeMonorepo.as_str(),
        "very_large_monorepo"
    );
    assert_eq!(IndexingLaneClass::TextIndex.as_str(), "text_index");
    assert_eq!(
        GracefulDegradationClass::IndexUnavailableDisclosed.as_str(),
        "index_unavailable_disclosed"
    );
    assert_eq!(
        GracefulDegradationClass::PausedForResourcePressure.as_str(),
        "paused_for_resource_pressure"
    );
    assert_eq!(
        MonorepoTruthFindingKind::FirstUsefulRowBudgetMissed.as_str(),
        "first_useful_row_budget_missed"
    );
    assert_eq!(SearchReadinessState::HotSetReady.as_str(), "hot_set_ready");
}
