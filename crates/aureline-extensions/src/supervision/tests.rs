//! Unit and fixture coverage for the extension host isolation,
//! restart-budget, resource-limit, and quarantine supervision contract.

use serde::Deserialize;

use super::{
    evaluate_extension_host_supervision, project_extension_host_supervision_support_export,
    validate_extension_host_supervision, BudgetPressureClass, DiscoveryRankingPostureClass,
    ExtensionHostSupervisionInput, MaintainerCoverageClass, RecoveryPreconditionClass,
    RecoveryVisibleProjectionClass, SupervisionAxisClass, SupervisionDecisionClass,
    SupervisionReasonClass, SupervisionResponseClass, VisibilityPostureClass,
    EXTENSION_HOST_SUPERVISION_RECORD_KIND, EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION,
    EXTENSION_HOST_SUPERVISION_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::manifest_baseline::RedactionClass;
use crate::runtime::RestartPostureClass;

#[derive(Debug, Deserialize)]
struct SupervisionFixture {
    input: ExtensionHostSupervisionInput,
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    expected_response_class: SupervisionResponseClass,
    expected_decision_class: SupervisionDecisionClass,
    expected_reason_class: SupervisionReasonClass,
}

fn load_fixture(name: &str) -> SupervisionFixture {
    let raw = match name {
        "wasm_in_process_nominal_continue" => include_str!(
            "../../../../fixtures/extensions/m3/isolation_and_quarantine/wasm_in_process_nominal_continue.json"
        ),
        "wasm_subprocess_soft_breach_throttled" => include_str!(
            "../../../../fixtures/extensions/m3/isolation_and_quarantine/wasm_subprocess_soft_breach_throttled.json"
        ),
        "external_host_memory_hard_cap_disabled" => include_str!(
            "../../../../fixtures/extensions/m3/isolation_and_quarantine/external_host_memory_hard_cap_disabled.json"
        ),
        "external_host_crash_loop_quarantined" => include_str!(
            "../../../../fixtures/extensions/m3/isolation_and_quarantine/external_host_crash_loop_quarantined.json"
        ),
        "wasm_subprocess_publisher_blocked_hold" => include_str!(
            "../../../../fixtures/extensions/m3/isolation_and_quarantine/wasm_subprocess_publisher_blocked_hold.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"))
}

fn run_fixture(name: &str) {
    let fixture = load_fixture(name);
    assert_eq!(fixture.meta.name, name);
    assert!(!fixture.meta.scenario.trim().is_empty());

    let record = evaluate_extension_host_supervision(fixture.input.clone());
    assert_eq!(record.record_kind, EXTENSION_HOST_SUPERVISION_RECORD_KIND);
    assert_eq!(
        record.extension_host_supervision_schema_version,
        EXTENSION_HOST_SUPERVISION_SCHEMA_VERSION
    );
    assert_eq!(record.redaction_class, RedactionClass::MetadataSafeDefault);
    assert_eq!(
        record.response_class, fixture.meta.expected_response_class,
        "response mismatch for {name}"
    );
    assert_eq!(
        record.supervision_decision_class, fixture.meta.expected_decision_class,
        "decision mismatch for {name}"
    );
    assert_eq!(
        record.supervision_reason_class, fixture.meta.expected_reason_class,
        "reason mismatch for {name}"
    );

    let findings = validate_extension_host_supervision(&record);
    assert!(
        findings.is_empty(),
        "fixture {name} produced validation findings: {findings:?}"
    );

    let export = project_extension_host_supervision_support_export(&record);
    assert_eq!(
        export.record_kind,
        EXTENSION_HOST_SUPERVISION_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.supervision_ref, record.supervision_id);
    assert_eq!(export.contract_ref, record.contract_ref);
    assert_eq!(export.blocks_activation, record.blocks_activation);
    assert_eq!(
        export.response_class, record.response_class,
        "export response_class must round-trip"
    );
    assert_eq!(
        export.supervision_decision_class, record.supervision_decision_class,
        "export decision_class must round-trip"
    );
}

#[test]
fn nominal_continue_round_trips() {
    run_fixture("wasm_in_process_nominal_continue");
}

#[test]
fn soft_breach_throttled_round_trips() {
    run_fixture("wasm_subprocess_soft_breach_throttled");
}

#[test]
fn memory_hard_cap_disabled_round_trips() {
    run_fixture("external_host_memory_hard_cap_disabled");
}

#[test]
fn crash_loop_quarantined_round_trips() {
    run_fixture("external_host_crash_loop_quarantined");
}

#[test]
fn publisher_blocked_hold_round_trips() {
    run_fixture("wasm_subprocess_publisher_blocked_hold");
}

#[test]
fn quarantine_without_maintainer_coverage_refuses() {
    let mut fixture = load_fixture("external_host_crash_loop_quarantined");
    fixture.input.maintainer_coverage_class = MaintainerCoverageClass::RequiredQuorumMissing;
    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RefuseInconsistentInput
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RefusedMaintainerCoverageMissingOnQuorumDecision
    );
    assert!(record.blocks_activation);
}

#[test]
fn quarantine_without_trigger_rule_refuses() {
    let mut fixture = load_fixture("external_host_crash_loop_quarantined");
    fixture.input.trigger_rule_ref = None;
    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RefuseInconsistentInput
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RefusedQuarantineWithoutTriggerRule
    );
}

#[test]
fn quarantine_without_discovery_removal_refuses() {
    let mut fixture = load_fixture("external_host_crash_loop_quarantined");
    fixture.input.discovery_ranking_posture_class =
        DiscoveryRankingPostureClass::DemotedFairRankedWithVisibleWarning;
    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RefuseInconsistentInput
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RefusedQuarantineWithoutDiscoveryRemoval
    );
}

