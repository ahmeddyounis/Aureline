//! Unit coverage for the dry-run/explain preview and its first-consumers packet.

use super::*;
use crate::m5_automation_contract_baseline::{AutomationSafetyLabelId, DryRunOutcomeClass};

fn notebook_preview() -> DryRunExplainPreview {
    seeded_consumer_preview(RecipeBuilderEntrypoint::Notebook)
}

#[test]
fn adds_actions_in_order() {
    let preview = notebook_preview();
    let ids: Vec<&str> = preview
        .actions
        .iter()
        .map(|action| action.step_id.as_str())
        .collect();
    assert_eq!(ids, vec!["step:run-cells", "step:write-export"]);
}

#[test]
fn rejects_duplicate_step_ids() {
    let mut preview = notebook_preview();
    let duplicate = action(
        "step:write-export",
        "notebook.export_rendered",
        "duplicate",
        SideEffectClass::PredictedWrite,
        IdempotenceClass::Idempotent,
        true,
        vec![write(WriteKind::CreateFile, "path:dup", true, "dup")],
        vec![],
        vec![],
        &[],
        "dup",
    );
    let err = preview.add_action(duplicate).unwrap_err();
    assert_eq!(
        err,
        DryRunExplainError::DuplicateStepId("step:write-export".into())
    );
}

#[test]
fn every_entrypoint_binds_a_consistent_preview() {
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let preview = seeded_consumer_preview(entrypoint);
        assert!(
            !preview.actions.is_empty(),
            "{} has actions",
            entrypoint.as_str()
        );
        assert!(
            preview.every_action_side_effect_consistent(),
            "{} side effects consistent",
            entrypoint.as_str()
        );
        // A predicted write always declares at least one write.
        for action in &preview.actions {
            if action.side_effect_class == SideEffectClass::PredictedWrite {
                assert!(!action.predicted_writes.is_empty());
            }
        }
    }
}

#[test]
fn predicted_write_action_declares_its_writes() {
    let preview = notebook_preview();
    let export = preview.action("step:write-export").expect("write-export");
    assert_eq!(export.side_effect_class, SideEffectClass::PredictedWrite);
    assert_eq!(export.predicted_writes.len(), 1);
    assert!(export
        .projected_safety_labels()
        .contains(&AutomationSafetyLabelId::WritesFiles));
}

#[test]
fn read_only_inspection_declares_no_write() {
    let preview = notebook_preview();
    let run = preview.action("step:run-cells").expect("run-cells");
    assert_eq!(run.side_effect_class, SideEffectClass::ReadOnlyInspection);
    assert!(run.predicted_writes.is_empty());
    assert!(run.side_effect_consistent());
    assert!(run.projected_safety_labels().is_empty());
}

#[test]
fn mutating_action_mislabeled_read_only_is_inconsistent() {
    let mut preview = notebook_preview();
    // Force the write action to claim read-only while keeping its write.
    preview.actions[1].side_effect_class = SideEffectClass::ReadOnlyInspection;
    assert!(!preview.actions[1].side_effect_consistent());
    assert!(!preview.every_action_side_effect_consistent());
}

#[test]
fn outcome_is_derived_from_blockers_and_posture() {
    // Notebook: writes only, no gate -> would apply.
    assert_eq!(
        notebook_preview().dry_run_outcome_class(),
        DryRunOutcomeClass::WouldApply
    );
    // Request: network call with an approval gate -> would apply under approval.
    assert_eq!(
        seeded_consumer_preview(RecipeBuilderEntrypoint::RequestApi).dry_run_outcome_class(),
        DryRunOutcomeClass::WouldApplyUnderApproval
    );
    // Incident: remote runbook denied at a trust gate -> would be denied.
    assert_eq!(
        seeded_blocked_preview().dry_run_outcome_class(),
        DryRunOutcomeClass::WouldBeDeniedAtGate
    );
}

