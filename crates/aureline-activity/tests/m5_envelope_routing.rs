//! Freeze gate for the M5 envelope-routing bundle.
//!
//! The checked-in fixture
//! `fixtures/activity/m5-envelope-routing/canonical_bundle.json` is the published
//! bundle. This gate rebuilds the bundle in code and asserts it equals the fixture
//! after a serialize round-trip, so the typed envelope path and its routing
//! decisions cannot drift from the published artifact without failing CI. It also
//! re-proves support-export safety, that every M5 subsystem has a producer routing
//! the typed path, that every routing decision is reproducible, and every frozen
//! invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_routing::FanoutChannelClass;
use aureline_activity::m5_envelope_routing::{
    envelope_routing_bundle, route_envelope, EnvelopeRoutingBundle, RouteDispositionClass,
    SourceSubsystemClass, M5_ENVELOPE_ROUTING_RECORD_KIND, M5_ENVELOPE_ROUTING_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-envelope-routing/canonical_bundle.json")
}

fn load_fixture() -> EnvelopeRoutingBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = envelope_routing_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code envelope-routing bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-activity --example dump_m5_envelope_routing`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ENVELOPE_ROUTING_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ENVELOPE_ROUTING_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: EnvelopeRoutingBundle =
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
fn every_m5_subsystem_routes_the_typed_path() {
    let fixture = load_fixture();
    for subsystem in SourceSubsystemClass::ALL {
        let producer = fixture
            .producers
            .iter()
            .find(|p| p.source_subsystem == subsystem)
            .unwrap_or_else(|| panic!("subsystem {} has a producer", subsystem.as_str()));
        assert!(producer.routes_through_typed_envelope);
        assert!(!producer.retains_surface_local_logic);
        assert!(
            fixture.envelope(&producer.emits_envelope_id).is_some(),
            "producer {} emits a known envelope",
            producer.producer_id
        );
    }
}

#[test]
fn every_decision_is_reproducible_from_its_inputs() {
    let fixture = load_fixture();
    assert!(!fixture.decisions.is_empty());
    for decision in &fixture.decisions {
        let envelope = fixture
            .envelope(&decision.envelope_id)
            .expect("decision envelope present");
        let context = fixture
            .context(&decision.context_id)
            .expect("decision context present");
        assert_eq!(
            &route_envelope(envelope, context),
            decision,
            "decision {} must reproduce from its envelope and context",
            decision.decision_id
        );
    }
}

#[test]
fn every_decision_keeps_a_durable_record_and_one_action_target() {
    let fixture = load_fixture();
    for decision in &fixture.decisions {
        assert!(
            decision.durable_record_present,
            "decision {} must keep a durable record",
            decision.decision_id
        );
        let in_app = decision
            .outcome(FanoutChannelClass::InAppActivityCenter)
            .expect("in-app outcome present");
        assert_eq!(in_app.disposition, RouteDispositionClass::Deliver);
        for outcome in &decision.outcomes {
            assert_eq!(outcome.action_target_id, decision.action_target_id);
        }
    }
}
