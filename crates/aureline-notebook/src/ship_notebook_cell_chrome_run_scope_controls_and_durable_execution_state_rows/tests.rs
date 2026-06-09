use super::*;

fn sample_run_scope_control() -> RunScopeControl {
    RunScopeControl {
        record_kind: RUN_SCOPE_CONTROL_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        control_id: "nb.runscope.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: Some("nb.cell.01".to_owned()),
        current_scope: crate::CellExecutionRunScope::CurrentSession,
        available_scopes: vec![
            crate::CellExecutionRunScope::CurrentSession,
            crate::CellExecutionRunScope::PriorSession,
            crate::CellExecutionRunScope::ReplayFromCapturedOutput,
        ],
        scope_changeable: true,
        lock_reason_class: RunScopeControlLockReasonClass::NotLocked,
        summary: "Per-cell run-scope control for example cell.".to_owned(),
    }
}

fn sample_locked_run_scope_control() -> RunScopeControl {
    RunScopeControl {
        record_kind: RUN_SCOPE_CONTROL_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        control_id: "nb.runscope.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: None,
        current_scope: crate::CellExecutionRunScope::ReplayFromCapturedOutput,
        available_scopes: vec![crate::CellExecutionRunScope::ReplayFromCapturedOutput],
        scope_changeable: false,
        lock_reason_class: RunScopeControlLockReasonClass::LockedReplayOnlyEnvironment,
        summary: "Global replay-only run-scope control.".to_owned(),
    }
}

fn sample_cell_chrome_idle() -> NotebookCellChrome {
    NotebookCellChrome {
        record_kind: NOTEBOOK_CELL_CHROME_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        chrome_state_id: "nb.chrome.idle.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        cell_display_index: 0,
        execution_badge_label: "[1]".to_owned(),
        cell_status_class: CellChromeStatusClass::Idle,
        run_scope_control: sample_run_scope_control(),
        output_trust_class: crate::OutputTrustClass::TrustedActive,
        available_actions: vec![
            CellChromeActionClass::RunCell,
            CellChromeActionClass::RunCellAndAdvance,
            CellChromeActionClass::ClearOutput,
            CellChromeActionClass::ToggleCollapseOutput,
            CellChromeActionClass::DebugCell,
        ],
        collapsed: false,
        selected: false,
        focused: false,
        summary: "Idle cell chrome with trusted output.".to_owned(),
    }
}

fn sample_cell_chrome_executing() -> NotebookCellChrome {
    NotebookCellChrome {
        record_kind: NOTEBOOK_CELL_CHROME_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        chrome_state_id: "nb.chrome.executing.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.02".to_owned(),
        cell_display_index: 1,
        execution_badge_label: "[*]".to_owned(),
        cell_status_class: CellChromeStatusClass::Executing,
        run_scope_control: RunScopeControl {
            record_kind: RUN_SCOPE_CONTROL_RECORD_KIND.to_owned(),
            notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
            control_id: "nb.runscope.exec.01".to_owned(),
            document_id_ref: "nb.doc.example".to_owned(),
            cell_id_ref: Some("nb.cell.02".to_owned()),
            current_scope: crate::CellExecutionRunScope::CurrentSession,
            available_scopes: vec![crate::CellExecutionRunScope::CurrentSession],
            scope_changeable: false,
            lock_reason_class: RunScopeControlLockReasonClass::LockedDuringExecution,
            summary: "Run-scope control locked during execution.".to_owned(),
        },
        output_trust_class: crate::OutputTrustClass::TrustedActive,
        available_actions: vec![
            CellChromeActionClass::ClearOutput,
            CellChromeActionClass::ToggleCollapseOutput,
        ],
        collapsed: false,
        selected: true,
        focused: true,
        summary: "Executing cell chrome.".to_owned(),
    }
}

