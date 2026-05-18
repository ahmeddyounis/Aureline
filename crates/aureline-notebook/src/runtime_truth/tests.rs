use super::*;

fn local_kernel_summary() -> KernelSessionSummary {
    KernelSessionSummary {
        record_kind: KERNEL_SESSION_SUMMARY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        summary_id: "nb.kernel_bar.local.01".to_owned(),
        kernel_session_id_ref: Some("kernel.session.local.01".to_owned()),
        kernelspec_ref: Some("kernelspec.python.312.local".to_owned()),
        header: NotebookHeaderBlock {
            document_id_ref: "nb.doc.local.01".to_owned(),
            document_path_token_ref: "vfs.path.token.local.01".to_owned(),
            document_title_label: "Local notebook".to_owned(),
            document_trust_class: NotebookDocumentTrustClass::InheritedFromWorkspace,
            dirty_state_class: NotebookDirtyStateClass::Clean,
            paired_export_posture: NotebookPairedExportPosture::NotApplicable,
            paired_export_ref: None,
        },
        kernel_selection_state: KernelSelectionState::SelectedKernelResolved,
        kernel_origin_class: KernelOriginClass::LocalManagedToolchainKernel,
        local_vs_remote_boundary_cue_visible: false,
        target_identity_witness_ref: None,
        remote_agent_session_id_ref: None,
        execution_context_root_ref: Some("exec.ctx.local.01".to_owned()),
        last_successful_run: Some(NotebookLastSuccessfulRunSummary {
            run_id_ref: "run.local.notebook.01".to_owned(),
            attempt_id_ref: "attempt.local.notebook.01".to_owned(),
            completed_at: "2026-05-18T10:00:00Z".to_owned(),
            cells_completed: 4,
            summary_label: "Last run completed 4 cells on local kernel.".to_owned(),
        }),
        available_actions: vec![
            KernelBarActionClass::Restart,
            KernelBarActionClass::Interrupt,
            KernelBarActionClass::ChangeKernel,
        ],
        auto_rerun_forbidden: true,
        captured_at: "2026-05-18T10:05:00Z".to_owned(),
        summary: "Local managed-toolchain kernel selected and resolved.".to_owned(),
    }
}

fn no_kernel_summary() -> KernelSessionSummary {
    KernelSessionSummary {
        record_kind: KERNEL_SESSION_SUMMARY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        summary_id: "nb.kernel_bar.no_kernel.01".to_owned(),
        kernel_session_id_ref: None,
        kernelspec_ref: None,
        header: NotebookHeaderBlock {
            document_id_ref: "nb.doc.no_kernel.01".to_owned(),
            document_path_token_ref: "vfs.path.token.no_kernel.01".to_owned(),
            document_title_label: "No-kernel notebook".to_owned(),
            document_trust_class: NotebookDocumentTrustClass::InheritedFromWorkspace,
            dirty_state_class: NotebookDirtyStateClass::Clean,
            paired_export_posture: NotebookPairedExportPosture::NotApplicable,
            paired_export_ref: None,
        },
        kernel_selection_state: KernelSelectionState::NoKernelSelected,
        kernel_origin_class: KernelOriginClass::NoKernel,
        local_vs_remote_boundary_cue_visible: false,
        target_identity_witness_ref: None,
        remote_agent_session_id_ref: None,
        execution_context_root_ref: None,
        last_successful_run: None,
        available_actions: vec![KernelBarActionClass::SelectKernel],
        auto_rerun_forbidden: true,
        captured_at: "2026-05-18T10:05:00Z".to_owned(),
        summary: "No kernel selected; document is editable and reviewable.".to_owned(),
    }
}

fn remote_kernel_summary() -> KernelSessionSummary {
    KernelSessionSummary {
        record_kind: KERNEL_SESSION_SUMMARY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        summary_id: "nb.kernel_bar.remote.01".to_owned(),
        kernel_session_id_ref: Some("kernel.session.remote.01".to_owned()),
        kernelspec_ref: Some("kernelspec.python.312.remote".to_owned()),
        header: NotebookHeaderBlock {
            document_id_ref: "nb.doc.remote.01".to_owned(),
            document_path_token_ref: "vfs.path.token.remote.01".to_owned(),
            document_title_label: "Remote notebook".to_owned(),
            document_trust_class: NotebookDocumentTrustClass::InheritedFromWorkspace,
            dirty_state_class: NotebookDirtyStateClass::Clean,
            paired_export_posture: NotebookPairedExportPosture::DerivedNotebookToScript,
            paired_export_ref: Some("paired.export.script.remote.01".to_owned()),
        },
        kernel_selection_state: KernelSelectionState::SelectedKernelResolved,
        kernel_origin_class: KernelOriginClass::RemoteAgentPrimaryKernel,
        local_vs_remote_boundary_cue_visible: true,
        target_identity_witness_ref: Some("witness.remote.01".to_owned()),
        remote_agent_session_id_ref: Some("remote.agent.session.01".to_owned()),
        execution_context_root_ref: Some("exec.ctx.remote.01".to_owned()),
        last_successful_run: None,
        available_actions: vec![
            KernelBarActionClass::Restart,
            KernelBarActionClass::Interrupt,
            KernelBarActionClass::Reconnect,
        ],
        auto_rerun_forbidden: true,
        captured_at: "2026-05-18T10:05:00Z".to_owned(),
        summary: "Remote agent primary kernel; local-vs-remote boundary cue visible.".to_owned(),
    }
}

