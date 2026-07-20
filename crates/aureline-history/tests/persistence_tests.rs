use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_history::checkpoints::{
    AliasSetRecord, CanonicalFilesystemObjectRecord, CaptureDescriptor, CaptureMode,
    CaptureOmissionReasonClass, FilesystemIdentityRecord, IdentityTokenRecord,
    LocalHistoryEntryRecord, LocalHistoryGroupKind, LocalHistoryGroupRecord,
    LocalHistoryGroupResolution, LogicalDocumentIdentity, LogicalWorkspaceIdentityRecord,
    MutationJournalLink, MutationJournalLinkKind, PresentationPathRecord, RetentionScopeClass,
    SnapshotClass,
};
use aureline_history::mutation_journal::{MutationGroupKind, MutationGroupResolution};
use aureline_history::{
    ActorClass, ActorRef, DurableVsDisposable, HistoryStorageRoot, MutationGroupRecord,
    MutationJournalEntryRecord, MutationJournalStore, RedactionClass, ReversalClass, ScopeClass,
    ScopeRef, SideEffectSummary, SourceClass, TargetKind, TargetRef,
};

fn unique_temp_root(label: &str) -> PathBuf {
    let pid = std::process::id();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut root = std::env::temp_dir();
    root.push(format!("aureline_history_{label}_{pid}_{stamp}"));
    root
}

fn fixture_filesystem_identity() -> FilesystemIdentityRecord {
    FilesystemIdentityRecord {
        record_kind: "filesystem_identity_record".to_owned(),
        filesystem_identity_schema_version: 1,
        presentation_path: PresentationPathRecord {
            uri: "file:///tmp/aureline-history-fixture.txt".to_owned(),
            display_label: "aureline-history-fixture.txt".to_owned(),
            root_badge: "fixture".to_owned(),
        },
        logical_workspace_identity: LogicalWorkspaceIdentityRecord {
            workspace_id: "ws-fixture".to_owned(),
            root_id: "root-fixture".to_owned(),
            logical_uri: "file:///tmp".to_owned(),
            trust_state: "trusted".to_owned(),
            policy_scope: None,
        },
        canonical_filesystem_object: CanonicalFilesystemObjectRecord {
            canonical_uri: "file:///tmp/aureline-history-fixture.txt".to_owned(),
            normalization_form: "posix".to_owned(),
            strongest_identity_token: IdentityTokenRecord {
                kind: "inode".to_owned(),
                value: "fixture".to_owned(),
            },
            fallback_identity_tokens: Vec::new(),
        },
        alias_set: AliasSetRecord {
            aliases: Vec::new(),
        },
    }
}

