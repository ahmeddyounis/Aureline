//! Unit tests for the recovery-state lineage projection.

use super::*;
use aureline_buffer::{Buffer, TransactionSpec, UndoClass};
use aureline_history::{ActorLineageClass, ActorLineageRow, LocalHistoryConsumerSurface};
use aureline_recovery::crash_journal::{
    ActorClass, ActorSurfaceRecord, AutosaveJournalEntryRecord, BaseOnDiskTokenRecord,
    CaptureClass, CaptureDescriptorRecord, CaptureMode, CaptureOmissionReason, ChecksumAlgorithm,
    DecoderPosture, DowngradeReasonClass, EncodingLabelClass, ExternalChangeState,
    FinalNewlineState, FrameIntegrityState, GuidedChoiceClass, IdentityRelation, IntegrityRecord,
    NewlineMode, ObjectClass, ObjectIdentityRecord, ReplayIntegrityPosture, ReplayPostureClass,
    ReplayPostureRecord, RetentionClass, RetentionPostureRecord, SourceClass,
    SupportBundleInclusionState, SupportExportRecord, SurfaceClass, TextFormatRecord, TokenClass,
    TokenConfidenceClass,
};

/// A clean, source-faithful, exactly-identified autosave entry whose restore is
/// allowed, creates a recovery checkpoint, and verified cleanly.
fn clean_entry() -> AutosaveJournalEntryRecord {
    AutosaveJournalEntryRecord::new(
        "autosave.entry.clean".to_owned(),
        "autosave.journal.clean".to_owned(),
        "workspace.fixture.clean".to_owned(),
        ObjectIdentityRecord {
            logical_document_id: "doc.clean".to_owned(),
            object_ref: "object.clean".to_owned(),
            object_class: ObjectClass::CanonicalFile,
            presentation_hint: None,
            filesystem_identity_ref: Some("fsid.clean".to_owned()),
            canonical_identity_ref: Some("canon.clean".to_owned()),
            branch_worktree_ref: None,
            identity_relation: IdentityRelation::ExactObjectIdentity,
            identity_notes: "Captured object matches current object identity.".to_owned(),
        },
        BaseOnDiskTokenRecord {
            token_class: TokenClass::InodeMtimeSizeHash,
            token_ref: Some("token.clean".to_owned()),
            observed_revision_ref: None,
            token_confidence: TokenConfidenceClass::Strong,
            compare_before_write_required: true,
            external_change_state: ExternalChangeState::NoExternalChangeKnown,
        },
        TextFormatRecord {
            encoding_label: EncodingLabelClass::Utf8,
            bom_policy: "preserve_absent".to_owned(),
            newline_mode: NewlineMode::Lf,
            decoder_posture: DecoderPosture::ExactDecode,
            final_newline_state: FinalNewlineState::Present,
            large_file_mode: false,
            format_notes: "UTF-8 LF, final newline present.".to_owned(),
        },
        ActorSurfaceRecord {
            actor_class: ActorClass::UserKeystroke,
            source_class: SourceClass::HumanLocal,
            surface_class: SurfaceClass::EditorTyping,
            command_ref: None,
            session_ref: None,
            actor_display: "Local typing".to_owned(),
        },
        CaptureDescriptorRecord {
            capture_class: CaptureClass::FullBufferSnapshot,
            capture_mode: CaptureMode::ContentAddressedSnapshot,
            body_available: true,
            body_object_refs: vec!["snapshot.clean".to_owned()],
            dirty_range_summary_ref: None,
            group_member_refs: Vec::new(),
            omission_reason: CaptureOmissionReason::NotOmitted,
            capture_notes: "Full content-addressed snapshot captured.".to_owned(),
        },
        IntegrityRecord {
            checksum_algorithm: ChecksumAlgorithm::Blake3,
            checksum_ref: "checksum.clean".to_owned(),
            frame_integrity_state: FrameIntegrityState::Verified,
            replay_integrity_posture: ReplayIntegrityPosture::ReplayAllowed,
            last_good_frame_ref: None,
            failed_frame_ref: None,
            corruption_evidence_refs: Vec::new(),
            integrity_notes: "Frame verified.".to_owned(),
        },
        ReplayPostureRecord {
            object_class_replay_posture: ReplayPostureClass::RestoreAllowed,
            recommended_choice_class: GuidedChoiceClass::Restore,
            blocked_choice_classes: Vec::new(),
            downgrade_reason_classes: vec![DowngradeReasonClass::NotDowngraded],
            new_local_history_checkpoint_on_restore: Some(true),
            new_checkpoint_ref: None,
            open_without_replay_retains_journal: true,
            replay_notes: "Restore applies the captured snapshot bytes.".to_owned(),
        },
        RetentionPostureRecord {
            retention_class: RetentionClass::ActiveReplayWindow,
            local_only_default: true,
            ordinary_cache_clear_excluded: true,
            settings_reset_excluded: true,
            local_history_clear_excluded: true,
            journal_reset_required_for_delete: true,
            export_before_reset: "offered".to_owned(),
            expiry_policy_ref: None,
            pin_refs: Vec::new(),
        },
        SupportExportRecord {
            support_bundle_inclusion_state: SupportBundleInclusionState::MetadataRefAllowed,
            redaction_class: "metadata_only".to_owned(),
            support_export_refs: Vec::new(),
            export_notes: "Metadata refs only.".to_owned(),
        },
        "2026-05-25T00:00:00Z".to_owned(),
    )
}

