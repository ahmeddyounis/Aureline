//! Freeze gate for the M5 debug-contracts matrix.
//!
//! The checked-in fixture `fixtures/debug/m5_debug_contracts/canonical_matrix.json`
//! is the published matrix. This gate rebuilds the matrix in code and asserts it
//! equals the fixture after a serialize round-trip, so the debug-contracts contract
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full object/state coverage, that every named controlled
//! vocabulary is bound, that every object maps to a proof packet, that every mapped
//! proof packet and producing module exists on disk, and every frozen invariant.
//! This test runs under `cargo test --workspace`, so stable promotion cannot harden
//! a debugger claim without current proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_debug_contracts::{
    m5_debug_contracts_matrix, DebugObjectClass, DebugStateClass, DebugVocabulary,
    M5DebugContractsMatrix, M5_DEBUG_CONTRACTS_RECORD_KIND, M5_DEBUG_CONTRACTS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_debug_contracts/canonical_matrix.json")
}

fn load_fixture() -> M5DebugContractsMatrix {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_matrix_matches_checked_in_fixture() {
    let built = m5_debug_contracts_matrix();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code m5 debug-contracts matrix drifted from the checked-in fixture; regenerate it \
         with `cargo run -p aureline-debug --example dump_m5_debug_contracts > \
         fixtures/debug/m5_debug_contracts/canonical_matrix.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_DEBUG_CONTRACTS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_DEBUG_CONTRACTS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: M5DebugContractsMatrix =
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
fn matrix_covers_every_object_and_state() {
    let fixture = load_fixture();
    assert_eq!(fixture.objects.len(), DebugObjectClass::ALL.len());
    for object in DebugObjectClass::ALL {
        assert!(
            fixture.object(object).is_some(),
            "missing object {}",
            object.as_str()
        );
    }
    assert_eq!(fixture.state_vocabulary.len(), DebugStateClass::ALL.len());
    for state in DebugStateClass::ALL {
        assert!(
            fixture.state_term(state).is_some(),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn matrix_covers_all_ten_debug_objects_explicitly() {
    let fixture = load_fixture();
    for object in [
        DebugObjectClass::DebugSession,
        DebugObjectClass::AttachTarget,
        DebugObjectClass::BreakpointSpec,
        DebugObjectClass::FrameMapping,
        DebugObjectClass::VariableWatchSnapshot,
        DebugObjectClass::EvaluateRequestResult,
        DebugObjectClass::ConsoleEmission,
        DebugObjectClass::ChronologyCapability,
        DebugObjectClass::ReplaySession,
        DebugObjectClass::NotebookDebugParity,
    ] {
        let entry = fixture.object(object).expect("object present");
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.required_fields.is_empty());
        assert!(!entry.applicable_states.is_empty());
        assert!(!entry.consumed_by.is_empty());
    }
}

#[test]
fn every_named_controlled_vocabulary_is_bound() {
    let fixture = load_fixture();
    for vocab in DebugVocabulary::ALL {
        assert!(
            fixture.objects.iter().any(|o| o.binds(vocab)),
            "controlled vocabulary {} is bound by no object",
            vocab.as_str()
        );
    }
}

#[test]
fn every_object_maps_to_a_proof_packet() {
    let fixture = load_fixture();
    for object in &fixture.objects {
        assert!(
            !object.proof_packet_ref.is_empty(),
            "object {} lacks a mapped proof packet",
            object.object.as_str()
        );
    }
}

#[test]
fn every_proof_packet_and_producer_exists_on_disk() {
    let root = repo_root();
    let fixture = load_fixture();
    for object in &fixture.objects {
        let proof = root.join(&object.proof_packet_ref);
        assert!(
            proof.exists(),
            "object {} proof packet {} does not exist",
            object.object.as_str(),
            object.proof_packet_ref
        );
        for producer in &object.produced_by_refs {
            assert!(
                root.join(producer).exists(),
                "object {} producer {} does not exist",
                object.object.as_str(),
                producer
            );
        }
        for schema in &object.canonical_schema_refs {
            assert!(
                root.join(schema).exists(),
                "object {} schema {} does not exist",
                object.object.as_str(),
                schema
            );
        }
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_debug_contracts.md",
        "schemas/debug/m5_debug_contracts.schema.json",
        "artifacts/debug/m5_debug_contracts.md",
        "fixtures/debug/m5_debug_contracts/canonical_matrix.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
