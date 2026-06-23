//! Unit tests for the M5 procurement bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(procurement_bundle(), procurement_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = procurement_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = procurement_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: ProcurementBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_managed_profile() {
    let bundle = procurement_bundle();
    assert_eq!(bundle.profiles.len(), PROCUREMENT_PROFILES.len());
    for profile in PROCUREMENT_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert_eq!(
            packet.surface,
            AdminSurfaceClass::ProcurementVerificationPacket
        );
        assert!(!packet.event_cards.is_empty());
    }
}

#[test]
fn every_rendered_state_is_admitted_by_the_matrix() {
    let bundle = procurement_bundle();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::ProcurementVerificationPacket)
            .expect("surface present")
            .applicable_states
            .contains(&state)
    };
    for packet in &bundle.profiles {
        assert!(admitted(packet.verification_packet.machine_state));
        assert!(admitted(packet.admin_handoff.machine_state));
        assert!(admitted(packet.coverage.coverage_state));
        for card in &packet.event_cards {
            assert!(
                admitted(card.machine_state),
                "{}: card state {} not admitted",
                packet.profile.as_str(),
                card.machine_state.as_str()
            );
        }
    }
}

#[test]
fn every_event_class_appears_and_discloses_impact() {
    let bundle = procurement_bundle();
    for event in CommercialEventClass::ALL {
        assert!(
            bundle
                .profiles
                .iter()
                .any(|p| p.event_cards.iter().any(|c| c.event == event)),
            "event class {} never appears",
            event.as_str()
        );
    }
    for packet in &bundle.profiles {
        for card in &packet.event_cards {
            assert!(!card.effective_date.is_empty());
            assert!(!card.impacted_features.is_empty());
            assert!(!card.local_only_path.is_empty());
            assert!(!card.export_next_step.is_empty());
            assert!(!card.support_next_step.is_empty());
        }
    }
}

#[test]
fn commercial_cards_never_outrank_recovery_actions() {
    let bundle = procurement_bundle();
    let mut any_loss = false;
    for packet in &bundle.profiles {
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
                assert!(
                    card.next_actions.iter().any(|a| a.action == action),
                    "{}: card {} missing recovery action {}",
                    packet.profile.as_str(),
                    card.event.as_str(),
                    action.as_str()
                );
            }
            let max_recovery = card.max_recovery_order().expect("recovery actions present");
            if let Some(min_commercial) = card.min_commercial_order() {
                assert!(max_recovery < min_commercial);
            }
        }
    }
    assert!(any_loss, "no entitlement-loss card appears");
}

#[test]
fn verification_packets_label_validity_and_never_silent_green() {
    let bundle = procurement_bundle();
    for packet in &bundle.profiles {
        let vp = &packet.verification_packet;
        assert!(!vp.validity_window.window_label.is_empty());
        if !vp.validity_window.within_window {
            assert_ne!(vp.machine_state, AdminStateClass::ActiveEnforced);
        }
        if !vp.is_verified_now() {
            assert_ne!(vp.machine_state, AdminStateClass::ActiveEnforced);
        }
        if vp.evidence_age.is_stale() {
            assert_ne!(vp.machine_state, AdminStateClass::ActiveEnforced);
        }
        assert!(!vp.supported_export_paths.is_empty());
        assert!(vp
            .supported_export_paths
            .iter()
            .any(|e| e.available_offline));
        assert!(vp
            .supported_export_paths
            .iter()
            .all(|e| !e.requires_paid_seat));
        assert!(!vp.requires_paid_seat_for_export);
    }
}

#[test]
fn surfaces_reuse_canonical_objects() {
    let bundle = procurement_bundle();
    for packet in &bundle.profiles {
        assert!(!packet.verification_packet.canonical_sources.is_empty());
        assert!(!packet.admin_handoff.canonical_sources.is_empty());
        for card in &packet.event_cards {
            assert!(!card.canonical_sources.is_empty());
        }
    }
    for object in CanonicalObjectClass::ALL {
        assert!(
            bundle.profiles.iter().any(|p| {
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
fn admin_handoffs_are_complete_and_auto_derived() {
    let bundle = procurement_bundle();
    for packet in &bundle.profiles {
        let h = &packet.admin_handoff;
        assert!(!h.build_ref.is_empty());
        assert!(!h.bundle_ids.is_empty());
        assert!(!h.affected_features.is_empty());
        assert!(!h.export_safe_summary.is_empty());
        assert!(h.auto_derived);
    }
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = procurement_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn human_readable_projection_renders_for_support() {
    let bundle = procurement_bundle();
    let lines = procurement_lines(&bundle);
    assert!(lines.iter().any(|line| line.contains("Procurement bundle")));
    for profile in PROCUREMENT_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
