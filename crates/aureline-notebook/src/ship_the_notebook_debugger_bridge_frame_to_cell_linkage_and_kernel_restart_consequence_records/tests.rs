use super::*;

fn sample_exact_match_link() -> DebuggerFrameCellLink {
    DebuggerFrameCellLink {
        record_kind: DEBUGGER_FRAME_CELL_LINK_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        link_id: "nb.debugger.link.exact.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: "nb.kernel.session.01".to_owned(),
        frame_ref: "nb.debugger.frame.01".to_owned(),
        cell_id_ref: Some("nb.cell.01".to_owned()),
        link_class: DebuggerFrameCellLinkClass::ExactCellMatch,
        link_posture_class: DebuggerFrameCellLinkPostureClass::ActionableStepIntoCell,
        source_line_ref: Some("nb.source.line.01".to_owned()),
        summary: "Debugger frame exactly matches cell 1.".to_owned(),
    }
}

fn sample_no_mapping_link() -> DebuggerFrameCellLink {
    DebuggerFrameCellLink {
        record_kind: DEBUGGER_FRAME_CELL_LINK_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        link_id: "nb.debugger.link.none.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: "nb.kernel.session.01".to_owned(),
        frame_ref: "nb.debugger.frame.02".to_owned(),
        cell_id_ref: None,
        link_class: DebuggerFrameCellLinkClass::NoCellMapping,
        link_posture_class: DebuggerFrameCellLinkPostureClass::UnsupportedNoSourceMap,
        source_line_ref: None,
        summary: "Debugger frame cannot be mapped to any cell.".to_owned(),
    }
}

fn sample_stale_link() -> DebuggerFrameCellLink {
    DebuggerFrameCellLink {
        record_kind: DEBUGGER_FRAME_CELL_LINK_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        link_id: "nb.debugger.link.stale.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: "nb.kernel.session.01".to_owned(),
        frame_ref: "nb.debugger.frame.03".to_owned(),
        cell_id_ref: Some("nb.cell.02".to_owned()),
        link_class: DebuggerFrameCellLinkClass::FrameStaleAfterRestart,
        link_posture_class: DebuggerFrameCellLinkPostureClass::StaleReinitializeRequired,
        source_line_ref: None,
        summary: "Debugger frame is stale after kernel restart.".to_owned(),
    }
}

fn sample_library_link() -> DebuggerFrameCellLink {
    DebuggerFrameCellLink {
        record_kind: DEBUGGER_FRAME_CELL_LINK_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        link_id: "nb.debugger.link.lib.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: "nb.kernel.session.01".to_owned(),
        frame_ref: "nb.debugger.frame.04".to_owned(),
        cell_id_ref: Some("nb.cell.03".to_owned()),
        link_class: DebuggerFrameCellLinkClass::InCellLibraryCode,
        link_posture_class: DebuggerFrameCellLinkPostureClass::ViewOnlyNoStep,
        source_line_ref: Some("nb.source.line.02".to_owned()),
        summary: "Debugger frame is in library code called from cell 3.".to_owned(),
    }
}

fn sample_heuristic_link() -> DebuggerFrameCellLink {
    DebuggerFrameCellLink {
        record_kind: DEBUGGER_FRAME_CELL_LINK_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        link_id: "nb.debugger.link.heuristic.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: "nb.kernel.session.01".to_owned(),
        frame_ref: "nb.debugger.frame.05".to_owned(),
        cell_id_ref: Some("nb.cell.04".to_owned()),
        link_class: DebuggerFrameCellLinkClass::NearestCellHeuristic,
        link_posture_class: DebuggerFrameCellLinkPostureClass::ActionableStepOverCell,
        source_line_ref: Some("nb.source.line.03".to_owned()),
        summary: "Debugger frame mapped to nearest cell 4 via heuristic.".to_owned(),
    }
}

