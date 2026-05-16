//! Unit and fixture coverage for registry catalog descriptors.

use serde::Deserialize;

use super::{
    evaluate_catalog_descriptor, project_catalog_descriptor_support_export,
    validate_catalog_descriptor_record, validate_catalog_descriptor_support_export_record,
    CatalogDescriptorDecisionClass, CatalogDescriptorInput, CatalogDescriptorReasonClass,
    CatalogDisclosureClass, CatalogMirrorabilityClass, CatalogSupportExplanationClass,
    CATALOG_DESCRIPTOR_RECORD_KIND, CATALOG_DESCRIPTOR_SCHEMA_VERSION,
    CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;

#[derive(Debug, Deserialize)]
struct CatalogFixture {
    input: CatalogDescriptorInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: CatalogDescriptorDecisionClass,
    expected_reason_class: CatalogDescriptorReasonClass,
    expected_support_explanation_class: CatalogSupportExplanationClass,
    expected_mirrorable_catalog_metadata: bool,
    expected_revocation_ready: bool,
    expected_publisher_continuity_ready: bool,
}

fn load_fixture(name: &str) -> CatalogFixture {
    let raw = match name {
        "mirrorable_catalog_approved" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/mirrorable_catalog_approved.json"
        ),
        "staged_pending_moderation" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/staged_pending_moderation.json"
        ),
        "limited_compatibility_catalog" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/limited_compatibility_catalog.json"
        ),
        "revoked_catalog_refused" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/revoked_catalog_refused.json"
        ),
        "quarantined_publisher_refused" => include_str!(
            "../../../../fixtures/extensions/m3/registry_moderation/quarantined_publisher_refused.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let record = evaluate_catalog_descriptor(fixture.input);
    assert_eq!(record.record_kind, CATALOG_DESCRIPTOR_RECORD_KIND);
    assert_eq!(
        record.catalog_descriptor_schema_version,
        CATALOG_DESCRIPTOR_SCHEMA_VERSION
    );
    assert_eq!(record.redaction_class, RedactionClass::MetadataSafeDefault);
    assert_eq!(
        record.decision_class, fixture.meta.expected_decision_class,
        "decision mismatch for {name}"
    );
    assert_eq!(
        record.reason_class, fixture.meta.expected_reason_class,
        "reason mismatch for {name}"
    );
    assert_eq!(
        record.support_explanation_class, fixture.meta.expected_support_explanation_class,
        "support explanation mismatch for {name}"
    );
    assert_eq!(
        record.mirrorable_catalog_metadata, fixture.meta.expected_mirrorable_catalog_metadata,
        "mirrorable metadata mismatch for {name}"
    );
    assert_eq!(
        record.revocation_ready, fixture.meta.expected_revocation_ready,
        "revocation-ready mismatch for {name}"
    );
    assert_eq!(
        record.publisher_continuity_ready, fixture.meta.expected_publisher_continuity_ready,
        "publisher-continuity mismatch for {name}"
    );

    let findings = validate_catalog_descriptor_record(&record);
    assert!(
        findings.is_empty(),
        "fixture {name} produced catalog validation findings: {findings:?}"
    );

    let export = project_catalog_descriptor_support_export(
        &record,
        &format!("catalog_descriptor_support_export:{}", record.descriptor_id),
    );
    assert_eq!(
        export.record_kind,
        CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.descriptor_ref, record.descriptor_id);
    assert_eq!(export.decision_class, record.decision_class);
    assert_eq!(export.reason_class, record.reason_class);
    assert_eq!(
        export.blocks_install_or_update,
        record.decision_class == CatalogDescriptorDecisionClass::Refused
    );
    assert_eq!(export.redaction_class, RedactionClass::MetadataSafeDefault);
    let export_findings = validate_catalog_descriptor_support_export_record(&export);
    assert!(
        export_findings.is_empty(),
        "fixture {name} produced support-export validation findings: {export_findings:?}"
    );
}

#[test]
fn mirrorable_catalog_approved_round_trips() {
    run_fixture("mirrorable_catalog_approved");
}

#[test]
fn staged_pending_moderation_round_trips() {
    run_fixture("staged_pending_moderation");
}

#[test]
fn limited_compatibility_catalog_round_trips() {
    run_fixture("limited_compatibility_catalog");
}

#[test]
fn revoked_catalog_refused_round_trips() {
    run_fixture("revoked_catalog_refused");
}

#[test]
fn quarantined_publisher_refused_round_trips() {
    run_fixture("quarantined_publisher_refused");
}

#[test]
fn missing_required_disclosure_blocks_catalog_mutation() {
    let mut fixture = load_fixture("mirrorable_catalog_approved");
    fixture
        .input
        .rendered_disclosures
        .retain(|disclosure| !matches!(disclosure, CatalogDisclosureClass::MirrorMetadata));

    let record = evaluate_catalog_descriptor(fixture.input);
    assert_eq!(
        record.decision_class,
        CatalogDescriptorDecisionClass::Refused
    );
    assert_eq!(
        record.reason_class,
        CatalogDescriptorReasonClass::RefusedRequiredDisclosureMissing
    );
    assert!(validate_catalog_descriptor_record(&record)
        .iter()
        .any(|finding| finding.check_id == "catalog_descriptor.required_disclosure_missing"));
}

#[test]
fn mirror_digest_or_signature_block_precedes_staging() {
    let mut fixture = load_fixture("staged_pending_moderation");
    fixture.input.mirror.mirrorability_class =
        CatalogMirrorabilityClass::MirrorBlockedDigestOrSignature;

    let record = evaluate_catalog_descriptor(fixture.input);
    assert_eq!(
        record.decision_class,
        CatalogDescriptorDecisionClass::Refused
    );
    assert_eq!(
        record.reason_class,
        CatalogDescriptorReasonClass::RefusedMirrorBlocked
    );
    assert_eq!(
        record.support_explanation_class,
        CatalogSupportExplanationClass::MirrorBlocked
    );
}
