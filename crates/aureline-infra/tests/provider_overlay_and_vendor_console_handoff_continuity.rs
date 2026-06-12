//! Integration coverage for provider-overlay and vendor-console handoff continuity packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    seeded_provider_overlay_handoff_packet, OverlayContinuitySurface,
    ProviderOverlayHandoffContinuityPacket, PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND,
    PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION,
};

#[test]
fn qualified_fixture_validates() {
    let packet = load_fixture("qualified_overlay_handoff_packet.json");
    let report = packet.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(
        packet.record_kind,
        PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION
    );
}

#[test]
fn qualified_fixture_matches_seeded_packet() {
    let fixture = load_fixture("qualified_overlay_handoff_packet.json");
    let seeded = seeded_provider_overlay_handoff_packet();
    assert_eq!(fixture, seeded);
}

#[test]
fn qualified_fixture_covers_required_surfaces() {
    let report = load_fixture("qualified_overlay_handoff_packet.json").validate();
    for required in [
        OverlayContinuitySurface::CodeBreadcrumb,
        OverlayContinuitySurface::IncidentWorkspace,
        OverlayContinuitySurface::PreviewRoute,
        OverlayContinuitySurface::RouteExplorer,
        OverlayContinuitySurface::InfrastructurePanel,
    ] {
        assert!(report.surfaces.contains(&required));
    }
}

#[test]
fn blurred_overlay_fixture_fails() {
    let report = load_fixture("blurred_overlay_truth_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "overlay_truth_disclosure"));
}

#[test]
fn generic_shell_return_fixture_fails() {
    let report = load_fixture("generic_shell_return_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "handoff_return_anchor"));
}

fn load_fixture(name: &str) -> ProviderOverlayHandoffContinuityPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/provider-overlay-and-vendor-console-handoff-continuity")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
