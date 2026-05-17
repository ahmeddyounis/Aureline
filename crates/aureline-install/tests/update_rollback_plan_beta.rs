//! Fixture-driven tests for beta update rollback plans.

use std::path::{Path, PathBuf};

use aureline_install::{
    DowngradeEligibilityState, RetainedArtifactState, SchemaRollbackHookState, UpdateRollbackPlan,
    UpdateRollbackSupportExport,
};

fn plan_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/release/m3/update_rollback/rollback_plan.json")
}

fn support_export_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/release/m3/update_rollback/support_export_projection.json")
}

fn load_plan() -> UpdateRollbackPlan {
    let bytes = std::fs::read(plan_path()).expect("read update rollback plan");
    serde_json::from_slice(&bytes).expect("parse update rollback plan")
}

#[test]
fn update_rollback_plan_passes_validation() {
    let plan = load_plan();
    let report = plan.validate();

    assert!(
        report.passed,
        "update rollback plan failed validation: {:#?}",
        report.findings
    );
    assert_eq!(
        report.coverage.downgrade_eligibility_state,
        DowngradeEligibilityState::EligibleWithReview
    );
    assert_eq!(report.coverage.schema_hook_count, 3);
    assert!(plan
        .retained_prior_artifacts
        .iter()
        .all(|artifact| artifact.rollback_atom_member));
    assert!(plan
        .schema_rollback_hooks
        .iter()
        .all(|hook| hook.hook_state != SchemaRollbackHookState::Blocked));
}

#[test]
fn support_export_is_projected_from_the_plan() {
    let plan = load_plan();
    let expected_support = plan.support_export_projection();
    let support_bytes = std::fs::read(support_export_path()).expect("read support export");
    let checked_in_support: UpdateRollbackSupportExport =
        serde_json::from_slice(&support_bytes).expect("parse support export");

    assert_eq!(checked_in_support, expected_support);
}

#[test]
fn retained_metadata_only_artifact_is_rejected() {
    let mut plan = load_plan();
    plan.retained_prior_artifacts[0].retention_state = RetainedArtifactState::RetainedMetadataOnly;

    let report = plan.validate();

    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "retained_artifacts.not_exact_build_retained" }));
}

#[test]
fn schema_hook_outside_update_checkpoint_is_rejected() {
    let mut plan = load_plan();
    plan.schema_rollback_hooks[0].invoked_checkpoint_id =
        "checkpoint.migration.rollback_triggered".to_string();

    let report = plan.validate();

    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "schema_hooks.invoked_checkpoint_not_update_sequence"
    }));
}
