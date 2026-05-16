//! Unit and fixture coverage for the extension publication pipeline.

use serde::Deserialize;

use super::{
    evaluate_extension_publication_pipeline, project_extension_publication_support_export,
    validate_extension_publication_pipeline_record,
    validate_extension_publication_support_export_record, ExtensionPublicationPipelineInput,
    PublicationDecisionClass, PublicationReasonClass, PublicationSignatureClass,
    PublicationTransactionWriteClass, EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND,
    EXTENSION_PUBLICATION_SCHEMA_VERSION, EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;

#[derive(Debug, Deserialize)]
struct PublicationFixture {
    input: ExtensionPublicationPipelineInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: PublicationDecisionClass,
    expected_reason_class: PublicationReasonClass,
    expected_promotion_step_count: u32,
    expected_required_evidence_ref_count: u32,
    expected_approver_ref_count: u32,
    expected_preserves_prior_installable_artifact: bool,
    expected_transactional_catalog_update: bool,
}

fn load_fixture(name: &str) -> PublicationFixture {
    let raw = match name {
        "ready_signed_provenance_rollback_safe" => include_str!(
            "../../../../fixtures/extensions/m3/publication_pipeline/ready_signed_provenance_rollback_safe.json"
        ),
        "refused_missing_rollback_target" => include_str!(
            "../../../../fixtures/extensions/m3/publication_pipeline/refused_missing_rollback_target.json"
        ),
        "refused_identity_mutation" => include_str!(
            "../../../../fixtures/extensions/m3/publication_pipeline/refused_identity_mutation.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let record = evaluate_extension_publication_pipeline(fixture.input);
    assert_eq!(
        record.record_kind,
        EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND
    );
    assert_eq!(
        record.extension_publication_schema_version,
        EXTENSION_PUBLICATION_SCHEMA_VERSION
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
        record.promotion_step_count, fixture.meta.expected_promotion_step_count,
        "promotion step count mismatch for {name}"
    );
    assert_eq!(
        record.required_evidence_ref_count, fixture.meta.expected_required_evidence_ref_count,
        "evidence ref count mismatch for {name}"
    );
    assert_eq!(
        record.approver_ref_count, fixture.meta.expected_approver_ref_count,
        "approver ref count mismatch for {name}"
    );
    assert_eq!(
        record.preserves_prior_installable_artifact,
        fixture.meta.expected_preserves_prior_installable_artifact,
        "rollback preservation mismatch for {name}"
    );
    assert_eq!(
        record.transactional_catalog_update, fixture.meta.expected_transactional_catalog_update,
        "transaction guard mismatch for {name}"
    );

    let findings = validate_extension_publication_pipeline_record(&record);
    assert!(
        findings.is_empty(),
        "fixture {name} produced publication validation findings: {findings:?}"
    );

    let export = project_extension_publication_support_export(
        &record,
        &format!(
            "extension_publication_support_export:{}",
            record.publication_id
        ),
    );
    assert_eq!(
        export.record_kind,
        EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.publication_ref, record.publication_id);
    assert_eq!(export.decision_class, record.decision_class);
    assert_eq!(export.reason_class, record.reason_class);
    assert_eq!(
        export.bridge_matrix_ref,
        record.compatibility_metadata.bridge_matrix_ref
    );
    assert_eq!(
        export.bridge_matrix_row_ref,
        record.compatibility_metadata.bridge_matrix_row_ref
    );
    assert_eq!(
        export.lifecycle_metadata_ref,
        record.compatibility_metadata.lifecycle_metadata_ref
    );
    assert_eq!(
        export.deprecation_packet_template_ref,
        record
            .compatibility_metadata
            .deprecation_packet_template_ref
    );
    assert_eq!(
        export.blocks_catalog_mutation,
        record.decision_class == PublicationDecisionClass::Refused
    );
    assert_eq!(
        export.transactional_catalog_update,
        record.transactional_catalog_update
    );
    assert_eq!(export.redaction_class, RedactionClass::MetadataSafeDefault);
    let export_findings = validate_extension_publication_support_export_record(&export);
    assert!(
        export_findings.is_empty(),
        "fixture {name} produced support-export validation findings: {export_findings:?}"
    );
}

#[test]
fn ready_publication_round_trips() {
    run_fixture("ready_signed_provenance_rollback_safe");
}

#[test]
fn refused_missing_rollback_target_round_trips() {
    run_fixture("refused_missing_rollback_target");
}

#[test]
fn refused_identity_mutation_round_trips() {
    run_fixture("refused_identity_mutation");
}

#[test]
fn unsigned_artifacts_are_refused() {
    let mut fixture = load_fixture("ready_signed_provenance_rollback_safe");
    fixture.input.signer_metadata.signature_class =
        PublicationSignatureClass::UnsignedDeniedOnPolicy;
    let record = evaluate_extension_publication_pipeline(fixture.input);
    assert_eq!(record.decision_class, PublicationDecisionClass::Refused);
    assert_eq!(
        record.reason_class,
        PublicationReasonClass::RefusedUnsignedArtifact
    );
}

#[test]
fn unsafe_partial_catalog_writes_are_refused() {
    let mut fixture = load_fixture("ready_signed_provenance_rollback_safe");
    fixture
        .input
        .failure_atomicity_guard
        .transaction_write_class = PublicationTransactionWriteClass::UnsafePartialWrites;
    let record = evaluate_extension_publication_pipeline(fixture.input);
    assert_eq!(record.decision_class, PublicationDecisionClass::Refused);
    assert_eq!(
        record.reason_class,
        PublicationReasonClass::RefusedTransactionalWriteGuardMissing
    );
}

#[test]
fn valid_non_production_publication_is_held_for_review() {
    let mut fixture = load_fixture("ready_signed_provenance_rollback_safe");
    fixture.input.promotion_steps.pop();
    let record = evaluate_extension_publication_pipeline(fixture.input);
    assert_eq!(
        record.decision_class,
        PublicationDecisionClass::HeldForReview
    );
    assert_eq!(
        record.reason_class,
        PublicationReasonClass::HeldNoProductionPromotionRequested
    );
}
