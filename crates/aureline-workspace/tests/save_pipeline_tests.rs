use std::cell::Cell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_vfs::save::open_save_target;
use aureline_vfs::{HookCounters, LocalFilesystemRoot, SaveOutcome, VfsUri};

use aureline_workspace::save::{
    BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent, FileEffectSummary,
    FinalNewlineDetected, NewlineModeDetected, SaveParticipant,
    SaveParticipantCheckpointPolicyClass, SaveParticipantClass, SaveParticipantFixSafetyClass,
    SaveParticipantOutputOrigin, SaveParticipantReviewTriggerClass, SaveParticipantRiskDeclaration,
    SaveParticipantRiskOutcomeClass, SaveParticipantRunStateClass, SourceFidelityAdjustment,
    SourceFidelityRecord, SourceFidelityRewriteClass, StagedSaveCoordinator, StagedSaveRequest,
    WriteStrategy,
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

struct WholeFileRewriteParticipant {
    ran: Rc<Cell<bool>>,
}

impl SaveParticipant for WholeFileRewriteParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:whole_file_rewrite"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::whole_file_rewrite(
            self.participant_id(),
            SaveParticipantClass::Formatter,
            SaveParticipantOutputOrigin::ImportedConfig,
            512,
            "Formatter would rewrite the whole file and requires review before save.",
        )
    }

    fn run(&mut self, _staged: &[u8]) -> Result<Vec<u8>, String> {
        self.ran.set(true);
        Ok(b"rewritten".to_vec())
    }
}

struct UndeclaredWholeFileRewriteParticipant;

impl SaveParticipant for UndeclaredWholeFileRewriteParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:undeclared_whole_file_rewrite"
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        Ok(vec![b'z'; staged.len().max(256)])
    }
}

struct AiApplyParticipant {
    ran: Rc<Cell<bool>>,
}

impl SaveParticipant for AiApplyParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:ai_apply"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration {
            participant_id: self.participant_id().to_owned(),
            participant_class: SaveParticipantClass::AiApply,
            output_origin_class: SaveParticipantOutputOrigin::AiSuggestion,
            fix_safety_class: SaveParticipantFixSafetyClass::SafeLocalTextEdit,
            declared_file_effect_summary: FileEffectSummary::safe_single_file(),
            source_fidelity_rewrite_class: SourceFidelityRewriteClass::TargetedContentPatch,
            review_trigger_classes: vec![SaveParticipantReviewTriggerClass::NotRequired],
            checkpoint_policy_class: SaveParticipantCheckpointPolicyClass::LocalHistoryCheckpoint,
            reviewed_ticket_ref: None,
            visible_disclosure: "AI apply requires review before mutating staged content."
                .to_owned(),
        }
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        self.ran.set(true);
        Ok(staged.to_vec())
    }
}

struct NormalizeLineEndingsParticipant;

impl SaveParticipant for NormalizeLineEndingsParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:normalize_line_endings"
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(staged).map_err(|err| err.to_string())?;
        Ok(text.replace("\r\n", "\n").into_bytes())
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

fn crlf_source_fidelity() -> SourceFidelityRecord {
    SourceFidelityRecord {
        newline_mode_detected: NewlineModeDetected::Crlf,
        final_newline_detected: FinalNewlineDetected::Present,
        ..default_source_fidelity()
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

#[test]
fn staged_save_holds_declared_whole_file_rewrite_before_participant_runs() {
    let tmp_path = unique_temp_path("whole_rewrite_declared");
    fs::write(&tmp_path, b"alpha\nbeta\n").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    let ran = Rc::new(Cell::new(false));
    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(WholeFileRewriteParticipant { ran: ran.clone() })];
    let request = StagedSaveRequest {
        token,
        new_content: b"alpha\nbeta\n".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };

    let mut coordinator = StagedSaveCoordinator::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert_eq!(
        result.save_participant_risk_review.outcome_class,
        SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation
    );
    assert_eq!(
        result.save_participant_risk_review.participant_entries[0].run_state_class,
        SaveParticipantRunStateClass::HeldForReview
    );
    assert!(!ran.get(), "participant must not run before review");
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        "alpha\nbeta\n"
    );

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_detects_undeclared_whole_file_rewrite_before_commit() {
    let tmp_path = unique_temp_path("whole_rewrite_undeclared");
    let original = "a".repeat(512);
    fs::write(&tmp_path, original.as_bytes()).expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(UndeclaredWholeFileRewriteParticipant)];
    let request = StagedSaveRequest {
        token,
        new_content: original.as_bytes().to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        committed_at: "mono:commit".to_owned(),
    };

    let mut coordinator = StagedSaveCoordinator::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert_eq!(
        result.save_participant_risk_review.outcome_class,
        SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeCommit
    );
    assert!(
        result.save_participant_risk_review.participant_entries[0]
            .actual_file_effect_summary
            .as_ref()
            .expect("actual effect")
            .whole_file_rewrite
    );
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        original
    );

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_holds_ai_apply_before_participant_runs() {
    let tmp_path = unique_temp_path("ai_apply_requires_review");
    fs::write(&tmp_path, b"alpha\nbeta\n").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    let ran = Rc::new(Cell::new(false));
    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(AiApplyParticipant { ran: ran.clone() })];
    let request = StagedSaveRequest {
        token,
        new_content: b"alpha\nbeta\n".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: Some("checkpoint:before-ai-apply".to_owned()),
        committed_at: "mono:commit".to_owned(),
    };

    let mut coordinator = StagedSaveCoordinator::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert_eq!(
        result.save_participant_risk_review.outcome_class,
        SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation
    );
    assert_eq!(
        result.save_participant_risk_review.participant_entries[0].run_state_class,
        SaveParticipantRunStateClass::HeldForReview
    );
    assert!(!ran.get(), "participant must not run before review");
    assert_eq!(
        fs::read_to_string(&tmp_path).expect("read temp file"),
        "alpha\nbeta\n"
    );

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_holds_participant_source_fidelity_conversion_before_commit() {
    let tmp_path = unique_temp_path("source_fidelity_conversion");
    fs::write(&tmp_path, b"alpha\r\nbeta\r\n").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");

    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(NormalizeLineEndingsParticipant)];
    let request = StagedSaveRequest {
        token,
        new_content: b"alpha\r\nbeta\r\n".to_vec(),
        source_fidelity: crlf_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: Some("checkpoint:before-format".to_owned()),
        committed_at: "mono:commit".to_owned(),
    };

    let mut coordinator = StagedSaveCoordinator::new();
    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert_eq!(
        result.save_participant_risk_review.outcome_class,
        SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeCommit
    );
    assert!(result
        .save_participant_risk_review
        .source_fidelity_adjustments
        .contains(&SourceFidelityAdjustment::LineEndingPosturePreserved));
    assert_eq!(
        fs::read(&tmp_path).expect("read temp file"),
        b"alpha\r\nbeta\r\n"
    );

    let _ = fs::remove_file(&tmp_path);
}
