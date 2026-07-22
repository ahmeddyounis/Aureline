use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use aureline_vfs::save::open_save_target;
use aureline_vfs::{
    Alias, AliasKind, CapabilityFlags, CaseSensitivity, GenerationToken, HookCounters,
    LocalFilesystemRoot, NormalizationForm, PermissionSnapshot, ReviewedInPlaceSave, RootClass,
    SaveOutcome, SaveTargetToken, SymlinkEscapePolicy, SyntheticRoot, SyntheticRootBuilder,
    VfsRoot, VfsUri,
};

use aureline_workspace::save::coordinator::{
    SaveCancellationToken, SaveParticipantExecutionDeclaration,
    SaveParticipantExecutionOutcomeClass,
};
use aureline_workspace::save::risk::SaveParticipantEffectRecordOutcome;
use aureline_workspace::save::{
    BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent, FileEffectSummary,
    FinalNewlineDetected, NewlineModeDetected, SaveParticipant,
    SaveParticipantCheckpointPolicyClass, SaveParticipantClass, SaveParticipantFixSafetyClass,
    SaveParticipantOutputOrigin, SaveParticipantReviewTriggerClass, SaveParticipantRiskDeclaration,
    SaveParticipantRiskOutcomeClass, SaveParticipantRiskReview, SaveParticipantRunStateClass,
    SourceFidelityAdjustment, SourceFidelityRecord, SourceFidelityRewriteClass,
    StagedSaveCoordinator, StagedSaveRequest, WriteStrategy,
};

struct FailingParticipant;

const PRIVATE_PARTICIPANT_ERROR: &str = "PRIVATE_PARTICIPANT_ERROR_7f13";
static PARTICIPANT_WORKER_TEST_LOCK: Mutex<()> = Mutex::new(());

fn participant_worker_test_lock() -> MutexGuard<'static, ()> {
    PARTICIPANT_WORKER_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl SaveParticipant for FailingParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:fail"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, _staged: &[u8]) -> Result<Vec<u8>, String> {
        Err(format!("injected failure: {PRIVATE_PARTICIPANT_ERROR}"))
    }
}

struct WholeFileRewriteParticipant {
    ran: Arc<AtomicBool>,
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

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, _staged: &[u8]) -> Result<Vec<u8>, String> {
        self.ran.store(true, Ordering::Release);
        Ok(b"rewritten".to_vec())
    }
}

struct UndeclaredWholeFileRewriteParticipant;

impl SaveParticipant for UndeclaredWholeFileRewriteParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:undeclared_whole_file_rewrite"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        Ok(vec![b'z'; staged.len().max(256)])
    }
}

struct AiApplyParticipant {
    ran: Arc<AtomicBool>,
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

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        self.ran.store(true, Ordering::Release);
        Ok(staged.to_vec())
    }
}

struct NormalizeLineEndingsParticipant;

impl SaveParticipant for NormalizeLineEndingsParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:normalize_line_endings"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(staged).map_err(|err| err.to_string())?;
        Ok(text.replace("\r\n", "\n").into_bytes())
    }
}

struct MismatchedDeclarationIdParticipant;

impl SaveParticipant for MismatchedDeclarationIdParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:runtime_id"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local("test:participant:different_declaration_id")
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        Ok(staged.to_vec())
    }
}

struct NamedNoopParticipant {
    participant_id: &'static str,
}

impl SaveParticipant for NamedNoopParticipant {
    fn participant_id(&self) -> &'static str {
        self.participant_id
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        Ok(staged.to_vec())
    }
}

struct TightEffectCeilingParticipant;

impl SaveParticipant for TightEffectCeilingParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:tight_effect_ceiling"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        let mut declaration = SaveParticipantRiskDeclaration::safe_local(self.participant_id());
        declaration.declared_file_effect_summary =
            FileEffectSummary::safe_single_file_with_byte_ceiling(1);
        declaration
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = staged.to_vec();
        if output.len() >= 2 {
            output[0] = b'X';
            output[1] = b'Y';
        }
        Ok(output)
    }
}

struct CooperativeWaitParticipant {
    participant_id: &'static str,
    started: Arc<AtomicBool>,
    timeout_ms: u64,
}

struct SlowDescriptorParticipant;

impl SaveParticipant for SlowDescriptorParticipant {
    fn participant_id(&self) -> &'static str {
        std::thread::sleep(Duration::from_millis(150));
        "test:participant:slow_descriptor"
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        Ok(staged.to_vec())
    }
}