#[test]
fn disable_without_user_surface_refuses() {
    let mut fixture = load_fixture("external_host_memory_hard_cap_disabled");
    fixture.input.visibility_posture_class = VisibilityPostureClass::RuntimeStatusPillOnly;
    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RefuseInconsistentInput
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RefusedResponseVisibilityMissingFromUserSurfaces
    );
}

#[test]
fn egress_hard_breach_requires_user_reenable() {
    let mut fixture = load_fixture("wasm_in_process_nominal_continue");
    for entry in fixture.input.axis_budget_entries.iter_mut() {
        if entry.axis_class == SupervisionAxisClass::Egress {
            entry.pressure_class = BudgetPressureClass::HardBreach;
        }
    }
    fixture.input.visibility_posture_class =
        VisibilityPostureClass::InstallReviewAndPermissionInspectorAndRuntimeStatusPill;
    fixture.input.discovery_ranking_posture_class =
        DiscoveryRankingPostureClass::SuppressInstalledInManyWorkspacesSignal;
    fixture.input.maintainer_coverage_class = MaintainerCoverageClass::RequiredQuorumRecorded;
    fixture.input.trigger_rule_ref =
        Some("quarantine_rule:egress_sustained_hard_breach_disables_until_reenable".to_string());
    fixture.input.repair_affordance_label =
        "review_egress_budget_in_permission_inspector".to_string();

    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.response_class,
        SupervisionResponseClass::DisableUntilUserExplicitReenable
    );
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::DisableUntilUserExplicitReenable
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::EgressHardBreachRequiresUserReenable
    );
    assert!(record.blocks_activation);
}

#[test]
fn egress_disable_without_maintainer_coverage_refuses() {
    let mut fixture = load_fixture("wasm_in_process_nominal_continue");
    for entry in fixture.input.axis_budget_entries.iter_mut() {
        if entry.axis_class == SupervisionAxisClass::Egress {
            entry.pressure_class = BudgetPressureClass::HardBreach;
        }
    }
    fixture.input.visibility_posture_class =
        VisibilityPostureClass::InstallReviewAndPermissionInspectorAndRuntimeStatusPill;
    fixture.input.discovery_ranking_posture_class =
        DiscoveryRankingPostureClass::SuppressInstalledInManyWorkspacesSignal;
    fixture.input.maintainer_coverage_class = MaintainerCoverageClass::NotRequired;
    fixture.input.repair_affordance_label =
        "review_egress_budget_in_permission_inspector".to_string();

    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RefuseInconsistentInput
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RefusedMaintainerCoverageMissingOnQuorumDecision
    );
}

#[test]
fn recovery_in_progress_when_precondition_pending() {
    let mut fixture = load_fixture("wasm_in_process_nominal_continue");
    fixture.input.recovery_precondition_class =
        RecoveryPreconditionClass::ResourceGovernorReturnedToNominal;
    fixture.input.recovery_visible_projection_class = RecoveryVisibleProjectionClass::Warming;
    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RecoveryInProgress
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RecoveryInProgressReturningToNominal
    );
}

#[test]
fn validation_flags_supervision_id_missing_prefix() {
    let fixture = load_fixture("wasm_in_process_nominal_continue");
    let mut record = evaluate_extension_host_supervision(fixture.input);
    record.supervision_id = "ad-hoc-id".to_string();
    let findings = validate_extension_host_supervision(&record);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"extension_host_supervision.id_unprefixed"));
}

#[test]
fn validation_flags_attempts_disagreement() {
    let fixture = load_fixture("external_host_memory_hard_cap_disabled");
    let mut record = evaluate_extension_host_supervision(fixture.input);
    record.restart_budget.attempts_used = record.restart_attempt_count.saturating_add(1);
    let findings = validate_extension_host_supervision(&record);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"extension_host_supervision.restart_attempts_disagree"));
}

#[test]
fn validation_flags_restart_posture_disagreement() {
    let fixture = load_fixture("external_host_memory_hard_cap_disabled");
    let mut record = evaluate_extension_host_supervision(fixture.input);
    record.restart_budget.restart_posture_class = RestartPostureClass::NoRestartAttempted;
    let findings = validate_extension_host_supervision(&record);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"extension_host_supervision.restart_posture_disagree"));
}

#[test]
fn validation_flags_crash_loop_thresholds_invalid() {
    let fixture = load_fixture("external_host_crash_loop_quarantined");
    let mut record = evaluate_extension_host_supervision(fixture.input);
    record.restart_budget.crash_loop_trip_disable_threshold = 10;
    record.restart_budget.crash_loop_trip_quarantine_threshold = 5;
    let findings = validate_extension_host_supervision(&record);
    let ids: Vec<&str> = findings.iter().map(|f| f.check_id).collect();
    assert!(ids.contains(&"extension_host_supervision.crash_loop_thresholds_invalid"));
}

#[test]
fn nominal_response_under_runtime_budget_quarantine_refuses() {
    let mut fixture = load_fixture("wasm_in_process_nominal_continue");
    fixture
        .input
        .runtime_contract
        .runtime_budget_quarantine_active = true;
    let record = evaluate_extension_host_supervision(fixture.input);
    assert_eq!(
        record.supervision_decision_class,
        SupervisionDecisionClass::RefuseInconsistentInput
    );
    assert_eq!(
        record.supervision_reason_class,
        SupervisionReasonClass::RefusedAxisPressureInconsistentWithResponse
    );
}
