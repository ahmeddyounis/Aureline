//! Unit and fixture coverage for extension lifecycle metadata packets.

use serde::Deserialize;

use super::{
    current_extension_lifecycle_metadata_packet, evaluate_lifecycle_metadata_packet,
    project_lifecycle_metadata_support_export, validate_lifecycle_metadata_packet,
    validate_lifecycle_metadata_support_export, LifecycleDeprecationPostureClass,
    LifecycleMetadataDecisionClass, LifecycleMetadataPacketInput, LifecycleMetadataReasonClass,
    LIFECYCLE_METADATA_PACKET_RECORD_KIND, LIFECYCLE_METADATA_SCHEMA_VERSION,
    LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;

#[derive(Debug, Deserialize)]
struct LifecycleFixture {
    input: LifecycleMetadataPacketInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: LifecycleMetadataDecisionClass,
    expected_reason_class: LifecycleMetadataReasonClass,
    expected_row_count: u32,
    expected_deprecated_row_count: u32,
    expected_beta_or_stable_row_count: u32,
}

fn load_fixture(name: &str) -> LifecycleFixture {
    let raw = match name {
        "ready_beta_surfaces_with_deprecation" => include_str!(
            "../../../../fixtures/extensions/m3/lifecycle_metadata/ready_beta_surfaces_with_deprecation.json"
        ),
        "refused_deprecated_missing_replacement" => include_str!(
            "../../../../fixtures/extensions/m3/lifecycle_metadata/refused_deprecated_missing_replacement.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let record = evaluate_lifecycle_metadata_packet(fixture.input);
    assert_eq!(record.record_kind, LIFECYCLE_METADATA_PACKET_RECORD_KIND);
    assert_eq!(
        record.lifecycle_metadata_schema_version,
        LIFECYCLE_METADATA_SCHEMA_VERSION
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
    assert_eq!(record.row_count, fixture.meta.expected_row_count);
    assert_eq!(
        record.deprecated_row_count,
        fixture.meta.expected_deprecated_row_count
    );
    assert_eq!(
        record.beta_or_stable_row_count,
        fixture.meta.expected_beta_or_stable_row_count
    );

    let findings = validate_lifecycle_metadata_packet(&record);
    if record.decision_class == LifecycleMetadataDecisionClass::RefusedIncompleteMetadata {
        assert!(
            !findings.is_empty(),
            "refused fixture {name} must surface validation findings"
        );
    } else {
        assert!(
            findings.is_empty(),
            "fixture {name} produced lifecycle metadata findings: {findings:?}"
        );
    }

    let export = project_lifecycle_metadata_support_export(
        &record,
        &format!(
            "extension_lifecycle_metadata_support_export:{}",
            record.packet_id
        ),
    );
    assert_eq!(
        export.record_kind,
        LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.packet_ref, record.packet_id);
    assert_eq!(export.decision_class, record.decision_class);
    assert_eq!(
        export.blocks_publication,
        record.decision_class == LifecycleMetadataDecisionClass::RefusedIncompleteMetadata
    );
    assert_eq!(
        export.deprecation_disclosure_required,
        record.decision_class == LifecycleMetadataDecisionClass::DeprecatedMigrationRequired
    );
    assert_eq!(export.redaction_class, RedactionClass::MetadataSafeDefault);
    let export_findings = validate_lifecycle_metadata_support_export(&export);
    assert!(
        export_findings.is_empty(),
        "fixture {name} produced support-export findings: {export_findings:?}"
    );
}

#[test]
fn ready_lifecycle_metadata_round_trips() {
    run_fixture("ready_beta_surfaces_with_deprecation");
}

#[test]
fn refused_deprecated_missing_replacement_round_trips() {
    run_fixture("refused_deprecated_missing_replacement");
}

#[test]
fn checked_lifecycle_packet_is_valid() {
    let record = current_extension_lifecycle_metadata_packet()
        .expect("checked lifecycle metadata packet must deserialize");
    let findings = validate_lifecycle_metadata_packet(&record);
    assert!(
        findings.is_empty(),
        "checked lifecycle metadata packet produced findings: {findings:?}"
    );
    assert_eq!(
        record.decision_class,
        LifecycleMetadataDecisionClass::DeprecatedMigrationRequired
    );
    assert!(
        record
            .rows
            .iter()
            .any(|row| row.deprecation.deprecation_posture_class
                == LifecycleDeprecationPostureClass::DeprecatedWithReplacement),
        "checked packet must include a deprecated row with replacement guidance"
    );
}
