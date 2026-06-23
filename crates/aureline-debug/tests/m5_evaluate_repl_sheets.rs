//! Freeze gate for the M5 evaluate/REPL sheet set.
//!
//! The checked-in fixture `fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json` is
//! the published set. This gate rebuilds the set in code and asserts it equals the fixture
//! after a serialize round-trip, so the evaluate/console contract cannot drift from the
//! published artifact without failing CI. It also re-proves support-export safety, that
//! the full purity, disposition, and direction vocabularies are materialized, that an
//! unknown or mutating expression discloses its risk and never runs unless approved, that
//! a pending/denied/blocked/expired evaluation carries no result, that an effectful
//! expression against an inspect-only context is blocked, that actor lineage and reviewer
//! attribution are preserved, that console input and output stay separate, that a replayed
//! line is never shown as live, that redaction review is preserved, that every cited proof
//! packet and producing module exists on disk, and every frozen invariant. This test runs
//! under `cargo test --workspace`, so stable promotion cannot harden an evaluate or console
//! claim without current proof.

use std::path::{Path, PathBuf};

use aureline_debug::m5_evaluate_repl_sheets::{
    m5_evaluate_repl_sheet_set, ApprovalDisposition, ConsoleDirection, ConsoleLiveness,
    EvaluatePurityClass, EvaluateReplSheetSet, M5_EVALUATE_REPL_SHEETS_RECORD_KIND,
    M5_EVALUATE_REPL_SHEETS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join("fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json")
}

fn load_fixture() -> EvaluateReplSheetSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = m5_evaluate_repl_sheet_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code evaluate/REPL sheet set drifted from the checked-in fixture; regenerate it \
         with `cargo run -p aureline-debug --example dump_m5_evaluate_repl_sheets > \
         fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_EVALUATE_REPL_SHEETS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_EVALUATE_REPL_SHEETS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: EvaluateReplSheetSet =
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
fn set_materializes_every_purity_disposition_and_direction() {
    let fixture = load_fixture();
    for purity in EvaluatePurityClass::ALL {
        assert!(
            fixture.evaluation_in_purity(purity).is_some(),
            "missing purity {}",
            purity.as_str()
        );
    }
    for disposition in ApprovalDisposition::ALL {
        assert!(
            fixture.evaluation_in_disposition(disposition).is_some(),
            "missing disposition {}",
            disposition.as_str()
        );
    }
    for direction in ConsoleDirection::ALL {
        assert!(
            fixture.emission_in_direction(direction).is_some(),
            "missing direction {}",
            direction.as_str()
        );
    }
}

#[test]
fn unknown_or_mutating_never_runs_unless_approved() {
    let fixture = load_fixture();
    for ev in &fixture.evaluations {
        if ev.posture.approval_required {
            assert!(ev.posture.discloses_side_effect_risk);
            if ev.disposition != ApprovalDisposition::Approved {
                assert!(!ev.posture.permits_dispatch);
                assert!(ev.result.is_none());
            }
        } else {
            // A pure expression never claims a side-effect risk.
            assert!(!ev.posture.discloses_side_effect_risk);
            assert_eq!(ev.disposition, ApprovalDisposition::NotRequired);
        }
    }
}

#[test]
fn blocked_denied_expired_states_are_preserved_and_inspect_only_is_blocked() {
    let fixture = load_fixture();
    for disposition in [
        ApprovalDisposition::Blocked,
        ApprovalDisposition::Denied,
        ApprovalDisposition::Expired,
    ] {
        let ev = fixture
            .evaluation_in_disposition(disposition)
            .unwrap_or_else(|| panic!("missing disposition {}", disposition.as_str()));
        assert!(!ev.posture.permits_dispatch);
        assert!(ev.result.is_none());
    }
    // An effectful expression against an inspect-only context is blocked.
    let inspect_only: Vec<_> = fixture
        .evaluations
        .iter()
        .filter(|e| !e.context.authority.allows_mutation() && e.purity.requires_approval())
        .collect();
    assert!(!inspect_only.is_empty());
    for ev in inspect_only {
        assert!(!ev.posture.permits_dispatch);
        assert!(ev.posture.blocked_by_inspect_only);
    }
}

#[test]
fn actor_lineage_and_reviewer_attribution_are_preserved() {
    let fixture = load_fixture();
    for ev in &fixture.evaluations {
        assert!(!ev.actor.requested_by_ref.is_empty());
        if ev.purity.requires_approval() && ev.disposition == ApprovalDisposition::Approved {
            assert!(ev.actor.reviewed_by_ref.is_some());
        }
    }
}

#[test]
fn console_separates_input_and_output_and_never_shows_replay_as_live() {
    let fixture = load_fixture();
    assert!(fixture
        .console
        .iter()
        .any(|c| c.direction() == ConsoleDirection::UserInput));
    assert!(fixture
        .console
        .iter()
        .any(|c| c.direction() == ConsoleDirection::TargetOutput));
    for em in &fixture.console {
        assert_eq!(em.direction, em.stream.direction());
        if em.liveness == ConsoleLiveness::ReplayedCapture {
            assert!(em.pill.is_replayed);
            assert!(!em.pill.is_live);
            assert!(em.pill.requires_disclosure);
        }
        if em.redaction.is_redacted() {
            assert!(em.pill.is_redacted);
            assert!(em.body_digest.is_none());
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
    for ev in &fixture.evaluations {
        assert!(
            root.join(&ev.proof_packet_ref).exists(),
            "evaluation {} proof packet {} does not exist",
            ev.evaluate_id,
            ev.proof_packet_ref
        );
    }
    for em in &fixture.console {
        assert!(
            root.join(&em.proof_packet_ref).exists(),
            "emission {} proof packet {} does not exist",
            em.emission_id,
            em.proof_packet_ref
        );
    }
}

#[test]
fn checked_in_docs_schema_and_artifact_exist() {
    let root = repo_root();
    for rel in [
        "docs/debug/m5_evaluate_repl_sheets.md",
        "schemas/debug/m5_evaluate_repl_sheets.schema.json",
        "artifacts/debug/m5_evaluate_repl_sheets.md",
        "fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
