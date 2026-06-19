//! Inline unit tests for the typed M5 public-contract matrix.

use super::*;

fn matrix() -> M5PublicContractMatrix {
    current_m5_public_contract_matrix().expect("checked-in matrix parses into the model")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let m = matrix();
    assert_eq!(m.schema_version, M5_PUBLIC_CONTRACT_SCHEMA_VERSION);
    assert_eq!(m.record_kind, M5_PUBLIC_CONTRACT_RECORD_KIND);
    let violations = m.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn closed_vocabularies_round_trip() {
    let m = matrix();
    assert_eq!(m.contract_forms, ContractForm::ALL.to_vec());
    assert_eq!(m.maturity_lanes, MaturityLane::ALL.to_vec());
    assert_eq!(
        m.publication_artifact_kinds,
        PublicationArtifactKind::ALL.to_vec()
    );
    assert_eq!(m.gap_reasons, GapReason::ALL.to_vec());
    assert_eq!(m.remediation_actions, RemediationAction::ALL.to_vec());
}

#[test]
fn every_gap_reason_has_a_stop_rule() {
    let m = matrix();
    for reason in GapReason::ALL {
        assert!(
            m.stop_rules
                .iter()
                .any(|rule| rule.trigger_reason == reason),
            "gap reason {} must have a stop rule",
            reason.as_str()
        );
    }
}

#[test]
fn wit_and_openapi_forms_are_inventoried() {
    let m = matrix();
    assert!(
        !m.rows_for_form(ContractForm::WitWorldPackage).is_empty(),
        "the matrix must inventory at least one WIT world package family"
    );
    assert!(
        !m.rows_for_form(ContractForm::OpenapiFamily).is_empty(),
        "the matrix must inventory at least one OpenAPI family"
    );
}

#[test]
fn matrix_narrows_a_family_missing_a_required_form() {
    let m = matrix();
    let narrowed = m
        .rows
        .iter()
        .find(|row| row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the matrix must narrow at least one family put forward at the cutline"
    );
    let row = narrowed.unwrap();
    assert!(!row.active_gap_reasons.is_empty());
    assert_eq!(row.row_state, RowState::Narrowed);
}

#[test]
fn published_families_publish_at_their_claim_label() {
    let m = matrix();
    for row in &m.rows {
        if row.row_state == RowState::Published {
            assert_eq!(row.published_label, row.claim_label, "{}", row.family_id);
            assert!(row.active_gap_reasons.is_empty(), "{}", row.family_id);
        }
    }
}

#[test]
fn computed_summary_matches_recorded_summary() {
    let m = matrix();
    assert_eq!(m.summary, m.computed_summary());
}

#[test]
fn computed_promotion_matches_recorded_promotion() {
    let m = matrix();
    assert_eq!(m.promotion.decision, m.computed_promotion_decision());
    assert_eq!(
        m.promotion.blocking_rule_ids,
        m.computed_blocking_rule_ids()
    );
    assert_eq!(
        m.promotion.blocking_claim_ids,
        m.computed_blocking_family_ids()
    );
}

#[test]
fn export_projection_has_one_row_per_family() {
    let m = matrix();
    let projection = m.support_export_projection();
    assert_eq!(projection.rows.len(), m.rows.len());
    assert_eq!(projection.matrix_id, m.matrix_id);
}

#[test]
fn published_row_with_unpublished_requirement_fails() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.row_state == RowState::Published)
        .expect("matrix has a published row");
    for cell in &mut row.publication_requirements {
        if cell.artifact_kind == PublicationArtifactKind::MarkdownSummary {
            cell.state = PublicationState::Missing;
            cell.refs.clear();
        }
    }
    m.summary = m.computed_summary();
    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5PublicContractViolation::PublishedWithUnpublishedRequirement { .. }
        )),
        "a published row may not leave a required form unpublished"
    );
}

#[test]
fn narrowed_row_without_reason_fails() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.row_state == RowState::Narrowed)
        .expect("matrix has a narrowed row");
    row.active_gap_reasons.clear();
    m.summary = m.computed_summary();
    assert!(
        m.validate()
            .iter()
            .any(|v| matches!(v, M5PublicContractViolation::GapReasonsMismatch { .. })),
        "a narrowed row that drops its gap reasons must fail"
    );
}

#[test]
fn required_but_not_applicable_fails() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.row_state == RowState::Published)
        .expect("matrix has a published row");
    for cell in &mut row.publication_requirements {
        if cell.artifact_kind == PublicationArtifactKind::ValidatorSuite {
            cell.required = true;
            cell.state = PublicationState::NotApplicable;
            cell.refs.clear();
        }
    }
    m.summary = m.computed_summary();
    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5PublicContractViolation::RequiredButNotApplicable { .. }
        )),
        "a required form may not be marked not_applicable"
    );
}

#[test]
fn published_wider_than_claim_fails() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|row| row.claim_label == StableClaimLevel::Beta)
        .expect("matrix has a beta-claim row");
    row.published_label = StableClaimLevel::Stable;
    m.summary = m.computed_summary();
    assert!(
        m.validate()
            .iter()
            .any(|v| matches!(v, M5PublicContractViolation::PublishedWiderThanClaim { .. })),
        "a row may not publish wider than its claim"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut m = matrix();
    m.promotion.decision = PromotionDecision::Proceed;
    assert!(
        m.validate().iter().any(|v| matches!(
            v,
            M5PublicContractViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn duplicate_family_id_fails() {
    let mut m = matrix();
    let first = m.rows[0].family_id.clone();
    m.rows[1].family_id = first;
    m.summary = m.computed_summary();
    assert!(
        m.validate()
            .iter()
            .any(|v| matches!(v, M5PublicContractViolation::DuplicateFamilyId { .. })),
        "two rows may not share a family id"
    );
}
