//! Fixture-driven coverage for the M5 cross-surface event-reuse packet: one
//! shared execution history bound to every major M5 consumer, the reopen /
//! export / rerun-review / evidence-link flows that point back to the same
//! authoritative event objects, and the fail-closed guardrails against forked,
//! log-reconstructed, id-rewritten, or provenance-flattening surfaces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_cross_surface_event_reuse_input, ConsumerSurface, CrossSurfaceEventReusePacket,
    CrossSurfaceEventReusePacketInput, CrossSurfaceEventReuseSupportExport,
    CrossSurfaceEvidenceJoinView, CrossSurfaceFlowKind, ReuseEvidenceSurface,
    CROSS_SURFACE_EVENT_REUSE_DOC_REF, CROSS_SURFACE_EVENT_REUSE_ENVELOPE_SCHEMA_REF,
    CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR, CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF,
    CROSS_SURFACE_EVENT_REUSE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReuseFixture {
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
    consumer_surface_tokens: Vec<String>,
    flow_kind_tokens: Vec<String>,
    source_kind_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ReuseFixture {
    let path = repo_root()
        .join(CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

/// Mirrors the mutations applied by the `dump_m5_cross_surface_event_reuse`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> CrossSurfaceEventReusePacketInput {
    let mut input = current_stable_cross_surface_event_reuse_input();
    match mutation {
        "none" => {}
        "consumer_reconstructs_from_logs" => {
            for binding in &mut input.consumer_bindings {
                if binding.surface == ConsumerSurface::CoverageFlakySnapshot {
                    binding.reconstructs_from_logs = true;
                }
            }
        }
        "consumer_forks_history" => {
            for binding in &mut input.consumer_bindings {
                if binding.surface == ConsumerSurface::TestTree {
                    binding.reads_shared_history = false;
                }
            }
        }
        "consumer_rewrites_ids" => {
            for binding in &mut input.consumer_bindings {
                if binding.surface == ConsumerSurface::NotebookRun {
                    binding.preserves_stable_ids = false;
                }
            }
        }
        "consumer_binding_missing" => {
            input
                .consumer_bindings
                .retain(|binding| binding.surface != ConsumerSurface::NotebookRun);
        }
        "flow_target_missing" => {
            for flow in &mut input.cross_surface_flows {
                if flow.flow_kind == CrossSurfaceFlowKind::Reopen {
                    flow.authoritative_event_id = "event:does-not-exist".to_owned();
                }
            }
        }
        "flow_drops_provenance" => {
            for flow in &mut input.cross_surface_flows {
                if flow.flow_kind == CrossSurfaceFlowKind::EvidenceLink {
                    flow.preserves_provenance = false;
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
        fixture.record_kind, "m5_cross_surface_event_reuse_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let packet = CrossSurfaceEventReusePacket::materialize(mutated_input(&fixture.mutation));
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
        packet.consumer_surface_tokens(),
        &fixture.expect.consumer_surface_tokens,
        "consumer surface",
    );
    assert_token_list(
        packet.flow_kind_tokens(),
        &fixture.expect.flow_kind_tokens,
        "flow kind",
    );
    assert_token_list(
        packet.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
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
    assert_exists(CROSS_SURFACE_EVENT_REUSE_SCHEMA_REF);
    assert_exists(CROSS_SURFACE_EVENT_REUSE_ENVELOPE_SCHEMA_REF);
    assert_exists(CROSS_SURFACE_EVENT_REUSE_DOC_REF);
    assert_exists(CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR);
    assert_exists(CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_packet_validates_clean() {
    let path = repo_root().join(CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let packet: CrossSurfaceEventReusePacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"));
    assert!(
        packet.validate().is_empty(),
        "checked-in packet must validate without findings: {:?}",
        packet.validate()
    );
    assert_eq!(packet.promotion_state.as_str(), "stable");
}

#[test]
fn checked_in_artifacts_match_the_seed() {
    let packet =
        CrossSurfaceEventReusePacket::materialize(current_stable_cross_surface_event_reuse_input());

    let packet_path = repo_root().join(CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF);
    let on_disk: CrossSurfaceEventReusePacket =
        serde_json::from_str(&std::fs::read_to_string(&packet_path).expect("read packet"))
            .expect("parse packet");
    assert_eq!(on_disk, packet, "checked-in packet drifted from the seed");

    let support_path = packet_path.with_file_name("support_export.json");
    let support: CrossSurfaceEventReuseSupportExport =
        serde_json::from_str(&std::fs::read_to_string(&support_path).expect("read support export"))
            .expect("parse support export");
    assert!(support.is_export_safe());
    assert_eq!(support.packet, packet);

    for (file_name, surface) in [
        ("ai_evidence.json", ReuseEvidenceSurface::AiEvidence),
        ("incident_packet.json", ReuseEvidenceSurface::IncidentPacket),
    ] {
        let path = packet_path.with_file_name(file_name);
        let view: CrossSurfaceEvidenceJoinView =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read evidence join"))
                .expect("parse evidence join");
        assert_eq!(view.surface, surface);
        assert!(
            view.explains_consistently(),
            "{file_name} must explain consistently"
        );
        assert_eq!(view.shared_history_digest, packet.shared_history_digest);
        assert_eq!(view.shared_event_rows.len(), packet.events.len());
        assert_eq!(view.flow_rows.len(), packet.cross_surface_flows.len());
    }
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn consumer_reconstructs_from_logs_blocks_stable() {
    assert_fixture_matches("consumer_reconstructs_from_logs_blocks_stable.json");
}

#[test]
fn consumer_forks_history_blocks_stable() {
    assert_fixture_matches("consumer_forks_history_blocks_stable.json");
}

#[test]
fn consumer_rewrites_ids_blocks_stable() {
    assert_fixture_matches("consumer_rewrites_ids_blocks_stable.json");
}

#[test]
fn consumer_binding_missing_blocks_stable() {
    assert_fixture_matches("consumer_binding_missing_blocks_stable.json");
}

#[test]
fn flow_target_missing_blocks_stable() {
    assert_fixture_matches("flow_target_missing_blocks_stable.json");
}

#[test]
fn flow_drops_provenance_blocks_stable() {
    assert_fixture_matches("flow_drops_provenance_blocks_stable.json");
}