fn typing_row(id: &str) -> ActorLineageRow {
    ActorLineageRow {
        row_id: id.to_owned(),
        display_label: "Typed edits".to_owned(),
        actor_lineage_class: ActorLineageClass::Typing,
        actor_class: "user_keystroke".to_owned(),
        source_class: "human_local".to_owned(),
        reversal_class: "exact_undo".to_owned(),
        redaction_class: "metadata_only".to_owned(),
        snapshot_class: "dirty_buffer_checkpoint".to_owned(),
        capture_mode: Some("content_addressed_snapshot".to_owned()),
        omission_reason: Some("not_omitted".to_owned()),
        local_history_entry_refs: vec!["entry.typing".to_owned()],
        local_history_group_ref: None,
        mutation_journal_ref: Some("mutation.typing".to_owned()),
        ai_apply_lineage: None,
        command_id: Some("cmd:editor.type".to_owned()),
        checkpoint_refs: vec!["entry.typing".to_owned()],
        side_effect_summary: Some("In-file text edit.".to_owned()),
        body_available_locally: true,
        raw_body_refs_exported: false,
    }
}

fn clean_packet() -> LocalHistoryAlphaPacket {
    LocalHistoryAlphaPacket::new(
        "packet.clean",
        "2026-05-25T00:00:00Z",
        LocalHistoryConsumerSurface::RestorePreview,
    )
    .with_actor_lineage_row(typing_row("row.typing"))
}

fn text_edit_group() -> UndoGroupObservation {
    UndoGroupObservation {
        undo_group_id: 1,
        class_id: "text_edit".to_owned(),
        compensation_posture: CompensationPostureClass::Compensatable,
        is_named_group: false,
        originator: "user_keystroke".to_owned(),
        label: None,
        operation_count: 3,
    }
}

fn named_group(label: Option<String>) -> UndoGroupObservation {
    UndoGroupObservation {
        undo_group_id: 2,
        class_id: "save_participant_group".to_owned(),
        compensation_posture: CompensationPostureClass::OnlyRevertible,
        is_named_group: true,
        originator: "command:save".to_owned(),
        label,
        operation_count: 2,
    }
}

