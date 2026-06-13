//! Protected tests for the M5 support-side forensic packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_m5_forensic_packet, ForensicArtifactStateClass, ForensicShareDestinationClass,
    M5ForensicPacket, M5_FORENSIC_PACKET_ARTIFACT_REF, M5_FORENSIC_PACKET_DOC_REF,
    M5_FORENSIC_PACKET_FIXTURE_DIR, M5_FORENSIC_PACKET_RECORD_KIND, M5_FORENSIC_PACKET_SCHEMA_REF,
    M5_FORENSIC_PACKET_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_FORENSIC_PACKET_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5ForensicPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_validates_and_covers_every_claimed_host_family() {
    let packet = seeded_m5_forensic_packet();
    assert_eq!(packet.record_kind, M5_FORENSIC_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_FORENSIC_PACKET_SCHEMA_VERSION);
    assert_eq!(packet.doc_ref, M5_FORENSIC_PACKET_DOC_REF);
    assert_eq!(packet.schema_ref, M5_FORENSIC_PACKET_SCHEMA_REF);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for host in [
        "notebook_kernel_host",
        "data_api_connector_host",
        "preview_dev_server_host",
        "provider_run_session_host",
        "profiler_replay_session_host",
        "pipeline_viewer_host",
        "query_runtime_host",
        "docs_browser_bridge_host",
        "registry_database_connector_host",
        "infra_helper_job",
    ] {
        assert!(
            packet.rows.iter().any(|row| row.host_family_id == host),
            "missing forensic row for {host}",
        );
    }
}

#[test]
fn seeded_packet_distinguishes_local_imported_mirrored_and_uploaded_states() {
    let packet = seeded_m5_forensic_packet();
    let states = packet
        .rows
        .iter()
        .flat_map(|row| {
            row.artifact_states
                .iter()
                .map(|artifact| artifact.state_class)
        })
        .collect::<Vec<_>>();
    for state in ForensicArtifactStateClass::ALL {
        assert!(
            states.contains(&state),
            "missing artifact state {}",
            state.as_str()
        );
    }
}

#[test]
fn seeded_packet_keeps_local_preview_ahead_of_egress() {
    let packet = seeded_m5_forensic_packet();
    for row in &packet.rows {
        assert!(row
            .share_actions
            .iter()
            .any(
                |action| action.destination_class == ForensicShareDestinationClass::LocalPreview
                    && action.enabled
                    && !action.network_egress
            ));
        assert!(row.share_actions.iter().all(|action| {
            !action.network_egress || action.explicit_user_or_policy_action_required
        }));
    }
}

#[test]
fn packet_round_trips_and_is_export_safe() {
    let packet = seeded_m5_forensic_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let round: M5ForensicPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(round, packet);
    assert!(packet.is_export_safe());
    assert!(!json.contains("/Users/"));
    assert!(!json.contains("BEGIN PRIVATE KEY"));
}

#[test]
fn docs_schema_artifact_and_fixture_files_exist() {
    let root = repo_root();
    for rel in [
        M5_FORENSIC_PACKET_SCHEMA_REF,
        M5_FORENSIC_PACKET_DOC_REF,
        M5_FORENSIC_PACKET_ARTIFACT_REF,
        "fixtures/support/m5/forensic_packets/README.md",
        "fixtures/support/m5/forensic_packets/manifest.yaml",
        "fixtures/support/m5/forensic_packets/packet.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    let fixture = load_fixture("packet.json");
    let seeded = seeded_m5_forensic_packet();
    assert_eq!(fixture, seeded);
}
