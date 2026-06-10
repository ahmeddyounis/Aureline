//! Integration test: the embedded qualification packets parse and validate.

use aureline_api::{
    current_request_composer_qualification, current_request_workspace_qualification,
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
