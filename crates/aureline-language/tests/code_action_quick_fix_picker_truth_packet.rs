//! Fixture-driven coverage for the stable code-action / quick-fix picker truth
//! packet that binds, for each new M5 artifact family (framework pack, notebook
//! cell, docs artifact, request/structured artifact, config artifact, and
//! generated source), the acting provider, apply posture, mutation scope,
//! validation hook, generated-asset policy, fallback / manual path,
//! provider-disagreement visibility, and rollback checkpoint route its
//! code-action and quick-fix entries may claim. Every mutating apply states
//! whether it is inline-safe, preview-required, compare-only, or
//! blocked-pending-review; one-click inline apply never widens into generated or
//! structured artifacts without a preview; preview-required actions export a
//! preview hash and a typed completeness label; mutating applies export a
//! rollback checkpoint ref; disagreement keeps winner and loser inspectable; and
//! low-confidence providers keep their manual-fix guidance visible.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::code_action_quick_fix_picker_truth_packet::{
    current_stable_code_action_quick_fix_picker_truth_packet, ApplyPostureClass,
    ArtifactFamilyLaneClass, CodeActionQuickFixPickerTruthPacket,
    CodeActionQuickFixPickerTruthPacketInput, MutationScopeClass, PickerRowClass,
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_ARTIFACT_DOC_REF,
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF, CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR,
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_PACKET_ARTIFACT_REF,
    CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_REF,
};
use aureline_language::provider_refactor_matrix_truth_packet::{
    ConsumerSurface, ProviderFamilyClass, SupportClass,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PickerFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CodeActionQuickFixPickerTruthPacketInput,
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
    acting_provider_tokens: Vec<String>,
    apply_posture_tokens: Vec<String>,
    mutation_scope_tokens: Vec<String>,
    validation_hook_tokens: Vec<String>,
    generated_asset_policy_tokens: Vec<String>,
    fallback_path_tokens: Vec<String>,
    disagreement_visibility_tokens: Vec<String>,
    rollback_checkpoint_tokens: Vec<String>,
    preview_completeness_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> PickerFixture {
    let path = repo_root()
        .join(CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "code_action_quick_fix_picker_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = CodeActionQuickFixPickerTruthPacket::materialize(fixture.input.clone());
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
        &packet.acting_provider_tokens(),
        &expect.acting_provider_tokens,
        "acting_provider",
    );
    assert_token_set_matches(
        &packet.apply_posture_tokens(),
        &expect.apply_posture_tokens,
        "apply_posture",
    );
    assert_token_set_matches(
        &packet.mutation_scope_tokens(),
        &expect.mutation_scope_tokens,
        "mutation_scope",
    );
    assert_token_set_matches(
        &packet.validation_hook_tokens(),
        &expect.validation_hook_tokens,
        "validation_hook",
    );
    assert_token_set_matches(
        &packet.generated_asset_policy_tokens(),
        &expect.generated_asset_policy_tokens,
        "generated_asset_policy",
    );
    assert_token_set_matches(
        &packet.fallback_path_tokens(),
        &expect.fallback_path_tokens,
        "fallback_path",
    );
    assert_token_set_matches(
        &packet.disagreement_visibility_tokens(),
        &expect.disagreement_visibility_tokens,
        "disagreement_visibility",
    );
    assert_token_set_matches(
        &packet.rollback_checkpoint_tokens(),
        &expect.rollback_checkpoint_tokens,
        "rollback_checkpoint",
    );
    assert_token_set_matches(
        &packet.preview_completeness_tokens(),
        &expect.preview_completeness_tokens,
        "preview_completeness",
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
    assert_exists(CODE_ACTION_QUICK_FIX_PICKER_TRUTH_SCHEMA_REF);
    assert_exists(CODE_ACTION_QUICK_FIX_PICKER_TRUTH_DOC_REF);
    assert_exists(CODE_ACTION_QUICK_FIX_PICKER_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(CODE_ACTION_QUICK_FIX_PICKER_TRUTH_FIXTURE_DIR);
    assert_exists(CODE_ACTION_QUICK_FIX_PICKER_TRUTH_PACKET_ARTIFACT_REF);
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
fn inline_apply_widens_scope_without_preview_blocks_stable() {
    assert_fixture_matches("inline_apply_widens_scope_without_preview_blocks_stable.json");
}

#[test]
fn preview_required_without_preview_hash_blocks_stable() {
    assert_fixture_matches("preview_required_without_preview_hash_blocks_stable.json");
}

#[test]
fn mutating_action_without_checkpoint_blocks_stable() {
    assert_fixture_matches("mutating_action_without_checkpoint_blocks_stable.json");
}

#[test]
fn missing_acting_provider_label_blocks_stable() {
    assert_fixture_matches("missing_acting_provider_label_blocks_stable.json");
}

#[test]
fn disagreement_collapsed_to_ranking_only_blocks_stable() {
    assert_fixture_matches("disagreement_collapsed_to_ranking_only_blocks_stable.json");
}

#[test]
fn manual_fix_guidance_hidden_blocks_stable() {
    assert_fixture_matches("manual_fix_guidance_hidden_blocks_stable.json");
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
    let packet = current_stable_code_action_quick_fix_picker_truth_packet()
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
fn checked_in_artifact_covers_every_picker_dimension_for_every_certified_lane() {
    let packet = current_stable_code_action_quick_fix_picker_truth_packet()
        .expect("checked-in packet validates");
    let required_dimensions = [
        PickerRowClass::ApplyPostureAdmission,
        PickerRowClass::GeneratedAssetPolicyAdmission,
        PickerRowClass::FallbackPathAdmission,
        PickerRowClass::ProviderDisagreementAdmission,
        PickerRowClass::RollbackCheckpointAdmission,
    ];
    for lane in ArtifactFamilyLaneClass::REQUIRED {
        let lane_claims_certified = packet.rows.iter().any(|row| {
            row.lane_class == lane
                && row.row_class == PickerRowClass::PickerLaneQuality
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
                "certified lane {} must enumerate the {} picker dimension",
                lane.as_str(),
                dimension.as_str()
            );
        }
        // The lane's headline row must name a concrete acting provider and
        // export an acting-provider label.
        assert!(
            packet.rows.iter().any(|row| row.lane_class == lane
                && row.row_class == PickerRowClass::PickerLaneQuality
                && row.acting_provider_class != ProviderFamilyClass::NotApplicable
                && row.acting_provider_class != ProviderFamilyClass::ProviderUnbound
                && row.acting_provider_label.is_some()),
            "certified lane {} must name a concrete acting provider with a label",
            lane.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_states_apply_posture_for_every_mutating_action() {
    let packet = current_stable_code_action_quick_fix_picker_truth_packet()
        .expect("checked-in packet validates");
    for row in &packet.rows {
        if row.row_class != PickerRowClass::ApplyPostureAdmission {
            continue;
        }
        // Every apply-posture admission states one of the four postures and
        // never widens inline into protected artifacts without a preview.
        assert!(
            matches!(
                row.apply_posture_class,
                ApplyPostureClass::InlineSafe
                    | ApplyPostureClass::PreviewRequired
                    | ApplyPostureClass::CompareOnly
                    | ApplyPostureClass::BlockedPendingReview
            ),
            "apply-posture row {} must state a concrete posture",
            row.row_id
        );
        if matches!(row.apply_posture_class, ApplyPostureClass::InlineSafe) {
            assert!(
                !matches!(
                    row.mutation_scope_class,
                    MutationScopeClass::CrossArtifactScope
                        | MutationScopeClass::GeneratedArtifactScope
                        | MutationScopeClass::StructuredArtifactScope
                        | MutationScopeClass::WorkspaceWideScope
                ),
                "inline-safe apply row {} must not widen into protected artifacts without a preview",
                row.row_id
            );
        }
    }
}
