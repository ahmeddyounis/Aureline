use aureline_recovery::crash_journal::{CrashJournalCaptureInput, CrashJournalStore, ObjectClass};
use aureline_recovery::session_restore::records::{
    DowngradeTriggerClass, ProducerBuildStamp, RestoreClass, SurfaceClass, SurfaceRole, WindowRole,
};
use aureline_recovery::session_restore::{
    RestorePaneExecutionKind, RestoreProposal, RestoreRuntime, SessionRestoreCaptureInput,
    SessionRestoreStore, TabGroupCaptureInput, TabItemCaptureInput,
};

fn producer() -> ProducerBuildStamp {
    ProducerBuildStamp {
        producer_name: "aureline-recovery-execute-test".to_string(),
        producer_version: "0.0.0".to_string(),
        producer_channel: None,
        producer_platform_class: None,
        producer_instance_handle: None,
    }
}

fn capture_layout(store: &mut SessionRestoreStore, with_terminal: bool) {
    let mut tabs = vec![TabItemCaptureInput {
        tab_id: "tab-edit-router".to_string(),
        tab_label: Some("router.ts".to_string()),
        pinned: false,
        dirty_badge_visible: true,
        surface_role: SurfaceRole::Editor,
        surface_class: SurfaceClass::TextEditor,
    }];
    if with_terminal {
        tabs.push(TabItemCaptureInput {
            tab_id: "tab-terminal".to_string(),
            tab_label: Some("zsh".to_string()),
            pinned: false,
            dirty_badge_visible: false,
            surface_role: SurfaceRole::Terminal,
            surface_class: SurfaceClass::TerminalView,
        });
    }

    store
        .capture(SessionRestoreCaptureInput {
            workspace_ref: "ws-restore-execute".to_string(),
            producer_build: producer(),
            source_schema_version: "1".to_string(),
            trusted_root_refs: Vec::new(),
            active_workset_ids: Vec::new(),
            dirty_buffer_journal_identities: Vec::new(),
            recovery_journal_refs: vec!["recovery:packet:1".to_string()],
            local_history_snapshot_refs: Vec::new(),
            evidence_bundle_refs: vec!["evidence:packet:1".to_string()],
            excluded_live_authority_classes: Vec::new(),
            downgrade_triggers: Vec::new(),
            window_id: "win-primary".to_string(),
            window_role: WindowRole::Primary,
            topology_family_ref: None,
            sibling_window_refs: Vec::new(),
            tab_groups: vec![TabGroupCaptureInput {
                group_id: "tg-main".to_string(),
                ordered_tabs: tabs,
                active_tab_id: Some("tab-edit-router".to_string()),
            }],
            emitted_at: "mono:test:00001".to_string(),
            notes: None,
        })
        .expect("capture layout");
}

fn capture_dirty_buffer(store: &mut CrashJournalStore, bytes: &[u8]) {
    store
        .capture_minimal_full_snapshot(CrashJournalCaptureInput {
            journal_id: "journal:ws-restore-execute".to_string(),
            workspace_ref: "ws-restore-execute".to_string(),
            logical_document_id: "ld:router".to_string(),
            object_ref: "aureline-ws://ws-shell_proto/root-local/src/router.ts".to_string(),
            object_class: ObjectClass::CanonicalFile,
            presentation_hint: Some("router.ts".to_string()),
            emitted_at: "mono:test:00002".to_string(),
            bytes: bytes.to_vec(),
        })
        .expect("capture dirty buffer");
}

#[test]
fn exact_restore_reopens_panes_and_replays_verified_dirty_buffer() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut session_store = SessionRestoreStore::new(dir.path());
    let mut crash_store = CrashJournalStore::new(dir.path());
    capture_layout(&mut session_store, false);
    capture_dirty_buffer(&mut crash_store, b"restored bytes\n");

    let mut proposal =
        RestoreProposal::build(&session_store, &crash_store, true).expect("build proposal");
    proposal.restore_class = RestoreClass::ExactRestore;
    let mut runtime = RestoreRuntime::new(&session_store, &crash_store);
    let outcome = proposal.execute(&mut runtime);

    assert!(outcome.succeeded_without_failures());
    assert_eq!(outcome.restore_class, RestoreClass::ExactRestore);
    assert_eq!(outcome.pane_outcomes.len(), 1);
    assert_eq!(
        outcome.pane_outcomes[0].execution_kind,
        RestorePaneExecutionKind::Reopened
    );
    assert_eq!(outcome.dirty_buffer_replays.len(), 1);
    assert_eq!(outcome.dirty_buffer_replays[0].bytes, b"restored bytes\n");
}

#[test]
fn layout_only_reopens_layout_without_dirty_buffer_replay() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut session_store = SessionRestoreStore::new(dir.path());
    let crash_store = CrashJournalStore::new(dir.path());
    capture_layout(&mut session_store, false);

    let proposal =
        RestoreProposal::build(&session_store, &crash_store, false).expect("build proposal");
    let mut runtime = RestoreRuntime::new(&session_store, &crash_store);
    let outcome = proposal.execute(&mut runtime);

    assert_eq!(outcome.restore_class, RestoreClass::LayoutOnly);
    assert_eq!(outcome.pane_outcomes.len(), 1);
    assert_eq!(
        outcome.pane_outcomes[0].execution_kind,
        RestorePaneExecutionKind::Reopened
    );
    assert!(outcome.dirty_buffer_replays.is_empty());
    assert!(outcome.dirty_buffer_failures.is_empty());
}

#[test]
fn manual_repair_required_keeps_corrupt_dirty_buffer_out_of_replay() {
    let dir = tempfile::tempdir().expect("tempdir");
    let session_store = SessionRestoreStore::new(dir.path());
    let mut crash_store = CrashJournalStore::new(dir.path());
    capture_dirty_buffer(&mut crash_store, b"unsafe bytes\n");

    let mut proposal =
        RestoreProposal::build(&session_store, &crash_store, true).expect("build proposal");
    proposal.downgrade_triggers = vec![DowngradeTriggerClass::ManualRepairRequired];
    proposal.dirty_buffer_entries[0].frame_integrity =
        aureline_recovery::crash_journal::FrameIntegrityState::TruncatedFrame;

    let mut runtime = RestoreRuntime::new(&session_store, &crash_store);
    let outcome = proposal.execute(&mut runtime);

    assert!(outcome.manual_repair_required);
    assert!(outcome.dirty_buffer_replays.is_empty());
    assert_eq!(outcome.dirty_buffer_failures.len(), 1);
}

#[test]
fn side_effectful_terminal_surface_stays_blocked_and_inactive() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut session_store = SessionRestoreStore::new(dir.path());
    let crash_store = CrashJournalStore::new(dir.path());
    capture_layout(&mut session_store, true);

    let proposal =
        RestoreProposal::build(&session_store, &crash_store, false).expect("build proposal");
    let mut runtime = RestoreRuntime::new(&session_store, &crash_store);
    let outcome = proposal.execute(&mut runtime);

    assert_eq!(outcome.blocked_side_effectful_count(), 1);
    let terminal = outcome
        .pane_outcomes
        .iter()
        .find(|pane| pane.surface_role == SurfaceRole::Terminal)
        .expect("terminal pane outcome");
    assert_eq!(
        terminal.execution_kind,
        RestorePaneExecutionKind::BlockedSideEffectful
    );
}
