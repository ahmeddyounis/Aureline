//! Fixture-driven coverage for the stable provider/refactor matrix truth
//! packet that freezes which M5 framework, notebook, generated-source,
//! structured-artifact, and code-understanding-graph lanes may claim
//! semantic rename, previewable refactor, code-action mutation, text
//! fallback, notebook/generated bridging, or compare-only posture — each
//! bound to a provider family, capability-negotiation outcome, conflict
//! class, diagnostic source, result provenance, refactor transaction
//! class, completeness label, generated-artifact policy, allowed
//! downgrade label, and rollback posture.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_provider_refactor_matrix_truth_packet, MatrixRowClass,
    ProviderRefactorMatrixConsumerSurface, ProviderRefactorMatrixLaneClass,
    ProviderRefactorMatrixPromotionState, ProviderRefactorMatrixProviderFamilyClass,
    ProviderRefactorMatrixSupportClass, ProviderRefactorMatrixTruthPacket,
    ProviderRefactorMatrixTruthPacketInput, PROVIDER_REFACTOR_MATRIX_TRUTH_ARTIFACT_DOC_REF,
    PROVIDER_REFACTOR_MATRIX_TRUTH_DOC_REF, PROVIDER_REFACTOR_MATRIX_TRUTH_FIXTURE_DIR,
    PROVIDER_REFACTOR_MATRIX_TRUTH_PACKET_ARTIFACT_REF, PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MatrixFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: ProviderRefactorMatrixTruthPacketInput,
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
    provider_family_tokens: Vec<String>,
    capability_negotiation_tokens: Vec<String>,
    conflict_tokens: Vec<String>,
    diagnostic_source_tokens: Vec<String>,
    result_provenance_tokens: Vec<String>,
    semantic_layer_mode_tokens: Vec<String>,
    refactor_transaction_tokens: Vec<String>,
    completeness_tokens: Vec<String>,
    generated_artifact_policy_tokens: Vec<String>,
    downgrade_label_tokens: Vec<String>,
    rollback_path_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> MatrixFixture {
    let path = repo_root()
        .join(PROVIDER_REFACTOR_MATRIX_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "provider_refactor_matrix_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = ProviderRefactorMatrixTruthPacket::materialize(fixture.input.clone());
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
        &packet.provider_family_tokens(),
        &expect.provider_family_tokens,
        "provider_family",
    );
    assert_token_set_matches(
        &packet.capability_negotiation_tokens(),
        &expect.capability_negotiation_tokens,
        "capability_negotiation",
    );
    assert_token_set_matches(
        &packet.conflict_tokens(),
        &expect.conflict_tokens,
        "conflict",
    );
    assert_token_set_matches(
        &packet.diagnostic_source_tokens(),
        &expect.diagnostic_source_tokens,
        "diagnostic_source",
    );
    assert_token_set_matches(
        &packet.result_provenance_tokens(),
        &expect.result_provenance_tokens,
        "result_provenance",
    );
    assert_token_set_matches(
        &packet.semantic_layer_mode_tokens(),
        &expect.semantic_layer_mode_tokens,
        "semantic_layer_mode",
    );
    assert_token_set_matches(
        &packet.refactor_transaction_tokens(),
        &expect.refactor_transaction_tokens,
        "refactor_transaction",
    );
    assert_token_set_matches(
        &packet.completeness_tokens(),
        &expect.completeness_tokens,
        "completeness",
    );
    assert_token_set_matches(
        &packet.generated_artifact_policy_tokens(),
        &expect.generated_artifact_policy_tokens,
        "generated_artifact_policy",
    );
    assert_token_set_matches(
        &packet.downgrade_label_tokens(),
        &expect.downgrade_label_tokens,
        "downgrade_label",
    );
    assert_token_set_matches(
        &packet.rollback_path_tokens(),
        &expect.rollback_path_tokens,
        "rollback_path",
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
    assert_exists(PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_REF);
    assert_exists(PROVIDER_REFACTOR_MATRIX_TRUTH_DOC_REF);
    assert_exists(PROVIDER_REFACTOR_MATRIX_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(PROVIDER_REFACTOR_MATRIX_TRUTH_FIXTURE_DIR);
    assert_exists(PROVIDER_REFACTOR_MATRIX_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_semantic_mode_admission_blocks_stable() {
    assert_fixture_matches("missing_semantic_mode_admission_blocks_stable.json");
}

#[test]
fn mutating_refactor_without_safe_rollback_blocks_stable() {
    assert_fixture_matches("mutating_refactor_without_safe_rollback_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks_stable() {
    assert_fixture_matches("dimension_bound_on_wrong_row_class_blocks_stable.json");
}

#[test]
fn projection_collapses_provider_family_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_provider_family_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_provider_refactor_matrix_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        ProviderRefactorMatrixPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in ProviderRefactorMatrixLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include a row for artifact-family lane {}",
            required.as_str()
        );
    }
    for surface in ProviderRefactorMatrixConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve the {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_every_matrix_dimension_for_every_certified_lane() {
    let packet = current_stable_provider_refactor_matrix_truth_packet()
        .expect("checked-in packet validates");
    let required_dimensions = [
        MatrixRowClass::CapabilityNegotiationAdmission,
        MatrixRowClass::ConflictArbitrationAdmission,
        MatrixRowClass::DiagnosticSourceAdmission,
        MatrixRowClass::ResultProvenanceAdmission,
        MatrixRowClass::SemanticLayerModeAdmission,
        MatrixRowClass::RefactorTransactionAdmission,
        MatrixRowClass::GeneratedArtifactPolicyAdmission,
        MatrixRowClass::DowngradeLabelAdmission,
    ];
    for lane in ProviderRefactorMatrixLaneClass::REQUIRED {
        let lane_claims_certified = packet.rows.iter().any(|row| {
            row.lane_class == lane
                && row.row_class == MatrixRowClass::MatrixLaneQuality
                && row.support_class == ProviderRefactorMatrixSupportClass::Certified
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
                "certified lane {} must enumerate the {} matrix dimension",
                lane.as_str(),
                dimension.as_str()
            );
        }
        // The lane's headline row must name a concrete acting provider family.
        assert!(
            packet.rows.iter().any(|row| row.lane_class == lane
                && row.row_class == MatrixRowClass::MatrixLaneQuality
                && row.provider_family_class
                    != ProviderRefactorMatrixProviderFamilyClass::NotApplicable
                && row.provider_family_class
                    != ProviderRefactorMatrixProviderFamilyClass::ProviderUnbound),
            "certified lane {} must name a concrete acting provider family",
            lane.as_str()
        );
    }
}