#[test]
fn clean_recovery_is_stable_and_export_safe() {
    let record = project_recovery_state_lineage(
        "lineage.clean",
        &clean_entry(),
        &[
            text_edit_group(),
            named_group(Some("Save + format".to_owned())),
        ],
        &clean_packet(),
    );

    assert!(record.is_stable_qualified());
    assert!(record.is_support_export_safe());
    assert!(record.buffer_recovery.round_trip_provable);
    assert!(record.canonical_path_truth.wrong_target_write_guarded);
    assert!(record.restore_safety.no_rerun_guaranteed);
    assert_eq!(record.undo_grouping.len(), 2);
    assert_eq!(
        record.undo_grouping[0].recovery_action_class,
        UndoRecoveryClass::InverseReplay
    );
    assert_eq!(
        record.undo_grouping[1].recovery_action_class,
        UndoRecoveryClass::SnapshotRestore
    );
    assert!(record.undo_grouping.iter().all(|g| g.grouping_integrity_ok));
}

#[test]
fn unknown_encoding_narrows_source_fidelity() {
    let mut entry = clean_entry();
    entry.text_format.encoding_label = EncodingLabelClass::Unknown;
    entry.text_format.decoder_posture = DecoderPosture::Unknown;

    let record = project_recovery_state_lineage(
        "lineage.unknown",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    assert!(!record.buffer_recovery.round_trip_provable);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::SourceFidelityUnprovable));
}

#[test]
fn identity_drift_without_compare_guard_narrows_canonical_path() {
    let mut entry = clean_entry();
    entry.object_identity.identity_relation = IdentityRelation::SamePathDifferentObject;
    entry.base_on_disk_token.compare_before_write_required = false;

    let record = project_recovery_state_lineage(
        "lineage.drift",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    assert!(!record.canonical_path_truth.wrong_target_write_guarded);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::CanonicalPathUnproven));
}

#[test]
fn identity_drift_with_compare_guard_stays_stable() {
    let mut entry = clean_entry();
    entry.object_identity.identity_relation = IdentityRelation::SamePathDifferentObject;
    entry.base_on_disk_token.compare_before_write_required = true;
    entry.base_on_disk_token.external_change_state = ExternalChangeState::ExternalChangeDetected;
    entry.replay_posture.object_class_replay_posture = ReplayPostureClass::RestoreRequiresReview;

    let record = project_recovery_state_lineage(
        "lineage.drift_guarded",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    assert!(record.canonical_path_truth.wrong_target_write_guarded);
    assert!(record.is_stable_qualified());
}

#[test]
fn restore_recommended_without_body_narrows_rerun() {
    let mut entry = clean_entry();
    // A generated artifact whose body was not captured but whose recovery still
    // recommends restore would have to reconstruct (re-run) the content.
    entry.object_identity.object_class = ObjectClass::GeneratedArtifact;
    entry.capture_descriptor.capture_mode = CaptureMode::GroupManifestOnly;
    entry.capture_descriptor.body_available = false;
    entry.capture_descriptor.omission_reason =
        CaptureOmissionReason::OmittedGeneratedArtifactUseLineage;
    entry.replay_posture.object_class_replay_posture = ReplayPostureClass::RestoreRequiresReview;

    let record = project_recovery_state_lineage(
        "lineage.rerun",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    assert!(record.restore_safety.restore_recommended);
    assert!(!record.restore_safety.byte_restore_faithful);
    assert!(!record.restore_safety.no_rerun_guaranteed);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::RestoreWouldRerun));
}

#[test]
fn restore_without_checkpoint_narrows_destructive() {
    let mut entry = clean_entry();
    entry.replay_posture.new_local_history_checkpoint_on_restore = Some(false);

    let record = project_recovery_state_lineage(
        "lineage.destructive",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    assert!(record.restore_safety.restore_recommended);
    assert!(!record.restore_safety.restore_creates_new_checkpoint);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::DestructiveRestoreNoRecoveryPath));
}

#[test]
fn integrity_failure_with_unconditional_restore_is_inconsistent() {
    let mut entry = clean_entry();
    entry.integrity.frame_integrity_state = FrameIntegrityState::ChecksumMismatch;
    // Posture still claims unconditional restore despite the integrity failure.
    entry.replay_posture.object_class_replay_posture = ReplayPostureClass::RestoreAllowed;

    let record = project_recovery_state_lineage(
        "lineage.inconsistent",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    assert!(!record.buffer_recovery.integrity_verified);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::RecoveryIntegrityInconsistent));
}

