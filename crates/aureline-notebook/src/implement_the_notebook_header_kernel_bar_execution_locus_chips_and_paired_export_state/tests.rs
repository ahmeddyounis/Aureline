use super::*;

fn sample_local_chip() -> ExecutionLocusChip {
    ExecutionLocusChip {
        record_kind: EXECUTION_LOCUS_CHIP_RECORD_KIND.to_owned(),
        notebook_header_kernel_bar_schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        chip_id: "nb.chip.local.01".to_owned(),
        document_id_ref: "nb.doc.local.01".to_owned(),
        chip_class: ExecutionLocusChipClass::LocalHost,
        chip_state: ExecutionLocusChipState::Active,
        target_name_label: "localhost".to_owned(),
        tooltip_label: "Running on local host.".to_owned(),
        boundary_cue_visible: false,
        summary: "Local host execution-locus chip.".to_owned(),
    }
}

fn sample_remote_chip() -> ExecutionLocusChip {
    ExecutionLocusChip {
        record_kind: EXECUTION_LOCUS_CHIP_RECORD_KIND.to_owned(),
        notebook_header_kernel_bar_schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        chip_id: "nb.chip.remote.01".to_owned(),
        document_id_ref: "nb.doc.remote.01".to_owned(),
        chip_class: ExecutionLocusChipClass::ManagedWorkspace,
        chip_state: ExecutionLocusChipState::Active,
        target_name_label: "managed-workspace:gpu-pool".to_owned(),
        tooltip_label: "Running on managed workspace gpu-pool.".to_owned(),
        boundary_cue_visible: true,
        summary: "Managed workspace execution-locus chip.".to_owned(),
    }
}

fn sample_no_kernel_chip() -> ExecutionLocusChip {
    ExecutionLocusChip {
        record_kind: EXECUTION_LOCUS_CHIP_RECORD_KIND.to_owned(),
        notebook_header_kernel_bar_schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        chip_id: "nb.chip.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.no_kernel.01".to_owned(),
        chip_class: ExecutionLocusChipClass::NoKernel,
        chip_state: ExecutionLocusChipState::Disconnected,
        target_name_label: "No kernel".to_owned(),
        tooltip_label: "No kernel selected; document is editable and reviewable.".to_owned(),
        boundary_cue_visible: false,
        summary: "No-kernel execution-locus chip.".to_owned(),
    }
}

fn local_state() -> NotebookHeaderKernelBarState {
    NotebookHeaderKernelBarState {
        record_kind: NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND.to_owned(),
        notebook_header_kernel_bar_schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        state_id: "nb.header_kernel_bar.local.01".to_owned(),
        document_id_ref: "nb.doc.local.01".to_owned(),
        document_path_token_ref: "vfs.path.token.local.01".to_owned(),
        document_title_label: "Local notebook".to_owned(),
        document_trust_class: crate::NotebookDocumentTrustClass::InheritedFromWorkspace,
        dirty_state_class: crate::NotebookDirtyStateClass::Clean,
        kernel_selection_state: crate::KernelSelectionState::SelectedKernelResolved,
        kernel_origin_class: crate::KernelOriginClass::LocalManagedToolchainKernel,
        kernel_session_id_ref: Some("kernel.session.local.01".to_owned()),
        kernelspec_ref: Some("kernelspec.python.312.local".to_owned()),
        local_vs_remote_boundary_cue_visible: false,
        target_identity_witness_ref: None,
        remote_agent_session_id_ref: None,
        execution_context_root_ref: Some("exec.ctx.local.01".to_owned()),
        execution_locus_chips: vec![sample_local_chip()],
        paired_export_posture: crate::NotebookPairedExportPosture::NotApplicable,
        paired_export_ref: None,
        available_actions: vec![
            crate::KernelBarActionClass::Restart,
            crate::KernelBarActionClass::Interrupt,
            crate::KernelBarActionClass::ChangeKernel,
        ],
        last_successful_run: Some(crate::NotebookLastSuccessfulRunSummary {
            run_id_ref: "run.local.notebook.01".to_owned(),
            attempt_id_ref: "attempt.local.notebook.01".to_owned(),
            completed_at: "2026-05-18T10:00:00Z".to_owned(),
            cells_completed: 4,
            summary_label: "Last run completed 4 cells on local kernel.".to_owned(),
        }),
        auto_rerun_forbidden: true,
        captured_at: "2026-05-18T10:05:00Z".to_owned(),
        summary: "Local managed-toolchain kernel; header and kernel bar state.".to_owned(),
    }
}

