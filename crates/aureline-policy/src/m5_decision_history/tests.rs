//! Unit tests for the M5 decision-history bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(decision_history_bundle(), decision_history_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = decision_history_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = decision_history_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: DecisionHistoryBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_managed_profile() {
    let bundle = decision_history_bundle();
    assert_eq!(bundle.profiles.len(), HISTORY_PROFILES.len());
    for profile in HISTORY_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.timeline.events.is_empty());
    }
}

#[test]
fn every_rendered_state_is_admitted_by_the_matrix() {
    let bundle = decision_history_bundle();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::DecisionHistoryTimeline)
            .expect("surface present")
            .applicable_states
            .contains(&state)
    };
    for packet in &bundle.profiles {
        for event in &packet.timeline.events {
            assert!(
                admitted(event.outcome_state),
                "{}: event state {} not admitted by the matrix",
                packet.profile.as_str(),
                event.outcome_state.as_str()
            );
        }
        assert!(
            admitted(packet.timeline.coverage.coverage_state),
            "{}: coverage state {} not admitted by the matrix",
            packet.profile.as_str(),
            packet.timeline.coverage.coverage_state.as_str()
        );
    }
}

#[test]
fn explorer_offers_every_family_and_every_event_resolves() {
    let bundle = decision_history_bundle();
    for packet in &bundle.profiles {
        for family in EventFamilyClass::ALL {
            assert!(
                packet.timeline.filter(family).is_some(),
                "{}: missing filter {}",
                packet.profile.as_str(),
                family.as_str()
            );
        }
        for event in &packet.timeline.events {
            let filter = packet
                .timeline
                .filter(event.event_family)
                .expect("family filter present");
            assert!(
                filter.matched_event_ids.contains(&event.event_id),
                "{}: event {} not listed under its filter",
                packet.profile.as_str(),
                event.event_id
            );
        }
    }
}

#[test]
fn actor_classes_are_distinguished_and_all_present() {
    let bundle = decision_history_bundle();
    for packet in &bundle.profiles {
        assert!(
            packet.timeline.actor_classes().len() >= 2,
            "{}: timeline collapses to one actor class",
            packet.profile.as_str()
        );
    }
    for actor in ActorClass::ALL {
        assert!(
            bundle.profiles.iter().any(|p| p
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
fn every_event_has_export_parity_and_both_forms_offered() {
    let bundle = decision_history_bundle();
    for packet in &bundle.profiles {
        for event in &packet.timeline.events {
            assert!(
                event.has_export_parity(),
                "{}: event {} lacks an export representation",
                packet.profile.as_str(),
                event.event_id
            );
        }
        assert!(packet
            .timeline
            .offers(ExportFormatClass::MachineReadableJson));
        assert!(packet
            .timeline
            .offers(ExportFormatClass::PlainLanguageHandoff));
    }
}

#[test]
fn stale_evidence_never_sits_under_a_confirmed_state() {
    let bundle = decision_history_bundle();
    for packet in &bundle.profiles {
        for event in &packet.timeline.events {
            if event.evidence_age.is_stale() {
                assert!(
                    !requires_fresh_evidence(event.outcome_state),
                    "{}: stale event {} shown under a confirmed state {}",
                    packet.profile.as_str(),
                    event.event_id,
                    event.outcome_state.as_str()
                );
            }
        }
    }
}

#[test]
fn every_profile_is_locally_inspectable_without_a_console() {
    let bundle = decision_history_bundle();
    for packet in &bundle.profiles {
        assert!(packet.timeline.coverage.locally_inspectable);
        assert!(packet.timeline.coverage.vendor_console_independent);
    }
}

#[test]
fn force_disable_decisions_link_to_an_explanation() {
    let bundle = decision_history_bundle();
    for packet in &bundle.profiles {
        for event in &packet.timeline.events {
            if event.decision_code == DecisionCodeClass::ForceDisable {
                assert!(
                    event.explanation_ref.is_some(),
                    "{}: force-disable event {} has no explanation link",
                    packet.profile.as_str(),
                    event.event_id
                );
            }
        }
    }
}

#[test]
fn provider_and_client_limitations_are_not_collapsed_into_denials() {
    let bundle = decision_history_bundle();
    let has = |actor: ActorClass| {
        bundle
            .profiles
            .iter()
            .any(|p| p.timeline.events.iter().any(|e| e.actor_class == actor))
    };
    assert!(has(ActorClass::ProviderLimitation));
    assert!(has(ActorClass::ClientLimitation));
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = decision_history_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn consumer_parity_matches_the_matrix_declaration() {
    let bundle = decision_history_bundle();
    let declared = admin_plane_matrix()
        .surface(AdminSurfaceClass::DecisionHistoryTimeline)
        .expect("surface present")
        .consumed_by
        .clone();
    assert!(!declared.is_empty());
    for packet in &bundle.profiles {
        for consumer in &declared {
            assert!(
                packet.consumers.contains(consumer),
                "{}: packet does not serve declared consumer {:?}",
                packet.profile.as_str(),
                consumer
            );
        }
    }
}

#[test]
fn human_readable_projection_mentions_every_profile() {
    let bundle = decision_history_bundle();
    let lines = decision_history_lines(&bundle);
    assert!(lines.iter().any(|l| l.contains("Decision-history bundle")));
    for profile in HISTORY_PROFILES {
        assert!(
            lines.iter().any(|l| l.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_a_force_disable_with_no_explanation() {
    let mut bundle = decision_history_bundle();
    let packet = &mut bundle.profiles[0];
    let event = packet
        .timeline
        .events
        .iter_mut()
        .find(|e| e.decision_code == DecisionCodeClass::ForceDisable)
        .expect("a force-disable event exists");
    event.explanation_ref = None;
    // Recompute invariants so the ownership invariant reflects the edit.
    bundle.invariants = compute_invariants(&bundle.profiles);
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_an_event_missing_from_its_filter() {
    let mut bundle = decision_history_bundle();
    let packet = &mut bundle.profiles[0];
    // Drop the matched ids from the filter that backs the first event's family;
    // that event no longer resolves to its filter.
    let family = packet.timeline.events[0].event_family;
    let filter = packet
        .timeline
        .filters
        .iter_mut()
        .find(|f| f.family == family)
        .expect("family filter present");
    filter.matched_event_ids.clear();
    assert!(bundle.validate().is_err());
}
