//! Fixture-driven coverage for the queue, restore, and terminal-boundary
//! governance packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_queue_session_terminal_governance_packet, ActivityJobStateClass, GovernanceFindingKind,
    GovernancePromotionState, GovernanceSupportClass, GovernedJobKind, GovernedWorkloadClass,
    QueueSessionTerminalGovernancePacket, QueueSessionTerminalGovernanceRowClass,
    BACKGROUND_QUEUE_CONTRACT_DOC_REF, CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF,
    HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF, QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF, QUEUE_SESSION_TERMINAL_GOVERNANCE_FIXTURE_DIR,
    QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GovernanceFixtureCase {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: String,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    support_export_safe: bool,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
    #[serde(default)]
    workload_tokens: Vec<String>,
    #[serde(default)]
    queue_lane_tokens: Vec<String>,
    #[serde(default)]
    job_kind_tokens: Vec<String>,
    #[serde(default)]
    row_class_tokens: Vec<String>,
    #[serde(default)]
    restore_fidelity_tokens: Vec<String>,
    #[serde(default)]
    terminal_boundary_tokens: Vec<String>,
    #[serde(default)]
    clipboard_posture_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> GovernanceFixtureCase {
    let path = repo_root()
        .join(QUEUE_SESSION_TERMINAL_GOVERNANCE_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    if expected.is_empty() {
        return;
    }
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn remove_row(
    packet: &mut QueueSessionTerminalGovernancePacket,
    workload: GovernedWorkloadClass,
    row_class: QueueSessionTerminalGovernanceRowClass,
) {
    packet
        .rows
        .retain(|row| !(row.workload_class == workload && row.row_class == row_class));
}

fn apply_mutation(packet: &mut QueueSessionTerminalGovernancePacket, mutation: &str) {
    match mutation {
        "none" => {}
        "remove_notebook_queue_identity_row" => remove_row(
            packet,
            GovernedWorkloadClass::NotebookSession,
            QueueSessionTerminalGovernanceRowClass::QueueIdentityAdmission,
        ),
        "remove_incident_terminal_boundary_row" => remove_row(
            packet,
            GovernedWorkloadClass::IncidentWorkspace,
            QueueSessionTerminalGovernanceRowClass::TerminalBoundaryAdmission,
        ),
        "narrow_preview_quality_without_disclosure" => {
            let row = packet
                .rows
                .iter_mut()
                .find(|row| {
                    row.workload_class == GovernedWorkloadClass::PreviewRoute
                        && row.row_class
                            == QueueSessionTerminalGovernanceRowClass::ContinuityQuality
                })
                .expect("preview quality row");
            row.support_class = GovernanceSupportClass::StableBelow;
            row.disclosure_ref = None;
        }
        "collapse_terminal_boundary_projection_vocabulary" => {
            let projection = packet
                .consumer_projections
                .iter_mut()
                .find(|projection| projection.projection_ref == "projection:terminal_header")
                .expect("terminal header projection");
            projection.preserves_terminal_boundary_vocabulary = false;
        }
        "expose_raw_source_material" => {
            let row = packet
                .rows
                .iter_mut()
                .find(|row| {
                    row.workload_class == GovernedWorkloadClass::InfrastructureSession
                        && row.row_class
                            == QueueSessionTerminalGovernanceRowClass::TerminalBoundaryAdmission
                })
                .expect("infrastructure terminal row");
            row.raw_private_material_excluded = false;
        }
        "consume_hot_path_budget" => {
            let row = packet
                .rows
                .iter_mut()
                .find(|row| {
                    row.workload_class == GovernedWorkloadClass::PreviewRoute
                        && row.row_class
                            == QueueSessionTerminalGovernanceRowClass::QueueIdentityAdmission
                })
                .expect("preview queue row");
            row.job_identities[0]
                .budget_domain_refs
                .push(HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF.to_owned());
        }
        other => panic!("unknown mutation: {other}"),
    }

    let findings = packet.validate();
    packet.validation_findings = findings.clone();
    packet.promotion_state = if findings
        .iter()
        .any(|finding| finding.severity == aureline_runtime::GovernanceFindingSeverity::Blocker)
    {
        GovernancePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == aureline_runtime::GovernanceFindingSeverity::Warning)
    {
        GovernancePromotionState::NarrowedBelowStable
    } else {
        GovernancePromotionState::Stable
    };
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "queue_session_terminal_governance_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let mut packet = current_queue_session_terminal_governance_packet();
    apply_mutation(&mut packet, &fixture.mutation);

    let findings = packet.validate();
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        fixture.expect.promotion_state,
        packet.promotion_state
    );
    assert_eq!(
        packet.rows.len(),
        fixture.expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift",
        fixture.case_name
    );
    assert_token_set_matches(
        &packet.workload_tokens(),
        &fixture.expect.workload_tokens,
        "workload",
    );
    assert_token_set_matches(
        &packet.queue_lane_tokens(),
        &fixture.expect.queue_lane_tokens,
        "queue_lane",
    );
    assert_token_set_matches(
        &packet.job_kind_tokens(),
        &fixture.expect.job_kind_tokens,
        "job_kind",
    );
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &fixture.expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.restore_fidelity_tokens(),
        &fixture.expect.restore_fidelity_tokens,
        "restore_fidelity",
    );
    assert_token_set_matches(
        &packet.terminal_boundary_tokens(),
        &fixture.expect.terminal_boundary_tokens,
        "terminal_boundary",
    );
    assert_token_set_matches(
        &packet.clipboard_posture_tokens(),
        &fixture.expect.clipboard_posture_tokens,
        "clipboard_posture",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-12T00:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !fixture.expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &fixture.expect.expected_finding_kinds {
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
    assert_exists(QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF);
    assert_exists(QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF);
    assert_exists(QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF);
    assert_exists(QUEUE_SESSION_TERMINAL_GOVERNANCE_FIXTURE_DIR);
    assert_exists(BACKGROUND_QUEUE_CONTRACT_DOC_REF);
    assert_exists(CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn removing_queue_identity_blocks_stable() {
    assert_fixture_matches("remove_notebook_queue_identity_row.json");
}

#[test]
fn removing_terminal_boundary_blocks_stable() {
    assert_fixture_matches("remove_incident_terminal_boundary_row.json");
}

#[test]
fn narrowed_row_without_disclosure_blocks_stable() {
    assert_fixture_matches("narrow_preview_quality_without_disclosure.json");
}

#[test]
fn projection_vocabulary_collapse_blocks_stable() {
    assert_fixture_matches("collapse_terminal_boundary_projection_vocabulary.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("expose_raw_source_material.json");
}

#[test]
fn protected_hot_path_budget_cannot_be_consumed() {
    assert_fixture_matches("consume_hot_path_budget.json");
}

#[test]
fn checked_in_packet_validates_and_covers_every_required_class() {
    let packet = current_queue_session_terminal_governance_packet();
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "checked-in packet must validate cleanly: {findings:#?}"
    );
    assert_eq!(packet.covered_workloads.len(), 10);
    assert_eq!(packet.rows.len(), 47);
    let observed_job_kinds: BTreeSet<&str> = packet.job_kind_tokens().into_iter().collect();
    for job_kind in GovernedJobKind::REQUIRED {
        assert!(
            observed_job_kinds.contains(job_kind.as_str()),
            "job kind {} must be covered",
            job_kind.as_str()
        );
    }
    for workload in GovernedWorkloadClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.workload_class == workload),
            "workload {} must be covered",
            workload.as_str()
        );
    }
    assert!(packet
        .validation_findings
        .iter()
        .all(|finding| finding.finding_kind != GovernanceFindingKind::PromotionStateMismatch));
}

#[test]
fn checked_in_packet_ships_activity_rows_and_scheduler_lane_rows() {
    let packet = current_queue_session_terminal_governance_packet();
    assert_eq!(packet.activity_job_rows.len(), 10);
    assert_eq!(packet.scheduler_lane_rows.len(), 5);

    let observed_states: BTreeSet<&str> = packet.activity_state_tokens().into_iter().collect();
    for state in ActivityJobStateClass::REQUIRED {
        assert!(
            observed_states.contains(state.as_str()),
            "activity state {} must be covered",
            state.as_str()
        );
    }
    for workload in GovernedWorkloadClass::REQUIRED {
        assert!(
            packet
                .activity_job_rows
                .iter()
                .any(|row| row.workload_class == workload),
            "activity row coverage missing for workload {}",
            workload.as_str()
        );
    }
    for scheduler_row in &packet.scheduler_lane_rows {
        assert!(
            !scheduler_row.activity_row_refs.is_empty(),
            "scheduler lane {} must reference activity rows",
            scheduler_row.queue_lane_token
        );
        assert!(
            !scheduler_row.inspect_ref.is_empty(),
            "scheduler lane {} must carry an inspect ref",
            scheduler_row.queue_lane_token
        );
    }
    for activity_row in &packet.activity_job_rows {
        assert!(
            !activity_row.job_identity_refs.is_empty(),
            "activity row {} must cite at least one governed job id",
            activity_row.row_id
        );
        assert!(
            !activity_row.exact_target_reopen_ref.is_empty()
                && !activity_row.exact_target_identity_ref.is_empty()
                && !activity_row.inspect_ref.is_empty()
                && !activity_row.next_action_ref.is_empty(),
            "activity row {} must carry exact reopen, inspect, and next-action refs",
            activity_row.row_id
        );
    }
}
