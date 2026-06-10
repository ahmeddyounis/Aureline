//! Integration test: the embedded qualification packets parse and validate.

use aureline_api::{
    current_database_browser_qualification, current_explain_plan_qualification,
    current_handoff_qualification, current_request_composer_qualification,
    current_request_workspace_qualification, current_response_viewer_qualification,
    current_result_grid_qualification, current_staged_row_mutation_qualification,
    current_statement_safety_qualification,
};

#[test]
fn embedded_workspace_packet_parses() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.documents.is_empty());
    assert!(!packet.environment_sets.is_empty());
    assert!(!packet.auth_sources.is_empty());
}

#[test]
fn embedded_workspace_packet_has_no_violations() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_workspace_summary_matches_computed() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_composer_packet_parses() {
    let packet =
        current_request_composer_qualification().expect("embedded composer packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.composers.is_empty());
    assert!(!packet.mutation_review_sheets.is_empty());
    assert!(!packet.history_lanes.is_empty());
    assert!(!packet.replay_configs.is_empty());
    assert!(!packet.redaction_safe_exports.is_empty());
}

#[test]
fn embedded_composer_packet_has_no_violations() {
    let packet =
        current_request_composer_qualification().expect("embedded composer packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_composer_summary_matches_computed() {
    let packet =
        current_request_composer_qualification().expect("embedded composer packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_response_viewer_packet_parses() {
    let packet = current_response_viewer_qualification()
        .expect("embedded response viewer packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.response_viewers.is_empty());
    assert!(!packet.assertions.is_empty());
    assert!(!packet.timing_tabs.is_empty());
    assert!(!packet.browser_runtime_trusts.is_empty());
}

#[test]
fn embedded_response_viewer_packet_has_no_violations() {
    let packet = current_response_viewer_qualification()
        .expect("embedded response viewer packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_response_viewer_summary_matches_computed() {
    let packet = current_response_viewer_qualification()
        .expect("embedded response viewer packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_database_browser_packet_parses() {
    let packet = current_database_browser_qualification()
        .expect("embedded database browser packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.connection_browsers.is_empty());
    assert!(!packet.schema_trees.is_empty());
    assert!(!packet.target_context_envelopes.is_empty());
}

#[test]
fn embedded_database_browser_packet_has_no_violations() {
    let packet = current_database_browser_qualification()
        .expect("embedded database browser packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_database_browser_summary_matches_computed() {
    let packet = current_database_browser_qualification()
        .expect("embedded database browser packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_statement_safety_packet_parses() {
    let packet = current_statement_safety_qualification()
        .expect("embedded statement safety packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.classifiers.is_empty());
    assert!(!packet.write_mode_bars.is_empty());
    assert!(!packet.protected_target_step_ups.is_empty());
}

#[test]
fn embedded_statement_safety_packet_has_no_violations() {
    let packet = current_statement_safety_qualification()
        .expect("embedded statement safety packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_statement_safety_summary_matches_computed() {
    let packet = current_statement_safety_qualification()
        .expect("embedded statement safety packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_result_grid_packet_parses() {
    let packet =
        current_result_grid_qualification().expect("embedded result grid packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.result_grid_viewers.is_empty());
    assert!(!packet.typed_copy_actions.is_empty());
    assert!(!packet.typed_export_actions.is_empty());
    assert!(!packet.filter_sort_state_panels.is_empty());
    assert!(!packet.row_count_boundary_chips.is_empty());
}

#[test]
fn embedded_result_grid_packet_has_no_violations() {
    let packet =
        current_result_grid_qualification().expect("embedded result grid packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_result_grid_summary_matches_computed() {
    let packet =
        current_result_grid_qualification().expect("embedded result grid packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_explain_plan_packet_parses() {
    let packet =
        current_explain_plan_qualification().expect("embedded explain plan packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.freshness_notes.is_empty());
    assert!(!packet.engine_version_contexts.is_empty());
    assert!(!packet.plan_comparison_flows.is_empty());
}

#[test]
fn embedded_explain_plan_packet_has_no_violations() {
    let packet =
        current_explain_plan_qualification().expect("embedded explain plan packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_explain_plan_summary_matches_computed() {
    let packet =
        current_explain_plan_qualification().expect("embedded explain plan packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_handoff_packet_parses() {
    let packet = current_handoff_qualification().expect("embedded handoff packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.notebook_handoffs.is_empty());
    assert!(!packet.chart_handoffs.is_empty());
    assert!(!packet.ai_handoffs.is_empty());
    assert!(!packet.support_exports.is_empty());
}

#[test]
fn embedded_handoff_packet_has_no_violations() {
    let packet = current_handoff_qualification().expect("embedded handoff packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_handoff_summary_matches_computed() {
    let packet = current_handoff_qualification().expect("embedded handoff packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_staged_row_mutation_packet_parses() {
    let packet = current_staged_row_mutation_qualification()
        .expect("embedded staged row mutation packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.staged_row_mutation_sheets.is_empty());
    assert!(!packet.optimistic_concurrency_cues.is_empty());
    assert!(!packet.rollback_actions.is_empty());
    assert!(!packet.checkpoint_actions.is_empty());
}

#[test]
fn embedded_staged_row_mutation_packet_has_no_violations() {
    let packet = current_staged_row_mutation_qualification()
        .expect("embedded staged row mutation packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_staged_row_mutation_summary_matches_computed() {
    let packet = current_staged_row_mutation_qualification()
        .expect("embedded staged row mutation packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}
