//! Freeze gate for the M5 attention-routing matrix.
//!
//! The checked-in fixture
//! `fixtures/activity/m5-attention-routing/canonical_matrix.json` is the
//! published matrix. This gate rebuilds the matrix in code and asserts it equals
//! the fixture after a serialize round-trip, so the attention-routing contract
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full object/channel/state coverage, that every named
//! controlled vocabulary is bound, that every object maps to a proof packet, and
//! every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_routing::{
    attention_routing_matrix, AttentionObjectClass, AttentionRoutingMatrix, AttentionStateClass,
    ControlledVocabulary, FanoutChannelClass, M5_ATTENTION_ROUTING_RECORD_KIND,
    M5_ATTENTION_ROUTING_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-attention-routing/canonical_matrix.json")
}

fn load_fixture() -> AttentionRoutingMatrix {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_matrix_matches_checked_in_fixture() {
    let built = attention_routing_matrix();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code attention-routing matrix drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-activity --example dump_m5_attention_routing`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ATTENTION_ROUTING_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ATTENTION_ROUTING_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: AttentionRoutingMatrix =
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
fn matrix_covers_every_object_channel_and_state() {
    let fixture = load_fixture();
    assert_eq!(fixture.objects.len(), AttentionObjectClass::ALL.len());
    for object in AttentionObjectClass::ALL {
        assert!(
            fixture.object(object).is_some(),
            "missing object {}",
            object.as_str()
        );
    }
    assert_eq!(fixture.channels.len(), FanoutChannelClass::ALL.len());
    for channel in FanoutChannelClass::ALL {
        assert!(
            fixture.channel(channel).is_some(),
            "missing channel {}",
            channel.as_str()
        );
    }
    assert_eq!(
        fixture.state_vocabulary.len(),
        AttentionStateClass::ALL.len()
    );
    for state in AttentionStateClass::ALL {
        assert!(
            fixture.state_term(state).is_some(),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn matrix_covers_all_seven_attention_objects_explicitly() {
    let fixture = load_fixture();
    for object in [
        AttentionObjectClass::NotificationEnvelope,
        AttentionObjectClass::ActivityObject,
        AttentionObjectClass::BadgeAggregate,
        AttentionObjectClass::FanoutReceipt,
        AttentionObjectClass::RoutingContext,
        AttentionObjectClass::PrivacyClass,
        AttentionObjectClass::ActionRetentionSemantics,
    ] {
        let entry = fixture.object(object).expect("object present");
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.required_fields.is_empty());
    }
}

#[test]
fn every_named_controlled_vocabulary_is_bound() {
    let fixture = load_fixture();
    for vocab in ControlledVocabulary::ALL {
        assert!(
            fixture.objects.iter().any(|o| o.binds(vocab)),
            "controlled vocabulary {} is bound by no object",
            vocab.as_str()
        );
    }
}

#[test]
fn every_object_maps_to_a_proof_packet() {
    let fixture = load_fixture();
    for object in &fixture.objects {
        assert!(
            !object.proof_packet_ref.is_empty(),
            "object {} lacks a mapped proof packet",
            object.object.as_str()
        );
    }
}

#[test]
fn no_fanout_channel_bypasses_preview_approval() {
    let fixture = load_fixture();
    for channel in &fixture.channels {
        assert!(
            !channel.can_bypass_preview_approval,
            "channel {} must not bypass preview/approval",
            channel.channel.as_str()
        );
    }
}
