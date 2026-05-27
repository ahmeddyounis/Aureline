//! Fixture-driven coverage for the stable
//! harden coverage / flaky-test / snapshot-golden / baseline-truth
//! packet covering the coverage, flaky_test, snapshot_golden, and
//! baseline_truth lanes plus the four-wedge admission coverage, the
//! six stability-verdict admissions, the four quarantine-mute state
//! admissions, the four test-source admissions, the four
//! coverage-impact admissions, the three candidate-lineage
//! admissions, the five consumer-surface bindings, and the
//! lineage_admission row binding `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_coverage_quality_truth_packet, CoverageQualityCandidateLineageClass,
    CoverageQualityConsumerSurface, CoverageQualityConsumerSurfaceBindingClass,
    CoverageQualityCoverageImpactClass, CoverageQualityLaneClass, CoverageQualityPromotionState,
    CoverageQualityQuarantineMuteStateClass, CoverageQualityRowClass,
    CoverageQualityStabilityVerdictClass, CoverageQualitySupportClass,
    CoverageQualityTestSourceClass, CoverageQualityTruthPacket, CoverageQualityTruthPacketInput,
    CoverageQualityWedgeClass, COVERAGE_QUALITY_TRUTH_ARTIFACT_DOC_REF,
    COVERAGE_QUALITY_TRUTH_DOC_REF, COVERAGE_QUALITY_TRUTH_FIXTURE_DIR,
    COVERAGE_QUALITY_TRUTH_PACKET_ARTIFACT_REF, COVERAGE_QUALITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CoverageQualityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CoverageQualityTruthPacketInput,
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
    stability_verdict_tokens: Vec<String>,
    quarantine_mute_state_tokens: Vec<String>,
    test_source_tokens: Vec<String>,
    coverage_impact_tokens: Vec<String>,
    candidate_lineage_tokens: Vec<String>,
    consumer_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> CoverageQualityFixture {
    let path = repo_root()
        .join(COVERAGE_QUALITY_TRUTH_FIXTURE_DIR)
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
        "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = CoverageQualityTruthPacket::materialize(fixture.input.clone());
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
        &packet.stability_verdict_tokens(),
        &expect.stability_verdict_tokens,
        "stability_verdict",
    );
    assert_token_set_matches(
        &packet.quarantine_mute_state_tokens(),
        &expect.quarantine_mute_state_tokens,
        "quarantine_mute_state",
    );
    assert_token_set_matches(
        &packet.test_source_tokens(),
        &expect.test_source_tokens,
        "test_source",
    );
    assert_token_set_matches(
        &packet.coverage_impact_tokens(),
        &expect.coverage_impact_tokens,
        "coverage_impact",
    );
    assert_token_set_matches(
        &packet.candidate_lineage_tokens(),
        &expect.candidate_lineage_tokens,
        "candidate_lineage",
    );
    assert_token_set_matches(
        &packet.consumer_surface_tokens(),
        &expect.consumer_surface_tokens,
        "consumer_surface",
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
    assert_exists(COVERAGE_QUALITY_TRUTH_SCHEMA_REF);
    assert_exists(COVERAGE_QUALITY_TRUTH_DOC_REF);
    assert_exists(COVERAGE_QUALITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(COVERAGE_QUALITY_TRUTH_FIXTURE_DIR);
    assert_exists(COVERAGE_QUALITY_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_stability_verdict_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_stability_verdict_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_quarantine_mute_state_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_quarantine_mute_state_for_launch_stable_blocks_stable.json");
}

#[test]
fn candidate_ai_test_without_lineage_blocks_stable() {
    assert_fixture_matches("candidate_ai_test_without_lineage_blocks_stable.json");
}

#[test]
fn missing_coverage_impact_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_coverage_impact_for_launch_stable_blocks_stable.json");
}

#[test]
fn consumer_surface_missing_candidate_lineage_attestation_blocks_stable() {
    assert_fixture_matches(
        "consumer_surface_missing_candidate_lineage_attestation_blocks_stable.json",
    );
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_coverage_impact_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_coverage_impact_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet =
        current_stable_coverage_quality_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        CoverageQualityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in CoverageQualityLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for coverage-quality lane {}",
            required.as_str()
        );
    }
    for surface in CoverageQualityConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_admissions_per_launch_stable_lane() {
    let packet =
        current_stable_coverage_quality_truth_packet().expect("checked-in packet validates");
    for required in CoverageQualityLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == CoverageQualityRowClass::CoverageFlakySnapshotBaselineQuality
                && row.support_class == CoverageQualitySupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in CoverageQualityWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for verdict in CoverageQualityStabilityVerdictClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::StabilityVerdictAdmission
                    && row.stability_verdict_class == verdict),
                "stable packet must cover the {} stability-verdict admission on the {} lane",
                verdict.as_str(),
                required.as_str()
            );
        }
        for state in CoverageQualityQuarantineMuteStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::QuarantineMuteStateAdmission
                    && row.quarantine_mute_state_class == state),
                "stable packet must cover the {} quarantine-mute-state admission on the {} lane",
                state.as_str(),
                required.as_str()
            );
        }
        for source in CoverageQualityTestSourceClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::TestSourceAdmission
                    && row.test_source_class == source
                    && (!source.requires_candidate_lineage()
                        || row.attests_candidate_lineage_bound)),
                "stable packet must cover the {} test-source admission on the {} lane with required lineage attestation",
                source.as_str(),
                required.as_str()
            );
        }
        for impact in CoverageQualityCoverageImpactClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::CoverageImpactAdmission
                    && row.coverage_impact_class == impact),
                "stable packet must cover the {} coverage-impact admission on the {} lane",
                impact.as_str(),
                required.as_str()
            );
        }
        for lineage in CoverageQualityCandidateLineageClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::CandidateLineageAdmission
                    && row.candidate_lineage_class == lineage),
                "stable packet must cover the {} candidate-lineage admission on the {} lane",
                lineage.as_str(),
                required.as_str()
            );
        }
        for surface in CoverageQualityConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == CoverageQualityRowClass::ConsumerSurfaceBinding
                    && row.consumer_surface_class == surface
                    && (!surface.requires_stability_verdict_attestation()
                        || row.attests_stability_verdict_preserved)
                    && (!surface.requires_quarantine_mute_state_attestation()
                        || row.attests_quarantine_mute_state_preserved)
                    && (!surface.requires_test_source_attestation()
                        || row.attests_test_source_preserved)
                    && (!surface.requires_coverage_impact_attestation()
                        || row.attests_coverage_impact_preserved)
                    && (!surface.requires_candidate_lineage_attestation()
                        || row.attests_candidate_lineage_preserved)),
                "stable packet must cover the {} consumer surface binding on the {} lane with required attestations",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == CoverageQualityRowClass::LineageAdmission
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
fn closed_coverage_quality_truth_tokens_are_pinned() {
    assert_eq!(
        CoverageQualityLaneClass::CoverageLane.as_str(),
        "coverage_lane"
    );
    assert_eq!(
        CoverageQualityLaneClass::BaselineTruthLane.as_str(),
        "baseline_truth_lane"
    );
    assert_eq!(
        CoverageQualityRowClass::CoverageFlakySnapshotBaselineQuality.as_str(),
        "coverage_flaky_snapshot_baseline_quality"
    );
    assert_eq!(
        CoverageQualitySupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        CoverageQualityWedgeClass::AiCandidateSourceAttribution.as_str(),
        "ai_candidate_source_attribution"
    );
    assert_eq!(
        CoverageQualityStabilityVerdictClass::Quarantined.as_str(),
        "quarantined"
    );
    assert_eq!(
        CoverageQualityStabilityVerdictClass::Muted.as_str(),
        "muted"
    );
    assert_eq!(
        CoverageQualityQuarantineMuteStateClass::ExpiredPendingRenewal.as_str(),
        "expired_pending_renewal"
    );
    assert_eq!(
        CoverageQualityTestSourceClass::CandidateAiTest.as_str(),
        "candidate_ai_test"
    );
    assert_eq!(
        CoverageQualityTestSourceClass::ImportedCiEvidence.as_str(),
        "imported_ci_evidence"
    );
    assert_eq!(
        CoverageQualityCoverageImpactClass::NotComparable.as_str(),
        "not_comparable"
    );
    assert_eq!(
        CoverageQualityCandidateLineageClass::ReviewCheckpointBound.as_str(),
        "review_checkpoint_bound"
    );
    assert_eq!(
        CoverageQualityConsumerSurfaceBindingClass::ReleasePacketSurface.as_str(),
        "release_packet_surface"
    );
    assert_eq!(
        CoverageQualityConsumerSurface::AiToolSurface.as_str(),
        "ai_tool_surface"
    );
}
