//! Fixture-driven tests for the M5 install-and-update diagnostics packet.

use std::path::{Path, PathBuf};

use aureline_install::{
    current_m5_install_portability_governance_matrix, current_m5_install_update_diagnostics,
    DiagnosticDrill, DiagnosticIncident, DiagnosticsConsumer, M5ArtifactFamily,
    M5InstallUpdateDiagnostics,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/install/m5/m5-install-diagnostics")
}

fn packet() -> M5InstallUpdateDiagnostics {
    current_m5_install_update_diagnostics().expect("embedded diagnostics packet parses")
}

#[test]
fn embedded_packet_validates_clean() {
    let packet = packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "diagnostics packet failed validation: {violations:#?}"
    );
}

#[test]
fn every_artifact_family_is_covered() {
    let packet = packet();
    for family in M5ArtifactFamily::ALL {
        assert!(
            packet.artifact_row(family).is_some(),
            "missing diagnostics row for {}",
            family.as_str()
        );
    }
}

#[test]
fn diagnostics_is_bound_to_the_governance_matrix() {
    let packet = packet();
    let matrix = current_m5_install_portability_governance_matrix().expect("governance parses");
    assert_eq!(packet.governance_packet_id_ref, matrix.packet_id);
    for row in &packet.artifacts {
        let lane_row = matrix
            .lane_row(row.governs_lane)
            .expect("governance lane exists");
        assert_eq!(row.governs_assurance, lane_row.published_assurance);
        assert!(
            row.published_support.rank() <= lane_row.published_assurance.rank(),
            "{} claims support beyond its governance lane",
            row.artifact_id
        );
    }
}

#[test]
fn drill_fixtures_match_the_embedded_packet() {
    let packet = packet();
    let dir = fixture_dir();
    let cases = [
        ("drill_root_mismatch.json", DiagnosticIncident::RootMismatch),
        (
            "drill_stale_verification.json",
            DiagnosticIncident::StaleVerification,
        ),
        (
            "drill_missing_rollback_target.json",
            DiagnosticIncident::MissingRollbackTarget,
        ),
        (
            "drill_wrong_root_support.json",
            DiagnosticIncident::WrongRootSupport,
        ),
    ];
    for (file, incident) in cases {
        let bytes = std::fs::read(dir.join(file)).unwrap_or_else(|_| panic!("read {file}"));
        let drill: DiagnosticDrill =
            serde_json::from_slice(&bytes).unwrap_or_else(|_| panic!("parse {file}"));
        assert_eq!(drill.incident, incident, "{file} has the wrong incident");
        assert!(drill.detected, "{file} must detect its incident");
        // The drill targets a real artifact row, and the embedded packet carries the same drill.
        assert!(
            packet.artifact_row_by_id(&drill.artifact_id).is_some(),
            "{file} targets an unknown artifact"
        );
        let embedded = packet
            .drills
            .iter()
            .find(|d| d.drill_id == drill.drill_id)
            .unwrap_or_else(|| panic!("embedded packet missing drill {}", drill.drill_id));
        assert_eq!(embedded, &drill, "{file} drifted from the embedded packet");
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for consumer in DiagnosticsConsumer::REQUIRED {
        assert!(
            packet.has_binding_for(consumer),
            "missing binding for {}",
            consumer.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_clean() {
    let packet = packet();
    let export = packet.support_export("support-export-fixture", "2026-06-11T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.diagnostics_packet_id_ref, packet.packet_id);
    // The export serializes and the embedded diagnostics packet still validates clean.
    serde_json::to_string(&export).expect("serialize export");
    assert!(export.diagnostics.validate().is_empty());
}
