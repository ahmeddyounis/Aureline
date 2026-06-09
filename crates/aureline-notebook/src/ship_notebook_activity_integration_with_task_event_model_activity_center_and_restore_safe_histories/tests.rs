use super::*;

fn sample_task_event_queued() -> NotebookTaskEvent {
    NotebookTaskEvent {
        record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
        notebook_activity_integration_schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        event_id: "nb.task_event.queued.01".to_owned(),
        notebook_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        cell_execution_id_ref: "nb.exec.01".to_owned(),
        task_event_kind: NotebookTaskEventKind::TaskQueued,
        task_state_class: NotebookTaskStateClass::Queued,
        execution_context_ref: "ctx.notebook.01".to_owned(),
        occurred_at: "2026-06-09T10:00:00Z".to_owned(),
        summary: "Cell execution queued.".to_owned(),
    }
}

fn sample_task_event_completed() -> NotebookTaskEvent {
    NotebookTaskEvent {
        record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
        notebook_activity_integration_schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        event_id: "nb.task_event.completed.01".to_owned(),
        notebook_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        cell_execution_id_ref: "nb.exec.01".to_owned(),
        task_event_kind: NotebookTaskEventKind::TaskCompleted,
        task_state_class: NotebookTaskStateClass::Completed,
        execution_context_ref: "ctx.notebook.01".to_owned(),
        occurred_at: "2026-06-09T10:00:10Z".to_owned(),
        summary: "Cell execution completed successfully.".to_owned(),
    }
}

fn sample_task_event_failed() -> NotebookTaskEvent {
    NotebookTaskEvent {
        record_kind: NOTEBOOK_TASK_EVENT_RECORD_KIND.to_owned(),
        notebook_activity_integration_schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        event_id: "nb.task_event.failed.01".to_owned(),
        notebook_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.02".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        cell_execution_id_ref: "nb.exec.02".to_owned(),
        task_event_kind: NotebookTaskEventKind::TaskFailed,
        task_state_class: NotebookTaskStateClass::Failed,
        execution_context_ref: "ctx.notebook.01".to_owned(),
        occurred_at: "2026-06-09T10:01:00Z".to_owned(),
        summary: "Cell execution failed with error.".to_owned(),
    }
}

fn sample_activity_row_started() -> NotebookActivityCenterRow {
    NotebookActivityCenterRow {
        record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
        notebook_activity_integration_schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        row_id: "nb.activity.started.01".to_owned(),
        notebook_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: Some("nb.cell.01".to_owned()),
        actor_kind: NotebookActivityActorKind::UserActor,
        action: NotebookActivityAction::Started,
        object_kind: NotebookActivityObjectKind::NotebookCellRun,
        outcome: NotebookActivityOutcome::InProgress,
        occurred_at: "2026-06-09T10:00:01Z".to_owned(),
        surface_class: NotebookActivitySurfaceClass::ActivityCenter,
        source_class: NotebookActivitySourceClass::FirstPartyDirectObservation,
        freshness_class: NotebookActivityFreshnessClass::Current,
        follow_up_state: NotebookActivityFollowUpState::Open,
        summary: "User started cell run.".to_owned(),
    }
}

fn sample_activity_row_succeeded() -> NotebookActivityCenterRow {
    NotebookActivityCenterRow {
        record_kind: NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND.to_owned(),
        notebook_activity_integration_schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        row_id: "nb.activity.succeeded.01".to_owned(),
        notebook_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: Some("nb.cell.01".to_owned()),
        actor_kind: NotebookActivityActorKind::KernelActor,
        action: NotebookActivityAction::Succeeded,
        object_kind: NotebookActivityObjectKind::NotebookCellRun,
        outcome: NotebookActivityOutcome::Succeeded,
        occurred_at: "2026-06-09T10:00:10Z".to_owned(),
        surface_class: NotebookActivitySurfaceClass::ActivityCenter,
        source_class: NotebookActivitySourceClass::FirstPartyDirectObservation,
        freshness_class: NotebookActivityFreshnessClass::Current,
        follow_up_state: NotebookActivityFollowUpState::Resolved,
        summary: "Cell run completed successfully.".to_owned(),
    }
}

