use super::*;

fn register() -> BoundarySkewInspectorRegister {
    current_m5_boundary_skew_inspectors().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        SHIP_M5_BOUNDARY_SKEW_INSPECTORS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, SHIP_M5_BOUNDARY_SKEW_INSPECTORS_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.inspectors.is_empty());
}

#[test]
fn covers_every_boundary_kind() {
    let r = register();
    for kind in BoundaryKind::ALL {
        assert!(
            !r.inspectors_for_kind(kind).is_empty(),
            "boundary kind {} must have at least one inspector",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_downgrade_subject() {
    let r = register();
    for subject in DowngradeSubject::ALL {
        assert!(
            !r.inspectors_for_subject(subject).is_empty(),
            "downgrade subject {} must have at least one inspector",
            subject.as_str()
        );
    }
}

#[test]
fn gated_action_matches_boundary_kind() {
    let r = register();
    for row in &r.inspectors {
        assert_eq!(
            row.gated_action,
            row.boundary_kind.gated_action(),
            "inspector {} must gate the action of its boundary kind",
            row.entry_id
        );
    }
}

#[test]
fn gate_posture_tracks_the_verdict() {
    let r = register();
    for row in &r.inspectors {
        assert_eq!(
            row.action_allowed(),
            row.verdict.is_inside_window(),
            "inspector {} gate posture must track its verdict",
            row.entry_id
        );
    }
}

#[test]
fn fail_closed_verdicts_carry_an_upgrade_order_guide() {
    let r = register();
    for row in &r.inspectors {
        if row.verdict.requires_upgrade_guide() {
            assert!(
                row.upgrade_order_guide.lead_side.requires_upgrade()
                    && !row.upgrade_order_guide.steps.is_empty(),
                "inspector {} must carry an upgrade-order guide for verdict {}",
                row.entry_id,
                row.verdict.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_boundary() {
    let r = register();
    assert!(!r.release_blocking_boundary_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_inspectors()
        .iter()
        .map(|row| row.boundary_ref.as_str())
        .collect();
    for declared in &r.release_blocking_boundary_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking inspector"
        );
    }
}

#[test]
fn summary_counts_match_inspectors() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.inspectors_publishing_stable + r.summary.inspectors_narrowed,
        r.inspectors.len()
    );
}

#[test]
fn promotion_decision_matches_computed() {
    let r = register();
    assert_eq!(r.promotion.decision, r.computed_promotion_decision());
    assert_eq!(
        r.promotion.blocking_rule_ids,
        r.computed_blocking_rule_ids()
    );
    assert_eq!(
        r.promotion.blocking_claim_ids,
        r.computed_blocking_entry_ids()
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let r = register();
    let covered: BTreeSet<NarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn validate_flags_a_held_inspector_with_active_gap() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held inspector exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::SkewWindowExceeded);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, BoundarySkewInspectorViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_an_incoherent_gate_posture() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| !row.verdict.is_inside_window())
        .expect("a fail-closed inspector exists");
    row.gate_posture = GatePosture::Allow;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        BoundarySkewInspectorViolation::GatePostureIncoherent { .. }
    )));
}

#[test]
fn validate_flags_a_fail_closed_verdict_without_a_guide() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| row.verdict.requires_upgrade_guide())
        .expect("a skew-recovery inspector exists");
    row.upgrade_order_guide.lead_side = UpgradeLeadSide::NoneRequired;
    row.upgrade_order_guide.steps.clear();
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        BoundarySkewInspectorViolation::UpgradeGuideMissing { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        BoundarySkewInspectorViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn validate_flags_a_limited_inspector_without_caveat() {
    let mut r = register();
    let row = r
        .inspectors
        .iter_mut()
        .find(|row| row.inspector_state == InspectorState::Limited)
        .expect("a limited inspector exists");
    row.compatibility_caveats.clear();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        BoundarySkewInspectorViolation::LimitedWithoutCaveat { .. }
    )));
}

#[test]
fn export_projection_mirrors_inspectors() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.inspectors.len());
    for (row, proj) in r.inspectors.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.verdict, proj.verdict);
        assert_eq!(row.gate_posture, proj.gate_posture);
        assert_eq!(row.upgrade_order_guide.lead_side, proj.upgrade_lead_side);
    }
}
