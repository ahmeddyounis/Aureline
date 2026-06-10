use super::*;

fn sample_follow_state_following() -> NotebookCollaborationFollowState {
    NotebookCollaborationFollowState {
        record_kind: NOTEBOOK_COLLABORATION_FOLLOW_STATE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        follow_state_id: "nb.follow.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        session_envelope_ref: "collab.session.01".to_owned(),
        participant_ref: "actor.alice".to_owned(),
        follow_mode: NotebookFollowMode::FollowingPresenter,
        follow_target_class: NotebookFollowTargetClass::Presenter,
        follow_target_ref: "actor.bob".to_owned(),
        current_cell_id_ref: Some("nb.cell.intro".to_owned()),
        current_output_handle_ref: None,
        follow_explanation: None,
        summary: "Alice is following Bob the presenter on the intro cell.".to_owned(),
    }
}

fn sample_follow_state_breakaway() -> NotebookCollaborationFollowState {
    NotebookCollaborationFollowState {
        record_kind: NOTEBOOK_COLLABORATION_FOLLOW_STATE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        follow_state_id: "nb.follow.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        session_envelope_ref: "collab.session.01".to_owned(),
        participant_ref: "actor.charlie".to_owned(),
        follow_mode: NotebookFollowMode::Breakaway,
        follow_target_class: NotebookFollowTargetClass::SpecificCell,
        follow_target_ref: "nb.cell.plot".to_owned(),
        current_cell_id_ref: Some("nb.cell.plot".to_owned()),
        current_output_handle_ref: Some("output.plot.01".to_owned()),
        follow_explanation: Some("Charlie browsed independently to inspect the plot output.".to_owned()),
        summary: "Charlie broke away to inspect the plot output independently.".to_owned(),
    }
}

fn sample_follow_state_degraded() -> NotebookCollaborationFollowState {
    NotebookCollaborationFollowState {
        record_kind: NOTEBOOK_COLLABORATION_FOLLOW_STATE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        follow_state_id: "nb.follow.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        session_envelope_ref: "collab.session.01".to_owned(),
        participant_ref: "actor.dana".to_owned(),
        follow_mode: NotebookFollowMode::FollowDegraded,
        follow_target_class: NotebookFollowTargetClass::Presenter,
        follow_target_ref: "actor.bob".to_owned(),
        current_cell_id_ref: None,
        current_output_handle_ref: None,
        follow_explanation: Some("Relay degraded; follow position is stale.".to_owned()),
        summary: "Dana's follow state is degraded due to relay issues.".to_owned(),
    }
}

fn sample_presenter_state_active() -> NotebookPresenterState {
    NotebookPresenterState {
        record_kind: NOTEBOOK_PRESENTER_STATE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        presenter_state_id: "nb.presenter.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        session_envelope_ref: "collab.session.01".to_owned(),
        presenter_mode: NotebookPresenterMode::ActivePresenter,
        presenter_actor_ref: "actor.bob".to_owned(),
        shared_cell_id_ref: Some("nb.cell.intro".to_owned()),
        shared_output_handle_ref: None,
        presenter_actions: vec![
            NotebookPresenterActionClass::ShareCell,
            NotebookPresenterActionClass::Handoff,
            NotebookPresenterActionClass::Pause,
        ],
        summary: "Bob is actively presenting the intro cell.".to_owned(),
    }
}

fn sample_presenter_state_handoff() -> NotebookPresenterState {
    NotebookPresenterState {
        record_kind: NOTEBOOK_PRESENTER_STATE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        presenter_state_id: "nb.presenter.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        session_envelope_ref: "collab.session.01".to_owned(),
        presenter_mode: NotebookPresenterMode::HandoffPending,
        presenter_actor_ref: "actor.bob".to_owned(),
        shared_cell_id_ref: Some("nb.cell.plot".to_owned()),
        shared_output_handle_ref: Some("output.plot.01".to_owned()),
        presenter_actions: vec![NotebookPresenterActionClass::Handoff],
        summary: "Bob is handing off presenter control to a co-presenter.".to_owned(),
    }
}

