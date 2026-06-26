//! Freeze gate for the M5 dump/mapping/restore set.
//!
//! The checked-in fixture `fixtures/debug/m5_dump_mapping_restore/canonical_set.json` is the
//! published set. This gate rebuilds the set in code and asserts it equals the fixture after
//! a serialize round-trip, so the dump/mapping/restore contract cannot drift from the
//! published artifact without failing CI. It also re-proves support-export safety, that the
//! full six-state mapping vocabulary, every artifact kind, every entrypoint, every source
//! class, and every restore posture are materialized, that a precise source link never hides
//! a degraded mapping, that imported and build-mismatched strips never render the exact link,
//! that the four session entrypoints stay distinct and inspect-only, that restored layouts
//! never imply live continuity or reacquired process authority, that every cited proof packet
//! and producing module exists on disk, and every frozen invariant. This test runs under
//! `cargo test --workspace`, so stable promotion cannot harden a dump or restore claim
//! without current proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_dump_mapping_restore::{
    m5_dump_mapping_restore_set, ArtifactBuildMatch, ArtifactSourceClass, DebugArtifactEntrypoint,
    DebugArtifactKind, DebugMappingFidelity, DumpMappingRestoreSet, RestorePosture,
    M5_DUMP_MAPPING_RESTORE_RECORD_KIND, M5_DUMP_MAPPING_RESTORE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_dump_mapping_restore/canonical_set.json")
}

fn load_fixture() -> DumpMappingRestoreSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_dump_mapping_restore_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code dump/mapping/restore set drifted from the checked-in fixture; regenerate it \
         with `cargo run -p aureline-debug --example dump_m5_dump_mapping_restore > \
         fixtures/debug/m5_dump_mapping_restore/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_DUMP_MAPPING_RESTORE_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_DUMP_MAPPING_RESTORE_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: DumpMappingRestoreSet =
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
fn set_materializes_every_vocabulary() {
    let fixture = load_fixture();
    for fidelity in DebugMappingFidelity::ALL {
        assert!(
            fixture.artifact_in_fidelity(fidelity).is_some(),
            "missing fidelity {}",
            fidelity.as_str()
        );
    }
    for kind in DebugArtifactKind::ALL {
        assert!(
            fixture.artifacts.iter().any(|a| a.artifact_kind == kind),
            "missing kind {}",
            kind.as_str()
        );
    }
    for entrypoint in DebugArtifactEntrypoint::ALL {
        assert!(
            fixture.artifacts.iter().any(|a| a.entrypoint == entrypoint),
            "missing entrypoint {}",
            entrypoint.as_str()
        );
    }
    for source in ArtifactSourceClass::ALL {
        assert!(
            fixture.artifacts.iter().any(|a| a.source_class() == source),
            "missing source class {}",
            source.as_str()
        );
    }
    for posture in RestorePosture::ALL {
        assert!(
            fixture.restored_layout_in_posture(posture).is_some(),
            "missing restore posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn precise_source_link_never_hides_a_degraded_mapping() {
    let fixture = load_fixture();
    for a in &fixture.artifacts {
        if a.pill.shows_exact_source_link {
            assert_eq!(a.fidelity(), DebugMappingFidelity::Exact);
            assert!(a.build_match().proves_exact_build());
            assert!(!a.pill.requires_disclosure);
        } else {
            assert!(a.pill.requires_disclosure);
        }
        if a.fidelity().is_imported() {
            assert!(a.source_class().is_imported());
            assert!(!a.pill.shows_exact_source_link);
        }
        if a.fidelity().is_build_mismatch() {
            assert_eq!(a.build_match(), ArtifactBuildMatch::MismatchedRejected);
            assert!(!a.pill.shows_exact_source_link);
        }
    }
}

#[test]
fn session_entrypoints_stay_distinct_and_inspect_only() {
    let fixture = load_fixture();
    for entrypoint in DebugArtifactEntrypoint::SESSION_ENTRYPOINTS {
        let matching: Vec<_> = fixture
            .artifacts
            .iter()
            .filter(|a| a.entrypoint == entrypoint)
            .collect();
        assert!(
            !matching.is_empty(),
            "missing session entrypoint {}",
            entrypoint.as_str()
        );
        assert!(matching.iter().all(|a| a.opens_inspect_only_session));
    }
    for a in fixture
        .artifacts
        .iter()
        .filter(|a| a.entrypoint == DebugArtifactEntrypoint::ImportSymbolsOrSourceMap)
    {
        assert!(!a.opens_inspect_only_session);
        assert!(a.entrypoint.accepts_kind(a.artifact_kind));
    }
}

#[test]
fn restored_layouts_never_imply_live_authority() {
    let fixture = load_fixture();
    for r in &fixture.restored_layouts {
        assert!(!r.pill.implies_live_continuity);
        assert!(!r.pill.implies_process_authority);
        assert!(r.pill.requires_disclosure);
        let expected_exact = r.fidelity().preserves_exact_source() && r.exact_build_still_verified;
        assert_eq!(r.pill.implies_exact_build_mapping, expected_exact);
        assert!(
            fixture.artifact(&r.restored_strip_ref).is_some(),
            "{} references unknown strip {}",
            r.layout_id,
            r.restored_strip_ref
        );
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
    for a in &fixture.artifacts {
        assert!(
            root.join(&a.proof_packet_ref).exists(),
            "strip {} proof packet {} does not exist",
            a.strip_id,
            a.proof_packet_ref
        );
    }
    for r in &fixture.restored_layouts {
        assert!(
            root.join(&r.proof_packet_ref).exists(),
            "restore {} proof packet {} does not exist",
            r.layout_id,
            r.proof_packet_ref
        );
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_dump_mapping_restore.md",
        "schemas/debug/m5_dump_mapping_restore.schema.json",
        "artifacts/debug/m5_dump_mapping_restore.md",
        "fixtures/debug/m5_dump_mapping_restore/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
