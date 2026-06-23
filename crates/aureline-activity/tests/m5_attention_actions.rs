//! Freeze gate for the M5 attention-actions bundle.
//!
//! The checked-in fixture
//! `fixtures/activity/m5-attention-actions/canonical_bundle.json` is the published
//! bundle. This gate rebuilds the bundle in code and asserts it equals the fixture
//! after a serialize round-trip, so the action semantics and their applied outcomes
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, that every action is defined, that every outcome is
//! reproducible, that exact reopen continuity survives every action, and every frozen
//! invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_actions::{
    apply_attention_action, attention_actions_bundle, AttentionActionClass, AttentionActionsBundle,
    SurfaceActionPropagationClass, M5_ATTENTION_ACTIONS_RECORD_KIND,
    M5_ATTENTION_ACTIONS_SCHEMA_REF,
};
use aureline_activity::m5_attention_routing::FanoutChannelClass;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-attention-actions/canonical_bundle.json")
}

fn load_fixture() -> AttentionActionsBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = attention_actions_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code attention-actions bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-activity --example dump_m5_attention_actions`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ATTENTION_ACTIONS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ATTENTION_ACTIONS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: AttentionActionsBundle =
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
fn every_action_is_defined() {
    let fixture = load_fixture();
    for action in AttentionActionClass::ALL {
        assert!(
            fixture.definition(action).is_some(),
            "missing action definition {}",
            action.as_str()
        );
    }
}

#[test]
fn every_outcome_is_reproducible_and_reopen_safe() {
    let fixture = load_fixture();
    assert!(!fixture.outcomes.is_empty());
    for outcome in &fixture.outcomes {
        let item = fixture
            .item(&outcome.item_id)
            .expect("outcome item present");
        assert_eq!(
            &apply_attention_action(item, outcome.action),
            outcome,
            "outcome {} must reproduce from its item and action",
            outcome.outcome_id
        );
        // Exact reopen continuity: same authoritative target, anchor, and action target.
        assert!(outcome.reopen_continuity_preserved);
        assert_eq!(outcome.reopen_target, item.reopen_target);
        assert_eq!(outcome.reopen_anchor_ref, item.reopen_anchor_ref);
        assert_eq!(outcome.action_target_id, item.action_target_id);
        assert!(!outcome.replays_side_effects);
        // The in-app activity center is the authoritative surface for every action.
        assert_eq!(
            outcome
                .propagation(FanoutChannelClass::InAppActivityCenter)
                .expect("in-app propagation present")
                .propagation,
            SurfaceActionPropagationClass::ApplyAuthoritative
        );
    }
}

#[test]
fn a_security_advisory_can_only_be_acknowledged_or_resolved() {
    let fixture = load_fixture();
    let security = fixture
        .item("attention_item:security.credential_revoked:0001")
        .expect("security item present");
    for action in &security.supported_actions {
        assert!(
            matches!(
                action,
                AttentionActionClass::Acknowledge | AttentionActionClass::Resolve
            ),
            "a security advisory must not support {}",
            action.as_str()
        );
    }
    for outcome in fixture
        .outcomes
        .iter()
        .filter(|o| o.item_id == security.item_id)
    {
        assert!(!outcome.action.is_silencing());
    }
}
