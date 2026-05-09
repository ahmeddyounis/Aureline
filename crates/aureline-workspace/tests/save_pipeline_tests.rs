use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_vfs::save::open_save_target;
use aureline_vfs::{HookCounters, LocalFilesystemRoot, SaveOutcome, VfsUri};

use aureline_workspace::save::{
    BomStateDetected, DetectionSource, DetectedEncoding, ExecutableIntent, FinalNewlineDetected,
    NewlineModeDetected, SaveParticipant, SourceFidelityRecord, StagedSaveCoordinator,
    StagedSaveRequest, WriteStrategy,
};

struct FailingParticipant;

impl SaveParticipant for FailingParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:fail"
    }

    fn run(&mut self, _staged: &[u8]) -> Result<Vec<u8>, String> {
        Err("injected failure".to_owned())
    }
}

fn unique_temp_path(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("aureline_save_pipeline_{label}_{suffix}.txt"))
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
fn staged_save_commits_via_atomic_replace_and_refreshes_token() {
    let tmp_path = unique_temp_path("atomic_replace");
    fs::write(&tmp_path, b"alpha").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = Vec::new();
    let request = StagedSaveRequest {
        token: token.clone(),
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit:1".to_owned(),
    };

    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert!(result.committed(), "expected committed outcome");
    assert_eq!(result.write_strategy, WriteStrategy::AtomicReplace);
    assert_eq!(result.manifest.outcome, SaveOutcome::Committed);
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        "beta"
    );

    assert_ne!(
        token.compare_before_write_generation_token.value,
        result
            .next_token
            .compare_before_write_generation_token
            .value,
        "expected the refreshed token to pin the new generation token"
    );

    let request2 = StagedSaveRequest {
        token: result.next_token.clone(),
        new_content: b"gamma".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit:2".to_owned(),
    };
    let result2 = coordinator.save(&mut root, request2, participants.as_mut_slice());
    assert!(result2.committed(), "expected second save to commit");
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        "gamma"
    );

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_detects_external_change_before_write() {
    let tmp_path = unique_temp_path("external_change");
    fs::write(&tmp_path, b"alpha").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    fs::write(&tmp_path, b"external").expect("external change");

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = Vec::new();
    let request = StagedSaveRequest {
        token,
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };

    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::ExternalChangeDetected,
        "expected compare-before-write mismatch"
    );
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        "external"
    );

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_fails_closed_when_participant_errors() {
    let tmp_path = unique_temp_path("participant_failed");
    fs::write(&tmp_path, b"alpha").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = vec![Box::new(FailingParticipant)];
    let request = StagedSaveRequest {
        token,
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };

    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(result.manifest.outcome, SaveOutcome::SaveParticipantFailed);
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        "alpha"
    );

    let _ = fs::remove_file(&tmp_path);
}
