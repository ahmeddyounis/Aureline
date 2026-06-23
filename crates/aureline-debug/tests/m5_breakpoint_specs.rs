//! Freeze gate for the M5 breakpoint-spec set.
//!
//! The checked-in fixture `fixtures/debug/m5_breakpoint_specs/canonical_set.json` is
//! the published set. This gate rebuilds the set in code and asserts it equals the
//! fixture after a serialize round-trip, so the breakpoint contract cannot drift from
//! the published artifact without failing CI. It also re-proves support-export safety,
//! that the full verification and mapping vocabulary is materialized, that a green
//! confirmed-stop icon never hides a caveat, that a lost source identity degrades to
//! needs-remap, that a lexical fallback never poses as exact, that notebook and replay
//! scopes keep their stable anchors, that every cited proof packet and producing
//! module exists on disk, and every frozen invariant. This test runs under `cargo test
//! --workspace`, so stable promotion cannot harden a breakpoint claim without current
//! proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_breakpoint_specs::{
    m5_breakpoint_spec_set, BreakpointMappingProvenance, BreakpointMappingState,
    BreakpointScopeClass, BreakpointSpecSet, BreakpointVerificationState,
    M5_BREAKPOINT_SPECS_RECORD_KIND, M5_BREAKPOINT_SPECS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_breakpoint_specs/canonical_set.json")
}

fn load_fixture() -> BreakpointSpecSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_breakpoint_spec_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code breakpoint-spec set drifted from the checked-in fixture; regenerate it with \
         `cargo run -p aureline-debug --example dump_m5_breakpoint_specs > \
         fixtures/debug/m5_breakpoint_specs/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_BREAKPOINT_SPECS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_BREAKPOINT_SPECS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: BreakpointSpecSet =
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
fn set_materializes_every_verification_and_mapping_state() {
    let fixture = load_fixture();
    for state in BreakpointVerificationState::ALL {
        assert!(
            fixture.in_verification_state(state).is_some(),
            "missing verification state {}",
            state.as_str()
        );
    }
    for mapping in BreakpointMappingState::ALL {
        assert!(
            fixture.in_mapping_state(mapping).is_some(),
            "missing mapping state {}",
            mapping.as_str()
        );
    }
}

#[test]
fn green_icon_never_hides_a_caveat() {
    let fixture = load_fixture();
    for bp in &fixture.breakpoints {
        if bp.pill.shows_clean_confirmed {
            assert_eq!(bp.verification(), BreakpointVerificationState::Verified);
            assert_eq!(bp.mapping(), BreakpointMappingState::Exact);
            assert!(!bp.scope.is_replay_only());
            assert!(!bp.pill.requires_disclosure);
        } else {
            assert!(bp.pill.requires_disclosure);
        }
    }
}

#[test]
fn lost_identity_breakpoints_stay_visible_as_needs_remap() {
    let fixture = load_fixture();
    let lost: Vec<_> = fixture
        .breakpoints
        .iter()
        .filter(|b| b.mapping_provenance == BreakpointMappingProvenance::SourceIdentityLost)
        .collect();
    assert!(!lost.is_empty(), "a lost-identity case must exist");
    for bp in lost {
        assert_eq!(bp.mapping(), BreakpointMappingState::NeedsRemap);
    }
}

#[test]
fn notebook_and_replay_scopes_keep_their_anchors() {
    let fixture = load_fixture();
    for bp in &fixture.breakpoints {
        if bp.scope == BreakpointScopeClass::NotebookCell {
            assert!(
                bp.notebook_anchor.is_some(),
                "notebook breakpoint {} must keep its cell anchor",
                bp.breakpoint_id
            );
        }
        if bp.scope == BreakpointScopeClass::ReplayTimeline {
            assert!(
                bp.replay_anchor.is_some() && bp.pill.is_replay_only,
                "replay breakpoint {} must keep its frame anchor and stay replay-only",
                bp.breakpoint_id
            );
        }
    }
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
    for bp in &fixture.breakpoints {
        assert!(
            root.join(&bp.proof_packet_ref).exists(),
            "breakpoint {} proof packet {} does not exist",
            bp.breakpoint_id,
            bp.proof_packet_ref
        );
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_breakpoint_specs.md",
        "schemas/debug/m5_breakpoint_specs.schema.json",
        "artifacts/debug/m5_breakpoint_specs.md",
        "fixtures/debug/m5_breakpoint_specs/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
