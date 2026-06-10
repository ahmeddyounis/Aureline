//! Integration test: the embedded qualification packets parse and validate.

use aureline_profiler::current_profile_launcher_qualification;

#[test]
fn embedded_profile_launcher_packet_parses() {
    let packet = current_profile_launcher_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.launchers.is_empty());
    assert!(!packet.attach_sheets.is_empty());
    assert!(!packet.capture_modes.is_empty());
    assert!(!packet.storage_locations.is_empty());
}

#[test]
fn embedded_profile_launcher_packet_has_no_violations() {
    let packet = current_profile_launcher_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_profile_launcher_summary_matches_computed() {
    let packet = current_profile_launcher_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}
