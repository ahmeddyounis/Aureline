//! Fixture-driven coverage for the stable launch-language conformance
//! pack publication truth packet covering the Python, TypeScript /
//! JavaScript, Rust, Go, Java / Kotlin, and C / C++ launch-language
//! lanes with daily-loop coverage, support-class-evidence admission,
//! downgrade-rule admission, known limits, downgrade automation, and
//! evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_publish_launch_language_conformance_packs_truth_packet,
    ConformancePackRowClass, DowngradeRuleClass, LaunchLanguageLaneClass,
    PublishLaunchLanguageConformancePacksConsumerSurface,
    PublishLaunchLanguageConformancePacksDailyLoopStepClass,
    PublishLaunchLanguageConformancePacksFindingKind,
    PublishLaunchLanguageConformancePacksKnownLimitClass,
    PublishLaunchLanguageConformancePacksPromotionState,
    PublishLaunchLanguageConformancePacksSupportClass,
    PublishLaunchLanguageConformancePacksTruthPacket,
    PublishLaunchLanguageConformancePacksTruthPacketInput, SupportClassEvidenceClass,
    PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_ARTIFACT_DOC_REF,
    PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_DOC_REF,
    PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_FIXTURE_DIR,
    PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_PACKET_ARTIFACT_REF,
    PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConformancePacksFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: PublishLaunchLanguageConformancePacksTruthPacketInput,
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
    support_class_evidence_tokens: Vec<String>,
    downgrade_rule_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ConformancePacksFixture {
    let path = repo_root()
        .join(PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "publish_launch_language_conformance_packs_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet =
        PublishLaunchLanguageConformancePacksTruthPacket::materialize(fixture.input.clone());
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
        &packet.support_class_evidence_tokens(),
        &expect.support_class_evidence_tokens,
        "support_class_evidence",
    );
    assert_token_set_matches(
        &packet.downgrade_rule_tokens(),
        &expect.downgrade_rule_tokens,
        "downgrade_rule",
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
    assert_exists(PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_REF);
    assert_exists(PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_DOC_REF);
    assert_exists(PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_FIXTURE_DIR);
    assert_exists(PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_daily_loop_step_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_daily_loop_step_for_launch_stable_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_support_class_evidence_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_support_class_evidence_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_publish_launch_language_conformance_packs_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        PublishLaunchLanguageConformancePacksPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in LaunchLanguageLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for launch-language lane {}",
            required.as_str()
        );
    }
    for surface in PublishLaunchLanguageConformancePacksConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_full_daily_loop_per_launch_stable_lane() {
    let packet = current_stable_publish_launch_language_conformance_packs_truth_packet()
        .expect("checked-in packet validates");
    for required in LaunchLanguageLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == ConformancePackRowClass::ConformancePackQuality
                && row.support_class == PublishLaunchLanguageConformancePacksSupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        for step in PublishLaunchLanguageConformancePacksDailyLoopStepClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ConformancePackRowClass::DailyLoopStep
                    && row.daily_loop_step_class == step),
                "stable packet must cover the {} daily-loop step on the {} lane",
                step.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class
                    == ConformancePackRowClass::SupportClassEvidenceAdmission
                && row.support_class_evidence_class.is_bound()
                && !matches!(
                    row.support_class_evidence_class,
                    SupportClassEvidenceClass::NotApplicable
                )),
            "stable packet must surface a support_class_evidence_admission row on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == ConformancePackRowClass::DowngradeRuleAdmission
                && row.downgrade_rule_class.is_bound()
                && !matches!(row.downgrade_rule_class, DowngradeRuleClass::NotApplicable)),
            "stable packet must surface a downgrade_rule_admission row on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_conformance_pack_tokens_are_pinned() {
    assert_eq!(
        LaunchLanguageLaneClass::PythonLaunchLanguageLane.as_str(),
        "python_launch_language_lane"
    );
    assert_eq!(
        LaunchLanguageLaneClass::TypescriptJavascriptLaunchLanguageLane.as_str(),
        "typescript_javascript_launch_language_lane"
    );
    assert_eq!(
        LaunchLanguageLaneClass::CCppLaunchLanguageLane.as_str(),
        "c_cpp_launch_language_lane"
    );
    assert_eq!(
        ConformancePackRowClass::ConformancePackQuality.as_str(),
        "conformance_pack_quality"
    );
    assert_eq!(
        ConformancePackRowClass::SupportClassEvidenceAdmission.as_str(),
        "support_class_evidence_admission"
    );
    assert_eq!(
        ConformancePackRowClass::DowngradeRuleAdmission.as_str(),
        "downgrade_rule_admission"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksSupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        SupportClassEvidenceClass::ArchetypeRepoBacked.as_str(),
        "archetype_repo_backed"
    );
    assert_eq!(
        SupportClassEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        DowngradeRuleClass::NarrowOnMissingFixture.as_str(),
        "narrow_on_missing_fixture"
    );
    assert_eq!(DowngradeRuleClass::RuleUnbound.as_str(), "rule_unbound");
    assert_eq!(
        PublishLaunchLanguageConformancePacksDailyLoopStepClass::Recover.as_str(),
        "recover"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksFindingKind::MissingSupportClassEvidenceCoverage
            .as_str(),
        "missing_support_class_evidence_coverage"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksFindingKind::MissingDowngradeRuleCoverage.as_str(),
        "missing_downgrade_rule_coverage"
    );
    assert_eq!(
        PublishLaunchLanguageConformancePacksFindingKind::SupportClassEvidenceVocabularyCollapsed
            .as_str(),
        "support_class_evidence_vocabulary_collapsed"
    );
}
