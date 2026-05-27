//! Fixture-driven coverage for the stable environment + toolchain
//! manager and execution-context inspector parity truth packet covering
//! the local, remote_helper, container, and managed execution lanes
//! plus the eight inspector-field admissions, the four parity-surface
//! bindings, the five recovery admissions, the toolchain-manager
//! admission binding a manager id, and the lineage_admission row
//! binding `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_inspector_parity_truth_packet, InspectorFieldClass,
    InspectorParityConsumerSurface, InspectorParityDowngradeAutomationClass,
    InspectorParityEvidenceClass, InspectorParityFindingKind, InspectorParityKnownLimitClass,
    InspectorParityLaneClass, InspectorParityPromotionState, InspectorParityRowClass,
    InspectorParitySupportClass, InspectorParityTruthPacket, InspectorParityTruthPacketInput,
    ParitySurfaceClass, RecoveryStateClass, INSPECTOR_PARITY_TRUTH_ARTIFACT_DOC_REF,
    INSPECTOR_PARITY_TRUTH_DOC_REF, INSPECTOR_PARITY_TRUTH_FIXTURE_DIR,
    INSPECTOR_PARITY_TRUTH_PACKET_ARTIFACT_REF, INSPECTOR_PARITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InspectorParityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: InspectorParityTruthPacketInput,
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
    inspector_field_tokens: Vec<String>,
    parity_surface_tokens: Vec<String>,
    recovery_state_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> InspectorParityFixture {
    let path = repo_root()
        .join(INSPECTOR_PARITY_TRUTH_FIXTURE_DIR)
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
        "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = InspectorParityTruthPacket::materialize(fixture.input.clone());
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
    assert_token_set_matches(
        &packet.inspector_field_tokens(),
        &expect.inspector_field_tokens,
        "inspector_field",
    );
    assert_token_set_matches(
        &packet.parity_surface_tokens(),
        &expect.parity_surface_tokens,
        "parity_surface",
    );
    assert_token_set_matches(
        &packet.recovery_state_tokens(),
        &expect.recovery_state_tokens,
        "recovery_state",
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
    assert_exists(INSPECTOR_PARITY_TRUTH_SCHEMA_REF);
    assert_exists(INSPECTOR_PARITY_TRUTH_DOC_REF);
    assert_exists(INSPECTOR_PARITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(INSPECTOR_PARITY_TRUTH_FIXTURE_DIR);
    assert_exists(INSPECTOR_PARITY_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_inspector_field_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_inspector_field_for_launch_stable_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_parity_surface_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_parity_surface_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_inspector_parity_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, InspectorParityPromotionState::Stable);
    assert!(packet.validate().is_empty());
    for required in InspectorParityLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for execution-context lane {}",
            required.as_str()
        );
    }
    for surface in InspectorParityConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_fields_surfaces_and_admissions_per_launch_stable_lane() {
    let packet = current_stable_inspector_parity_truth_packet()
        .expect("checked-in packet validates");
    for required in InspectorParityLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == InspectorParityRowClass::InspectorParityQuality
                && row.support_class == InspectorParitySupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for field in InspectorFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == InspectorParityRowClass::InspectorFieldAdmission
                    && row.inspector_field_class == field),
                "stable packet must cover the {} inspector field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        for surface in ParitySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == InspectorParityRowClass::ParitySurfaceBinding
                    && row.parity_surface_class == surface),
                "stable packet must cover the {} parity surface on the {} lane",
                surface.as_str(),
                required.as_str()
            );
        }
        for state in RecoveryStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == InspectorParityRowClass::RecoveryAdmission
                    && row.recovery_state_class == state),
                "stable packet must cover the {} recovery admission on the {} lane",
                state.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == InspectorParityRowClass::ToolchainManagerAdmission
                && row
                    .toolchain_manager_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)),
            "stable packet must include a toolchain_manager_admission row binding a manager id on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == InspectorParityRowClass::LineageAdmission
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)),
            "stable packet must include a lineage_admission row binding execution_context_id on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == InspectorParityRowClass::RecoveryAdmission
                && row.recovery_state_class == RecoveryStateClass::RestoreNoRerun
                && row.restore_preserves_no_rerun),
            "stable packet must include a restore_no_rerun recovery_admission row attesting no-silent-rerun on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_inspector_parity_tokens_are_pinned() {
    assert_eq!(InspectorParityLaneClass::LocalLane.as_str(), "local_lane");
    assert_eq!(
        InspectorParityLaneClass::ManagedLane.as_str(),
        "managed_lane"
    );
    assert_eq!(
        InspectorParityRowClass::InspectorParityQuality.as_str(),
        "inspector_parity_quality"
    );
    assert_eq!(
        InspectorParityRowClass::ToolchainManagerAdmission.as_str(),
        "toolchain_manager_admission"
    );
    assert_eq!(
        InspectorParitySupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        InspectorParitySupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(InspectorFieldClass::Interpreter.as_str(), "interpreter");
    assert_eq!(InspectorFieldClass::PolicySource.as_str(), "policy_source");
    assert_eq!(ParitySurfaceClass::Ui.as_str(), "ui");
    assert_eq!(ParitySurfaceClass::SupportExport.as_str(), "support_export");
    assert_eq!(RecoveryStateClass::Reconnect.as_str(), "reconnect");
    assert_eq!(
        RecoveryStateClass::ArtifactProvenance.as_str(),
        "artifact_provenance"
    );
    assert_eq!(
        InspectorParityEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        InspectorParityKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        InspectorParityDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        InspectorParityConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        InspectorParityFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        InspectorParityFindingKind::ToolchainManagerAdmissionMissingManagerId.as_str(),
        "toolchain_manager_admission_missing_manager_id"
    );
    assert_eq!(
        InspectorParityFindingKind::RestoreRecoveryAdmitsSilentRerun.as_str(),
        "restore_recovery_admits_silent_rerun"
    );
    assert_eq!(
        InspectorParityFindingKind::ParitySurfaceVocabularyCollapsed.as_str(),
        "parity_surface_vocabulary_collapsed"
    );
}
