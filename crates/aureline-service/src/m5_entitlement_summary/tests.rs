//! Tests for the frozen entitlement-summary set.

use super::*;

fn set() -> EntitlementSummarySet {
    canonical_stable_entitlement_summary_set()
}

#[test]
fn canonical_set_validates_clean() {
    let s = set();
    let violations = s.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn checked_in_set_matches_canonical_builder() {
    let stable =
        current_stable_entitlement_summary_set().expect("checked-in set parses and validates");
    assert_eq!(
        stable,
        set(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn every_managed_state_is_summarized_exactly_once() {
    let s = set();
    assert!(s.inspection.managed_state_vocab_complete);
    assert_eq!(s.summaries.len(), ManagedStateClass::ALL.len());
    for token in ManagedStateClass::ALL {
        let found = s
            .summaries
            .iter()
            .filter(|x| x.managed_state == token)
            .count();
        assert_eq!(found, 1, "managed state {token:?} must appear exactly once");
    }
}

#[test]
fn every_surface_is_bound_and_renders_continuation() {
    let s = set();
    assert!(s.inspection.surface_coverage_complete);
    for surface in SummarySurface::ALL {
        let binding = s
            .surface_bindings
            .iter()
            .find(|b| b.surface == surface)
            .unwrap_or_else(|| panic!("missing surface {surface:?}"));
        assert!(binding.projects_effective_claim);
        assert!(binding.renders_local_only_continuation);
    }
}

#[test]
fn every_summary_carries_local_only_continuation() {
    let s = set();
    assert!(s.inspection.all_summaries_carry_local_only_continuation);
    for summary in &s.summaries {
        assert!(
            !summary.local_only_continuation.is_empty(),
            "summary {} lost its local-only continuation",
            summary.summary_id
        );
        assert!(summary
            .local_only_continuation
            .iter()
            .all(|n| !n.trim().is_empty()));
    }
}

#[test]
fn effective_claim_and_degradation_track_the_state() {
    let s = set();
    for summary in &s.summaries {
        assert_eq!(
            summary.effective_marketed_claim,
            summary.managed_state.claim_cap(),
            "claim drift for {}",
            summary.summary_id
        );
        assert_eq!(
            summary.degradation,
            DegradationKind::for_state(summary.managed_state),
            "degradation drift for {}",
            summary.summary_id
        );
    }
}

#[test]
fn seat_loss_is_cited_to_the_seat_not_a_generic_sign_in() {
    let s = set();
    let seat = s
        .summary_for_state(ManagedStateClass::SeatRemoved)
        .expect("seat-removed summary present");
    assert_eq!(seat.posture_origin, PostureOrigin::Seat);
    assert!(seat.is_explicitly_blocked());
    // The sign-in (reauth) family is distinct from a seat loss.
    let reauth = s
        .summary_for_state(ManagedStateClass::ReauthRequired)
        .expect("reauth summary present");
    assert_eq!(reauth.posture_origin, PostureOrigin::Account);
    assert_ne!(seat.posture_origin, reauth.posture_origin);
    assert!(!reauth.is_explicitly_blocked());
}

#[test]
fn expired_entitlement_degrades_to_explicit_managed_blocked() {
    let s = set();
    for summary in &s.summaries {
        if summary.entitlement_state == EntitlementState::EntitlementExpired {
            assert_eq!(
                summary.degradation,
                DegradationKind::ManagedBlockedExplicit,
                "expired summary {} must be an explicit block",
                summary.summary_id
            );
        }
    }
    // The managed-blocked summary is the expiry exemplar.
    let blocked = s
        .summary_for_state(ManagedStateClass::ManagedBlocked)
        .expect("managed-blocked summary present");
    assert_eq!(
        blocked.entitlement_state,
        EntitlementState::EntitlementExpired
    );
    assert!(blocked.is_explicitly_blocked());
}

#[test]
fn local_only_summary_has_no_plan_role_or_quota() {
    let s = set();
    let local = s
        .summary_for_state(ManagedStateClass::LocalOnly)
        .expect("local-only summary present");
    assert_eq!(local.plan_tier, PlanTier::LocalOnlyNoPlan);
    assert_eq!(local.role, AccountRole::NoManagedRole);
    assert!(local.quota_snapshot.is_none());
    assert_eq!(local.degradation, DegradationKind::LocalOnlyNoAccount);
}

#[test]
fn meter_stale_labels_the_snapshot_and_never_blocks_local_core() {
    let s = set();
    let stale = s
        .summary_for_state(ManagedStateClass::MeterStale)
        .expect("meter-stale summary present");
    let snapshot = stale.quota_snapshot.as_ref().expect("snapshot present");
    assert_eq!(snapshot.freshness, SnapshotFreshness::FreshnessStale);
    // A stale meter narrows but never collapses to local-safe-only.
    assert_ne!(stale.effective_marketed_claim, MarketedClaim::LocalSafeOnly);
    assert!(!stale.local_only_continuation.is_empty());
}

#[test]
fn no_quota_snapshot_carries_a_raw_number() {
    let s = set();
    for summary in &s.summaries {
        if let Some(snapshot) = &summary.quota_snapshot {
            assert!(
                !snapshot.carries_raw_number,
                "summary {} leaked a raw number",
                summary.summary_id
            );
            assert!(!snapshot.as_of.trim().is_empty());
        }
    }
}

#[test]
fn forged_generic_error_is_rejected() {
    // Forge a seat loss attributed to a generic account/sign-in origin.
    let mut s = set();
    let idx = s
        .summaries
        .iter()
        .position(|x| x.managed_state == ManagedStateClass::SeatRemoved)
        .unwrap();
    s.summaries[idx].posture_origin = PostureOrigin::Account;
    s.inspection = EntitlementSummaryInspection::derive(&s.summaries, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "summary.posture_origin"),
        "expected a posture-origin violation, got {violations:?}"
    );
}

#[test]
fn emptying_local_only_continuation_is_rejected() {
    let mut s = set();
    s.summaries[0].local_only_continuation.clear();
    s.inspection = EntitlementSummaryInspection::derive(&s.summaries, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "summary.local_only_continuation"),
        "expected a local-only-continuation violation, got {violations:?}"
    );
}

#[test]
fn forged_raw_number_in_snapshot_is_rejected() {
    let mut s = set();
    let idx = s
        .summaries
        .iter()
        .position(|x| x.quota_snapshot.is_some())
        .unwrap();
    if let Some(snapshot) = s.summaries[idx].quota_snapshot.as_mut() {
        snapshot.carries_raw_number = true;
    }
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "summary.quota_snapshot.carries_raw_number"),
        "expected a raw-number violation, got {violations:?}"
    );
}

#[test]
fn missing_surface_is_rejected() {
    let mut s = set();
    s.surface_bindings
        .retain(|b| b.surface != SummarySurface::FeatureEntryPoint);
    s.inspection = EntitlementSummaryInspection::derive(&s.summaries, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "surface_bindings"),
        "expected a surface-binding violation, got {violations:?}"
    );
}

#[test]
fn export_json_round_trips() {
    let s = set();
    let json = s.export_safe_json();
    let parsed: EntitlementSummarySet =
        serde_json::from_str(&json).expect("set round-trips through JSON");
    assert_eq!(parsed, s);
}