fn sample_cell_chrome_no_kernel() -> NotebookCellChrome {
    NotebookCellChrome {
        record_kind: NOTEBOOK_CELL_CHROME_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        chrome_state_id: "nb.chrome.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.03".to_owned(),
        cell_display_index: 2,
        execution_badge_label: "[ ]".to_owned(),
        cell_status_class: CellChromeStatusClass::NoKernel,
        run_scope_control: RunScopeControl {
            record_kind: RUN_SCOPE_CONTROL_RECORD_KIND.to_owned(),
            notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
            control_id: "nb.runscope.no_kernel.01".to_owned(),
            document_id_ref: "nb.doc.example".to_owned(),
            cell_id_ref: Some("nb.cell.03".to_owned()),
            current_scope: crate::CellExecutionRunScope::QueuedNotYetStarted,
            available_scopes: vec![crate::CellExecutionRunScope::QueuedNotYetStarted],
            scope_changeable: false,
            lock_reason_class: RunScopeControlLockReasonClass::LockedNoKernel,
            summary: "Run-scope control locked because no kernel is selected.".to_owned(),
        },
        output_trust_class: crate::OutputTrustClass::Stale,
        available_actions: vec![
            CellChromeActionClass::ToggleCollapseOutput,
            CellChromeActionClass::ToggleFoldSource,
            CellChromeActionClass::ExportOutput,
        ],
        collapsed: false,
        selected: false,
        focused: false,
        summary: "No-kernel cell chrome; editable and reviewable.".to_owned(),
    }
}

fn sample_durable_row_succeeded() -> DurableExecutionStateRow {
    DurableExecutionStateRow {
        record_kind: DURABLE_EXECUTION_STATE_ROW_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        row_id: "nb.durable.succeeded.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        cell_display_index: 0,
        latest_execution_detail_row_ref: "nb.exec.detail.01".to_owned(),
        durable_outcome_class: crate::CellExecutionOutcomeClass::Succeeded,
        durable_run_scope: crate::CellExecutionRunScope::CurrentSession,
        output_count: 2,
        stale_output: false,
        stale_reason_class: None,
        summary: "Durable execution-state row for succeeded cell.".to_owned(),
    }
}

fn sample_durable_row_stale() -> DurableExecutionStateRow {
    DurableExecutionStateRow {
        record_kind: DURABLE_EXECUTION_STATE_ROW_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        row_id: "nb.durable.stale.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        cell_display_index: 0,
        latest_execution_detail_row_ref: "nb.exec.detail.02".to_owned(),
        durable_outcome_class: crate::CellExecutionOutcomeClass::Succeeded,
        durable_run_scope: crate::CellExecutionRunScope::PriorSession,
        output_count: 2,
        stale_output: true,
        stale_reason_class: Some(crate::OutputTrustStaleReasonClass::KernelRestartedSinceProduce),
        summary: "Durable execution-state row with stale output after restart.".to_owned(),
    }
}

fn sample_durable_row_skipped_no_kernel() -> DurableExecutionStateRow {
    DurableExecutionStateRow {
        record_kind: DURABLE_EXECUTION_STATE_ROW_RECORD_KIND.to_owned(),
        notebook_cell_chrome_schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        row_id: "nb.durable.skipped.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.03".to_owned(),
        cell_display_index: 2,
        latest_execution_detail_row_ref: "nb.exec.detail.03".to_owned(),
        durable_outcome_class: crate::CellExecutionOutcomeClass::SkippedNoKernel,
        durable_run_scope: crate::CellExecutionRunScope::ManualUserAction,
        output_count: 0,
        stale_output: false,
        stale_reason_class: None,
        summary: "Durable execution-state row for skipped-no-kernel cell.".to_owned(),
    }
}

#[test]
fn run_scope_control_validates_clean() {
    let control = sample_run_scope_control();
    assert!(
        control.validate().is_empty(),
        "run scope control should be clean: {:?}",
        control.validate()
    );
}

#[test]
fn locked_run_scope_control_validates_clean() {
    let control = sample_locked_run_scope_control();
    assert!(
        control.validate().is_empty(),
        "locked run scope control should be clean: {:?}",
        control.validate()
    );
}