fn no_kernel_state() -> NotebookHeaderKernelBarState {
    NotebookHeaderKernelBarState {
        record_kind: NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND.to_owned(),
        notebook_header_kernel_bar_schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        state_id: "nb.header_kernel_bar.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.no_kernel.01".to_owned(),
        document_path_token_ref: "vfs.path.token.no_kernel.01".to_owned(),
        document_title_label: "No-kernel notebook".to_owned(),
        document_trust_class: crate::NotebookDocumentTrustClass::InheritedFromWorkspace,
        dirty_state_class: crate::NotebookDirtyStateClass::Clean,
        kernel_selection_state: crate::KernelSelectionState::NoKernelSelected,
        kernel_origin_class: crate::KernelOriginClass::NoKernel,
        kernel_session_id_ref: None,
        kernelspec_ref: None,
        local_vs_remote_boundary_cue_visible: false,
        target_identity_witness_ref: None,
        remote_agent_session_id_ref: None,
        execution_context_root_ref: None,
        execution_locus_chips: vec![sample_no_kernel_chip()],
        paired_export_posture: crate::NotebookPairedExportPosture::NotApplicable,
        paired_export_ref: None,
        available_actions: vec![crate::KernelBarActionClass::SelectKernel],
        last_successful_run: None,
        auto_rerun_forbidden: true,
        captured_at: "2026-05-18T10:05:00Z".to_owned(),
        summary: "No kernel selected; document is editable and reviewable.".to_owned(),
    }
}

fn remote_state() -> NotebookHeaderKernelBarState {
    NotebookHeaderKernelBarState {
        record_kind: NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND.to_owned(),
        notebook_header_kernel_bar_schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        state_id: "nb.header_kernel_bar.remote.01".to_owned(),
        document_id_ref: "nb.doc.remote.01".to_owned(),
        document_path_token_ref: "vfs.path.token.remote.01".to_owned(),
        document_title_label: "Remote notebook".to_owned(),
        document_trust_class: crate::NotebookDocumentTrustClass::InheritedFromWorkspace,
        dirty_state_class: crate::NotebookDirtyStateClass::Clean,
        kernel_selection_state: crate::KernelSelectionState::SelectedKernelResolved,
        kernel_origin_class: crate::KernelOriginClass::RemoteAgentPrimaryKernel,
        kernel_session_id_ref: Some("kernel.session.remote.01".to_owned()),
        kernelspec_ref: Some("kernelspec.python.312.remote".to_owned()),
        local_vs_remote_boundary_cue_visible: true,
        target_identity_witness_ref: Some("witness.remote.01".to_owned()),
        remote_agent_session_id_ref: Some("remote.agent.session.01".to_owned()),
        execution_context_root_ref: Some("exec.ctx.remote.01".to_owned()),
        execution_locus_chips: vec![sample_remote_chip()],
        paired_export_posture: crate::NotebookPairedExportPosture::DerivedNotebookToScript,
        paired_export_ref: Some("paired.export.script.remote.01".to_owned()),
        available_actions: vec![
            crate::KernelBarActionClass::Restart,
            crate::KernelBarActionClass::Interrupt,
            crate::KernelBarActionClass::Reconnect,
        ],
        last_successful_run: None,
        auto_rerun_forbidden: true,
        captured_at: "2026-05-18T10:05:00Z".to_owned(),
        summary: "Remote agent primary kernel; local-vs-remote boundary cue visible.".to_owned(),
    }
}

#[test]
fn local_chip_validates_clean() {
    let chip = sample_local_chip();
    assert!(
        chip.validate().is_empty(),
        "local chip should be clean: {:?}",
        chip.validate()
    );
}

#[test]
fn remote_chip_validates_clean() {
    let chip = sample_remote_chip();
    assert!(
        chip.validate().is_empty(),
        "remote chip should be clean: {:?}",
        chip.validate()
    );
}

#[test]
fn no_kernel_chip_validates_clean() {
    let chip = sample_no_kernel_chip();
    assert!(
        chip.validate().is_empty(),
        "no-kernel chip should be clean: {:?}",
        chip.validate()
    );
}

