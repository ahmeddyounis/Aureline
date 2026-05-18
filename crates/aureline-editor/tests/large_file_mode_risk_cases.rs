use std::fs;
use std::path::{Path, PathBuf};

use aureline_editor::{
    open_document, ClassificationPolicy, DocumentOpenDisposition, DocumentOpenOutcome,
    LargeFileViewerConfig, LimitedModeCapabilityState, LimitedModeFileRecord,
    LIMITED_MODE_FILE_RECORD_KIND,
};
use aureline_vfs::{LocalFilesystemRoot, VfsUri};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn limited_mode_record_explains_binary_safe_preview_and_write_limits() {
    let repo_root = repo_root();
    let file_path = repo_root.join("fixtures/text/large/null_byte_blob.bin");
    let uri = VfsUri::file_url_for_path(&file_path).expect("fixture file uri");
    let root = LocalFilesystemRoot::new("ws-test", "repo-root", repo_root.clone())
        .expect("local filesystem root");

    let outcome = open_document(
        &root,
        &uri,
        &ClassificationPolicy::default(),
        LargeFileViewerConfig::default(),
        DocumentOpenDisposition::Auto,
    )
    .expect("document opens");

    let DocumentOpenOutcome::LargeFile(doc) = outcome else {
        panic!("binary-like fixture must open in limited mode");
    };

    let record = LimitedModeFileRecord::from_large_file_document(
        "limited_mode.file.test",
        "workspace.test",
        "document.test",
        &doc,
    );

    assert_eq!(record.record_kind, LIMITED_MODE_FILE_RECORD_KIND);
    assert!(record.is_support_export_safe());
    assert_eq!(record.override_action.action_id, "open_anyway");

    let whole_file_load = record
        .capability("whole_file_load_into_ram")
        .expect("whole-file load capability exists");
    assert_eq!(whole_file_load.state, LimitedModeCapabilityState::Denied);

    let whole_save = record
        .capability("save_participant_whole_file")
        .expect("whole-file save participant capability exists");
    assert_eq!(whole_save.state, LimitedModeCapabilityState::Denied);
}

#[test]
fn limited_mode_fixture_round_trips_as_support_safe_record() {
    let fixture_path = repo_root()
        .join("fixtures/editor/m3/large_file_and_whole_rewrite/limited_mode_binary_blob.json");
    let raw = fs::read_to_string(&fixture_path).expect("fixture reads");
    let fixture: LimitedModeFileRecord = serde_json::from_str(&raw).expect("fixture parses");

    assert!(fixture.is_support_export_safe());
    assert_eq!(fixture.record_kind, LIMITED_MODE_FILE_RECORD_KIND);
    assert_eq!(
        fixture
            .capability("save_participant_whole_file")
            .expect("whole-file participant capability")
            .state,
        LimitedModeCapabilityState::Denied
    );
}