struct NonCooperativeWaitParticipant;

impl SaveParticipant for NonCooperativeWaitParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:non_cooperative"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(20)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        std::thread::sleep(Duration::from_millis(150));
        Ok(staged.to_vec())
    }
}

impl SaveParticipant for CooperativeWaitParticipant {
    fn participant_id(&self) -> &'static str {
        self.participant_id
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration::safe_local(self.participant_id())
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(self.timeout_ms)
    }

    fn run(&mut self, _staged: &[u8]) -> Result<Vec<u8>, String> {
        Err("run_with_control must be used".to_owned())
    }

    fn run_with_control(
        &mut self,
        _staged: &[u8],
        control: &aureline_workspace::save::coordinator::SaveParticipantRunControl,
    ) -> Result<Vec<u8>, String> {
        self.started.store(true, Ordering::Release);
        while !control.is_cancelled() {
            std::thread::yield_now();
        }
        Err("participant observed cancellation".to_owned())
    }
}

fn reviewed_ai_declaration() -> SaveParticipantRiskDeclaration {
    SaveParticipantRiskDeclaration {
        participant_id: "test:participant:reviewed_ai".to_owned(),
        participant_class: SaveParticipantClass::AiApply,
        output_origin_class: SaveParticipantOutputOrigin::AiSuggestion,
        fix_safety_class: SaveParticipantFixSafetyClass::SafeLocalTextEdit,
        declared_file_effect_summary: FileEffectSummary::safe_single_file(),
        source_fidelity_rewrite_class: SourceFidelityRewriteClass::TargetedContentPatch,
        review_trigger_classes: vec![SaveParticipantReviewTriggerClass::OutputOriginHeuristicOrAi],
        checkpoint_policy_class: SaveParticipantCheckpointPolicyClass::LocalHistoryCheckpoint,
        reviewed_ticket_ref: Some("review:participant:ai:1".to_owned()),
        visible_disclosure: "Reviewed AI apply may edit only the staged visible file.".to_owned(),
    }
}

struct ReviewedAiParticipant {
    ran: Arc<AtomicBool>,
}

