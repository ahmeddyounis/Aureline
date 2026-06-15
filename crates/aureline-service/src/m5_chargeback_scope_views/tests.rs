//! Tests for the frozen chargeback-scope view set.

use super::*;

fn set() -> ChargebackScopeViewSet {
    canonical_stable_chargeback_scope_view_set()
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
        current_stable_chargeback_scope_view_set().expect("checked-in set parses and validates");
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
fn personal_workspace_team_org_scopes_never_collapse() {
    let s = set();
    assert!(s.inspection.no_collapsed_scope_total);
    // The headline four scopes are all exercised, plus tenant.
    for scope in [
        ScopeOwner::Personal,
        ScopeOwner::Workspace,
        ScopeOwner::Team,
        ScopeOwner::Organization,
    ] {
        assert!(
            s.views
                .iter()
                .any(|v| v.scope_cost_truths.iter().any(|t| t.scope_owner == scope)),
            "scope {scope:?} never appears"
        );
    }
    for v in &s.views {
        let scopes: std::collections::BTreeSet<_> =
            v.scope_cost_truths.iter().map(|t| t.scope_owner).collect();
        assert!(scopes.len() >= 2, "view {} collapsed its scopes", v.view_id);
        assert_eq!(
            scopes.len(),
            v.scope_cost_truths.len(),
            "view {} repeats a scope",
            v.view_id
        );
    }
}

#[test]
fn every_scope_separates_direct_from_inherited() {
    let s = set();
    assert!(s.inspection.all_views_separate_direct_and_inherited);
    for v in &s.views {
        for t in &v.scope_cost_truths {
            assert_eq!(t.direct.attribution_basis, AttributionBasis::Direct);
            assert_eq!(t.inherited.attribution_basis, AttributionBasis::Inherited);
            // A direct line never names a parent; it is always bound.
            assert!(t.direct.inherited_from.is_none());
            assert_eq!(
                t.direct.value_presentation,
                ValuePresentation::MonthToDateBoundToUnitAsOfScope
            );
        }
    }
}

#[test]
fn inherited_line_names_recomputed_parent_or_marks_root() {
    let s = set();
    for v in &s.views {
        let offered: Vec<ScopeOwner> = v.scope_cost_truths.iter().map(|t| t.scope_owner).collect();
        // Exactly one scope per view is the chain root with no inherited parent.
        let roots = v
            .scope_cost_truths
            .iter()
            .filter(|t| t.inherited.inherited_from.is_none())
            .count();
        assert_eq!(
            roots, 1,
            "view {} must have exactly one chain root",
            v.view_id
        );
        for t in &v.scope_cost_truths {
            let expected = parent_scope_in(t.scope_owner, &offered);
            assert_eq!(
                t.inherited.inherited_from, expected,
                "view {} scope {:?} parent drifted",
                v.view_id, t.scope_owner
            );
            match expected {
                Some(_) => assert_eq!(
                    t.inherited.value_presentation,
                    ValuePresentation::MonthToDateBoundToUnitAsOfScope
                ),
                None => assert_eq!(
                    t.inherited.value_presentation,
                    ValuePresentation::SuppressedNoManagedNumber,
                    "the root inherited line must suppress, not imply a zero"
                ),
            }
        }
    }
}

#[test]
fn no_cost_line_carries_a_raw_number() {
    let s = set();
    assert!(s.inspection.value_never_bare);
    for v in &s.views {
        for t in &v.scope_cost_truths {
            assert!(
                !t.direct.carries_raw_number,
                "view {} leaked a raw number",
                v.view_id
            );
            assert!(!t.inherited.carries_raw_number);
            assert!(!t.direct.as_of.trim().is_empty());
            assert!(!t.inherited.as_of.trim().is_empty());
        }
    }
}

#[test]
fn csv_and_json_export_at_parity() {
    let s = set();
    assert!(s.inspection.all_views_export_csv_json_parity);
    let csv = s.export_safe_csv();
    let mut lines = csv.lines();
    assert_eq!(lines.next(), Some(CHARGEBACK_CSV_HEADER));
    let columns = CHARGEBACK_CSV_HEADER.split(',').count();
    // One CSV row per view, scope, and attribution basis (direct + inherited).
    let expected_rows: usize = s.views.iter().map(|v| v.scope_cost_truths.len() * 2).sum();
    let body: Vec<&str> = lines.collect();
    assert_eq!(
        body.len(),
        expected_rows,
        "CSV must carry one row per scope line"
    );
    for row in &body {
        assert_eq!(
            row.split(',').count(),
            columns,
            "CSV row column count must match the header"
        );
    }
    // The CSV export is deterministic.
    assert_eq!(csv, s.export_safe_csv());
    // JSON round-trips, so CSV and JSON project the same set.
    let parsed: ChargebackScopeViewSet =
        serde_json::from_str(&s.export_safe_json()).expect("set round-trips through JSON");
    assert_eq!(parsed, s);
}

#[test]
fn switcher_preserves_active_scope_on_switch() {
    let mut s = set();
    assert_eq!(s.switcher.active_scope, STABLE_ACTIVE_SCOPE);
    for scope in s.switcher.available_scopes.clone() {
        s.switch_scope(scope);
        assert!(s.validate().is_empty(), "switch to {scope:?} broke the set");
        assert_eq!(s.switcher.active_scope, scope);
        assert_eq!(s.inspection.active_scope, scope);
        // Switching never collapses scopes or drops a view.
        assert_eq!(s.inspection.view_count, 6);
        assert!(s.switcher.preserves_active_scope);
        assert!(s.switcher.preserves_inherited_direct_separation);
        assert!(s.switcher.preserves_owner_identity);
        assert!(s.switcher.never_collapses_scopes);
        assert!(s.inspection.all_views_separate_direct_and_inherited);
    }
}

#[test]
fn views_project_their_control_plane_lane() {
    let s = set();
    let violations = s.cross_check_against_control_plane();
    assert!(
        violations.is_empty(),
        "chargeback views drifted from the control-plane matrix: {violations:?}"
    );
}

#[test]
fn every_surface_is_bound() {
    let s = set();
    for surface in ChargebackScopeSurface::ALL {
        let binding = s
            .surface_bindings
            .iter()
            .find(|b| b.surface == surface)
            .unwrap_or_else(|| panic!("missing surface {surface:?}"));
        assert!(binding.projects_effective_claim);
        assert!(binding.renders_inherited_direct_separation);
        assert!(binding.renders_local_safe_baseline);
    }
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
            // Local core is never blocked and the scopes stay inspectable.
            assert!(!v.local_safe_baseline.is_empty());
            assert!(v.recovery_cue.is_some());
            assert!(v.narrowing_reasons.contains(&state));
            assert!(v.scope_cost_truths.len() >= 2);
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
fn distinct_loss_states_each_narrow_with_their_own_recovery_cue() {
    // Seat loss, org switch, grace, and reauth never collapse into one error.
    let states = [
        ManagedStateClass::SeatRemoved,
        ManagedStateClass::OrgSwitched,
        ManagedStateClass::GracePeriod,
        ManagedStateClass::ReauthRequired,
    ];
    let mut cues = std::collections::BTreeSet::new();
    for state in states {
        let mut s = set();
        s.apply_managed_state(state);
        assert!(s.validate().is_empty(), "{state:?}");
        let cue = s.views[0]
            .recovery_cue
            .clone()
            .expect("narrowed view has a cue");
        assert!(
            cues.insert(cue),
            "recovery cue for {state:?} collapsed with another state"
        );
    }
}

#[test]
fn corrupted_effective_claim_is_rejected() {
    let mut s = set();
    s.apply_managed_state(ManagedStateClass::ManagedBlocked);
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
fn forged_raw_number_in_cost_line_is_rejected() {
    let mut s = set();
    s.views[0].scope_cost_truths[0].direct.carries_raw_number = true;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "scope_cost_truth.direct"),
        "expected a raw-number violation, got {violations:?}"
    );
}

#[test]
fn swapping_direct_and_inherited_basis_is_rejected() {
    let mut s = set();
    s.views[0].scope_cost_truths[0].direct.attribution_basis = AttributionBasis::Inherited;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "scope_cost_truth.direct"),
        "expected an attribution-basis violation, got {violations:?}"
    );
}

