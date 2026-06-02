//! Fixture-driven coverage for the stable assistant-surface hardening
//! truth packet covering completion, signature help, snippet session,
//! code action, additional edit, source labeling, and AI ghost text
//! assistant-surface lanes with cross-cut condition coverage (IME,
//! multi-cursor, large-file, restricted-mode, degraded-provider),
//! provider/source bindings (deterministic completion, cached/local
//! word fallback, snippet-only suggestion, AI ghost text), snippet
//! session field truth (label/source, placeholder index/count, exit
//! route, multi-cursor compatibility), code action field truth
//! (provider/source, side-effect class, partial-support reason,
//! preview requirement), additional-edit side-effect admission, known
//! limits, downgrade automation, and evidence binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_assistant_surface_hardening_truth_packet, AssistantSurfaceCodeActionFieldClass,
    AssistantSurfaceCrossCutConditionClass, AssistantSurfaceDowngradeAutomationClass,
    AssistantSurfaceEvidenceClass, AssistantSurfaceHardeningConsumerSurface,
    AssistantSurfaceHardeningFindingKind, AssistantSurfaceHardeningPromotionState,
    AssistantSurfaceHardeningTruthPacket, AssistantSurfaceHardeningTruthPacketInput,
    AssistantSurfaceKnownLimitClass, AssistantSurfaceLaneClass,
    AssistantSurfacePreviewRequirementClass, AssistantSurfaceProviderSourceClass,
    AssistantSurfaceRowClass, AssistantSurfaceSideEffectClass,
    AssistantSurfaceSnippetSessionFieldClass, AssistantSurfaceSupportClass,
    ASSISTANT_SURFACE_HARDENING_TRUTH_ARTIFACT_DOC_REF, ASSISTANT_SURFACE_HARDENING_TRUTH_DOC_REF,
    ASSISTANT_SURFACE_HARDENING_TRUTH_FIXTURE_DIR,
    ASSISTANT_SURFACE_HARDENING_TRUTH_PACKET_ARTIFACT_REF,
    ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AssistantSurfaceFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: AssistantSurfaceHardeningTruthPacketInput,
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
    provider_source_class_tokens: Vec<String>,
    side_effect_class_tokens: Vec<String>,
    preview_requirement_tokens: Vec<String>,
    snippet_session_field_tokens: Vec<String>,
    code_action_field_tokens: Vec<String>,
    cross_cut_condition_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> AssistantSurfaceFixture {
    let path = repo_root()
        .join(ASSISTANT_SURFACE_HARDENING_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "assistant_surface_hardening_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = AssistantSurfaceHardeningTruthPacket::materialize(fixture.input.clone());
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
        &packet.provider_source_class_tokens(),
        &expect.provider_source_class_tokens,
        "provider_source_class",
    );
    assert_token_set_matches(
        &packet.side_effect_class_tokens(),
        &expect.side_effect_class_tokens,
        "side_effect_class",
    );
    assert_token_set_matches(
        &packet.preview_requirement_tokens(),
        &expect.preview_requirement_tokens,
        "preview_requirement",
    );
    assert_token_set_matches(
        &packet.snippet_session_field_tokens(),
        &expect.snippet_session_field_tokens,
        "snippet_session_field",
    );
    assert_token_set_matches(
        &packet.code_action_field_tokens(),
        &expect.code_action_field_tokens,
        "code_action_field",
    );
    assert_token_set_matches(
        &packet.cross_cut_condition_tokens(),
        &expect.cross_cut_condition_tokens,
        "cross_cut_condition",
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
    assert_exists(ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_REF);
    assert_exists(ASSISTANT_SURFACE_HARDENING_TRUTH_DOC_REF);
    assert_exists(ASSISTANT_SURFACE_HARDENING_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(ASSISTANT_SURFACE_HARDENING_TRUTH_FIXTURE_DIR);
    assert_exists(ASSISTANT_SURFACE_HARDENING_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn launch_hardened_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("launch_hardened_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_cross_cut_condition_for_launch_hardened_blocks_stable() {
    assert_fixture_matches("missing_cross_cut_condition_for_launch_hardened_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_provider_source_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_provider_source_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_assistant_surface_hardening_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        AssistantSurfaceHardeningPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in AssistantSurfaceLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for assistant-surface lane {}",
            required.as_str()
        );
    }
    for surface in AssistantSurfaceHardeningConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_cross_cut_conditions_for_every_launch_hardened_lane() {
    let packet = current_stable_assistant_surface_hardening_truth_packet()
        .expect("checked-in packet validates");
    for required in AssistantSurfaceLaneClass::REQUIRED {
        let lane_claims_hardened = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == AssistantSurfaceRowClass::AssistantSurfaceQuality
                && row.support_class == AssistantSurfaceSupportClass::LaunchHardened
        });
        if !lane_claims_hardened {
            continue;
        }
        for condition in AssistantSurfaceCrossCutConditionClass::REQUIRED_FOR_LAUNCH {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == AssistantSurfaceRowClass::CrossCutCondition
                    && row.cross_cut_condition_class == condition),
                "stable packet must cover the {} cross-cut condition on the {} lane",
                condition.as_str(),
                required.as_str()
            );
        }
    }
}

