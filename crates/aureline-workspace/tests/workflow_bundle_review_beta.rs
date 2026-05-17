//! Fixture-driven coverage for workflow-bundle review packets.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_workflow_bundle_review, WorkflowBundleReviewRecord, WORKFLOW_BUNDLE_REQUIRED_DIFF_AXES,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/workflow_bundle_review")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("workflow-bundle review fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_record(name: &str) -> WorkflowBundleReviewRecord {
    let payload = std::fs::read_to_string(fixtures_dir().join(name)).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

#[test]
fn every_fixture_projects_through_the_beta_contract() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "workflow-bundle review fixtures dir must contain fixtures"
    );

    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_workflow_bundle_review(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert!(
            projection.missing_required_diff_axes.is_empty(),
            "fixture {path:?} must cover every required diff axis"
        );
        assert_eq!(
            projection.required_diff_axes_present.len(),
            WORKFLOW_BUNDLE_REQUIRED_DIFF_AXES.len(),
            "fixture {path:?} must expose the required axes exactly once in projection"
        );
        assert!(!projection.raw_export_allowed, "fixture {path:?}");
        assert!(projection.guardrails_pass, "fixture {path:?}");
        assert!(
            projection
                .review_actions
                .iter()
                .any(|action| action == "review.compare"),
            "fixture {path:?} must expose compare"
        );
    }
}

#[test]
fn launch_wedge_review_exposes_install_drift_remove_and_rollback() {
    let record = load_record("launch_wedge_install_update_drift_rollback.json");
    let projection = record.project();

    assert_eq!(projection.bundle_class, "launch_bundle");
    assert_eq!(projection.bundle_source_class, "certified");
    assert_eq!(projection.effective_badge_class, "certified");
    assert_eq!(projection.drift_entry_count, 2);
    assert_eq!(projection.removable_asset_count, 3);
    assert_eq!(projection.retained_override_count, 2);
    assert!(projection
        .resolve_actions
        .iter()
        .any(|action| action == "resolve.keep_local"));
    assert!(projection
        .resolve_actions
        .iter()
        .any(|action| action == "resolve.rebase_to_bundle"));
    assert!(projection
        .support_export_refs
        .iter()
        .any(|reference| reference.contains("remove_review")));

    record.validate().expect("launch review must validate");
}

#[test]
fn imported_user_review_stays_pending_and_preserves_user_owned_assets() {
    let record = load_record("imported_user_pending_review.json");
    let projection = record.project();

    assert_eq!(projection.bundle_class, "imported_user_bundle");
    assert_eq!(projection.bundle_source_class, "imported");
    assert_eq!(projection.effective_badge_class, "imported");
    assert!(record
        .install_update_review
        .actions
        .iter()
        .any(|action| action.action_id == "review.confirm"
            && action.rendered_state == "visible_disabled"));
    assert!(record
        .remove_rollback_review
        .removable_assets
        .iter()
        .any(|asset| asset.ownership_class == "user_owned"
            && asset.safe_to_remove_class == "not_safe_to_remove_user_owned"));

    record.validate().expect("imported review must validate");
}

#[test]
fn rejects_install_update_review_missing_required_axis() {
    let mut record = load_record("launch_wedge_install_update_drift_rollback.json");
    record
        .install_update_review
        .diff_entries
        .retain(|entry| entry.change_axis != "migration_mapping");

    let err = record
        .validate()
        .expect_err("required diff axis must be enforced");
    assert!(err.message().contains("migration_mapping"));
}

#[test]
fn rejects_drift_adopt_without_change_preview_route() {
    let mut record = load_record("launch_wedge_install_update_drift_rollback.json");
    let action = record.drift_override_review.drift_entries[0]
        .resolve_actions
        .iter_mut()
        .find(|action| action.action_id == "resolve.adopt_bundle")
        .expect("fixture has adopt action");
    action.destination_ref = Some("ack:not_a_preview".to_string());

    let err = record
        .validate()
        .expect_err("adopt must route through change preview");
    assert!(err.message().contains("bundle_change_preview"));
}

#[test]
fn rejects_user_owned_asset_marked_safe_to_remove() {
    let mut record = load_record("imported_user_pending_review.json");
    let asset = record
        .remove_rollback_review
        .removable_assets
        .iter_mut()
        .find(|asset| asset.ownership_class == "user_owned")
        .expect("fixture has user-owned asset");
    asset.safe_to_remove_class = "safe_to_remove_no_user_data".to_string();

    let err = record
        .validate()
        .expect_err("user-owned asset cannot be safe to remove");
    assert!(err.message().contains("user_owned"));
}

#[test]
fn rejects_stale_certification_overclaim() {
    let mut record = load_record("launch_wedge_install_update_drift_rollback.json");
    record.certification.evidence_freshness_class = "stale_past_window".to_string();
    record.certification.retest_required = true;
    record.certification.effective_badge_class = "certified".to_string();

    let err = record
        .validate()
        .expect_err("stale evidence cannot keep certified badge");
    assert!(err.message().contains("stale"));
}

#[test]
fn rejects_hidden_authority_widening() {
    let mut record = load_record("launch_wedge_install_update_drift_rollback.json");
    record.guardrails.network_egress_widened_without_review = true;

    let err = record
        .validate()
        .expect_err("network widening must be rejected");
    assert!(err.message().contains("guardrails"));
}

#[test]
fn rejects_raw_support_export() {
    let mut record = load_record("launch_wedge_install_update_drift_rollback.json");
    record.support_export.raw_user_content_export_allowed = true;

    let err = record
        .validate()
        .expect_err("raw user content export must be rejected");
    assert!(err.message().contains("raw export"));
}
