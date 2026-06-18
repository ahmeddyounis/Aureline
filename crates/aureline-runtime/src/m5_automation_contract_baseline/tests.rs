//! Inline unit coverage for the automation contract baseline: seed stability,
//! the six frozen object families, the reused safety-label vocabulary, the frozen
//! invariants, the projections, and the fail-closed guardrails against a missing
//! family, an incomplete or miscategorized safety-label set, a dropped
//! reused-contract ref, and a violated invariant.

use super::*;

fn seed() -> AutomationContractBaselinePacket {
    seeded_automation_contract_baseline_packet()
}

#[test]
fn seed_materializes_stable() {
    let packet = seed();
    assert!(
        packet.validate().is_empty(),
        "seed must validate clean: {:?}",
        packet.validate()
    );
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::Stable
    );
    assert!(packet.is_stable());
    assert_eq!(packet.record_kind, AUTOMATION_CONTRACT_BASELINE_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION
    );
    assert!(packet.baseline_digest.starts_with("fnv1a64:"));
}

#[test]
fn seed_carries_every_object_family() {
    let packet = seed();
    assert_eq!(
        packet.family_tokens(),
        vec![
            "recipe_builder",
            "parameter_review",
            "dry_run_explain",
            "run_history",
            "macro_recorder",
            "safety_labels",
        ]
    );
    for family in AutomationObjectFamily::ALL {
        let binding = packet.family(family).expect("family present");
        assert!(!binding.schema_ref.is_empty());
        assert!(!binding.evidence_hook_refs.is_empty());
        assert!(!binding.consumer_surfaces.is_empty());
        assert!(!binding.state_vocabulary.is_empty());
    }
}

#[test]
fn seed_carries_the_whole_reused_safety_label_vocabulary() {
    let packet = seed();
    assert_eq!(
        packet.safety_label_tokens(),
        vec![
            "macro_safe",
            "recipe_safe",
            "headless_safe",
            "ui_only",
            "approval_required",
            "writes_files",
            "runs_process",
            "network_call",
            "remote_mutation",
        ]
    );
    for label in &packet.safety_labels {
        assert_eq!(label.label_kind, label.label_id.kind());
        assert!(
            label
                .source_axis_ref
                .starts_with("schemas/automation/automation-manifest.schema.json"),
            "labels must project from the existing axis, got {}",
            label.source_axis_ref
        );
    }
}

#[test]
fn safety_labels_split_into_cues_and_effects() {
    let cues: Vec<_> = AutomationSafetyLabelId::ALL
        .into_iter()
        .filter(|label| label.kind() == SafetyLabelKind::AdmissibilityCue)
        .map(|label| label.as_str())
        .collect();
    let effects: Vec<_> = AutomationSafetyLabelId::ALL
        .into_iter()
        .filter(|label| label.kind() == SafetyLabelKind::EffectDisclosure)
        .map(|label| label.as_str())
        .collect();
    assert_eq!(
        cues,
        vec![
            "macro_safe",
            "recipe_safe",
            "headless_safe",
            "ui_only",
            "approval_required"
        ]
    );
    assert_eq!(
        effects,
        vec![
            "writes_files",
            "runs_process",
            "network_call",
            "remote_mutation"
        ]
    );
}

#[test]
fn invariants_are_all_true() {
    let packet = seed();
    for (name, value) in packet.invariants.entries() {
        assert!(value, "invariant {name} must be true");
    }
}

#[test]
fn missing_object_family_blocks_stable() {
    let mut input = current_automation_contract_baseline_input();
    input
        .object_families
        .retain(|binding| binding.family != AutomationObjectFamily::MacroRecorder);
    let packet = AutomationContractBaselinePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BaselineFindingKind::MissingObjectFamily));
}

#[test]
fn family_missing_evidence_hook_blocks_stable() {
    let mut input = current_automation_contract_baseline_input();
    input
        .object_families
        .iter_mut()
        .find(|binding| binding.family == AutomationObjectFamily::RecipeBuilder)
        .expect("recipe builder present")
        .evidence_hook_refs
        .clear();
    let packet = AutomationContractBaselinePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BaselineFindingKind::FamilyMissingEvidenceHook));
}

#[test]
fn incomplete_safety_label_set_blocks_stable() {
    let mut input = current_automation_contract_baseline_input();
    input
        .safety_labels
        .retain(|label| label.label_id != AutomationSafetyLabelId::NetworkCall);
    let packet = AutomationContractBaselinePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BaselineFindingKind::SafetyLabelSetIncomplete));
}

