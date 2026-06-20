//! Inline unit tests for the typed M5 JSON Schema catalog.

use super::*;

fn catalog() -> M5JsonSchemaCatalog {
    current_m5_json_schema_catalog().expect("checked-in catalog parses")
}

#[test]
fn checked_in_catalog_parses_and_validates() {
    let c = catalog();
    assert_eq!(c.schema_version, M5_JSON_SCHEMA_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_JSON_SCHEMA_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_JSON_SCHEMA_CATALOG_ID);
    let violations = c.validate();
    assert!(
        violations.is_empty(),
        "catalog must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_recomputes_from_packages() {
    let c = catalog();
    assert_eq!(c.summary, c.computed_summary());
    assert_eq!(c.summary.total_packages, c.packages.len());
    assert!(c.summary.total_packages > 0);
    assert_eq!(
        c.summary.stable_label_packages + c.summary.beta_label_packages,
        c.packages.len(),
        "every package publishes at the stable or beta label"
    );
}

#[test]
fn every_package_preserves_unknown_fields() {
    let c = catalog();
    assert_eq!(c.summary.preserve_unknown_packages, c.packages.len());
    for pkg in &c.packages {
        assert!(
            pkg.preserves_unknown_fields(),
            "{} must preserve unknown fields",
            pkg.family_id
        );
    }
}

#[test]
fn every_package_declares_a_version_field_and_identity() {
    let c = catalog();
    for pkg in &c.packages {
        assert!(!pkg.version_field_names.is_empty());
        assert!(pkg.version_field_names.contains(&pkg.primary_version_field));
        assert!(!pkg.primary_identifier_field.is_empty());
        assert_eq!(pkg.record_kind_field, "record_kind");
        assert!(!pkg.field_contract.migration_note_hooks.is_empty());
    }
}

#[test]
fn resolves_schema_id_and_lifecycle_label() {
    let c = catalog();
    let (schema_id, label) = c
        .resolve_schema_label("command_descriptors")
        .expect("command_descriptors resolves");
    assert!(schema_id.ends_with("command_descriptors.schema.json"));
    assert_eq!(label, LifecycleLabel::Stable);
    assert!(c.resolve_schema_label("not_a_family").is_none());
}

#[test]
fn stable_and_beta_packages_partition_the_set() {
    let c = catalog();
    let stable = c.packages_for_label(LifecycleLabel::Stable).len();
    let beta = c.packages_for_label(LifecycleLabel::Beta).len();
    assert_eq!(stable, c.summary.stable_label_packages);
    assert_eq!(beta, c.summary.beta_label_packages);
    assert_eq!(c.stable_packages().len(), stable);
}

#[test]
fn offline_bundle_is_runtime_free() {
    let c = catalog();
    assert!(c.offline_bundle.mirrorable);
    assert!(!c.offline_bundle.requires_runtime_service);
    assert!(!c.offline_bundle.bundle_members.is_empty());
}

#[test]
fn duplicate_package_id_is_rejected() {
    let mut c = catalog();
    let dup = c.packages[0].clone();
    c.packages.push(dup);
    c.summary = c.computed_summary();
    let violations = c.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "packages.duplicate_package_id"),
        "duplicate package id must be rejected: {violations:#?}"
    );
}

#[test]
fn summary_drift_is_rejected() {
    let mut c = catalog();
    c.summary.total_packages += 1;
    assert!(c
        .validate()
        .iter()
        .any(|v| v.check_id == "summary.count_mismatch"));
}
