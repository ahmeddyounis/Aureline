//! Fixture-driven coverage for the stable Rust daily-driver quality
//! truth packet covering the open/import, navigate, edit, complete,
//! refactor, run/test/debug, review, migrate, and recover daily-loop
//! steps plus the Cargo workspaces truth (single-package `Cargo.toml`
//! / `Cargo.lock`, multi-package `[workspace]` / `members` /
//! `exclude` / `default-members` / `resolver`; `rust-toolchain.toml`
//! channel pinning; `CARGO_HOME` / `CARGO_TARGET_DIR` /
//! `CARGO_REGISTRIES_*` / `CARGO_NET_OFFLINE` resolution; `[patch]` /
//! `[replace]` / `[profile.*]` directives), the clippy / rustfmt
//! lint-format truth, the test-runner truth (`cargo test` /
//! `cargo nextest` / `cargo bench`), the debugger truth
//! (`rust-lldb` / `rust-gdb` / CodeLLDB / `lldb-dap`), and the
//! rust-analyzer large-workspace indexing truth with known limits,
//! downgrade automation, and evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_rust_daily_driver_quality_truth_packet, RustDailyDriverConfidenceClass,
    RustDailyDriverConsumerSurface, RustDailyDriverDowngradeAutomationClass,
    RustDailyDriverEvidenceClass, RustDailyDriverKnownLimitClass, RustDailyDriverLanguageLaneClass,
    RustDailyDriverQualityFindingKind, RustDailyDriverQualityPromotionState,
    RustDailyDriverQualityRowClass, RustDailyDriverQualityTruthPacket,
    RustDailyDriverQualityTruthPacketInput, RustDailyDriverStepClass, RustDailyDriverSupportClass,
    RUST_DAILY_DRIVER_QUALITY_TRUTH_ARTIFACT_DOC_REF, RUST_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF,
    RUST_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR,
    RUST_DAILY_DRIVER_QUALITY_TRUTH_PACKET_ARTIFACT_REF,
    RUST_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RustDailyDriverQualityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: RustDailyDriverQualityTruthPacketInput,
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

fn load_fixture(file_name: &str) -> RustDailyDriverQualityFixture {
    let path = repo_root()
        .join(RUST_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "rust_daily_driver_quality_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = RustDailyDriverQualityTruthPacket::materialize(fixture.input.clone());
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
    assert_exists(RUST_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_REF);
    assert_exists(RUST_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF);
    assert_exists(RUST_DAILY_DRIVER_QUALITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(RUST_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR);
    assert_exists(RUST_DAILY_DRIVER_QUALITY_TRUTH_PACKET_ARTIFACT_REF);
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
    let packet = current_stable_rust_daily_driver_quality_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        RustDailyDriverQualityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in RustDailyDriverLanguageLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for language lane {}",
            required.as_str()
        );
    }
    for surface in RustDailyDriverConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_cargo_workspace_lint_format_test_runner_debugger_and_workspace_index()
{
    let packet = current_stable_rust_daily_driver_quality_truth_packet()
        .expect("checked-in packet validates");
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == RustDailyDriverQualityRowClass::CargoWorkspaceRow),
        "stable packet must include a cargo_workspace_row certifying the Cargo workspaces contract"
    );
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == RustDailyDriverQualityRowClass::LintFormatRow),
        "stable packet must include a lint_format_row certifying the cargo clippy / cargo fmt surface"
    );
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == RustDailyDriverQualityRowClass::TestRunnerRow),
        "stable packet must include a test_runner_row certifying the cargo test / cargo nextest surface"
    );
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == RustDailyDriverQualityRowClass::DebuggerRow),
        "stable packet must include a debugger_row certifying the rust-lldb / rust-gdb / CodeLLDB surface"
    );
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == RustDailyDriverQualityRowClass::WorkspaceIndexRow),
        "stable packet must include a workspace_index_row certifying the rust-analyzer large-workspace indexing surface"
    );
    assert!(
        packet
            .rows
            .iter()
            .any(|row| row.row_class == RustDailyDriverQualityRowClass::MigrationEvidence),
        "stable packet must include a migration_evidence row certifying a Rust migration archetype"
    );
}