fn sample_restore_safe_history() -> NotebookRestoreSafeHistory {
    NotebookRestoreSafeHistory {
        record_kind: NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND.to_owned(),
        notebook_activity_integration_schema_version: NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
        history_id: "nb.restore.exact.01".to_owned(),
        notebook_id_ref: "nb.doc.example".to_owned(),
        restore_class: NotebookRestoreClass::ExactRestore,
        restore_posture: NotebookRestorePosture::ReconnectAvailable,
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        cell_execution_id_refs: vec!["nb.exec.01".to_owned(), "nb.exec.02".to_owned()],
        document_restored_at: "2026-06-09T11:00:00Z".to_owned(),
        honest_state_label: "Exact restore; kernel session reconnect available.".to_owned(),
        summary: "Exact notebook restore with reconnectable kernel session.".to_owned(),
    }
}

#[test]
fn task_event_queued_validates_clean() {
    let event = sample_task_event_queued();
    assert!(
        event.validate().is_empty(),
        "queued task event should be clean: {:?}",
        event.validate()
    );
}

#[test]
fn task_event_completed_validates_clean() {
    let event = sample_task_event_completed();
    assert!(
        event.validate().is_empty(),
        "completed task event should be clean: {:?}",
        event.validate()
    );
}

#[test]
fn task_event_failed_validates_clean() {
    let event = sample_task_event_failed();
    assert!(
        event.validate().is_empty(),
        "failed task event should be clean: {:?}",
        event.validate()
    );
}

#[test]
fn task_event_rejects_bad_record_kind() {
    let mut event = sample_task_event_queued();
    event.record_kind = "wrong_kind".to_owned();
    let findings = event.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_task_event.record_kind"));
}

#[test]
fn task_event_rejects_empty_notebook_id() {
    let mut event = sample_task_event_queued();
    event.notebook_id_ref = "".to_owned();
    let findings = event.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_task_event.notebook_id_ref_required"));
}

#[test]
fn task_event_rejects_empty_cell_id() {
    let mut event = sample_task_event_queued();
    event.cell_id_ref = "".to_owned();
    let findings = event.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_task_event.cell_id_ref_required"));
}

#[test]
fn task_event_rejects_terminal_kind_with_non_terminal_state() {
    let mut event = sample_task_event_completed();
    event.task_state_class = NotebookTaskStateClass::Running;
    let findings = event.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_task_event.terminal_kind_mismatched_state"));
}

