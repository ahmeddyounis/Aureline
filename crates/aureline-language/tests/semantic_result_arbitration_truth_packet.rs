//! Fixture-driven coverage for the stable semantic-result arbitration truth
//! packet that binds the arbitration inspector, disagreement detail, and
//! semantic-to-text fallback banner for definition, references, hierarchy, and
//! completion results across the M5 search, docs, framework, notebook, and
//! generated-source surfaces. Each row names the acting provider that won and
//! the basis it won on, keeps the losing providers inspectable, surfaces a
//! visible detail path whenever a conflict changes target identity / scope /
//! refactor safety, and carries a fallback banner that records both the
//! guarantee retained and the guarantee lost whenever the answer degraded.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_semantic_result_arbitration_truth_packet, ResultLaneClass, ResultSurfaceClass,
    SemanticResultArbitrationTruthPacket, SemanticResultArbitrationTruthPacketInput,
    SEMANTIC_RESULT_ARBITRATION_TRUTH_ARTIFACT_DOC_REF, SEMANTIC_RESULT_ARBITRATION_TRUTH_DOC_REF,
    SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR,
    SEMANTIC_RESULT_ARBITRATION_TRUTH_PACKET_ARTIFACT_REF,
    SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResultFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SemanticResultArbitrationTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    result_surface_tokens: Vec<String>,
    result_lane_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    provider_family_tokens: Vec<String>,
    arbitration_basis_tokens: Vec<String>,
    alternate_provider_visibility_tokens: Vec<String>,
    inspector_route_tokens: Vec<String>,
    conflict_tokens: Vec<String>,
    disagreement_impact_tokens: Vec<String>,
    disagreement_visibility_tokens: Vec<String>,
    result_tier_tokens: Vec<String>,
    fallback_banner_tokens: Vec<String>,
    retained_guarantee_tokens: Vec<String>,
    lost_guarantee_tokens: Vec<String>,
    claim_scope_tokens: Vec<String>,
    coverage_gap_tokens: Vec<String>,
    anchor_action_tokens: Vec<String>,
    completeness_tokens: Vec<String>,
    evidence_class_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ResultFixture {
    let path = repo_root()
        .join(SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "semantic_result_arbitration_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = SemanticResultArbitrationTruthPacket::materialize(fixture.input.clone());
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

    assert_token_set_matches(
        &packet.result_surface_tokens(),
        &expect.result_surface_tokens,
        "result_surface",
    );
    assert_token_set_matches(
        &packet.result_lane_tokens(),
        &expect.result_lane_tokens,
        "result_lane",
    );
    assert_token_set_matches(
        &packet.support_class_tokens(),
        &expect.support_class_tokens,
        "support_class",
    );
    assert_token_set_matches(
        &packet.provider_family_tokens(),
        &expect.provider_family_tokens,
        "provider_family",
    );
    assert_token_set_matches(
        &packet.arbitration_basis_tokens(),
        &expect.arbitration_basis_tokens,
        "arbitration_basis",
    );
    assert_token_set_matches(
        &packet.alternate_provider_visibility_tokens(),
        &expect.alternate_provider_visibility_tokens,
        "alternate_provider_visibility",
    );
    assert_token_set_matches(
        &packet.inspector_route_tokens(),
        &expect.inspector_route_tokens,
        "inspector_route",
    );
    assert_token_set_matches(
        &packet.conflict_tokens(),
        &expect.conflict_tokens,
        "conflict",
    );
    assert_token_set_matches(
        &packet.disagreement_impact_tokens(),
        &expect.disagreement_impact_tokens,
        "disagreement_impact",
    );
    assert_token_set_matches(
        &packet.disagreement_visibility_tokens(),
        &expect.disagreement_visibility_tokens,
        "disagreement_visibility",
    );
    assert_token_set_matches(
        &packet.result_tier_tokens(),
        &expect.result_tier_tokens,
        "result_tier",
    );
    assert_token_set_matches(
        &packet.fallback_banner_tokens(),
        &expect.fallback_banner_tokens,
        "fallback_banner",
    );
    assert_token_set_matches(
        &packet.retained_guarantee_tokens(),
        &expect.retained_guarantee_tokens,
        "retained_guarantee",
    );
    assert_token_set_matches(
        &packet.lost_guarantee_tokens(),
        &expect.lost_guarantee_tokens,
        "lost_guarantee",
    );
    assert_token_set_matches(
        &packet.claim_scope_tokens(),
        &expect.claim_scope_tokens,
        "claim_scope",
    );
    assert_token_set_matches(
        &packet.coverage_gap_tokens(),
        &expect.coverage_gap_tokens,
        "coverage_gap",
    );
    assert_token_set_matches(
        &packet.anchor_action_tokens(),
        &expect.anchor_action_tokens,
        "anchor_action",
    );
    assert_token_set_matches(
        &packet.completeness_tokens(),
        &expect.completeness_tokens,
        "completeness",
    );
    assert_token_set_matches(
        &packet.evidence_class_tokens(),
        &expect.evidence_class_tokens,
        "evidence_class",
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
                "fixture {} expected finding kind {kind}; observed {observed:?}",
                fixture.case_name,
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_REF);
    assert_exists(SEMANTIC_RESULT_ARBITRATION_TRUTH_DOC_REF);
    assert_exists(SEMANTIC_RESULT_ARBITRATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR);
    assert_exists(SEMANTIC_RESULT_ARBITRATION_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn losing_provider_collapsed_blocks_stable() {
    assert_fixture_matches("losing_provider_collapsed_blocks_stable.json");
}

#[test]
fn material_conflict_without_detail_path_blocks_stable() {
    assert_fixture_matches("material_conflict_without_detail_path_blocks_stable.json");
}

#[test]
fn opaque_spinner_route_blocks_stable() {
    assert_fixture_matches("opaque_spinner_route_blocks_stable.json");
}

#[test]
fn silently_fused_conflict_blocks_stable() {
    assert_fixture_matches("silently_fused_conflict_blocks_stable.json");
}

#[test]
fn fallback_banner_missing_blocks_stable() {
    assert_fixture_matches("fallback_banner_missing_blocks_stable.json");
}

#[test]
fn exact_result_with_fallback_banner_blocks_stable() {
    assert_fixture_matches("exact_result_with_fallback_banner_blocks_stable.json");
}

#[test]
fn overclaimed_all_references_on_lexical_blocks_stable() {
    assert_fixture_matches("overclaimed_all_references_on_lexical_blocks_stable.json");
}

#[test]
fn whole_workspace_wording_with_excluded_roots_blocks_stable() {
    assert_fixture_matches("whole_workspace_wording_with_excluded_roots_blocks_stable.json");
}

#[test]
fn mutating_followup_without_rollback_blocks_stable() {
    assert_fixture_matches("mutating_followup_without_rollback_blocks_stable.json");
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("certified_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_surface_and_lane() {
    let packet = current_stable_semantic_result_arbitration_truth_packet()
        .expect("checked-in packet validates");
    assert!(packet.validate().is_empty());
    for surface in ResultSurfaceClass::REQUIRED {
        assert!(
            packet
                .rows
                .iter()
                .any(|row| row.result_surface_class == surface),
            "stable packet must cover surface {}",
            surface.as_str()
        );
    }
    for lane in ResultLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.result_lane_class == lane),
            "stable packet must cover lane {}",
            lane.as_str()
        );
    }
}