#[test]
fn local_kernel_summary_validates_clean() {
    let summary = local_kernel_summary();
    assert!(
        summary.validate().is_empty(),
        "local kernel summary should be clean: {:?}",
        summary.validate()
    );
}

#[test]
fn no_kernel_summary_validates_clean() {
    let summary = no_kernel_summary();
    assert!(
        summary.validate().is_empty(),
        "no-kernel summary should be clean: {:?}",
        summary.validate()
    );
}

#[test]
fn remote_kernel_summary_validates_clean() {
    let summary = remote_kernel_summary();
    assert!(
        summary.validate().is_empty(),
        "remote kernel summary should be clean: {:?}",
        summary.validate()
    );
}

#[test]
fn no_kernel_must_not_expose_restart() {
    let mut summary = no_kernel_summary();
    summary
        .available_actions
        .push(KernelBarActionClass::Restart);
    let findings = summary.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernel_session_summary.no_kernel_actions_running"));
}

#[test]
fn remote_kernel_requires_boundary_cue_and_witnesses() {
    let mut summary = remote_kernel_summary();
    summary.local_vs_remote_boundary_cue_visible = false;
    summary.target_identity_witness_ref = None;
    summary.remote_agent_session_id_ref = None;
    let findings = summary.validate();
    let codes: Vec<&str> = findings.iter().map(|f| f.check_id.as_str()).collect();
    assert!(codes.contains(&"kernel_session_summary.remote_boundary_cue"));
    assert!(codes.contains(&"kernel_session_summary.target_identity_witness_required"));
    assert!(codes.contains(&"kernel_session_summary.remote_agent_session_id_required"));
}

#[test]
fn auto_rerun_forbidden_must_be_true() {
    let mut summary = local_kernel_summary();
    summary.auto_rerun_forbidden = false;
    let findings = summary.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernel_session_summary.auto_rerun_forbidden"));
}

fn cell_row_succeeded() -> CellExecutionDetailRow {
    CellExecutionDetailRow {
        record_kind: CELL_EXECUTION_DETAIL_ROW_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        row_id: "nb.cell_row.succeeded.01".to_owned(),
        kernel_session_id_ref: Some("kernel.session.local.01".to_owned()),
        document_id_ref: "nb.doc.local.01".to_owned(),
        cell_id_ref: "nb.cell.eval".to_owned(),
        cell_display_index: 2,
        cell_execution_id_ref: "nb.exec.eval.18".to_owned(),
        run_scope: CellExecutionRunScope::CurrentSession,
        started_at: Some("2026-05-18T10:01:00Z".to_owned()),
        finished_at: Some("2026-05-18T10:01:02Z".to_owned()),
        duration_millis: Some(2_000),
        outcome_class: CellExecutionOutcomeClass::Succeeded,
        output_count: 1,
        task_event_envelope_ref: Some("task.event.envelope.eval.18".to_owned()),
        log_slice_ref: Some("log.slice.eval.18".to_owned()),
        diagnostic_ref: None,
        summary: "Cell evaluated dataframe in 2 seconds.".to_owned(),
    }
}

#[test]
fn succeeded_cell_row_validates_clean() {
    let row = cell_row_succeeded();
    assert!(
        row.validate().is_empty(),
        "succeeded cell row should be clean: {:?}",
        row.validate()
    );
}

#[test]
fn errored_cell_row_requires_diagnostic_ref() {
    let mut row = cell_row_succeeded();
    row.outcome_class = CellExecutionOutcomeClass::Errored;
    row.diagnostic_ref = None;
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "cell_execution_detail_row.errored_diagnostic_required"));
}

