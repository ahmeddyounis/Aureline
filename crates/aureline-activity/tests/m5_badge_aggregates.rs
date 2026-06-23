//! Freeze gate for the M5 badge-aggregates bundle.
//!
//! The checked-in fixture `fixtures/activity/m5-badge-aggregates/canonical_bundle.json` is
//! the published bundle. This gate rebuilds the bundle in code and asserts it equals the
//! fixture after a serialize round-trip, so the aggregation, coalescing, surface-projection,
//! and telemetry rules cannot drift from the published artifact without failing CI. It also
//! re-proves support-export safety, that the aggregates reproduce from the corpus, that badge
//! counts derive from deduped durable items, that repeated failures coalesce into one object
//! counted once, that every governed surface shows the same count, that a security advisory
//! is never silenced, that telemetry captures no message text, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_activity::m5_attention_routing::FanoutChannelClass;
use aureline_activity::m5_badge_aggregates::{
    aggregate_badges, badge_aggregates_bundle, coalesce_failures, BadgeAggregatesBundle,
    BadgeContributionClass, GOVERNED_BADGE_SURFACES, M5_BADGE_AGGREGATES_RECORD_KIND,
    M5_BADGE_AGGREGATES_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/activity/m5-badge-aggregates/canonical_bundle.json")
}

fn load_fixture() -> BadgeAggregatesBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = badge_aggregates_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code badge-aggregates bundle drifted from the checked-in fixture; regenerate it \
         with `cargo run -p aureline-activity --example dump_m5_badge_aggregates`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_BADGE_AGGREGATES_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_BADGE_AGGREGATES_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(!fixture.telemetry.captures_message_text);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: BadgeAggregatesBundle =
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
fn aggregates_reproduce_and_counts_are_deduped() {
    let fixture = load_fixture();
    assert_eq!(aggregate_badges(&fixture.items), fixture.aggregates);
    // At least one scope shows dedupe collapsing repeats.
    assert!(fixture
        .aggregates
        .iter()
        .any(|a| a.raw_event_count > a.count));
    for aggregate in &fixture.aggregates {
        assert_eq!(aggregate.count, aggregate.deduped_objects.len());
        assert_eq!(aggregate.count, aggregate.deduped_count);
        assert!(aggregate.raw_event_count >= aggregate.count);
        assert!(aggregate.derives_from_durable_items);
    }
}

#[test]
fn every_surface_matches_the_deduped_aggregate() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.surface_badges.len(),
        GOVERNED_BADGE_SURFACES.len() * fixture.aggregates.len()
    );
    for aggregate in &fixture.aggregates {
        for surface in GOVERNED_BADGE_SURFACES {
            let badge = fixture
                .surface_badge(surface, aggregate.scope)
                .expect("surface badge present");
            assert_eq!(
                badge.count, aggregate.count,
                "surface {surface:?} count parity for scope {:?}",
                aggregate.scope
            );
            assert_eq!(badge.count_class, aggregate.count_class);
            assert_eq!(badge.reopen_anchor_ref, aggregate.reopen_anchor_ref);
        }
    }
}

#[test]
fn repeated_failures_coalesce_into_one_durable_object() {
    let fixture = load_fixture();
    assert_eq!(
        coalesce_failures(&fixture.items),
        fixture.coalesced_failures
    );
    assert!(fixture
        .coalesced_failures
        .iter()
        .any(|f| f.spam_prevented && f.occurrence_count > 1));
    for failure in &fixture.coalesced_failures {
        // The collapsed object reopens the representative's exact authoritative object.
        let rep = fixture
            .items
            .iter()
            .find(|i| i.item_id == failure.representative_item_id)
            .expect("representative present");
        assert_eq!(failure.reopen_anchor_ref, rep.reopen_anchor_ref);
        assert_eq!(failure.reopen_target, rep.reopen_target);
        assert!(failure.durable_record_present);
        // The badge counts the coalesced object exactly once.
        let aggregate = fixture
            .aggregate(failure.scope)
            .expect("scope aggregate present");
        let counted = aggregate
            .deduped_objects
            .iter()
            .filter(|o| o.object_key == failure.root_cause_key)
            .count();
        assert_eq!(counted, 1);
    }
}

#[test]
fn security_advisories_are_never_silenced() {
    let fixture = load_fixture();
    let mut saw_silencing_signal = false;
    for item in &fixture.items {
        if item.severity.is_security()
            && item.contribution() == BadgeContributionClass::Counted
            && (item.mute_reason.is_named() || item.quiet_hours_mode.is_deferring())
        {
            saw_silencing_signal = true;
        }
    }
    // The corpus proves the rule with a real advisory carrying a mute/quiet-hours signal.
    assert!(
        saw_silencing_signal,
        "a security advisory with a silencing signal must still be counted"
    );
}

#[test]
fn telemetry_records_stable_enums_without_message_text() {
    let fixture = load_fixture();
    let telemetry = &fixture.telemetry;
    assert!(!telemetry.captures_message_text);
    let outcome_sum: usize = telemetry.outcome_rollup.iter().map(|r| r.count).sum();
    assert_eq!(outcome_sum, telemetry.total_items);
    // Every route shows the same total badge count — parity at the telemetry level.
    for row in &telemetry.route_rollup {
        assert_eq!(row.badge_count_total, telemetry.total_counted);
    }
    // No badge surface ever drops below the in-app authoritative record's parity.
    assert!(telemetry
        .route_rollup
        .iter()
        .any(|r| r.route == FanoutChannelClass::InAppActivityCenter));
}
