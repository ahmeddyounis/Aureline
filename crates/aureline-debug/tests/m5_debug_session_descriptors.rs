//! Freeze gate for the M5 debug-session descriptor set.
//!
//! The checked-in fixture
//! `fixtures/debug/m5_debug_session_descriptors/canonical_set.json` is the published
//! set. This gate rebuilds the set in code and asserts it equals the fixture after a
//! serialize round-trip, so the descriptor contract cannot drift from the published
//! artifact without failing CI. It also re-proves support-export safety, that all
//! five session modes are materialized, that every session echoes its referenced
//! target's identity, that every cited proof packet and producing module exists on
//! disk, and every frozen invariant. This test runs under `cargo test --workspace`,
//! so stable promotion cannot harden a debugger claim without current proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_debug_session_descriptors::{
    m5_debug_session_descriptor_set, DebugSessionDescriptorSet, DebugSessionModeClass,
    M5_DEBUG_SESSION_DESCRIPTORS_RECORD_KIND, M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_debug_session_descriptors/canonical_set.json")
}

fn load_fixture() -> DebugSessionDescriptorSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_debug_session_descriptor_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code descriptor set drifted from the checked-in fixture; regenerate it with \
         `cargo run -p aureline-debug --example dump_m5_debug_session_descriptors > \
         fixtures/debug/m5_debug_session_descriptors/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.record_kind,
        M5_DEBUG_SESSION_DESCRIPTORS_RECORD_KIND
    );
    assert_eq!(fixture.schema_ref, M5_DEBUG_SESSION_DESCRIPTORS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: DebugSessionDescriptorSet =
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
fn set_materializes_every_session_mode() {
    let fixture = load_fixture();
    for mode in DebugSessionModeClass::ALL {
        assert!(
            fixture.session_in_mode(mode).is_some(),
            "missing session for mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn every_session_echoes_its_referenced_target() {
    let fixture = load_fixture();
    for session in &fixture.sessions {
        let target = fixture
            .target(&session.target_descriptor_ref)
            .unwrap_or_else(|| panic!("session {} resolves a target", session.session_id));
        assert!(
            session.target_identity_echo.matches(target),
            "session {} echo must match target {}",
            session.session_id,
            target.descriptor_id
        );
    }
}

#[test]
fn inspect_only_and_restored_sessions_hold_no_live_authority() {
    let fixture = load_fixture();
    for session in &fixture.sessions {
        if session.mode.is_inspect_only() {
            assert!(
                !session.holds_live_authority,
                "inspect-only session {} must not hold live authority",
                session.session_id
            );
        }
    }
    let restored = fixture
        .sessions
        .iter()
        .find(|s| {
            s.reentry_posture
                == aureline_debug::m5_debug_session_descriptors::ReentryPosture::RestoredLayoutOnly
        })
        .expect("a restored-layout session exists");
    assert!(
        !restored.holds_live_authority,
        "a restored-layout session must never silently hold live authority"
    );
}

#[test]
fn every_proof_packet_and_producer_exists_on_disk() {
    let root = repo_root();
    let fixture = load_fixture();
    for schema in &fixture.source_schema_refs {
        assert!(
            root.join(schema).exists(),
            "source schema {schema} does not exist"
        );
    }
    for producer in &fixture.producer_refs {
        assert!(
            root.join(producer).exists(),
            "producer {producer} does not exist"
        );
    }
    for target in &fixture.targets {
        assert!(
            root.join(&target.negotiation_evidence_ref).exists(),
            "target {} negotiation evidence {} does not exist",
            target.descriptor_id,
            target.negotiation_evidence_ref
        );
    }
    for session in &fixture.sessions {
        assert!(
            root.join(&session.proof_packet_ref).exists(),
            "session {} proof packet {} does not exist",
            session.session_id,
            session.proof_packet_ref
        );
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_debug_session_descriptors.md",
        "schemas/debug/m5_debug_session_descriptors.schema.json",
        "artifacts/debug/m5_debug_session_descriptors.md",
        "fixtures/debug/m5_debug_session_descriptors/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
