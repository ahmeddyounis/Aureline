use std::collections::BTreeSet;
use std::path::Path;

use aureline_shell::start_center::templates::{
    build_alpha_template_scaffold_rows, render_alpha_template_scaffold_plaintext,
};

fn fixture(path: &str) -> serde_json::Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let payload = std::fs::read_to_string(path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

#[test]
fn start_center_template_scaffold_projection_surfaces_review_and_lineage_truth() {
    let expected = fixture("../../fixtures/ux/templates/start_center_template_scaffold_alpha.json");
    let rows = build_alpha_template_scaffold_rows().expect("template rows project");
    assert_eq!(rows.len(), 1);

    let row = &rows[0];
    assert_eq!(
        row.template_id,
        expected["expected_template_id"].as_str().unwrap()
    );
    assert_eq!(
        row.template_label,
        expected["expected_template_label"].as_str().unwrap()
    );
    assert_eq!(
        row.manifest_ref,
        expected["expected_manifest_ref"].as_str().unwrap()
    );
    assert_eq!(
        row.source_class,
        expected["expected_source_class"].as_str().unwrap()
    );
    assert_eq!(
        row.signature_state,
        expected["expected_signature_state"].as_str().unwrap()
    );
    assert_eq!(
        row.support_class,
        expected["expected_support_class"].as_str().unwrap()
    );
    assert_eq!(
        row.file_impact_summary,
        expected["expected_file_impact_summary"].as_str().unwrap()
    );
    assert_eq!(
        row.health_counts_label,
        expected["expected_health_counts_label"].as_str().unwrap()
    );
    assert_eq!(
        row.checkpoint_ref,
        expected["expected_checkpoint_ref"].as_str().unwrap()
    );
    assert_eq!(
        row.lineage_ref,
        expected["expected_lineage_ref"].as_str().unwrap()
    );
    assert_eq!(
        row.divergence_state,
        expected["expected_divergence_state"].as_str().unwrap()
    );
    assert_eq!(
        row.manual_edit_detection_state,
        expected["expected_manual_edit_detection_state"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        row.update_rebase_compatibility_state,
        expected["expected_update_rebase_compatibility_state"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        row.no_writes_before_review,
        expected["expected_no_writes_before_review"]
            .as_bool()
            .unwrap()
    );
    assert_eq!(
        row.raw_content_export_allowed,
        expected["expected_raw_content_export_allowed"]
            .as_bool()
            .unwrap()
    );

    let expected_sources = expected["expected_health_freshness_sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect::<BTreeSet<_>>();
    let actual_sources = row
        .health_freshness_sources
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_sources, expected_sources);

    let expected_bypass = expected["expected_bypass_path_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect::<BTreeSet<_>>();
    let actual_bypass = row.bypass_path_ids.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(actual_bypass, expected_bypass);

    assert_eq!(row.required_parameter_count, 2);
    assert_eq!(row.declared_hook_count, 3);
    assert_eq!(row.declared_setup_task_count, 2);
    assert!(row
        .setup_summary
        .iter()
        .any(|summary| summary.contains("Run later")));
    assert!(row
        .support_export_refs
        .iter()
        .all(|support_ref| support_ref.starts_with("support_export:")));
}

#[test]
fn template_scaffold_plaintext_is_support_safe_and_reviewable() {
    let text = render_alpha_template_scaffold_plaintext().expect("plaintext renders");
    assert!(text.contains("Template scaffold alpha"));
    assert!(text.contains("template.alpha.ts_web.vite_react_seed"));
    assert!(text.contains("first_party/verified"));
    assert!(text.contains("cached,live,policy-evaluated,unchecked"));
    assert!(text.contains("checkpoint:scaffold.typescript_web_vite_local.001"));
    assert!(text.contains("generated_project_lineage_alpha:typescript_web_vite_local:001"));
}
