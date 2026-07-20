// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

use aureline_recovery::crash_journal::{CrashJournalCaptureInput, CrashJournalStore, ObjectClass};
use aureline_recovery::session_restore::records::{
    DowngradeTriggerClass, ProducerBuildStamp, RestoreClass, SurfaceClass, SurfaceRole,
    TerminalPaneRestoreMetadata, WindowRole,
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
        surface_binding_ref: Some("document:router".to_string()),
        pinned: false,
        dirty_badge_visible: true,
        surface_role: SurfaceRole::Editor,
        surface_class: SurfaceClass::TextEditor,
        restore_metadata: None,
    }];
    if with_terminal {
        tabs.push(TabItemCaptureInput {
            tab_id: "tab-terminal".to_string(),
            tab_label: Some("zsh".to_string()),
            surface_binding_ref: None,
            pinned: false,
            dirty_badge_visible: false,
            surface_role: SurfaceRole::Terminal,
            surface_class: SurfaceClass::TerminalView,
            restore_metadata: Some(TerminalPaneRestoreMetadata {
                restore_metadata_ref: "terminal-restore-metadata:tab-terminal".to_string(),
                working_directory: Some("/workspace/service".to_string()),
                environment_scope_token: "workspace".to_string(),
                shell_identity: "zsh".to_string(),
                shell_family_token: "zsh".to_string(),
                last_command_class_token: "build".to_string(),
                auto_rerun_forbidden: true,
                raw_command_body_present: false,
                raw_environment_body_present: false,
            }),
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
            pane_tree_layout: None,
            focused_group_id: Some("tg-main".to_string()),
            emitted_at: "mono:test:00001".to_string(),
            notes: None,
        })
        .expect("capture layout");
}

fn capture_dirty_buffer(store: &mut CrashJournalStore, bytes: &[u8]) {
    capture_dirty_buffer_for_workspace(store, "ws-restore-execute", "router", bytes);
}

fn capture_dirty_buffer_for_workspace(
    store: &mut CrashJournalStore,
    workspace_ref: &str,
    document: &str,
    bytes: &[u8],
) {
    store
        .capture_minimal_full_snapshot(CrashJournalCaptureInput {
            journal_id: format!("journal:{workspace_ref}"),
            workspace_ref: workspace_ref.to_string(),
            logical_document_id: format!("ld:{document}"),
            object_ref: format!("document:{document}"),
            object_class: ObjectClass::CanonicalFile,
            presentation_hint: Some(format!("{document}.ts")),
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
    capture_dirty_buffer_for_workspace(
        &mut crash_store,
        "ws-foreign",
        "foreign-secret",
        b"foreign bytes must never cross scope\n",
    );

    let mut proposal =
        RestoreProposal::build(&session_store, &crash_store, true).expect("build proposal");
    proposal.restore_class = RestoreClass::ExactRestore;
    assert_eq!(
        proposal.pane_plans[0].surface_binding_ref.as_deref(),
        Some("document:router"),
        "capture binding must survive the joined pane-tree proposal"
    );
    let mut runtime = RestoreRuntime::new(&session_store, &crash_store);
    let outcome = proposal.execute(&mut runtime);

    assert!(outcome.succeeded_without_failures());
    assert_eq!(outcome.restore_class, RestoreClass::ExactRestore);
    assert_eq!(outcome.pane_outcomes.len(), 1);
    assert_eq!(
        outcome.pane_outcomes[0].execution_kind,
        RestorePaneExecutionKind::Reopened
    );
    assert_eq!(
        outcome.pane_outcomes[0].surface_binding_ref.as_deref(),
        Some("document:router"),
        "validated clean-editor binding must survive execution"
    );
    assert_eq!(outcome.dirty_buffer_replays.len(), 1);
    assert_eq!(outcome.dirty_buffer_replays[0].bytes, b"restored bytes\n");
    assert_eq!(
        outcome.dirty_buffer_replays[0].object_ref,
        "document:router"
    );
    assert_eq!(
        outcome.pane_outcomes[0].surface_binding_ref.as_deref(),
        Some(outcome.dirty_buffer_replays[0].object_ref.as_str()),
        "clean editor binding and dirty replay must share an opaque logical identity"
    );
    assert!(!outcome.dirty_buffer_replays[0].object_ref.contains('/'));
    assert!(!outcome.dirty_buffer_replays[0].object_ref.contains("\\"));
    assert!(!outcome.dirty_buffer_replays[0].object_ref.contains("://"));
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
    let mut session_store = SessionRestoreStore::new(dir.path());
    let mut crash_store = CrashJournalStore::new(dir.path());
    capture_layout(&mut session_store, false);
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
    let terminal_plan = proposal
        .pane_plans
        .iter()
        .find(|pane| pane.surface_role == SurfaceRole::Terminal)
        .expect("terminal pane plan");
    assert!(terminal_plan.surface_binding_ref.is_none());
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
    assert!(terminal.surface_binding_ref.is_none());
    let metadata = terminal
        .restore_metadata
        .as_ref()
        .expect("terminal restore metadata survives restore outcome");
    assert_eq!(
        metadata.working_directory.as_deref(),
        Some("/workspace/service")
    );
    assert_eq!(metadata.shell_identity, "zsh");
    assert_eq!(metadata.shell_family_token, "zsh");
    assert_eq!(metadata.environment_scope_token, "workspace");
    assert_eq!(metadata.last_command_class_token, "build");
    assert!(metadata.auto_rerun_forbidden);
    assert!(!metadata.raw_command_body_present);
    assert!(!metadata.raw_environment_body_present);
}