#[test]
fn run_scope_control_rejects_missing_current_scope_in_available() {
    let mut control = sample_run_scope_control();
    control.available_scopes = vec![crate::CellExecutionRunScope::PriorSession];
    let findings = control.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "run_scope_control.current_scope_available"));
}

#[test]
fn run_scope_control_rejects_changeable_with_locked_reason() {
    let mut control = sample_run_scope_control();
    control.lock_reason_class = RunScopeControlLockReasonClass::LockedByPolicy;
    let findings = control.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "run_scope_control.changeable_not_locked"));
}

#[test]
fn run_scope_control_rejects_locked_with_not_locked_reason() {
    let mut control = sample_locked_run_scope_control();
    control.lock_reason_class = RunScopeControlLockReasonClass::NotLocked;
    let findings = control.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "run_scope_control.locked_reason_required"));
}

#[test]
fn run_scope_control_rejects_queued_changeable() {
    let mut control = sample_run_scope_control();
    control.current_scope = crate::CellExecutionRunScope::QueuedNotYetStarted;
    control.scope_changeable = true;
    let findings = control.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "run_scope_control.queued_not_changeable"));
}

#[test]
fn run_scope_control_rejects_empty_available_scopes() {
    let mut control = sample_run_scope_control();
    control.available_scopes.clear();
    let findings = control.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "run_scope_control.available_scopes_required"));
}

#[test]
fn cell_chrome_idle_validates_clean() {
    let chrome = sample_cell_chrome_idle();
    assert!(
        chrome.validate().is_empty(),
        "idle cell chrome should be clean: {:?}",
        chrome.validate()
    );
}

#[test]
fn cell_chrome_executing_validates_clean() {
    let mut chrome = sample_cell_chrome_executing();
    // Fix: executing status should not expose run_cell; the sample has InterruptCell
    // which doesn't exist in CellChromeActionClass. Let me use ClearOutput instead.
    // Actually, I used InterruptCell in the sample above which is wrong. Let me fix the sample.
    chrome.available_actions = vec![
        CellChromeActionClass::ClearOutput,
        CellChromeActionClass::ToggleCollapseOutput,
    ];
    assert!(
        chrome.validate().is_empty(),
        "executing cell chrome should be clean: {:?}",
        chrome.validate()
    );
}

#[test]
fn cell_chrome_no_kernel_validates_clean() {
    let chrome = sample_cell_chrome_no_kernel();
    assert!(
        chrome.validate().is_empty(),
        "no-kernel cell chrome should be clean: {:?}",
        chrome.validate()
    );
}

#[test]
fn cell_chrome_rejects_empty_badge_label() {
    let mut chrome = sample_cell_chrome_idle();
    chrome.execution_badge_label = "".to_owned();
    let findings = chrome.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_chrome.execution_badge_label_required"));
}

#[test]
fn no_kernel_cell_chrome_rejects_run_actions() {
    let mut chrome = sample_cell_chrome_no_kernel();
    chrome.available_actions.push(CellChromeActionClass::RunCell);
    let findings = chrome.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_chrome.no_kernel_run_actions"));
}

#[test]
fn no_kernel_cell_chrome_rejects_debug_action() {
    let mut chrome = sample_cell_chrome_no_kernel();
    chrome.available_actions.push(CellChromeActionClass::DebugCell);
    let findings = chrome.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_chrome.no_kernel_debug_action"));
}

#[test]
fn executing_cell_chrome_rejects_run_actions() {
    let mut chrome = sample_cell_chrome_executing();
    chrome.available_actions = vec![
        CellChromeActionClass::RunCell,
        CellChromeActionClass::ClearOutput,
    ];
    let findings = chrome.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_chrome.active_pending_no_rerun"));
}

