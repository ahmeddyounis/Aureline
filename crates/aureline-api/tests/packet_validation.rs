//! Integration test: the embedded qualification packets parse and validate.

use aureline_api::{
    current_database_browser_qualification, current_request_composer_qualification,
    current_request_workspace_qualification, current_response_viewer_qualification,
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
    let packet = current_request_composer_qualification().expect("embedded composer packet must parse");
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
    let packet = current_request_composer_qualification().expect("embedded composer packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_composer_summary_matches_computed() {
    let packet = current_request_composer_qualification().expect("embedded composer packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_response_viewer_packet_parses() {
    let packet = current_response_viewer_qualification().expect("embedded response viewer packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.response_viewers.is_empty());
    assert!(!packet.assertions.is_empty());
    assert!(!packet.timing_tabs.is_empty());
    assert!(!packet.browser_runtime_trusts.is_empty());
}

#[test]
fn embedded_response_viewer_packet_has_no_violations() {
    let packet = current_response_viewer_qualification().expect("embedded response viewer packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_response_viewer_summary_matches_computed() {
    let packet = current_response_viewer_qualification().expect("embedded response viewer packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_database_browser_packet_parses() {
    let packet = current_database_browser_qualification().expect("embedded database browser packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.connection_browsers.is_empty());
    assert!(!packet.schema_trees.is_empty());
    assert!(!packet.target_context_envelopes.is_empty());
}

#[test]
fn embedded_database_browser_packet_has_no_violations() {
    let packet = current_database_browser_qualification().expect("embedded database browser packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_database_browser_summary_matches_computed() {
    let packet = current_database_browser_qualification().expect("embedded database browser packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}
