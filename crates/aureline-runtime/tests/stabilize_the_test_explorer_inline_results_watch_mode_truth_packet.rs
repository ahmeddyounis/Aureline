//! Fixture-driven coverage for the stable
//! stabilize-the-test-explorer / inline-results / watch-mode / rerun /
//! debug-from-test truth packet covering the local, remote_helper,
//! container, and notebook test-explorer lanes plus the four-wedge
//! admission coverage, the four test-identity admissions, the three
//! discovery-posture admissions, the four watch-mode-support
//! admissions, the three selector-durability admissions, the five
//! consumer-surface bindings, and the lineage_admission row binding
//! `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_test_explorer_stabilization_truth_packet,
    TestExplorerStabilizationConsumerSurface, TestExplorerStabilizationConsumerSurfaceBindingClass,
    TestExplorerStabilizationDiscoveryPostureClass, TestExplorerStabilizationLaneClass,
    TestExplorerStabilizationPromotionState, TestExplorerStabilizationRowClass,
    TestExplorerStabilizationSelectorDurabilityClass, TestExplorerStabilizationSupportClass,
    TestExplorerStabilizationTestIdentityClass, TestExplorerStabilizationTruthPacket,
    TestExplorerStabilizationTruthPacketInput, TestExplorerStabilizationWatchModeSupportClass,
    TestExplorerStabilizationWedgeClass, TEST_EXPLORER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_DOC_REF, TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR,
    TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestExplorerFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: TestExplorerStabilizationTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    lane_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    wedge_tokens: Vec<String>,
    test_identity_tokens: Vec<String>,
    discovery_posture_tokens: Vec<String>,
    watch_mode_support_tokens: Vec<String>,
    selector_durability_tokens: Vec<String>,
    consumer_surface_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
    evidence_class_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> TestExplorerFixture {
    let path = repo_root()
        .join(TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind,
        "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = TestExplorerStabilizationTruthPacket::materialize(fixture.input.clone());
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
    assert_token_set_matches(&packet.lane_tokens(), &expect.lane_tokens, "lane");
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.support_class_tokens(),
        &expect.support_class_tokens,
        "support_class",
    );
    assert_token_set_matches(&packet.wedge_tokens(), &expect.wedge_tokens, "wedge");
    assert_token_set_matches(
        &packet.test_identity_tokens(),
        &expect.test_identity_tokens,
        "test_identity",
    );
    assert_token_set_matches(
        &packet.discovery_posture_tokens(),
        &expect.discovery_posture_tokens,
        "discovery_posture",
    );
    assert_token_set_matches(
        &packet.watch_mode_support_tokens(),
        &expect.watch_mode_support_tokens,
        "watch_mode_support",
    );
    assert_token_set_matches(
        &packet.selector_durability_tokens(),
        &expect.selector_durability_tokens,
        "selector_durability",
    );
    assert_token_set_matches(
        &packet.consumer_surface_tokens(),
        &expect.consumer_surface_tokens,
        "consumer_surface",
    );
    assert_token_set_matches(
        &packet.known_limit_tokens(),
        &expect.known_limit_tokens,
        "known_limit",
    );
    assert_token_set_matches(
        &packet.downgrade_automation_tokens(),
        &expect.downgrade_automation_tokens,
        "downgrade_automation",
    );
    assert_token_set_matches(
        &packet.evidence_class_tokens(),
        &expect.evidence_class_tokens,
        "evidence_class",
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
    assert_exists(TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_REF);
    assert_exists(TEST_EXPLORER_STABILIZATION_TRUTH_DOC_REF);
    assert_exists(TEST_EXPLORER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR);
    assert_exists(TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn launch_stable_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("launch_stable_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_watch_mode_support_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_watch_mode_support_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_selector_durability_for_launch_stable_blocks_stable() {
    assert_fixture_matches(
        "missing_selector_durability_for_launch_stable_blocks_stable.json",
    );
}

#[test]
fn consumer_surface_missing_durable_selector_attestation_blocks_stable() {
    assert_fixture_matches(
        "consumer_surface_missing_durable_selector_attestation_blocks_stable.json",
    );
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_selector_durability_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_selector_durability_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_test_explorer_stabilization_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        TestExplorerStabilizationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in TestExplorerStabilizationLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for test-explorer lane {}",
            required.as_str()
        );
    }
    for surface in TestExplorerStabilizationConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_admissions_per_launch_stable_lane() {
    let packet = current_stable_test_explorer_stabilization_truth_packet()
        .expect("checked-in packet validates");
    for required in TestExplorerStabilizationLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class
                    == TestExplorerStabilizationRowClass::TestExplorerStabilizationQuality
                && row.support_class == TestExplorerStabilizationSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in TestExplorerStabilizationWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == TestExplorerStabilizationRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for identity in TestExplorerStabilizationTestIdentityClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == TestExplorerStabilizationRowClass::TestIdentityAdmission
                    && row.test_identity_class == identity),
                "stable packet must cover the {} test-identity admission on the {} lane",
                identity.as_str(),
                required.as_str()
            );
        }
        for posture in TestExplorerStabilizationDiscoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE
        {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TestExplorerStabilizationRowClass::DiscoveryPostureAdmission
                    && row.discovery_posture_class == posture),
                "stable packet must cover the {} discovery-posture admission on the {} lane",
                posture.as_str(),
                required.as_str()
            );
        }
        for support in
            TestExplorerStabilizationWatchModeSupportClass::REQUIRED_FOR_LAUNCH_STABLE
        {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TestExplorerStabilizationRowClass::WatchModeSupportAdmission
                    && row.watch_mode_support_class == support),
                "stable packet must cover the {} watch-mode-support admission on the {} lane",
                support.as_str(),
                required.as_str()
            );
        }
        for durability in
            TestExplorerStabilizationSelectorDurabilityClass::REQUIRED_FOR_LAUNCH_STABLE
        {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TestExplorerStabilizationRowClass::SelectorDurabilityAdmission
                    && row.selector_durability_class == durability),
                "stable packet must cover the {} selector-durability admission on the {} lane",
                durability.as_str(),
                required.as_str()
            );
        }
        for surface in
            TestExplorerStabilizationConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE
        {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TestExplorerStabilizationRowClass::ConsumerSurfaceBinding
                    && row.consumer_surface_class == surface
                    && (!surface.requires_test_identity_attestation()
                        || row.attests_test_identity_preserved)
                    && (!surface.requires_watch_mode_support_attestation()
                        || row.attests_watch_mode_support_preserved)
                    && (!surface.requires_durable_selector_attestation()
                        || row.attests_durable_selector_preserved)),
                "stable packet must cover the {} consumer surface binding on the {} lane with required attestations",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == TestExplorerStabilizationRowClass::LineageAdmission
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            }),
            "stable packet must include a lineage_admission row binding execution_context_id on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_test_explorer_stabilization_truth_tokens_are_pinned() {
    assert_eq!(
        TestExplorerStabilizationLaneClass::LocalLane.as_str(),
        "local_lane"
    );
    assert_eq!(
        TestExplorerStabilizationLaneClass::NotebookLane.as_str(),
        "notebook_lane"
    );
    assert_eq!(
        TestExplorerStabilizationRowClass::TestExplorerStabilizationQuality.as_str(),
        "test_explorer_stabilization_quality"
    );
    assert_eq!(
        TestExplorerStabilizationSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        TestExplorerStabilizationWedgeClass::TestExplorerIdentityTruth.as_str(),
        "test_explorer_identity_truth"
    );
    assert_eq!(
        TestExplorerStabilizationWedgeClass::RerunDebugFromTestParity.as_str(),
        "rerun_debug_from_test_parity"
    );
    assert_eq!(
        TestExplorerStabilizationTestIdentityClass::CaseIdentity.as_str(),
        "case_identity"
    );
    assert_eq!(
        TestExplorerStabilizationTestIdentityClass::InvocationIdentity.as_str(),
        "invocation_identity"
    );
    assert_eq!(
        TestExplorerStabilizationWatchModeSupportClass::Polling.as_str(),
        "polling"
    );
    assert_eq!(
        TestExplorerStabilizationSelectorDurabilityClass::SnapshotScopedQuerySelector.as_str(),
        "snapshot_scoped_query_selector"
    );
    assert_eq!(
        TestExplorerStabilizationConsumerSurfaceBindingClass::RerunSurface.as_str(),
        "rerun_surface"
    );
    assert_eq!(
        TestExplorerStabilizationConsumerSurface::AiToolSurface.as_str(),
        "ai_tool_surface"
    );
}
