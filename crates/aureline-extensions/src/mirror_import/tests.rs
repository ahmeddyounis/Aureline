//! Unit and fixture coverage for mirror and manual-import baselines.

use serde::Deserialize;

use super::{
    evaluate_mirror_import_baseline, project_mirror_import_support_export,
    validate_mirror_import_baseline_record, validate_mirror_import_support_export_record,
    MirrorImportDecisionClass, MirrorImportDisclosureClass, MirrorImportFinding,
    MirrorImportReasonClass, MirrorImportRouteClass, MirrorImportSupportExplanationClass,
    MirrorImportSupportExportRecord, MirrorImportTrustClaimClass, MirrorImportTrustClaimStateClass,
    MIRROR_IMPORT_BASELINE_RECORD_KIND, MIRROR_IMPORT_BASELINE_SCHEMA_VERSION,
    MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;

#[derive(Debug, Deserialize)]
struct MirrorImportFixture {
    input: super::MirrorImportBaselineInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: MirrorImportDecisionClass,
    expected_reason_class: MirrorImportReasonClass,
    expected_support_explanation_class: MirrorImportSupportExplanationClass,
    expected_install_lane_continues: bool,
    expected_downgraded_trust_claim_count: u32,
}

fn load_fixture(name: &str) -> MirrorImportFixture {
    let raw = match name {
        "primary_catalog_baseline_ready" => include_str!(
            "../../../../fixtures/extensions/m3/mirror_import/primary_catalog_baseline_ready.json"
        ),
        "approved_mirror_degraded_trust_claim_ready" => include_str!(
            "../../../../fixtures/extensions/m3/mirror_import/approved_mirror_degraded_trust_claim_ready.json"
        ),
        "manual_artifact_import_preserves_metadata" => include_str!(
            "../../../../fixtures/extensions/m3/mirror_import/manual_artifact_import_preserves_metadata.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) -> MirrorImportSupportExportRecord {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let record = evaluate_mirror_import_baseline(fixture.input);
    assert_eq!(record.record_kind, MIRROR_IMPORT_BASELINE_RECORD_KIND);
    assert_eq!(
        record.mirror_import_baseline_schema_version,
        MIRROR_IMPORT_BASELINE_SCHEMA_VERSION
    );
    assert_eq!(record.redaction_class, RedactionClass::MetadataSafeDefault);
    assert_eq!(record.decision_class, fixture.meta.expected_decision_class);
    assert_eq!(record.reason_class, fixture.meta.expected_reason_class);
    assert_eq!(
        record.support_explanation_class,
        fixture.meta.expected_support_explanation_class
    );
    assert_eq!(
        record.install_lane_continues,
        fixture.meta.expected_install_lane_continues
    );
    assert_eq!(
        record.downgraded_trust_claim_count,
        fixture.meta.expected_downgraded_trust_claim_count
    );
    assert!(record.artifact_identity_preserved);
    assert!(record.publisher_continuity_preserved);
    assert!(record.permission_metadata_preserved);
    assert!(record.compatibility_metadata_preserved);
    assert!(record.lifecycle_metadata_preserved);
    assert!(record.source_visible_to_users_admins);
    assert!(
        validate_mirror_import_baseline_record(&record).is_empty(),
        "fixture {name} produced mirror-import validation findings"
    );

    let export = project_mirror_import_support_export(
        &record,
        &format!("mirror_import_support_export:{}", record.baseline_id),
    );
    assert_eq!(export.record_kind, MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.baseline_ref, record.baseline_id);
    assert_eq!(export.route_class, record.route_class);
    assert_eq!(
        export.delivered_registry_source_class,
        record.delivered_registry_source_class
    );
    assert_eq!(export.install_lane_continues, record.install_lane_continues);
    assert_eq!(
        export.downgraded_trust_claim_count,
        record.downgraded_trust_claim_count
    );
    assert!(validate_mirror_import_support_export_record(&export).is_empty());
    export
}

#[test]
fn primary_catalog_baseline_preserves_all_metadata() {
    let export = run_fixture("primary_catalog_baseline_ready");
    assert_eq!(export.route_class, MirrorImportRouteClass::PrimaryCatalog);
    assert!(export.install_lane_continues);
}

#[test]
fn approved_mirror_downgrade_keeps_install_lane_open() {
    let export = run_fixture("approved_mirror_degraded_trust_claim_ready");
    assert_eq!(export.route_class, MirrorImportRouteClass::ApprovedMirror);
    assert_eq!(export.downgraded_trust_claim_count, 1);
    assert!(export.install_lane_continues);
}

#[test]
fn manual_artifact_preserves_metadata_with_visible_unverified_trust() {
    let export = run_fixture("manual_artifact_import_preserves_metadata");
    assert_eq!(export.route_class, MirrorImportRouteClass::ManualArtifact);
    assert_eq!(export.downgraded_trust_claim_count, 2);
    assert!(export.install_lane_continues);
}

#[test]
fn missing_required_disclosure_refuses_import() {
    let mut fixture = load_fixture("primary_catalog_baseline_ready");
    fixture
        .input
        .rendered_disclosures
        .retain(|disclosure| !matches!(disclosure, MirrorImportDisclosureClass::SourceLane));

    let record = evaluate_mirror_import_baseline(fixture.input);
    assert_eq!(record.decision_class, MirrorImportDecisionClass::Refused);
    assert_eq!(
        record.reason_class,
        MirrorImportReasonClass::RefusedRequiredDisclosureMissing
    );
    assert!(has_finding(
        &validate_mirror_import_baseline_record(&record),
        "mirror_import.required_disclosure_missing"
    ));
}

#[test]
fn content_address_mismatch_refuses_import() {
    let mut fixture = load_fixture("approved_mirror_degraded_trust_claim_ready");
    fixture.input.content_address.digest_hex =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();

    let record = evaluate_mirror_import_baseline(fixture.input);
    assert_eq!(record.decision_class, MirrorImportDecisionClass::Refused);
    assert_eq!(
        record.reason_class,
        MirrorImportReasonClass::RefusedArtifactIdentityMismatch
    );
    assert!(!record.install_lane_continues);
}

#[test]
fn refused_trust_claim_blocks_install_lane() {
    let mut fixture = load_fixture("primary_catalog_baseline_ready");
    let signature_claim = fixture
        .input
        .trust_claims
        .iter_mut()
        .find(|claim| claim.claim_class == MirrorImportTrustClaimClass::Signature)
        .expect("fixture carries a signature claim");
    signature_claim.state_class = MirrorImportTrustClaimStateClass::Refused;

    let record = evaluate_mirror_import_baseline(fixture.input);
    assert_eq!(record.decision_class, MirrorImportDecisionClass::Refused);
    assert_eq!(
        record.reason_class,
        MirrorImportReasonClass::RefusedTrustClaimBlocksInstall
    );
    assert!(!record.install_lane_continues);
}

fn has_finding(findings: &[MirrorImportFinding], check_id: &str) -> bool {
    findings.iter().any(|finding| finding.check_id == check_id)
}
