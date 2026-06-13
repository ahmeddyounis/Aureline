use super::*;

#[test]
fn current_export_is_clean() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    assert!(export.is_clean(), "{export:?}");
    assert!(export.raw_private_material_excluded);
}

#[test]
fn projection_covers_every_family() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    assert_eq!(
        export.projection_rows.len(),
        export.hold_retention_packet.rows.len()
    );
}

#[test]
fn every_referenced_exception_resolves() {
    let export = M5RecordsPolicyGovernanceSupportExport::current();
    let known = export.exception_expiry_packet.exception_ids();
    for exception_id in export.hold_retention_packet.referenced_exception_ids() {
        assert!(known.contains(&exception_id), "unresolved: {exception_id}");
    }
}

#[test]
fn unresolved_exception_ref_is_flagged() {
    let mut hold_retention_packet =
        aureline_records::m5_records_policy::seeded_m5_records_policy_packet();
    hold_retention_packet.rows[0]
        .exception_refs
        .push("m5-exception:does-not-exist".to_owned());
    let exception_expiry_packet =
        aureline_policy::m5_exception_expiry::seeded_m5_exception_expiry_packet();

    let violations = M5RecordsPolicyGovernanceSupportExport::cross_validate(
        &hold_retention_packet,
        &exception_expiry_packet,
    );
    assert!(violations.iter().any(|violation| matches!(
        violation,
        M5RecordsPolicyGovernanceViolation::ExceptionRefUnresolved { .. }
    )));
}
