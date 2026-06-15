//! Tests for the frozen metering-degradation rule set.

use super::*;

fn set() -> MeteringDegradationRuleSet {
    canonical_stable_metering_degradation_rule_set()
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
    let stable = current_stable_metering_degradation_rule_set()
        .expect("checked-in set parses and validates");
    assert_eq!(
        stable,
        set(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn matrix_covers_every_family_and_trigger_exactly_once() {
    let s = set();
    assert!(s.inspection.matrix_complete);
    assert_eq!(
        s.inspection.rule_count,
        ServiceFamily::ALL.len() * DegradationTrigger::ALL.len()
    );
    assert_eq!(
        s.inspection.service_families_covered,
        ServiceFamily::ALL.len()
    );
    assert_eq!(
        s.inspection.degradation_triggers_covered,
        DegradationTrigger::ALL.len()
    );
    for family in ServiceFamily::ALL {
        for trigger in DegradationTrigger::ALL {
            assert!(
                s.rule_for(family, trigger).is_some(),
                "missing rule for {family:?} / {trigger:?}"
            );
        }
    }
}

#[test]
fn every_rule_keeps_a_local_safe_promise_and_never_blocks_local_core() {
    let s = set();
    assert!(s.inspection.all_rules_local_safe_backed);
    assert!(s.inspection.never_blocks_local_core);
    for r in &s.rules {
        assert!(
            !r.local_safe_promise.is_empty(),
            "rule {} lost its promise",
            r.rule_id
        );
        assert!(r.local_safe_promise.iter().all(|p| !p.trim().is_empty()));
        // A metering degradation never collapses the local core.
        assert!(
            !r.narrows_to_local_safe_only,
            "rule {} collapsed the local core",
            r.rule_id
        );
        assert_ne!(r.effective_marketed_claim, MarketedClaim::LocalSafeOnly);
    }
}

#[test]
fn disposition_matches_the_frozen_fail_posture() {
    let s = set();
    for r in &s.rules {
        assert_eq!(
            r.disposition,
            DegradationDisposition::for_posture(r.fail_posture),
            "rule {} disposition drifted from its fail posture",
            r.rule_id
        );
    }
    // Fail-open and fail-closed rules partition the matrix.
    assert_eq!(
        s.inspection.fail_open_rule_count + s.inspection.fail_closed_rule_count,
        s.rules.len()
    );
}

#[test]
fn fail_open_lanes_never_gate_and_fail_closed_lanes_gate_one_action() {
    let s = set();
    for r in &s.rules {
        if r.is_fail_open() {
            assert!(
                r.gated_optional_action.is_none(),
                "fail-open rule {} gated an action",
                r.rule_id
            );
            assert!(r.blocking_reason.is_none());
        } else {
            // Fail-closed gates exactly one named optional action with a reason.
            let action = r
                .gated_optional_action
                .as_ref()
                .unwrap_or_else(|| panic!("fail-closed rule {} must gate an action", r.rule_id));
            assert!(!action.trim().is_empty());
            let reason = r
                .blocking_reason
                .as_ref()
                .unwrap_or_else(|| panic!("fail-closed rule {} must name a reason", r.rule_id));
            assert!(!reason.trim().is_empty());
        }
    }
    // The companion relay and managed workspace fail closed; the others fail open.
    assert_eq!(
        s.inspection.fail_closed_rule_count,
        2 * DegradationTrigger::ALL.len()
    );
}

#[test]
fn metering_stale_and_service_unreachable_name_family_promise_and_actions() {
    let s = set();
    for trigger in [
        DegradationTrigger::MeteringStale,
        DegradationTrigger::ServiceUnreachable,
    ] {
        for family in ServiceFamily::ALL {
            let r = s
                .rule_for(family, trigger)
                .unwrap_or_else(|| panic!("missing rule for {family:?} / {trigger:?}"));
            assert_eq!(r.service_family, family, "rule names its family");
            assert!(!r.local_safe_promise.is_empty(), "rule names the promise");
            assert_eq!(r.retry_action.kind, DegradationActionKind::Retry);
            assert!(!r.retry_action.label.trim().is_empty());
            assert_eq!(r.details_action.kind, DegradationActionKind::Details);
            assert!(!r.details_action.label.trim().is_empty());
        }
    }
}

#[test]
fn no_number_crosses_the_boundary_bare() {
    let s = set();
    assert!(s.inspection.value_never_bare);
    for r in &s.rules {
        assert!(
            !r.as_of.trim().is_empty(),
            "rule {} lost its as-of",
            r.rule_id
        );
        match r.degradation_trigger {
            DegradationTrigger::MeteringStale => {
                assert_eq!(
                    r.value_disclosure,
                    DegradationValueDisclosure::LabeledStaleBoundToUnitAsOfScope
                );
                assert_eq!(r.freshness, SnapshotFreshness::FreshnessStale);
            }
            DegradationTrigger::ServiceUnreachable | DegradationTrigger::RatingPathUnavailable => {
                assert_eq!(
                    r.value_disclosure,
                    DegradationValueDisclosure::SuppressedNoManagedNumber
                );
                assert_eq!(r.freshness, SnapshotFreshness::FreshnessUnknown);
            }
        }
    }
}

#[test]
fn a_degradation_is_distinct_from_account_loss_states() {
    let s = set();
    assert!(s.inspection.account_state_distinctions_complete);
    for r in &s.rules {
        assert!(r.not_an_account_error);
        for state in [
            ManagedStateClass::SeatRemoved,
            ManagedStateClass::OrgSwitched,
            ManagedStateClass::GracePeriod,
            ManagedStateClass::ReauthRequired,
        ] {
            assert!(
                r.distinct_from_account_states.contains(&state),
                "rule {} must stay distinct from {state:?}",
                r.rule_id
            );
        }
        // Only the stale trigger borrows the meter-stale managed state.
        match r.degradation_trigger {
            DegradationTrigger::MeteringStale => {
                assert_eq!(r.related_managed_state, Some(ManagedStateClass::MeterStale));
            }
            _ => assert_eq!(r.related_managed_state, None),
        }
    }
}

#[test]
fn every_degradation_narrows_the_marketed_claim_to_managed_narrowed() {
    let s = set();
    for r in &s.rules {
        assert_eq!(r.declared_marketed_claim, MarketedClaim::ManagedFull);
        assert_eq!(r.effective_marketed_claim, MarketedClaim::ManagedNarrowed);
    }
}

#[test]
fn rules_project_their_control_plane_lane() {
    let s = set();
    let violations = s.cross_check_against_control_plane();
    assert!(
        violations.is_empty(),
        "rules drifted from the control-plane matrix: {violations:?}"
    );
}

#[test]
fn every_consumer_surface_is_bound() {
    let s = set();
    for surface in ConsumerSurface::ALL {
        let binding = s
            .surface_bindings
            .iter()
            .find(|b| b.consumer_surface == surface)
            .unwrap_or_else(|| panic!("missing surface {surface:?}"));
        assert!(binding.projects_effective_claim);
        assert!(binding.renders_local_safe_promise);
        assert!(binding.names_blocking_reason);
        assert!(!binding.bound_rule_ids.is_empty());
    }
}

#[test]
fn rule_for_and_rules_for_family_resolve() {
    let s = set();
    let r = s
        .rule_for(
            ServiceFamily::AiGatewayFamily,
            DegradationTrigger::MeteringStale,
        )
        .expect("ai gateway stale rule present");
    assert!(r.is_fail_open());
    assert_eq!(
        s.rules_for_family(ServiceFamily::CollaborationRelayFamily)
            .len(),
        DegradationTrigger::ALL.len()
    );
    // The relay fails closed, so each of its rules gates one action.
    for r in s.rules_for_family(ServiceFamily::CollaborationRelayFamily) {
        assert!(r.gates_optional_action());
    }
}

#[test]
fn forged_disposition_is_rejected() {
    let mut s = set();
    // Forge a fail-open disposition onto a fail-closed lane.
    let idx = s
        .rules
        .iter()
        .position(|r| r.gates_optional_action())
        .expect("a fail-closed rule is present");
    s.rules[idx].disposition = DegradationDisposition::FailOpenLocalSafePath;
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "rule.disposition"),
        "expected a disposition violation, got {violations:?}"
    );
}

#[test]
fn fail_open_rule_that_gates_is_rejected() {
    let mut s = set();
    let idx = s
        .rules
        .iter()
        .position(|r| r.is_fail_open())
        .expect("a fail-open rule is present");
    s.rules[idx].gated_optional_action = Some("forged gate".to_owned());
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "rule.gated_optional_action"),
        "expected a gated-action violation, got {violations:?}"
    );
}

