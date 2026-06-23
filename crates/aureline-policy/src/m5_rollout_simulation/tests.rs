//! Unit tests for the M5 rollout-simulation bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(rollout_simulation_bundle(), rollout_simulation_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = rollout_simulation_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = rollout_simulation_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: RolloutSimulationBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_managed_profile() {
    let bundle = rollout_simulation_bundle();
    assert_eq!(bundle.profiles.len(), SIMULATED_PROFILES.len());
    for profile in SIMULATED_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        assert!(!packet.scenarios.is_empty());
    }
}

#[test]
fn every_endpoint_state_is_admitted_by_the_matrix() {
    let bundle = rollout_simulation_bundle();
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| {
        matrix
            .surface(AdminSurfaceClass::EndpointPostureCard)
            .expect("surface present")
            .applicable_states
            .contains(&state)
    };
    for packet in &bundle.profiles {
        assert!(
            admitted(packet.claim_state),
            "{}: claim state {} not admitted",
            packet.profile.as_str(),
            packet.claim_state.as_str()
        );
        for scenario in &packet.scenarios {
            for ep in &scenario.impacted_endpoints {
                assert!(admitted(ep.posture_before), "before state not admitted");
                assert!(admitted(ep.posture_after), "after state not admitted");
            }
        }
    }
}

#[test]
fn widening_is_gated_harder_than_tightening() {
    let bundle = rollout_simulation_bundle();
    for scenario in bundle.scenarios() {
        if scenario.is_widening() {
            assert!(
                scenario.meets_widening_floor(),
                "widening scenario {} must meet the widening floor",
                scenario.scenario_id
            );
            assert!(
                scenario.review_requirement.rank()
                    >= ReviewRequirementClass::DualControlReview.rank()
            );
            assert!(!scenario.staging.is_immediate());
            assert!(!scenario.rollback.is_instant());
        } else {
            assert!(
                scenario.within_tightening_ceiling(),
                "tightening scenario {} exceeded the tightening ceiling",
                scenario.scenario_id
            );
            assert!(scenario.widening_dimensions.is_empty());
        }
    }
}

#[test]
fn at_least_one_tightening_stays_light() {
    let bundle = rollout_simulation_bundle();
    assert!(
        bundle.scenarios().any(RolloutScenario::is_light_tightening),
        "a light, immediately-applicable tightening must exist to prove restrictions are not \
         over-gated"
    );
}

#[test]
fn stale_scenarios_are_never_safe_to_promote() {
    let bundle = rollout_simulation_bundle();
    let mut saw_stale = false;
    for scenario in bundle.scenarios() {
        if scenario.simulation_freshness.is_stale() {
            saw_stale = true;
            assert_eq!(
                scenario.outcome,
                SimulationOutcomeClass::BlockedStaleEvidence
            );
            assert!(!scenario.outcome.is_safe_to_promote());
        }
    }
    assert!(saw_stale, "the bundle should exercise a stale scenario");
}

#[test]
fn claim_auto_narrows_when_evidence_is_stale() {
    let bundle = rollout_simulation_bundle();

    // Sovereign: simulation and posture evidence are stale.
    let sovereign = bundle
        .packet(AdminPathClass::SovereignAirGapped)
        .expect("sovereign present");
    assert!(!sovereign.claim_confirmed());
    assert!(sovereign
        .narrow_reasons
        .contains(&ClaimNarrowReasonClass::SimulationEvidenceStale));
    assert!(sovereign
        .narrow_reasons
        .contains(&ClaimNarrowReasonClass::EndpointPostureStale));

    // Mirrored: the offline mirror is stale.
    let mirrored = bundle
        .packet(AdminPathClass::MirroredOffline)
        .expect("mirrored present");
    assert!(!mirrored.claim_confirmed());
    assert_eq!(
        mirrored.claim_state,
        AdminStateClass::MirrorOfflineLastKnown
    );
    assert!(mirrored
        .narrow_reasons
        .contains(&ClaimNarrowReasonClass::MirrorFreshnessStale));

    // Managed cloud: all evidence fresh, claim confirmed.
    let managed = bundle
        .packet(AdminPathClass::ManagedCloud)
        .expect("managed present");
    assert!(managed.claim_confirmed());
    assert!(managed.narrow_reasons.is_empty());
}

#[test]
fn boundary_recheck_blocks_promotion() {
    let bundle = rollout_simulation_bundle();
    let blocked = bundle
        .scenarios()
        .find(|s| s.review_requirement == ReviewRequirementClass::BlockedPendingBoundaryRecheck)
        .expect("a boundary-recheck scenario exists");
    assert_eq!(
        blocked.outcome,
        SimulationOutcomeClass::BlockedBoundaryRecheck
    );
    assert_eq!(blocked.staging, RolloutStagingClass::PinnedManualSignedOnly);
    assert!(blocked.outcome.is_blocked());
}

#[test]
fn every_change_kind_and_widening_dimension_is_covered() {
    let bundle = rollout_simulation_bundle();
    for kind in RolloutChangeKindClass::ALL {
        assert!(
            bundle.scenarios().any(|s| s.change_kind == kind),
            "change kind {} not covered",
            kind.as_str()
        );
    }
    for dim in WideningDimensionClass::ALL {
        assert!(
            bundle
                .scenarios()
                .any(|s| s.widening_dimensions.contains(&dim)),
            "widening dimension {} not covered",
            dim.as_str()
        );
    }
}

#[test]
fn every_scenario_is_a_reviewable_dry_run() {
    let bundle = rollout_simulation_bundle();
    for scenario in bundle.scenarios() {
        assert!(scenario.dry_run);
        assert!(!scenario.impacted_endpoints.is_empty());
        assert!(!scenario.impacted_features.is_empty());
        assert!(!scenario.review_note.is_empty());
    }
}

#[test]
fn tightening_never_flags_a_widened_feature() {
    let bundle = rollout_simulation_bundle();
    for scenario in bundle.scenarios() {
        if !scenario.is_widening() {
            assert!(scenario.impacted_features.iter().all(|f| !f.newly_widened));
        }
    }
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = rollout_simulation_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn human_readable_projection_mentions_every_profile() {
    let bundle = rollout_simulation_bundle();
    let lines = rollout_simulation_lines(&bundle);
    assert!(lines
        .iter()
        .any(|l| l.contains("Rollout-simulation bundle")));
    for profile in SIMULATED_PROFILES {
        assert!(
            lines.iter().any(|l| l.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_a_widening_scenario_that_can_apply_immediately() {
    let mut bundle = rollout_simulation_bundle();
    let scenario = bundle
        .profiles
        .iter_mut()
        .flat_map(|p| p.scenarios.iter_mut())
        .find(|s| s.is_widening())
        .expect("a widening scenario exists");
    // Drop the staged-rollout requirement without changing the direction.
    scenario.staging = RolloutStagingClass::ImmediateAllowed;
    // Recompute would fix invariants, but the frozen bundle keeps stale flags;
    // re-run validation against recomputed invariants.
    let rebuilt = RolloutSimulationBundle {
        invariants: compute_invariants(&bundle.profiles),
        ..bundle
    };
    assert!(rebuilt.validate().is_err());
}