#[test]
fn drifting_the_inherited_parent_is_rejected() {
    let mut s = set();
    // Point an inherited line at a parent that is not in the recomputed chain.
    s.views[0].scope_cost_truths[0].inherited.inherited_from = Some(ScopeOwner::Tenant);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "scope_cost_truth.inherited"),
        "expected an inherited-parent violation, got {violations:?}"
    );
}

#[test]
fn collapsing_a_view_to_one_scope_is_rejected() {
    let mut s = set();
    s.views[0].scope_cost_truths.truncate(1);
    s.inspection = ChargebackScopeInspection::derive(
        &s.switcher,
        &s.views,
        &s.surface_bindings,
        s.active_managed_state,
    );
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "view.scope_cost_truths"),
        "expected a collapsed-scope violation, got {violations:?}"
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
    s.inspection = ChargebackScopeInspection::derive(
        &s.switcher,
        &s.views,
        &s.surface_bindings,
        s.active_managed_state,
    );
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
    s.views[1].service_family = s.views[0].service_family;
    s.inspection = ChargebackScopeInspection::derive(
        &s.switcher,
        &s.views,
        &s.surface_bindings,
        s.active_managed_state,
    );
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
        .retain(|b| b.surface != ChargebackScopeSurface::ReleaseCenter);
    s.inspection = ChargebackScopeInspection::derive(
        &s.switcher,
        &s.views,
        &s.surface_bindings,
        s.active_managed_state,
    );
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "surface_bindings"),
        "expected a surface-binding violation, got {violations:?}"
    );
}

#[test]
fn switcher_dropping_a_preservation_guarantee_is_rejected() {
    let mut s = set();
    s.switcher.preserves_inherited_direct_separation = false;
    s.inspection = ChargebackScopeInspection::derive(
        &s.switcher,
        &s.views,
        &s.surface_bindings,
        s.active_managed_state,
    );
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "switcher.preserves_inherited_direct_separation"),
        "expected a switcher-preservation violation, got {violations:?}"
    );
}

#[test]
fn switching_to_an_unavailable_scope_is_rejected() {
    let mut s = set();
    // ByokExternal is not an offered switcher scope.
    s.switch_scope(ScopeOwner::ByokExternal);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "switcher.active_scope"),
        "expected an active-scope violation, got {violations:?}"
    );
}

#[test]
fn view_for_family_resolves() {
    let s = set();
    let v = s
        .view_for_family(ServiceFamily::AiGatewayFamily)
        .expect("ai gateway view present");
    assert_eq!(v.meter_unit, MeterUnit::Tokens);
    assert!(v.backs_full_managed_claim());
    let truth = v
        .scope_truth(ScopeOwner::Team)
        .expect("ai gateway carries a team scope");
    assert_eq!(
        truth.inherited.inherited_from,
        Some(ScopeOwner::Organization)
    );
}
