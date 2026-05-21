//! Protected tests binding the typed stable qualification matrix to the
//! checked-in artifact and the frozen CI validation capture.
//!
//! The positive case is the frozen, checked-in matrix; the capture cross-check
//! proves the typed model and the Python gate agree on the promotion verdict,
//! summary, and mixed-version posture; the negative cases mutate a parsed copy
//! to prove that an incomplete mixed-version section that does not narrow, a
//! cross-binary lane missing its mixed-version section, a narrowed lane that
//! inherits a Stable mixed-version claim, and a promotion verdict that disagrees
//! with the firing rules all fail validation.

use aureline_release::stable_claim_matrix::PromotionDecision;
use aureline_release::stable_qualification_matrix::{
    current_stable_qualification_matrix, BoundaryFamily, MixedVersionPosture, MixedVersionSection,
    QualificationRowScope, StableQualificationMatrix, StableQualificationMatrixViolation,
    STABLE_QUALIFICATION_MATRIX_RECORD_KIND, STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stable_qualification_matrix_validation_capture.json"
));

fn matrix() -> StableQualificationMatrix {
    current_stable_qualification_matrix()
        .expect("checked-in stable qualification matrix parses into the model")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let matrix = matrix();
    assert_eq!(
        matrix.schema_version,
        STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(matrix.record_kind, STABLE_QUALIFICATION_MATRIX_RECORD_KIND);
    let violations = matrix.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_enumerated_boundary_family_is_covered() {
    let matrix = matrix();
    let covered: std::collections::BTreeSet<BoundaryFamily> = matrix
        .cross_binary_rows()
        .iter()
        .filter_map(|row| row.mixed_version.as_ref().map(|mv| mv.boundary_family))
        .collect();
    for family in BoundaryFamily::ALL {
        assert!(
            covered.contains(&family),
            "boundary family {} must be covered by a cross-binary row",
            family.as_str()
        );
    }
}

#[test]
fn accessibility_lane_carries_no_mixed_version_section() {
    let matrix = matrix();
    let accessibility = matrix
        .rows
        .iter()
        .find(|row| row.row_scope == QualificationRowScope::Accessibility)
        .expect("matrix has an accessibility lane");
    assert!(accessibility.mixed_version.is_none());
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
        summary["cross_binary_rows"].as_u64().unwrap() as usize,
        matrix.cross_binary_rows().len(),
        "capture cross-binary count must match the model"
    );
    assert_eq!(
        summary["coordinated_upgrade_only_rows"].as_u64().unwrap() as usize,
        matrix.computed_summary().coordinated_upgrade_only_rows,
        "capture coordinated-upgrade-only count must match the model"
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
fn incomplete_mixed_version_that_does_not_narrow_fails() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| {
            row.mixed_version
                .as_ref()
                .is_some_and(|mv| !mv.publishes_complete_negotiation_data())
        })
        .expect("matrix has an incomplete mixed-version row");
    // Pretend the boundary holds a Stable mixed-version posture on incomplete data.
    if let Some(mv) = row.mixed_version.as_mut() {
        mv.effective_posture = MixedVersionPosture::RollingSkewSupported;
    }

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::IncompleteMixedVersionNotCoordinatedOnly { .. }
        )),
        "a boundary lacking complete data must be coordinated-upgrade-only"
    );
}

#[test]
fn cross_binary_lane_without_a_mixed_version_section_fails() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.requires_mixed_version())
        .expect("matrix has a cross-binary lane");
    row.mixed_version = None;

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::MissingMixedVersionSection { .. }
        )),
        "a cross-binary lane must carry a mixed-version section"
    );
}

#[test]
fn narrowed_lane_inheriting_a_stable_mixed_version_claim_fails() {
    let mut matrix = matrix();
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| {
            !row.holds_stable()
                && row
                    .mixed_version
                    .as_ref()
                    .is_some_and(MixedVersionSection::publishes_complete_negotiation_data)
        })
        .expect("matrix has a narrowed lane with a complete mixed-version section");
    if let Some(mv) = row.mixed_version.as_mut() {
        mv.claimed_posture = MixedVersionPosture::BoundedSkewSupported;
        mv.effective_posture = MixedVersionPosture::BoundedSkewSupported;
    }

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::StableMixedVersionOnNarrowedRow { .. }
        )),
        "a narrowed lane may not inherit a Stable mixed-version claim"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut matrix = matrix();
    matrix.promotion.decision = PromotionDecision::Proceed;

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking downgrade rule fires"
    );
}
