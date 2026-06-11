use super::*;

fn register() -> MaintenanceTruthRegister {
    current_maintenance_truth_register().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, MAINTENANCE_TRUTH_SCHEMA_VERSION);
    assert_eq!(r.record_kind, MAINTENANCE_TRUTH_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.rows.is_empty());
}

#[test]
fn covers_every_lane_kind() {
    let r = register();
    for kind in LaneKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "lane kind {} must have at least one lane",
            kind.as_str()
        );
    }
}

#[test]
fn every_lane_covers_every_dimension() {
    let r = register();
    for row in &r.rows {
        for dimension in MaintenanceDimension::ALL {
            assert!(
                row.cell(dimension).is_some(),
                "lane {} must cover dimension {}",
                row.entry_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let r = register();
    assert!(!r.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &r.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking lane"
        );
    }
}

#[test]
fn every_held_lane_discloses_truth_and_verified_automation() {
    let r = register();
    for row in r.rows_published_stable() {
        assert!(
            row.disclosure.truth_disclosed,
            "held lane {} must disclose its Help/About truth",
            row.entry_id
        );
        assert!(
            row.downgrade_automation.state.holds() && row.downgrade_automation.rollback_verified,
            "held lane {} must carry defined+verified downgrade automation",
            row.entry_id
        );
    }
}

#[test]
fn summary_counts_match_lanes() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_certified + r.summary.entries_narrowed,
        r.rows.len()
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
fn validate_flags_a_held_lane_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held lane exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::RollbackPlanUnverified);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, MaintenanceTruthViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_held_lane_without_truth_disclosure() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held lane exists");
    row.disclosure.truth_disclosed = false;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        MaintenanceTruthViolation::HeldWithoutTruthDisclosure { .. }
    )));
}

#[test]
fn validate_flags_a_missing_dimension_cell() {
    let mut r = register();
    r.rows[0]
        .scorecard
        .retain(|cell| cell.dimension != MaintenanceDimension::DocsTruth);
    assert!(r.validate().iter().any(|v| matches!(
        v,
        MaintenanceTruthViolation::MaintenanceIncompleteCoverage { .. }
    )));
}

#[test]
fn validate_flags_a_held_lane_without_downgrade_automation() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held lane exists");
    row.downgrade_automation.rollback_verified = false;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        MaintenanceTruthViolation::HeldWithoutDowngradeAutomation { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        MaintenanceTruthViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn export_projection_mirrors_lanes() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.disclosure.trust_tier, proj.trust_tier);
        assert_eq!(row.disclosure.truth_disclosed, proj.truth_disclosed);
        assert_eq!(row.downgrade_automation.state, proj.automation_state);
    }
}
