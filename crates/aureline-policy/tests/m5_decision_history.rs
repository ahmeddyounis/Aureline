//! Freeze gate for the M5 decision-history bundle.
//!
//! The checked-in fixture
//! `fixtures/admin/m5-decision-history/canonical_history.json` is the published
//! decision-history bundle. This gate rebuilds the bundle in code and asserts it
//! equals the fixture after a serialize round-trip, so the rendered timelines
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full profile coverage, that every rendered state is one
//! the frozen matrix admits, that every event resolves to its explorer filter,
//! that the actor classes stay distinguished, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass, AdminSurfaceClass};
use aureline_policy::m5_decision_history::{
    decision_history_bundle, decision_history_lines, ActorClass, DecisionHistoryBundle,
    EventFamilyClass, ExportFormatClass, HISTORY_PROFILES, M5_DECISION_HISTORY_RECORD_KIND,
    M5_DECISION_HISTORY_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-decision-history/canonical_history.json")
}

fn load_fixture() -> DecisionHistoryBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = decision_history_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code decision-history bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-policy --example dump_m5_decision_history`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_DECISION_HISTORY_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_DECISION_HISTORY_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: DecisionHistoryBundle =
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
fn bundle_renders_every_managed_profile() {
    let fixture = load_fixture();
    assert_eq!(fixture.profiles.len(), HISTORY_PROFILES.len());
    for profile in HISTORY_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.timeline.events.is_empty());
        assert!(packet.timeline.coverage.locally_inspectable);
        assert!(packet.timeline.coverage.vendor_console_independent);
    }
}

#[test]
fn rendered_states_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::DecisionHistoryTimeline)
            .expect("surface present in matrix")
            .applicable_states
            .contains(&state)
    };
    for packet in &fixture.profiles {
        for event in &packet.timeline.events {
            assert!(
                admitted(event.outcome_state),
                "decision-history state {} not admitted by the matrix",
                event.outcome_state.as_str()
            );
        }
        assert!(admitted(packet.timeline.coverage.coverage_state));
    }
}

#[test]
fn every_event_resolves_to_its_filter() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        for family in EventFamilyClass::ALL {
            assert!(packet.timeline.filter(family).is_some());
        }
        for event in &packet.timeline.events {
            let filter = packet
                .timeline
                .filter(event.event_family)
                .expect("family filter present");
            assert!(filter.matched_event_ids.contains(&event.event_id));
        }
    }
}

#[test]
fn actor_classes_stay_distinguished() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        assert!(packet.timeline.actor_classes().len() >= 2);
    }
    for actor in ActorClass::ALL {
        assert!(
            fixture.profiles.iter().any(|p| p
                .timeline
                .events
                .iter()
                .any(|e| e.actor_class == actor)),
            "actor class {} never appears",
            actor.as_str()
        );
    }
}

#[test]
fn every_row_exports_both_machine_and_plain_language() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        assert!(packet
            .timeline
            .offers(ExportFormatClass::MachineReadableJson));
        assert!(packet
            .timeline
            .offers(ExportFormatClass::PlainLanguageHandoff));
        for event in &packet.timeline.events {
            assert!(event.has_export_parity());
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = decision_history_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Decision-history bundle")));
    for profile in HISTORY_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