fn sample_external_dependency_link() -> DebuggerFrameCellLink {
    DebuggerFrameCellLink {
        record_kind: DEBUGGER_FRAME_CELL_LINK_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        link_id: "nb.debugger.link.ext.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: "nb.kernel.session.01".to_owned(),
        frame_ref: "nb.debugger.frame.06".to_owned(),
        cell_id_ref: Some("nb.cell.05".to_owned()),
        link_class: DebuggerFrameCellLinkClass::InCellExternalDependency,
        link_posture_class: DebuggerFrameCellLinkPostureClass::ViewOnlyNoStep,
        source_line_ref: Some("nb.source.line.04".to_owned()),
        summary: "Debugger frame is in an external dependency reached from cell 5.".to_owned(),
    }
}

fn sample_preserved_consequence() -> KernelRestartDebuggerConsequence {
    KernelRestartDebuggerConsequence {
        record_kind: KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        consequence_id: "nb.debugger.consequence.preserved.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        prior_kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        next_kernel_session_id_ref: Some("nb.kernel.session.02".to_owned()),
        restart_kind: KernelRestartKind::UserInitiatedRestart,
        consequence_class: KernelRestartConsequenceClass::BridgePreservedSameSession,
        in_flight_debug_cancelled: false,
        breakpoints_affected: 0,
        reattach_action_class: KernelRestartDebuggerActionClass::ReattachAutomatically,
        reconnect_review_sheet_ref: None,
        auto_rerun_forbidden: true,
        summary: "Debugger bridge preserved across same-identity restart.".to_owned(),
    }
}

fn sample_reset_consequence() -> KernelRestartDebuggerConsequence {
    KernelRestartDebuggerConsequence {
        record_kind: KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        consequence_id: "nb.debugger.consequence.reset.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        prior_kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        next_kernel_session_id_ref: Some("nb.kernel.session.02".to_owned()),
        restart_kind: KernelRestartKind::UserInitiatedRestart,
        consequence_class: KernelRestartConsequenceClass::BridgeResetFreshSession,
        in_flight_debug_cancelled: true,
        breakpoints_affected: 3,
        reattach_action_class: KernelRestartDebuggerActionClass::ReattachOnDemand,
        reconnect_review_sheet_ref: Some("nb.reconnect.review.01".to_owned()),
        auto_rerun_forbidden: true,
        summary: "Debugger bridge reset on fresh session; breakpoints lost.".to_owned(),
    }
}

fn sample_cancelled_consequence() -> KernelRestartDebuggerConsequence {
    KernelRestartDebuggerConsequence {
        record_kind: KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        consequence_id: "nb.debugger.consequence.cancelled.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        prior_kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        next_kernel_session_id_ref: None,
        restart_kind: KernelRestartKind::TransportLostReconnectAttempted,
        consequence_class: KernelRestartConsequenceClass::BridgeCancelledPendingReconnect,
        in_flight_debug_cancelled: true,
        breakpoints_affected: 2,
        reattach_action_class: KernelRestartDebuggerActionClass::ReattachRequiresReview,
        reconnect_review_sheet_ref: Some("nb.reconnect.review.02".to_owned()),
        auto_rerun_forbidden: true,
        summary: "Debugger bridge cancelled pending reconnect; review required.".to_owned(),
    }
}

fn sample_unavailable_consequence() -> KernelRestartDebuggerConsequence {
    KernelRestartDebuggerConsequence {
        record_kind: KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        consequence_id: "nb.debugger.consequence.unavailable.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        prior_kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        next_kernel_session_id_ref: None,
        restart_kind: KernelRestartKind::PolicyDeniesContinuedExecution,
        consequence_class: KernelRestartConsequenceClass::BridgeUnavailableNoKernel,
        in_flight_debug_cancelled: true,
        breakpoints_affected: 1,
        reattach_action_class: KernelRestartDebuggerActionClass::ReattachUnavailable,
        reconnect_review_sheet_ref: Some("nb.reconnect.review.03".to_owned()),
        auto_rerun_forbidden: true,
        summary: "Debugger bridge unavailable after policy denies execution.".to_owned(),
    }
}

