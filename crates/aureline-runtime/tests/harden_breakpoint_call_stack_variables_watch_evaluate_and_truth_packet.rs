//! Fixture-driven coverage for the stable debug-fidelity
//! (breakpoint / call-stack / variables / watch / evaluate /
//! debug-console) truth packet covering the local, remote_helper,
//! container, and notebook_bridge debug lanes plus the six-wedge
//! admission coverage, the six inspector-state admissions, the six
//! mapping-fidelity badge admissions, the six inspector-surface
//! bindings, and the lineage_admission row binding
//! `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_debug_fidelity_truth_packet, DebugFidelityConsumerSurface,
    DebugFidelityInspectorStateClass, DebugFidelityInspectorSurfaceClass, DebugFidelityLaneClass,
    DebugFidelityMappingFidelityBadgeClass, DebugFidelityPromotionState, DebugFidelityRowClass,
    DebugFidelitySupportClass, DebugFidelityTruthPacket, DebugFidelityTruthPacketInput,
    DebugFidelityWedgeClass, DEBUG_FIDELITY_TRUTH_ARTIFACT_DOC_REF, DEBUG_FIDELITY_TRUTH_DOC_REF,
    DEBUG_FIDELITY_TRUTH_FIXTURE_DIR, DEBUG_FIDELITY_TRUTH_PACKET_ARTIFACT_REF,
    DEBUG_FIDELITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DebugFidelityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DebugFidelityTruthPacketInput,
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
    inspector_state_tokens: Vec<String>,
    mapping_fidelity_badge_tokens: Vec<String>,
    inspector_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> DebugFidelityFixture {
    let path = repo_root()
        .join(DEBUG_FIDELITY_TRUTH_FIXTURE_DIR)
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
        "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = DebugFidelityTruthPacket::materialize(fixture.input.clone());
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
        &packet.inspector_state_tokens(),
        &expect.inspector_state_tokens,
        "inspector_state",
    );
    assert_token_set_matches(
        &packet.mapping_fidelity_badge_tokens(),
        &expect.mapping_fidelity_badge_tokens,
        "mapping_fidelity_badge",
    );
    assert_token_set_matches(
        &packet.inspector_surface_tokens(),
        &expect.inspector_surface_tokens,
        "inspector_surface",
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
    assert_exists(DEBUG_FIDELITY_TRUTH_SCHEMA_REF);
    assert_exists(DEBUG_FIDELITY_TRUTH_DOC_REF);
    assert_exists(DEBUG_FIDELITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(DEBUG_FIDELITY_TRUTH_FIXTURE_DIR);
    assert_exists(DEBUG_FIDELITY_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_inspector_state_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_inspector_state_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_mapping_fidelity_badge_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_mapping_fidelity_badge_for_launch_stable_blocks_stable.json");
}

#[test]
fn inspector_surface_missing_state_attestation_blocks_stable() {
    assert_fixture_matches("inspector_surface_missing_state_attestation_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_inspector_state_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_inspector_state_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_debug_fidelity_truth_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, DebugFidelityPromotionState::Stable);
    assert!(packet.validate().is_empty());
    for required in DebugFidelityLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for debug-fidelity lane {}",
            required.as_str()
        );
    }
    for surface in DebugFidelityConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_wedges_states_badges_and_surfaces_per_launch_stable_lane() {
    let packet = current_stable_debug_fidelity_truth_packet().expect("checked-in packet validates");
    for required in DebugFidelityLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == DebugFidelityRowClass::DebugFidelityQuality
                && row.support_class == DebugFidelitySupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in DebugFidelityWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == DebugFidelityRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for state in DebugFidelityInspectorStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == DebugFidelityRowClass::InspectorStateAdmission
                    && row.inspector_state_class == state),
                "stable packet must cover the {} inspector state on the {} lane",
                state.as_str(),
                required.as_str()
            );
        }
        for badge in DebugFidelityMappingFidelityBadgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == DebugFidelityRowClass::MappingFidelityBadgeAdmission
                    && row.mapping_fidelity_badge_class == badge),
                "stable packet must cover the {} mapping-fidelity badge on the {} lane",
                badge.as_str(),
                required.as_str()
            );
        }
        for surface in DebugFidelityInspectorSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == DebugFidelityRowClass::InspectorSurfaceBinding
                    && row.inspector_surface_class == surface
                    && (!surface.requires_inspector_state_attestation()
                        || row.attests_inspector_state_preserved)
                    && (!surface.requires_mapping_fidelity_attestation()
                        || row.attests_mapping_fidelity_preserved)),
                "stable packet must cover the {} inspector surface on the {} lane with required attestations",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == DebugFidelityRowClass::LineageAdmission
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
fn closed_debug_fidelity_truth_tokens_are_pinned() {
    assert_eq!(DebugFidelityLaneClass::LocalLane.as_str(), "local_lane");
    assert_eq!(
        DebugFidelityLaneClass::NotebookBridgeLane.as_str(),
        "notebook_bridge_lane"
    );
    assert_eq!(
        DebugFidelityRowClass::DebugFidelityQuality.as_str(),
        "debug_fidelity_quality"
    );
    assert_eq!(
        DebugFidelitySupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        DebugFidelityWedgeClass::BreakpointFidelity.as_str(),
        "breakpoint_fidelity"
    );
    assert_eq!(
        DebugFidelityWedgeClass::DebugConsoleFidelity.as_str(),
        "debug_console_fidelity"
    );
    assert_eq!(DebugFidelityInspectorStateClass::Live.as_str(), "live");
    assert_eq!(
        DebugFidelityInspectorStateClass::PolicyBlocked.as_str(),
        "policy_blocked"
    );
    assert_eq!(
        DebugFidelityMappingFidelityBadgeClass::Exact.as_str(),
        "exact"
    );
    assert_eq!(
        DebugFidelityMappingFidelityBadgeClass::Mismatched.as_str(),
        "mismatched"
    );
    assert_eq!(
        DebugFidelityInspectorSurfaceClass::WatchSurface.as_str(),
        "watch_surface"
    );
    assert_eq!(
        DebugFidelityConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
}
