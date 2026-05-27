//! Fixture-driven coverage for the stable debugger-host / adapter-negotiation
//! / attach-launch / crash-isolation truth packet covering the local,
//! remote_helper, container, and notebook_bridge debugger lanes plus the
//! four-wedge admission coverage, the six adapter-descriptor field bindings,
//! the four attach/launch parity-surface bindings, the five crash-isolation
//! assertion bindings, and the lineage_admission row binding
//! `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_debugger_stabilization_truth_packet,
    DebuggerStabilizationAdapterDescriptorFieldClass,
    DebuggerStabilizationAttachLaunchParitySurfaceClass,
    DebuggerStabilizationAttachLaunchPostureClass, DebuggerStabilizationConsumerSurface,
    DebuggerStabilizationCrashIsolationAssertionClass,
    DebuggerStabilizationDowngradeAutomationClass, DebuggerStabilizationEvidenceClass,
    DebuggerStabilizationFindingKind, DebuggerStabilizationKnownLimitClass,
    DebuggerStabilizationLaneClass, DebuggerStabilizationPromotionState,
    DebuggerStabilizationRowClass, DebuggerStabilizationSupportClass,
    DebuggerStabilizationTruthPacket, DebuggerStabilizationTruthPacketInput,
    DebuggerStabilizationWedgeClass, DEBUGGER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF,
    DEBUGGER_STABILIZATION_TRUTH_DOC_REF, DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR,
    DEBUGGER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF, DEBUGGER_STABILIZATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DebuggerStabilizationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DebuggerStabilizationTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    lane_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    wedge_tokens: Vec<String>,
    adapter_descriptor_field_tokens: Vec<String>,
    attach_launch_parity_surface_tokens: Vec<String>,
    attach_launch_posture_tokens: Vec<String>,
    crash_isolation_assertion_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
    evidence_class_tokens: Vec<String>,
    support_export_safe: bool,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> DebuggerStabilizationFixture {
    let path = repo_root()
        .join(DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = DebuggerStabilizationTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_token_set_matches(&packet.lane_tokens(), &expect.lane_tokens, "lane");
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.support_class_tokens(),
        &expect.support_class_tokens,
        "support_class",
    );
    assert_token_set_matches(&packet.wedge_tokens(), &expect.wedge_tokens, "wedge");
    assert_token_set_matches(
        &packet.adapter_descriptor_field_tokens(),
        &expect.adapter_descriptor_field_tokens,
        "adapter_descriptor_field",
    );
    assert_token_set_matches(
        &packet.attach_launch_parity_surface_tokens(),
        &expect.attach_launch_parity_surface_tokens,
        "attach_launch_parity_surface",
    );
    assert_token_set_matches(
        &packet.attach_launch_posture_tokens(),
        &expect.attach_launch_posture_tokens,
        "attach_launch_posture",
    );
    assert_token_set_matches(
        &packet.crash_isolation_assertion_tokens(),
        &expect.crash_isolation_assertion_tokens,
        "crash_isolation_assertion",
    );
    assert_token_set_matches(
        &packet.known_limit_tokens(),
        &expect.known_limit_tokens,
        "known_limit",
    );
    assert_token_set_matches(
        &packet.downgrade_automation_tokens(),
        &expect.downgrade_automation_tokens,
        "downgrade_automation",
    );
    assert_token_set_matches(
        &packet.evidence_class_tokens(),
        &expect.evidence_class_tokens,
        "evidence_class",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(DEBUGGER_STABILIZATION_TRUTH_SCHEMA_REF);
    assert_exists(DEBUGGER_STABILIZATION_TRUTH_DOC_REF);
    assert_exists(DEBUGGER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR);
    assert_exists(DEBUGGER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn launch_stable_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("launch_stable_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_adapter_descriptor_field_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_adapter_descriptor_field_for_launch_stable_blocks_stable.json");
}

#[test]
fn attach_launch_posture_drift_blocks_stable() {
    assert_fixture_matches("attach_launch_posture_drift_blocks_stable.json");
}

#[test]
fn crash_isolation_assertion_not_attested_blocks_stable() {
    assert_fixture_matches("crash_isolation_assertion_not_attested_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_attach_launch_posture_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_attach_launch_posture_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet =
        current_stable_debugger_stabilization_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        DebuggerStabilizationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in DebuggerStabilizationLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for debugger lane {}",
            required.as_str()
        );
    }
    for surface in DebuggerStabilizationConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_wedges_fields_surfaces_and_assertions_per_launch_stable_lane(
) {
    let packet =
        current_stable_debugger_stabilization_truth_packet().expect("checked-in packet validates");
    for required in DebuggerStabilizationLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == DebuggerStabilizationRowClass::DebuggerStabilizationQuality
                && row.support_class == DebuggerStabilizationSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in DebuggerStabilizationWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == DebuggerStabilizationRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for field in DebuggerStabilizationAdapterDescriptorFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == DebuggerStabilizationRowClass::AdapterDescriptorFieldBinding
                    && row.adapter_descriptor_field_class == field),
                "stable packet must cover the {} adapter descriptor field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        for surface in
            DebuggerStabilizationAttachLaunchParitySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE
        {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == DebuggerStabilizationRowClass::AttachLaunchParitySurfaceBinding
                    && row.attach_launch_parity_surface_class == surface),
                "stable packet must cover the {} attach/launch parity surface on the {} lane",
                surface.as_str(),
                required.as_str()
            );
        }
        for assertion in
            DebuggerStabilizationCrashIsolationAssertionClass::REQUIRED_FOR_LAUNCH_STABLE
        {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == DebuggerStabilizationRowClass::CrashIsolationAssertionBinding
                    && row.crash_isolation_assertion_class == assertion
                    && row.attests_crash_isolation_assertion),
                "stable packet must include an attested crash-isolation assertion binding for {} on the {} lane",
                assertion.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == DebuggerStabilizationRowClass::LineageAdmission
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            }),
            "stable packet must include a lineage_admission row binding execution_context_id on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_debugger_stabilization_truth_tokens_are_pinned() {
    assert_eq!(
        DebuggerStabilizationLaneClass::LocalLane.as_str(),
        "local_lane"
    );
    assert_eq!(
        DebuggerStabilizationLaneClass::RemoteHelperLane.as_str(),
        "remote_helper_lane"
    );
    assert_eq!(
        DebuggerStabilizationLaneClass::ContainerLane.as_str(),
        "container_lane"
    );
    assert_eq!(
        DebuggerStabilizationLaneClass::NotebookBridgeLane.as_str(),
        "notebook_bridge_lane"
    );
    assert_eq!(
        DebuggerStabilizationRowClass::DebuggerStabilizationQuality.as_str(),
        "debugger_stabilization_quality"
    );
    assert_eq!(
        DebuggerStabilizationSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        DebuggerStabilizationSupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        DebuggerStabilizationSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        DebuggerStabilizationWedgeClass::DebuggerHost.as_str(),
        "debugger_host"
    );
    assert_eq!(
        DebuggerStabilizationWedgeClass::CrashIsolation.as_str(),
        "crash_isolation"
    );
    assert_eq!(
        DebuggerStabilizationAdapterDescriptorFieldClass::AdapterIdentity.as_str(),
        "adapter_identity"
    );
    assert_eq!(
        DebuggerStabilizationAdapterDescriptorFieldClass::NotebookBridgeOrReplayOnlyLimitation
            .as_str(),
        "notebook_bridge_or_replay_only_limitation"
    );
    assert_eq!(
        DebuggerStabilizationAttachLaunchParitySurfaceClass::UiSurface.as_str(),
        "ui_surface"
    );
    assert_eq!(
        DebuggerStabilizationAttachLaunchPostureClass::Supported.as_str(),
        "supported"
    );
    assert_eq!(
        DebuggerStabilizationAttachLaunchPostureClass::PolicyBlocked.as_str(),
        "policy_blocked"
    );
    assert_eq!(
        DebuggerStabilizationCrashIsolationAssertionClass::BoundedRestartBudget.as_str(),
        "bounded_restart_budget"
    );
    assert_eq!(
        DebuggerStabilizationEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        DebuggerStabilizationKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        DebuggerStabilizationDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        DebuggerStabilizationConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        DebuggerStabilizationFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        DebuggerStabilizationFindingKind::AttachLaunchPostureDrift.as_str(),
        "attach_launch_posture_drift"
    );
    assert_eq!(
        DebuggerStabilizationFindingKind::CrashIsolationAssertionNotAttested.as_str(),
        "crash_isolation_assertion_not_attested"
    );
    assert_eq!(
        DebuggerStabilizationFindingKind::LineageAdmissionMissingExecutionContextId.as_str(),
        "lineage_admission_missing_execution_context_id"
    );
}
