//! Unit coverage for plan and validation viewer packets.

use super::*;

#[test]
fn valid_packet_passes() {
    let report = seeded_plan_and_validation_viewer_packet().validate();
    assert!(report.passed, "expected pass: {:#?}", report.findings);
    assert_eq!(report.viewer_kinds.len(), 5);
    assert!(report
        .consumer_surfaces
        .contains(&ViewerConsumerSurface::Review));
    assert!(report
        .consumer_surfaces
        .contains(&ViewerConsumerSurface::Incident));
    assert!(report
        .consumer_surfaces
        .contains(&ViewerConsumerSurface::SupportExport));
}

#[test]
fn hidden_live_authority_is_rejected() {
    let mut packet = seeded_plan_and_validation_viewer_packet();
    packet.viewer_records[0].inherits_live_authority_from_viewer = true;
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "hidden_live_authority"));
}

#[test]
fn missing_tool_identity_is_rejected() {
    let mut packet = seeded_plan_and_validation_viewer_packet();
    packet.viewer_records[0].tool_identity.tool_version.clear();
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "tool_identity"));
}

#[test]
fn mutate_adjacent_view_requires_review_before_apply() {
    let mut packet = seeded_plan_and_validation_viewer_packet();
    packet.viewer_records[0].review_before_apply_required = false;
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "review_before_apply"));
}

#[test]
fn provider_handoff_gate_requires_handoff_ref() {
    let mut packet = seeded_plan_and_validation_viewer_packet();
    let gate = packet
        .follow_up_gates
        .iter_mut()
        .find(|gate| gate.gate_id == "gate:policy:admission_handoff")
        .expect("policy handoff gate");
    gate.handoff_ref = None;
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "gate_missing_handoff"));
}

#[test]
fn review_join_must_cover_all_viewers() {
    let mut packet = seeded_plan_and_validation_viewer_packet();
    let join = packet
        .consumer_joins
        .iter_mut()
        .find(|join| join.surface == ViewerConsumerSurface::Review)
        .expect("review join");
    join.viewer_refs.pop();
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "join_coverage"));
}
