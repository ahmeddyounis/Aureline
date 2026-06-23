//! Freeze gate for the M5 fanout-receipts bundle.
//!
//! The checked-in fixture `fixtures/activity/m5-fanout-receipts/canonical_bundle.json` is
//! the published bundle. This gate rebuilds the bundle in code and asserts it equals the
//! fixture after a serialize round-trip, so the minting rules and their receipts cannot
//! drift from the published artifact without failing CI. It also re-proves support-export
//! safety, that every dispatch is reproducible, that no failure is counted as delivered,
//! that managed endpoints never receive the payload, that the durable record survives any
//! fanout outcome, that an approval-gated alert never acts inline, and every frozen
//! invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_routing::FanoutChannelClass;
use aureline_activity::m5_fanout_receipts::{
    fanout_receipts_bundle, mint_dispatch, FanoutConditionClass, FanoutDeliveryStateClass,
    FanoutReceiptsBundle, FanoutSummaryPostureClass, StaleUndeliveredReasonClass,
    M5_FANOUT_RECEIPTS_RECORD_KIND, M5_FANOUT_RECEIPTS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-fanout-receipts/canonical_bundle.json")
}

fn load_fixture() -> FanoutReceiptsBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = fanout_receipts_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code fanout-receipts bundle drifted from the checked-in fixture; regenerate it \
         with `cargo run -p aureline-activity --example dump_m5_fanout_receipts`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_FANOUT_RECEIPTS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_FANOUT_RECEIPTS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: FanoutReceiptsBundle =
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
fn every_dispatch_is_reproducible() {
    let fixture = load_fixture();
    assert!(!fixture.dispatches.is_empty());
    for dispatch in &fixture.dispatches {
        let source = fixture
            .source(&dispatch.source_envelope_id)
            .expect("dispatch source present");
        assert_eq!(
            &mint_dispatch(source, dispatch.condition),
            dispatch,
            "dispatch {} must reproduce from its source and condition",
            dispatch.dispatch_id
        );
    }
}

#[test]
fn no_failure_is_counted_as_delivered() {
    let fixture = load_fixture();
    for dispatch in &fixture.dispatches {
        let delivered = dispatch
            .receipts
            .iter()
            .filter(|r| r.delivery_state == FanoutDeliveryStateClass::Delivered)
            .count();
        assert_eq!(dispatch.delivered_count, delivered);
        assert!(dispatch.all_failures_labeled);
        // The durable record survives any fanout outcome.
        assert!(dispatch.durable_record_present);
    }
}

#[test]
fn managed_endpoint_never_receives_the_payload() {
    let fixture = load_fixture();
    for dispatch in &fixture.dispatches {
        if dispatch.condition != FanoutConditionClass::ManagedEndpointBlocked {
            continue;
        }
        for receipt in &dispatch.receipts {
            assert_eq!(
                receipt.delivery_state,
                FanoutDeliveryStateClass::Undelivered
            );
            assert_eq!(
                receipt.stale_or_undelivered_reason,
                StaleUndeliveredReasonClass::ManagedEndpointBlocked
            );
            assert_eq!(
                receipt.summary_posture,
                FanoutSummaryPostureClass::NoSummary
            );
        }
    }
}

#[test]
fn approval_gated_alerts_never_act_inline_and_reopen_exactly() {
    let fixture = load_fixture();
    for dispatch in &fixture.dispatches {
        let source = fixture
            .source(&dispatch.source_envelope_id)
            .expect("source present");
        for receipt in &dispatch.receipts {
            // No external copy lands on a generic shell; it reopens the source's exact object.
            assert_ne!(receipt.destination, FanoutChannelClass::InAppActivityCenter);
            assert_eq!(receipt.reopen_anchor_ref, source.reopen_anchor_ref);
            assert!(receipt.reopen_is_exact);
            // A preview/approval-gated alert may not act inline.
            if receipt.routes_through_preview_approval {
                assert!(!receipt.inline_action_allowed);
            }
        }
    }
}
