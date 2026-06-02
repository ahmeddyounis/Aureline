//! Fixture-driven coverage for the stable support-export-parity truth
//! packet covering the terminal, task, test, and debug lanes plus the
//! four-wedge admission coverage, the five export-field bindings, the
//! four diagnosis-packet bindings, the four repair-hook bindings, the
//! five recovery-posture admissions, and the lineage_admission row
//! binding `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_terminal::{
    current_stable_support_export_parity_truth_packet, DiagnosisPacketFieldClass, ExportFieldClass,
    RecoveryPostureClass, RepairHookFieldClass, SupportExportParityConsumerSurface,
    SupportExportParityLaneClass, SupportExportParityPromotionState, SupportExportParityRowClass,
    SupportExportParitySupportClass, SupportExportParityTruthPacket,
    SupportExportParityTruthPacketInput, SupportExportParityWedgeClass,
    SUPPORT_EXPORT_PARITY_TRUTH_ARTIFACT_DOC_REF, SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF,
    SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR, SUPPORT_EXPORT_PARITY_TRUTH_PACKET_ARTIFACT_REF,
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
    #[serde(default)]
    row_count: usize,
    #[serde(default)]
    lane_tokens: Vec<String>,
    #[serde(default)]
    row_class_tokens: Vec<String>,
    #[serde(default)]
    support_class_tokens: Vec<String>,
    #[serde(default)]
    wedge_tokens: Vec<String>,
    #[serde(default)]
    export_field_tokens: Vec<String>,
    #[serde(default)]
    diagnosis_packet_field_tokens: Vec<String>,
    #[serde(default)]
    repair_hook_field_tokens: Vec<String>,
    #[serde(default)]
    recovery_posture_tokens: Vec<String>,
    #[serde(default)]
    known_limit_tokens: Vec<String>,
    #[serde(default)]
    downgrade_automation_tokens: Vec<String>,
    #[serde(default)]
    evidence_class_tokens: Vec<String>,
    #[serde(default)]
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
        "finalize_terminal_task_test_and_debug_support_export_parity_truth_stable_case",
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
        &packet.export_field_tokens(),
        &expect.export_field_tokens,
        "export_field",
    );
    assert_token_set_matches(
        &packet.diagnosis_packet_field_tokens(),
        &expect.diagnosis_packet_field_tokens,
        "diagnosis_packet_field",
    );
    assert_token_set_matches(
        &packet.repair_hook_field_tokens(),
        &expect.repair_hook_field_tokens,
        "repair_hook_field",
    );
    assert_token_set_matches(
        &packet.recovery_posture_tokens(),
        &expect.recovery_posture_tokens,
        "recovery_posture",
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
        "2026-05-27T12:00:10Z",
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
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn launch_stable_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("launch_stable_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_export_field_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_export_field_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_diagnosis_packet_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_diagnosis_packet_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_repair_hook_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_repair_hook_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_recovery_posture_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_recovery_posture_for_launch_stable_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_diagnosis_packet_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_diagnosis_packet_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet =
        current_stable_support_export_parity_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        SupportExportParityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in SupportExportParityLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for support-export-parity lane {}",
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
fn checked_in_artifact_covers_required_wedges_fields_and_postures_per_launch_stable_lane() {
    let packet =
        current_stable_support_export_parity_truth_packet().expect("checked-in packet validates");
    for required in SupportExportParityLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == SupportExportParityRowClass::SupportExportParityQuality
                && row.support_class == SupportExportParitySupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in SupportExportParityWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == SupportExportParityRowClass::WedgeAdmission
                    && row.wedge_class == wedge
                    && row.support_class == SupportExportParitySupportClass::LaunchStable),
                "stable packet must cover wedge {} for lane {}",
                wedge.as_str(),
                required.as_str()
            );
        }
        for field in ExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == SupportExportParityRowClass::ExportFieldBinding
                    && row.export_field_class == field
                    && row.support_class == SupportExportParitySupportClass::LaunchStable),
                "stable packet must cover export field {} for lane {}",
                field.as_str(),
                required.as_str()
            );
        }
        for field in DiagnosisPacketFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == SupportExportParityRowClass::DiagnosisPacketBinding
                    && row.diagnosis_packet_field_class == field
                    && row.support_class == SupportExportParitySupportClass::LaunchStable),
                "stable packet must cover diagnosis-packet field {} for lane {}",
                field.as_str(),
                required.as_str()
            );
        }
        for field in RepairHookFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == SupportExportParityRowClass::RepairHookBinding
                    && row.repair_hook_field_class == field
                    && row.support_class == SupportExportParitySupportClass::LaunchStable),
                "stable packet must cover repair-hook field {} for lane {}",
                field.as_str(),
                required.as_str()
            );
        }
        for posture in RecoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == SupportExportParityRowClass::RecoveryPostureAdmission
                    && row.recovery_posture_class == posture
                    && row.support_class == SupportExportParitySupportClass::LaunchStable),
                "stable packet must cover recovery posture {} for lane {}",
                posture.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == SupportExportParityRowClass::LineageAdmission
                && row.support_class == SupportExportParitySupportClass::LaunchStable),
            "stable packet must cover lineage admission for lane {}",
            required.as_str()
        );
    }
}
