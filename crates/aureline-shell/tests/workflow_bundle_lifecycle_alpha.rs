use std::path::Path;

use aureline_shell::start_center::bundles::{
    build_alpha_bundle_lifecycle_rows, render_alpha_bundle_lifecycle_plaintext,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LifecycleFixture {
    expected_bundle_id: String,
    expected_scorecard_status_class: String,
    expected_display_status_label: String,
    expected_review_refs: Vec<String>,
    expected_drift_states: Vec<String>,
    expected_lifecycle_actions: Vec<String>,
    expected_template_refs: Vec<String>,
    expected_support_export_refs: Vec<String>,
    template_refs_must_be_explicit_and_mirrorable: bool,
    raw_content_export_allowed: bool,
    rollback_checkpoint_policy: String,
}

fn lifecycle_fixture() -> LifecycleFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ux/workflow_bundles/start_center_lifecycle_typescript.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

#[test]
fn start_center_lifecycle_projection_joins_bundle_review_drift_and_scorecard() {
    let fixture = lifecycle_fixture();
    let rows = build_alpha_bundle_lifecycle_rows().expect("lifecycle rows project");
    assert_eq!(rows.len(), 2);

    let row = rows
        .iter()
        .find(|row| row.bundle_id == fixture.expected_bundle_id)
        .expect("typescript lifecycle row");

    assert_eq!(
        row.scorecard_status_class,
        fixture.expected_scorecard_status_class
    );
    assert_eq!(
        row.scorecard_display_label,
        fixture.expected_display_status_label
    );
    assert_eq!(row.install_preview_ref, fixture.expected_review_refs[0]);
    assert_eq!(row.update_preview_ref, fixture.expected_review_refs[1]);
    assert_eq!(row.remove_review_ref, fixture.expected_review_refs[2]);
    assert_eq!(
        row.rollback_checkpoint_policy,
        fixture.rollback_checkpoint_policy
    );

    for drift_state in &fixture.expected_drift_states {
        assert!(
            row.drift_states.contains(drift_state),
            "missing drift state {drift_state}"
        );
    }
    for action in &fixture.expected_lifecycle_actions {
        assert!(
            row.lifecycle_actions.contains(action),
            "missing lifecycle action {action}"
        );
    }
    for template_ref in &fixture.expected_template_refs {
        assert!(
            row.template_scaffold_refs.contains(template_ref),
            "missing template ref {template_ref}"
        );
    }
    for support_ref in &fixture.expected_support_export_refs {
        assert!(
            row.support_export_refs.contains(support_ref),
            "missing support export ref {support_ref}"
        );
    }

    assert_eq!(
        row.template_refs_explicit_and_mirrorable,
        fixture.template_refs_must_be_explicit_and_mirrorable
    );
    assert_eq!(
        row.raw_content_export_allowed,
        fixture.raw_content_export_allowed
    );
    assert!(row
        .archetype_row_refs
        .iter()
        .any(|archetype| archetype.contains("archetype_row:ts_web_app_or_service")));
}

#[test]
fn lifecycle_plaintext_is_support_safe_and_reviewable() {
    let text = render_alpha_bundle_lifecycle_plaintext().expect("lifecycle plaintext");
    assert!(text.contains("Workflow bundle lifecycle alpha"));
    assert!(text.contains("launch_bundle:typescript_web_app.seed"));
    assert!(text.contains("launch_bundle:python_service_or_data_app.seed"));
    assert!(text.contains("Partial"));
    assert!(text.contains("Retest pending"));
    assert!(text.contains("local_override"));
    assert!(text.contains("missing_artifact"));
    assert!(text.contains("remove_bundle"));
    assert!(text.contains("support_export:workflow_bundle.typescript_web_app.review"));
}