#[test]
fn writes_local_history_entry_and_content_addressed_body() {
    let root = unique_temp_root("local_history");
    let storage = HistoryStorageRoot::new(&root);
    let mut store = aureline_history::LocalHistoryStore::new(storage);
    let entry_id = store.mint_entry_id();

    let body_ref = store
        .write_body_object(b"fixture-body")
        .expect("body write succeeds");

    let filesystem_identity = fixture_filesystem_identity();
    let entry = LocalHistoryEntryRecord::new(
        entry_id.clone(),
        SnapshotClass::EditSaveCheckpoint,
        "t-1".to_owned(),
        LogicalDocumentIdentity {
            logical_document_id: "ld-fixture".to_owned(),
            current_filesystem_identity: filesystem_identity.clone(),
            canonical_identity_drift: None,
            rename_move_history: Vec::new(),
        },
        CaptureDescriptor {
            capture_mode: CaptureMode::ContentAddressedSnapshot,
            omission_reason: CaptureOmissionReasonClass::NotOmitted,
            body_available: true,
            body_object_refs: vec![body_ref.clone()],
            reference_digest: None,
            bytes_estimated: Some("fixture-body".len() as u64),
            omission_note: None,
        },
        MutationJournalLink {
            linked_kind: MutationJournalLinkKind::MutationJournalEntry,
            linked_id: "m-fixture".to_owned(),
            actor_class: None,
            source_class: None,
            reversal_class: None,
            redaction_class: None,
            ai_apply_lineage: None,
        },
        RetentionScopeClass::RetainedByPolicyWindow,
        Some("fixture entry".to_owned()),
    );

    let path = store.write_entry(&entry).expect("entry write succeeds");
    assert!(path.exists(), "entry record persisted");

    let raw = fs::read_to_string(&path).expect("entry record readable");
    let parsed: LocalHistoryEntryRecord = serde_json::from_str(&raw).expect("json is valid");
    assert_eq!(parsed.entry_id, entry_id);

    assert!(
        store.write_entry(&entry).is_err(),
        "entry ids are immutable"
    );
    assert_eq!(
        fs::read(&path).expect("immutable entry remains readable"),
        raw.as_bytes(),
        "a duplicate publication must not replace the original record"
    );
    assert_eq!(
        store
            .read_entry_body_for_workspace(&entry_id, "ws-fixture", &body_ref)
            .expect("scoped body read"),
        b"fixture-body"
    );
    assert!(matches!(
        store.read_entry_body_for_workspace(&entry_id, "ws-other", &body_ref),
        Err(aureline_history::HistoryError::InvalidInput(_))
    ));

    let grouped_entry_id = store.mint_entry_id();
    let mut grouped_entry = entry.clone();
    grouped_entry.entry_id = grouped_entry_id.clone();
    grouped_entry.group_id = Some("lhg-real".to_owned());
    store
        .write_entry(&grouped_entry)
        .expect("write group-bound entry");
    let legitimate_group = LocalHistoryGroupRecord::new(
        "lhg-real".to_owned(),
        LocalHistoryGroupKind::SaveParticipantGroup,
        SnapshotClass::EditSaveCheckpoint,
        "t-1".to_owned(),
        "t-1".to_owned(),
        LocalHistoryGroupResolution::Applied,
        vec![grouped_entry_id],
        grouped_entry.mutation_journal_link.clone(),
        RetentionScopeClass::RetainedByPolicyWindow,
        Some("grouped checkpoint".to_owned()),
    );
    let mut spliced_group = legitimate_group.clone();
    spliced_group.group_id = "lhg-spliced".to_owned();
    assert!(matches!(
        store.write_group(&spliced_group),
        Err(aureline_history::HistoryError::InvalidInput(_))
    ));
    store
        .write_group(&legitimate_group)
        .expect("matching group publication");

    let mut unsafe_id = entry.clone();
    unsafe_id.entry_id = "../../outside".to_owned();
    assert!(matches!(
        store.write_entry(&unsafe_id),
        Err(aureline_history::HistoryError::InvalidInput(_))
    ));
    assert!(!root.join("outside.json").exists());

    let mut dishonest_restore = entry.clone();
    dishonest_restore.entry_id = store.mint_entry_id();
    dishonest_restore.snapshot_class = SnapshotClass::RestoreRollbackCheckpoint;
    assert!(matches!(
        store.write_entry(&dishonest_restore),
        Err(aureline_history::HistoryError::InvalidInput(_))
    ));

    let body_path = store.objects_root_path().join(format!(
        "{}.blob",
        body_ref
            .strip_prefix("obj:blake3:")
            .expect("content-addressed ref")
    ));
    fs::write(&body_path, b"tampered-body").expect("tamper body");
    assert!(matches!(
        store.read_body_object(&body_ref),
        Err(aureline_history::HistoryError::Integrity(_))
    ));

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn history_storage_rejects_escape_symlinks_and_oversized_records() {
    let root = unique_temp_root("storage_boundaries");
    let storage = HistoryStorageRoot::new(&root);
    let outside = root
        .parent()
        .expect("temp root parent")
        .join("aureline-history-outside.bin");
    assert!(matches!(
        storage.write_new_blob(&outside, b"escape"),
        Err(aureline_history::HistoryError::InvalidInput(_))
    ));
    assert!(!outside.exists());

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        fs::create_dir_all(&root).expect("create root");
        let victim_dir = unique_temp_root("storage_symlink_victim");
        fs::create_dir_all(&victim_dir).expect("create victim dir");
        symlink(&victim_dir, root.join("objects")).expect("symlink objects");
        let error = storage
            .write_new_blob(&root.join("objects/escaped.blob"), b"escape")
            .expect_err("symlinked store directory must fail closed");
        assert!(matches!(
            error,
            aureline_history::HistoryError::InvalidInput(_)
        ));
        assert!(!victim_dir.join("escaped.blob").exists());
        let _ = fs::remove_dir_all(&victim_dir);
    }

    #[derive(serde::Serialize)]
    struct OversizedRecord {
        body: String,
    }
    let oversized = OversizedRecord {
        body: "x".repeat(2 * 1024 * 1024 + 1),
    };
    let record_path = root.join("records/too-large.json");
    assert!(matches!(
        storage.write_new_json(&record_path, &oversized),
        Err(aureline_history::HistoryError::TooLarge(_))
    ));
    assert!(!record_path.exists());

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn local_history_refuses_preexisting_wrong_digest_body() {
    let root = unique_temp_root("wrong_digest");
    let storage = HistoryStorageRoot::new(&root);
    let store = aureline_history::LocalHistoryStore::new(storage);
    let wanted = b"wanted-body";
    let object_ref = aureline_history::body_object_id(wanted);
    let digest = object_ref
        .strip_prefix("obj:blake3:")
        .expect("object digest");
    let objects = store.objects_root_path();
    fs::create_dir_all(&objects).expect("create object root");
    let path = objects.join(format!("{digest}.blob"));
    fs::write(&path, b"wrong-existing-body").expect("seed wrong body");

    assert!(matches!(
        store.write_body_object(wanted),
        Err(aureline_history::HistoryError::Integrity(_))
    ));
    assert_eq!(
        fs::read(&path).expect("wrong body remains quarantined"),
        b"wrong-existing-body"
    );

    let oversized_ref = aureline_history::body_object_id(b"oversized-placeholder");
    let oversized_digest = oversized_ref
        .strip_prefix("obj:blake3:")
        .expect("oversized digest");
    let oversized_path = objects.join(format!("{oversized_digest}.blob"));
    let oversized_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&oversized_path)
        .expect("create sparse body");
    oversized_file
        .set_len(64 * 1024 * 1024 + 1)
        .expect("extend sparse body");
    assert!(matches!(
        store.read_body_object(&oversized_ref),
        Err(aureline_history::HistoryError::TooLarge(_))
    ));

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn writes_mutation_journal_entry() {
    let root = unique_temp_root("mutation_journal");
    let storage = HistoryStorageRoot::new(&root);
    let mut store = MutationJournalStore::new(storage);
    let mutation_id = store.mint_mutation_id();

    let entry = MutationJournalEntryRecord::new(
        mutation_id.clone(),
        "editor.type".to_owned(),
        ActorClass::UserKeystroke,
        SourceClass::HumanLocal,
        ActorRef {
            display_name: "fixture user".to_owned(),
            stable_id: None,
            role: Some("author".to_owned()),
        },
        ScopeRef {
            class: ScopeClass::Buffer,
            id: "buf:ld-fixture".to_owned(),
        },
        vec![TargetRef {
            target_kind: TargetKind::Buffer,
            filesystem_identity: Some(fixture_filesystem_identity()),
            logical_ref: Some("ld-fixture".to_owned()),
            affected_range: None,
        }],
        "t-1".to_owned(),
        "t-1".to_owned(),
        "text_edit".to_owned(),
        ReversalClass::ExactUndo,
        RedactionClass::CodeAdjacent,
        DurableVsDisposable::DurableUserAuthored,
        SideEffectSummary::new("fixture mutation"),
        Vec::new(),
    );

    let path = store.write_entry(&entry).expect("journal write succeeds");
    assert!(path.exists(), "journal record persisted");

    let raw = fs::read_to_string(&path).expect("journal record readable");
    let parsed: MutationJournalEntryRecord = serde_json::from_str(&raw).expect("json is valid");
    assert_eq!(parsed.mutation_id, mutation_id);

    let grouped_mutation_id = store.mint_mutation_id();
    let mut grouped_entry = entry.clone();
    grouped_entry.mutation_id = grouped_mutation_id.clone();
    grouped_entry.group_id = Some("g-real".to_owned());
    store
        .write_entry(&grouped_entry)
        .expect("write group-bound mutation");
    let legitimate_group = MutationGroupRecord::new(
        "g-real".to_owned(),
        MutationGroupKind::SaveParticipantGroup,
        "editor.type".to_owned(),
        ActorClass::UserKeystroke,
        SourceClass::HumanLocal,
        ActorRef {
            display_name: "fixture user".to_owned(),
            stable_id: None,
            role: Some("author".to_owned()),
        },
        ScopeRef {
            class: ScopeClass::Buffer,
            id: "buf:ld-fixture".to_owned(),
        },
        "t-1".to_owned(),
        "t-1".to_owned(),
        MutationGroupResolution::Applied,
        vec![grouped_mutation_id],
        ReversalClass::ExactUndo,
        RedactionClass::CodeAdjacent,
        DurableVsDisposable::DurableUserAuthored,
        SideEffectSummary::new("fixture group"),
        Vec::new(),
    );
    let mut spliced_group = legitimate_group.clone();
    spliced_group.group_id = "g-spliced".to_owned();
    assert!(matches!(
        store.write_group(&spliced_group),
        Err(aureline_history::HistoryError::InvalidInput(_))
    ));
    store
        .write_group(&legitimate_group)
        .expect("matching mutation group publication");

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn rejects_unregistered_mutation_journal_record_kind() {
    let root = unique_temp_root("mutation_journal_bad_kind");
    let storage = HistoryStorageRoot::new(&root);
    let mut store = MutationJournalStore::new(storage);
    let mutation_id = store.mint_mutation_id();

    let mut entry = MutationJournalEntryRecord::new(
        mutation_id,
        "editor.type".to_owned(),
        ActorClass::UserKeystroke,
        SourceClass::HumanLocal,
        ActorRef {
            display_name: "fixture user".to_owned(),
            stable_id: None,
            role: Some("author".to_owned()),
        },
        ScopeRef {
            class: ScopeClass::Buffer,
            id: "buf:ld-fixture".to_owned(),
        },
        vec![TargetRef {
            target_kind: TargetKind::Buffer,
            filesystem_identity: Some(fixture_filesystem_identity()),
            logical_ref: Some("ld-fixture".to_owned()),
            affected_range: None,
        }],
        "t-1".to_owned(),
        "t-1".to_owned(),
        "text_edit".to_owned(),
        ReversalClass::ExactUndo,
        RedactionClass::CodeAdjacent,
        DurableVsDisposable::DurableUserAuthored,
        SideEffectSummary::new("fixture mutation"),
        Vec::new(),
    );
    entry.record_kind = "unregistered_mutation_record".to_owned();

    let error = store
        .write_entry(&entry)
        .expect_err("unregistered record kind rejected");
    assert!(matches!(
        error,
        aureline_history::HistoryError::RecordRegistry(
            aureline_records::RecordRegistryError::UnknownRecordKind { .. }
        )
    ));

    let _ = fs::remove_dir_all(&root);
}