#[test]
fn aggregate_labels_union_portability_and_side_effects() {
    let preview = seeded_consumer_preview(RecipeBuilderEntrypoint::Package);
    let labels = preview.aggregate_safety_labels();
    // Canonical order, includes portability and the mutation labels.
    assert_eq!(
        labels,
        vec![
            AutomationSafetyLabelId::RecipeSafe,
            AutomationSafetyLabelId::HeadlessSafe,
            AutomationSafetyLabelId::ApprovalRequired,
            AutomationSafetyLabelId::WritesFiles,
            AutomationSafetyLabelId::RemoteMutation,
        ]
    );
}

#[test]
fn projects_the_frozen_packet_record() {
    let preview = notebook_preview();
    let record = preview.to_packet_record();
    assert_eq!(record.record_kind, DRY_RUN_EXPLAIN_PACKET_RECORD_KIND);
    assert_eq!(record.step_explanations.len(), preview.actions.len());
    assert_eq!(
        record.dry_run_outcome_class,
        preview.dry_run_outcome_class()
    );
    assert_eq!(
        record.aggregate_safety_labels,
        preview.aggregate_safety_labels()
    );
    assert_eq!(record.run_record_schema_ref, RUN_RECORD_SCHEMA_REF);
}

#[test]
fn run_history_row_carries_the_preview_result() {
    let preview = notebook_preview();
    let row = preview.to_run_history_row("run-history:test", "2026-06-18T00:02:00Z");
    assert_eq!(row.preview_id, preview.preview_id);
    assert_eq!(row.dry_run_outcome_class, preview.dry_run_outcome_class());
    assert_eq!(row.preview_digest, preview.preview_digest());
    assert_eq!(row.predicted_write_count, preview.predicted_write_count());
    assert_eq!(row.run_history_row_schema_ref, RUN_HISTORY_ROW_SCHEMA_REF);
}

#[test]
fn export_round_trips_and_preserves_side_effects() {
    let export = seeded_dry_run_explain_export_roundtrip();
    let imported = export.import();
    let reexported = imported.export(export.export_id.clone(), export.exported_at.clone());
    assert_eq!(reexported, export);
    assert!(export.side_effects_preserved());
}

#[test]
fn seeded_packet_is_stable() {
    let packet = seeded_dry_run_explain_first_consumers_packet();
    assert!(packet.is_stable());
    assert!(packet.validation_findings.is_empty());
    assert_eq!(
        packet.consumer_bindings.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
    assert!(validate_dry_run_explain_first_consumers_packet(&packet).is_ok());
}

#[test]
fn dropping_an_entrypoint_blocks_stable() {
    let mut input = current_dry_run_explain_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
    let packet = DryRunExplainFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == DryRunExplainFindingKind::MissingEntrypoint));
}

#[test]
fn undeclared_predicted_write_blocks_stable() {
    let mut input = current_dry_run_explain_first_consumers_input();
    let binding = input
        .consumer_bindings
        .iter_mut()
        .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::Notebook)
        .expect("notebook");
    binding
        .previewed_actions
        .iter_mut()
        .find(|action| action.step_id == "step:write-export")
        .expect("write-export")
        .predicted_writes
        .clear();
    let packet = DryRunExplainFirstConsumersPacket::materialize(input);
    assert!(
        packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == DryRunExplainFindingKind::PredictedWriteNotDeclared)
    );
}

#[test]
fn outcome_projection_mismatch_blocks_stable() {
    let mut input = current_dry_run_explain_first_consumers_input();
    input
        .consumer_bindings
        .iter_mut()
        .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::RequestApi)
        .expect("request")
        .packet_record
        .dry_run_outcome_class = DryRunOutcomeClass::WouldApply;
    let packet = DryRunExplainFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DryRunExplainFindingKind::OutcomeProjectionInconsistent));
}

#[test]
fn invariant_violation_blocks_stable() {
    let mut input = current_dry_run_explain_first_consumers_input();
    input.invariants.predicted_writes_are_explicit_before_apply = false;
    let packet = DryRunExplainFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == DryRunExplainFindingKind::InvariantViolated));
}
