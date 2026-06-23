//! Freeze gate for the M5 quiet-hours-suppression bundle.
//!
//! The checked-in fixture
//! `fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json` is the
//! published bundle. This gate rebuilds the bundle in code and asserts it equals the
//! fixture after a serialize round-trip, so the suppression rules and their decisions
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, that every decision is reproducible, that the in-app activity
//! center always shows the durable record, that a security advisory is never silenced,
//! and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_routing::FanoutChannelClass;
use aureline_activity::m5_quiet_hours_suppression::{
    evaluate_suppression, quiet_hours_suppression_bundle, QuietHoursSuppressionBundle,
    SuppressionDispositionClass, M5_QUIET_HOURS_SUPPRESSION_RECORD_KIND,
    M5_QUIET_HOURS_SUPPRESSION_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json")
}

fn load_fixture() -> QuietHoursSuppressionBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = quiet_hours_suppression_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code quiet-hours-suppression bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-activity --example dump_m5_quiet_hours_suppression`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_QUIET_HOURS_SUPPRESSION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_QUIET_HOURS_SUPPRESSION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: QuietHoursSuppressionBundle =
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
fn every_decision_is_reproducible() {
    let fixture = load_fixture();
    assert!(!fixture.decisions.is_empty());
    for decision in &fixture.decisions {
        let signal = fixture
            .signal(&decision.signal_id)
            .expect("decision signal present");
        let policy = fixture
            .policy(&decision.policy_id)
            .expect("decision policy present");
        assert_eq!(
            &evaluate_suppression(signal, policy),
            decision,
            "decision {} must reproduce from its signal and policy",
            decision.decision_id
        );
    }
}

#[test]
fn in_app_always_shows_and_security_is_never_silenced() {
    let fixture = load_fixture();
    for decision in &fixture.decisions {
        // The in-app activity center always holds the durable record.
        assert_eq!(
            decision
                .outcome(FanoutChannelClass::InAppActivityCenter)
                .expect("in-app outcome present")
                .disposition,
            SuppressionDispositionClass::Shown
        );
        assert!(decision.durable_record_present);
        // No security advisory is ever silenced on every surface.
        assert!(!decision.security_silenced);
    }
}

#[test]
fn suppression_ledger_is_separate_from_audit_history() {
    let fixture = load_fixture();
    for decision in &fixture.decisions {
        for entry in &decision.ledger_entries {
            assert!(
                entry.separate_from_audit_history,
                "ledger entry {} must be separate from audit history",
                entry.ledger_entry_id
            );
            assert!(
                !entry.implies_underlying_disappeared,
                "ledger entry {} must not imply the underlying object disappeared",
                entry.ledger_entry_id
            );
            assert_ne!(entry.surface, FanoutChannelClass::InAppActivityCenter);
        }
    }
}