fn sample_presenter_state_idle() -> NotebookPresenterState {
    NotebookPresenterState {
        record_kind: NOTEBOOK_PRESENTER_STATE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        presenter_state_id: "nb.presenter.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        session_envelope_ref: "collab.session.01".to_owned(),
        presenter_mode: NotebookPresenterMode::Idle,
        presenter_actor_ref: "actor.bob".to_owned(),
        shared_cell_id_ref: None,
        shared_output_handle_ref: None,
        presenter_actions: vec![],
        summary: "Bob is idle; no cell or output is currently shared.".to_owned(),
    }
}

fn sample_runtime_disclosure_live() -> NotebookRuntimeDisclosure {
    NotebookRuntimeDisclosure {
        record_kind: NOTEBOOK_RUNTIME_DISCLOSURE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        runtime_disclosure_id: "nb.runtime.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        disclosure_class: NotebookRuntimeDisclosureClass::LiveRuntime,
        kernel_session_ref: Some("kernel.session.py.01".to_owned()),
        captured_at: None,
        disclosure_actions: vec![
            NotebookRuntimeDisclosureActionClass::RefreshRuntime,
            NotebookRuntimeDisclosureActionClass::SwitchToCaptured,
        ],
        summary: "The collaborative view is showing live runtime state from the active Python kernel.".to_owned(),
    }
}

fn sample_runtime_disclosure_captured() -> NotebookRuntimeDisclosure {
    NotebookRuntimeDisclosure {
        record_kind: NOTEBOOK_RUNTIME_DISCLOSURE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        runtime_disclosure_id: "nb.runtime.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        disclosure_class: NotebookRuntimeDisclosureClass::CapturedOutput,
        kernel_session_ref: None,
        captured_at: Some("2026-06-09T12:00:00Z".to_owned()),
        disclosure_actions: vec![
            NotebookRuntimeDisclosureActionClass::AcknowledgeCaptured,
            NotebookRuntimeDisclosureActionClass::SwitchToLive,
            NotebookRuntimeDisclosureActionClass::RequestKernel,
        ],
        summary: "The collaborative view is showing captured output from a prior session.".to_owned(),
    }
}

fn sample_runtime_disclosure_no_kernel() -> NotebookRuntimeDisclosure {
    NotebookRuntimeDisclosure {
        record_kind: NOTEBOOK_RUNTIME_DISCLOSURE_RECORD_KIND.to_owned(),
        notebook_collaboration_follow_presenter_schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        runtime_disclosure_id: "nb.runtime.03".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        disclosure_class: NotebookRuntimeDisclosureClass::NoKernel,
        kernel_session_ref: None,
        captured_at: None,
        disclosure_actions: vec![NotebookRuntimeDisclosureActionClass::RequestKernel],
        summary: "No kernel is selected; only the saved document and captured outputs are visible.".to_owned(),
    }
}

#[test]
fn follow_state_following_validates_clean() {
    let f = sample_follow_state_following();
    assert!(
        f.validate().is_empty(),
        "following follow_state should be clean: {:?}",
        f.validate()
    );
}

#[test]
fn follow_state_breakaway_validates_clean() {
    let f = sample_follow_state_breakaway();
    assert!(
        f.validate().is_empty(),
        "breakaway follow_state should be clean: {:?}",
        f.validate()
    );
}

#[test]
fn follow_state_degraded_validates_clean() {
    let f = sample_follow_state_degraded();
    assert!(
        f.validate().is_empty(),
        "degraded follow_state should be clean: {:?}",
        f.validate()
    );
}

#[test]
fn presenter_state_active_validates_clean() {
    let p = sample_presenter_state_active();
    assert!(
        p.validate().is_empty(),
        "active presenter_state should be clean: {:?}",
        p.validate()
    );
}

#[test]
fn presenter_state_handoff_validates_clean() {
    let p = sample_presenter_state_handoff();
    assert!(
        p.validate().is_empty(),
        "handoff presenter_state should be clean: {:?}",
        p.validate()
    );
}

#[test]
fn presenter_state_idle_validates_clean() {
    let p = sample_presenter_state_idle();
    assert!(
        p.validate().is_empty(),
        "idle presenter_state should be clean: {:?}",
        p.validate()
    );
}

