use super::*;

#[test]
fn seeded_snapshot_covers_every_family() {
    let snapshot = seeded_records_policy_governance_snapshot();
    assert!(snapshot.validate().is_empty(), "{:?}", snapshot.validate());
    for family in PolicyGovernanceFamily::ALL {
        assert!(
            snapshot.row_for_family(family).is_some(),
            "family {family:?} must be covered"
        );
    }
}

#[test]
fn remembered_decision_rows_preserve_all_required_triggers() {
    let snapshot = seeded_records_policy_governance_snapshot();
    for row in &snapshot.rows {
        if row.remembered_decision_revalidation_supported {
            assert_eq!(row.required_reapproval_trigger_tokens.len(), 4);
        }
        assert!(!row.chronology_refs.is_empty());
        assert!(!row.evidence_refs.is_empty());
    }
}

#[test]
fn missing_reapproval_trigger_emits_a_defect() {
    let mut snapshot = seeded_records_policy_governance_snapshot();
    snapshot.rows[0].required_reapproval_trigger_tokens.pop();
    assert!(snapshot.validate().iter().any(|defect| {
        defect.defect_kind == PolicyGovernanceSnapshotDefectKind::ReapprovalTriggerMissing
    }));
}
