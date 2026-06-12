//! Unit coverage for provider-overlay and vendor-console handoff continuity packets.

use super::*;

#[test]
fn valid_packet_passes() {
    let report = seeded_provider_overlay_handoff_packet().validate();
    assert!(report.passed, "expected pass: {:#?}", report.findings);
    assert_eq!(report.overlay_row_ids.len(), 3);
    assert_eq!(report.handoff_ids.len(), 3);
    assert!(report
        .surfaces
        .contains(&OverlayContinuitySurface::InfrastructurePanel));
}

#[test]
fn hiding_canonical_truth_fails() {
    let mut packet = seeded_provider_overlay_handoff_packet();
    packet.overlay_rows[0].canonical_truth_visible = false;
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "overlay_truth_disclosure"));
}

#[test]
fn generic_shell_return_fails() {
    let mut packet = seeded_provider_overlay_handoff_packet();
    packet.handoff_rows[0]
        .return_anchor
        .generic_shell_reopen_forbidden = false;
    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "handoff_return_anchor"));
}

#[test]
fn missing_surface_handoff_parity_fails() {
    let mut packet = seeded_provider_overlay_handoff_packet();
    let preview = packet
        .surface_bindings
        .iter_mut()
        .find(|binding| binding.surface == OverlayContinuitySurface::PreviewRoute)
        .expect("preview binding");
    preview.handoff_refs.clear();

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "surface_binding_refs"));
}
