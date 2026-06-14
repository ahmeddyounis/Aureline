//! Fixture-driven coverage for the stable wide-scope refactor fallback truth
//! packet that certifies the safe fallback posture a wide-scope or
//! low-confidence transform takes instead of an apply-all on the live
//! workspace. Each lane offers a safe fallback (side-branch, worktree, staged,
//! or compare-only) unless the transform is narrow, complete, and
//! high-confidence; carries an impact packet that preserves the missing-scope
//! explanation; routes a reviewer/owner with a review anchor; carries a safe
//! rollback path with a checkpoint ref; preserves the refactor lineage through
//! support/export; and keeps provider disagreement inspectable.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::code_action_quick_fix_picker_truth_packet::ArtifactFamilyLaneClass;
use aureline_language::provider_refactor_matrix_truth_packet::{
    ConsumerSurface, ProviderFamilyClass, RefactorTransactionClass, SupportClass,
};
use aureline_language::wide_scope_refactor_fallback_truth_packet::{
    current_stable_wide_scope_refactor_fallback_truth_packet, ApplyFallbackPostureClass,
    FallbackRowClass, WideScopeRefactorFallbackTruthPacket,
    WideScopeRefactorFallbackTruthPacketInput, WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_ARTIFACT_DOC_REF,
    WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF, WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR,
    WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_PACKET_ARTIFACT_REF,
    WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FallbackFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: WideScopeRefactorFallbackTruthPacketInput,
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
    engine_identity_tokens: Vec<String>,
    refactor_class_tokens: Vec<String>,
    apply_posture_tokens: Vec<String>,
    target_scope_tokens: Vec<String>,
    scope_completeness_tokens: Vec<String>,
    confidence_tokens: Vec<String>,
    reviewer_hint_tokens: Vec<String>,
    rollback_path_tokens: Vec<String>,
    disagreement_visibility_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> FallbackFixture {
    let path = repo_root()
        .join(WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "wide_scope_refactor_fallback_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = WideScopeRefactorFallbackTruthPacket::materialize(fixture.input.clone());
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
        &packet.engine_identity_tokens(),
        &expect.engine_identity_tokens,
        "engine_identity",
    );
    assert_token_set_matches(
        &packet.refactor_class_tokens(),
        &expect.refactor_class_tokens,
        "refactor_class",
    );
    assert_token_set_matches(
        &packet.apply_posture_tokens(),
        &expect.apply_posture_tokens,
        "apply_posture",
    );
    assert_token_set_matches(
        &packet.target_scope_tokens(),
        &expect.target_scope_tokens,
        "target_scope",
    );
    assert_token_set_matches(
        &packet.scope_completeness_tokens(),
        &expect.scope_completeness_tokens,
        "scope_completeness",
    );
    assert_token_set_matches(
        &packet.confidence_tokens(),
        &expect.confidence_tokens,
        "confidence",
    );
    assert_token_set_matches(
        &packet.reviewer_hint_tokens(),
        &expect.reviewer_hint_tokens,
        "reviewer_hint",
    );
    assert_token_set_matches(
        &packet.rollback_path_tokens(),
        &expect.rollback_path_tokens,
        "rollback_path",
    );
    assert_token_set_matches(
        &packet.disagreement_visibility_tokens(),
        &expect.disagreement_visibility_tokens,
        "disagreement_visibility",
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
        "2026-06-14T12:00:10Z",
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
    assert_exists(WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_SCHEMA_REF);
    assert_exists(WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_DOC_REF);
    assert_exists(WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_FIXTURE_DIR);
    assert_exists(WIDE_SCOPE_REFACTOR_FALLBACK_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("certified_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_apply_posture_admission_blocks_stable() {
    assert_fixture_matches("missing_apply_posture_admission_blocks_stable.json");
}

#[test]
fn unsafe_apply_all_on_wide_scope_blocks_stable() {
    assert_fixture_matches("unsafe_apply_all_on_wide_scope_blocks_stable.json");
}

#[test]
fn unsafe_apply_all_on_low_confidence_blocks_stable() {
    assert_fixture_matches("unsafe_apply_all_on_low_confidence_blocks_stable.json");
}

#[test]
fn scope_completeness_overclaimed_blocks_stable() {
    assert_fixture_matches("scope_completeness_overclaimed_blocks_stable.json");
}

#[test]
fn impact_packet_missing_summary_blocks_stable() {
    assert_fixture_matches("impact_packet_missing_summary_blocks_stable.json");
}

#[test]
fn impact_packet_missing_ref_blocks_stable() {
    assert_fixture_matches("impact_packet_missing_ref_blocks_stable.json");
}

#[test]
fn impact_packet_drops_missing_scope_blocks_stable() {
    assert_fixture_matches("impact_packet_drops_missing_scope_blocks_stable.json");
}

#[test]
fn reviewer_hint_missing_anchor_blocks_stable() {
    assert_fixture_matches("reviewer_hint_missing_anchor_blocks_stable.json");
}

#[test]
fn reviewer_hint_missing_owner_hint_blocks_stable() {
    assert_fixture_matches("reviewer_hint_missing_owner_hint_blocks_stable.json");
}

#[test]
fn writing_fallback_without_safe_rollback_blocks_stable() {
    assert_fixture_matches("writing_fallback_without_safe_rollback_blocks_stable.json");
}

#[test]
fn mutating_fallback_without_checkpoint_blocks_stable() {
    assert_fixture_matches("mutating_fallback_without_checkpoint_blocks_stable.json");
}

#[test]
fn support_export_drops_lineage_blocks_stable() {
    assert_fixture_matches("support_export_drops_lineage_blocks_stable.json");
}

#[test]
fn support_export_missing_lineage_ref_blocks_stable() {
    assert_fixture_matches("support_export_missing_lineage_ref_blocks_stable.json");
}

#[test]
fn disagreement_collapsed_to_ranking_only_blocks_stable() {
    assert_fixture_matches("disagreement_collapsed_to_ranking_only_blocks_stable.json");
}

#[test]
fn missing_engine_identity_label_blocks_stable() {
    assert_fixture_matches("missing_engine_identity_label_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn projection_collapses_apply_posture_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_apply_posture_vocabulary_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_wide_scope_refactor_fallback_truth_packet()
        .expect("checked-in packet validates");
    assert!(packet.is_stable());
    assert!(packet.validate().is_empty());
    for required in ArtifactFamilyLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include a row for artifact-family lane {}",
            required.as_str()
        );
    }
    for surface in ConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve the {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_every_fallback_dimension_for_every_certified_lane() {
    let packet = current_stable_wide_scope_refactor_fallback_truth_packet()
        .expect("checked-in packet validates");
    let required_dimensions = [
        FallbackRowClass::ApplyPostureAdmission,
        FallbackRowClass::ImpactPacketAdmission,
        FallbackRowClass::ReviewerHintAdmission,
        FallbackRowClass::RollbackPathAdmission,
        FallbackRowClass::SupportExportParityAdmission,
        FallbackRowClass::ProviderDisagreementAdmission,
    ];
    for lane in ArtifactFamilyLaneClass::REQUIRED {
        let lane_claims_certified = packet.rows.iter().any(|row| {
            row.lane_class == lane
                && row.row_class == FallbackRowClass::FallbackLaneQuality
                && row.support_class == SupportClass::Certified
        });
        if !lane_claims_certified {
            continue;
        }
        for dimension in required_dimensions {
            assert!(
                packet
                    .rows
                    .iter()
                    .any(|row| row.lane_class == lane && row.row_class == dimension),
                "certified lane {} must enumerate the {} fallback dimension",
                lane.as_str(),
                dimension.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == lane
                && row.row_class == FallbackRowClass::FallbackLaneQuality
                && row.acting_provider_class != ProviderFamilyClass::NotApplicable
                && row.acting_provider_class != ProviderFamilyClass::ProviderUnbound
                && row.refactor_class != RefactorTransactionClass::NotApplicable
                && row.engine_identity_label.is_some()),
            "certified lane {} must name a concrete acting engine and refactor class with a label",
            lane.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_defaults_wide_scope_lanes_to_safe_fallbacks() {
    let packet = current_stable_wide_scope_refactor_fallback_truth_packet()
        .expect("checked-in packet validates");
    for row in &packet.rows {
        if row.row_class != FallbackRowClass::ApplyPostureAdmission {
            continue;
        }
        // Apply-all on the live workspace is only ever permitted for a narrow,
        // complete, high-confidence transform; every other posture is a safe
        // fallback.
        if row.apply_posture_class != ApplyFallbackPostureClass::ApplyAllOnLiveWorkspace {
            assert!(
                row.apply_posture_class.is_safe_fallback(),
                "apply-posture row {} must offer a safe fallback posture",
                row.row_id
            );
        }
    }
}