#[test]
fn activity_center_row_started_validates_clean() {
    let row = sample_activity_row_started();
    assert!(
        row.validate().is_empty(),
        "started activity row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn activity_center_row_succeeded_validates_clean() {
    let row = sample_activity_row_succeeded();
    assert!(
        row.validate().is_empty(),
        "succeeded activity row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn activity_center_row_rejects_bad_record_kind() {
    let mut row = sample_activity_row_started();
    row.record_kind = "wrong_kind".to_owned();
    let findings = row.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_center_row.record_kind"));
}

#[test]
fn activity_center_row_rejects_pending_with_wrong_action() {
    let mut row = sample_activity_row_started();
    row.outcome = NotebookActivityOutcome::Pending;
    row.action = NotebookActivityAction::Succeeded;
    let findings = row.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_center_row.pending_action_invariant"));
}

#[test]
fn activity_center_row_rejects_in_progress_with_wrong_action() {
    let mut row = sample_activity_row_succeeded();
    row.outcome = NotebookActivityOutcome::InProgress;
    row.action = NotebookActivityAction::Succeeded;
    let findings = row.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_center_row.in_progress_action_invariant"));
}

#[test]
fn activity_center_row_rejects_succeeded_with_wrong_action() {
    let mut row = sample_activity_row_started();
    row.outcome = NotebookActivityOutcome::Succeeded;
    row.action = NotebookActivityAction::Started;
    let findings = row.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_center_row.succeeded_action_invariant"));
}

#[test]
fn activity_center_row_rejects_failed_with_wrong_action() {
    let mut row = sample_activity_row_started();
    row.outcome = NotebookActivityOutcome::Failed;
    row.action = NotebookActivityAction::Started;
    let findings = row.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_center_row.failed_action_invariant"));
}

#[test]
fn activity_center_row_rejects_cancelled_with_wrong_action() {
    let mut row = sample_activity_row_started();
    row.outcome = NotebookActivityOutcome::Cancelled;
    row.action = NotebookActivityAction::Started;
    let findings = row.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_center_row.cancelled_action_invariant"));
}

#[test]
fn restore_safe_history_validates_clean() {
    let history = sample_restore_safe_history();
    assert!(
        history.validate().is_empty(),
        "restore safe history should be clean: {:?}",
        history.validate()
    );
}

#[test]
fn restore_safe_history_rejects_bad_record_kind() {
    let mut history = sample_restore_safe_history();
    history.record_kind = "wrong_kind".to_owned();
    let findings = history.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_restore_safe_history.record_kind"));
}

#[test]
fn restore_safe_history_rejects_empty_notebook_id() {
    let mut history = sample_restore_safe_history();
    history.notebook_id_ref = "".to_owned();
    let findings = history.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_restore_safe_history.notebook_id_ref_required"));
}

#[test]
fn restore_safe_history_rejects_empty_honest_state_label() {
    let mut history = sample_restore_safe_history();
    history.honest_state_label = "".to_owned();
    let findings = history.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_restore_safe_history.honest_state_label_required"));
}

#[test]
fn restore_safe_history_rejects_transcript_with_kernel_session() {
    let mut history = sample_restore_safe_history();
    history.restore_posture = NotebookRestorePosture::TranscriptRestored;
    history.kernel_session_id_ref = Some("nb.kernel.session.01".to_owned());
    let findings = history.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_restore_safe_history.transcript_no_session"));
}

#[test]
fn restore_safe_history_rejects_session_ended_with_kernel_session() {
    let mut history = sample_restore_safe_history();
    history.restore_posture = NotebookRestorePosture::SessionEnded;
    history.kernel_session_id_ref = Some("nb.kernel.session.01".to_owned());
    let findings = history.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_restore_safe_history.ended_no_session"));
}

#[test]
fn restore_safe_history_rejects_empty_cell_execution_refs() {
    let mut history = sample_restore_safe_history();
    history.cell_execution_id_refs = vec![];
    let findings = history.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_restore_safe_history.cell_execution_id_refs_required"));
}

#[test]
fn packet_validates_clean() {
    let packet = current_notebook_activity_integration_packet();
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn packet_rejects_bad_record_kind() {
    let mut packet = current_notebook_activity_integration_packet();
    packet.record_kind = "wrong_kind".to_owned();
    let findings = packet.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_integration_packet.record_kind"));
}

#[test]
fn packet_rejects_wrong_schema_version() {
    let mut packet = current_notebook_activity_integration_packet();
    packet.schema_version = 999;
    let findings = packet.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_activity_integration_packet.schema_version"));
}

#[test]
fn all_closed_vocab_tokens_are_stable() {
    assert_eq!(NotebookTaskEventKind::ALL.len(), 6);
    assert_eq!(NotebookTaskStateClass::ALL.len(), 5);
    assert_eq!(NotebookActivityActorKind::ALL.len(), 3);
    assert_eq!(NotebookActivityAction::ALL.len(), 6);
    assert_eq!(NotebookActivityObjectKind::ALL.len(), 3);
    assert_eq!(NotebookActivityOutcome::ALL.len(), 6);
    assert_eq!(NotebookActivitySurfaceClass::ALL.len(), 1);
    assert_eq!(NotebookActivitySourceClass::ALL.len(), 3);
    assert_eq!(NotebookActivityFreshnessClass::ALL.len(), 6);
    assert_eq!(NotebookActivityFollowUpState::ALL.len(), 7);
    assert_eq!(NotebookRestoreClass::ALL.len(), 5);
    assert_eq!(NotebookRestorePosture::ALL.len(), 5);
}
