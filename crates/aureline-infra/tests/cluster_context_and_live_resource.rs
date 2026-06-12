//! Integration coverage for cluster-context and live-resource packets.

use std::fs;
use std::path::PathBuf;

use aureline_infra::{
    ClusterLiveResourcePacket, OpsToolKind, TruthMode, CLUSTER_LIVE_RESOURCE_PACKET_RECORD_KIND,
    CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION,
};

#[test]
fn qualified_fixture_validates() {
    let packet = load_fixture("qualified_cluster_context_packet.json");
    let report = packet.validate();
    assert!(report.passed, "fixture must pass: {:#?}", report.findings);
    assert_eq!(packet.record_kind, CLUSTER_LIVE_RESOURCE_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION);
}

#[test]
fn qualified_fixture_covers_truth_modes_and_tools() {
    let report = load_fixture("qualified_cluster_context_packet.json").validate();
    for required in [
        TruthMode::Desired,
        TruthMode::Rendered,
        TruthMode::Plan,
        TruthMode::Live,
        TruthMode::ProviderOverlay,
    ] {
        assert!(report.truth_modes.contains(&required));
    }
    for required in [
        OpsToolKind::Terraform,
        OpsToolKind::Kubernetes,
        OpsToolKind::IncidentAdjacent,
    ] {
        assert!(report.tool_kinds.contains(&required));
    }
}

#[test]
fn stale_fixture_passes_with_no_mutating_views() {
    let packet = load_fixture("stale_live_downgraded_packet.json");
    let report = packet.validate();
    assert!(
        report.passed,
        "stale fixture must pass: {:#?}",
        report.findings
    );
    for projection in &packet.surface_projections {
        for view in &projection.truth_mode_views {
            assert!(
                !view.mutation_capable,
                "stale packet must not expose a mutation-capable view"
            );
        }
    }
}

#[test]
fn wrong_target_and_blended_drill_fails() {
    let report = load_fixture("wrong_target_blended_view_packet.json").validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "strip_identity"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "blended_truth_mode"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "gate_preview_or_handoff"));
}

fn load_fixture(name: &str) -> ClusterLiveResourcePacket {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/infra/cluster-context-and-live-resource")
        .join(name);
    let payload = fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&payload).expect("fixture parses")
}
