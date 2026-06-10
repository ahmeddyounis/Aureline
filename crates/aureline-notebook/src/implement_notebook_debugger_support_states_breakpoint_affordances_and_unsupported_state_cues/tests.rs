use super::*;

fn sample_live_state() -> NotebookDebuggerSupportState {
    NotebookDebuggerSupportState {
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        state_id: "nb.debugger.state.live.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        debugger_support_state_class: DebuggerSupportStateClass::Idle,
        underlying_bridge_state_ref: "nb.debugger.bridge.01".to_owned(),
        breakpoint_affordances: vec![
            BreakpointAffordance {
                record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                affordance_id: "nb.debugger.bp.set.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.01".to_owned()),
                breakpoint_affordance_class: BreakpointAffordanceClass::SetBreakpoint,
                posture_class: BreakpointAffordancePostureClass::Available,
                summary: "Set breakpoint in cell 1.".to_owned(),
            },
            BreakpointAffordance {
                record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                affordance_id: "nb.debugger.bp.clear.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: None,
                breakpoint_affordance_class: BreakpointAffordanceClass::ClearAllBreakpoints,
                posture_class: BreakpointAffordancePostureClass::Available,
                summary: "Clear all breakpoints.".to_owned(),
            },
        ],
        unsupported_state_cues: vec![],
        summary: "Debugger idle with available breakpoint actions.".to_owned(),
    }
}

fn sample_paused_state() -> NotebookDebuggerSupportState {
    NotebookDebuggerSupportState {
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        state_id: "nb.debugger.state.paused.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        debugger_support_state_class: DebuggerSupportStateClass::Paused,
        underlying_bridge_state_ref: "nb.debugger.bridge.02".to_owned(),
        breakpoint_affordances: vec![
            BreakpointAffordance {
                record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                affordance_id: "nb.debugger.bp.remove.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.02".to_owned()),
                breakpoint_affordance_class: BreakpointAffordanceClass::RemoveBreakpoint,
                posture_class: BreakpointAffordancePostureClass::Available,
                summary: "Remove breakpoint in cell 2.".to_owned(),
            },
        ],
        unsupported_state_cues: vec![],
        summary: "Debugger paused on a breakpoint.".to_owned(),
    }
}

fn sample_no_kernel_state() -> NotebookDebuggerSupportState {
    NotebookDebuggerSupportState {
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        state_id: "nb.debugger.state.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: None,
        debugger_support_state_class: DebuggerSupportStateClass::Unsupported,
        underlying_bridge_state_ref: "nb.debugger.bridge.none".to_owned(),
        breakpoint_affordances: vec![
            BreakpointAffordance {
                record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                affordance_id: "nb.debugger.bp.unavailable.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: None,
                breakpoint_affordance_class: BreakpointAffordanceClass::SetBreakpoint,
                posture_class: BreakpointAffordancePostureClass::UnavailableNoKernel,
                summary: "Breakpoint unavailable because no kernel is selected.".to_owned(),
            },
        ],
        unsupported_state_cues: vec![UnsupportedStateCue {
            record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
            notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
            cue_id: "nb.debugger.cue.no_kernel.01".to_owned(),
            document_id_ref: "nb.doc.example".to_owned(),
            unsupported_state_cue_class: UnsupportedStateCueClass::NoKernel,
            tooltip_label: "No kernel is selected.".to_owned(),
            action_hint_label: "Select a kernel to enable debugging.".to_owned(),
            summary: "Debugging is unavailable because no kernel is selected.".to_owned(),
        }],
        summary: "Debugger unsupported because no kernel is selected.".to_owned(),
    }
}

fn sample_degraded_state() -> NotebookDebuggerSupportState {
    NotebookDebuggerSupportState {
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        state_id: "nb.debugger.state.degraded.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        debugger_support_state_class: DebuggerSupportStateClass::Degraded,
        underlying_bridge_state_ref: "nb.debugger.bridge.03".to_owned(),
        breakpoint_affordances: vec![
            BreakpointAffordance {
                record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                affordance_id: "nb.debugger.bp.policy.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.03".to_owned()),
                breakpoint_affordance_class: BreakpointAffordanceClass::SetConditionalBreakpoint,
                posture_class: BreakpointAffordancePostureClass::UnavailablePolicyBlocked,
                summary: "Conditional breakpoints are blocked by policy.".to_owned(),
            },
        ],
        unsupported_state_cues: vec![
            UnsupportedStateCue {
                record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                cue_id: "nb.debugger.cue.policy.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                unsupported_state_cue_class: UnsupportedStateCueClass::PolicyBlocked,
                tooltip_label: "Debugging is restricted by policy.".to_owned(),
                action_hint_label: "Contact your administrator to request debugging access."
                    .to_owned(),
                summary: "Debugger degraded due to policy restrictions.".to_owned(),
            },
        ],
        summary: "Debugger degraded due to policy restrictions.".to_owned(),
    }
}

