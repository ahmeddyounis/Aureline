//! Fixture-driven coverage for the M5 task-event first-consumers packet:
//! the canonical record history, the replay-stable trace summaries, and the
//! seven consumer-surface projections that keep every claimed M5 execution
//! surface off log-only event truth.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_task_event_first_consumers_input, BuildTestInteropConfidence,
    BuildTestInteropPayloadKind, BuildTestInteropSourceKind, TaskEventFirstConsumersPacket,
    TaskEventFirstConsumersPacketInput, TaskEventSurface, TASK_EVENT_FIRST_CONSUMERS_DOC_REF,
    TASK_EVENT_FIRST_CONSUMERS_ENVELOPE_SCHEMA_REF, TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR,
    TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF, TASK_EVENT_FIRST_CONSUMERS_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PacketFixture {
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
    expected_finding_kinds: Vec<String>,
    surface_tokens: Vec<String>,
    source_kind_tokens: Vec<String>,
    payload_kind_tokens: Vec<String>,
    support_export_safe: bool,
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

fn load_fixture(file_name: &str) -> PacketFixture {
    let path = repo_root()
        .join(TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

/// Mirrors the mutations applied by the `dump_m5_task_event_envelope_bus`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> TaskEventFirstConsumersPacketInput {
    let mut input = current_stable_task_event_first_consumers_input();
    match mutation {
        "none" => {}
        "lane_missing_test_session" => {
            input
                .events
                .retain(|event| event.producer_lane != TaskEventSurface::TestSession);
        }
        "heuristic_overclaims" => {
            for event in &mut input.events {
                if event.source_kind == BuildTestInteropSourceKind::HeuristicParser {
                    event.confidence = BuildTestInteropConfidence::High;
                }
            }
        }
        "payload_kind_mismatch" => {
            for event in &mut input.events {
                if event.event_id == "event:test:finished" {
                    event.payload_kind = BuildTestInteropPayloadKind::Lifecycle;
                }
            }
        }
        "sequence_collision" => {
            for event in &mut input.events {
                if event.event_id == "event:task:progress" {
                    event.sequence = 2;
                }
            }
        }
        "downgrade_inconsistent" => {
            for event in &mut input.events {
                if event.event_id == "event:pipeline:diagnostic-shadow" {
                    event.downgraded = false;
                }
            }
        }
        "projection_drops_truth" => {
            for projection in &mut input.surface_projections {
                if projection.surface == TaskEventSurface::Pipeline {
                    projection.preserves_confidence = false;
                }
            }
        }
        "export_cannot_explain" => {
            for projection in &mut input.surface_projections {
                if projection.surface == TaskEventSurface::SupportExport {
                    projection.explains_source_and_confidence = false;
                }
            }
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn assert_token_list(observed: Vec<&str>, expected: &[String], label: &str) {
    let observed: Vec<String> = observed.into_iter().map(str::to_owned).collect();
    assert_eq!(&observed, expected, "{label} token drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "m5_task_event_first_consumers_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let packet = TaskEventFirstConsumersPacket::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    let observed_kinds: Vec<&str> = packet
        .validation_findings
        .iter()
        .map(|f| f.finding_kind.as_str())
        .collect();
    assert_eq!(
        observed_kinds,
        fixture
            .expect
            .expected_finding_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} finding kinds drift",
        fixture.case_name
    );
    assert_token_list(
        packet.surface_tokens(),
        &fixture.expect.surface_tokens,
        "surface",
    );
    assert_token_list(
        packet.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_list(
        packet.payload_kind_tokens(),
        &fixture.expect.payload_kind_tokens,
        "payload kind",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-17T00:01:00Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(TASK_EVENT_FIRST_CONSUMERS_SCHEMA_REF);
    assert_exists(TASK_EVENT_FIRST_CONSUMERS_ENVELOPE_SCHEMA_REF);
    assert_exists(TASK_EVENT_FIRST_CONSUMERS_DOC_REF);
    assert_exists(TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR);
    assert_exists(TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_packet_validates_clean() {
    let path = repo_root().join(TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let packet: TaskEventFirstConsumersPacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"));
    assert!(
        packet.validate().is_empty(),
        "checked-in packet must validate without findings: {:?}",
        packet.validate()
    );
    assert_eq!(packet.promotion_state.as_str(), "stable");
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn lane_missing_events_blocks_stable() {
    assert_fixture_matches("lane_missing_events_blocks_stable.json");
}

#[test]
fn heuristic_overclaims_blocks_stable() {
    assert_fixture_matches("heuristic_overclaims_blocks_stable.json");
}

#[test]
fn payload_kind_mismatch_blocks_stable() {
    assert_fixture_matches("payload_kind_mismatch_blocks_stable.json");
}

#[test]
fn replay_sequence_collision_blocks_stable() {
    assert_fixture_matches("replay_sequence_collision_blocks_stable.json");
}

#[test]
fn downgrade_inconsistent_blocks_stable() {
    assert_fixture_matches("downgrade_inconsistent_blocks_stable.json");
}

#[test]
fn projection_drops_truth_blocks_stable() {
    assert_fixture_matches("projection_drops_truth_blocks_stable.json");
}

#[test]
fn export_cannot_explain_blocks_stable() {
    assert_fixture_matches("export_cannot_explain_blocks_stable.json");
}
