//! Unit tests for the reactive-recovery packet, drills, and fixtures.

use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_reactive_recovery_packet();
    validate_reactive_recovery_packet(&packet)
        .expect("seeded reactive-recovery packet must validate");
}

#[test]
fn seeded_fixtures_validate() {
    let packet = seeded_reactive_recovery_packet();
    for fixture in seeded_reactive_recovery_fixtures() {
        validate_reactive_recovery_fixture(&packet, &fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn no_flow_offers_exact_truth_while_behind() {
    let packet = seeded_reactive_recovery_packet();
    for row in &packet.flows {
        assert!(
            !row.offers_exact_truth_action,
            "lagging flow {} must not offer an exact-truth action",
            row.flow_id
        );
        assert!(
            !row.epoch_posture.is_current(),
            "recovery flow {} should describe a non-current epoch",
            row.flow_id
        );
        assert!(
            !row.action_posture.allows_exact_truth(),
            "recovery flow {} must not keep an exact-truth action posture",
            row.flow_id
        );
    }
}

#[test]
fn no_flow_allows_silent_retry_after_material_change() {
    let packet = seeded_reactive_recovery_packet();
    for row in &packet.flows {
        assert!(
            !row.silent_retry_allowed,
            "flow {} changed action posture and must not retry silently",
            row.flow_id
        );
    }
}

#[test]
fn every_named_drill_is_present_and_honest() {
    let packet = seeded_reactive_recovery_packet();
    let drilled: std::collections::BTreeSet<_> = packet
        .drills
        .iter()
        .map(|drill| drill.lag_condition)
        .collect();
    for required in [
        LagCondition::RapidInvalidationBurst,
        LagCondition::ConsumerLag,
        LagCondition::ReconnectAfterDrop,
        LagCondition::ProviderOverlayDisappeared,
    ] {
        assert!(
            drilled.contains(&required),
            "packet must drill {}",
            required.as_str()
        );
    }

    for drill in &packet.drills {
        assert!(drill.asserts_no_stale_exact_action);
        assert!(drill.asserts_recovery_visible);
        for window in drill.steps.windows(2) {
            // Steps never reach the verify phase before a narrow-action step.
            if window[0].phase == DrillPhase::Verify {
                panic!("drill {} verified before finishing", drill.drill_id);
            }
        }
    }
}

#[test]
fn provider_overlay_drill_stays_blocked_when_provider_is_gone() {
    let packet = seeded_reactive_recovery_packet();
    let drill = packet
        .drills
        .iter()
        .find(|drill| drill.lag_condition == LagCondition::ProviderOverlayDisappeared)
        .expect("provider overlay drill exists");
    assert_eq!(
        drill.expected_final_epoch_posture,
        EpochPosture::StaleEpoch,
        "a missing provider must not resolve to the current epoch"
    );
    assert_eq!(
        drill.expected_final_action_posture,
        ActionPosture::Blocked,
        "a missing provider must keep dependent actions blocked"
    );
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_reactive_recovery_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let parsed: ReactiveRecoveryPacket = serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn fixture_is_seeded_for_every_flow() {
    let packet = seeded_reactive_recovery_packet();
    let fixtures = seeded_reactive_recovery_fixtures();
    assert_eq!(
        fixtures.len(),
        packet.flows.len(),
        "one fixture per recovery flow"
    );
}
