//! Freeze gate for the editor-assist qualification packet.
//!
//! The checked-in fixture
//! `fixtures/editor/m5-assist-qualification/canonical_packet.json` is the
//! published packet. This gate rebuilds the packet in code from the real
//! in-code proof sources and asserts it equals the fixture after a serialize
//! round-trip, so the certified state cannot drift from the published artifact
//! without failing CI. It also re-proves every frozen invariant, support-export
//! safety, full family coverage, and the auto-narrowing behavior that downgrades
//! a family when its assist-surface proof is stale or failing.

use std::path::{Path, PathBuf};

use aureline_editor::{
    assist_qualification_lines, assist_qualification_packet, project_assist_qualification,
    AssistQualificationPacket, ClaimSupportClass, EditorSurfaceClass, ProofDimension, ProofInput,
    ProofState, DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS, M5_ASSIST_QUALIFICATION_RECORD_KIND,
    M5_ASSIST_QUALIFICATION_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-assist-qualification/canonical_packet.json")
}

fn load_fixture() -> AssistQualificationPacket {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_packet_matches_checked_in_fixture() {
    let built = assist_qualification_packet();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code qualification packet drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_assist_qualification`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ASSIST_QUALIFICATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ASSIST_QUALIFICATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: AssistQualificationPacket =
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
    assert_eq!(fixture.families.len(), EditorSurfaceClass::ALL.len());
    for surface in EditorSurfaceClass::ALL {
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
        ProofDimension::Completion,
        ProofDimension::Hint,
        ProofDimension::Hover,
        ProofDimension::Peek,
        ProofDimension::ConstrainedFileNarrowing,
        ProofDimension::ImeMultiCursorSafety,
        ProofDimension::AccessibilityParity,
    ] {
        assert!(
            fixture.dimension(dimension).is_some(),
            "release-evidence dimension {} must be present",
            dimension.as_str()
        );
    }
}

#[test]
fn stale_micro_surface_proof_auto_narrows_the_affected_family() {
    // Start from the same canonical inputs but age out one micro-surface proof,
    // proving a family cannot stay green while its evidence silently ages.
    let mut inputs = canonical_like_inputs();
    for input in &mut inputs {
        if input.dimension == ProofDimension::Hover {
            input.captured_as_of = Some("2025-01-01T00:00:00Z".to_owned());
        }
    }
    let packet = project_assist_qualification("2026-06-22T00:00:00Z", &inputs);
    assert_eq!(
        packet.dimension(ProofDimension::Hover).unwrap().state,
        ProofState::Stale
    );
    let code = packet.family(EditorSurfaceClass::CodeFile).unwrap();
    assert_eq!(code.support, ClaimSupportClass::Narrowed);
    assert!(code.narrowed_by.contains(&ProofDimension::Hover));
    assert!(packet.all_invariants_hold());
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = assist_qualification_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Editor-assist qualification")));
    assert!(lines.iter().any(|line| line.contains("Families:")));
    for surface in EditorSurfaceClass::ALL {
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
            proof_source_ref: "schemas/editor/m5-editor-assist.schema.json".to_owned(),
            contributing_proof_refs: vec!["schemas/editor/m5-editor-assist.schema.json".to_owned()],
            captured_as_of: Some("2026-06-22T00:00:00Z".to_owned()),
            passing: true,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "canonical-like".to_owned(),
        })
        .collect()
}