impl SaveParticipant for ReviewedAiParticipant {
    fn participant_id(&self) -> &'static str {
        "test:participant:reviewed_ai"
    }

    fn risk_declaration(&self) -> SaveParticipantRiskDeclaration {
        reviewed_ai_declaration()
    }

    fn execution_declaration(&self) -> SaveParticipantExecutionDeclaration {
        SaveParticipantExecutionDeclaration::staged_buffer_format_fix(100)
    }

    fn run(&mut self, staged: &[u8]) -> Result<Vec<u8>, String> {
        self.ran.store(true, Ordering::Release);
        Ok(staged.to_vec())
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

fn reviewed_in_place<R: VfsRoot>(
    root: &mut R,
    token: &SaveTargetToken,
    final_bytes: &[u8],
    review_id: &str,
) -> (String, ReviewedInPlaceSave) {
    let generation = GenerationToken {
        kind: token.compare_before_write_generation_token.kind,
        value: token.compare_before_write_generation_token.value.clone(),
    };
    let admission = root
        .prepare_reviewed_in_place_save(
            &token.identity.canonical_filesystem_object.canonical_uri,
            &token
                .identity
                .canonical_filesystem_object
                .strongest_identity_token,
            &generation,
            &token.permission_snapshot,
            final_bytes,
            review_id,
        )
        .expect("reviewed preimage checkpoint");
    (admission.checkpoint_ref().to_owned(), admission)
}

fn synthetic_in_place_root(
    initial_content: &[u8],
) -> (SyntheticRoot, VfsUri, VfsUri, SaveTargetToken) {
    let flags = CapabilityFlags {
        supports_atomic_replace: true,
        supports_in_place_write: true,
        supports_conditional_remote_write: false,
        case_sensitivity: CaseSensitivity::Sensitive,
        unicode_normalization: NormalizationForm::Nfc,
        supports_case_only_rename: true,
        supports_unicode_normalization_rename: true,
        symlink_escape_policy: SymlinkEscapePolicy::Warn,
        read_only: false,
        policy_constrained: false,
        review_required_before_save: false,
        review_required_before_rename: false,
        remote_container_adaptation: false,
    };
    let uri = VfsUri::parse("file:///synthetic/in-place.txt").expect("synthetic uri");
    let sibling =
        VfsUri::parse("file:///synthetic/in-place-alias.txt").expect("synthetic sibling uri");
    let aliases = vec![Alias {
        alias_uri: sibling.clone(),
        alias_kind: AliasKind::HardlinkSibling,
        resolution_chain: vec!["synthetic hardlink sibling -> canonical".to_owned()],
    }];
    let root = SyntheticRootBuilder::new("root-save-in-place", RootClass::LocalPosixLike, flags)
        .with_workspace_id("ws-save-in-place")
        .add_canonical_object(
            uri.as_str(),
            "aureline-ws://ws-save-in-place/root-save-in-place/in-place.txt",
            NormalizationForm::Nfc,
            "dev:1/ino:in-place",
            1,
            Vec::new(),
            PermissionSnapshot::writable_default(),
            aliases,
            initial_content.to_vec(),
        )
        .add_presentation(
            uri.as_str(),
            "in-place.txt",
            uri.as_str(),
            None,
            vec!["presentation -> canonical".to_owned()],
        )
        .add_presentation(
            sibling.as_str(),
            "in-place-alias.txt",
            uri.as_str(),
            Some(AliasKind::HardlinkSibling),
            vec!["hardlink sibling -> canonical".to_owned()],
        )
        .build();
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("synthetic token");
    (root, uri, sibling, token)
}

fn synthetic_atomic_root(initial_content: &[u8]) -> (SyntheticRoot, VfsUri, SaveTargetToken) {
    let flags = CapabilityFlags {
        supports_atomic_replace: true,
        supports_in_place_write: false,
        supports_conditional_remote_write: false,
        case_sensitivity: CaseSensitivity::Sensitive,
        unicode_normalization: NormalizationForm::Nfc,
        supports_case_only_rename: true,
        supports_unicode_normalization_rename: true,
        symlink_escape_policy: SymlinkEscapePolicy::Warn,
        read_only: false,
        policy_constrained: false,
        review_required_before_save: false,
        review_required_before_rename: false,
        remote_container_adaptation: false,
    };
    let uri = VfsUri::parse("file:///synthetic/save-participant.txt").expect("synthetic uri");
    let root = SyntheticRootBuilder::new("root-save-participant", RootClass::LocalPosixLike, flags)
        .with_workspace_id("ws-save-participant")
        .add_canonical_object(
            uri.as_str(),
            "aureline-ws://ws-save-participant/root-save-participant/save-participant.txt",
            NormalizationForm::Nfc,
            "dev:1/ino:participant",
            1,
            Vec::new(),
            PermissionSnapshot::writable_default(),
            Vec::new(),
            initial_content.to_vec(),
        )
        .add_presentation(
            uri.as_str(),
            "save-participant.txt",
            uri.as_str(),
            None,
            vec!["presentation -> canonical".to_owned()],
        )
        .build();
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("synthetic token");
    (root, uri, token)
}

#[test]
fn staged_save_commits_via_reviewed_in_place_and_uses_commit_observation() {
    let (mut root, uri, _, token) = synthetic_in_place_root(b"alpha");

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = Vec::new();
    let (checkpoint_ref, admission) =
        reviewed_in_place(&mut root, &token, b"beta", "review:first-save");
    let request = StagedSaveRequest {
        token: token.clone(),
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: Some(checkpoint_ref),
        reviewed_in_place_admission: Some(admission),
        committed_at: "mono:commit:1".to_owned(),
    };

    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert!(result.committed(), "expected committed outcome");
    assert_eq!(result.write_strategy, WriteStrategy::InPlaceWrite);
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::DegradedGuaranteeDeclared
    );
    assert_eq!(root.read_bytes(&uri).expect("saved bytes"), b"beta");

    assert_ne!(
        token.compare_before_write_generation_token.value,
        result
            .next_token
            .compare_before_write_generation_token
            .value,
        "expected the refreshed token to pin the new generation token"
    );

    let (checkpoint_ref, admission) = reviewed_in_place(
        &mut root,
        &result.next_token,
        b"gamma",
        "review:second-save",
    );
    let request2 = StagedSaveRequest {
        token: result.next_token.clone(),
        new_content: b"gamma".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: None,
        checkpoint_ref: Some(checkpoint_ref),
        reviewed_in_place_admission: Some(admission),
        committed_at: "mono:commit:2".to_owned(),
    };
    let result2 = coordinator.save(&mut root, request2, participants.as_mut_slice());
    assert!(result2.committed(), "expected second save to commit");
    assert_eq!(root.read_bytes(&uri).expect("saved bytes"), b"gamma");
}

