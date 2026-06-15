//! Tests for the frozen commercial-control-plane matrix.

use super::*;

fn matrix() -> CommercialControlPlaneMatrix {
    canonical_stable_commercial_control_plane_matrix()
}

#[test]
fn canonical_matrix_validates_clean() {
    let m = matrix();
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn checked_in_matrix_matches_canonical_builder() {
    let stable = current_stable_commercial_control_plane_matrix()
        .expect("checked-in matrix parses and validates");
    assert_eq!(
        stable,
        matrix(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn every_service_and_meter_family_has_a_lane() {
    let m = matrix();
    assert_eq!(m.inspection.lane_count, 6);
    assert_eq!(
        m.inspection.service_families_covered,
        ServiceFamily::ALL.len()
    );
    assert_eq!(m.inspection.meter_families_covered, MeterFamily::ALL.len());
    for family in ServiceFamily::ALL {
        assert!(
            m.lanes.iter().any(|l| l.service_family == family),
            "missing service family {family:?}"
        );
    }
    for family in MeterFamily::ALL {
        assert!(
            m.lanes.iter().any(|l| l.meter_family == family),
            "missing meter family {family:?}"
        );
    }
}

#[test]
fn managed_state_vocabulary_is_complete_and_locked() {
    let m = matrix();
    assert!(m.inspection.managed_state_vocab_complete);
    assert_eq!(m.managed_states.len(), ManagedStateClass::ALL.len());
    for token in ManagedStateClass::ALL {
        let row = m
            .managed_states
            .iter()
            .find(|r| r.managed_state == token)
            .unwrap_or_else(|| panic!("missing managed-state token {token:?}"));
        assert_eq!(row.claim_cap, token.claim_cap());
        assert!(row.local_safe_guaranteed);
    }
}

#[test]
fn every_consumer_surface_is_bound() {
    let m = matrix();
    assert!(m.inspection.consumer_surface_coverage_complete);
    for surface in ConsumerSurface::ALL {
        assert!(
            m.consumer_bindings
                .iter()
                .any(|b| b.consumer_surface == surface),
            "missing consumer surface {surface:?}"
        );
    }
}

#[test]
fn every_lane_keeps_a_local_safe_baseline_and_an_as_of_time() {
    let m = matrix();
    assert!(m.inspection.all_lanes_local_safe_backed);
    for lane in &m.lanes {
        assert!(
            !lane.local_safe_baseline.is_empty(),
            "lane {} lost its local-safe baseline",
            lane.lane_id
        );
        assert_ne!(
            lane.as_of_time_requirement,
            AsOfTimeRequirement::NotApplicable,
            "lane {} must require an as-of time",
            lane.lane_id
        );
        assert!(
            lane.chargeback_scope_offers.len() >= 2,
            "lane {} must keep distinct chargeback scopes",
            lane.lane_id
        );
    }
}

#[test]
fn distinct_loss_states_never_collapse() {
    let m = matrix();
    let distinct = [
        ManagedStateClass::SeatRemoved,
        ManagedStateClass::OrgSwitched,
        ManagedStateClass::GracePeriod,
        ManagedStateClass::ReauthRequired,
    ];
    for token in distinct {
        let row = m
            .managed_states
            .iter()
            .find(|r| r.managed_state == token)
            .unwrap();
        for other in distinct {
            if other != token {
                assert!(
                    row.must_not_collapse_with.contains(&other),
                    "{token:?} must stay distinct from {other:?}"
                );
            }
        }
    }
}

#[test]
fn signed_in_does_not_narrow_any_lane() {
    let mut m = matrix();
    m.apply_managed_state(ManagedStateClass::SignedIn);
    assert!(m.validate().is_empty());
    assert_eq!(m.inspection.effective_full_lane_count, 6);
    assert_eq!(m.inspection.narrowed_lane_count, 0);
    for lane in &m.lanes {
        assert_eq!(lane.effective_marketed_claim, MarketedClaim::ManagedFull);
        assert!(lane.recovery_cue.is_none());
    }
}

#[test]
fn managed_blocked_narrows_every_lane_to_local_safe_only() {
    for state in [
        ManagedStateClass::ManagedBlocked,
        ManagedStateClass::SeatRemoved,
        ManagedStateClass::LocalOnly,
    ] {
        let mut m = matrix();
        m.apply_managed_state(state);
        assert!(m.validate().is_empty(), "validation failed for {state:?}");
        assert_eq!(m.inspection.local_safe_only_lane_count, 6, "{state:?}");
        assert_eq!(m.inspection.effective_full_lane_count, 0, "{state:?}");
        for lane in &m.lanes {
            assert_eq!(
                lane.effective_marketed_claim,
                MarketedClaim::LocalSafeOnly,
                "lane {} under {state:?}",
                lane.lane_id
            );
            // Local core is never blocked even when the marketed claim collapses.
            assert!(!lane.local_safe_baseline.is_empty());
            assert!(lane.recovery_cue.is_some());
            assert!(lane.narrowing_reasons.contains(&state));
        }
    }
}

#[test]
fn grace_and_threshold_states_narrow_to_managed_narrowed() {
    for state in [
        ManagedStateClass::GracePeriod,
        ManagedStateClass::PlanDowngrade,
        ManagedStateClass::OrgSwitched,
        ManagedStateClass::ForecastThreshold,
        ManagedStateClass::MeterStale,
        ManagedStateClass::ReauthRequired,
    ] {
        let mut m = matrix();
        m.apply_managed_state(state);
        assert!(m.validate().is_empty(), "validation failed for {state:?}");
        assert_eq!(m.inspection.narrowed_lane_count, 6, "{state:?}");
        assert_eq!(m.inspection.local_safe_only_lane_count, 0, "{state:?}");
        for lane in &m.lanes {
            assert_eq!(
                lane.effective_marketed_claim,
                MarketedClaim::ManagedNarrowed,
                "lane {} under {state:?}",
                lane.lane_id
            );
            assert!(lane.recovery_cue.is_some());
        }
    }
}

#[test]
fn meter_stale_does_not_collapse_to_local_safe_only() {
    // A stale meter narrows the marketed claim but must never force local-safe-only,
    // because the local core is unaffected by a stale metering path.
    let mut m = matrix();
    m.apply_managed_state(ManagedStateClass::MeterStale);
    assert_eq!(m.inspection.local_safe_only_lane_count, 0);
}

#[test]
fn corrupted_effective_claim_is_rejected() {
    let mut m = matrix();
    // Forge a stronger marketed claim than the recomputation allows.
    m.apply_managed_state(ManagedStateClass::ManagedBlocked);
    m.lanes[0].effective_marketed_claim = MarketedClaim::ManagedFull;
    let violations = m.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "lane.effective_marketed_claim"),
        "expected an effective-claim violation, got {violations:?}"
    );
}

#[test]
fn missing_consumer_surface_is_rejected() {
    let mut m = matrix();
    m.consumer_bindings
        .retain(|b| b.consumer_surface != ConsumerSurface::ClaimPublicTruthAutomation);
    m.inspection = CommercialControlPlaneInspection::derive(
        &m.lanes,
        &m.managed_states,
        &m.consumer_bindings,
        m.active_managed_state,
    );
    let violations = m.validate();
    assert!(
        violations.iter().any(|v| v.field == "consumer_bindings"),
        "expected a consumer-binding violation, got {violations:?}"
    );
}

#[test]
fn emptying_a_local_safe_baseline_is_rejected() {
    let mut m = matrix();
    m.lanes[0].local_safe_baseline.clear();
    m.inspection = CommercialControlPlaneInspection::derive(
        &m.lanes,
        &m.managed_states,
        &m.consumer_bindings,
        m.active_managed_state,
    );
    let violations = m.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "lane.local_safe_baseline"),
        "expected a local-safe-baseline violation, got {violations:?}"
    );
}

#[test]
fn export_json_round_trips() {
    let m = matrix();
    let json = m.export_safe_json();
    let parsed: CommercialControlPlaneMatrix =
        serde_json::from_str(&json).expect("matrix round-trips through JSON");
    assert_eq!(parsed, m);
}
