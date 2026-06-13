//! Protected tests for the M5 host-failure drill packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_m5_host_failure_drill_packet, HostFailureScenarioClass, M5HostFailureDrillPacket,
    M5_HOST_FAILURE_DRILL_ARTIFACT_REF, M5_HOST_FAILURE_DRILL_DOC_REF,
    M5_HOST_FAILURE_DRILL_FIXTURE_DIR, M5_HOST_FAILURE_DRILL_PACKET_RECORD_KIND,
    M5_HOST_FAILURE_DRILL_SCHEMA_REF, M5_HOST_FAILURE_DRILL_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_HOST_FAILURE_DRILL_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5HostFailureDrillPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_validates_and_covers_required_scenarios() {
    let packet = seeded_m5_host_failure_drill_packet();
    assert_eq!(packet.record_kind, M5_HOST_FAILURE_DRILL_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_HOST_FAILURE_DRILL_SCHEMA_VERSION);
    assert_eq!(packet.doc_ref, M5_HOST_FAILURE_DRILL_DOC_REF);
    assert_eq!(packet.schema_ref, M5_HOST_FAILURE_DRILL_SCHEMA_REF);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for scenario in HostFailureScenarioClass::ALL {
        assert!(
            packet
                .drills
                .iter()
                .any(|row| row.scenario_class == scenario),
            "missing drill scenario {}",
            scenario.as_str()
        );
    }
}

#[test]
fn seeded_packet_proves_scope_checkpoint_and_no_hidden_rerun() {
    let packet = seeded_m5_host_failure_drill_packet();
    for row in &packet.drills {
        assert!(row.restart_budget_enforced);
        assert!(row.scoped_failure_only);
        assert!(row.checkpoint_preserved);
        assert!(row.no_hidden_rerun);
        assert!(row.visible_fail_closed_state);
        assert!(!row.upload_guard_assertions.is_empty());
    }
}

#[test]
fn packet_round_trips_and_is_export_safe() {
    let packet = seeded_m5_host_failure_drill_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let round: M5HostFailureDrillPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(round, packet);
    assert!(packet.is_export_safe());
    assert!(!json.contains("/Users/"));
    assert!(!json.contains("BEGIN PRIVATE KEY"));
}

#[test]
fn docs_schema_artifact_and_fixture_files_exist() {
    let root = repo_root();
    for rel in [
        M5_HOST_FAILURE_DRILL_SCHEMA_REF,
        M5_HOST_FAILURE_DRILL_DOC_REF,
        M5_HOST_FAILURE_DRILL_ARTIFACT_REF,
        "fixtures/support/m5/host_failure_drills/README.md",
        "fixtures/support/m5/host_failure_drills/manifest.yaml",
        "fixtures/support/m5/host_failure_drills/packet.json",
        "fixtures/support/m5/host_failure_drills/notebook_kernel_crash_stall.json",
        "fixtures/support/m5/host_failure_drills/provider_run_failure.json",
        "fixtures/support/m5/host_failure_drills/preview_server_restart.json",
        "fixtures/support/m5/host_failure_drills/remote_connector_drift.json",
        "fixtures/support/m5/host_failure_drills/ai_broker_circuit_breaker.json",
        "fixtures/support/m5/host_failure_drills/query_runtime_crash.json",
        "fixtures/support/m5/host_failure_drills/pipeline_viewer_fault.json",
        "fixtures/support/m5/host_failure_drills/connector_host_mismatch.json",
        "fixtures/support/m5/host_failure_drills/docs_browser_bridge_route_drift.json",
        "fixtures/support/m5/host_failure_drills/profiler_replay_imported_gap.json",
        "fixtures/support/m5/host_failure_drills/infra_helper_signature_failure.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    let fixture = load_fixture("packet.json");
    let seeded = seeded_m5_host_failure_drill_packet();
    assert_eq!(fixture, seeded);
}
