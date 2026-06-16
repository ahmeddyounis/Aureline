//! Inline unit tests binding the typed register to the checked-in artifact and exercising
//! scan/surface parity, per-axis narrowing, the no-mask invariant, and the promotion verdict
//! against mutated copies.

use super::*;

fn register() -> CriticalUpstreamHealthRegister {
    current_m5_critical_upstream_health().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_CRITICAL_UPSTREAM_HEALTH_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_CRITICAL_UPSTREAM_HEALTH_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.records.is_empty());
}

#[test]
fn every_upstream_kind_is_exercised() {
    let r = register();
    for kind in UpstreamKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "upstream kind {} must have at least one record",
            kind.as_str()
        );
    }
}

#[test]
fn every_record_declares_all_control_dimensions() {
    let r = register();
    for rec in &r.records {
        for dimension in ControlDimension::ALL {
            let count = rec
                .controls
                .iter()
                .filter(|c| c.dimension == dimension)
                .count();
            assert_eq!(
                count,
                1,
                "record {} must declare control {} exactly once",
                rec.record_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn states_are_per_axis_not_one_global_flag() {
    let r = register();
    let states: BTreeSet<HealthState> = r.records.iter().map(|x| x.health_state).collect();
    assert!(states.contains(&HealthState::Cleared));
    assert!(
        states.len() >= 6,
        "expected several distinct health states, not one global flag"
    );
    let reasons: BTreeSet<HealthReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(!reasons.is_empty(), "narrowed records must name reasons");
}

#[test]
fn scan_and_surface_agree_on_every_record() {
    let r = register();
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface postures must agree",
            rec.record_id
        );
        // A green surface may never sit over a gap.
        assert_eq!(
            rec.surface_posture,
            rec.computed_posture(),
            "record {} surface posture must reflect its gaps",
            rec.record_id
        );
    }
}

#[test]
fn maintainer_and_ownership_gaps_are_first_class_not_masked() {
    let r = register();
    // An abandoned upstream still narrows on the maintainer axis even with a recorded plan and a
    // raised escalation, proving a green upstream card can't mask the collapse.
    let abandoned = r.records.iter().any(|rec| {
        rec.maintainer.rating == MaintainerRating::Abandoned
            && rec.health_state == HealthState::NarrowedMaintainer
    });
    assert!(
        abandoned,
        "expected an abandoned upstream narrowing on the maintainer axis"
    );
    // An upstream left unowned because it is "just infrastructure" still narrows.
    let unowned = r.records.iter().any(|rec| rec.is_unowned());
    assert!(
        unowned,
        "expected an unowned upstream narrowing on the ownership axis"
    );
    for rec in &r.records {
        if rec.maintainer.rating.is_abandoned() {
            assert!(rec.has_active_reason(HealthReason::MaintainerAbandoned));
        }
        if rec.is_unowned() {
            assert!(rec.has_active_reason(HealthReason::UpstreamUnowned));
        }
    }
}

#[test]
fn red_risk_upstreams_require_a_contingency_and_escalation() {
    let r = register();
    // Red-risk upstreams require a recorded sponsor/fork/replace plan and a raised escalation; a
    // pending one narrows on the ownership axis.
    let red: Vec<&UpstreamHealthRecord> = r.records.iter().filter(|x| x.is_red_risk()).collect();
    assert!(!red.is_empty(), "expected at least one red-risk upstream");
    for rec in red {
        assert!(rec.requires_contingency());
        assert!(rec.requires_escalation());
        if rec.contingency_missing() || rec.escalation_missing() {
            assert_eq!(rec.health_state, HealthState::NarrowedOwnership);
        }
    }
}

#[test]
fn no_record_publishes_wider_than_it_declares() {
    let r = register();
    for rec in &r.records {
        assert!(
            rec.effective_label.rank() <= rec.declared_label.rank(),
            "record {} effective label is wider than declared",
            rec.record_id
        );
        if rec.health_state.is_narrowed() {
            assert!(
                !rec.effective_label.is_at_or_above_cutline(),
                "narrowed record {} must drop below the cutline",
                rec.record_id
            );
        }
    }
}

#[test]
fn summary_and_parity_match_records() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(r.scan_surface_parity, r.computed_scan_surface_parity());
    assert_eq!(
        r.summary.records_cleared + r.summary.records_narrowed + r.summary.state_withdrawn,
        r.records.len()
    );
}

#[test]
fn reuse_projection_covers_every_record() {
    let r = register();
    let projection = r.reuse_projection();
    assert_eq!(projection.len(), r.records.len());
    for projected in &projection {
        assert!(
            !projected.surfaces.is_empty(),
            "projected record {} must carry reuse surfaces",
            projected.record_id
        );
    }
}

#[test]
fn health_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_decision());
    let blocking = r.computed_blocking_record_ids();
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
        assert!(rec.health_state.is_narrowed());
    }
    // An inherited (below-cutline) or waived narrowing is gated upstream.
    for rec in &r.records {
        if rec.health_state.is_narrowed()
            && (!rec.declares_at_or_above_cutline() || rec.is_waived())
        {
            assert!(
                !blocking.contains(&rec.record_id),
                "inherited/waived narrowing on {} must not hold promotion",
                rec.record_id
            );
        }
    }
}

#[test]
fn validate_flags_a_cleared_record_with_a_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    rec.active_reasons.push(HealthReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::ClearedWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_hidden_ownership_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    // Open an ownership gap without narrowing on it: the gap must surface its reason.
    rec.ownership.ownership_state = OwnershipState::Unowned;
    rec.ownership.owner_ref = String::new();
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: HealthReason::UpstreamUnowned,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_green_surface_over_a_gapped_scan() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.health_state.is_narrowed())
        .expect("a narrowed record exists");
    // Pretend the governance surface is green over a scan that found gaps.
    rec.surface_posture = Posture::Clear;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::ScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_narrowed_record_above_the_cutline() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.health_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_red_risk_upstream_without_a_contingency() {
    let mut r = register();
    // Find a red-risk record with a recorded plan and drop the plan to pending without narrowing.
    if let Some(rec) = r
        .records
        .iter_mut()
        .find(|x| x.is_red_risk() && x.contingency.plan_state == ContingencyState::Recorded)
    {
        rec.contingency.plan_state = ContingencyState::Pending;
        rec.contingency.disposition = ContingencyDisposition::None;
        assert!(r.validate().iter().any(|x| matches!(
            x,
            RegisterViolation::GapWithoutReason {
                reason: HealthReason::ContingencyPlanMissing,
                ..
            } | RegisterViolation::ControlStateInconsistent { .. }
        )));
    }
}

#[test]
fn validate_flags_a_proceed_verdict_while_a_rule_fires() {
    let mut r = register();
    if r.computed_decision() == PublicationDecision::Hold {
        r.publication.decision = PublicationDecision::Proceed;
        assert!(r
            .validate()
            .iter()
            .any(|x| matches!(x, RegisterViolation::PublicationDecisionInconsistent)));
    }
}
