//! Fixture-driven coverage for the stable launch-language tooling
//! truth packet covering the shell/bash, SQL, Markdown, JSON/YAML, and
//! Git-oriented launch-tooling lanes with the open/import, navigate,
//! edit, complete, refactor, run/test/debug, review, migrate, and
//! recover daily-loop steps plus known limits, downgrade automation,
//! and evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_launch_language_tooling_truth_packet, LaunchLanguageToolingConsumerSurface,
    LaunchLanguageToolingDowngradeAutomationClass, LaunchLanguageToolingEvidenceClass,
    LaunchLanguageToolingFindingKind, LaunchLanguageToolingKnownLimitClass,
    LaunchLanguageToolingLaneClass, LaunchLanguageToolingPromotionState,
    LaunchLanguageToolingRowClass, LaunchLanguageToolingStepClass,
    LaunchLanguageToolingSupportClass, LaunchLanguageToolingTruthPacket,
    LaunchLanguageToolingTruthPacketInput, LAUNCH_LANGUAGE_TOOLING_TRUTH_ARTIFACT_DOC_REF,
    LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF, LAUNCH_LANGUAGE_TOOLING_TRUTH_FIXTURE_DIR,
    LAUNCH_LANGUAGE_TOOLING_TRUTH_PACKET_ARTIFACT_REF, LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LaunchLanguageToolingFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: LaunchLanguageToolingTruthPacketInput,
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

fn load_fixture(file_name: &str) -> LaunchLanguageToolingFixture {
    let path = repo_root()
        .join(LAUNCH_LANGUAGE_TOOLING_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "launch_language_tooling_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = LaunchLanguageToolingTruthPacket::materialize(fixture.input.clone());
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
    assert_exists(LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_REF);
    assert_exists(LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF);
    assert_exists(LAUNCH_LANGUAGE_TOOLING_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(LAUNCH_LANGUAGE_TOOLING_TRUTH_FIXTURE_DIR);
    assert_exists(LAUNCH_LANGUAGE_TOOLING_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn launch_support_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("launch_support_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_daily_loop_step_for_launch_support_blocks_stable() {
    assert_fixture_matches("missing_daily_loop_step_for_launch_support_blocks_stable.json");
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
    let packet =
        current_stable_launch_language_tooling_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        LaunchLanguageToolingPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in LaunchLanguageToolingLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for launch-tooling lane {}",
            required.as_str()
        );
    }
    for surface in LaunchLanguageToolingConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_daily_loop_for_every_launch_support_lane() {
    let packet =
        current_stable_launch_language_tooling_truth_packet().expect("checked-in packet validates");
    for required in LaunchLanguageToolingLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == LaunchLanguageToolingRowClass::LaunchToolingQuality
                && row.support_class == LaunchLanguageToolingSupportClass::LaunchSupport
        });
        if !lane_claims_launch {
            continue;
        }
        for step in LaunchLanguageToolingStepClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == LaunchLanguageToolingRowClass::DailyLoopStep
                    && row.daily_loop_step_class == step),
                "stable packet must cover the {} daily-loop step on the {} lane",
                step.as_str(),
                required.as_str()
            );
        }
    }
}

#[test]
fn closed_launch_language_tooling_tokens_are_pinned() {
    assert_eq!(
        LaunchLanguageToolingLaneClass::ShellBashLane.as_str(),
        "shell_bash_lane"
    );
    assert_eq!(
        LaunchLanguageToolingLaneClass::GitOrientedLane.as_str(),
        "git_oriented_lane"
    );
    assert_eq!(
        LaunchLanguageToolingRowClass::LaunchToolingQuality.as_str(),
        "launch_tooling_quality"
    );
    assert_eq!(
        LaunchLanguageToolingSupportClass::LaunchSupport.as_str(),
        "launch_support"
    );
    assert_eq!(
        LaunchLanguageToolingSupportClass::LaunchSupportBelow.as_str(),
        "launch_support_below"
    );
    assert_eq!(
        LaunchLanguageToolingSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(LaunchLanguageToolingStepClass::Recover.as_str(), "recover");
    assert_eq!(
        LaunchLanguageToolingEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        LaunchLanguageToolingKnownLimitClass::LaunchToolingScopeOnly.as_str(),
        "launch_tooling_scope_only"
    );
    assert_eq!(
        LaunchLanguageToolingKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        LaunchLanguageToolingDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        LaunchLanguageToolingConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        LaunchLanguageToolingFindingKind::LaunchSupportWithUnboundBinding.as_str(),
        "launch_support_with_unbound_binding"
    );
    assert_eq!(
        LaunchLanguageToolingFindingKind::EvidenceClassVocabularyCollapsed.as_str(),
        "evidence_class_vocabulary_collapsed"
    );
}