fn sample_bridge_cancelled_consequence() -> KernelRestartDebuggerConsequence {
    KernelRestartDebuggerConsequence {
        record_kind: KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND.to_owned(),
        notebook_debugger_bridge_schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        consequence_id: "nb.debugger.consequence.bridge_cancelled.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        prior_kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        next_kernel_session_id_ref: None,
        restart_kind: KernelRestartKind::BridgeCancelledByRestart,
        consequence_class: KernelRestartConsequenceClass::BreakpointsLostOnRestart,
        in_flight_debug_cancelled: true,
        breakpoints_affected: 4,
        reattach_action_class: KernelRestartDebuggerActionClass::ReattachRequiresReview,
        reconnect_review_sheet_ref: Some("nb.reconnect.review.04".to_owned()),
        auto_rerun_forbidden: true,
        summary: "Debugger bridge cancelled by restart; breakpoints lost.".to_owned(),
    }
}

#[test]
fn exact_match_link_validates_clean() {
    let link = sample_exact_match_link();
    assert!(
        link.validate().is_empty(),
        "exact match link should be clean: {:?}",
        link.validate()
    );
}

#[test]
fn no_mapping_link_validates_clean() {
    let link = sample_no_mapping_link();
    assert!(
        link.validate().is_empty(),
        "no mapping link should be clean: {:?}",
        link.validate()
    );
}

#[test]
fn stale_link_validates_clean() {
    let link = sample_stale_link();
    assert!(
        link.validate().is_empty(),
        "stale link should be clean: {:?}",
        link.validate()
    );
}

#[test]
fn library_link_validates_clean() {
    let link = sample_library_link();
    assert!(
        link.validate().is_empty(),
        "library link should be clean: {:?}",
        link.validate()
    );
}

#[test]
fn heuristic_link_validates_clean() {
    let link = sample_heuristic_link();
    assert!(
        link.validate().is_empty(),
        "heuristic link should be clean: {:?}",
        link.validate()
    );
}

#[test]
fn external_dependency_link_validates_clean() {
    let link = sample_external_dependency_link();
    assert!(
        link.validate().is_empty(),
        "external dependency link should be clean: {:?}",
        link.validate()
    );
}

#[test]
fn preserved_consequence_validates_clean() {
    let consequence = sample_preserved_consequence();
    assert!(
        consequence.validate().is_empty(),
        "preserved consequence should be clean: {:?}",
        consequence.validate()
    );
}

#[test]
fn reset_consequence_validates_clean() {
    let consequence = sample_reset_consequence();
    assert!(
        consequence.validate().is_empty(),
        "reset consequence should be clean: {:?}",
        consequence.validate()
    );
}

#[test]
fn cancelled_consequence_validates_clean() {
    let consequence = sample_cancelled_consequence();
    assert!(
        consequence.validate().is_empty(),
        "cancelled consequence should be clean: {:?}",
        consequence.validate()
    );
}

#[test]
fn unavailable_consequence_validates_clean() {
    let consequence = sample_unavailable_consequence();
    assert!(
        consequence.validate().is_empty(),
        "unavailable consequence should be clean: {:?}",
        consequence.validate()
    );
}

#[test]
fn bridge_cancelled_consequence_validates_clean() {
    let consequence = sample_bridge_cancelled_consequence();
    assert!(
        consequence.validate().is_empty(),
        "bridge cancelled consequence should be clean: {:?}",
        consequence.validate()
    );
}

#[test]
fn no_mapping_link_with_cell_ref_is_rejected() {
    let mut link = sample_no_mapping_link();
    link.cell_id_ref = Some("nb.cell.01".to_owned());
    let findings = link.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "debugger_frame_cell_link.no_cell_mapping_no_cell_ref"));
}

#[test]
fn exact_match_link_missing_cell_ref_is_rejected() {
    let mut link = sample_exact_match_link();
    link.cell_id_ref = None;
    let findings = link.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "debugger_frame_cell_link.cell_ref_required"));
}

#[test]
fn stale_posture_without_stale_class_is_rejected() {
    let mut link = sample_stale_link();
    link.link_class = DebuggerFrameCellLinkClass::ExactCellMatch;
    let findings = link.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "debugger_frame_cell_link.stale_requires_stale_class"));
}