#[test]
fn coordinator_never_attributes_a_commit_on_a_portable_blocked_root() {
    let tmp_path = unique_temp_path("postcommit_race");
    fs::write(&tmp_path, b"alpha").expect("seed file");

    let uri = VfsUri::file_url_for_path(&tmp_path).expect("file uri");
    let mut root = LocalFilesystemRoot::host_root("ws-test", "root-local");
    let mut counters = HookCounters::default();
    let token = open_save_target(&root, &uri, "mono:open", &mut counters).expect("open token");
    assert_eq!(
        token.atomic_write_mode,
        aureline_vfs::AtomicWriteMode::Blocked
    );

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = Vec::new();
    let result = coordinator.save(
        &mut root,
        StagedSaveRequest {
            token,
            new_content: b"ours".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: None,
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: "mono:commit:race".to_owned(),
        },
        participants.as_mut_slice(),
    );

    assert!(!result.committed());
    assert_eq!(result.write_strategy, WriteStrategy::Blocked);
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::GeneratedOrManagedWriteBlocked
    );
    assert_eq!(fs::read(&tmp_path).expect("unchanged bytes"), b"alpha");

    let _ = fs::remove_file(&tmp_path);
}

#[test]
fn staged_save_preserves_hardlink_aliases_via_declared_in_place_lane() {
    let (mut root, uri, sibling, token) = synthetic_in_place_root(b"alpha");
    assert_eq!(
        token.atomic_write_mode,
        aureline_vfs::AtomicWriteMode::InPlaceWrite
    );

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = Vec::new();
    let blocked = coordinator.save(
        &mut root,
        StagedSaveRequest {
            token: token.clone(),
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: None,
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: "mono:commit:hardlink".to_owned(),
        },
        participants.as_mut_slice(),
    );

    assert_eq!(
        blocked.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert_eq!(root.read_bytes(&uri).expect("primary read"), b"alpha");
    assert_eq!(
        root.resolve(sibling.as_str())
            .expect("sibling presentation")
            .object
            .content,
        b"alpha"
    );

    let (checkpoint_ref, admission) =
        reviewed_in_place(&mut root, &token, b"beta", "review:hardlink-save");
    let result = coordinator.save(
        &mut root,
        StagedSaveRequest {
            token,
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: None,
            checkpoint_ref: Some(checkpoint_ref),
            reviewed_in_place_admission: Some(admission),
            committed_at: "mono:commit:hardlink-reviewed".to_owned(),
        },
        participants.as_mut_slice(),
    );

    assert_eq!(result.write_strategy, WriteStrategy::InPlaceWrite);
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::DegradedGuaranteeDeclared
    );
    assert_eq!(root.read_bytes(&uri).expect("primary read"), b"beta");
    assert_eq!(
        root.resolve(sibling.as_str())
            .expect("sibling presentation")
            .object
            .content,
        b"beta"
    );
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
        reviewed_in_place_admission: None,
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
    let _worker_test_guard = participant_worker_test_lock();
    let (mut root, uri, token) = synthetic_atomic_root(b"alpha");

    let mut coordinator = StagedSaveCoordinator::new();
    let mut participants: Vec<Box<dyn SaveParticipant>> = vec![Box::new(FailingParticipant)];
    let request = StagedSaveRequest {
        token,
        new_content: b"beta".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        reviewed_in_place_admission: None,
        committed_at: "mono:commit".to_owned(),
    };

    let result = coordinator.save(&mut root, request, participants.as_mut_slice());
    assert_eq!(result.manifest.outcome, SaveOutcome::SaveParticipantFailed);
    assert!(!result
        .participant_error
        .as_ref()
        .expect("typed participant error")
        .detail
        .contains(PRIVATE_PARTICIPANT_ERROR));
    assert!(!result
        .manifest
        .failure_detail
        .as_deref()
        .unwrap_or_default()
        .contains(PRIVATE_PARTICIPANT_ERROR));
    assert!(!format!("{result:?}").contains(PRIVATE_PARTICIPANT_ERROR));
    assert_eq!(root.read_bytes(&uri).expect("unchanged bytes"), b"alpha");
}

