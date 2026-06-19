//! Inline unit tests for the typed M5 contract catalog.

use super::*;

fn catalog() -> M5ContractCatalog {
    current_m5_contract_catalog().expect("checked-in catalog parses")
}

#[test]
fn checked_in_catalog_parses_and_validates() {
    let c = catalog();
    assert_eq!(c.schema_version, M5_CONTRACT_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_CONTRACT_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_CONTRACT_CATALOG_ID);
    let violations = c.validate();
    assert!(
        violations.is_empty(),
        "catalog must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_recomputes_from_families() {
    let c = catalog();
    assert_eq!(c.summary, c.computed_summary());
    assert_eq!(c.summary.total_families, c.families.len());
    assert!(c.summary.total_families > 0);
    assert_eq!(
        c.summary.families_stable_label + c.summary.families_beta_label,
        c.families.len(),
        "every family publishes at the stable or beta label"
    );
}

#[test]
fn every_family_has_a_partial_sample_and_field_notes_coverage() {
    let c = catalog();
    assert_eq!(c.summary.families_with_partial_sample, c.families.len());
    for fam in &c.families {
        assert!(
            fam.has_partial_sample(),
            "{} must publish a partial sample",
            fam.family_id
        );
        assert_eq!(fam.sample_classes, SampleClass::ALL.to_vec());
        assert_eq!(fam.sample_count, SampleClass::ALL.len());
    }
}

#[test]
fn identity_kind_agrees_with_contract_form() {
    let c = catalog();
    for fam in &c.families {
        assert_eq!(
            fam.contract_identity.identity_kind,
            fam.contract_form.identity_kind(),
            "{} identity kind must agree with its form",
            fam.family_id
        );
    }
    // The WIT family is the only one without a JSON Schema validation ref.
    let wit = c
        .family("extension_host_wit_world")
        .expect("wit family present");
    assert_eq!(wit.contract_identity.identity_kind, IdentityKind::WitWorld);
    assert!(wit.json_schema_validation_ref.is_none());
}

#[test]
fn resolves_contract_identity_and_lifecycle_label() {
    let c = catalog();
    let (id, label) = c
        .resolve_contract("command_descriptors")
        .expect("command_descriptors resolves");
    assert!(id.ends_with("command_descriptors.schema.json"));
    assert_eq!(label, LifecycleLabel::Stable);
    assert!(c.resolve_contract("not_a_family").is_none());
}

#[test]
fn narrowed_family_inherits_the_matrix_label() {
    let c = catalog();
    let te = c
        .family("task_event_envelope")
        .expect("task_event_envelope present");
    assert!(te.narrowed, "task_event_envelope narrows in the matrix");
    assert_eq!(te.lifecycle_label, LifecycleLabel::Beta);
    assert!(!te.publishes_stable());
    assert!(!te.active_gap_reasons.is_empty());
    assert_eq!(c.narrowed_families().len(), c.summary.families_narrowed);
}

#[test]
fn offline_bundle_is_runtime_free() {
    let c = catalog();
    assert!(c.offline_bundle.mirrorable);
    assert!(!c.offline_bundle.requires_runtime_service);
    assert!(!c.offline_bundle.bundle_members.is_empty());
    for fam in &c.families {
        assert!(fam.offline_parity.mirror_inspectable);
        assert!(!fam.offline_parity.requires_runtime_service);
    }
}

#[test]
fn support_export_projection_covers_every_family() {
    let c = catalog();
    let projection = c.support_export_projection();
    assert_eq!(projection.catalog_id, c.catalog_id);
    assert_eq!(projection.rows.len(), c.families.len());
    for row in &projection.rows {
        let fam = c
            .family(&row.family_id)
            .expect("row family is in the model");
        assert_eq!(row.lifecycle_label, fam.lifecycle_label);
        assert_eq!(
            row.schema_or_spec_id,
            fam.contract_identity.schema_or_spec_id
        );
    }
}

#[test]
fn duplicate_family_id_is_rejected() {
    let mut c = catalog();
    let dup = c.families[0].clone();
    c.families.push(dup);
    c.summary = c.computed_summary();
    let violations = c.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "families.duplicate_family_id"),
        "duplicate family id must be rejected: {violations:#?}"
    );
}

#[test]
fn missing_partial_sample_is_rejected() {
    let mut c = catalog();
    c.families[0].sample_classes = vec![SampleClass::Nominal];
    c.families[0].sample_count = 1;
    c.summary = c.computed_summary();
    let violations = c.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "families.missing_partial_sample"),
        "missing partial sample must be rejected: {violations:#?}"
    );
}

#[test]
fn summary_drift_is_rejected() {
    let mut c = catalog();
    c.summary.total_families += 1;
    assert!(c
        .validate()
        .iter()
        .any(|v| v.check_id == "summary.count_mismatch"));
}