#[test]
fn preserved_consequence_with_cancel_is_rejected() {
    let mut consequence = sample_preserved_consequence();
    consequence.in_flight_debug_cancelled = true;
    let findings = consequence.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernel_restart_debugger_consequence.preserved_no_cancel"));
}

#[test]
fn unavailable_consequence_with_next_session_is_rejected() {
    let mut consequence = sample_unavailable_consequence();
    consequence.next_kernel_session_id_ref = Some("nb.kernel.session.02".to_owned());
    let findings = consequence.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "kernel_restart_debugger_consequence.unavailable_no_next_session"));
}

#[test]
fn bridge_cancelled_consequence_with_auto_reattach_is_rejected() {
    let mut consequence = sample_bridge_cancelled_consequence();
    consequence.reattach_action_class = KernelRestartDebuggerActionClass::ReattachAutomatically;
    let findings = consequence.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "kernel_restart_debugger_consequence.restart_kind_requires_review_or_unavailable"));
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookDebuggerBridgePacket {
        schema_version: NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION,
        record_kind: NOTEBOOK_DEBUGGER_BRIDGE_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.debugger.bridge.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        debugger_frame_cell_link_classes: DebuggerFrameCellLinkClass::ALL.to_vec(),
        debugger_frame_cell_link_posture_classes: DebuggerFrameCellLinkPostureClass::ALL.to_vec(),
        kernel_restart_kinds: KernelRestartKind::ALL.to_vec(),
        kernel_restart_consequence_classes: KernelRestartConsequenceClass::ALL.to_vec(),
        kernel_restart_debugger_action_classes: KernelRestartDebuggerActionClass::ALL.to_vec(),
        example_debugger_frame_cell_links: vec![
            sample_exact_match_link(),
            sample_no_mapping_link(),
            sample_stale_link(),
            sample_library_link(),
            sample_heuristic_link(),
            sample_external_dependency_link(),
        ],
        example_kernel_restart_debugger_consequences: vec![
            sample_preserved_consequence(),
            sample_reset_consequence(),
            sample_cancelled_consequence(),
            sample_unavailable_consequence(),
            sample_bridge_cancelled_consequence(),
        ],
        summary: "Notebook debugger bridge, frame-to-cell linkage, and kernel restart consequence records packet v1."
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
    let packet = current_notebook_debugger_bridge_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_DEBUGGER_BRIDGE_PACKET_RECORD_KIND
    );
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(
        DebuggerFrameCellLinkClass::ExactCellMatch.as_str(),
        "exact_cell_match"
    );
    assert_eq!(
        DebuggerFrameCellLinkClass::FrameStaleAfterRestart.as_str(),
        "frame_stale_after_restart"
    );

    assert_eq!(
        DebuggerFrameCellLinkPostureClass::ActionableStepIntoCell.as_str(),
        "actionable_step_into_cell"
    );
    assert_eq!(
        DebuggerFrameCellLinkPostureClass::StaleReinitializeRequired.as_str(),
        "stale_reinitialize_required"
    );

    assert_eq!(
        KernelRestartKind::TrustDowngradeCancelsInFlight.as_str(),
        "trust_downgrade_cancels_in_flight"
    );
    assert_eq!(
        KernelRestartKind::BridgeCancelledByRestart.as_str(),
        "bridge_cancelled_by_restart"
    );

    assert_eq!(
        KernelRestartConsequenceClass::BreakpointsLostOnRestart.as_str(),
        "breakpoints_lost_on_restart"
    );
    assert_eq!(
        KernelRestartConsequenceClass::BridgePreservedSameSession.as_str(),
        "bridge_preserved_same_session"
    );

    assert_eq!(
        KernelRestartDebuggerActionClass::ReattachRequiresReview.as_str(),
        "reattach_requires_review"
    );
    assert_eq!(
        KernelRestartDebuggerActionClass::ReattachUnavailable.as_str(),
        "reattach_unavailable"
    );
}
