//! Unit coverage for the recipe-builder object and its first-consumers packet.

use super::*;
use crate::m5_automation_contract_baseline::{AutomationSafetyLabelId, RecipeBuilderStateClass};

fn notebook_builder() -> RecipeBuilder {
    seeded_consumer_builder(RecipeBuilderEntrypoint::Notebook)
}

#[test]
fn appends_steps_in_order() {
    let builder = notebook_builder();
    assert_eq!(
        builder.step_order(),
        vec!["step:run-all".to_owned(), "step:export-html".to_owned()]
    );
}

#[test]
fn rejects_duplicate_step_ids() {
    let mut builder = notebook_builder();
    let duplicate = draft(
        "step:run-all",
        "command:notebook.run_all_cells",
        "command-rev:notebook.run_all_cells:4",
        "notebook.run_all_cells",
        &["reversible_workspace_filesystem_mutation"],
        &[AutomationSafetyLabelId::RecipeSafe],
    );
    let err = builder.append_step(duplicate, Vec::new()).unwrap_err();
    assert_eq!(
        err,
        RecipeBuilderError::DuplicateStepId("step:run-all".into())
    );
}

#[test]
fn drag_and_keyboard_reorder_converge() {
    // Drag the last step to the front.
    let mut dragged = notebook_builder();
    dragged.drag_step_to("step:export-html", 0).unwrap();

    // Reach the same order with a keyboard move-up.
    let mut keyed = notebook_builder();
    keyed.move_step_up("step:export-html").unwrap();

    assert_eq!(dragged.step_order(), keyed.step_order());
    assert_eq!(
        dragged.step_order(),
        vec!["step:export-html".to_owned(), "step:run-all".to_owned()]
    );
    // Identity is preserved: the same step ids are present, just reordered.
    assert_eq!(dragged.steps.len(), 2);
    assert!(dragged.step_index("step:run-all").is_some());
}

#[test]
fn reorder_records_provenance() {
    let mut builder = notebook_builder();
    let event = builder.drag_step_to("step:export-html", 0).unwrap();
    assert_eq!(event.gesture_kind, ReorderGestureKind::DragToIndex);
    assert_eq!(event.from_index, 1);
    assert_eq!(event.to_index, 0);
    assert_eq!(builder.reorder_log.len(), 1);
}

#[test]
fn reorder_unknown_step_errors() {
    let mut builder = notebook_builder();
    let err = builder.move_step_up("step:missing").unwrap_err();
    assert_eq!(err, RecipeBuilderError::StepNotFound("step:missing".into()));
}

#[test]
fn ui_only_step_blocks_the_builder() {
    let builder = seeded_blocked_recipe_builder();
    assert_eq!(builder.state_class(), RecipeBuilderStateClass::Blocked);
    assert!(builder.has_blocked_steps());
    let findings = builder.validation_findings();
    assert!(findings.iter().any(|finding| finding.finding_kind
        == "ui_only_command_not_recipe_safe"
        && finding.severity == "blocker"));
}

#[test]
fn approval_label_drives_approval_required_state() {
    let builder = seeded_consumer_builder(RecipeBuilderEntrypoint::Package);
    assert_eq!(
        builder.state_class(),
        RecipeBuilderStateClass::ApprovalRequired
    );
    assert!(builder
        .projected_safety_labels()
        .contains(&AutomationSafetyLabelId::ApprovalRequired));
}

#[test]
fn unresolved_slot_keeps_builder_in_draft() {
    let builder = seeded_consumer_builder(RecipeBuilderEntrypoint::RequestApi);
    assert!(builder.has_unresolved_steps());
    assert_eq!(builder.unresolved_required_count(), 1);
    assert_eq!(builder.state_class(), RecipeBuilderStateClass::Draft);
}

#[test]
fn fully_resolved_recipe_is_preview_ready() {
    let builder = notebook_builder();
    assert_eq!(builder.state_class(), RecipeBuilderStateClass::PreviewReady);
    assert!(!builder.has_unresolved_steps());
}

#[test]
fn copy_cli_and_open_docs_cite_the_same_command() {
    let builder = notebook_builder();
    assert!(builder.parity_holds());
    for step in &builder.steps {
        assert!(step.copy_cli.contains(step.canonical_verb()));
        assert!(step.open_docs.ends_with(&format!(
            "#{}",
            slugify_canonical_verb(step.canonical_verb())
        )));
    }
}

