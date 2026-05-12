use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_vfs::save::open_save_target;
use aureline_vfs::{HookCounters, LocalFilesystemRoot, SaveOutcome, VfsUri};

use aureline_workspace::save::{
    detect_and_decode_for_buffer, encode_for_save, DetectedEncoding, NewlineModeDetected,
    StagedSaveCoordinator, StagedSaveRequest,
};

fn unique_temp_path(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("aureline_source_fidelity_{label}_{suffix}.bin"))
}

fn write_temp_file(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).expect("write temp fixture");
}

fn open_token(path: &Path) -> (LocalFilesystemRoot, VfsUri, aureline_vfs::SaveTargetToken) {
    let uri = VfsUri::file_url_for_path(path).expect("file uri");
    let root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");
    (root, uri, token)
}

#[test]
fn preserves_utf8_bom_crlf_and_final_newline_on_save() {
    let tmp_path = unique_temp_path("utf8_bom_crlf");
    let on_disk = b"\xEF\xBB\xBFa\r\nb\r\n";
    write_temp_file(&tmp_path, on_disk);

    let (mut root, _uri, token) = open_token(&tmp_path);
    let open = detect_and_decode_for_buffer(on_disk, &token.permission_snapshot);
    assert_eq!(open.record.detected_encoding, DetectedEncoding::Utf8Bom);
    assert_eq!(open.record.newline_mode_detected, NewlineModeDetected::Crlf);

    let mut buffer_bytes = open
        .buffer_utf8_bytes
        .clone()
        .expect("utf8 bom should decode");
    let Some(pos) = buffer_bytes.iter().position(|&b| b == b'a') else {
        panic!("expected ascii a in decoded buffer");
    };
    buffer_bytes[pos] = b'x';

    let expected = encode_for_save(&open.record, &buffer_bytes).expect("encode");

    let mut coordinator = StagedSaveCoordinator::new();
    let request = StagedSaveRequest {
        token,
        new_content: buffer_bytes,
        source_fidelity: open.record,
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };
    let mut participants = Vec::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert!(result.committed(), "expected committed save");

    let observed = fs::read(&tmp_path).expect("read saved bytes");
    assert_eq!(observed, expected);

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn preserves_utf16le_bom_and_lf_on_save() {
    let tmp_path = unique_temp_path("utf16le_bom_lf");
    let on_disk = vec![0xFF, 0xFE, 0x61, 0x00, 0x0A, 0x00, 0x62, 0x00];
    write_temp_file(&tmp_path, &on_disk);

    let (mut root, _uri, token) = open_token(&tmp_path);
    let open = detect_and_decode_for_buffer(&on_disk, &token.permission_snapshot);
    assert_eq!(open.record.detected_encoding, DetectedEncoding::Utf16LeBom);
    assert_eq!(open.record.newline_mode_detected, NewlineModeDetected::Lf);

    let mut buffer_bytes = open
        .buffer_utf8_bytes
        .clone()
        .expect("utf16le bom should decode");
    let Some(pos) = buffer_bytes.iter().position(|&b| b == b'b') else {
        panic!("expected ascii b in decoded buffer");
    };
    buffer_bytes[pos] = b'c';

    let expected = encode_for_save(&open.record, &buffer_bytes).expect("encode");

    let mut coordinator = StagedSaveCoordinator::new();
    let request = StagedSaveRequest {
        token,
        new_content: buffer_bytes,
        source_fidelity: open.record,
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };
    let mut participants = Vec::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert!(result.committed(), "expected committed save");

    let observed = fs::read(&tmp_path).expect("read saved bytes");
    assert_eq!(observed, expected);

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn blocks_save_when_encoding_is_unknown_binary_like() {
    let tmp_path = unique_temp_path("unknown_binary_like");
    let on_disk = vec![0xFF, 0x00, 0x80, 0xC0, 0xAF];
    write_temp_file(&tmp_path, &on_disk);

    let (mut root, _uri, token) = open_token(&tmp_path);
    let open = detect_and_decode_for_buffer(&on_disk, &token.permission_snapshot);
    assert_eq!(
        open.record.detected_encoding,
        DetectedEncoding::UnknownBinaryLike
    );
    assert!(
        open.buffer_utf8_bytes.is_none(),
        "expected no decoded bytes"
    );

    let mut coordinator = StagedSaveCoordinator::new();
    let request = StagedSaveRequest {
        token,
        new_content: b"beta".to_vec(),
        source_fidelity: open.record,
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };
    let mut participants = Vec::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(result.manifest.outcome, SaveOutcome::SaveParticipantFailed);
    assert!(result.participant_error.is_some());

    let observed = fs::read(&tmp_path).expect("read saved bytes");
    assert_eq!(observed, on_disk, "expected no write to land");

    let _ = fs::remove_file(&tmp_path);
}

#[cfg(unix)]
#[test]
fn preserves_executable_bit_on_atomic_replace_save() {
    use std::os::unix::fs::PermissionsExt;

    let tmp_path = unique_temp_path("exec_bit");
    write_temp_file(&tmp_path, b"#!/bin/sh\necho hi\n");
    fs::set_permissions(&tmp_path, fs::Permissions::from_mode(0o755)).expect("chmod");

    let on_disk = fs::read(&tmp_path).expect("read initial bytes");
    let (mut root, _uri, token) = open_token(&tmp_path);

    let open = detect_and_decode_for_buffer(&on_disk, &token.permission_snapshot);
    let mut buffer_bytes = open
        .buffer_utf8_bytes
        .clone()
        .expect("script bytes should decode");
    buffer_bytes.extend_from_slice(b"echo bye\n");

    let mut coordinator = StagedSaveCoordinator::new();
    let request = StagedSaveRequest {
        token,
        new_content: buffer_bytes,
        source_fidelity: open.record,
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };
    let mut participants = Vec::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert!(result.committed(), "expected committed save");

    let mode = fs::metadata(&tmp_path).expect("stat").permissions().mode();
    assert_ne!(mode & 0o111, 0, "expected executable bits preserved");

    let _ = fs::remove_file(&tmp_path);
}