#[test]
fn runtime_disclosure_live_validates_clean() {
    let r = sample_runtime_disclosure_live();
    assert!(
        r.validate().is_empty(),
        "live runtime_disclosure should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn runtime_disclosure_captured_validates_clean() {
    let r = sample_runtime_disclosure_captured();
    assert!(
        r.validate().is_empty(),
        "captured runtime_disclosure should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn runtime_disclosure_no_kernel_validates_clean() {
    let r = sample_runtime_disclosure_no_kernel();
    assert!(
        r.validate().is_empty(),
        "no_kernel runtime_disclosure should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn follow_state_rejects_empty_document_id_ref() {
    let mut f = sample_follow_state_following();
    f.document_id_ref = "".to_owned();
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_collaboration_follow_state.document_id_ref_required"));
}

#[test]
fn follow_state_rejects_empty_participant_ref() {
    let mut f = sample_follow_state_following();
    f.participant_ref = "".to_owned();
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_collaboration_follow_state.participant_ref_required"));
}

#[test]
fn follow_state_rejects_missing_explanation_when_breakaway() {
    let mut f = sample_follow_state_breakaway();
    f.follow_explanation = None;
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_collaboration_follow_state.follow_explanation_required"));
}

#[test]
fn follow_state_rejects_missing_explanation_when_degraded() {
    let mut f = sample_follow_state_degraded();
    f.follow_explanation = None;
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_collaboration_follow_state.follow_explanation_required"));
}

#[test]
fn presenter_state_rejects_empty_presenter_actor_ref() {
    let mut p = sample_presenter_state_active();
    p.presenter_actor_ref = "".to_owned();
    let findings = p.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_presenter_state.presenter_actor_ref_required"));
}

#[test]
fn presenter_state_rejects_missing_shared_ref_when_active() {
    let mut p = sample_presenter_state_active();
    p.shared_cell_id_ref = None;
    let findings = p.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_presenter_state.shared_ref_required_when_active"));
}

#[test]
fn presenter_state_allows_missing_shared_ref_when_idle() {
    let mut p = sample_presenter_state_idle();
    p.shared_cell_id_ref = None;
    p.shared_output_handle_ref = None;
    assert!(p.validate().is_empty(), "idle presenter may have no shared ref");
}

#[test]
fn runtime_disclosure_rejects_missing_kernel_session_ref_when_live() {
    let mut r = sample_runtime_disclosure_live();
    r.kernel_session_ref = None;
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_runtime_disclosure.kernel_session_ref_required"));
}

#[test]
fn runtime_disclosure_rejects_missing_captured_at_when_captured() {
    let mut r = sample_runtime_disclosure_captured();
    r.captured_at = None;
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_runtime_disclosure.captured_at_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookFollowMode::FollowingPresenter.as_str(), "following_presenter");
    assert_eq!(NotebookFollowMode::Breakaway.as_str(), "breakaway");
    assert_eq!(NotebookFollowTargetClass::SpecificCell.as_str(), "specific_cell");
    assert_eq!(NotebookPresenterMode::HandoffPending.as_str(), "handoff_pending");
    assert_eq!(NotebookPresenterActionClass::ShareOutput.as_str(), "share_output");
    assert_eq!(NotebookRuntimeDisclosureClass::StaleRuntime.as_str(), "stale_runtime");
    assert_eq!(NotebookRuntimeDisclosureActionClass::RequestKernel.as_str(), "request_kernel");
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookCollaborationFollowPresenterPacket {
        schema_version: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION,
        record_kind: NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.collab_follow_presenter.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        follow_modes: NotebookFollowMode::ALL.to_vec(),
        follow_target_classes: NotebookFollowTargetClass::ALL.to_vec(),
        presenter_modes: NotebookPresenterMode::ALL.to_vec(),
        presenter_actions: NotebookPresenterActionClass::ALL.to_vec(),
        runtime_disclosure_classes: NotebookRuntimeDisclosureClass::ALL.to_vec(),
        runtime_disclosure_actions: NotebookRuntimeDisclosureActionClass::ALL.to_vec(),
        example_follow_states: vec![
            sample_follow_state_following(),
            sample_follow_state_breakaway(),
            sample_follow_state_degraded(),
        ],
        example_presenter_states: vec![
            sample_presenter_state_active(),
            sample_presenter_state_handoff(),
            sample_presenter_state_idle(),
        ],
        example_runtime_disclosures: vec![
            sample_runtime_disclosure_live(),
            sample_runtime_disclosure_captured(),
            sample_runtime_disclosure_no_kernel(),
        ],
        summary: "Notebook collaboration follow and presenter state with live-versus-captured runtime disclosure packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_collaboration_follow_presenter_packet()
        .expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_RECORD_KIND
    );
}
