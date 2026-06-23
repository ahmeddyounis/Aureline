//! Freeze gate for the M5 chronology/replay/parity set.
//!
//! The checked-in fixture `fixtures/debug/m5_chronology_replay_parity/canonical_set.json` is
//! the published set. This gate rebuilds the set in code and asserts it equals the fixture
//! after a serialize round-trip, so the chronology/replay/notebook-parity contract cannot
//! drift from the published artifact without failing CI. It also re-proves support-export
//! safety, that the full support-class, timeline, fidelity, disposition, and trigger
//! vocabularies are materialized, that an unsupported backend inherits no claims, that
//! replay is inspect-only and capture-bound, that timeline bookmarks stay bound to one
//! capture identity and survive export, that every restart/reconnect consequence itemizes
//! the five subjects, that a degraded cell-frame link is never drawn exact, that every cited
//! proof packet and producing module exists on disk, and every frozen invariant. This test
//! runs under `cargo test --workspace`, so stable promotion cannot harden a chronology,
//! replay, bookmark, or notebook-parity claim without current proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_chronology_replay_parity::{
    m5_chronology_replay_parity_set, CellLinkFidelity, ChronologyReplayParitySet,
    ConsequenceDisposition, ConsequenceSubject, ConsequenceTrigger, DebugSupportClass,
    M5_CHRONOLOGY_REPLAY_PARITY_RECORD_KIND, M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_chronology_replay_parity/canonical_set.json")
}

fn load_fixture() -> ChronologyReplayParitySet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_chronology_replay_parity_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code chronology/replay/parity set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-debug --example \
         dump_m5_chronology_replay_parity > \
         fixtures/debug/m5_chronology_replay_parity/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_CHRONOLOGY_REPLAY_PARITY_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_CHRONOLOGY_REPLAY_PARITY_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: ChronologyReplayParitySet =
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
fn set_materializes_every_support_class_fidelity_disposition_and_trigger() {
    let fixture = load_fixture();
    for class in DebugSupportClass::ALL {
        assert!(
            fixture.chronology_in_support_class(class).is_some(),
            "missing support class {}",
            class.as_str()
        );
    }
    for fidelity in CellLinkFidelity::ALL {
        assert!(
            fixture
                .cell_frame_links
                .iter()
                .any(|l| l.fidelity == fidelity),
            "missing cell-link fidelity {}",
            fidelity.as_str()
        );
    }
    for disposition in ConsequenceDisposition::ALL {
        assert!(
            fixture
                .restart_consequences
                .iter()
                .flat_map(|c| c.entries.iter())
                .any(|e| e.disposition == disposition),
            "missing disposition {}",
            disposition.as_str()
        );
    }
    for trigger in ConsequenceTrigger::ALL {
        assert!(
            fixture
                .restart_consequences
                .iter()
                .any(|c| c.trigger == trigger),
            "missing trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn unsupported_backends_inherit_no_claims() {
    let fixture = load_fixture();
    let inert: Vec<_> = fixture
        .chronology_capabilities
        .iter()
        .filter(|c| c.support_pill.is_inert)
        .collect();
    assert!(!inert.is_empty());
    for c in inert {
        assert!(c.supported_verbs.is_empty());
        assert!(!c.support_pill.time_travel_available);
        assert!(!c.recorded_scope.records_history());
    }
}

#[test]
fn replay_is_inspect_only_and_bookmarks_stay_capture_bound() {
    let fixture = load_fixture();
    for r in &fixture.replay_sessions {
        assert!(r.inspect_only);
        assert!(r.capture.is_fully_bound());
        assert!(fixture.chronology(&r.source_chronology_ref).is_some());
    }
    for b in &fixture.timeline_bookmarks {
        assert!(b.capture.is_fully_bound());
        assert!(b.survives_support_export);
        assert!(b.survives_restore_review);
        let rs = fixture
            .replay_session(&b.replay_session_ref)
            .expect("bookmark resolves to a replay session");
        assert!(rs.capture.same_as(&b.capture));
    }
}

#[test]
fn every_restart_consequence_itemizes_the_five_subjects() {
    let fixture = load_fixture();
    assert!(!fixture.restart_consequences.is_empty());
    for c in &fixture.restart_consequences {
        for subject in ConsequenceSubject::ALL {
            assert!(
                c.disposition_for(subject).is_some(),
                "consequence {} must explain subject {}",
                c.consequence_id,
                subject.as_str()
            );
        }
    }
    // notebook, debug, and replay consequences all exist.
    let has = |t: ConsequenceTrigger| fixture.restart_consequences.iter().any(|c| c.trigger == t);
    assert!(
        has(ConsequenceTrigger::KernelRestart) || has(ConsequenceTrigger::TransportLostReconnect)
    );
    assert!(has(ConsequenceTrigger::SessionRestart) || has(ConsequenceTrigger::Reconnect));
    assert!(has(ConsequenceTrigger::ReplayReacquire));
}

#[test]
fn a_degraded_cell_link_is_never_drawn_exact() {
    let fixture = load_fixture();
    for l in &fixture.cell_frame_links {
        assert_eq!(
            l.renders_exact_link,
            l.fidelity.is_exact() && l.support_class.permits_use()
        );
        if !l.fidelity.is_exact() {
            assert!(!l.renders_exact_link);
        }
    }
    assert!(fixture
        .cell_frame_links
        .iter()
        .any(|l| l.renders_exact_link));
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
    let proof_refs = fixture
        .chronology_capabilities
        .iter()
        .map(|c| {
            (
                "chronology",
                c.descriptor_id.as_str(),
                c.proof_packet_ref.as_str(),
            )
        })
        .chain(fixture.replay_sessions.iter().map(|r| {
            (
                "replay",
                r.replay_session_id.as_str(),
                r.proof_packet_ref.as_str(),
            )
        }))
        .chain(fixture.timeline_bookmarks.iter().map(|b| {
            (
                "bookmark",
                b.bookmark_id.as_str(),
                b.proof_packet_ref.as_str(),
            )
        }))
        .chain(
            fixture
                .notebook_kernels
                .iter()
                .map(|k| ("kernel", k.kernel_id.as_str(), k.proof_packet_ref.as_str())),
        )
        .chain(
            fixture
                .cell_frame_links
                .iter()
                .map(|l| ("link", l.link_id.as_str(), l.proof_packet_ref.as_str())),
        )
        .chain(fixture.restart_consequences.iter().map(|c| {
            (
                "consequence",
                c.consequence_id.as_str(),
                c.proof_packet_ref.as_str(),
            )
        }));
    for (kind, id, proof) in proof_refs {
        assert!(
            root.join(proof).exists(),
            "{kind} {id} proof packet {proof} does not exist"
        );
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_chronology_replay_parity.md",
        "schemas/debug/m5_chronology_replay_parity.schema.json",
        "artifacts/debug/m5_chronology_replay_parity.md",
        "fixtures/debug/m5_chronology_replay_parity/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
