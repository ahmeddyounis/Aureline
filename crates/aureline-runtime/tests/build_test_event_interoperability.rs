//! Fixture-driven coverage for stable build/test event interoperability across
//! native, BSP, Bazel BEP, structured output, heuristic parser, replay, AI,
//! review, release, and support-export consumers.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_build_test_event_interoperability_input, BuildTestEventInteroperabilityPacket,
    BuildTestEventInteroperabilityPacketInput, BuildTestInteropFindingKind,
    BUILD_TEST_EVENT_INTEROPERABILITY_ARTIFACT_DOC_REF, BUILD_TEST_EVENT_INTEROPERABILITY_DOC_REF,
    BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR,
    BUILD_TEST_EVENT_INTEROPERABILITY_PACKET_ARTIFACT_REF,
    BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InteropFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: FixtureMutation,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FixtureMutation {
    None,
    DropRawPayloadRef,
    HeuristicOverclaimsConfidence,
    DropCapabilityNegotiation,
    ConsumerDropsConfidence,
    ExportRawPrivateMaterial,
    DropTargetGraphReady,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    source_kind_tokens: Vec<String>,
    event_kind_tokens: Vec<String>,
    consumer_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> InteropFixture {
    let path = repo_root()
        .join(BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn mutated_input(mutation: &FixtureMutation) -> BuildTestEventInteroperabilityPacketInput {
    let mut input = current_stable_build_test_event_interoperability_input();
    match mutation {
        FixtureMutation::None => {}
        FixtureMutation::DropRawPayloadRef => {
            input
                .raw_payload_refs
                .retain(|raw| raw.raw_payload_ref != "raw:heuristic:diagnostic");
        }
        FixtureMutation::HeuristicOverclaimsConfidence => {
            let heuristic = input
                .events
                .iter_mut()
                .find(|event| event.raw_payload_ref == "raw:heuristic:diagnostic")
                .expect("stable input contains heuristic diagnostic event");
            heuristic.confidence = aureline_runtime::BuildTestInteropConfidence::MediumHigh;
            heuristic.downgraded = false;
        }
        FixtureMutation::DropCapabilityNegotiation => {
            input.capability_negotiations.retain(|row| {
                !(row.lane == aureline_runtime::BuildTestInteropLane::ImportedProvider
                    && row.source_kind == aureline_runtime::BuildTestInteropSourceKind::BazelBep)
            });
        }
        FixtureMutation::ConsumerDropsConfidence => {
            let projection = input
                .consumer_projections
                .iter_mut()
                .find(|projection| {
                    projection.consumer_surface
                        == aureline_runtime::BuildTestConsumerSurface::AiExplanation
                })
                .expect("stable input contains AI explanation projection");
            projection.preserves_confidence = false;
        }
        FixtureMutation::ExportRawPrivateMaterial => {
            input.replay_export_parity.raw_private_material_excluded = false;
        }
        FixtureMutation::DropTargetGraphReady => {
            input.events.retain(|event| {
                event.event_kind != aureline_runtime::BuildTestInteropEventKind::TargetGraphReady
            });
        }
    }
    input
}

fn assert_token_set_matches(observed: Vec<&str>, expected: &[String], label: &str) {
    let mut observed = observed;
    observed.sort_unstable();
    let mut expected: Vec<&str> = expected.iter().map(String::as_str).collect();
    expected.sort_unstable();
    assert_eq!(observed, expected, "{label} token set drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "build_test_event_interoperability_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let packet =
        BuildTestEventInteroperabilityPacket::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state
    );
    assert_eq!(
        packet.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    for expected_kind in &fixture.expect.expected_finding_kinds {
        assert!(
            packet
                .validation_findings
                .iter()
                .any(|finding| finding.finding_kind.as_str() == expected_kind),
            "fixture {} expected finding kind {}",
            fixture.case_name,
            expected_kind
        );
    }
    assert_token_set_matches(
        packet.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_set_matches(
        packet.event_kind_tokens(),
        &fixture.expect.event_kind_tokens,
        "event kind",
    );
    assert_token_set_matches(
        packet.consumer_surface_tokens(),
        &fixture.expect.consumer_surface_tokens,
        "consumer surface",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-07T00:01:00Z",
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
    assert_exists(BUILD_TEST_EVENT_INTEROPERABILITY_SCHEMA_REF);
    assert_exists(BUILD_TEST_EVENT_INTEROPERABILITY_DOC_REF);
    assert_exists(BUILD_TEST_EVENT_INTEROPERABILITY_ARTIFACT_DOC_REF);
    assert_exists(BUILD_TEST_EVENT_INTEROPERABILITY_FIXTURE_DIR);
    assert_exists(BUILD_TEST_EVENT_INTEROPERABILITY_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_packet_validates_stable() {
    let path = repo_root().join(BUILD_TEST_EVENT_INTEROPERABILITY_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let packet: BuildTestEventInteroperabilityPacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"));
    assert!(
        packet.validate().is_empty(),
        "checked-in packet must validate without findings"
    );
    assert_eq!(packet.promotion_state.as_str(), "stable");
}

#[test]
fn closed_finding_tokens_are_pinned() {
    assert_eq!(
        BuildTestInteropFindingKind::MissingCapabilityNegotiation.as_str(),
        "missing_capability_negotiation"
    );
    assert_eq!(
        BuildTestInteropFindingKind::HeuristicOverclaimsConfidence.as_str(),
        "heuristic_overclaims_confidence"
    );
    assert_eq!(
        BuildTestInteropFindingKind::ConsumerProjectionDrift.as_str(),
        "consumer_projection_drift"
    );
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn missing_raw_payload_reference_blocks_stable() {
    assert_fixture_matches("missing_raw_payload_reference_blocks_stable.json");
}

#[test]
fn heuristic_overclaim_blocks_stable() {
    assert_fixture_matches("heuristic_overclaim_blocks_stable.json");
}

#[test]
fn missing_capability_negotiation_blocks_stable() {
    assert_fixture_matches("missing_capability_negotiation_blocks_stable.json");
}

#[test]
fn consumer_confidence_drift_blocks_stable() {
    assert_fixture_matches("consumer_confidence_drift_blocks_stable.json");
}

#[test]
fn export_raw_private_material_blocks_stable() {
    assert_fixture_matches("export_raw_private_material_blocks_stable.json");
}

#[test]
fn missing_target_graph_ready_blocks_stable() {
    assert_fixture_matches("missing_target_graph_ready_blocks_stable.json");
}
