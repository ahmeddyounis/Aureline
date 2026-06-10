//! Integration test: the embedded qualification packet parses and validates.

use aureline_api::current_request_workspace_qualification;

#[test]
fn embedded_packet_parses() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.documents.is_empty());
    assert!(!packet.environment_sets.is_empty());
    assert!(!packet.auth_sources.is_empty());
}

#[test]
fn embedded_packet_has_no_violations() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn summary_matches_computed() {
    let packet = current_request_workspace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}