#[test]
fn staged_save_holds_declared_whole_file_rewrite_before_participant_runs() {
    let _worker_test_guard = participant_worker_test_lock();
    let reviewed_bytes = b"alpha\nbeta\n";
    let (mut root, uri, token) = synthetic_atomic_root(reviewed_bytes);

    let ran = Arc::new(AtomicBool::new(false));
    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(WholeFileRewriteParticipant { ran: ran.clone() })];
    let request = StagedSaveRequest {
        token,
        new_content: b"alpha\nbeta\n".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        reviewed_in_place_admission: None,
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
    assert!(
        !ran.load(Ordering::Acquire),
        "participant must not run before review"
    );
    assert_eq!(
        root.read_bytes(&uri).expect("unchanged bytes"),
        reviewed_bytes
    );
}

#[test]
fn staged_save_detects_undeclared_whole_file_rewrite_before_commit() {
    let _worker_test_guard = participant_worker_test_lock();
    let original = "a".repeat(512);
    let (mut root, uri, token) = synthetic_atomic_root(original.as_bytes());

    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(UndeclaredWholeFileRewriteParticipant)];
    let request = StagedSaveRequest {
        token,
        new_content: original.as_bytes().to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        reviewed_in_place_admission: None,
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
        root.read_bytes(&uri).expect("unchanged bytes"),
        original.as_bytes()
    );
}

#[test]
fn staged_save_holds_ai_apply_before_participant_runs() {
    let _worker_test_guard = participant_worker_test_lock();
    let reviewed_bytes = b"alpha\nbeta\n";
    let (mut root, uri, token) = synthetic_atomic_root(reviewed_bytes);

    let ran = Arc::new(AtomicBool::new(false));
    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(AiApplyParticipant { ran: ran.clone() })];
    let request = StagedSaveRequest {
        token,
        new_content: b"alpha\nbeta\n".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        reviewed_in_place_admission: None,
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
    assert!(
        !ran.load(Ordering::Acquire),
        "participant must not run before review"
    );
    assert_eq!(
        root.read_bytes(&uri).expect("unchanged bytes"),
        reviewed_bytes
    );
}

#[test]
fn staged_save_holds_participant_source_fidelity_conversion_before_commit() {
    let _worker_test_guard = participant_worker_test_lock();
    let reviewed_bytes = b"alpha\r\nbeta\r\n";
    let (mut root, uri, token) = synthetic_atomic_root(reviewed_bytes);

    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(NormalizeLineEndingsParticipant)];
    let request = StagedSaveRequest {
        token,
        new_content: b"alpha\r\nbeta\r\n".to_vec(),
        source_fidelity: crlf_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:test".to_owned()),
        checkpoint_ref: None,
        reviewed_in_place_admission: None,
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
        root.read_bytes(&uri).expect("unchanged bytes"),
        reviewed_bytes
    );
}

#[test]
fn participant_guard_rejects_mismatched_and_duplicate_ids_before_run() {
    let _worker_test_guard = participant_worker_test_lock();
    for (label, mut participants) in [
        (
            "mismatched_id",
            vec![Box::new(MismatchedDeclarationIdParticipant) as Box<dyn SaveParticipant>],
        ),
        (
            "duplicate_id",
            vec![
                Box::new(NamedNoopParticipant {
                    participant_id: "test:participant:duplicate",
                }) as Box<dyn SaveParticipant>,
                Box::new(NamedNoopParticipant {
                    participant_id: "test:participant:duplicate",
                }) as Box<dyn SaveParticipant>,
            ],
        ),
    ] {
        let (mut root, uri, token) = synthetic_atomic_root(b"alpha");
        let request = StagedSaveRequest {
            token,
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: Some(format!("save_participant_group:{label}")),
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: format!("mono:commit:{label}"),
        };

        let result =
            StagedSaveCoordinator::new().save(&mut root, request, participants.as_mut_slice());
        assert_eq!(result.manifest.outcome, SaveOutcome::SaveParticipantFailed);
        assert!(result.participant_error.is_some());
        assert!(result.participant_effect_receipts.iter().all(|receipt| {
            receipt.outcome_class == SaveParticipantExecutionOutcomeClass::BlockedBeforeRun
        }));
        assert_eq!(root.read_bytes(&uri).expect("unchanged file"), b"alpha");
    }
}

