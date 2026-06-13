use super::*;

#[test]
fn current_export_is_metadata_only_and_valid() {
    let export = RecordsExportDeleteGovernanceSupportExport::current();
    assert!(export.raw_private_material_excluded);
    assert!(export.violations.is_empty(), "{:?}", export.violations);
    assert_eq!(
        export.projection_rows.len(),
        export.lifecycle_packet.family_links.len()
    );
}

#[test]
fn support_projection_rows_preserve_manifests_and_receipts_or_blockers() {
    let export = RecordsExportDeleteGovernanceSupportExport::current();
    for row in &export.projection_rows {
        assert!(!row.manifest_bundle_id.trim().is_empty());
        assert!(
            row.destruction_receipt_id.is_some() || row.delete_blocker_outcome.is_some(),
            "{row:?}"
        );
    }
}
