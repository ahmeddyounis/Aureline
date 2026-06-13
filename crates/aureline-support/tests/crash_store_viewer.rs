//! Protected tests for the M5 local crash-store viewer packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_crash::SymbolicationState;
use aureline_support::{
    seeded_crash_store_viewer_packet, seeded_expired_dump_crash_store_viewer_packet,
    CrashPreservationClass, CrashStoreActionClass, CrashStoreViewerPacket,
    CRASH_STORE_VIEWER_ARTIFACT_REF, CRASH_STORE_VIEWER_DOC_REF, CRASH_STORE_VIEWER_FIXTURE_DIR,
    CRASH_STORE_VIEWER_PACKET_RECORD_KIND, CRASH_STORE_VIEWER_SCHEMA_REF,
    CRASH_STORE_VIEWER_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(CRASH_STORE_VIEWER_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> CrashStoreViewerPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_covers_required_m5_crash_families() {
    let packet = seeded_crash_store_viewer_packet();
    assert_eq!(packet.record_kind, CRASH_STORE_VIEWER_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, CRASH_STORE_VIEWER_SCHEMA_VERSION);
    assert_eq!(packet.doc_ref, CRASH_STORE_VIEWER_DOC_REF);
    assert_eq!(packet.schema_ref, CRASH_STORE_VIEWER_SCHEMA_REF);

    for host in [
        "notebook_kernel_host",
        "preview_dev_server_host",
        "provider_run_session_host",
        "profiler_replay_session_host",
        "pipeline_viewer_host",
        "query_runtime_host",
    ] {
        assert!(
            packet.rows.iter().any(|row| row.host_family_id == host),
            "missing crash-store row for {host}",
        );
    }
}

#[test]
fn seeded_packet_keeps_local_preview_and_export_before_upload() {
    let packet = seeded_crash_store_viewer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());

    for row in &packet.rows {
        assert!(row.local_first_by_default);
        assert!(!row.raw_dump_exported);
        assert!(!row.trace_ids.is_empty());
        assert_eq!(row.module_ids.len(), row.module_build_ids.len());

        let preview = row
            .available_actions
            .iter()
            .find(|action| action.action_class == CrashStoreActionClass::PreviewSupportExportLocal)
            .expect("preview action");
        assert!(preview.enabled);
        assert!(!preview.network_egress);

        let export = row
            .available_actions
            .iter()
            .find(|action| action.action_class == CrashStoreActionClass::ExportMetadataBundle)
            .expect("export action");
        assert!(export.enabled);
        assert!(export.requires_local_review);
        assert!(!export.network_egress);

        if let Some(upload) = row
            .available_actions
            .iter()
            .find(|action| action.action_class == CrashStoreActionClass::UploadReviewedPacket)
        {
            if upload.enabled {
                assert!(upload.requires_local_review);
                assert!(upload.network_egress);
                assert!(row.upload_target_ref.is_some());
            }
        }
    }
}

#[test]
fn seeded_rows_bind_exact_build_identity_and_dump_metadata() {
    let packet = seeded_crash_store_viewer_packet();
    for row in &packet.rows {
        assert!(!row.crash_id.is_empty());
        assert!(!row.build_id.is_empty());
        assert!(!row.release_channel_class.is_empty());
        assert!(!row.session_type_id.is_empty());
        assert!(!row.extension_or_host_set_hash.is_empty());
        assert!(!row.policy_fingerprint.is_empty());
        assert!(!row.sandbox_profile.is_empty());
        assert!(!row.crash_window_started_at.is_empty());
        assert!(!row.crash_window_ended_at.is_empty());
        assert!(!row.architecture.is_empty());
        assert!(!row.signal_or_exception_class.is_empty());
        assert!(!row.dump_format_identity.is_empty());
    }

    let provider = packet
        .rows
        .iter()
        .find(|row| row.host_family_id == "provider_run_session_host")
        .expect("provider row");
    assert_eq!(provider.symbolication_state, SymbolicationState::Exact);
}

#[test]
fn expired_dump_variant_disables_dump_attach_and_upload() {
    let packet = seeded_expired_dump_crash_store_viewer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let row = packet
        .rows
        .iter()
        .find(|row| row.host_family_id == "query_runtime_host")
        .expect("query runtime row");
    assert_eq!(
        row.preservation_class,
        CrashPreservationClass::EnvelopeOnlyDumpExpiredMetadataPreserved
    );
    for action in &row.available_actions {
        if matches!(
            action.action_class,
            CrashStoreActionClass::AttachRawDumpOptIn | CrashStoreActionClass::UploadReviewedPacket
        ) {
            assert!(!action.enabled);
        }
    }
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        CRASH_STORE_VIEWER_SCHEMA_REF,
        CRASH_STORE_VIEWER_DOC_REF,
        CRASH_STORE_VIEWER_ARTIFACT_REF,
        "fixtures/support/m5/crash_store/README.md",
        "fixtures/support/m5/crash_store/manifest.yaml",
        "fixtures/support/m5/crash_store/packet.json",
        "fixtures/support/m5/crash_store/expired_dump_metadata_only.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn checked_in_fixtures_validate() {
    let packet = load_fixture("packet.json");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let expired = load_fixture("expired_dump_metadata_only.json");
    assert!(expired.validate().is_empty(), "{:?}", expired.validate());
}
