use super::*;

fn register() -> GeneratedArtifactLineageRegister {
    current_generated_artifact_lineage_register().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION);
    assert_eq!(r.record_kind, GENERATED_ARTIFACT_LINEAGE_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.rows.is_empty());
}

#[test]
fn covers_every_generator_kind() {
    let r = register();
    for kind in GeneratorKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "generator kind {} must have at least one surface",
            kind.as_str()
        );
    }
}

#[test]
fn every_surface_covers_every_dimension() {
    let r = register();
    for row in &r.rows {
        for dimension in LineageDimension::ALL {
            assert!(
                row.cell(dimension).is_some(),
                "surface {} must cover dimension {}",
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
            "{declared} has no covering release-blocking surface"
        );
    }
}

#[test]
fn every_held_surface_discloses_generation_and_verified_automation() {
    let r = register();
    for row in r.rows_published_stable() {
        assert!(
            row.lineage.generated_labeled,
            "held surface {} must label its artifact as generated",
            row.entry_id
        );
        assert!(
            row.downgrade_automation.state.holds() && row.downgrade_automation.rollback_verified,
            "held surface {} must carry defined+verified downgrade automation",
            row.entry_id
        );
    }
}

#[test]
fn summary_counts_match_surfaces() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_traced + r.summary.entries_narrowed,
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
fn validate_flags_a_held_surface_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held surface exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::RollbackPlanUnverified);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, LineageRegisterViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_held_surface_without_disclosure() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held surface exists");
    row.lineage.generated_labeled = false;
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, LineageRegisterViolation::HeldWithoutDisclosure { .. })));
}

#[test]
fn validate_flags_a_missing_dimension_cell() {
    let mut r = register();
    r.rows[0]
        .scorecard
        .retain(|cell| cell.dimension != LineageDimension::Reproducibility);
    assert!(r.validate().iter().any(|v| matches!(
        v,
        LineageRegisterViolation::LineageIncompleteCoverage { .. }
    )));
}

#[test]
fn validate_flags_a_held_surface_without_downgrade_automation() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held surface exists");
    row.downgrade_automation.rollback_verified = false;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        LineageRegisterViolation::HeldWithoutDowngradeAutomation { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        LineageRegisterViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn export_projection_mirrors_surfaces() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.lineage.trust_tier, proj.trust_tier);
        assert_eq!(row.lineage.generated_labeled, proj.generated_labeled);
        assert_eq!(row.downgrade_automation.state, proj.automation_state);
    }
}