fn sample_partial_state() -> NotebookDebuggerSupportState {
    NotebookDebuggerSupportState {
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        state_id: "nb.debugger.state.partial.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        debugger_support_state_class: DebuggerSupportStateClass::UnsupportedPartial,
        underlying_bridge_state_ref: "nb.debugger.bridge.04".to_owned(),
        breakpoint_affordances: vec![
            BreakpointAffordance {
                record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                affordance_id: "nb.debugger.bp.review.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: Some("nb.cell.04".to_owned()),
                breakpoint_affordance_class: BreakpointAffordanceClass::SetBreakpoint,
                posture_class: BreakpointAffordancePostureClass::UnavailableRequiresReview,
                summary: "Breakpoint requires review before use.".to_owned(),
            },
        ],
        unsupported_state_cues: vec![
            UnsupportedStateCue {
                record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                cue_id: "nb.debugger.cue.review.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                unsupported_state_cue_class: UnsupportedStateCueClass::RemoteParityUnverified,
                tooltip_label: "Remote debugger parity is unverified.".to_owned(),
                action_hint_label: "Run a diagnostic to verify remote debugger parity.".to_owned(),
                summary: "Partial support due to unverified remote parity.".to_owned(),
            },
        ],
        summary: "Debugger partially supported due to unverified remote parity.".to_owned(),
    }
}

fn sample_disconnected_state() -> NotebookDebuggerSupportState {
    NotebookDebuggerSupportState {
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        state_id: "nb.debugger.state.disconnected.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        debugger_support_state_class: DebuggerSupportStateClass::Disconnected,
        underlying_bridge_state_ref: "nb.debugger.bridge.05".to_owned(),
        breakpoint_affordances: vec![],
        unsupported_state_cues: vec![
            UnsupportedStateCue {
                record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
                notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
                cue_id: "nb.debugger.cue.restart.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                unsupported_state_cue_class: UnsupportedStateCueClass::BridgeCancelledByRestart,
                tooltip_label: "Debugger bridge was cancelled by a kernel restart.".to_owned(),
                action_hint_label: "Reconnect the kernel to restore debugging.".to_owned(),
                summary: "Debugger disconnected after kernel restart.".to_owned(),
            },
        ],
        summary: "Debugger disconnected after kernel restart.".to_owned(),
    }
}

