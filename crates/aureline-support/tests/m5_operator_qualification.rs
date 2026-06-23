//! Freeze gate for the operator-surface qualification packet.
//!
//! The checked-in fixture
//! `fixtures/ops/m5-operator-qualification/canonical_packet.json` is the
//! published packet. This gate rebuilds the packet in code from the real in-code
//! proof sources and asserts it equals the fixture after a serialize round-trip,
//! so the certified state cannot drift from the published artifact without
//! failing CI. It also re-proves every frozen invariant, support-export safety,
//! full family coverage, the explicit release-evidence rows, and the
//! auto-narrowing behavior that downgrades a family when its operator-surface
//! proof is stale or failing.

use std::path::{Path, PathBuf};

use aureline_support::m5_operator_qualification::{
    operator_qualification_lines, operator_qualification_packet, project_operator_qualification,
    ClaimSupportClass, OperatorQualificationPacket, ProofDimension, ProofInput, ProofState,
    DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS, M5_OPERATOR_QUALIFICATION_RECORD_KIND,
    M5_OPERATOR_QUALIFICATION_SCHEMA_REF,
};
use aureline_support::m5_operator_surfaces::OperatorSurfaceClass;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-operator-qualification/canonical_packet.json")
}

fn load_fixture() -> OperatorQualificationPacket {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_packet_matches_checked_in_fixture() {
    let built = operator_qualification_packet();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code qualification packet drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_operator_qualification`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_OPERATOR_QUALIFICATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_OPERATOR_QUALIFICATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: OperatorQualificationPacket =
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
fn packet_certifies_every_claimed_family() {
    let fixture = load_fixture();
    assert_eq!(fixture.families.len(), OperatorSurfaceClass::ALL.len());
    for surface in OperatorSurfaceClass::ALL {
        assert!(
            fixture.family(surface).is_some(),
            "missing family {}",
            surface.as_str()
        );
    }
    assert_eq!(fixture.dimensions.len(), ProofDimension::ALL.len());
}

#[test]
fn release_evidence_dimensions_are_present() {
    let fixture = load_fixture();
    for dimension in [
        ProofDimension::ServiceOwnership,
        ProofDimension::RunbookStepAuthority,
        ProofDimension::HandoffBundleFidelity,
        ProofDimension::MaintenanceFailoverCommunication,
        ProofDimension::EmbeddedBoundaryHonesty,
    ] {
        assert!(
            fixture.dimension(dimension).is_some(),
            "release-evidence dimension {} must be present",
            dimension.as_str()
        );
    }
}

#[test]
fn stale_operator_surface_proof_auto_narrows_the_affected_family() {
    // Start from canonical-like inputs but age out one operator-surface proof,
    // proving a family cannot stay green while its evidence silently ages.
    let mut inputs = canonical_like_inputs();
    for input in &mut inputs {
        if input.dimension == ProofDimension::MaintenanceFailoverCommunication {
            input.captured_as_of = Some("2025-01-01T00:00:00Z".to_owned());
        }
    }
    let packet = project_operator_qualification("2026-06-22T00:00:00Z", &inputs);
    assert_eq!(
        packet
            .dimension(ProofDimension::MaintenanceFailoverCommunication)
            .unwrap()
            .state,
        ProofState::Stale
    );
    let maintenance = packet
        .family(OperatorSurfaceClass::MaintenanceNotice)
        .unwrap();
    assert_eq!(maintenance.support, ClaimSupportClass::Narrowed);
    assert!(maintenance
        .narrowed_by
        .contains(&ProofDimension::MaintenanceFailoverCommunication));
    assert!(packet.all_invariants_hold());
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = operator_qualification_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator-surface qualification")));
    assert!(lines.iter().any(|line| line.contains("Families:")));
    for surface in OperatorSurfaceClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(surface.as_str())),
            "projection must mention family {}",
            surface.as_str()
        );
    }
}

/// Fresh, passing inputs for every dimension, mirroring the canonical binding's
/// shape so the auto-narrow gate exercises the real projection.
fn canonical_like_inputs() -> Vec<ProofInput> {
    ProofDimension::ALL
        .iter()
        .map(|dimension| ProofInput {
            dimension: *dimension,
            proof_source_ref: "schemas/ops/m5-operator-surfaces.schema.json".to_owned(),
            contributing_proof_refs: vec![
                "schemas/ops/m5-operator-surfaces.schema.json".to_owned()
            ],
            captured_as_of: Some("2026-06-22T00:00:00Z".to_owned()),
            passing: true,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "canonical-like".to_owned(),
        })
        .collect()
}
