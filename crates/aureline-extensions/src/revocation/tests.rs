//! Unit and fixture coverage for extension incident communication packets.

use serde::Deserialize;

use super::{
    evaluate_extension_incident_communication, project_extension_incident_support_export,
    validate_extension_incident_communication_record,
    validate_extension_incident_support_export_record, ExtensionIncidentCommunicationInput,
    ExtensionIncidentDecisionClass, ExtensionIncidentDecisionReasonClass,
    ExtensionIncidentLifecycleStateClass, ExtensionIncidentTrustStateClass,
    EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND, EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION,
    EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;
use crate::review_alpha::RevocationStateClass;

#[derive(Debug, Deserialize)]
struct IncidentFixture {
    input: ExtensionIncidentCommunicationInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: ExtensionIncidentDecisionClass,
    expected_reason_class: ExtensionIncidentDecisionReasonClass,
    expected_lifecycle_state_class: ExtensionIncidentLifecycleStateClass,
    expected_revocation_state_class: RevocationStateClass,
    expected_blocks_new_installs: bool,
    expected_blocks_updates: bool,
    expected_blocks_activation: bool,
    expected_blocks_execution: bool,
    expected_mirror_trust_unambiguous: bool,
    expected_recovery_guidance_ready: bool,
}

fn load_fixture(name: &str) -> IncidentFixture {
    let raw = match name {
        "primary_registry_emergency_disable" => include_str!(
            "../../../../fixtures/extensions/m3/revocation_and_emergency_disable/primary_registry_emergency_disable.json"
        ),
        "mirror_quarantine_pending_reverify" => include_str!(
            "../../../../fixtures/extensions/m3/revocation_and_emergency_disable/mirror_quarantine_pending_reverify.json"
        ),
        "artifact_revoked_mirror_parity" => include_str!(
            "../../../../fixtures/extensions/m3/revocation_and_emergency_disable/artifact_revoked_mirror_parity.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let record = evaluate_extension_incident_communication(fixture.input);
    assert_eq!(
        record.record_kind,
        EXTENSION_INCIDENT_COMMUNICATION_RECORD_KIND
    );
    assert_eq!(
        record.extension_incident_schema_version,
        EXTENSION_INCIDENT_COMMUNICATION_SCHEMA_VERSION
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
        record.lifecycle_state_class, fixture.meta.expected_lifecycle_state_class,
        "lifecycle mismatch for {name}"
    );
    assert_eq!(
        record.revocation_state_class, fixture.meta.expected_revocation_state_class,
        "revocation state mismatch for {name}"
    );
    assert_eq!(
        record.blocks_new_installs, fixture.meta.expected_blocks_new_installs,
        "new-install block mismatch for {name}"
    );
    assert_eq!(
        record.blocks_updates, fixture.meta.expected_blocks_updates,
        "update block mismatch for {name}"
    );
    assert_eq!(
        record.blocks_activation, fixture.meta.expected_blocks_activation,
        "activation block mismatch for {name}"
    );
    assert_eq!(
        record.blocks_execution, fixture.meta.expected_blocks_execution,
        "execution block mismatch for {name}"
    );
    assert_eq!(
        record.mirror_trust_unambiguous, fixture.meta.expected_mirror_trust_unambiguous,
        "mirror trust mismatch for {name}"
    );
    assert_eq!(
        record.recovery_guidance_ready, fixture.meta.expected_recovery_guidance_ready,
        "recovery guidance mismatch for {name}"
    );

    let findings = validate_extension_incident_communication_record(&record);
    assert!(
        findings.is_empty(),
        "fixture {name} produced incident validation findings: {findings:?}"
    );

    let export = project_extension_incident_support_export(
        &record,
        &format!("extension_incident_support_export:{}", record.incident_id),
    );
    assert_eq!(
        export.record_kind,
        EXTENSION_INCIDENT_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.incident_ref, record.incident_id);
    assert_eq!(export.advisory_id, record.advisory.advisory_id);
    assert_eq!(export.lifecycle_state_class, record.lifecycle_state_class);
    assert_eq!(export.revocation_state_class, record.revocation_state_class);
    assert_eq!(export.decision_class, record.decision_class);
    assert_eq!(export.reason_class, record.reason_class);
    assert_eq!(export.redaction_class, RedactionClass::MetadataSafeDefault);
    let export_findings = validate_extension_incident_support_export_record(&export);
    assert!(
        export_findings.is_empty(),
        "fixture {name} produced support-export validation findings: {export_findings:?}"
    );
}

#[test]
fn primary_registry_emergency_disable_round_trips() {
    run_fixture("primary_registry_emergency_disable");
}

#[test]
fn mirror_quarantine_pending_reverify_round_trips() {
    run_fixture("mirror_quarantine_pending_reverify");
}

#[test]
fn artifact_revoked_mirror_parity_round_trips() {
    run_fixture("artifact_revoked_mirror_parity");
}

#[test]
fn ambiguous_mirror_trust_refuses_incident_action() {
    let mut fixture = load_fixture("primary_registry_emergency_disable");
    fixture.input.mirror_lane.trust_state_class = ExtensionIncidentTrustStateClass::UnknownRefused;

    let record = evaluate_extension_incident_communication(fixture.input);
    assert_eq!(
        record.decision_class,
        ExtensionIncidentDecisionClass::Refused
    );
    assert_eq!(
        record.reason_class,
        ExtensionIncidentDecisionReasonClass::RefusedAmbiguousTrustState
    );
    assert!(!record.mirror_trust_unambiguous);
    assert!(validate_extension_incident_communication_record(&record)
        .iter()
        .any(|finding| finding.check_id == "extension_incident.trust_state_ambiguous"));
}

#[test]
fn missing_recovery_guidance_refuses_forced_action() {
    let mut fixture = load_fixture("artifact_revoked_mirror_parity");
    fixture.input.recovery.recovery_action_classes.clear();
    fixture.input.recovery.rollback_manifest_ref = None;
    fixture.input.recovery.safe_mode_profile_ref = None;
    fixture.input.recovery.admin_handoff_refs.clear();

    let record = evaluate_extension_incident_communication(fixture.input);
    assert_eq!(
        record.decision_class,
        ExtensionIncidentDecisionClass::Refused
    );
    assert_eq!(
        record.reason_class,
        ExtensionIncidentDecisionReasonClass::RefusedRecoveryGuidanceMissing
    );
    assert!(!record.recovery_guidance_ready);
    assert!(validate_extension_incident_communication_record(&record)
        .iter()
        .any(|finding| finding.check_id == "extension_incident.recovery_guidance_required"));
}
