//! Tests for the frozen usage-and-forecast view set.

use super::*;

fn set() -> UsageForecastViewSet {
    canonical_stable_usage_forecast_view_set()
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
        current_stable_usage_forecast_view_set().expect("checked-in set parses and validates");
    assert_eq!(
        stable,
        set(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn every_service_and_meter_family_has_a_view() {
    let s = set();
    assert_eq!(s.inspection.view_count, 6);
    assert_eq!(
        s.inspection.service_families_covered,
        ServiceFamily::ALL.len()
    );
    assert_eq!(s.inspection.meter_families_covered, MeterFamily::ALL.len());
    for family in ServiceFamily::ALL {
        assert!(
            s.views.iter().any(|v| v.service_family == family),
            "missing service family {family:?}"
        );
    }
    for family in MeterFamily::ALL {
        assert!(
            s.views.iter().any(|v| v.meter_family == family),
            "missing meter family {family:?}"
        );
    }
}

#[test]
fn unlike_families_never_merge_into_one_total() {
    let s = set();
    assert!(s.inspection.no_collapsed_family_total);
    // One view per family means as many views as families and distinct meter units.
    let families: std::collections::BTreeSet<_> =
        s.views.iter().map(|v| v.service_family).collect();
    assert_eq!(families.len(), s.views.len());
    let units: std::collections::BTreeSet<_> =
        s.views.iter().map(|v| v.measurement.meter_unit).collect();
    assert_eq!(
        units.len(),
        s.views.len(),
        "each family carries its own unit"
    );
}

#[test]
fn every_view_shows_unit_month_to_date_value_as_of_owner_threshold_and_export() {
    let s = set();
    assert!(s.inspection.value_never_bare);
    assert!(s.inspection.all_views_export_csv_json_parity);
    for v in &s.views {
        // Month-to-date value is bound to the unit, as-of time, and scope owner.
        assert_eq!(
            v.measurement.value_presentation,
            ValuePresentation::MonthToDateBoundToUnitAsOfScope,
            "view {} must bind its value",
            v.view_id
        );
        assert!(
            !v.measurement.carries_raw_number,
            "view {} leaked a raw number",
            v.view_id
        );
        assert!(
            !v.measurement.as_of.trim().is_empty(),
            "view {} missing as-of time",
            v.view_id
        );
        assert!(
            v.chargeback_scope_offers.len() >= 2,
            "view {} collapsed its scopes",
            v.view_id
        );
        // Threshold/forecast status and CSV/JSON export parity are present.
        assert!(v.export_parity.csv && v.export_parity.json && v.export_parity.parity_confirmed);
    }
}

#[test]
fn forecast_banners_explain_what_changes_next() {
    let s = set();
    assert!(s.inspection.all_banners_explain_what_changes_next);
    // Every threshold status is exercised across the views.
    assert_eq!(
        s.inspection.threshold_status_coverage,
        ThresholdStatus::ALL.len()
    );
    for v in &s.views {
        let banner = &v.forecast_banner;
        assert_eq!(banner.threshold_status, v.threshold_status);
        assert_eq!(banner, &ForecastBanner::for_status(v.threshold_status));
        assert!(
            !banner.what_changes_next.trim().is_empty(),
            "view {} banner must explain what changes next, not only a color",
            v.view_id
        );
        assert_eq!(banner.severity, v.threshold_status.severity());
    }
}

#[test]
fn every_surface_is_bound_and_renders_baseline_and_what_changes_next() {
    let s = set();
    for surface in UsageForecastSurface::ALL {
        let binding = s
            .surface_bindings
            .iter()
            .find(|b| b.surface == surface)
            .unwrap_or_else(|| panic!("missing surface {surface:?}"));
        assert!(binding.projects_effective_claim);
        assert!(binding.renders_local_safe_baseline);
        assert!(binding.explains_what_changes_next);
    }
}

#[test]
fn every_view_keeps_a_local_safe_baseline() {
    let s = set();
    assert!(s.inspection.all_views_local_safe_backed);
    for v in &s.views {
        assert!(
            !v.local_safe_baseline.is_empty(),
            "view {} lost its baseline",
            v.view_id
        );
        assert!(v.local_safe_baseline.iter().all(|b| !b.trim().is_empty()));
    }
}

#[test]
fn views_project_their_control_plane_lane() {
    let s = set();
    let violations = s.cross_check_against_control_plane();
    assert!(
        violations.is_empty(),
        "usage views drifted from the control-plane matrix: {violations:?}"
    );
}

#[test]
fn signed_in_does_not_narrow_any_view() {
    let mut s = set();
    s.apply_managed_state(ManagedStateClass::SignedIn);
    assert!(s.validate().is_empty());
    assert_eq!(s.inspection.effective_full_view_count, 6);
    assert_eq!(s.inspection.narrowed_view_count, 0);
    for v in &s.views {
        assert_eq!(v.effective_marketed_claim, MarketedClaim::ManagedFull);
        assert!(v.recovery_cue.is_none());
    }
}

#[test]
fn blocking_states_narrow_every_view_to_local_safe_only() {
    for state in [
        ManagedStateClass::ManagedBlocked,
        ManagedStateClass::SeatRemoved,
        ManagedStateClass::LocalOnly,
    ] {
        let mut s = set();
        s.apply_managed_state(state);
        assert!(s.validate().is_empty(), "validation failed for {state:?}");
        assert_eq!(s.inspection.local_safe_only_view_count, 6, "{state:?}");
        for v in &s.views {
            assert_eq!(
                v.effective_marketed_claim,
                MarketedClaim::LocalSafeOnly,
                "{state:?}"
            );
            // Local core is never blocked even when the marketed claim collapses.
            assert!(!v.local_safe_baseline.is_empty());
            assert!(v.recovery_cue.is_some());
            assert!(v.narrowing_reasons.contains(&state));
        }
    }
}

#[test]
fn meter_stale_narrows_but_never_blocks_the_local_core() {
    let mut s = set();
    s.apply_managed_state(ManagedStateClass::MeterStale);
    assert!(s.validate().is_empty());
    // A stale meter narrows the marketed claim but never forces local-safe-only.
    assert_eq!(s.inspection.local_safe_only_view_count, 0);
    assert_eq!(s.inspection.narrowed_view_count, 6);
    for v in &s.views {
        assert!(!v.local_safe_baseline.is_empty());
    }
}

#[test]
fn corrupted_effective_claim_is_rejected() {
    let mut s = set();
    s.apply_managed_state(ManagedStateClass::ManagedBlocked);
    // Forge a stronger marketed claim than the recomputation allows.
    s.views[0].effective_marketed_claim = MarketedClaim::ManagedFull;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "view.effective_marketed_claim"),
        "expected an effective-claim violation, got {violations:?}"
    );
}

#[test]
fn forged_raw_number_in_measurement_is_rejected() {
    let mut s = set();
    s.views[0].measurement.carries_raw_number = true;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "view.measurement.carries_raw_number"),
        "expected a raw-number violation, got {violations:?}"
    );
}

