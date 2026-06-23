//! Freeze gate for the M5 procurement bundle.
//!
//! The checked-in fixture
//! `fixtures/admin/m5-procurement/canonical_procurement.json` is the published
//! procurement bundle. This gate rebuilds the bundle in code and asserts it equals
//! the fixture after a serialize round-trip, so the rendered packets cannot drift
//! from the published artifact without failing CI. It also re-proves support-export
//! safety, full profile coverage, that every rendered state is one the frozen
//! matrix admits, that no commercial card outranks the export/delete/support/
//! local-continuation actions, that verification posture is never silently green,
//! that the admin handoff is complete and auto-derived, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass, AdminSurfaceClass};
use aureline_policy::m5_procurement::{
    procurement_bundle, procurement_lines, CanonicalObjectClass, CommercialEventClass,
    NextActionClass, ProcurementBundle, M5_PROCUREMENT_RECORD_KIND, M5_PROCUREMENT_SCHEMA_REF,
    PROCUREMENT_PROFILES,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-procurement/canonical_procurement.json")
}

fn load_fixture() -> ProcurementBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = procurement_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code procurement bundle drifted from the checked-in fixture; regenerate it with \
         `cargo run -p aureline-policy --example dump_m5_procurement`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_PROCUREMENT_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_PROCUREMENT_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: ProcurementBundle =
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
    assert_eq!(fixture.profiles.len(), PROCUREMENT_PROFILES.len());
    for profile in PROCUREMENT_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.event_cards.is_empty());
        assert!(packet.coverage.locally_inspectable);
        assert!(packet.coverage.vendor_console_independent);
        assert!(packet.coverage.exportable_without_paid_seat);
    }
}

#[test]
fn rendered_states_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::ProcurementVerificationPacket)
            .expect("surface present in matrix")
            .applicable_states
            .contains(&state)
    };
    for packet in &fixture.profiles {
        assert!(admitted(packet.verification_packet.machine_state));
        assert!(admitted(packet.admin_handoff.machine_state));
        assert!(admitted(packet.coverage.coverage_state));
        for card in &packet.event_cards {
            assert!(
                admitted(card.machine_state),
                "procurement card state {} not admitted by the matrix",
                card.machine_state.as_str()
            );
        }
    }
}

#[test]
fn commercial_cards_never_outrank_recovery_actions() {
    let fixture = load_fixture();
    let mut any_loss = false;
    for packet in &fixture.profiles {
        for card in &packet.event_cards {
            if card.entitlement_loss {
                any_loss = true;
            }
            assert!(!card.outranks_recovery_actions);
            assert!(!card.requires_paid_seat_for_recovery);
            assert!(card.recovery_outranks_commercial());
            for action in [
                NextActionClass::ExportUserData,
                NextActionClass::DeleteUserData,
                NextActionClass::OpenSupport,
                NextActionClass::ContinueLocalOnly,
            ] {
                assert!(card.next_actions.iter().any(|a| a.action == action));
            }
        }
    }
    assert!(any_loss, "no entitlement-loss card appears");
}

#[test]
fn every_event_class_and_canonical_object_appears() {
    let fixture = load_fixture();
    for event in CommercialEventClass::ALL {
        assert!(
            fixture
                .profiles
                .iter()
                .any(|p| p.event_cards.iter().any(|c| c.event == event)),
            "event class {} never appears",
            event.as_str()
        );
    }
    for object in CanonicalObjectClass::ALL {
        assert!(
            fixture.profiles.iter().any(|p| {
                p.verification_packet
                    .canonical_sources
                    .iter()
                    .any(|c| c.object == object)
                    || p.event_cards
                        .iter()
                        .any(|card| card.canonical_sources.iter().any(|c| c.object == object))
                    || p.admin_handoff
                        .canonical_sources
                        .iter()
                        .any(|c| c.object == object)
            }),
            "canonical object {} never reused",
            object.as_str()
        );
    }
}

#[test]
fn verification_posture_is_never_silently_green() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        let vp = &packet.verification_packet;
        assert!(!vp.validity_window.window_label.is_empty());
        if vp.evidence_age.is_stale() || !vp.is_verified_now() || !vp.validity_window.within_window
        {
            assert_ne!(
                vp.machine_state,
                AdminStateClass::ActiveEnforced,
                "stale/unverified/past-validity packet shown active for {}",
                packet.profile.as_str()
            );
        }
    }
}

#[test]
fn admin_handoff_is_complete_and_auto_derived() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        let h = &packet.admin_handoff;
        assert!(!h.build_ref.is_empty());
        assert!(!h.bundle_ids.is_empty());
        assert!(!h.affected_features.is_empty());
        assert!(!h.export_safe_summary.is_empty());
        assert!(h.auto_derived);
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = procurement_lines(&fixture);
    assert!(lines.iter().any(|line| line.contains("Procurement bundle")));
    for profile in PROCUREMENT_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