#[test]
fn checked_in_artifact_covers_provider_source_bindings_for_completion_and_ai_lanes() {
    let packet = current_stable_assistant_surface_hardening_truth_packet()
        .expect("checked-in packet validates");
    let provider_lanes = [
        AssistantSurfaceLaneClass::CompletionLane,
        AssistantSurfaceLaneClass::SourceLabelingLane,
        AssistantSurfaceLaneClass::AiGhostTextLane,
    ];
    for lane in provider_lanes {
        for provider in AssistantSurfaceProviderSourceClass::REQUIRED_FOR_PROVIDER_BINDING {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == lane
                    && row.row_class == AssistantSurfaceRowClass::ProviderSourceBinding
                    && row.provider_source_class == provider),
                "stable packet must bind provider/source {} on the {} lane",
                provider.as_str(),
                lane.as_str()
            );
        }
    }
}

#[test]
fn checked_in_artifact_covers_snippet_session_truth_fields() {
    let packet = current_stable_assistant_surface_hardening_truth_packet()
        .expect("checked-in packet validates");
    for field in AssistantSurfaceSnippetSessionFieldClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class
                == AssistantSurfaceLaneClass::SnippetSessionLane
                && row.row_class == AssistantSurfaceRowClass::SnippetSessionTruth
                && row.snippet_session_field_class == field),
            "stable packet must surface snippet-session field {}",
            field.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_code_action_truth_fields() {
    let packet = current_stable_assistant_surface_hardening_truth_packet()
        .expect("checked-in packet validates");
    for field in AssistantSurfaceCodeActionFieldClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class
                == AssistantSurfaceLaneClass::CodeActionLane
                && row.row_class == AssistantSurfaceRowClass::CodeActionTruth
                && row.code_action_field_class == field),
            "stable packet must preserve code-action field {}",
            field.as_str()
        );
    }
}

#[test]
fn closed_assistant_surface_tokens_are_pinned() {
    assert_eq!(
        AssistantSurfaceLaneClass::CompletionLane.as_str(),
        "completion_lane"
    );
    assert_eq!(
        AssistantSurfaceLaneClass::AiGhostTextLane.as_str(),
        "ai_ghost_text_lane"
    );
    assert_eq!(
        AssistantSurfaceRowClass::AssistantSurfaceQuality.as_str(),
        "assistant_surface_quality"
    );
    assert_eq!(
        AssistantSurfaceSupportClass::LaunchHardened.as_str(),
        "launch_hardened"
    );
    assert_eq!(
        AssistantSurfaceSupportClass::LaunchHardenedBelow.as_str(),
        "launch_hardened_below"
    );
    assert_eq!(
        AssistantSurfaceSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        AssistantSurfaceProviderSourceClass::DeterministicCompletion.as_str(),
        "deterministic_completion"
    );
    assert_eq!(
        AssistantSurfaceProviderSourceClass::AiGhostText.as_str(),
        "ai_ghost_text"
    );
    assert_eq!(
        AssistantSurfaceSideEffectClass::AdditionalEdits.as_str(),
        "additional_edits"
    );
    assert_eq!(
        AssistantSurfaceSideEffectClass::ProtectedSurfaceWrite.as_str(),
        "protected_surface_write"
    );
    assert_eq!(
        AssistantSurfacePreviewRequirementClass::PreviewRequiredForMultiFile.as_str(),
        "preview_required_for_multi_file"
    );
    assert_eq!(
        AssistantSurfaceSnippetSessionFieldClass::ExitRoute.as_str(),
        "exit_route"
    );
    assert_eq!(
        AssistantSurfaceCodeActionFieldClass::PartialSupportReason.as_str(),
        "partial_support_reason"
    );
    assert_eq!(
        AssistantSurfaceCrossCutConditionClass::DegradedProvider.as_str(),
        "degraded_provider"
    );
    assert_eq!(
        AssistantSurfaceEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        AssistantSurfaceKnownLimitClass::SideEffectScopeOnly.as_str(),
        "side_effect_scope_only"
    );
    assert_eq!(
        AssistantSurfaceKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        AssistantSurfaceDowngradeAutomationClass::AutoNarrowOnGhostTextLabelDrift.as_str(),
        "auto_narrow_on_ghost_text_label_drift"
    );
    assert_eq!(
        AssistantSurfaceDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        AssistantSurfaceHardeningConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        AssistantSurfaceHardeningFindingKind::LaunchHardenedWithUnboundBinding.as_str(),
        "launch_hardened_with_unbound_binding"
    );
    assert_eq!(
        AssistantSurfaceHardeningFindingKind::MissingCrossCutConditionCoverage.as_str(),
        "missing_cross_cut_condition_coverage"
    );
    assert_eq!(
        AssistantSurfaceHardeningFindingKind::ProviderSourceClassVocabularyCollapsed.as_str(),
        "provider_source_class_vocabulary_collapsed"
    );
}
