use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_vfs::save::open_save_target;
use aureline_vfs::{HookCounters, LocalFilesystemRoot, SaveOutcome, VfsUri};

use aureline_workspace::save::{
    detect_external_drift, BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent,
    ExternalDriftConflict, FinalNewlineDetected, NewlineModeDetected, SourceFidelityRecord,
    StagedSaveCoordinator, StagedSaveRequest,
};

fn unique_temp_path(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("aureline_external_drift_{label}_{suffix}.txt"))
}

fn open_token(root: &LocalFilesystemRoot, path: &Path) -> (VfsUri, aureline_vfs::SaveTargetToken) {
    let uri = VfsUri::file_url_for_path_lossy(path)
        .or_else(|| VfsUri::file_url_for_path(path))
        .expect("file uri");
    let mut counters = HookCounters::default();
    let token = open_save_target(root, &uri, "mono:open", &mut counters).expect("open token");
    (uri, token)
}

fn default_source_fidelity() -> SourceFidelityRecord {
    SourceFidelityRecord {
        detected_encoding: DetectedEncoding::Utf8,
        detection_source: DetectionSource::Utf8Heuristic,
        bom_state_detected: BomStateDetected::Absent,
        newline_mode_detected: NewlineModeDetected::Lf,
        final_newline_detected: FinalNewlineDetected::Absent,
        executable_intent: ExecutableIntent::NonExecutable,
    }
}

#[test]
fn external_drift_detector_refuses_on_generation_token_mismatch() {
    let tmp_path = unique_temp_path("generation_mismatch");
    fs::write(&tmp_path, b"alpha").expect("seed file");

    let root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let (_uri, token) = open_token(&root, &tmp_path);

    fs::write(&tmp_path, b"external").expect("external change");

    let err = detect_external_drift(&root, &token).expect_err("expected drift conflict");
    assert_eq!(err.outcome, SaveOutcome::ExternalChangeDetected);

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_refuses_when_presentation_path_no_longer_resolves() {
    let tmp_path = unique_temp_path("rename_drift");
    fs::write(&tmp_path, b"alpha").expect("seed file");

    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let (_uri, token) = open_token(&root, &tmp_path);

    let moved_path = tmp_path.with_extension("moved");
    fs::rename(&tmp_path, &moved_path).expect("rename");

    let mut coordinator = StagedSaveCoordinator::new();
    let request = StagedSaveRequest {
        token,
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };
    let mut participants = Vec::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::WrongTargetPrevented,
        "expected wrong-target prevention when the opened path no longer resolves"
    );
    assert!(
        !tmp_path.exists(),
        "expected save pipeline not to recreate a moved path"
    );
    assert_eq!(
        fs::read_to_string(&moved_path).expect("read moved file"),
        "alpha",
        "expected no overwrite of the moved target"
    );

    let _ = fs::remove_file(&moved_path);
}

#[cfg(unix)]
#[test]
fn staged_save_refuses_when_symlink_target_drifts() {
    use std::os::unix::fs::symlink;

    let dir = std::env::temp_dir().join(format!(
        "aureline_external_drift_symlink_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("temp dir");

    let canonical_a = dir.join("a.txt");
    let canonical_b = dir.join("b.txt");
    let alias = dir.join("alias.txt");

    fs::write(&canonical_a, b"alpha").expect("seed a");
    fs::write(&canonical_b, b"external").expect("seed b");
    symlink(&canonical_a, &alias).expect("symlink alias");

    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let (_uri, token) = open_token(&root, &alias);

    let mut coordinator = StagedSaveCoordinator::new();
    let request = StagedSaveRequest {
        token,
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };

    fs::remove_file(&alias).expect("remove old alias");
    symlink(&canonical_b, &alias).expect("retarget alias");

    let mut participants = Vec::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::WrongTargetPrevented,
        "expected wrong-target prevention when the alias resolves elsewhere"
    );
    assert_eq!(fs::read_to_string(&canonical_a).expect("read a"), "alpha");
    assert_eq!(
        fs::read_to_string(&canonical_b).expect("read b"),
        "external"
    );

    let _ = fs::remove_file(&alias);
    let _ = fs::remove_file(&canonical_a);
    let _ = fs::remove_file(&canonical_b);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn external_drift_conflict_is_formattable() {
    let conflict = ExternalDriftConflict {
        outcome: SaveOutcome::ExternalChangeDetected,
        detail: "generation_token_mismatch: pinned a observed b".to_owned(),
    };
    assert!(
        conflict.to_string().contains("external_change_detected"),
        "expected formatted conflict to include the stable outcome token"
    );
}
