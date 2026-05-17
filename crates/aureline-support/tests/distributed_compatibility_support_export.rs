//! Protected tests for generated distributed compatibility support exports.

use std::collections::BTreeSet;

use aureline_support::distributed_compatibility::{
    current_distributed_compatibility_support_export,
    DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_RECORD_KIND,
    DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_SCHEMA_VERSION,
};

#[test]
fn generated_support_export_validates_and_covers_required_families() {
    let export = current_distributed_compatibility_support_export().expect("support export parses");
    let violations = export.validate();

    assert_eq!(violations, Vec::new());
    assert_eq!(
        export.schema_version,
        DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_SCHEMA_VERSION
    );
    assert_eq!(
        export.record_kind,
        DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.redaction_class, "metadata_safe_default");
    assert_eq!(export.support_rows.len(), 7);
    assert_eq!(export.harness_summary.total_cases, 8);
    assert_eq!(export.harness_summary.supported_case_count, 4);
    assert_eq!(export.harness_summary.unsupported_case_count, 4);

    let families = export
        .manifest_families
        .iter()
        .map(|family| family.manifest_family.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        families,
        BTreeSet::from(["client_extension", "client_helper", "provider", "schema"])
    );
}

#[test]
fn support_rows_quote_manifest_and_release_packet_refs() {
    let export = current_distributed_compatibility_support_export().expect("support export parses");

    for row in &export.support_rows {
        assert_eq!(row.release_packet_ref, export.release_packet_ref);
        assert!(row
            .manifest_ref
            .starts_with("artifacts/compat/m3/distributed_manifests/"));
        assert!(row.compatibility_row_ref.starts_with("compat_row:"));
        assert!(row.current_skew_case_ref.starts_with("skew_case:"));
        assert!(!row.unsupported_case_refs.is_empty());
        assert!(!row.repair_hints.is_empty());
    }
}

#[test]
fn validation_rejects_private_material_and_missing_unsupported_cases() {
    let mut export =
        current_distributed_compatibility_support_export().expect("support export parses");
    export.raw_private_material_excluded = false;
    export.support_rows[0].unsupported_case_refs.clear();

    let violations = export.validate();
    let check_ids = violations
        .iter()
        .map(|violation| violation.check_id.as_str())
        .collect::<BTreeSet<_>>();

    assert!(check_ids.contains("support_export.raw_private_material_excluded"));
    assert!(check_ids.contains("support_row.unsupported_case_refs_empty"));
}