#[test]
fn live_state_validates_clean() {
    let state = sample_live_state();
    assert!(
        state.validate().is_empty(),
        "live state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn paused_state_validates_clean() {
    let state = sample_paused_state();
    assert!(
        state.validate().is_empty(),
        "paused state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn no_kernel_state_validates_clean() {
    let state = sample_no_kernel_state();
    assert!(
        state.validate().is_empty(),
        "no-kernel state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn degraded_state_validates_clean() {
    let state = sample_degraded_state();
    assert!(
        state.validate().is_empty(),
        "degraded state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn partial_state_validates_clean() {
    let state = sample_partial_state();
    assert!(
        state.validate().is_empty(),
        "partial state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn disconnected_state_validates_clean() {
    let state = sample_disconnected_state();
    assert!(
        state.validate().is_empty(),
        "disconnected state should be clean: {:?}",
        state.validate()
    );
}

#[test]
fn live_state_missing_kernel_is_rejected() {
    let mut state = sample_live_state();
    state.kernel_session_id_ref = None;
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_debugger_support_state.live_session_requires_kernel"));
}

#[test]
fn degraded_state_missing_cues_is_rejected() {
    let mut state = sample_degraded_state();
    state.unsupported_state_cues = vec![];
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_debugger_support_state.cues_required_for_degraded"));
}

#[test]
fn degraded_state_with_available_breakpoint_is_rejected() {
    let mut state = sample_degraded_state();
    state.breakpoint_affordances = vec![BreakpointAffordance {
        record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        affordance_id: "nb.debugger.bp.bad.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: None,
        breakpoint_affordance_class: BreakpointAffordanceClass::SetBreakpoint,
        posture_class: BreakpointAffordancePostureClass::Available,
        summary: "Bad available breakpoint.".to_owned(),
    }];
    let findings = state.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_debugger_support_state.no_available_breakpoints_when_degraded"));
}

#[test]
fn supported_state_with_cues_is_rejected() {
    let mut state = sample_live_state();
    state.unsupported_state_cues = vec![UnsupportedStateCue {
        record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        cue_id: "nb.debugger.cue.bad.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        unsupported_state_cue_class: UnsupportedStateCueClass::AdapterUnavailable,
        tooltip_label: "Adapter unavailable.".to_owned(),
        action_hint_label: "Install adapter.".to_owned(),
        summary: "Bad cue on supported state.".to_owned(),
    }];
    let findings = state.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_debugger_support_state.no_cues_when_supported"));
}

#[test]
fn empty_bridge_ref_is_rejected() {
    let mut state = sample_live_state();
    state.underlying_bridge_state_ref = "".to_owned();
    let findings = state.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_debugger_support_state.underlying_bridge_state_ref_required"));
}

#[test]
fn breakpoint_affordance_clear_all_with_cell_ref_is_rejected() {
    let affordance = BreakpointAffordance {
        record_kind: BREAKPOINT_AFFORDANCE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        affordance_id: "nb.debugger.bp.clear.bad.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: Some("nb.cell.01".to_owned()),
        breakpoint_affordance_class: BreakpointAffordanceClass::ClearAllBreakpoints,
        posture_class: BreakpointAffordancePostureClass::Available,
        summary: "Clear all with cell ref.".to_owned(),
    };
    let findings = affordance.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "breakpoint_affordance.global_no_cell_ref"));
}

#[test]
fn unsupported_state_cue_empty_tooltip_is_rejected() {
    let cue = UnsupportedStateCue {
        record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        cue_id: "nb.debugger.cue.bad.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        unsupported_state_cue_class: UnsupportedStateCueClass::PolicyBlocked,
        tooltip_label: "".to_owned(),
        action_hint_label: "Action hint.".to_owned(),
        summary: "Bad cue.".to_owned(),
    };
    let findings = cue.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "unsupported_state_cue.tooltip_label_required"));
}

#[test]
fn unsupported_state_cue_empty_action_hint_is_rejected() {
    let cue = UnsupportedStateCue {
        record_kind: UNSUPPORTED_STATE_CUE_RECORD_KIND.to_owned(),
        notebook_debugger_support_schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        cue_id: "nb.debugger.cue.bad.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        unsupported_state_cue_class: UnsupportedStateCueClass::PolicyBlocked,
        tooltip_label: "Tooltip.".to_owned(),
        action_hint_label: "".to_owned(),
        summary: "Bad cue.".to_owned(),
    };
    let findings = cue.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "unsupported_state_cue.action_hint_label_required"));
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookDebuggerSupportPacket {
        schema_version: NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION,
        record_kind: NOTEBOOK_DEBUGGER_SUPPORT_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.debugger.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        debugger_support_state_classes: DebuggerSupportStateClass::ALL.to_vec(),
        breakpoint_affordance_classes: BreakpointAffordanceClass::ALL.to_vec(),
        breakpoint_affordance_posture_classes: BreakpointAffordancePostureClass::ALL.to_vec(),
        unsupported_state_cue_classes: UnsupportedStateCueClass::ALL.to_vec(),
        example_notebook_debugger_support_states: vec![
            sample_live_state(),
            sample_paused_state(),
            sample_no_kernel_state(),
            sample_degraded_state(),
            sample_partial_state(),
            sample_disconnected_state(),
        ],
        example_breakpoint_affordances: vec![
            sample_live_state().breakpoint_affordances[0].clone(),
            sample_live_state().breakpoint_affordances[1].clone(),
            sample_no_kernel_state().breakpoint_affordances[0].clone(),
            sample_degraded_state().breakpoint_affordances[0].clone(),
            sample_partial_state().breakpoint_affordances[0].clone(),
        ],
        example_unsupported_state_cues: vec![
            sample_no_kernel_state().unsupported_state_cues[0].clone(),
            sample_degraded_state().unsupported_state_cues[0].clone(),
            sample_partial_state().unsupported_state_cues[0].clone(),
            sample_disconnected_state().unsupported_state_cues[0].clone(),
        ],
        summary: "Notebook debugger-support states, breakpoint affordances, and unsupported-state cues packet v1."
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
    let packet = current_notebook_debugger_support_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_DEBUGGER_SUPPORT_PACKET_RECORD_KIND);
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(DebuggerSupportStateClass::Idle.as_str(), "idle");
    assert_eq!(DebuggerSupportStateClass::UnsupportedPartial.as_str(), "unsupported_partial");

    assert_eq!(BreakpointAffordanceClass::SetBreakpoint.as_str(), "set_breakpoint");
    assert_eq!(
        BreakpointAffordanceClass::SetConditionalBreakpoint.as_str(),
        "set_conditional_breakpoint"
    );

    assert_eq!(
        BreakpointAffordancePostureClass::UnavailablePolicyBlocked.as_str(),
        "unavailable_policy_blocked"
    );
    assert_eq!(
        BreakpointAffordancePostureClass::UnavailableRequiresReview.as_str(),
        "unavailable_requires_review"
    );

    assert_eq!(UnsupportedStateCueClass::NoKernel.as_str(), "no_kernel");
    assert_eq!(
        UnsupportedStateCueClass::CellSteppingUnsupported.as_str(),
        "cell_stepping_unsupported"
    );
}