#[test]
fn queued_cell_row_must_not_carry_timestamps() {
    let row = CellExecutionDetailRow {
        record_kind: CELL_EXECUTION_DETAIL_ROW_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        row_id: "nb.cell_row.queued.01".to_owned(),
        kernel_session_id_ref: Some("kernel.session.local.01".to_owned()),
        document_id_ref: "nb.doc.local.01".to_owned(),
        cell_id_ref: "nb.cell.train".to_owned(),
        cell_display_index: 4,
        cell_execution_id_ref: "nb.exec.train.queued.01".to_owned(),
        run_scope: CellExecutionRunScope::QueuedNotYetStarted,
        started_at: Some("2026-05-18T10:02:00Z".to_owned()),
        finished_at: None,
        duration_millis: None,
        outcome_class: CellExecutionOutcomeClass::Queued,
        output_count: 0,
        task_event_envelope_ref: None,
        log_slice_ref: None,
        diagnostic_ref: None,
        summary: "Queued.".to_owned(),
    };
    let findings = row.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "cell_execution_detail_row.queued_timestamps"));
}

fn live_variable_entry() -> VariableExplorerEntry {
    VariableExplorerEntry {
        record_kind: VARIABLE_EXPLORER_ENTRY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        entry_id: "nb.var.live.df".to_owned(),
        document_id_ref: "nb.doc.local.01".to_owned(),
        kernel_session_id_ref: Some("kernel.session.local.01".to_owned()),
        variable_handle_ref: "var.handle.df".to_owned(),
        display_name_label: "df".to_owned(),
        type_descriptor_ref: "type.pandas.dataframe".to_owned(),
        freshness_class: VariableExplorerFreshnessClass::LiveFromCurrentSession,
        truncation_class: VariableExplorerTruncationClass::TruncatedForDisplay,
        available_actions: vec![
            VariableExplorerEntryActionClass::OpenLiveViewer,
            VariableExplorerEntryActionClass::ExportWithRedaction,
        ],
        summary: "Live dataframe; display preview truncated.".to_owned(),
    }
}

#[test]
fn live_variable_entry_validates_clean() {
    let entry = live_variable_entry();
    assert!(
        entry.validate().is_empty(),
        "live variable entry should be clean: {:?}",
        entry.validate()
    );
}

#[test]
fn stale_variable_entry_rejects_open_live_viewer() {
    let mut entry = live_variable_entry();
    entry.freshness_class = VariableExplorerFreshnessClass::StaleAfterRestart;
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_entry.snapshot_open_live_viewer"));
}

#[test]
fn no_kernel_variable_entry_must_drop_session_ref() {
    let mut entry = live_variable_entry();
    entry.freshness_class = VariableExplorerFreshnessClass::NoLiveVariablesNoKernel;
    entry.available_actions = vec![VariableExplorerEntryActionClass::DismissFromExplorer];
    let findings = entry.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_entry.no_live_no_kernel_session_ref"));
}

fn sanitized_output() -> OutputTrustRecord {
    OutputTrustRecord {
        record_kind: OUTPUT_TRUST_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        record_id: "nb.output_trust.sanitized.01".to_owned(),
        document_id_ref: "nb.doc.local.01".to_owned(),
        cell_id_ref: "nb.cell.eval".to_owned(),
        output_block_ref: "nb.output_block.html.01".to_owned(),
        mime_bundle_descriptor_ref: "mime.bundle.html.sanitized".to_owned(),
        trust_class: OutputTrustClass::Sanitized,
        hidden_escalation_posture: OutputTrustHiddenEscalationPosture::NoHiddenEscalationAllowed,
        stale_reason_class: None,
        fallback_actions: vec![
            OutputTrustFallbackActionClass::OpenCompatibleViewer,
            OutputTrustFallbackActionClass::OpenRawFallback,
            OutputTrustFallbackActionClass::ExportWithRedaction,
            OutputTrustFallbackActionClass::ReviewBeforeTrust,
        ],
        compatible_viewer_available: true,
        raw_fallback_available: true,
        summary: "Sanitized HTML output; explicit review required to escalate to trusted_active."
            .to_owned(),
    }
}

#[test]
fn sanitized_output_validates_clean() {
    let record = sanitized_output();
    assert!(
        record.validate().is_empty(),
        "sanitized output should be clean: {:?}",
        record.validate()
    );
}

#[test]
fn stale_output_requires_reason_class() {
    let mut record = sanitized_output();
    record.trust_class = OutputTrustClass::Stale;
    record.compatible_viewer_available = false;
    record.fallback_actions = vec![
        OutputTrustFallbackActionClass::OpenRawFallback,
        OutputTrustFallbackActionClass::ExportWithRedaction,
    ];
    record.stale_reason_class = None;
    let findings = record.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_trust_record.stale_reason_required"));
}

#[test]
fn sanitized_without_review_before_trust_is_rejected() {
    let mut record = sanitized_output();
    record
        .fallback_actions
        .retain(|action| !matches!(action, OutputTrustFallbackActionClass::ReviewBeforeTrust));
    let findings = record.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_trust_record.review_before_trust_required"));
}

