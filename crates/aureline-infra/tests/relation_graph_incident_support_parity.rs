//! Integration coverage for infrastructure relation-graph incident/support parity packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    seeded_relation_graph_incident_support_parity_packet, ConnectorSkewState, InfrastructureFamily,
    LocalityMismatchState, ParityDrillClass, RelationGraphIncidentSupportParityPacket,
    RelationGraphParitySurface, RELATION_GRAPH_PARITY_PACKET_RECORD_KIND,
    RELATION_GRAPH_PARITY_SCHEMA_VERSION,
};

#[test]
fn qualified_fixture_validates() {
    let packet = load_fixture("qualified_parity_packet.json");
    let report = packet.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(packet.record_kind, RELATION_GRAPH_PARITY_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, RELATION_GRAPH_PARITY_SCHEMA_VERSION);
}

#[test]
fn qualified_fixture_matches_seeded_packet() {
    let fixture = load_fixture("qualified_parity_packet.json");
    let seeded = seeded_relation_graph_incident_support_parity_packet();
    assert_eq!(fixture, seeded);
}

#[test]
fn qualified_fixture_covers_all_surfaces_and_drills() {
    let report = load_fixture("qualified_parity_packet.json").validate();
    for required in [
        RelationGraphParitySurface::IncidentPacket,
        RelationGraphParitySurface::SupportExport,
        RelationGraphParitySurface::ProofCorpus,
    ] {
        assert!(report.consumer_surfaces.contains(&required));
    }
    for required in [
        ParityDrillClass::WrongTarget,
        ParityDrillClass::StaleLiveOverlay,
        ParityDrillClass::MissingPermission,
        ParityDrillClass::ConnectorSkew,
        ParityDrillClass::LocalityMismatch,
    ] {
        assert!(report.drill_classes.contains(&required));
    }
}

#[test]
fn policy_selection_keeps_permission_and_skew_explicit() {
    let packet = load_fixture("qualified_parity_packet.json");
    let selection = packet
        .relation_graph_selections
        .iter()
        .find(|selection| selection.family == InfrastructureFamily::PolicyManifest)
        .expect("policy selection");
    assert_eq!(selection.connector_skew_state, ConnectorSkewState::UnsupportedSkew);
    assert_eq!(
        selection.locality_mismatch_state,
        LocalityMismatchState::LocalVsManaged
    );
}

#[test]
fn artifact_support_export_matches_seeded_packet() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/infra/relation-graph-incident-support-parity-support-export.json");
    let payload = fs::read_to_string(path).expect("artifact exists");
    let artifact: RelationGraphIncidentSupportParityPacket =
        serde_json::from_str(&payload).expect("artifact parses");
    assert_eq!(artifact, seeded_relation_graph_incident_support_parity_packet());
}

#[test]
fn missing_drill_fixture_fails_validation() {
    let report = load_fixture("missing_connector_skew_drill_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "drill_class_coverage"));
}

#[test]
fn permission_limited_binding_fixture_fails_validation() {
    let report = load_fixture("permission_limited_binding_dropped_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "binding_permission_limited"));
}

fn load_fixture(name: &str) -> RelationGraphIncidentSupportParityPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/relation-graph-incident-support-parity")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
