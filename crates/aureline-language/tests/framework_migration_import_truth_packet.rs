//! Fixture-driven coverage for the stable framework migration and
//! import guidance truth packet covering the framework migration
//! guidance, import guidance, and unsupported-gap labeling lanes with
//! outcome-label coverage (`exact_match`, `translated_match`,
//! `partial_match`, `shimmed_match`, `unsupported_gap`),
//! rollback-checkpoint admission, diagnostic-preservation admission,
//! launch-bundle coverage, known limits, downgrade automation, and
//! evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_framework_migration_import_truth_packet,
    FrameworkMigrationDiagnosticPreservationClass, FrameworkMigrationImportConsumerSurface,
    FrameworkMigrationImportFindingKind, FrameworkMigrationImportPromotionState,
    FrameworkMigrationImportSupportClass, FrameworkMigrationImportTruthPacket,
    FrameworkMigrationImportTruthPacketInput, FrameworkMigrationKnownLimitClass,
    FrameworkMigrationLaunchBundleClass, FrameworkMigrationOutcomeLabelClass,
    FrameworkMigrationRollbackCheckpointClass, FrameworkMigrationRowClass, MigrationLaneClass,
    FRAMEWORK_MIGRATION_IMPORT_TRUTH_ARTIFACT_DOC_REF, FRAMEWORK_MIGRATION_IMPORT_TRUTH_DOC_REF,
    FRAMEWORK_MIGRATION_IMPORT_TRUTH_FIXTURE_DIR,
    FRAMEWORK_MIGRATION_IMPORT_TRUTH_PACKET_ARTIFACT_REF,
    FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FrameworkMigrationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: FrameworkMigrationImportTruthPacketInput,
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
    outcome_label_tokens: Vec<String>,
    rollback_checkpoint_tokens: Vec<String>,
    diagnostic_preservation_tokens: Vec<String>,
    launch_bundle_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> FrameworkMigrationFixture {
    let path = repo_root()
        .join(FRAMEWORK_MIGRATION_IMPORT_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "framework_migration_import_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = FrameworkMigrationImportTruthPacket::materialize(fixture.input.clone());
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
        &packet.outcome_label_tokens(),
        &expect.outcome_label_tokens,
        "outcome_label",
    );
    assert_token_set_matches(
        &packet.rollback_checkpoint_tokens(),
        &expect.rollback_checkpoint_tokens,
        "rollback_checkpoint",
    );
    assert_token_set_matches(
        &packet.diagnostic_preservation_tokens(),
        &expect.diagnostic_preservation_tokens,
        "diagnostic_preservation",
    );
    assert_token_set_matches(
        &packet.launch_bundle_tokens(),
        &expect.launch_bundle_tokens,
        "launch_bundle",
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
    assert_exists(FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_REF);
    assert_exists(FRAMEWORK_MIGRATION_IMPORT_TRUTH_DOC_REF);
    assert_exists(FRAMEWORK_MIGRATION_IMPORT_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(FRAMEWORK_MIGRATION_IMPORT_TRUTH_FIXTURE_DIR);
    assert_exists(FRAMEWORK_MIGRATION_IMPORT_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_outcome_label_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_outcome_label_for_launch_stable_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_outcome_label_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_outcome_label_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_framework_migration_import_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        FrameworkMigrationImportPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in MigrationLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for migration lane {}",
            required.as_str()
        );
    }
    for surface in FrameworkMigrationImportConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_every_outcome_label_for_every_launch_stable_lane() {
    let packet = current_stable_framework_migration_import_truth_packet()
        .expect("checked-in packet validates");
    for required in MigrationLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == FrameworkMigrationRowClass::MigrationGuidanceQuality
                && row.support_class == FrameworkMigrationImportSupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        for label in FrameworkMigrationOutcomeLabelClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == FrameworkMigrationRowClass::OutcomeLabelTruth
                    && row.outcome_label_class == label),
                "stable packet must cover the {} outcome label on the {} lane",
                label.as_str(),
                required.as_str()
            );
        }
    }
}

