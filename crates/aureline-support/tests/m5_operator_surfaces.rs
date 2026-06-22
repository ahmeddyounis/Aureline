//! Freeze gate for the M5 operator-surface matrix.
//!
//! The checked-in fixture
//! `fixtures/ops/m5-operator-surfaces/canonical_matrix.json` is the published
//! matrix. This gate rebuilds the matrix in code and asserts it equals the
//! fixture after a serialize round-trip, so the operator-surface contract
//! cannot drift from the published artifact without failing CI. It also
//! re-proves support-export safety, full surface/path/state coverage, and every
//! frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_operator_surfaces::{
    operator_surface_lines, operator_surface_matrix, OperatorPathClass, OperatorStateClass,
    OperatorSurfaceClass, OperatorSurfaceMatrix, M5_OPERATOR_SURFACES_RECORD_KIND,
    M5_OPERATOR_SURFACES_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-operator-surfaces/canonical_matrix.json")
}

fn load_fixture() -> OperatorSurfaceMatrix {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_matrix_matches_checked_in_fixture() {
    let built = operator_surface_matrix();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code operator-surface matrix drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_operator_surfaces`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_OPERATOR_SURFACES_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_OPERATOR_SURFACES_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: OperatorSurfaceMatrix =
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
    assert_eq!(fixture.surfaces.len(), OperatorSurfaceClass::ALL.len());
    for surface in OperatorSurfaceClass::ALL {
        assert!(
            fixture.surface(surface).is_some(),
            "missing surface {}",
            surface.as_str()
        );
    }
    assert_eq!(fixture.operator_paths.len(), OperatorPathClass::ALL.len());
    for path in OperatorPathClass::ALL {
        assert!(
            fixture.path(path).is_some(),
            "missing operator path {}",
            path.as_str()
        );
    }
    assert_eq!(
        fixture.state_vocabulary.len(),
        OperatorStateClass::ALL.len()
    );
    for state in OperatorStateClass::ALL {
        assert!(
            fixture.state_term(state).is_some(),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn matrix_covers_all_six_operator_paths_explicitly() {
    let fixture = load_fixture();
    for path in [
        OperatorPathClass::Local,
        OperatorPathClass::Remote,
        OperatorPathClass::Managed,
        OperatorPathClass::MirroredOffline,
        OperatorPathClass::BrowserWebview,
        OperatorPathClass::ImportedSnapshot,
    ] {
        let entry = fixture.path(path).expect("path present");
        assert!(!entry.deployment_profiles.is_empty());
        assert!(!entry.local_safe_baseline_ref.is_empty());
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = operator_surface_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator-surface matrix")));
    assert!(lines.iter().any(|line| line.contains("Surfaces:")));
    for surface in OperatorSurfaceClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(surface.as_str())),
            "projection must mention surface {}",
            surface.as_str()
        );
    }
}
