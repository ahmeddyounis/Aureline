//! Protected tests binding the typed stable claim matrix to the checked-in
//! artifact and the frozen CI validation capture.
//!
//! The positive case is the frozen, checked-in matrix; the capture cross-check
//! proves the typed model and the Python gate agree on the promotion verdict and
//! summary; the negative cases mutate a parsed copy to prove that a row which
//! fails to narrow, or a promotion verdict that disagrees with the firing stop
//! rules, fails validation.

use aureline_release::stable_claim_matrix::{
    current_stable_claim_matrix, DowngradeReason, PromotionDecision, QualificationState,
    StableClaimLevel, StableClaimMatrix, StableClaimMatrixViolation,
    STABLE_CLAIM_MATRIX_RECORD_KIND, STABLE_CLAIM_MATRIX_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_claim_matrix_validation_capture.json"
));

fn matrix() -> StableClaimMatrix {
    current_stable_claim_matrix().expect("checked-in stable claim matrix parses into the model")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let matrix = matrix();
    assert_eq!(matrix.schema_version, STABLE_CLAIM_MATRIX_SCHEMA_VERSION);
    assert_eq!(matrix.record_kind, STABLE_CLAIM_MATRIX_RECORD_KIND);
    let violations = matrix.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let matrix = matrix();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(matrix.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_rows"].as_u64().unwrap() as usize,
        matrix.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_holding_stable"].as_u64().unwrap() as usize,
        matrix.rows_holding_stable().len(),
        "capture held-stable count must match the model"
    );
    assert_eq!(
        summary["stop_rules_firing"].as_u64().unwrap() as usize,
        matrix
            .stop_rules
            .iter()
            .filter(|rule| matrix.stop_rule_fires(rule))
            .count(),
        "capture firing-stop-rule count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        matrix.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(
        matrix.promotion.decision,
        matrix.computed_promotion_decision()
    );

    // Every negative drill recorded in the capture must have passed.
    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
}

#[test]
fn unqualified_row_that_does_not_narrow_fails() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.holds_stable())
        .expect("matrix has a held stable row");
    row.qualification_state = QualificationState::NotQualified;
    row.active_downgrade_reasons
        .push(DowngradeReason::QualificationEvidenceMissing);
    row.effective_level = row.claimed_level;
    matrix.summary = matrix.computed_summary();
    matrix.promotion.decision = matrix.computed_promotion_decision();
    matrix.promotion.blocking_rule_ids = matrix.computed_blocking_rule_ids();
    matrix.promotion.blocking_claim_ids = matrix.computed_blocking_claim_ids();

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            StableClaimMatrixViolation::EffectiveLevelNotNarrowed { .. }
        )),
        "a surface lacking stable qualification must narrow below the cutline"
    );
}

#[test]
fn promotion_decision_mismatch_fails() {
    let mut matrix = matrix();
    matrix.promotion.decision = PromotionDecision::Hold;

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            StableClaimMatrixViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion decision must agree with computed stop rules"
    );
}

#[test]
fn stripping_a_stop_rule_for_an_active_reason_fails() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.holds_stable())
        .expect("matrix has a held stable row");
    row.qualification_state = QualificationState::NotQualified;
    row.effective_level = StableClaimLevel::Beta;
    row.active_downgrade_reasons
        .push(DowngradeReason::QualificationEvidenceMissing);
    matrix
        .stop_rules
        .retain(|rule| rule.trigger_reason != DowngradeReason::QualificationEvidenceMissing);
    matrix.summary = matrix.computed_summary();
    matrix.promotion.decision = matrix.computed_promotion_decision();
    matrix.promotion.blocking_rule_ids = matrix.computed_blocking_rule_ids();
    matrix.promotion.blocking_claim_ids = matrix.computed_blocking_claim_ids();

    assert!(
        matrix.validate().contains(
            &StableClaimMatrixViolation::DowngradeReasonWithoutStopRule {
                reason: DowngradeReason::QualificationEvidenceMissing,
            }
        ),
        "every downgrade reason must keep a stop rule watching for it"
    );
}
