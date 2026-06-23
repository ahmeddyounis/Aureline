//! Freeze gate for the M5 frame-mapping and variable/watch snapshot set.
//!
//! The checked-in fixture `fixtures/debug/m5_frame_variable_snapshots/canonical_set.json`
//! is the published set. This gate rebuilds the set in code and asserts it equals the
//! fixture after a serialize round-trip, so the frame/variable contract cannot drift from
//! the published artifact without failing CI. It also re-proves support-export safety,
//! that the full fidelity and disclosure vocabularies are materialized, that a precise
//! source link never hides a caveat, that current-frame identity is preserved per thread,
//! that a lost mapping degrades to an explicit unmapped frame, that a source-map mapping
//! always discloses, that a value implies live authority only when truly live, that
//! unavailable and redacted values withhold their bodies, that notebook and replay
//! inspectors reuse the snapshot vocabulary, that every cited proof packet and producing
//! module exists on disk, and every frozen invariant. This test runs under `cargo test
//! --workspace`, so stable promotion cannot harden a frame or value claim without current
//! proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_frame_variable_snapshots::{
    m5_frame_variable_snapshot_set, FrameContinuityClass, FrameMappingFidelity,
    FrameMappingProvenance, FrameVariableSnapshotSet, SnapshotEntryKind, ValueDisclosure,
    M5_FRAME_VARIABLE_SNAPSHOTS_RECORD_KIND, M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_frame_variable_snapshots/canonical_set.json")
}

fn load_fixture() -> FrameVariableSnapshotSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_frame_variable_snapshot_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code frame/variable snapshot set drifted from the checked-in fixture; regenerate \
         it with `cargo run -p aureline-debug --example dump_m5_frame_variable_snapshots > \
         fixtures/debug/m5_frame_variable_snapshots/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_FRAME_VARIABLE_SNAPSHOTS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_FRAME_VARIABLE_SNAPSHOTS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: FrameVariableSnapshotSet =
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
fn set_materializes_every_fidelity_disclosure_and_entry_kind() {
    let fixture = load_fixture();
    for fidelity in FrameMappingFidelity::ALL {
        assert!(
            fixture.frame_in_fidelity(fidelity).is_some(),
            "missing fidelity {}",
            fidelity.as_str()
        );
    }
    for disclosure in ValueDisclosure::ALL {
        assert!(
            fixture.snapshot_in_disclosure(disclosure).is_some(),
            "missing disclosure {}",
            disclosure.as_str()
        );
    }
    for kind in SnapshotEntryKind::ALL {
        assert!(
            fixture.snapshots.iter().any(|s| s.entry_kind == kind),
            "missing entry kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn precise_source_link_never_hides_a_caveat() {
    let fixture = load_fixture();
    for fr in &fixture.frames {
        if fr.pill.shows_exact_source_link {
            assert_eq!(fr.fidelity(), FrameMappingFidelity::Exact);
            assert!(fr.build_identity.match_state.proves_exact_build());
            assert!(!fr.pill.requires_disclosure);
        } else {
            assert!(fr.pill.requires_disclosure);
        }
    }
}

#[test]
fn current_frame_identity_is_preserved_per_thread() {
    let fixture = load_fixture();
    let mut threads: Vec<(String, String)> = Vec::new();
    for fr in &fixture.frames {
        let key = (fr.session_id.clone(), fr.thread_id.clone());
        if !threads.contains(&key) {
            threads.push(key);
        }
    }
    for (s, t) in threads {
        let current = fixture
            .frames
            .iter()
            .filter(|f| f.session_id == s && f.thread_id == t && f.is_current_frame)
            .count();
        assert_eq!(current, 1, "thread {s}/{t} must have one current frame");
    }
}

#[test]
fn lost_mapping_and_source_map_are_disclosed() {
    let fixture = load_fixture();
    for fr in &fixture.frames {
        if fr.mapping_provenance == FrameMappingProvenance::Unresolved {
            assert_eq!(fr.fidelity(), FrameMappingFidelity::Unmapped);
        }
        if fr.fidelity() == FrameMappingFidelity::Unmapped {
            assert_eq!(fr.mapping_provenance, FrameMappingProvenance::Unresolved);
        }
        if fr.mapping_provenance.is_source_map() {
            assert!(fr.mapping_provenance_requires_disclosure);
            assert!(fr.pill.label.contains("source-map"));
        }
        if fr.continuity != FrameContinuityClass::Contiguous {
            assert!(fr.is_async_boundary);
            assert!(fr.pill.label.contains("async boundary"));
        }
    }
}

#[test]
fn unavailable_and_redacted_values_withhold_their_bodies() {
    let fixture = load_fixture();
    for sn in &fixture.snapshots {
        match sn.disclosure_class() {
            ValueDisclosure::Live => assert!(sn.disclosure.implies_live_authority),
            ValueDisclosure::Unavailable => {
                assert!(sn.unavailable_reason.is_some());
                assert!(sn.value_repr_digest.is_none());
                assert!(!sn.disclosure.implies_live_authority);
            }
            ValueDisclosure::Redacted => {
                assert!(sn.redaction.is_redacted());
                assert!(sn.value_repr_digest.is_none());
                assert!(!sn.disclosure.implies_live_authority);
            }
            ValueDisclosure::Captured | ValueDisclosure::Stale => {
                assert!(!sn.disclosure.implies_live_authority);
            }
        }
    }
}

#[test]
fn notebook_and_replay_inspectors_reuse_the_vocabulary() {
    let fixture = load_fixture();
    assert!(fixture
        .snapshots
        .iter()
        .any(|s| s.capture_context.notebook_cell_ref.is_some()));
    assert!(fixture
        .snapshots
        .iter()
        .any(|s| s.capture_context.replay_capture_ref.is_some()));
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
    for fr in &fixture.frames {
        assert!(
            root.join(&fr.proof_packet_ref).exists(),
            "frame {} proof packet {} does not exist",
            fr.frame_id,
            fr.proof_packet_ref
        );
    }
    for sn in &fixture.snapshots {
        assert!(
            root.join(&sn.proof_packet_ref).exists(),
            "snapshot {} proof packet {} does not exist",
            sn.snapshot_id,
            sn.proof_packet_ref
        );
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_frame_variable_snapshots.md",
        "schemas/debug/m5_frame_variable_snapshots.schema.json",
        "artifacts/debug/m5_frame_variable_snapshots.md",
        "fixtures/debug/m5_frame_variable_snapshots/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
