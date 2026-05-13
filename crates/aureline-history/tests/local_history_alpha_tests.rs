use aureline_history::checkpoints::{
    AliasSetRecord, CanonicalFilesystemObjectRecord, CaptureDescriptor, CaptureMode,
    CaptureOmissionReasonClass, FilesystemIdentityRecord, IdentityTokenRecord,
    LocalHistoryEntryRecord, LocalHistoryGroupKind, LocalHistoryGroupRecord,
    LocalHistoryGroupResolution, LogicalDocumentIdentity, LogicalWorkspaceIdentityRecord,
    MutationJournalLink, MutationJournalLinkActorClass, MutationJournalLinkKind,
    MutationJournalLinkReversalClass, PresentationPathRecord, RestoreOfEntryRef,
    RestorePreviewRequiredFields, SnapshotClass,
};
use aureline_history::{
    ActorLineageClass, ActorLineageRow, HistoryExportMode, LocalHistoryAlphaPacket,
    LocalHistoryConsumerSurface, RedactionClass, RestoreCheckpointAlpha, RetentionScopeClass,
    SourceClass,
};

fn fixture_filesystem_identity() -> FilesystemIdentityRecord {
    FilesystemIdentityRecord {
        record_kind: "filesystem_identity_record".to_owned(),
        filesystem_identity_schema_version: 1,
        presentation_path: PresentationPathRecord {
            uri: "file:///workspace/src/lib.rs".to_owned(),
            display_label: "lib.rs".to_owned(),
            root_badge: "local".to_owned(),
        },
        logical_workspace_identity: LogicalWorkspaceIdentityRecord {
            workspace_id: "ws-lineage".to_owned(),
            root_id: "root-lineage".to_owned(),
            logical_uri: "aureline://src/lib.rs".to_owned(),
            trust_state: "trusted".to_owned(),
            policy_scope: None,
        },
        canonical_filesystem_object: CanonicalFilesystemObjectRecord {
            canonical_uri: "file:///workspace/src/lib.rs".to_owned(),
            normalization_form: "posix".to_owned(),
            strongest_identity_token: IdentityTokenRecord {
                kind: "inode".to_owned(),
                value: "lineage-fixture".to_owned(),
            },
            fallback_identity_tokens: Vec::new(),
        },
        alias_set: AliasSetRecord {
            aliases: Vec::new(),
        },
    }
}

fn entry_with_actor(
    entry_id: &str,
    actor_class: MutationJournalLinkActorClass,
    source_class: SourceClass,
    snapshot_class: SnapshotClass,
) -> LocalHistoryEntryRecord {
    LocalHistoryEntryRecord::new(
        entry_id.to_owned(),
        snapshot_class,
        "2026-05-13T22:00:00Z".to_owned(),
        LogicalDocumentIdentity {
            logical_document_id: "ld-lineage".to_owned(),
            current_filesystem_identity: fixture_filesystem_identity(),
            canonical_identity_drift: None,
            rename_move_history: Vec::new(),
        },
        CaptureDescriptor {
            capture_mode: CaptureMode::ContentAddressedSnapshot,
            omission_reason: CaptureOmissionReasonClass::NotOmitted,
            body_available: true,
            body_object_refs: vec!["obj:blake3:not-exported".to_owned()],
            reference_digest: None,
            bytes_estimated: Some(128),
            omission_note: None,
        },
        MutationJournalLink {
            linked_kind: MutationJournalLinkKind::MutationJournalEntry,
            linked_id: format!("m-{entry_id}"),
            actor_class: Some(actor_class),
            source_class: Some(source_class),
            reversal_class: Some(MutationJournalLinkReversalClass::ExactUndo),
            redaction_class: Some(RedactionClass::CodeAdjacent),
        },
        RetentionScopeClass::RetainedByPolicyWindow,
        Some(format!("checkpoint for {entry_id}")),
    )
}

#[test]
fn protected_fixture_packet_is_export_safe_and_covers_required_classes() {
    let raw = include_str!(
        "../../../fixtures/history/actor_lineage_alpha/protected_actor_lineage_packet.json"
    );
    let packet: LocalHistoryAlphaPacket = serde_json::from_str(raw).expect("fixture parses");

    packet.validate().expect("fixture is export safe");
    assert_eq!(
        packet.export_safety.export_mode,
        HistoryExportMode::MetadataOnly
    );
    assert!(packet.missing_required_alpha_coverage().is_empty());
    assert!(packet
        .actor_lineage_rows
        .iter()
        .any(|row| row.actor_lineage_class == ActorLineageClass::GitMutation));
    assert_eq!(packet.restore_checkpoints.len(), 1);
    assert!(!packet.restore_checkpoints[0].checkpoint_name.is_empty());
}

