//! Fixture-driven coverage for the stable integrated-terminal
//! stabilization truth packet covering the local, remote/helper,
//! container, and restored terminal-session lanes plus the four-wedge
//! admission coverage (host_boundary_chip, clipboard_posture,
//! transcript_export, restore_no_rerun), the five host-boundary
//! field bindings, the five clipboard-posture surface bindings, the
//! three transcript-export field bindings, the
//! restore_no_rerun_attestation row, and the lineage_admission row
//! binding `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_terminal::{
    current_stable_terminal_stabilization_truth_packet,
    TerminalStabilizationClipboardPostureClass, TerminalStabilizationConsumerSurface,
    TerminalStabilizationDowngradeAutomationClass, TerminalStabilizationEvidenceClass,
    TerminalStabilizationFindingKind, TerminalStabilizationHostBoundaryFieldClass,
    TerminalStabilizationKnownLimitClass, TerminalStabilizationLaneClass,
    TerminalStabilizationPromotionState, TerminalStabilizationRowClass,
    TerminalStabilizationSupportClass, TerminalStabilizationTranscriptExportFieldClass,
    TerminalStabilizationTruthPacket, TerminalStabilizationTruthPacketInput,
    TerminalStabilizationWedgeClass, TERMINAL_STABILIZATION_TRUTH_ARTIFACT_DOC_REF,
    TERMINAL_STABILIZATION_TRUTH_DOC_REF, TERMINAL_STABILIZATION_TRUTH_FIXTURE_DIR,
    TERMINAL_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF, TERMINAL_STABILIZATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TerminalStabilizationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: TerminalStabilizationTruthPacketInput,
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
    host_boundary_field_tokens: Vec<String>,
    clipboard_posture_tokens: Vec<String>,
    transcript_export_field_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> TerminalStabilizationFixture {
    let path = repo_root()
        .join(TERMINAL_STABILIZATION_TRUTH_FIXTURE_DIR)
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
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = TerminalStabilizationTruthPacket::materialize(fixture.input.clone());
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
        &packet.host_boundary_field_tokens(),
        &expect.host_boundary_field_tokens,
        "host_boundary_field",
    );
    assert_token_set_matches(
        &packet.clipboard_posture_tokens(),
        &expect.clipboard_posture_tokens,
        "clipboard_posture",
    );
    assert_token_set_matches(
        &packet.transcript_export_field_tokens(),
        &expect.transcript_export_field_tokens,
        "transcript_export_field",
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
    assert_exists(TERMINAL_STABILIZATION_TRUTH_SCHEMA_REF);
    assert_exists(TERMINAL_STABILIZATION_TRUTH_DOC_REF);
    assert_exists(TERMINAL_STABILIZATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(TERMINAL_STABILIZATION_TRUTH_FIXTURE_DIR);
    assert_exists(TERMINAL_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_clipboard_posture_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_clipboard_posture_for_launch_stable_blocks_stable.json");
}

#[test]
fn restore_admits_silent_rerun_blocks_stable() {
    assert_fixture_matches("restore_admits_silent_rerun_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_clipboard_posture_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_clipboard_posture_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_terminal_stabilization_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        TerminalStabilizationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in TerminalStabilizationLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for terminal-stabilization lane {}",
            required.as_str()
        );
    }
    for surface in TerminalStabilizationConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_wedges_fields_postures_and_admissions_per_launch_stable_lane(
) {
    let packet = current_stable_terminal_stabilization_truth_packet()
        .expect("checked-in packet validates");
    for required in TerminalStabilizationLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == TerminalStabilizationRowClass::TerminalStabilizationQuality
                && row.support_class == TerminalStabilizationSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in TerminalStabilizationWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == TerminalStabilizationRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for field in TerminalStabilizationHostBoundaryFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TerminalStabilizationRowClass::HostBoundaryFieldBinding
                    && row.host_boundary_field_class == field),
                "stable packet must cover the {} host-boundary field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        for surface in TerminalStabilizationClipboardPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TerminalStabilizationRowClass::ClipboardPostureBinding
                    && row.clipboard_posture_class == surface),
                "stable packet must cover the {} clipboard-posture surface on the {} lane",
                surface.as_str(),
                required.as_str()
            );
        }
        for field in TerminalStabilizationTranscriptExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == TerminalStabilizationRowClass::TranscriptExportFieldBinding
                    && row.transcript_export_field_class == field),
                "stable packet must cover the {} transcript-export field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class
                    == TerminalStabilizationRowClass::RestoreNoRerunAttestation
                && row.attests_no_silent_rerun),
            "stable packet must include a restore_no_rerun_attestation row attesting no_silent_rerun on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == TerminalStabilizationRowClass::LineageAdmission
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
fn closed_terminal_stabilization_tokens_are_pinned() {
    assert_eq!(
        TerminalStabilizationLaneClass::LocalLane.as_str(),
        "local_lane"
    );
    assert_eq!(
        TerminalStabilizationLaneClass::RemoteHelperLane.as_str(),
        "remote_helper_lane"
    );
    assert_eq!(
        TerminalStabilizationLaneClass::ContainerLane.as_str(),
        "container_lane"
    );
    assert_eq!(
        TerminalStabilizationLaneClass::RestoredLane.as_str(),
        "restored_lane"
    );
    assert_eq!(
        TerminalStabilizationRowClass::TerminalStabilizationQuality.as_str(),
        "terminal_stabilization_quality"
    );
    assert_eq!(
        TerminalStabilizationSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        TerminalStabilizationSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        TerminalStabilizationWedgeClass::HostBoundaryChip.as_str(),
        "host_boundary_chip"
    );
    assert_eq!(
        TerminalStabilizationWedgeClass::RestoreNoRerun.as_str(),
        "restore_no_rerun"
    );
    assert_eq!(
        TerminalStabilizationHostBoundaryFieldClass::RouteCue.as_str(),
        "route_cue"
    );
    assert_eq!(
        TerminalStabilizationClipboardPostureClass::AdminSuppression.as_str(),
        "admin_suppression"
    );
    assert_eq!(
        TerminalStabilizationTranscriptExportFieldClass::RedactionState.as_str(),
        "redaction_state"
    );
    assert_eq!(
        TerminalStabilizationEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        TerminalStabilizationKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        TerminalStabilizationDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        TerminalStabilizationConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        TerminalStabilizationFindingKind::RestoreNoRerunAttestationAdmitsSilentRerun.as_str(),
        "restore_no_rerun_attestation_admits_silent_rerun"
    );
    assert_eq!(
        TerminalStabilizationFindingKind::LineageAdmissionMissingExecutionContextId.as_str(),
        "lineage_admission_missing_execution_context_id"
    );
}