#[test]
fn session_record_reuses_command_truth() {
    let builder = notebook_builder();
    let session = builder.to_session_record();
    assert_eq!(session.record_kind, RECIPE_BUILDER_SESSION_RECORD_KIND);
    assert_eq!(session.step_drafts.len(), builder.steps.len());
    for (draft, step) in session.step_drafts.iter().zip(&builder.steps) {
        assert_eq!(draft.command_id, step.draft.command_id);
        assert_eq!(draft.canonical_verb, step.draft.canonical_verb);
    }
    assert_eq!(
        session.manifest_target_schema_ref,
        RECIPE_MANIFEST_SCHEMA_REF
    );
}

#[test]
fn export_round_trips_to_an_equal_builder() {
    let mut builder = notebook_builder();
    builder.drag_step_to("step:export-html", 0).unwrap();
    let export = builder.export("export:test", "2026-06-18T00:01:00Z");
    let imported = export.import();
    assert_eq!(imported, builder);
    assert!(export.provenance_preserved());
    assert_eq!(export.record_kind, RECIPE_BUILDER_EXPORT_RECORD_KIND);
    // The reorder log survives the round trip.
    assert_eq!(imported.reorder_log.len(), 1);
}

#[test]
fn seeded_packet_is_stable() {
    let packet = seeded_recipe_builder_first_consumers_packet();
    assert!(packet.is_stable());
    assert!(packet.validation_findings.is_empty());
    assert_eq!(
        packet.consumer_bindings.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        assert!(packet.binding(entrypoint).is_some());
    }
}

#[test]
fn missing_entrypoint_blocks_stable() {
    let mut input = current_recipe_builder_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
    let packet = RecipeBuilderFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FirstConsumersFindingKind::MissingEntrypoint));
}

#[test]
fn ui_only_step_not_blocked_blocks_stable() {
    let mut input = current_recipe_builder_first_consumers_input();
    let binding = RecipeBuilderConsumerBinding::from_builder(&seeded_blocked_recipe_builder());
    // Forge an inadmissible packet: keep the UI-only step but claim preview-ready.
    let mut forged = binding;
    forged.builder_state_class = RecipeBuilderStateClass::PreviewReady;
    forged.session_record.builder_state_class = RecipeBuilderStateClass::PreviewReady;
    input
        .consumer_bindings
        .retain(|b| b.entrypoint != RecipeBuilderEntrypoint::TaskTestDebug);
    input.consumer_bindings.push(forged);
    let packet = RecipeBuilderFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FirstConsumersFindingKind::UiOnlyStepNotBlocked));
}

#[test]
fn broken_cli_docs_parity_blocks_stable() {
    let mut input = current_recipe_builder_first_consumers_input();
    let binding = input
        .consumer_bindings
        .iter_mut()
        .find(|b| b.entrypoint == RecipeBuilderEntrypoint::Notebook)
        .expect("notebook binding");
    binding.copy_cli_lines[0] = "aureline command run wrong.verb".to_owned();
    let packet = RecipeBuilderFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FirstConsumersFindingKind::CliDocsParityBroken));
}

#[test]
fn non_declarative_manifest_blocks_stable() {
    let mut input = current_recipe_builder_first_consumers_input();
    let binding = input
        .consumer_bindings
        .iter_mut()
        .find(|b| b.entrypoint == RecipeBuilderEntrypoint::Notebook)
        .expect("notebook binding");
    binding.session_record.manifest_target_schema_ref =
        "schemas/automation/shell_script.schema.json".to_owned();
    let packet = RecipeBuilderFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == FirstConsumersFindingKind::NonDeclarativeManifestTarget));
}

#[test]
fn violated_invariant_blocks_stable() {
    let mut input = current_recipe_builder_first_consumers_input();
    input
        .invariants
        .builder_reuses_command_truth_not_private_form_state = false;
    let packet = RecipeBuilderFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == FirstConsumersFindingKind::InvariantViolated));
}

#[test]
fn support_export_and_cli_view_are_consistent() {
    let packet = seeded_recipe_builder_first_consumers_packet();
    let export = packet.support_export("support:test", "2026-06-18T00:02:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id, packet.packet_id);
    assert_eq!(export.consumer_rows.len(), packet.consumer_bindings.len());

    let view = packet.cli_headless_view("cli:test", "2026-06-18T00:02:00Z");
    assert!(view.every_entrypoint_explained());
}

#[test]
fn digest_is_order_invariant() {
    let packet = seeded_recipe_builder_first_consumers_packet();
    let mut shuffled = current_recipe_builder_first_consumers_input();
    shuffled.consumer_bindings.reverse();
    let reshuffled = RecipeBuilderFirstConsumersPacket::materialize(shuffled);
    assert_eq!(packet.packet_digest, reshuffled.packet_digest);
}
