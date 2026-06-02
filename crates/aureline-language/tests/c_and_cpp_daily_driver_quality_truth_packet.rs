//! Fixture-driven coverage for the stable C and C++ daily-driver
//! quality truth packet covering the open/import, navigate, edit,
//! complete, refactor, run/test/debug, review, migrate, and recover
//! daily-loop steps plus the CMake/Ninja build workspace truth, the
//! compile/run/debug fidelity truth, and the clangd rename/navigation
//! truth with known limits, downgrade automation, and evidence
//! binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_c_and_cpp_daily_driver_quality_truth_packet, CAndCppDailyDriverConfidenceClass,
    CAndCppDailyDriverConsumerSurface, CAndCppDailyDriverDowngradeAutomationClass,
    CAndCppDailyDriverEvidenceClass, CAndCppDailyDriverKnownLimitClass,
    CAndCppDailyDriverLanguageLaneClass, CAndCppDailyDriverQualityFindingKind,
    CAndCppDailyDriverQualityPromotionState, CAndCppDailyDriverQualityRowClass,
    CAndCppDailyDriverQualityTruthPacket, CAndCppDailyDriverQualityTruthPacketInput,
    CAndCppDailyDriverStepClass, CAndCppDailyDriverSupportClass,
    C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_ARTIFACT_DOC_REF,
    C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF, C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR,
    C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_PACKET_ARTIFACT_REF,
    C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CAndCppDailyDriverQualityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CAndCppDailyDriverQualityTruthPacketInput,
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
    daily_loop_step_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> CAndCppDailyDriverQualityFixture {
    let path = repo_root()
        .join(C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "c_and_cpp_daily_driver_quality_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = CAndCppDailyDriverQualityTruthPacket::materialize(fixture.input.clone());
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
        &packet.daily_loop_step_tokens(),
        &expect.daily_loop_step_tokens,
        "daily_loop_step",
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
    assert_exists(C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_REF);
    assert_exists(C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF);
    assert_exists(C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR);
    assert_exists(C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn replacement_grade_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("replacement_grade_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_daily_loop_step_for_replacement_grade_blocks_stable() {
    assert_fixture_matches("missing_daily_loop_step_for_replacement_grade_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_evidence_class_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_evidence_class_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_c_and_cpp_daily_driver_quality_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        CAndCppDailyDriverQualityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in CAndCppDailyDriverLanguageLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for language lane {}",
            required.as_str()
        );
    }
    for surface in CAndCppDailyDriverConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_build_workspace_compile_run_debug_and_clangd_navigation() {
    let packet = current_stable_c_and_cpp_daily_driver_quality_truth_packet()
        .expect("checked-in packet validates");
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == CAndCppDailyDriverQualityRowClass::BuildWorkspaceRow),
        "stable packet must include a build_workspace_row certifying the CMake/Ninja contract"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == CAndCppDailyDriverQualityRowClass::CompileRunDebugRow),
        "stable packet must include a compile_run_debug_row certifying the compile/run/debug fidelity surface"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == CAndCppDailyDriverQualityRowClass::ClangdNavigationRow),
        "stable packet must include a clangd_navigation_row certifying the clangd LSP rename and navigation surface"
    );
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == CAndCppDailyDriverQualityRowClass::MigrationEvidence),
        "stable packet must include a migration_evidence row certifying a C or C++ migration archetype"
    );
}

#[test]
fn closed_c_and_cpp_daily_driver_quality_tokens_are_pinned() {
    assert_eq!(
        CAndCppDailyDriverLanguageLaneClass::CAndCppDailyDriverLane.as_str(),
        "c_and_cpp_daily_driver_lane"
    );
    assert_eq!(
        CAndCppDailyDriverQualityRowClass::BuildWorkspaceRow.as_str(),
        "build_workspace_row"
    );
    assert_eq!(
        CAndCppDailyDriverQualityRowClass::CompileRunDebugRow.as_str(),
        "compile_run_debug_row"
    );
    assert_eq!(
        CAndCppDailyDriverQualityRowClass::ClangdNavigationRow.as_str(),
        "clangd_navigation_row"
    );
    assert_eq!(
        CAndCppDailyDriverSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(CAndCppDailyDriverStepClass::Recover.as_str(), "recover");
    assert_eq!(
        CAndCppDailyDriverEvidenceClass::BuildWorkspaceEvidence.as_str(),
        "build_workspace_evidence"
    );
    assert_eq!(
        CAndCppDailyDriverEvidenceClass::CompileRunDebugEvidence.as_str(),
        "compile_run_debug_evidence"
    );
    assert_eq!(
        CAndCppDailyDriverEvidenceClass::ClangdNavigationEvidence.as_str(),
        "clangd_navigation_evidence"
    );
    assert_eq!(
        CAndCppDailyDriverEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        CAndCppDailyDriverKnownLimitClass::BuildWorkspaceSubsetOnly.as_str(),
        "build_workspace_subset_only"
    );
    assert_eq!(
        CAndCppDailyDriverKnownLimitClass::CompileRunDebugSubsetOnly.as_str(),
        "compile_run_debug_subset_only"
    );
    assert_eq!(
        CAndCppDailyDriverKnownLimitClass::ClangdNavigationSubsetOnly.as_str(),
        "clangd_navigation_subset_only"
    );
    assert_eq!(
        CAndCppDailyDriverKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        CAndCppDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenBuildWorkspace.as_str(),
        "auto_narrow_on_unproven_build_workspace"
    );
    assert_eq!(
        CAndCppDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenCompileRunDebug.as_str(),
        "auto_narrow_on_unproven_compile_run_debug"
    );
    assert_eq!(
        CAndCppDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenClangdNavigation.as_str(),
        "auto_narrow_on_unproven_clangd_navigation"
    );
    assert_eq!(
        CAndCppDailyDriverDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        CAndCppDailyDriverConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        CAndCppDailyDriverConfidenceClass::HighConfidence.as_str(),
        "high_confidence"
    );
    assert_eq!(
        CAndCppDailyDriverQualityFindingKind::EvidenceClassVocabularyCollapsed.as_str(),
        "evidence_class_vocabulary_collapsed"
    );
}
