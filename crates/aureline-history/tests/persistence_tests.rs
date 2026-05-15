use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_history::checkpoints::{
    AliasSetRecord, CanonicalFilesystemObjectRecord, CaptureDescriptor, CaptureMode,
    CaptureOmissionReasonClass, FilesystemIdentityRecord, IdentityTokenRecord,
    LocalHistoryEntryRecord, LogicalDocumentIdentity, LogicalWorkspaceIdentityRecord,
    MutationJournalLink, MutationJournalLinkKind, PresentationPathRecord, RetentionScopeClass,
    SnapshotClass,
};
use aureline_history::{
    ActorClass, ActorRef, DurableVsDisposable, HistoryStorageRoot, MutationJournalEntryRecord,
    MutationJournalStore, RedactionClass, ReversalClass, ScopeClass, ScopeRef, SideEffectSummary,
    SourceClass, TargetKind, TargetRef,
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
            body_object_refs: vec![body_ref],
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
