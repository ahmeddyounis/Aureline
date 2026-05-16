//! Unit and fixture coverage for the runtime v1 beta admission contract.

use serde::Deserialize;

use super::{
    evaluate_runtime_v1_beta_contract, project_runtime_v1_beta_support_export,
    validate_runtime_v1_beta_contract, HostPlacementClass, HostSupervisionClass,
    RuntimeAdmissionDecisionClass, RuntimeAdmissionReasonClass, RuntimeV1BetaContractInput,
    SdkAlignmentClass, RUNTIME_V1_BETA_CONTRACT_RECORD_KIND, RUNTIME_V1_BETA_SCHEMA_VERSION,
    RUNTIME_V1_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::{HostContractFamilyClass, InstallDecisionClass, RedactionClass};

#[derive(Debug, Deserialize)]
struct ContractFixture {
    input: RuntimeV1BetaContractInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_decision_class: RuntimeAdmissionDecisionClass,
    expected_reason_class: RuntimeAdmissionReasonClass,
}

fn load_fixture(name: &str) -> ContractFixture {
    let raw = match name {
        "wasm_in_process_admitted" => include_str!(
            "../../../../fixtures/extensions/runtime_v1_beta_cases/wasm_in_process_admitted.json"
        ),
        "wasm_subprocess_admitted_narrowed" => include_str!(
            "../../../../fixtures/extensions/runtime_v1_beta_cases/wasm_subprocess_admitted_narrowed.json"
        ),
        "external_host_quarantined" => include_str!(
            "../../../../fixtures/extensions/runtime_v1_beta_cases/external_host_quarantined.json"
        ),
        "anonymous_publisher_refused" => include_str!(
            "../../../../fixtures/extensions/runtime_v1_beta_cases/anonymous_publisher_refused.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let contract = evaluate_runtime_v1_beta_contract(fixture.input.clone());
    assert_eq!(contract.record_kind, RUNTIME_V1_BETA_CONTRACT_RECORD_KIND);
    assert_eq!(
        contract.runtime_v1_beta_schema_version,
        RUNTIME_V1_BETA_SCHEMA_VERSION
    );
    assert_eq!(
        contract.redaction_class,
        RedactionClass::MetadataSafeDefault
    );
    assert_eq!(
        contract.admission_decision_class, fixture.meta.expected_decision_class,
        "decision mismatch for {name}"
    );
    assert_eq!(
        contract.admission_reason_class, fixture.meta.expected_reason_class,
        "reason mismatch for {name}"
    );

    let findings = validate_runtime_v1_beta_contract(&contract);
    assert!(
        findings.is_empty(),
        "fixture {name} produced validation findings: {findings:?}"
    );

    let export = project_runtime_v1_beta_support_export(&contract);
    assert_eq!(
        export.record_kind,
        RUNTIME_V1_BETA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.contract_ref, contract.contract_id);
    assert_eq!(
        export.declared_world_count as usize,
        contract.declared_capability_world_refs.len()
    );
    assert_eq!(
        export.negotiated_world_count as usize,
        contract.negotiated_capability_world_refs.len()
    );
    assert_eq!(
        export.narrowed_world_count as usize,
        contract.narrowed_capability_world_refs.len()
    );
    let blocks = matches!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused | RuntimeAdmissionDecisionClass::Quarantined
    );
    assert_eq!(export.blocks_activation, blocks);
}

#[test]
fn wasm_in_process_admitted_round_trips() {
    run_fixture("wasm_in_process_admitted");
}

#[test]
fn wasm_subprocess_admitted_narrowed_round_trips() {
    run_fixture("wasm_subprocess_admitted_narrowed");
}

#[test]
fn external_host_quarantined_round_trips() {
    run_fixture("external_host_quarantined");
}

#[test]
fn anonymous_publisher_refused_round_trips() {
    run_fixture("anonymous_publisher_refused");
}

#[test]
fn missing_permission_diff_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture.input.effective_permission_diff_present = false;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::PermissionDiffMissing
    );
}

