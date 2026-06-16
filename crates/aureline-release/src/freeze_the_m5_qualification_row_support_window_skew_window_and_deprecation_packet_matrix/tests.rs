use super::*;

fn matrix() -> QualificationSkewMatrix {
    current_m5_qualification_and_skew_matrix().expect("matrix parses")
}

#[test]
fn embedded_matrix_parses_and_validates() {
    let m = matrix();
    assert_eq!(
        m.schema_version,
        FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(
        m.record_kind,
        FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_RECORD_KIND
    );
    assert_eq!(m.validate(), Vec::new());
    assert!(!m.rows.is_empty());
}

#[test]
fn covers_every_family_kind() {
    let m = matrix();
    for kind in FamilyKind::ALL {
        assert!(
            !m.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn every_row_covers_every_dimension() {
    let m = matrix();
    for row in &m.rows {
        for dimension in QualificationDimension::ALL {
            assert!(
                row.cell(dimension).is_some(),
                "row {} must cover dimension {}",
                row.entry_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let m = matrix();
    assert!(!m.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = m
        .release_blocking_rows()
        .iter()
        .map(|row| row.family_ref.as_str())
        .collect();
    for declared in &m.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn summary_counts_match_rows() {
    let m = matrix();
    assert_eq!(m.summary, m.computed_summary());
    assert_eq!(
        m.summary.rows_qualified + m.summary.rows_narrowed,
        m.rows.len()
    );
}

#[test]
fn promotion_decision_matches_computed() {
    let m = matrix();
    assert_eq!(m.promotion.decision, m.computed_promotion_decision());
    assert_eq!(
        m.promotion.blocking_rule_ids,
        m.computed_blocking_rule_ids()
    );
    assert_eq!(
        m.promotion.blocking_claim_ids,
        m.computed_blocking_entry_ids()
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let m = matrix();
    let covered: BTreeSet<NarrowingReason> = m
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn validate_flags_a_held_row_with_active_gap() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held row exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::SkewWindowExceeded);
    m.summary = m.computed_summary();
    assert!(m.validate().iter().any(|v| matches!(
        v,
        QualificationSkewMatrixViolation::HeldWithActiveGap { .. }
    )));
}

#[test]
fn validate_flags_a_missing_dimension_cell() {
    let mut m = matrix();
    m.rows[0]
        .qualification_row
        .retain(|cell| cell.dimension != QualificationDimension::ClientScope);
    assert!(m.validate().iter().any(|v| matches!(
        v,
        QualificationSkewMatrixViolation::QualificationRowIncompleteCoverage { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut m = matrix();
    m.promotion.decision = PromotionDecision::Proceed;
    assert!(m.validate().iter().any(|v| matches!(
        v,
        QualificationSkewMatrixViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn validate_flags_a_limited_row_without_caveat() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.row_state == RowState::Limited)
        .expect("a limited row exists");
    row.compatibility_caveats.clear();
    assert!(m.validate().iter().any(|v| matches!(
        v,
        QualificationSkewMatrixViolation::LimitedWithoutCaveat { .. }
    )));
}

#[test]
fn export_projection_mirrors_rows() {
    let m = matrix();
    let projection = m.support_export_projection();
    assert_eq!(projection.rows.len(), m.rows.len());
    for (row, proj) in m.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.skew_window.skew_window_class, proj.skew_window_class);
        assert_eq!(row.deprecation_packet.status, proj.deprecation_status);
    }
}