#[test]
fn forged_banner_is_rejected() {
    let mut s = set();
    // Swap the banner to one that does not match the view's threshold status.
    s.views[0].forecast_banner = ForecastBanner::for_status(ThresholdStatus::BudgetExhausted);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "view.forecast_banner"),
        "expected a forecast-banner violation, got {violations:?}"
    );
}

#[test]
fn broken_export_parity_is_rejected() {
    let mut s = set();
    s.views[0].export_parity.csv = false;
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "view.export_parity"),
        "expected an export-parity violation, got {violations:?}"
    );
}

#[test]
fn emptying_a_local_safe_baseline_is_rejected() {
    let mut s = set();
    s.views[0].local_safe_baseline.clear();
    s.inspection =
        UsageForecastInspection::derive(&s.views, &s.surface_bindings, s.active_managed_state);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "view.local_safe_baseline"),
        "expected a local-safe-baseline violation, got {violations:?}"
    );
}

#[test]
fn merging_two_families_into_one_view_is_rejected() {
    let mut s = set();
    // Force a second view onto the same service family as the first.
    s.views[1].service_family = s.views[0].service_family;
    s.inspection =
        UsageForecastInspection::derive(&s.views, &s.surface_bindings, s.active_managed_state);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "views"),
        "expected a views violation, got {violations:?}"
    );
}

#[test]
fn missing_surface_is_rejected() {
    let mut s = set();
    s.surface_bindings
        .retain(|b| b.surface != UsageForecastSurface::ReleaseCenter);
    s.inspection =
        UsageForecastInspection::derive(&s.views, &s.surface_bindings, s.active_managed_state);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "surface_bindings"),
        "expected a surface-binding violation, got {violations:?}"
    );
}

#[test]
fn meter_stale_status_must_label_measurement_stale() {
    let mut s = set();
    let idx = s
        .views
        .iter()
        .position(|v| v.threshold_status == ThresholdStatus::MeterStaleUnconfirmed)
        .expect("a meter-stale view is present");
    s.views[idx].measurement.freshness = SnapshotFreshness::FreshnessLive;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "view.measurement.freshness"),
        "expected a freshness violation, got {violations:?}"
    );
}

#[test]
fn view_for_family_resolves() {
    let s = set();
    let v = s
        .view_for_family(ServiceFamily::AiGatewayFamily)
        .expect("ai gateway view present");
    assert_eq!(v.measurement.meter_unit, MeterUnit::Tokens);
    assert!(v.backs_full_managed_claim());
}

#[test]
fn export_json_round_trips() {
    let s = set();
    let json = s.export_safe_json();
    let parsed: UsageForecastViewSet =
        serde_json::from_str(&json).expect("set round-trips through JSON");
    assert_eq!(parsed, s);
}
