//! Integration coverage for runtime-continuity surface qualification packets.

use std::fs;
use std::path::PathBuf;

use aureline_runtime::{
    current_runtime_continuity_surface_qualification_export,
    seeded_runtime_continuity_surface_qualification_packet, RuntimeContinuityEvidenceConsumer,
    RuntimeContinuityProfile, RuntimeContinuitySurfaceQualificationPacket,
    RuntimeContinuitySurfaceQualificationViolation,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND,
    RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

#[test]
fn seeded_packet_validates() {
    let packet = seeded_runtime_continuity_surface_qualification_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:#?}"
    );
    assert_eq!(
        packet.record_kind,
        RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        RUNTIME_CONTINUITY_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
}

#[test]
fn seeded_packet_covers_all_profiles_and_consumers() {
    let packet = seeded_runtime_continuity_surface_qualification_packet();
    for required in RuntimeContinuityProfile::ALL {
        assert!(packet
            .profile_rows
            .iter()
            .any(|row| row.profile == required));
        assert!(packet
            .evidence_index
            .iter()
            .any(|entry| entry.profile == required));
    }
    for required in RuntimeContinuityEvidenceConsumer::ALL {
        assert!(packet
            .consumer_bindings
            .iter()
            .any(|binding| binding.consumer == required));
    }
}

#[test]
fn checked_artifact_matches_seeded_packet() {
    let current = current_runtime_continuity_surface_qualification_export()
        .expect("checked support export validates");
    assert_eq!(
        current,
        seeded_runtime_continuity_surface_qualification_packet()
    );
}

#[test]
fn browser_handoff_widening_fixture_fails() {
    let packet = load_fixture("browser_handoff_widened_packet.json");
    let violations = packet.validate();
    assert!(violations
        .contains(&RuntimeContinuitySurfaceQualificationViolation::DisplayedLabelMismatch));
}

#[test]
fn stale_profile_fixture_fails() {
    let packet = load_fixture("stale_managed_profile_packet.json");
    let violations = packet.validate();
    assert!(violations
        .contains(&RuntimeContinuitySurfaceQualificationViolation::DisplayedLabelMismatch));
}

#[test]
fn consumer_binding_fixture_fails() {
    let packet = load_fixture("missing_consumer_binding_packet.json");
    let violations = packet.validate();
    assert!(violations
        .contains(&RuntimeContinuitySurfaceQualificationViolation::ConsumerBindingIncomplete));
}

fn load_fixture(name: &str) -> RuntimeContinuitySurfaceQualificationPacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/runtime-continuity-surface-qualification")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
