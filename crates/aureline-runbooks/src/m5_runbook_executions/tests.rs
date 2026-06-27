//! Inline tests for the M5 runbook execution history.

use super::*;

use crate::m5_runbook_governance::{RunbookStepClass, StepOutcomeClass};

fn canonical() -> M5RunbookExecutionHistory {
    seeded_m5_runbook_execution_history()
}

fn row(history: &M5RunbookExecutionHistory, step_id: &str) -> RunbookExecutionRowProjection {
    history
        .row_projections
        .iter()
        .find(|r| r.step_id == step_id)
        .cloned()
        .unwrap_or_else(|| panic!("row {step_id} absent"))
}

#[test]
fn canonical_history_validates() {
    let history = canonical();
    assert!(history.validate().is_empty(), "{:?}", history.validate());
    assert_eq!(history.history_id, M5_RUNBOOK_EXECUTION_HISTORY_ID);
    assert_eq!(
        history.record_kind,
        M5_RUNBOOK_EXECUTION_HISTORY_RECORD_KIND
    );
    assert_eq!(history.executions.len(), 4);
    assert!(!history.row_projections.is_empty());
}

#[test]
fn every_record_is_attributable_and_export_safe() {
    let history = canonical();
    for execution in &history.executions {
        assert!(
            execution.attributable,
            "{} unattributable",
            execution.execution_id
        );
        assert!(execution.no_hidden_mutate_channel);
        assert!(!execution.archival_export.raw_content_exported);
    }
    for r in &history.row_projections {
        assert!(r.attributable, "row {} unattributable", r.step_id);
        assert!(!r.actor_ref.is_empty());
        assert!(!r.creates_hidden_mutate_channel);
    }
}

#[test]
fn mutating_rows_reuse_shared_preview_and_approval() {
    let history = canonical();
    let mitigate = row(&history, "restart.mitigate");
    assert!(mitigate.mutating);
    assert!(mitigate.reuses_shared_preview);
    assert!(mitigate.preview_hash.is_some());
    assert!(mitigate.reuses_shared_approval);
    assert!(mitigate.approval_ref.is_some());
    assert_eq!(mitigate.preview_disposition, "diff_then_confirm");

    let rollback = row(&history, "failover.rollback");
    assert!(rollback.mutating);
    assert!(rollback.reuses_shared_preview);
    assert!(rollback.requires_explicit_human_approval);
}

#[test]
fn observe_verify_communicate_rows_have_no_fake_mutation() {
    let history = canonical();
    for step_id in ["restart.inspect", "restart.diagnose", "companion.request"] {
        let r = row(&history, step_id);
        assert!(!r.mutating);
        assert!(!r.reuses_shared_preview, "{step_id} carries a preview");
        assert!(r.preview_hash.is_none());
        assert_eq!(r.preview_disposition, "read_only_preview");
    }
    // The annotate communication row records execution + evidence with no approval.
    let request = row(&history, "companion.request");
    assert!(!request.requires_approval);
    assert!(request.approval_ref.is_none());
    assert!(request.audit_expects_evidence);
}

#[test]
fn read_only_rows_carry_no_approval_ref() {
    let history = canonical();
    let inspect = row(&history, "restart.inspect");
    assert!(!inspect.requires_approval);
    assert!(!inspect.reuses_shared_approval);
    assert!(inspect.approval_ref.is_none());
}

#[test]
fn handoff_row_previews_the_boundary_crossing() {
    let history = canonical();
    let console = row(&history, "vendor.console");
    assert_eq!(
        console.step_class,
        RunbookStepClass::ConsoleHandoff.as_str()
    );
    assert!(console.handed_off);
    assert_eq!(console.preview_disposition, "handoff_preview");
    // It required human approval, so it reuses the shared approval authority, but it
    // does not mint an in-plane preview (no fake mutation).
    assert!(console.reuses_shared_approval);
    assert!(!console.reuses_shared_preview);
    assert!(console.attributable);
}

#[test]
fn history_rows_explain_what_ran_under_which_approval_with_which_evidence() {
    let history = canonical();
    for r in &history.row_projections {
        assert!(!r.step_label.is_empty());
        assert!(!r.outcome.is_empty());
        assert!(!r.approval_scope.is_empty());
        assert!(!r.deviation_class.is_empty());
        // A completed mutating row must show the evidence outputs it produced.
        if r.mutating && r.outcome == StepOutcomeClass::Completed.as_str() {
            assert!(r.audit_expects_evidence, "row {} lacks evidence", r.step_id);
        }
    }
}

#[test]
fn deviation_lineage_surfaces_in_rows() {
    let history = canonical();
    let drain = row(&history, "failover.drain");
    assert_eq!(drain.outcome, StepOutcomeClass::Skipped.as_str());
    assert_eq!(drain.deviation_class, "step_skipped");
    let rollback = row(&history, "failover.rollback");
    assert_eq!(rollback.deviation_class, "step_added_ad_hoc");
}

