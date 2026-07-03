use super::*;

fn seeded() -> PipelineRunRow {
    current_m5_pipeline_run_row().expect("canonical pipeline run row loads and validates")
}

fn cloned() -> PipelineRunRow {
    seeded()
}

#[test]
fn checked_in_pipeline_run_row_validates_clean() {
    let row = seeded();
    assert_eq!(row.record_kind, M5_PIPELINE_RUN_ROW_RECORD_KIND);
    assert_eq!(row.schema_version, M5_PIPELINE_RUN_ROW_SCHEMA_VERSION);
    assert!(row.validate().is_empty(), "{:?}", row.validate());
}

#[test]
fn row_discloses_provider_trigger_branch_artifacts_and_authority() {
    let row = seeded();
    assert_eq!(row.provider_label, "github_actions");
    assert_eq!(row.trigger.actor_class, "install_or_bot_account");
    assert_eq!(row.branch_change_relation.base_relation, "stale_base");
    assert!(row.branch_change_relation.stale_base);
    assert_eq!(row.artifact_summary.artifact_count, 3);
    assert_eq!(row.artifact_summary.unavailable_count, 2);
    assert_eq!(row.run_control_authority.authority_label, "provider_owned");
    assert!(!row.run_control_authority.rerun_available);
    assert!(!row.run_control_authority.cancel_available);
    assert!(row.provider_handoff.provider_native_required);
}

#[test]
fn partial_provider_owned_row_keeps_freshness_note_and_limited_action_reason() {
    let row = seeded();
    assert!(row.requires_freshness_note());
    assert_eq!(row.freshness_state, "partial");
    assert_eq!(row.degraded_state, "partial");
    assert_eq!(
        row.run_control_authority.disabled_reason,
        "provider_owned_and_stale_base_requires_reapproval"
    );
    assert!(row
        .freshness_note
        .as_deref()
        .unwrap_or("")
        .contains("partial"));
}

#[test]
fn projections_reuse_one_contract_for_review_pipeline_health_companion_and_support() {
    let row = seeded();
    let review = row
        .projection_for("review_pane")
        .expect("review projection exists");
    let pipeline = row
        .projection_for("pipeline_viewer")
        .expect("pipeline projection exists");
    let health = row
        .projection_for("project_health_center")
        .expect("health projection exists");
    let companion = row
        .projection_for("companion_client")
        .expect("companion projection exists");
    let support = row
        .projection_for("support_export")
        .expect("support projection exists");

    for projection in [&review, &pipeline, &health, &companion, &support] {
        assert_eq!(projection.row_id, row.row_id);
        assert_eq!(projection.pipeline_run_id, row.pipeline_run_id);
        assert_eq!(projection.provider_label, row.provider_label);
        assert_eq!(projection.workflow_or_job_name, row.workflow_or_job_name);
        assert_eq!(projection.trigger_actor_class, row.trigger.actor_class);
        assert_eq!(
            projection.base_relation,
            row.branch_change_relation.base_relation
        );
        assert_eq!(projection.normalized_status, row.normalized_status);
        assert_eq!(
            projection.artifact_count,
            row.artifact_summary.artifact_count
        );
        assert_eq!(
            projection.unavailable_count,
            row.artifact_summary.unavailable_count
        );
        assert_eq!(projection.freshness_state, row.freshness_state);
        assert_eq!(projection.degraded_state, row.degraded_state);
        assert_eq!(
            projection.authority_label,
            row.run_control_authority.authority_label
        );
        assert_eq!(
            projection.limited_action_note,
            row.run_control_authority.disabled_reason
        );
        assert_eq!(
            projection.provider_handoff_target,
            row.provider_handoff.handoff_target_ref
        );
    }
}

#[test]
fn export_copy_preserves_pipeline_truth() {
    let row = seeded();
    let exported = row.export_safe_json();
    for required in [
        "run:m5:review-ci:1042",
        "github_actions",
        "Review CI / dependency-refresh",
        "install_or_bot_account",
        "stale_base",
        "blocked",
        "partial",
        "provider_owned",
        "provider_run:github_actions:1042",
    ] {
        assert!(exported.contains(required), "export dropped {required}");
    }
    for forbidden in ["api_key", "password", "bearer ", "raw provider", "raw log"] {
        assert!(!exported.to_lowercase().contains(forbidden));
    }
}

#[test]
fn limited_authority_without_reason_fails_validation() {
    let mut row = cloned();
    row.run_control_authority.disabled_reason = "not_applicable".to_owned();
    assert!(
        row.validate()
            .contains(&PipelineRunRowViolation::LimitedAuthorityHidden),
        "{:?}",
        row.validate()
    );
}

#[test]
fn partial_row_without_freshness_note_fails_validation() {
    let mut row = cloned();
    row.freshness_note = None;
    assert!(
        row.validate()
            .contains(&PipelineRunRowViolation::FreshnessNoteMissing),
        "{:?}",
        row.validate()
    );
}

#[test]
fn copy_export_that_drops_handoff_fails_validation() {
    let mut row = cloned();
    row.copy_export
        .export_fields
        .retain(|field| field != "provider_handoff");
    assert!(
        row.validate()
            .contains(&PipelineRunRowViolation::CopyExportDropsRunTruth),
        "{:?}",
        row.validate()
    );
}
