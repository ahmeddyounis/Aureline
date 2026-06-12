//! Integration coverage for infrastructure surface qualification packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    current_infrastructure_surface_qualification_export,
    seeded_infrastructure_surface_qualification_packet, InfrastructureEvidenceConsumer,
    InfrastructureSurface, InfrastructureSurfaceQualificationPacket,
    InfrastructureSurfaceQualificationViolation, INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND,
    INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

#[test]
fn seeded_packet_validates() {
    let packet = seeded_infrastructure_surface_qualification_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:#?}"
    );
    assert_eq!(
        packet.record_kind,
        INFRASTRUCTURE_SURFACE_QUALIFICATION_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        INFRASTRUCTURE_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
}

#[test]
fn seeded_packet_covers_all_surfaces_and_consumers() {
    let packet = seeded_infrastructure_surface_qualification_packet();
    for required in InfrastructureSurface::ALL {
        assert!(packet
            .surface_rows
            .iter()
            .any(|row| row.surface == required));
        assert!(packet
            .evidence_index
            .iter()
            .any(|entry| entry.surface == required));
    }
    for required in InfrastructureEvidenceConsumer::ALL {
        assert!(packet
            .consumer_bindings
            .iter()
            .any(|binding| binding.consumer == required));
    }
}

#[test]
fn checked_artifact_matches_seeded_packet() {
    let current = current_infrastructure_surface_qualification_export()
        .expect("checked support export validates");
    assert_eq!(
        current,
        seeded_infrastructure_surface_qualification_packet()
    );
}

#[test]
fn missing_relationship_fixture_fails() {
    let packet = load_fixture("missing_relationship_proof_packet.json");
    let violations = packet.validate();
    assert!(
        violations.contains(&InfrastructureSurfaceQualificationViolation::DisplayedPostureMismatch)
    );
}

#[test]
fn stale_public_index_fixture_fails() {
    let packet = load_fixture("stale_public_index_packet.json");
    let violations = packet.validate();
    assert!(
        violations.contains(&InfrastructureSurfaceQualificationViolation::DisplayedPostureMismatch)
    );
}

#[test]
fn consumer_binding_fixture_fails() {
    let packet = load_fixture("missing_consumer_binding_packet.json");
    let violations = packet.validate();
    assert!(violations
        .contains(&InfrastructureSurfaceQualificationViolation::ConsumerBindingIncomplete));
}

fn load_fixture(name: &str) -> InfrastructureSurfaceQualificationPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/infrastructure-surface-qualification")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
