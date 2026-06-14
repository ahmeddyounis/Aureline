//! Fixture-driven coverage for the stable typed refactor transaction truth
//! packet that generalizes the launch-language refactor transaction model onto
//! the new M5 artifact families (framework pack, notebook cell, docs artifact,
//! request/structured artifact, config artifact, and generated source). Each
//! transaction is a typed transaction carrying its refactor id, acting engine,
//! target scope, missing-scope set, confidence tier, grouped hunks with impact
//! and ownership hints, a validation plan, a generated-asset policy, an apply
//! pipeline that reuses the save pipeline and mutation journal, and a rollback
//! checkpoint. The preview never overclaims completeness; the apply never
//! bypasses the save pipeline, mutation journal, or source fidelity; no
//! transform takes a privileged fast path; generated source is never treated as
//! ordinary text; and disagreement keeps the winner and loser inspectable.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::code_action_quick_fix_picker_truth_packet::ArtifactFamilyLaneClass;
use aureline_language::provider_refactor_matrix_truth_packet::{
    ConsumerSurface, ProviderFamilyClass, RefactorTransactionClass, SupportClass,
};
use aureline_language::typed_refactor_transaction_truth_packet::{
    current_stable_typed_refactor_transaction_truth_packet, ApplyPipelineClass,
    TransactionRowClass, TypedRefactorTransactionTruthPacket,
    TypedRefactorTransactionTruthPacketInput, TYPED_REFACTOR_TRANSACTION_TRUTH_ARTIFACT_DOC_REF,
    TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF, TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR,
    TYPED_REFACTOR_TRANSACTION_TRUTH_PACKET_ARTIFACT_REF,
    TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TransactionFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: TypedRefactorTransactionTruthPacketInput,
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
    target_scope_tokens: Vec<String>,
    scope_completeness_tokens: Vec<String>,
    validation_plan_tokens: Vec<String>,
    generated_asset_policy_tokens: Vec<String>,
    apply_pipeline_tokens: Vec<String>,
    rollback_checkpoint_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> TransactionFixture {
    let path = repo_root()
        .join(TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "typed_refactor_transaction_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = TypedRefactorTransactionTruthPacket::materialize(fixture.input.clone());
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
        &packet.validation_plan_tokens(),
        &expect.validation_plan_tokens,
        "validation_plan",
    );
    assert_token_set_matches(
        &packet.generated_asset_policy_tokens(),
        &expect.generated_asset_policy_tokens,
        "generated_asset_policy",
    );
    assert_token_set_matches(
        &packet.apply_pipeline_tokens(),
        &expect.apply_pipeline_tokens,
        "apply_pipeline",
    );
    assert_token_set_matches(
        &packet.rollback_checkpoint_tokens(),
        &expect.rollback_checkpoint_tokens,
        "rollback_checkpoint",
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
    assert_exists(TYPED_REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF);
    assert_exists(TYPED_REFACTOR_TRANSACTION_TRUTH_DOC_REF);
    assert_exists(TYPED_REFACTOR_TRANSACTION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(TYPED_REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR);
    assert_exists(TYPED_REFACTOR_TRANSACTION_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_target_scope_admission_blocks_stable() {
    assert_fixture_matches("missing_target_scope_admission_blocks_stable.json");
}

#[test]
fn scope_completeness_overclaimed_blocks_stable() {
    assert_fixture_matches("scope_completeness_overclaimed_blocks_stable.json");
}

#[test]
fn grouped_hunks_missing_impact_summary_blocks_stable() {
    assert_fixture_matches("grouped_hunks_missing_impact_summary_blocks_stable.json");
}

#[test]
fn validation_plan_missing_plan_ref_blocks_stable() {
    assert_fixture_matches("validation_plan_missing_plan_ref_blocks_stable.json");
}

#[test]
fn apply_pipeline_bypasses_save_pipeline_blocks_stable() {
    assert_fixture_matches("apply_pipeline_bypasses_save_pipeline_blocks_stable.json");
}

#[test]
fn apply_pipeline_bypasses_mutation_journal_blocks_stable() {
    assert_fixture_matches("apply_pipeline_bypasses_mutation_journal_blocks_stable.json");
}

#[test]
fn source_fidelity_bypassed_blocks_stable() {
    assert_fixture_matches("source_fidelity_bypassed_blocks_stable.json");
}

#[test]
fn privileged_fast_path_blocks_stable() {
    assert_fixture_matches("privileged_fast_path_blocks_stable.json");
}

#[test]
fn mutating_transaction_without_checkpoint_blocks_stable() {
    assert_fixture_matches("mutating_transaction_without_checkpoint_blocks_stable.json");
}

#[test]
fn generated_policy_bypassed_blocks_stable() {
    assert_fixture_matches("generated_policy_bypassed_blocks_stable.json");
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
fn projection_collapses_target_scope_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_target_scope_vocabulary_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_typed_refactor_transaction_truth_packet()
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
fn checked_in_artifact_covers_every_transaction_dimension_for_every_certified_lane() {
    let packet = current_stable_typed_refactor_transaction_truth_packet()
        .expect("checked-in packet validates");
    let required_dimensions = [
        TransactionRowClass::TargetScopeAdmission,
        TransactionRowClass::GroupedHunksAdmission,
        TransactionRowClass::ValidationPlanAdmission,
        TransactionRowClass::GeneratedAssetPolicyAdmission,
        TransactionRowClass::ApplyPipelineAdmission,
        TransactionRowClass::RollbackCheckpointAdmission,
        TransactionRowClass::ProviderDisagreementAdmission,
    ];
    for lane in ArtifactFamilyLaneClass::REQUIRED {
        let lane_claims_certified = packet.rows.iter().any(|row| {
            row.lane_class == lane
                && row.row_class == TransactionRowClass::TransactionLaneQuality
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
                "certified lane {} must enumerate the {} transaction dimension",
                lane.as_str(),
                dimension.as_str()
            );
        }
        // The lane's headline row must name a concrete acting engine, export an
        // engine-identity label, and bind a concrete refactor class.
        assert!(
            packet.rows.iter().any(|row| row.lane_class == lane
                && row.row_class == TransactionRowClass::TransactionLaneQuality
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
fn checked_in_artifact_keeps_every_mutating_apply_on_the_save_pipeline() {
    let packet = current_stable_typed_refactor_transaction_truth_packet()
        .expect("checked-in packet validates");
    for row in &packet.rows {
        if row.row_class != TransactionRowClass::ApplyPipelineAdmission {
            continue;
        }
        // Every apply-pipeline admission preserves source fidelity, refuses a
        // privileged fast path, and (when it mutates) reuses the save pipeline
        // and mutation journal.
        assert!(
            row.source_fidelity_preserved,
            "apply-pipeline row {} must preserve source fidelity",
            row.row_id
        );
        assert!(
            !row.privileged_fast_path,
            "apply-pipeline row {} must not take a privileged fast path",
            row.row_id
        );
        if matches!(
            row.apply_pipeline_class,
            ApplyPipelineClass::SavePipelineWithJournal
                | ApplyPipelineClass::PreviewThenSavePipeline
        ) {
            assert!(
                row.reuses_save_pipeline && row.reuses_mutation_journal,
                "mutating apply-pipeline row {} must reuse the save pipeline and mutation journal",
                row.row_id
            );
        }
    }
}