#[test]
fn integrity_failure_that_downgrades_posture_stays_protective() {
    let mut entry = clean_entry();
    entry.integrity.frame_integrity_state = FrameIntegrityState::ChecksumMismatch;
    // Correct protective response: integrity failure downgrades to inspect-only.
    entry.replay_posture.object_class_replay_posture = ReplayPostureClass::InspectOnly;
    entry.replay_posture.recommended_choice_class = GuidedChoiceClass::InspectOnly;
    entry.replay_posture.downgrade_reason_classes = vec![DowngradeReasonClass::ChecksumMismatch];

    let record = project_recovery_state_lineage(
        "lineage.protective",
        &entry,
        &[text_edit_group()],
        &clean_packet(),
    );

    // Integrity is unverified, but the posture correctly refused restore, so no
    // inconsistency narrow fires and the record stays Stable.
    assert!(!record.buffer_recovery.integrity_verified);
    assert!(!record.restore_safety.restore_recommended);
    assert!(record.is_stable_qualified());
}

#[test]
fn named_group_without_label_narrows_grouping_contract() {
    let record = project_recovery_state_lineage(
        "lineage.unlabeled",
        &clean_entry(),
        &[named_group(None)],
        &clean_packet(),
    );

    assert!(record
        .undo_grouping
        .iter()
        .any(|g| !g.grouping_integrity_ok));
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::UndoGroupingContractViolation));
}

#[test]
fn leaked_raw_body_ref_narrows_export_safety() {
    let mut row = typing_row("row.leaky");
    row.checkpoint_refs = vec!["obj:blake3:deadbeef".to_owned()];
    let packet = LocalHistoryAlphaPacket::new(
        "packet.leaky",
        "2026-05-25T00:00:00Z",
        LocalHistoryConsumerSurface::SupportExport,
    )
    .with_actor_lineage_row(row);

    let record = project_recovery_state_lineage(
        "lineage.leaky",
        &clean_entry(),
        &[text_edit_group()],
        &packet,
    );

    assert!(!record.actor_lineage[0].export_safe);
    assert!(!record.is_support_export_safe());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&RecoveryNarrowReason::ActorLineageExportUnsafe));
}

#[test]
fn observation_from_live_buffer_journal_matches_taxonomy() {
    let mut buffer = Buffer::from_str("a=1\nb=2\n");
    let mut tx = buffer
        .begin(
            TransactionSpec::new(UndoClass::SaveParticipantGroup, "command:save")
                .with_label("Save + format"),
        )
        .unwrap();
    tx.replace(0..3, "a = 1").unwrap();
    tx.commit().unwrap();

    let entry = buffer.peek_undo().expect("a committed group must exist");
    let observation = UndoGroupObservation::from_journal_entry(&entry);

    assert_eq!(observation.class_id, "save_participant_group");
    assert_eq!(
        observation.compensation_posture,
        CompensationPostureClass::OnlyRevertible
    );
    assert!(observation.is_named_group);
    assert_eq!(observation.label.as_deref(), Some("Save + format"));
    assert!(observation.grouping_integrity_ok());
}

#[test]
fn lines_render_all_three_pillars() {
    let record = project_recovery_state_lineage(
        "lineage.lines",
        &clean_entry(),
        &[text_edit_group(), named_group(Some("Save".to_owned()))],
        &clean_packet(),
    );
    let lines = recovery_state_lineage_lines(&record);

    assert!(lines.iter().any(|l| l.contains("Recovery state lineage")));
    assert!(lines.iter().any(|l| l.contains("Undo groups:")));
    assert!(lines.iter().any(|l| l.contains("Actor lineage")));
    assert!(lines.iter().any(|l| l.contains("save_participant_group")));
    assert!(lines.iter().any(|l| l.contains("row.typing")));
}