#[test]
fn remote_chip_requires_boundary_cue() {
    let mut chip = sample_remote_chip();
    chip.boundary_cue_visible = false;
    let findings = chip.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "execution_locus_chip.remote_boundary_cue"));
}

#[test]
fn local_chip_must_not_show_boundary_cue() {
    let mut chip = sample_local_chip();
    chip.boundary_cue_visible = true;
    let findings = chip.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "execution_locus_chip.local_no_boundary_cue"));
}

#[test]
fn no_kernel_chip_must_not_be_active() {
    let mut chip = sample_no_kernel_chip();
    chip.chip_state = ExecutionLocusChipState::Active;
    let findings = chip.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "execution_locus_chip.no_kernel_active"));
}

#[test]
fn local_state_validates_clean() {
    let state = local_state();
    assert!(
        state.validate().is_empty(),
        "local state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn no_kernel_state_validates_clean() {
    let state = no_kernel_state();
    assert!(
        state.validate().is_empty(),
        "no-kernel state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn remote_state_validates_clean() {
    let state = remote_state();
    assert!(
        state.validate().is_empty(),
        "remote state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn no_kernel_must_not_expose_restart() {
    let mut state = no_kernel_state();
    state
        .available_actions
        .push(crate::KernelBarActionClass::Restart);
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| { f.check_id == "notebook_header_kernel_bar_state.no_kernel_actions_running" }));
}

#[test]
fn remote_state_requires_boundary_cue_and_witnesses() {
    let mut state = remote_state();
    state.local_vs_remote_boundary_cue_visible = false;
    state.target_identity_witness_ref = None;
    state.remote_agent_session_id_ref = None;
    let findings = state.validate();
    let codes: Vec<&str> = findings.iter().map(|f| f.check_id.as_str()).collect();
    assert!(codes.contains(&"notebook_header_kernel_bar_state.remote_boundary_cue"));
    assert!(codes.contains(&"notebook_header_kernel_bar_state.target_identity_witness_required"));
    assert!(codes.contains(&"notebook_header_kernel_bar_state.remote_agent_session_id_required"));
}

#[test]
fn auto_rerun_forbidden_must_be_true() {
    let mut state = local_state();
    state.auto_rerun_forbidden = false;
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| { f.check_id == "notebook_header_kernel_bar_state.auto_rerun_forbidden" }));
}

#[test]
fn paired_export_ref_required_when_derived() {
    let mut state = remote_state();
    state.paired_export_ref = None;
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| { f.check_id == "notebook_header_kernel_bar_state.paired_export_ref_required" }));
}

#[test]
fn paired_export_ref_must_be_none_when_not_applicable() {
    let mut state = local_state();
    state.paired_export_ref = Some("paired.export.01".to_owned());
    let findings = state.validate();
    assert!(findings.iter().any(|f| {
        f.check_id == "notebook_header_kernel_bar_state.paired_export_ref_not_applicable"
    }));
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookHeaderKernelBarPacket {
        schema_version: NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION,
        record_kind: NOTEBOOK_HEADER_KERNEL_BAR_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.header_kernel_bar.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        execution_locus_chip_classes: ExecutionLocusChipClass::ALL.to_vec(),
        execution_locus_chip_states: ExecutionLocusChipState::ALL.to_vec(),
        example_execution_locus_chips: vec![
            sample_local_chip(),
            sample_remote_chip(),
            sample_no_kernel_chip(),
        ],
        example_header_kernel_bar_states: vec![local_state(), no_kernel_state(), remote_state()],
        summary:
            "Notebook header, kernel bar, execution-locus chips, and paired-export state packet v1."
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
    let packet = current_notebook_header_kernel_bar_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_HEADER_KERNEL_BAR_PACKET_RECORD_KIND
    );
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(ExecutionLocusChipClass::LocalHost.as_str(), "local_host");
    assert_eq!(
        ExecutionLocusChipClass::ManagedWorkspace.as_str(),
        "managed_workspace"
    );
    assert!(ExecutionLocusChipClass::ManagedWorkspace.is_remote_boundary());
    assert!(!ExecutionLocusChipClass::LocalHost.is_remote_boundary());
    assert!(ExecutionLocusChipClass::NoKernel.is_no_kernel());
    assert_eq!(ExecutionLocusChipState::Active.as_str(), "active");
    assert_eq!(
        ExecutionLocusChipState::PolicyBlocked.as_str(),
        "policy_blocked"
    );
}
