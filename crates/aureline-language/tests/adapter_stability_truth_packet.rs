//! Fixture-driven coverage for the stable adapter stability truth
//! packet covering the formatter, linter, and test-adapter lanes with
//! adapter-capability coverage (discover, execute, report),
//! degraded-provider admission (provider_healthy,
//! provider_degraded_warned, provider_unavailable), adapter-outcome
//! admission, launch-wedge coverage, known limits, downgrade
//! automation, and evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_adapter_stability_truth_packet, AdapterLaneClass,
    AdapterStabilityCapabilityClass, AdapterStabilityConsumerSurface,
    AdapterStabilityDegradedProviderClass, AdapterStabilityDowngradeAutomationClass,
    AdapterStabilityEvidenceClass, AdapterStabilityFindingKind,
    AdapterStabilityKnownLimitClass, AdapterStabilityLaunchWedgeClass,
    AdapterStabilityOutcomeClass, AdapterStabilityPromotionState, AdapterStabilityRowClass,
    AdapterStabilitySupportClass, AdapterStabilityTruthPacket,
    AdapterStabilityTruthPacketInput, ADAPTER_STABILITY_TRUTH_ARTIFACT_DOC_REF,
    ADAPTER_STABILITY_TRUTH_DOC_REF, ADAPTER_STABILITY_TRUTH_FIXTURE_DIR,
    ADAPTER_STABILITY_TRUTH_PACKET_ARTIFACT_REF, ADAPTER_STABILITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AdapterStabilityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: AdapterStabilityTruthPacketInput,
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
    adapter_capability_tokens: Vec<String>,
    degraded_provider_tokens: Vec<String>,
    adapter_outcome_tokens: Vec<String>,
    launch_wedge_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> AdapterStabilityFixture {
    let path = repo_root()
        .join(ADAPTER_STABILITY_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "adapter_stability_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = AdapterStabilityTruthPacket::materialize(fixture.input.clone());
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
        &packet.adapter_capability_tokens(),
        &expect.adapter_capability_tokens,
        "adapter_capability",
    );
    assert_token_set_matches(
        &packet.degraded_provider_tokens(),
        &expect.degraded_provider_tokens,
        "degraded_provider",
    );
    assert_token_set_matches(
        &packet.adapter_outcome_tokens(),
        &expect.adapter_outcome_tokens,
        "adapter_outcome",
    );
    assert_token_set_matches(
        &packet.launch_wedge_tokens(),
        &expect.launch_wedge_tokens,
        "launch_wedge",
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
    assert_exists(ADAPTER_STABILITY_TRUTH_SCHEMA_REF);
    assert_exists(ADAPTER_STABILITY_TRUTH_DOC_REF);
    assert_exists(ADAPTER_STABILITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(ADAPTER_STABILITY_TRUTH_FIXTURE_DIR);
    assert_exists(ADAPTER_STABILITY_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_capability_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_capability_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_degraded_provider_state_blocks_stable() {
    assert_fixture_matches("missing_degraded_provider_state_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_degraded_provider_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_degraded_provider_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet =
        current_stable_adapter_stability_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        AdapterStabilityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in AdapterLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for adapter lane {}",
            required.as_str()
        );
    }
    for surface in AdapterStabilityConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_every_capability_for_every_launch_stable_lane() {
    let packet =
        current_stable_adapter_stability_truth_packet().expect("checked-in packet validates");
    for required in AdapterLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == AdapterStabilityRowClass::AdapterStabilityQuality
                && row.support_class == AdapterStabilitySupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        for capability in AdapterStabilityCapabilityClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == AdapterStabilityRowClass::AdapterCapabilityTruth
                    && row.adapter_capability_class == capability),
                "stable packet must cover the {} adapter capability on the {} lane",
                capability.as_str(),
                required.as_str()
            );
        }
    }
}

#[test]
fn checked_in_artifact_covers_every_required_degraded_provider_state() {
    let packet =
        current_stable_adapter_stability_truth_packet().expect("checked-in packet validates");
    for required in AdapterLaneClass::REQUIRED {
        let lane_claims_stable = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == AdapterStabilityRowClass::AdapterStabilityQuality
                && row.support_class == AdapterStabilitySupportClass::LaunchStable
        });
        if !lane_claims_stable {
            continue;
        }
        for state in AdapterStabilityDegradedProviderClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == AdapterStabilityRowClass::DegradedProviderAdmission
                    && row.degraded_provider_class == state),
                "stable packet must cover the {} degraded provider state on the {} lane",
                state.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == AdapterStabilityRowClass::AdapterOutcomeAdmission),
            "stable packet must surface an adapter_outcome_admission row on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == AdapterStabilityRowClass::LaunchWedgeCoverage),
            "stable packet must surface a launch_wedge_coverage row on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_adapter_stability_tokens_are_pinned() {
    assert_eq!(AdapterLaneClass::FormatterLane.as_str(), "formatter_lane");
    assert_eq!(AdapterLaneClass::LinterLane.as_str(), "linter_lane");
    assert_eq!(
        AdapterLaneClass::TestAdapterLane.as_str(),
        "test_adapter_lane"
    );
    assert_eq!(
        AdapterStabilityRowClass::AdapterStabilityQuality.as_str(),
        "adapter_stability_quality"
    );
    assert_eq!(
        AdapterStabilityRowClass::AdapterCapabilityTruth.as_str(),
        "adapter_capability_truth"
    );
    assert_eq!(
        AdapterStabilitySupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        AdapterStabilitySupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        AdapterStabilitySupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        AdapterStabilityCapabilityClass::Discover.as_str(),
        "discover"
    );
    assert_eq!(AdapterStabilityCapabilityClass::Report.as_str(), "report");
    assert_eq!(
        AdapterStabilityDegradedProviderClass::ProviderHealthy.as_str(),
        "provider_healthy"
    );
    assert_eq!(
        AdapterStabilityDegradedProviderClass::ProviderUnavailable.as_str(),
        "provider_unavailable"
    );
    assert_eq!(
        AdapterStabilityDegradedProviderClass::StateUnbound.as_str(),
        "state_unbound"
    );
    assert_eq!(AdapterStabilityOutcomeClass::Passed.as_str(), "passed");
    assert_eq!(
        AdapterStabilityOutcomeClass::OutcomeUnbound.as_str(),
        "outcome_unbound"
    );
    assert_eq!(
        AdapterStabilityLaunchWedgeClass::PythonWedge.as_str(),
        "python_wedge"
    );
    assert_eq!(
        AdapterStabilityLaunchWedgeClass::JavaKotlinWedge.as_str(),
        "java_kotlin_wedge"
    );
    assert_eq!(
        AdapterStabilityEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        AdapterStabilityKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        AdapterStabilityDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        AdapterStabilityConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        AdapterStabilityFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        AdapterStabilityFindingKind::MissingDegradedProviderCoverage.as_str(),
        "missing_degraded_provider_coverage"
    );
    assert_eq!(
        AdapterStabilityFindingKind::DegradedProviderVocabularyCollapsed.as_str(),
        "degraded_provider_vocabulary_collapsed"
    );
}