#[test]
fn archival_lineage_is_joinable_and_durable() {
    let history = canonical();
    assert_eq!(history.archival_lineage.len(), history.executions.len());
    for lineage in &history.archival_lineage {
        assert!(lineage.archived);
        assert!(!lineage.archived_at.is_empty());
        assert!(lineage.lineage_recoverable_from_metadata_only);
        assert!(!lineage.support_pack_item_id.is_empty());
        assert!(!lineage.joined_families.is_empty());
    }
    // The failover lineage keeps its durable deviation notes and joins all four families.
    let failover = history
        .lineage("failover-deviation-lineage")
        .expect("lineage present");
    assert_eq!(
        failover.joined_families,
        vec!["incident", "rollout", "review", "support_bundle"]
    );
    assert_eq!(failover.deviations.len(), 2);
    for deviation in &failover.deviations {
        assert!(!deviation.deviation_id.is_empty());
        assert!(!deviation.actor_ref.is_empty());
        assert!(!deviation.recorded_at.is_empty());
        assert!(deviation.attributable);
    }
}

#[test]
fn archival_lineage_is_identical_across_surfaces() {
    let history = canonical();
    let operator = history.lineage_for_surface(RunbookExecutionSurface::OperatorHistory);
    let support = history.lineage_for_surface(RunbookExecutionSurface::SupportExport);
    let incident = history.lineage_for_surface(RunbookExecutionSurface::IncidentPacket);
    assert_eq!(operator, support);
    assert_eq!(operator, incident);
    assert_eq!(operator, history.archival_lineage);
}

#[test]
fn archival_lineage_drift_is_caught() {
    let mut history = canonical();
    history.archival_lineage[0].archived_at = "tampered".to_owned();
    assert!(history
        .validate()
        .contains(&M5RunbookExecutionViolation::ArchivalLineageDrift));
}

#[test]
fn markdown_summary_reconstructs_archived_lineage() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Archived lineage (joinable after closure)"));
    assert!(summary.contains("no raw payload retained"));
    assert!(summary.contains("support.item.runbook.execution.failover-deviation-lineage"));
}

#[test]
fn projection_recomputes_from_the_records() {
    let history = canonical();
    let desktop = history.projections_for_surface(RunbookExecutionSurface::OperatorHistory);
    let support = history.projections_for_surface(RunbookExecutionSurface::SupportExport);
    let incident = history.projections_for_surface(RunbookExecutionSurface::IncidentPacket);
    assert_eq!(desktop, support);
    assert_eq!(desktop, incident);
    assert_eq!(desktop, history.row_projections);
}

#[test]
fn projection_drift_is_caught() {
    let mut history = canonical();
    history.row_projections[0].reuses_shared_approval =
        !history.row_projections[0].reuses_shared_approval;
    assert!(history
        .validate()
        .contains(&M5RunbookExecutionViolation::ProjectionDrift));
}

#[test]
fn duplicate_execution_ids_are_rejected() {
    let mut history = canonical();
    let dup = history.executions[0].clone();
    history.executions.push(dup);
    history.row_projections = derive_row_projections(&history.executions);
    history.conformance = derive_conformance(&history.executions);
    assert!(history
        .validate()
        .contains(&M5RunbookExecutionViolation::DuplicateExecutionId));
}

#[test]
fn conformance_review_holds_and_is_derived() {
    let history = canonical();
    assert!(history.conformance.all_hold());
    assert!(history.vocabulary.matches_canonical());
    let mut tampered = history.clone();
    tampered
        .conformance
        .mutating_rows_reuse_shared_preview_and_approval = false;
    assert!(tampered
        .validate()
        .contains(&M5RunbookExecutionViolation::ConformanceReviewFailed));
}

#[test]
fn surface_exposure_covers_operator_history_support_and_incident() {
    let history = canonical();
    assert!(history.surface_exposure.all_expose());
    let tokens: Vec<&str> = RunbookExecutionSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec!["operator_history", "support_export", "incident_packet"]
    );
}

#[test]
fn round_trips_through_json() {
    let history = canonical();
    let json = history.export_safe_json();
    let parsed: M5RunbookExecutionHistory = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, history);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_names_executions_and_reuse() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Execution rows"));
    assert!(summary.contains("restart.mitigate"));
    assert!(summary.contains("operator history, support exports, incident packets"));
}

#[test]
fn export_carries_no_forbidden_boundary_material() {
    let json = canonical().export_safe_json();
    for needle in ["credential", "secret", "password", "bearer_token"] {
        assert!(!json.contains(needle), "export leaked {needle}");
    }
}
