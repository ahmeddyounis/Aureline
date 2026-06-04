//! Protected tests for the stable mixed-version compatibility and skew matrix.

use aureline_release::mixed_version_compatibility_and_skew_governance::{
    current_mixed_version_compatibility_and_skew_governance, BoundaryFamily, GovernanceState,
    GovernanceViolation, MixedVersionCompatibilityAndSkewGovernance, UnsupportedFlowLabel,
    MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_RECORD_KIND,
    MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

fn matrix() -> MixedVersionCompatibilityAndSkewGovernance {
    current_mixed_version_compatibility_and_skew_governance()
        .expect("checked-in mixed-version governance matrix parses")
}

#[test]
fn checked_in_matrix_parses_and_validates() {
    let matrix = matrix();
    assert_eq!(
        matrix.schema_version,
        MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(
        matrix.record_kind,
        MIXED_VERSION_COMPATIBILITY_AND_SKEW_GOVERNANCE_RECORD_KIND
    );
    let violations = matrix.validate();
    assert!(
        violations.is_empty(),
        "checked-in matrix must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_required_boundary_family() {
    let matrix = matrix();
    for family in BoundaryFamily::ALL {
        assert!(
            !matrix.rows_for_family(family).is_empty(),
            "required boundary family {} must be covered",
            family.as_str()
        );
    }
}

#[test]
fn user_visible_labels_cover_required_flow_outcomes() {
    let matrix = matrix();
    let labels: Vec<UnsupportedFlowLabel> = matrix
        .boundary_rows
        .iter()
        .map(|row| row.unsupported_behavior.label)
        .collect();
    for label in [
        UnsupportedFlowLabel::CoordinatedUpgradeRequired,
        UnsupportedFlowLabel::FileOnlyFallback,
        UnsupportedFlowLabel::ReadOnlyDowngrade,
        UnsupportedFlowLabel::ExtensionQuarantined,
        UnsupportedFlowLabel::AttributedCompatibilityError,
    ] {
        assert!(
            labels.contains(&label),
            "matrix must expose controlled unsupported label {label:?}"
        );
    }
}

#[test]
fn stale_skew_row_narrows_and_holds_publication() {
    let matrix = matrix();
    let provider = matrix
        .boundary_rows
        .iter()
        .find(|row| row.boundary_family == BoundaryFamily::ProviderLinkedPacket)
        .expect("provider adapter boundary exists");

    assert_eq!(
        provider.governance_state,
        GovernanceState::NarrowedStaleSkewEvidence
    );
    assert_eq!(provider.claim_label, StableClaimLevel::Stable);
    assert_eq!(provider.effective_label, StableClaimLevel::Beta);
    assert_eq!(
        provider.skew_window_drill_packet.slo_state,
        FreshnessSloState::Breached
    );
    assert_eq!(matrix.publication.decision, PromotionDecision::Hold);
    assert_eq!(
        matrix.publication.decision,
        matrix.computed_publication_decision()
    );
}

#[test]
fn support_export_projection_uses_matrix_truth() {
    let matrix = matrix();
    let projection = matrix.support_export_projection();
    assert_eq!(projection.rows.len(), matrix.boundary_rows.len());

    let remote = projection
        .rows
        .iter()
        .find(|row| row.boundary_family == BoundaryFamily::DesktopCliAndRemoteAgent)
        .expect("remote row projects");
    assert_eq!(
        remote.unsupported_label,
        UnsupportedFlowLabel::FileOnlyFallback
    );
    assert!(remote
        .negotiated_fields
        .contains(&"agent_version".to_string()));
    assert!(remote
        .fail_closed_behavior
        .contains("Refuse mutating attach"));
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut matrix = matrix();
    let row = matrix
        .boundary_rows
        .iter_mut()
        .find(|row| row.boundary_family == BoundaryFamily::ProviderLinkedPacket)
        .expect("provider adapter boundary exists");
    row.effective_label = StableClaimLevel::Stable;
    matrix.summary = matrix.computed_summary();

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            GovernanceViolation::NarrowingRowNotNarrowed { .. }
        )),
        "stale skew evidence must narrow the boundary below Stable"
    );
}

#[test]
fn held_row_on_stale_proof_fails() {
    let mut matrix = matrix();
    let row = matrix
        .boundary_rows
        .iter_mut()
        .find(|row| row.boundary_family == BoundaryFamily::DesktopCliAndRemoteAgent)
        .expect("remote boundary exists");
    row.skew_window_drill_packet.slo_state = FreshnessSloState::Breached;
    matrix.summary = matrix.computed_summary();

    assert!(
        matrix
            .validate()
            .iter()
            .any(|violation| matches!(violation, GovernanceViolation::HeldOnStaleProof { .. })),
        "a stable row cannot ride a stale skew-window proof"
    );
}

#[test]
fn publication_proceed_while_rule_fires_fails() {
    let mut matrix = matrix();
    matrix.publication.decision = PromotionDecision::Proceed;

    assert!(
        matrix.validate().iter().any(|violation| matches!(
            violation,
            GovernanceViolation::PublicationDecisionInconsistent
        )),
        "publication must hold while stale skew or rollback rules fire"
    );
}
