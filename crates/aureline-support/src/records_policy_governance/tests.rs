use super::*;
use aureline_records::records_policy_simulation_matrix::{
    ProofFreshnessClass, RecordsPolicyGapReason, RecordsPolicyQualificationClass,
};

#[test]
fn seeded_support_export_validates_and_preserves_narrowed_rows() {
    let export = RecordsPolicyGovernanceSupportExport::current().expect("support export loads");
    assert_eq!(
        export.record_kind,
        RECORDS_POLICY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.validate().is_empty(), "{:?}", export.validate());
    assert_eq!(export.projection_rows.len(), export.matrix.rows.len());
    assert!(
        !export.narrowed_row_ids.is_empty(),
        "the checked-in matrix should expose at least one narrowed row"
    );
}

#[test]
fn missing_policy_snapshot_row_emits_issue() {
    let mut export = RecordsPolicyGovernanceSupportExport::current().expect("support export loads");
    export.policy_snapshot.rows.pop();
    let issues = export.validate();
    assert!(issues
        .iter()
        .any(|issue| issue.issue_code == "missing_policy_snapshot_row"));
}

#[test]
fn release_blocking_stale_row_requires_hold() {
    let mut export = RecordsPolicyGovernanceSupportExport::current().expect("support export loads");
    let row = export
        .matrix
        .rows
        .iter_mut()
        .find(|row| row.release_blocking)
        .expect("seeded matrix has a release-blocking row");
    row.proof_freshness = ProofFreshnessClass::Stale;
    row.published_qualification = RecordsPolicyQualificationClass::NeedsReview;
    row.active_gap_reasons = vec![RecordsPolicyGapReason::ProofStale];
    export.matrix.publication.decision = RecordsPolicyPublicationDecision::Proceed;
    let issues = export.validate();
    assert!(issues
        .iter()
        .any(|issue| issue.issue_code == "release_hold_mismatch"));
}