#[test]
fn participant_guard_rejects_every_effect_dimension_above_the_declared_ceiling() {
    let _worker_test_guard = participant_worker_test_lock();
    let declared = FileEffectSummary::no_write();
    let actual = FileEffectSummary {
        files_touched: 1,
        files_created: 1,
        files_deleted: 1,
        changed_bytes: 2,
        whole_file_rewrite: true,
        generated_artifacts_touched: 1,
        protected_paths_touched: 1,
        may_touch_outside_visible_file: true,
    };
    assert_eq!(
        actual.ceiling_violations(&declared),
        vec![
            "files_touched",
            "files_created",
            "files_deleted",
            "changed_bytes",
            "whole_file_rewrite",
            "generated_artifacts_touched",
            "protected_paths_touched",
            "may_touch_outside_visible_file",
        ]
    );

    let mut review = SaveParticipantRiskReview::open(
        "save_participant_risk_review:test:unknown",
        "save_packet:test:unknown",
        None,
        vec![SaveParticipantRiskDeclaration::safe_local(
            "test:participant:declared",
        )],
    );
    assert_eq!(
        review.record_actual_effect("test:participant:unknown", FileEffectSummary::no_write()),
        SaveParticipantEffectRecordOutcome::UnknownParticipant
    );
    assert_eq!(
        review.outcome_class,
        SaveParticipantRiskOutcomeClass::BlockedNoWrite
    );

    let (mut root, uri, token) = synthetic_atomic_root(b"abcd");
    let request = StagedSaveRequest {
        token,
        new_content: b"abcd".to_vec(),
        source_fidelity: default_source_fidelity(),
        save_participant_group_id: Some("save_participant_group:effect-ceiling".to_owned()),
        checkpoint_ref: None,
        reviewed_in_place_admission: None,
        committed_at: "mono:commit:effect-ceiling".to_owned(),
    };
    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(TightEffectCeilingParticipant)];

    let result = StagedSaveCoordinator::new().save(&mut root, request, participants.as_mut_slice());
    assert_eq!(
        result.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert_eq!(
        result.participant_effect_receipts[0].outcome_class,
        SaveParticipantExecutionOutcomeClass::EffectCeilingExceeded
    );
    assert!(!result.participant_effect_receipts[0].effect_ceiling_satisfied);
    assert_eq!(
        result.participant_effect_receipts[0]
            .actual_file_effect_summary
            .as_ref()
            .expect("actual effect")
            .changed_bytes,
        2
    );
    assert_eq!(root.read_bytes(&uri).expect("unchanged file"), b"abcd");
}

#[test]
fn participant_guard_times_out_and_cancels_without_waiting_unboundedly() {
    let _worker_test_guard = participant_worker_test_lock();
    for cancellation_case in [false, true] {
        let (mut root, uri, token) = synthetic_atomic_root(b"alpha");
        let request = StagedSaveRequest {
            token,
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: Some("save_participant_group:bounded".to_owned()),
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: "mono:commit:bounded".to_owned(),
        };
        let started = Arc::new(AtomicBool::new(false));
        let mut participants: Vec<Box<dyn SaveParticipant>> =
            vec![Box::new(CooperativeWaitParticipant {
                participant_id: if cancellation_case {
                    "test:participant:cancelled"
                } else {
                    "test:participant:timed_out"
                },
                started: started.clone(),
                timeout_ms: if cancellation_case { 1_000 } else { 20 },
            })];
        let cancellation = SaveCancellationToken::new();
        let cancel_thread = cancellation_case.then(|| {
            let cancellation = cancellation.clone();
            let started = started.clone();
            std::thread::spawn(move || {
                while !started.load(Ordering::Acquire) {
                    std::thread::yield_now();
                }
                cancellation.cancel();
            })
        });

        let began = Instant::now();
        let result = StagedSaveCoordinator::new().save_with_cancellation(
            &mut root,
            request,
            participants.as_mut_slice(),
            cancellation,
        );
        if let Some(cancel_thread) = cancel_thread {
            cancel_thread.join().expect("canceller joined");
        }
        assert!(
            began.elapsed() < Duration::from_millis(500),
            "bounded participant execution took {:?}",
            began.elapsed()
        );
        assert_eq!(result.manifest.outcome, SaveOutcome::SaveParticipantFailed);
        assert_eq!(
            result.participant_effect_receipts[0].outcome_class,
            if cancellation_case {
                SaveParticipantExecutionOutcomeClass::Cancelled
            } else {
                SaveParticipantExecutionOutcomeClass::TimedOut
            }
        );
        assert!(result.participant_effect_receipts[0].cancellation_requested);
        assert_eq!(root.read_bytes(&uri).expect("unchanged file"), b"alpha");
    }
}

