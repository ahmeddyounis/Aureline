//! Fixture-driven coverage for the stable refactor transaction truth
//! packet covering the rename, extract-function, inline-symbol,
//! move-symbol, update-imports, and cross-file-signature-change
//! launch-language refactor lanes with the full transaction-phase
//! coverage (preview, validate, apply, rollback), preview-outcome
//! admission, validation-hook admission, rollback-drill admission,
//! launch-language coverage, known limits, downgrade automation, and
//! evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_refactor_transaction_truth_packet, RefactorClassLaneClass,
    RefactorTransactionConsumerSurface, RefactorTransactionDowngradeAutomationClass,
    RefactorTransactionEvidenceClass, RefactorTransactionFindingKind,
    RefactorTransactionKnownLimitClass, RefactorTransactionLaunchLanguageClass,
    RefactorTransactionPhaseClass, RefactorTransactionPreviewCompletenessClass,
    RefactorTransactionPromotionState, RefactorTransactionRollbackPathClass,
    RefactorTransactionRowClass, RefactorTransactionSupportClass, RefactorTransactionTruthPacket,
    RefactorTransactionTruthPacketInput, RefactorTransactionValidationOutcomeClass,
    REFACTOR_TRANSACTION_TRUTH_ARTIFACT_DOC_REF, REFACTOR_TRANSACTION_TRUTH_DOC_REF,
    REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR, REFACTOR_TRANSACTION_TRUTH_PACKET_ARTIFACT_REF,
    REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RefactorTransactionFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: RefactorTransactionTruthPacketInput,
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
    transaction_phase_tokens: Vec<String>,
    preview_completeness_tokens: Vec<String>,
    validation_outcome_tokens: Vec<String>,
    rollback_path_tokens: Vec<String>,
    launch_language_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> RefactorTransactionFixture {
    let path = repo_root()
        .join(REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "refactor_transaction_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = RefactorTransactionTruthPacket::materialize(fixture.input.clone());
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
        &packet.transaction_phase_tokens(),
        &expect.transaction_phase_tokens,
        "transaction_phase",
    );
    assert_token_set_matches(
        &packet.preview_completeness_tokens(),
        &expect.preview_completeness_tokens,
        "preview_completeness",
    );
    assert_token_set_matches(
        &packet.validation_outcome_tokens(),
        &expect.validation_outcome_tokens,
        "validation_outcome",
    );
    assert_token_set_matches(
        &packet.rollback_path_tokens(),
        &expect.rollback_path_tokens,
        "rollback_path",
    );
    assert_token_set_matches(
        &packet.launch_language_tokens(),
        &expect.launch_language_tokens,
        "launch_language",
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
    assert_exists(REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF);
    assert_exists(REFACTOR_TRANSACTION_TRUTH_DOC_REF);
    assert_exists(REFACTOR_TRANSACTION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR);
    assert_exists(REFACTOR_TRANSACTION_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_transaction_phase_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_transaction_phase_for_launch_stable_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_rollback_path_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_rollback_path_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet =
        current_stable_refactor_transaction_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        RefactorTransactionPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in RefactorClassLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for refactor-class lane {}",
            required.as_str()
        );
    }
    for surface in RefactorTransactionConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_every_transaction_phase_for_every_launch_stable_lane() {
    let packet =
        current_stable_refactor_transaction_truth_packet().expect("checked-in packet validates");
    for required in RefactorClassLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == RefactorTransactionRowClass::RefactorTransactionQuality
                && row.support_class == RefactorTransactionSupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        for phase in RefactorTransactionPhaseClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == RefactorTransactionRowClass::TransactionPhaseTruth
                    && row.transaction_phase_class == phase),
                "stable packet must cover the {} transaction phase on the {} lane",
                phase.as_str(),
                required.as_str()
            );
        }
    }
}

#[test]
fn checked_in_artifact_covers_preview_validation_and_rollback_admissions() {
    let packet =
        current_stable_refactor_transaction_truth_packet().expect("checked-in packet validates");
    for required in RefactorClassLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == RefactorTransactionRowClass::RefactorTransactionQuality
                && row.support_class == RefactorTransactionSupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RefactorTransactionRowClass::PreviewOutcomeAdmission),
            "stable packet must surface a preview_outcome_admission row on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RefactorTransactionRowClass::ValidationHookAdmission),
            "stable packet must surface a validation_hook_admission row on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RefactorTransactionRowClass::RollbackDrillAdmission),
            "stable packet must surface a rollback_drill_admission row on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RefactorTransactionRowClass::LaunchLanguageCoverage),
            "stable packet must surface a launch_language_coverage row on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_refactor_transaction_tokens_are_pinned() {
    assert_eq!(
        RefactorClassLaneClass::RenameSymbolLane.as_str(),
        "rename_symbol_lane"
    );
    assert_eq!(
        RefactorClassLaneClass::CrossFileSignatureChangeLane.as_str(),
        "cross_file_signature_change_lane"
    );
    assert_eq!(
        RefactorTransactionRowClass::RefactorTransactionQuality.as_str(),
        "refactor_transaction_quality"
    );
    assert_eq!(
        RefactorTransactionRowClass::TransactionPhaseTruth.as_str(),
        "transaction_phase_truth"
    );
    assert_eq!(
        RefactorTransactionSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        RefactorTransactionSupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        RefactorTransactionSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(RefactorTransactionPhaseClass::Preview.as_str(), "preview");
    assert_eq!(RefactorTransactionPhaseClass::Rollback.as_str(), "rollback");
    assert_eq!(
        RefactorTransactionPreviewCompletenessClass::Complete.as_str(),
        "complete"
    );
    assert_eq!(
        RefactorTransactionPreviewCompletenessClass::PreviewUnbound.as_str(),
        "preview_unbound"
    );
    assert_eq!(
        RefactorTransactionValidationOutcomeClass::Passed.as_str(),
        "passed"
    );
    assert_eq!(
        RefactorTransactionValidationOutcomeClass::OutcomeUnbound.as_str(),
        "outcome_unbound"
    );
    assert_eq!(
        RefactorTransactionRollbackPathClass::ExactUndoViaLocalHistoryCheckpoint.as_str(),
        "exact_undo_via_local_history_checkpoint"
    );
    assert_eq!(
        RefactorTransactionRollbackPathClass::RollbackUnbound.as_str(),
        "rollback_unbound"
    );
    assert_eq!(
        RefactorTransactionLaunchLanguageClass::Python.as_str(),
        "python"
    );
    assert_eq!(
        RefactorTransactionLaunchLanguageClass::JavaKotlin.as_str(),
        "java_kotlin"
    );
    assert_eq!(
        RefactorTransactionEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        RefactorTransactionKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        RefactorTransactionDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        RefactorTransactionConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        RefactorTransactionFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        RefactorTransactionFindingKind::MissingTransactionPhaseCoverage.as_str(),
        "missing_transaction_phase_coverage"
    );
    assert_eq!(
        RefactorTransactionFindingKind::RollbackPathVocabularyCollapsed.as_str(),
        "rollback_path_vocabulary_collapsed"
    );
}