#[test]
fn closed_rust_daily_driver_quality_tokens_are_pinned() {
    assert_eq!(
        RustDailyDriverLanguageLaneClass::RustDailyDriverLane.as_str(),
        "rust_daily_driver_lane"
    );
    assert_eq!(
        RustDailyDriverQualityRowClass::CargoWorkspaceRow.as_str(),
        "cargo_workspace_row"
    );
    assert_eq!(
        RustDailyDriverQualityRowClass::LintFormatRow.as_str(),
        "lint_format_row"
    );
    assert_eq!(
        RustDailyDriverQualityRowClass::TestRunnerRow.as_str(),
        "test_runner_row"
    );
    assert_eq!(
        RustDailyDriverQualityRowClass::DebuggerRow.as_str(),
        "debugger_row"
    );
    assert_eq!(
        RustDailyDriverQualityRowClass::WorkspaceIndexRow.as_str(),
        "workspace_index_row"
    );
    assert_eq!(
        RustDailyDriverSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(RustDailyDriverStepClass::Recover.as_str(), "recover");
    assert_eq!(
        RustDailyDriverEvidenceClass::CargoWorkspaceEvidence.as_str(),
        "cargo_workspace_evidence"
    );
    assert_eq!(
        RustDailyDriverEvidenceClass::LintFormatEvidence.as_str(),
        "lint_format_evidence"
    );
    assert_eq!(
        RustDailyDriverEvidenceClass::TestRunnerEvidence.as_str(),
        "test_runner_evidence"
    );
    assert_eq!(
        RustDailyDriverEvidenceClass::DebuggerEvidence.as_str(),
        "debugger_evidence"
    );
    assert_eq!(
        RustDailyDriverEvidenceClass::WorkspaceIndexEvidence.as_str(),
        "workspace_index_evidence"
    );
    assert_eq!(
        RustDailyDriverEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        RustDailyDriverKnownLimitClass::CargoWorkspaceSubsetOnly.as_str(),
        "cargo_workspace_subset_only"
    );
    assert_eq!(
        RustDailyDriverKnownLimitClass::LintFormatSubsetOnly.as_str(),
        "lint_format_subset_only"
    );
    assert_eq!(
        RustDailyDriverKnownLimitClass::TestRunnerSubsetOnly.as_str(),
        "test_runner_subset_only"
    );
    assert_eq!(
        RustDailyDriverKnownLimitClass::DebuggerSubsetOnly.as_str(),
        "debugger_subset_only"
    );
    assert_eq!(
        RustDailyDriverKnownLimitClass::WorkspaceIndexSubsetOnly.as_str(),
        "workspace_index_subset_only"
    );
    assert_eq!(
        RustDailyDriverKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        RustDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenCargoWorkspace.as_str(),
        "auto_narrow_on_unproven_cargo_workspace"
    );
    assert_eq!(
        RustDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenLintFormat.as_str(),
        "auto_narrow_on_unproven_lint_format"
    );
    assert_eq!(
        RustDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenTestRunner.as_str(),
        "auto_narrow_on_unproven_test_runner"
    );
    assert_eq!(
        RustDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenDebugger.as_str(),
        "auto_narrow_on_unproven_debugger"
    );
    assert_eq!(
        RustDailyDriverDowngradeAutomationClass::AutoNarrowOnUnprovenWorkspaceIndex.as_str(),
        "auto_narrow_on_unproven_workspace_index"
    );
    assert_eq!(
        RustDailyDriverDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        RustDailyDriverConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        RustDailyDriverConfidenceClass::HighConfidence.as_str(),
        "high_confidence"
    );
    assert_eq!(
        RustDailyDriverQualityFindingKind::EvidenceClassVocabularyCollapsed.as_str(),
        "evidence_class_vocabulary_collapsed"
    );
}