#[test]
fn participant_guard_bounds_slow_descriptors_and_non_cooperative_execution() {
    let _worker_test_guard = participant_worker_test_lock();
    for (label, mut participants, expected_outcome) in [
        (
            "slow-descriptor",
            vec![Box::new(SlowDescriptorParticipant) as Box<dyn SaveParticipant>],
            SaveParticipantExecutionOutcomeClass::TimedOut,
        ),
        (
            "non-cooperative-run",
            vec![Box::new(NonCooperativeWaitParticipant) as Box<dyn SaveParticipant>],
            SaveParticipantExecutionOutcomeClass::TimedOut,
        ),
    ] {
        let (mut root, uri, token) = synthetic_atomic_root(b"alpha");
        let request = StagedSaveRequest {
            token,
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: Some(format!("save_participant_group:{label}")),
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: format!("mono:commit:{label}"),
        };

        let began = Instant::now();
        let result =
            StagedSaveCoordinator::new().save(&mut root, request, participants.as_mut_slice());
        assert!(
            began.elapsed() < Duration::from_millis(500),
            "bounded {label} took {:?}",
            began.elapsed()
        );
        assert_eq!(result.manifest.outcome, SaveOutcome::SaveParticipantFailed);
        assert_eq!(
            result.participant_effect_receipts[0].outcome_class,
            expected_outcome
        );
        assert!(result.participant_effect_receipts[0].cancellation_requested);
        assert_eq!(root.read_bytes(&uri).expect("unchanged file"), b"alpha");
    }

    // Detached test participants finish promptly so the process-wide bounded
    // worker budget is restored before other tests continue.
    std::thread::sleep(Duration::from_millis(175));
}

#[test]
fn participant_guard_binds_review_to_target_content_and_expiry() {
    let _worker_test_guard = participant_worker_test_lock();
    let (mut root, uri, token) = synthetic_atomic_root(b"alpha");
    let declaration = reviewed_ai_declaration();

    let ran = Arc::new(AtomicBool::new(false));
    let mut participants: Vec<Box<dyn SaveParticipant>> =
        vec![Box::new(ReviewedAiParticipant { ran: ran.clone() })];
    let blocked = StagedSaveCoordinator::new().save(
        &mut root,
        StagedSaveRequest {
            token: token.clone(),
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: Some("save_participant_group:review".to_owned()),
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: "mono:commit:raw-ticket".to_owned(),
        },
        participants.as_mut_slice(),
    );
    assert_eq!(
        blocked.manifest.outcome,
        SaveOutcome::ReviewRequiredBeforeSave
    );
    assert!(!ran.load(Ordering::Acquire));
    assert_eq!(
        blocked.participant_effect_receipts[0].outcome_class,
        SaveParticipantExecutionOutcomeClass::HeldForReview
    );

    let mut coordinator = StagedSaveCoordinator::new();
    assert!(coordinator
        .admit_reviewed_participant(
            &token,
            b"beta",
            &declaration,
            "review:participant:ai:1",
            SystemTime::now() - Duration::from_secs(1),
        )
        .is_err());
    coordinator
        .admit_reviewed_participant(
            &token,
            b"beta",
            &declaration,
            "review:participant:ai:1",
            SystemTime::now() + Duration::from_secs(60),
        )
        .expect("bound participant review admission");

    let admitted = coordinator.save(
        &mut root,
        StagedSaveRequest {
            token,
            new_content: b"beta".to_vec(),
            source_fidelity: default_source_fidelity(),
            save_participant_group_id: Some("save_participant_group:review".to_owned()),
            checkpoint_ref: None,
            reviewed_in_place_admission: None,
            committed_at: "mono:commit:bound-ticket".to_owned(),
        },
        participants.as_mut_slice(),
    );
    assert!(admitted.committed());
    assert!(ran.load(Ordering::Acquire));
    assert_eq!(
        admitted.participant_effect_receipts[0].outcome_class,
        SaveParticipantExecutionOutcomeClass::Ran
    );
    assert!(admitted.participant_effect_receipts[0].effect_ceiling_satisfied);
    assert_eq!(root.read_bytes(&uri).expect("committed file"), b"beta");
}