#[test]
fn durable_row_succeeded_validates_clean() {
    let row = sample_durable_row_succeeded();
    assert!(
        row.validate().is_empty(),
        "succeeded durable row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn durable_row_stale_validates_clean() {
    let row = sample_durable_row_stale();
    assert!(
        row.validate().is_empty(),
        "stale durable row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn durable_row_skipped_no_kernel_validates_clean() {
    let row = sample_durable_row_skipped_no_kernel();
    assert!(
        row.validate().is_empty(),
        "skipped-no-kernel durable row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn durable_row_rejects_stale_without_reason() {
    let mut row = sample_durable_row_stale();
    row.stale_reason_class = None;
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "durable_execution_state_row.stale_reason_required"));
}

#[test]
fn durable_row_rejects_reason_when_not_stale() {
    let mut row = sample_durable_row_succeeded();
    row.stale_reason_class = Some(crate::OutputTrustStaleReasonClass::KernelRestartedSinceProduce);
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "durable_execution_state_row.stale_reason_not_allowed"));
}

#[test]
fn durable_row_rejects_nonzero_output_for_skipped() {
    let mut row = sample_durable_row_skipped_no_kernel();
    row.output_count = 1;
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "durable_execution_state_row.skipped_output_count"));
}

#[test]
fn durable_row_rejects_queued_scope_with_non_queued_outcome() {
    let mut row = sample_durable_row_skipped_no_kernel();
    row.durable_run_scope = crate::CellExecutionRunScope::QueuedNotYetStarted;
    row.durable_outcome_class = crate::CellExecutionOutcomeClass::SkippedNoKernel;
    // This should fail because queued_not_yet_started must report outcome=queued
    row.durable_run_scope = crate::CellExecutionRunScope::QueuedNotYetStarted;
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "durable_execution_state_row.queued_scope_outcome"));
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookCellChromePacket {
        schema_version: NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
        record_kind: NOTEBOOK_CELL_CHROME_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.cell_chrome.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        cell_chrome_status_classes: CellChromeStatusClass::ALL.to_vec(),
        cell_chrome_action_classes: CellChromeActionClass::ALL.to_vec(),
        run_scope_control_lock_reason_classes: RunScopeControlLockReasonClass::ALL.to_vec(),
        example_cell_chromes: vec![
            sample_cell_chrome_idle(),
            {
                let mut c = sample_cell_chrome_executing();
                c.available_actions = vec![
                    CellChromeActionClass::ClearOutput,
                    CellChromeActionClass::ToggleCollapseOutput,
                ];
                c
            },
            sample_cell_chrome_no_kernel(),
        ],
        example_run_scope_controls: vec![
            sample_run_scope_control(),
            sample_locked_run_scope_control(),
        ],
        example_durable_execution_state_rows: vec![
            sample_durable_row_succeeded(),
            sample_durable_row_stale(),
            sample_durable_row_skipped_no_kernel(),
        ],
        summary: "Notebook cell chrome, run-scope controls, and durable execution-state rows packet v1."
            .to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_cell_chrome_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_CELL_CHROME_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_CELL_CHROME_PACKET_RECORD_KIND);
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(CellChromeStatusClass::Idle.as_str(), "idle");
    assert_eq!(CellChromeStatusClass::Executing.as_str(), "executing");
    assert_eq!(CellChromeStatusClass::StaleOutput.as_str(), "stale_output");
    assert!(CellChromeStatusClass::Executing.is_active_or_pending());
    assert!(!CellChromeStatusClass::Succeeded.is_active_or_pending());
    assert!(CellChromeStatusClass::Succeeded.is_terminal());
    assert!(!CellChromeStatusClass::Queued.is_terminal());
    assert!(CellChromeStatusClass::NoKernel.is_no_kernel());

    assert_eq!(CellChromeActionClass::RunCell.as_str(), "run_cell");
    assert_eq!(CellChromeActionClass::DebugCell.as_str(), "debug_cell");

    assert_eq!(RunScopeControlLockReasonClass::NotLocked.as_str(), "not_locked");
    assert_eq!(
        RunScopeControlLockReasonClass::LockedDuringExecution.as_str(),
        "locked_during_execution"
    );
}