#[test]
fn miscategorized_safety_label_blocks_stable() {
    let mut input = current_automation_contract_baseline_input();
    input
        .safety_labels
        .iter_mut()
        .find(|label| label.label_id == AutomationSafetyLabelId::WritesFiles)
        .expect("writes_files present")
        .label_kind = SafetyLabelKind::AdmissibilityCue;
    let packet = AutomationContractBaselinePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BaselineFindingKind::SafetyLabelMiscategorized));
}

#[test]
fn missing_reused_contract_refs_blocks_stable() {
    let mut input = current_automation_contract_baseline_input();
    input.reused_contract_refs.clear();
    let packet = AutomationContractBaselinePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BaselineFindingKind::ReusedContractRefMissing));
}

#[test]
fn violated_invariant_blocks_stable() {
    let mut input = current_automation_contract_baseline_input();
    input
        .invariants
        .reruns_reresolve_current_context_never_replay_stale_authority = false;
    let packet = AutomationContractBaselinePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BaselineFindingKind::InvariantViolated));
}

#[test]
fn support_export_is_safe_and_complete() {
    let packet = seed();
    let export = packet.support_export(
        AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.family_rows.len(), AutomationObjectFamily::ALL.len());
    assert_eq!(
        export.safety_labels.len(),
        AutomationSafetyLabelId::ALL.len()
    );
    assert_eq!(
        export.promotion_state,
        AutomationBaselinePromotionState::Stable
    );
}

#[test]
fn cli_headless_view_explains_every_family_and_label() {
    let packet = seed();
    let view = packet.cli_headless_view(
        AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(view.every_family_explained());
}

#[test]
fn safety_label_manifest_carries_all_labels() {
    let packet = seed();
    let manifest =
        packet.safety_label_manifest(AUTOMATION_SAFETY_LABEL_MANIFEST_ID, "2026-06-18T00:01:00Z");
    assert_eq!(
        manifest.record_kind,
        AUTOMATION_SAFETY_LABEL_MANIFEST_RECORD_KIND
    );
    assert_eq!(manifest.labels.len(), AutomationSafetyLabelId::ALL.len());
}

#[test]
fn compact_lines_cover_packet_families_and_labels() {
    let packet = seed();
    let lines = packet.compact_lines();
    assert!(lines[0].starts_with("packet automation:m5:contract-baseline:v1"));
    assert_eq!(
        lines.len(),
        1 + AutomationObjectFamily::ALL.len() + AutomationSafetyLabelId::ALL.len()
    );
}

#[test]
fn worked_examples_roundtrip_through_serde() {
    let builder = seeded_recipe_builder_session_preview_ready();
    let json = serde_json::to_string(&builder).expect("serialize builder");
    let back: RecipeBuilderSession = serde_json::from_str(&json).expect("deserialize builder");
    assert_eq!(builder, back);

    let blocked = seeded_recipe_builder_session_blocked();
    assert_eq!(
        blocked.builder_state_class,
        RecipeBuilderStateClass::Blocked
    );
    assert_eq!(blocked.validation_findings.len(), 1);

    let sheet = seeded_parameter_review_sheet();
    assert_eq!(sheet.unresolved_required_count, 0);

    let dry_run = seeded_dry_run_explain_packet();
    assert_eq!(
        dry_run.dry_run_outcome_class,
        DryRunOutcomeClass::WouldApply
    );

    let macro_promotable = seeded_macro_session_stopped_promotable();
    assert!(macro_promotable.resulting_macro_manifest_ref.is_some());
    assert_eq!(
        macro_promotable.projected_safety_labels,
        vec![
            AutomationSafetyLabelId::MacroSafe,
            AutomationSafetyLabelId::UiOnly
        ]
    );

    let macro_discarded = seeded_macro_session_discarded();
    assert!(macro_discarded.resulting_macro_manifest_ref.is_none());
    assert_eq!(
        macro_discarded.recorder_state_class,
        MacroRecorderStateClass::Discarded
    );
}

#[test]
fn seed_is_deterministic() {
    let a = seeded_automation_contract_baseline_packet();
    let b = seeded_automation_contract_baseline_packet();
    assert_eq!(a, b);
    assert_eq!(a.baseline_digest, b.baseline_digest);
}
