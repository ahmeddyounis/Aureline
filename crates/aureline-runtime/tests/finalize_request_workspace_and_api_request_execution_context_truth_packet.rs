//! Fixture-driven coverage for the stable request-workspace and
//! API-request execution-context reuse truth packet covering the
//! request_workspace_lane, api_request_lane, response_trust_lane, and
//! data_action_lane plus the four-wedge admission coverage, the six
//! auth-source-mode admissions, the six connection-state admissions,
//! the eight streaming-response-state admissions, the eight
//! consumer-surface bindings, and the lineage_admission row binding
//! `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_request_execution_context_truth_packet,
    RequestExecutionAuthSourceModeClass, RequestExecutionConnectionStateClass,
    RequestExecutionConsumerProjectionSurface, RequestExecutionConsumerSurfaceClass,
    RequestExecutionContextTruthPacket, RequestExecutionContextTruthPacketInput,
    RequestExecutionLaneClass, RequestExecutionPromotionState, RequestExecutionRowClass,
    RequestExecutionStreamingResponseStateClass, RequestExecutionSupportClass,
    RequestExecutionWedgeClass, REQUEST_EXECUTION_CONTEXT_TRUTH_ARTIFACT_DOC_REF,
    REQUEST_EXECUTION_CONTEXT_TRUTH_DOC_REF, REQUEST_EXECUTION_CONTEXT_TRUTH_FIXTURE_DIR,
    REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_ARTIFACT_REF,
    REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RequestExecutionFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: RequestExecutionContextTruthPacketInput,
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
    auth_source_tokens: Vec<String>,
    connection_state_tokens: Vec<String>,
    streaming_response_state_tokens: Vec<String>,
    consumer_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> RequestExecutionFixture {
    let path = repo_root()
        .join(REQUEST_EXECUTION_CONTEXT_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind,
        "finalize_request_workspace_and_api_request_execution_context_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = RequestExecutionContextTruthPacket::materialize(fixture.input.clone());
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
        &packet.auth_source_tokens(),
        &expect.auth_source_tokens,
        "auth_source",
    );
    assert_token_set_matches(
        &packet.connection_state_tokens(),
        &expect.connection_state_tokens,
        "connection_state",
    );
    assert_token_set_matches(
        &packet.streaming_response_state_tokens(),
        &expect.streaming_response_state_tokens,
        "streaming_response_state",
    );
    assert_token_set_matches(
        &packet.consumer_surface_tokens(),
        &expect.consumer_surface_tokens,
        "consumer_surface",
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
        "2026-05-27T12:00:10Z",
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
    assert_exists(REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_REF);
    assert_exists(REQUEST_EXECUTION_CONTEXT_TRUTH_DOC_REF);
    assert_exists(REQUEST_EXECUTION_CONTEXT_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(REQUEST_EXECUTION_CONTEXT_TRUTH_FIXTURE_DIR);
    assert_exists(REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_auth_source_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_auth_source_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_streaming_state_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_streaming_state_for_launch_stable_blocks_stable.json");
}

#[test]
fn approval_review_truth_without_attestation_blocks_stable() {
    assert_fixture_matches("approval_review_truth_without_attestation_blocks_stable.json");
}

#[test]
fn silent_deferred_queue_admitted_blocks_stable() {
    assert_fixture_matches("silent_deferred_queue_admitted_blocks_stable.json");
}

#[test]
fn lineage_admission_missing_execution_context_id_blocks_stable() {
    assert_fixture_matches("lineage_admission_missing_execution_context_id_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_streaming_response_state_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_streaming_response_state_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_request_execution_context_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, RequestExecutionPromotionState::Stable);
    assert!(packet.validate().is_empty());
    for required in RequestExecutionLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for execution-context lane {}",
            required.as_str()
        );
    }
    for surface in RequestExecutionConsumerProjectionSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_wedges_sources_states_and_admissions_per_launch_stable_lane() {
    let packet = current_stable_request_execution_context_truth_packet()
        .expect("checked-in packet validates");
    for required in RequestExecutionLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == RequestExecutionRowClass::ExecutionContextReuseQuality
                && row.support_class == RequestExecutionSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in RequestExecutionWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == RequestExecutionRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover wedge {} on lane {}",
                wedge.as_str(),
                required.as_str()
            );
        }
        for mode in RequestExecutionAuthSourceModeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == RequestExecutionRowClass::AuthSourceAdmission
                    && row.auth_source_mode == mode),
                "stable packet must cover auth source mode {} on lane {}",
                mode.as_str(),
                required.as_str()
            );
        }
        for state in RequestExecutionConnectionStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == RequestExecutionRowClass::ConnectionStateAdmission
                    && row.connection_state_class == state),
                "stable packet must cover connection state {} on lane {}",
                state.as_str(),
                required.as_str()
            );
        }
        for state in RequestExecutionStreamingResponseStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == RequestExecutionRowClass::StreamingResponseStateAdmission
                    && row.streaming_response_state_class == state),
                "stable packet must cover streaming response state {} on lane {}",
                state.as_str(),
                required.as_str()
            );
        }
        for surface in RequestExecutionConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == RequestExecutionRowClass::ConsumerSurfaceBinding
                    && row.consumer_surface_class == surface),
                "stable packet must cover consumer surface {} on lane {}",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RequestExecutionRowClass::LineageAdmission
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)),
            "stable packet must include a lineage_admission row binding execution_context_id on lane {}",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RequestExecutionRowClass::WedgeAdmission
                && row.wedge_class == RequestExecutionWedgeClass::ApprovalReviewTruth
                && row.approval_review_attested),
            "stable packet must include an approval_review_truth wedge_admission attesting mutation-review enforcement on lane {}",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == RequestExecutionRowClass::ConnectionStateAdmission
                && row.connection_state_class == RequestExecutionConnectionStateClass::ReconciliationPending
                && row.silent_deferred_queue_blocked),
            "stable packet must include a reconciliation_pending connection_state_admission row attesting silent-deferred-queue blocking on lane {}",
            required.as_str()
        );
    }
}
