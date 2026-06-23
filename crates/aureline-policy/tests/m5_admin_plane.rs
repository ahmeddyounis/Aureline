//! Freeze gate for the M5 admin-plane matrix.
//!
//! The checked-in fixture `fixtures/admin/m5-admin-plane/canonical_matrix.json`
//! is the published matrix. This gate rebuilds the matrix in code and asserts it
//! equals the fixture after a serialize round-trip, so the admin-plane contract
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full surface/path/state coverage, that every named
//! controlled vocabulary is bound, that every surface maps to a proof packet, and
//! every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{
    admin_plane_lines, admin_plane_matrix, AdminPathClass, AdminPlaneMatrix, AdminStateClass,
    AdminSurfaceClass, ControlledVocabulary, M5_ADMIN_PLANE_RECORD_KIND, M5_ADMIN_PLANE_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-admin-plane/canonical_matrix.json")
}

fn load_fixture() -> AdminPlaneMatrix {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_matrix_matches_checked_in_fixture() {
    let built = admin_plane_matrix();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code admin-plane matrix drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-policy --example dump_m5_admin_plane`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ADMIN_PLANE_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ADMIN_PLANE_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: AdminPlaneMatrix =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn matrix_covers_every_surface_path_and_state() {
    let fixture = load_fixture();
    assert_eq!(fixture.surfaces.len(), AdminSurfaceClass::ALL.len());
    for surface in AdminSurfaceClass::ALL {
        assert!(
            fixture.surface(surface).is_some(),
            "missing surface {}",
            surface.as_str()
        );
    }
    assert_eq!(fixture.admin_paths.len(), AdminPathClass::ALL.len());
    for path in AdminPathClass::ALL {
        assert!(
            fixture.path(path).is_some(),
            "missing admin path {}",
            path.as_str()
        );
    }
    assert_eq!(fixture.state_vocabulary.len(), AdminStateClass::ALL.len());
    for state in AdminStateClass::ALL {
        assert!(
            fixture.state_term(state).is_some(),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn matrix_covers_all_six_admin_paths_explicitly() {
    let fixture = load_fixture();
    for path in [
        AdminPathClass::LocalIndividual,
        AdminPathClass::ManagedCloud,
        AdminPathClass::SelfHosted,
        AdminPathClass::SovereignAirGapped,
        AdminPathClass::MirroredOffline,
        AdminPathClass::ImportedSnapshot,
    ] {
        let entry = fixture.path(path).expect("path present");
        assert!(!entry.deployment_profiles.is_empty());
        assert!(!entry.local_safe_baseline_ref.is_empty());
    }
}

#[test]
fn every_named_controlled_vocabulary_is_bound() {
    let fixture = load_fixture();
    for vocab in ControlledVocabulary::ALL {
        assert!(
            fixture.surfaces.iter().any(|s| s.binds(vocab)),
            "controlled vocabulary {} bound by no surface",
            vocab.as_str()
        );
    }
}

#[test]
fn every_surface_maps_to_a_proof_packet() {
    let fixture = load_fixture();
    for surface in &fixture.surfaces {
        assert!(
            !surface.proof_packet_ref.is_empty(),
            "surface {} lacks a mapped proof packet",
            surface.surface.as_str()
        );
    }
    // The same fact, surfaced as a computed invariant for release automation.
    let proof_invariant = fixture
        .invariants
        .iter()
        .find(|i| i.invariant_id == "admin_plane.proof_packet_mapped")
        .expect("proof-packet invariant present");
    assert!(proof_invariant.holds);
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = admin_plane_lines(&fixture);
    assert!(lines.iter().any(|line| line.contains("Admin-plane matrix")));
    assert!(lines.iter().any(|line| line.contains("Surfaces:")));
    for surface in AdminSurfaceClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(surface.as_str())),
            "projection must mention surface {}",
            surface.as_str()
        );
    }
}