#[test]
fn trusted_active_requires_compatible_viewer() {
    let mut record = sanitized_output();
    record.trust_class = OutputTrustClass::TrustedActive;
    record.compatible_viewer_available = false;
    let findings = record.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_trust_record.trusted_active_requires_viewer"));
}

fn unsupported_no_kernel_debugger() -> DebuggerBridgeState {
    DebuggerBridgeState {
        record_kind: DEBUGGER_BRIDGE_STATE_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        state_id: "nb.debug_bridge.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.no_kernel.01".to_owned(),
        kernel_session_id_ref: None,
        support_class: DebuggerBridgeSupportClass::UnsupportedNoKernel,
        unsupported_reason_class: DebuggerBridgeUnsupportedReasonClass::NoKernelSession,
        adapter_class: DebuggerBridgeAdapterClass::NoAdapter,
        kernel_class: DebuggerBridgeKernelClass::NoKernel,
        current_cell_id_ref: None,
        current_frame_ref: None,
        frame_relation_class: DebuggerBridgeFrameRelationClass::NoActiveFrame,
        breakpoint_posture_class:
            DebuggerBridgeBreakpointPostureClass::BreakpointsNotSupportedByKernel,
        reconnect_review_required: false,
        reconnect_review_sheet_ref: None,
        summary: "No kernel selected; debugger bridge is unavailable.".to_owned(),
    }
}

#[test]
fn unsupported_no_kernel_debugger_validates_clean() {
    let state = unsupported_no_kernel_debugger();
    assert!(
        state.validate().is_empty(),
        "no-kernel debugger should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn supported_debugger_requires_not_applicable_reason() {
    let mut state = unsupported_no_kernel_debugger();
    state.support_class = DebuggerBridgeSupportClass::Supported;
    state.kernel_session_id_ref = Some("kernel.session.local.01".to_owned());
    state.kernel_class = DebuggerBridgeKernelClass::LocalManagedToolchainKernel;
    state.adapter_class = DebuggerBridgeAdapterClass::KernelEmbeddedDebugProtocol;
    state.breakpoint_posture_class = DebuggerBridgeBreakpointPostureClass::BreakpointsHonoured;
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "debugger_bridge_state.supported_reason_class"));
}

fn user_restart_sheet() -> ReconnectReviewSheet {
    ReconnectReviewSheet {
        record_kind: RECONNECT_REVIEW_SHEET_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        sheet_id: "nb.reconnect.restart.01".to_owned(),
        document_id_ref: "nb.doc.local.01".to_owned(),
        prior_kernel_session_id_ref: Some("kernel.session.local.01".to_owned()),
        next_kernel_session_id_ref: Some("kernel.session.local.02".to_owned()),
        kind: ReconnectReviewKind::UserInitiatedRestart,
        consequence_class: ReconnectReviewConsequenceClass::ReopeningLiveKernelFreshSession,
        in_flight_executions_cancelled: true,
        queued_cells_affected: 2,
        live_variable_state_lost: true,
        user_confirmation_required: true,
        auto_rerun_forbidden: true,
        summary: "Restart will reopen a fresh kernel session; 2 queued cells will be cancelled."
            .to_owned(),
    }
}

#[test]
fn user_restart_sheet_validates_clean() {
    let sheet = user_restart_sheet();
    assert!(
        sheet.validate().is_empty(),
        "restart sheet should be clean: {:?}",
        sheet.validate()
    );
}

#[test]
fn restart_sheet_requires_inflight_cancellation_and_variable_loss() {
    let mut sheet = user_restart_sheet();
    sheet.in_flight_executions_cancelled = false;
    sheet.live_variable_state_lost = false;
    let findings = sheet.validate();
    let codes: Vec<&str> = findings.iter().map(|f| f.check_id.as_str()).collect();
    assert!(codes.contains(&"reconnect_review_sheet.consequence_cancels_inflight"));
    assert!(codes.contains(&"reconnect_review_sheet.consequence_loses_variables"));
}

#[test]
fn restart_sheet_requires_user_confirmation() {
    let mut sheet = user_restart_sheet();
    sheet.user_confirmation_required = false;
    let findings = sheet.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "reconnect_review_sheet.user_confirmation_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(KernelOriginClass::NoKernel.as_str(), "no_kernel");
    assert_eq!(OutputTrustClass::TrustedActive.as_str(), "trusted_active");
    assert!(OutputTrustClass::TrustedActive.admits_active_behavior());
    assert!(!OutputTrustClass::TrustedActive.requires_explicit_review_to_escalate());
    assert!(OutputTrustClass::Sanitized.requires_explicit_review_to_escalate());
    assert!(KernelOriginClass::RemoteAgentPrimaryKernel.is_remote_boundary());
    assert!(!KernelOriginClass::LocalManagedToolchainKernel.is_remote_boundary());
}