#[test]
fn widening_attempted_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture
        .input
        .effective_permission_widening_attempted_blocked_count = 1;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::EffectivePermissionWideningAttempted
    );
}

#[test]
fn host_placement_family_mismatch_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture.input.host_contract_family_class = HostContractFamilyClass::ExternalHostProcess;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::HostPlacementUnsupported
    );
}

#[test]
fn unknown_placement_class_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture.input.host_placement_class = HostPlacementClass::UnknownPlacementClass;
    fixture.input.host_supervision_class = HostSupervisionClass::UnknownSupervisionClass;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::HostPlacementUnsupported
    );
}

#[test]
fn negotiated_widening_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture
        .input
        .negotiated_capability_world_refs
        .push("aureline:network-egress@0.1.0".to_string());
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::HostPlacementUnsupported
    );
}

#[test]
fn missing_narrowing_reasons_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_subprocess_admitted_narrowed");
    fixture.input.narrowing_reasons_recorded = false;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::PermissionDiffMissing
    );
}

#[test]
fn sdk_drift_refuses_runtime_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture.input.sdk_alignment_class = SdkAlignmentClass::SdkMarketplaceDrift;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::SdkOrMarketplaceMetadataOutOfDate
    );
}

#[test]
fn narrowing_with_pending_activation_awaits_user_review() {
    let mut fixture = load_fixture("wasm_subprocess_admitted_narrowed");
    fixture.input.lifecycle_state_class = super::RuntimeLifecycleStateClass::PendingActivation;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::AwaitingUserReview
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::AwaitingUserWorldAcknowledgement
    );
}

#[test]
fn quarantined_lifecycle_holds_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture.input.lifecycle_state_class = super::RuntimeLifecycleStateClass::Quarantined;
    fixture.input.runtime_budget_quarantine_active = true;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Quarantined
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::RuntimeBudgetQuarantineActive
    );
    let validation = validate_runtime_v1_beta_contract(&contract);
    assert!(
        validation.is_empty(),
        "validation should pass: {validation:?}"
    );
}

#[test]
fn validation_flags_missing_prefixes() {
    let fixture = load_fixture("wasm_in_process_admitted");
    let mut contract = evaluate_runtime_v1_beta_contract(fixture.input);
    contract.contract_id = "ad-hoc-id".to_string();
    contract.manifest_baseline_ref = "baseline-without-prefix".to_string();
    contract.host_negotiation_packet_ref = "negotiation-without-prefix".to_string();
    contract.sdk_release_bundle_ref = "sdk-without-prefix".to_string();
    contract.marketplace_metadata_ref = "metadata-without-prefix".to_string();
    let findings = validate_runtime_v1_beta_contract(&contract);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"runtime_v1_beta.contract.id_unprefixed"));
    assert!(ids.contains(&"runtime_v1_beta.contract.manifest_baseline_ref_unprefixed"));
    assert!(ids.contains(&"runtime_v1_beta.contract.host_negotiation_packet_ref_unprefixed"));
    assert!(ids.contains(&"runtime_v1_beta.contract.sdk_release_bundle_ref_unprefixed"));
    assert!(ids.contains(&"runtime_v1_beta.contract.marketplace_metadata_ref_unprefixed"));
}

#[test]
fn manifest_decision_admit_required_for_admission() {
    let mut fixture = load_fixture("wasm_in_process_admitted");
    fixture.input.manifest_install_decision_class = InstallDecisionClass::Denied;
    fixture.input.manifest_install_decision_reason_class =
        crate::manifest_baseline::InstallDecisionReasonClass::ManifestScopeIncomplete;
    let contract = evaluate_runtime_v1_beta_contract(fixture.input);
    assert_eq!(
        contract.admission_decision_class,
        RuntimeAdmissionDecisionClass::Refused
    );
    assert_eq!(
        contract.admission_reason_class,
        RuntimeAdmissionReasonClass::ManifestInstallDenied
    );
}