#[test]
fn entry_projection_distinguishes_typing_import_and_formatter_rows() {
    let typing = entry_with_actor(
        "lh-typing-fixture",
        MutationJournalLinkActorClass::UserKeystroke,
        SourceClass::HumanLocal,
        SnapshotClass::EditSaveCheckpoint,
    );
    let import = entry_with_actor(
        "lh-import-fixture",
        MutationJournalLinkActorClass::PasteOrDropImport,
        SourceClass::HumanLocal,
        SnapshotClass::EditSaveCheckpoint,
    );
    let formatter = entry_with_actor(
        "lh-format-fixture",
        MutationJournalLinkActorClass::Formatter,
        SourceClass::MachineLocal,
        SnapshotClass::EditSaveCheckpoint,
    );

    let typing_row = ActorLineageRow::try_from_entry(
        "row-typing",
        "typing checkpoint",
        Some("editor.type".to_owned()),
        &typing,
    )
    .expect("typing row");
    let import_row = ActorLineageRow::try_from_entry(
        "row-import",
        "import checkpoint",
        Some("editor.paste".to_owned()),
        &import,
    )
    .expect("import row");
    let formatter_row = ActorLineageRow::try_from_entry(
        "row-format",
        "formatter checkpoint",
        Some("editor.format_on_save".to_owned()),
        &formatter,
    )
    .expect("formatter row");

    assert_eq!(typing_row.actor_lineage_class, ActorLineageClass::Typing);
    assert_eq!(import_row.actor_lineage_class, ActorLineageClass::Import);
    assert_eq!(
        formatter_row.actor_lineage_class,
        ActorLineageClass::Formatter
    );
    assert!(!typing_row.raw_body_refs_exported);
    assert!(
        !typing_row
            .checkpoint_refs
            .iter()
            .any(|reference| reference.starts_with("obj:")),
        "projection must cite entry refs, not body refs"
    );
}

#[test]
fn group_and_restore_projection_surface_names_and_support_refs() {
    let group = LocalHistoryGroupRecord::new(
        "lhg-ai-fixture".to_owned(),
        LocalHistoryGroupKind::AiPatch,
        SnapshotClass::AutomationAiCheckpoint,
        "2026-05-13T22:01:00Z".to_owned(),
        "2026-05-13T22:01:03Z".to_owned(),
        LocalHistoryGroupResolution::Applied,
        vec!["lh-ai-a".to_owned(), "lh-ai-b".to_owned()],
        MutationJournalLink {
            linked_kind: MutationJournalLinkKind::MutationGroupRecord,
            linked_id: "g-ai-fixture".to_owned(),
            actor_class: Some(MutationJournalLinkActorClass::AiApply),
            source_class: Some(SourceClass::AiHostedProvider),
            reversal_class: Some(MutationJournalLinkReversalClass::CompensatingUndo),
            redaction_class: Some(RedactionClass::CodeAdjacent),
        },
        RetentionScopeClass::RetainedByEvidenceReference,
        Some("AI apply group".to_owned()),
    );
    let group_row = ActorLineageRow::try_from_group(
        "row-ai",
        "AI apply group",
        Some("ai.apply_patch".to_owned()),
        &group,
    )
    .expect("group row");
    assert_eq!(group_row.actor_lineage_class, ActorLineageClass::AiApply);

    let restore = entry_with_actor(
        "lh-restore-fixture",
        MutationJournalLinkActorClass::RestoreRollbackRunner,
        SourceClass::HumanLocal,
        SnapshotClass::RestoreRollbackCheckpoint,
    )
    .with_restore_of_ref(RestoreOfEntryRef {
        restored_from_entry_id: "lh-typing-fixture".to_owned(),
        restore_preview: RestorePreviewRequiredFields {
            source_entry_ref: "lh-typing-fixture".to_owned(),
            last_known_canonical_identity_ref: "fsid-before".to_owned(),
            current_canonical_identity_ref: "fsid-now".to_owned(),
            canonical_identity_drift: "no_drift".to_owned(),
            rename_move_chain_ref: "rmc-none".to_owned(),
            body_availability: "body_captured_in_object_store".to_owned(),
            resulting_snapshot_class: "restore_rollback_checkpoint".to_owned(),
            new_checkpoint_entry_ref: "lh-restore-fixture".to_owned(),
            note: None,
        },
    });
    let checkpoint = RestoreCheckpointAlpha::try_from_restore_entry(
        "Restore checkpoint after replay",
        "support.restore.fixture",
        &restore,
    )
    .expect("restore checkpoint");

    let packet = LocalHistoryAlphaPacket::new(
        "packet-restore",
        "2026-05-13T22:01:05Z",
        LocalHistoryConsumerSurface::SupportExport,
    )
    .with_actor_lineage_row(group_row)
    .with_actor_lineage_row(
        ActorLineageRow::try_from_entry(
            "row-restore",
            "restore checkpoint",
            Some("cmd:local_history.restore_checkpoint".to_owned()),
            &restore,
        )
        .expect("restore row"),
    )
    .with_restore_checkpoint(checkpoint);

    packet.validate().expect("packet validates");
    assert_eq!(
        packet.restore_checkpoints[0].checkpoint_name,
        "Restore checkpoint after replay"
    );
    assert_eq!(
        packet.actor_lineage_rows[1].actor_lineage_class,
        ActorLineageClass::RestoreCheckpoint
    );
}
