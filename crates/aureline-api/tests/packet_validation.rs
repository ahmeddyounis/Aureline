//! Integration test: the embedded qualification packets parse and validate.

use aureline_api::{
    current_api_matrix_qualification, current_certification_qualification,
    current_database_browser_qualification, current_explain_plan_qualification,
    current_freshness_banner_qualification, current_handoff_qualification,
    current_origin_truth_qualification, current_persisted_operation_qualification,
    current_request_composer_qualification, current_request_views_qualification,
    current_request_workspace_qualification, current_response_viewer_qualification,
    current_result_grid_qualification, current_ship_query_history_qualification,
    current_staged_row_mutation_qualification, current_statement_safety_qualification,
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
fn workspace_packet_projects_m5_secret_boundary_states() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    let states = packet.secret_boundary_states();
    assert_eq!(states.len(), 2);
    assert_eq!(
        states[0].matrix_row_id,
        "m5.secret.request_workspace.send_http"
    );
    assert_eq!(
        states[1].matrix_row_id,
        "m5.secret.request_workspace.history_replay"
    );
    assert_eq!(
        states[0].secret_access_prompt.vocabulary_ref,
        "docs/security/m5/m5-secret-boundary-depth.md#shared-vocabulary"
    );
    assert_eq!(
        states[0]
            .consumer_identity_receipt
            .consumer_identity
            .as_str(),
        "local_workflow"
    );
    assert!(!states[0].repairable_states.is_empty());
    assert!(!states[0]
        .projection_mode_audit
        .available_controls
        .is_empty());
    assert!(!states[0].export_safety_banner.raw_secret_values_included);
    let lineage = states[0].lineage_bundle();
    assert!(!lineage.events.is_empty());
    assert!(!lineage.workflow_history_rows.is_empty());
    assert!(!lineage.activity_rows.is_empty());
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
fn embedded_ship_query_history_packet_parses() {
    let packet = current_ship_query_history_qualification()
        .expect("embedded ship query history packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.query_history_entries.is_empty());
    assert!(!packet.connection_profile_portabilities.is_empty());
    assert!(!packet.secret_safe_auth_storages.is_empty());
    assert!(!packet.mirror_or_offline_truths.is_empty());
}

#[test]
fn embedded_ship_query_history_packet_has_no_violations() {
    let packet = current_ship_query_history_qualification()
        .expect("embedded ship query history packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_ship_query_history_summary_matches_computed() {
    let packet = current_ship_query_history_qualification()
        .expect("embedded ship query history packet must parse");
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

#[test]
fn embedded_api_matrix_packet_parses() {
    let packet = current_api_matrix_qualification().expect("embedded api matrix packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.contracts.is_empty());
    assert!(!packet.collections.is_empty());
    assert!(!packet.requests.is_empty());
    assert!(!packet.origins.is_empty());
    assert!(!packet.persisted_operations.is_empty());
    assert!(!packet.retention_classes.is_empty());
    assert!(!packet.upstream_refs.is_empty());
}

#[test]
fn embedded_api_matrix_packet_has_no_violations() {
    let packet = current_api_matrix_qualification().expect("embedded api matrix packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_api_matrix_summary_matches_computed() {
    let packet = current_api_matrix_qualification().expect("embedded api matrix packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn api_matrix_projects_narrowing_and_drift_signals() {
    let packet = current_api_matrix_qualification().expect("embedded api matrix packet must parse");
    // Stale schema and unavailable contract must narrow any live claim.
    let narrowing = packet.narrowing_contract_ids();
    assert!(narrowing.contains(&"contract:graphql_stale".to_owned()));
    assert!(narrowing.contains(&"contract:plugin_unavailable".to_owned()));
    // Persisted-operation drift and origin change feed diagnostics and downgrade.
    assert_eq!(
        packet.persisted_operation_drift_ids(),
        vec!["binding:graphql_drift".to_owned()]
    );
    assert_eq!(
        packet.changed_origin_ids(),
        vec!["origin:browser_companion".to_owned()]
    );
}

#[test]
fn embedded_request_views_packet_parses() {
    let packet =
        current_request_views_qualification().expect("embedded request views packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.collection_views.is_empty());
    assert!(!packet.request_views.is_empty());
    assert!(!packet.environment_views.is_empty());
    assert!(!packet.saved_views.is_empty());
    assert!(!packet.upstream_refs.is_empty());
}

#[test]
fn embedded_request_views_packet_has_no_violations() {
    let packet =
        current_request_views_qualification().expect("embedded request views packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_request_views_summary_matches_computed() {
    let packet =
        current_request_views_qualification().expect("embedded request views packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn request_views_consume_api_matrix_and_project_drift() {
    let packet =
        current_request_views_qualification().expect("embedded request views packet must parse");
    // The views are a real consumer of the frozen API-collection matrix.
    let consumes_matrix = packet.upstream_refs.iter().any(|row| {
        row.upstream_record_kind
            == "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix"
            && row.integration_verified
    });
    assert!(
        consumes_matrix,
        "request views must reference the API-collection matrix packet"
    );
    // Provider-linked and drift-blocked rows are surfaced rather than hidden.
    assert_eq!(
        packet.provider_linked_request_ids(),
        vec!["request_view:graphql_provider_linked".to_owned()]
    );
    let blocked = packet.drift_blocked_request_ids();
    assert!(blocked.contains(&"request_view:graphql_stale_blocked".to_owned()));
    assert!(blocked.contains(&"request_view:rest_managed_shared".to_owned()));
}

#[test]
fn embedded_freshness_banner_packet_parses() {
    let packet = current_freshness_banner_qualification()
        .expect("embedded freshness banner packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.banners.is_empty());
    assert!(!packet.refresh_flows.is_empty());
    assert!(!packet.diff_flows.is_empty());
    assert!(!packet.open_spec_flows.is_empty());
    assert!(!packet.upstream_refs.is_empty());
}

#[test]
fn embedded_freshness_banner_packet_has_no_violations() {
    let packet = current_freshness_banner_qualification()
        .expect("embedded freshness banner packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_freshness_banner_summary_matches_computed() {
    let packet = current_freshness_banner_qualification()
        .expect("embedded freshness banner packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn freshness_banner_consumes_api_matrix_and_labels_snapshots() {
    let packet = current_freshness_banner_qualification()
        .expect("embedded freshness banner packet must parse");
    // The banners are a real consumer of the frozen API-collection matrix.
    let consumes_matrix = packet.upstream_refs.iter().any(|row| {
        row.upstream_record_kind
            == "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix"
            && row.integration_verified
    });
    assert!(
        consumes_matrix,
        "freshness banners must reference the API-collection matrix packet"
    );
    // Stale and unavailable banners narrow any live claim.
    let narrowing = packet.narrowing_banner_ids();
    assert!(narrowing.contains(&"banner:graphql_stale".to_owned()));
    assert!(narrowing.contains(&"banner:plugin_unavailable".to_owned()));
    // Imported snapshots are surfaced as their own labeled class.
    assert_eq!(
        packet.imported_snapshot_banner_ids(),
        vec!["banner:graphql_imported".to_owned()]
    );
    // GraphQL is the lane the row most depends on; it must be covered.
    assert!(!packet.graphql_banner_ids().is_empty());
}

#[test]
fn embedded_origin_truth_packet_parses() {
    let packet =
        current_origin_truth_qualification().expect("embedded origin truth packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.origins.is_empty());
    assert!(!packet.rerun_sheets.is_empty());
    assert!(!packet.origin_changes.is_empty());
    assert!(!packet.upstream_refs.is_empty());
}

#[test]
fn embedded_origin_truth_packet_has_no_violations() {
    let packet =
        current_origin_truth_qualification().expect("embedded origin truth packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_origin_truth_summary_matches_computed() {
    let packet =
        current_origin_truth_qualification().expect("embedded origin truth packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn origin_truth_consumes_api_matrix_and_projects_drift_review() {
    let packet =
        current_origin_truth_qualification().expect("embedded origin truth packet must parse");
    // The origin truth is a real consumer of the frozen API-collection matrix.
    let consumes_matrix = packet.upstream_refs.iter().any(|row| {
        row.upstream_record_kind
            == "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix"
            && row.integration_verified
    });
    assert!(
        consumes_matrix,
        "origin truth must reference the API-collection matrix packet"
    );
    // Drifted origins are surfaced for review rather than hidden.
    let changed = packet.changed_origin_ids();
    assert!(changed.contains(&"origin:browser_companion".to_owned()));
    assert!(changed.contains(&"origin:managed_workspace".to_owned()));
    // Managed and companion origins isolate desktop-local trust.
    let isolated = packet.trust_isolated_origin_ids();
    assert!(isolated.contains(&"origin:managed_workspace".to_owned()));
    assert!(isolated.contains(&"origin:browser_companion".to_owned()));
    // Drifted reruns block dispatch until the enumerated changes are reviewed.
    let blocked = packet.dispatch_blocked_sheet_ids();
    assert!(blocked.contains(&"sheet:browser_companion_current_context".to_owned()));
    assert!(blocked.contains(&"sheet:managed_current_context".to_owned()));
}

#[test]
fn certification_scorecard_consumes_api_matrix() {
    let packet =
        current_certification_qualification().expect("embedded certification packet must parse");
    let consumed = packet.upstream_packet_refs.iter().any(|row| {
        row.upstream_record_kind
            == "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix"
            && row.integration_verified
    });
    assert!(
        consumed,
        "certification scorecard must reference the API-collection matrix packet"
    );
}

#[test]
fn embedded_certification_packet_parses() {
    let packet =
        current_certification_qualification().expect("embedded certification packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.mutation_drills.is_empty());
    assert!(!packet.redaction_drills.is_empty());
    assert!(!packet.scale_drills.is_empty());
    assert!(!packet.upstream_packet_refs.is_empty());
}

#[test]
fn embedded_certification_packet_has_no_violations() {
    let packet =
        current_certification_qualification().expect("embedded certification packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_certification_summary_matches_computed() {
    let packet =
        current_certification_qualification().expect("embedded certification packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_persisted_op_packet_parses() {
    let packet = current_persisted_operation_qualification()
        .expect("embedded persisted-operation packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.details.is_empty());
    assert!(!packet.review_sheets.is_empty());
    assert!(!packet.review_choices.is_empty());
    assert!(!packet.upstream_refs.is_empty());
}

#[test]
fn embedded_persisted_op_packet_has_no_violations() {
    let packet = current_persisted_operation_qualification()
        .expect("embedded persisted-operation packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_persisted_op_summary_matches_computed() {
    let packet = current_persisted_operation_qualification()
        .expect("embedded persisted-operation packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn persisted_op_consumes_api_matrix_and_blocks_unsafe_fallback() {
    let packet = current_persisted_operation_qualification()
        .expect("embedded persisted-operation packet must parse");
    // The detail lane is a real consumer of the frozen API-collection matrix.
    let consumes_matrix = packet.upstream_refs.iter().any(|row| {
        row.upstream_record_kind
            == "freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix"
            && row.integration_verified
    });
    assert!(
        consumes_matrix,
        "persisted-operation detail must reference the API-collection matrix packet"
    );
    // Drift and deprecation surface for review rather than hiding.
    let drifted = packet.drifted_detail_ids();
    assert!(drifted.contains(&"detail:deprecated".to_owned()));
    assert!(drifted.contains(&"detail:hash_drift".to_owned()));
    // Material mismatches block the send instead of silently falling back.
    let material = packet.material_mismatch_detail_ids();
    assert!(material.contains(&"detail:hash_drift".to_owned()));
    assert!(material.contains(&"detail:id_drift".to_owned()));
    assert!(material.contains(&"detail:removed".to_owned()));
    // A raw send after a mismatch is only reachable through an explicit downgrade.
    let downgrades = packet.explicit_downgrade_choice_ids();
    assert!(downgrades.contains(&"choice:hash_downgrade".to_owned()));
    assert_eq!(downgrades.len(), 3);
    let blocked = packet.send_blocked_sheet_ids();
    assert!(blocked.contains(&"sheet:hash_drift".to_owned()));
    assert!(blocked.contains(&"sheet:removed".to_owned()));
}
