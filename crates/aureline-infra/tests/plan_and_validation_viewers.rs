//! Integration coverage for plan and validation viewer packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    seeded_plan_and_validation_viewer_packet, InfrastructureFamily, PlanAndValidationViewerPacket,
    PlanValidationViewerKind, ViewerConsumerSurface, PLAN_AND_VALIDATION_VIEWER_PACKET_RECORD_KIND,
    PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION,
};

#[test]
fn qualified_fixture_validates() {
    let packet = load_fixture("qualified_viewer_packet.json");
    let report = packet.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(
        packet.record_kind,
        PLAN_AND_VALIDATION_VIEWER_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION
    );
}

#[test]
fn qualified_fixture_covers_viewers_and_consumers() {
    let report = load_fixture("qualified_viewer_packet.json").validate();
    for required in [
        PlanValidationViewerKind::Plan,
        PlanValidationViewerKind::Diff,
        PlanValidationViewerKind::DryRun,
        PlanValidationViewerKind::Admission,
        PlanValidationViewerKind::PolicyCheck,
    ] {
        assert!(report.viewer_kinds.contains(&required));
    }
    for required in [
        ViewerConsumerSurface::Review,
        ViewerConsumerSurface::Incident,
        ViewerConsumerSurface::SupportExport,
    ] {
        assert!(report.consumer_surfaces.contains(&required));
    }
    for required in [
        InfrastructureFamily::TerraformHcl,
        InfrastructureFamily::KubernetesHelm,
        InfrastructureFamily::PolicyManifest,
        InfrastructureFamily::CiEnvironment,
    ] {
        assert!(report.families.contains(&required));
    }
}

#[test]
fn qualified_fixture_matches_seeded_packet() {
    let fixture = load_fixture("qualified_viewer_packet.json");
    let seeded = seeded_plan_and_validation_viewer_packet();
    assert_eq!(fixture, seeded);
}

#[test]
fn hidden_live_authority_fixture_fails() {
    let report = load_fixture("hidden_live_authority_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "hidden_live_authority"));
}

#[test]
fn missing_tool_identity_and_review_gate_fixture_fails() {
    let report = load_fixture("missing_tool_identity_and_review_gate_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "tool_identity"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "review_before_apply"));
}

fn load_fixture(name: &str) -> PlanAndValidationViewerPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/plan-and-validation-viewers")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