#[test]
fn checked_in_artifact_covers_required_rollback_and_diagnostic_states() {
    let packet = current_stable_framework_migration_import_truth_packet()
        .expect("checked-in packet validates");
    for required in MigrationLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == FrameworkMigrationRowClass::MigrationGuidanceQuality
                && row.support_class == FrameworkMigrationImportSupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        for checkpoint in FrameworkMigrationRollbackCheckpointClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == FrameworkMigrationRowClass::RollbackCheckpointAdmission
                    && row.rollback_checkpoint_class == checkpoint),
                "stable packet must cover the {} rollback checkpoint on the {} lane",
                checkpoint.as_str(),
                required.as_str()
            );
        }
        for diagnostic in FrameworkMigrationDiagnosticPreservationClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == FrameworkMigrationRowClass::DiagnosticPreservationAdmission
                    && row.diagnostic_preservation_class == diagnostic),
                "stable packet must cover the {} diagnostic preservation on the {} lane",
                diagnostic.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == FrameworkMigrationRowClass::LaunchBundleCoverage),
            "stable packet must surface a launch_bundle_coverage row on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_framework_migration_tokens_are_pinned() {
    assert_eq!(
        MigrationLaneClass::FrameworkMigrationGuidanceLane.as_str(),
        "framework_migration_guidance_lane"
    );
    assert_eq!(
        MigrationLaneClass::ImportGuidanceLane.as_str(),
        "import_guidance_lane"
    );
    assert_eq!(
        MigrationLaneClass::UnsupportedGapLabelingLane.as_str(),
        "unsupported_gap_labeling_lane"
    );
    assert_eq!(
        FrameworkMigrationRowClass::MigrationGuidanceQuality.as_str(),
        "migration_guidance_quality"
    );
    assert_eq!(
        FrameworkMigrationRowClass::OutcomeLabelTruth.as_str(),
        "outcome_label_truth"
    );
    assert_eq!(
        FrameworkMigrationImportSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        FrameworkMigrationImportSupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        FrameworkMigrationImportSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        FrameworkMigrationOutcomeLabelClass::ExactMatch.as_str(),
        "exact_match"
    );
    assert_eq!(
        FrameworkMigrationOutcomeLabelClass::UnsupportedGap.as_str(),
        "unsupported_gap"
    );
    assert_eq!(
        FrameworkMigrationOutcomeLabelClass::LabelUnbound.as_str(),
        "label_unbound"
    );
    assert_eq!(
        FrameworkMigrationRollbackCheckpointClass::CheckpointPreserved.as_str(),
        "checkpoint_preserved"
    );
    assert_eq!(
        FrameworkMigrationRollbackCheckpointClass::CheckpointUnbound.as_str(),
        "checkpoint_unbound"
    );
    assert_eq!(
        FrameworkMigrationDiagnosticPreservationClass::DiagnosticsPreserved.as_str(),
        "diagnostics_preserved"
    );
    assert_eq!(
        FrameworkMigrationDiagnosticPreservationClass::DiagnosticUnbound.as_str(),
        "diagnostic_unbound"
    );
    assert_eq!(
        FrameworkMigrationLaunchBundleClass::PythonLaunchBundle.as_str(),
        "python_launch_bundle"
    );
    assert_eq!(
        FrameworkMigrationLaunchBundleClass::CCppLaunchBundle.as_str(),
        "c_cpp_launch_bundle"
    );
    assert_eq!(
        FrameworkMigrationKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        FrameworkMigrationImportConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        FrameworkMigrationImportFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        FrameworkMigrationImportFindingKind::MissingOutcomeLabelCoverage.as_str(),
        "missing_outcome_label_coverage"
    );
    assert_eq!(
        FrameworkMigrationImportFindingKind::OutcomeLabelVocabularyCollapsed.as_str(),
        "outcome_label_vocabulary_collapsed"
    );
}