#[test]
fn gated_rule_without_reason_is_rejected() {
    let mut s = set();
    let idx = s
        .rules
        .iter()
        .position(|r| r.gates_optional_action())
        .expect("a fail-closed rule is present");
    s.rules[idx].blocking_reason = None;
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "rule.blocking_reason"),
        "expected a blocking-reason violation, got {violations:?}"
    );
}

#[test]
fn collapsing_the_local_core_is_rejected() {
    let mut s = set();
    s.rules[0].narrows_to_local_safe_only = true;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "rule.narrows_to_local_safe_only"),
        "expected a local-core violation, got {violations:?}"
    );
}

#[test]
fn emptying_a_local_safe_promise_is_rejected() {
    let mut s = set();
    s.rules[0].local_safe_promise.clear();
    s.inspection = MeteringDegradationInspection::derive(&s.rules, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "rule.local_safe_promise"),
        "expected a local-safe-promise violation, got {violations:?}"
    );
}

#[test]
fn dropping_an_account_state_distinction_is_rejected() {
    let mut s = set();
    s.rules[0]
        .distinct_from_account_states
        .retain(|state| *state != ManagedStateClass::SeatRemoved);
    s.inspection = MeteringDegradationInspection::derive(&s.rules, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "rule.distinct_from_account_states"),
        "expected an account-state distinction violation, got {violations:?}"
    );
}

#[test]
fn forged_marketed_claim_is_rejected() {
    let mut s = set();
    s.rules[0].effective_marketed_claim = MarketedClaim::ManagedFull;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "rule.effective_marketed_claim"),
        "expected an effective-claim violation, got {violations:?}"
    );
}

#[test]
fn dropping_a_family_trigger_pair_is_rejected() {
    let mut s = set();
    s.rules.remove(0);
    s.inspection = MeteringDegradationInspection::derive(&s.rules, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "rules"),
        "expected a rules violation, got {violations:?}"
    );
}

#[test]
fn missing_surface_is_rejected() {
    let mut s = set();
    s.surface_bindings
        .retain(|b| b.consumer_surface != ConsumerSurface::ClaimPublicTruthAutomation);
    s.inspection = MeteringDegradationInspection::derive(&s.rules, &s.surface_bindings);
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
    let parsed: MeteringDegradationRuleSet =
        serde_json::from_str(&json).expect("set round-trips through JSON");
    assert_eq!(parsed, s);
}
