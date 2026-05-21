use super::*;
use std::collections::BTreeSet;

fn baseline_manifest() -> InviteSessionManifest {
    InviteSessionManifest {
        invite_session_manifest_schema_version: INVITE_SESSION_MANIFEST_SCHEMA_VERSION,
        record_kind: INVITE_SESSION_MANIFEST_RECORD_KIND.to_owned(),
        invite_id: "collab_invite:terminal.pair".to_owned(),
        session_owner_ref: "collab_principal:owner.alex".to_owned(),
        session_owner_label: "Alex (session owner)".to_owned(),
        session_lane_class: SessionLaneClass::SharedTerminal,
        offered_role_class: OfferedRoleClass::DriverCandidate,
        invite_capability_class: InviteCapabilityClass::ObserverWithControlRequest,
        client_class: ClientClass::DesktopNative,
        retention_recording_posture_class: RetentionRecordingPostureClass::LiveOnlyNoRetention,
        invite_expiry: ExpiryWindow {
            posture_class: ExpiryPostureClass::ExpiresAtSessionEnd,
            expires_at_ref: None,
            expiry_label: "Expires when the session ends".to_owned(),
        },
        headline_label: "Join the shared terminal".to_owned(),
        manifest_summary:
            "Observer-first invite into the shared terminal; control is a separate request."
                .to_owned(),
        contract_doc_ref: INVITE_AND_AUTHORITY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn baseline_ticket() -> AuthorityEscrowTicket {
    AuthorityEscrowTicket {
        authority_escrow_ticket_schema_version: AUTHORITY_ESCROW_TICKET_SCHEMA_VERSION,
        record_kind: AUTHORITY_ESCROW_TICKET_RECORD_KIND.to_owned(),
        ticket_id: "authority_escrow:terminal.pair.input".to_owned(),
        invite_ref: "collab_invite:terminal.pair".to_owned(),
        authority_scope_class: AuthorityScopeClass::TerminalInput,
        grant_state_class: GrantStateClass::GrantedActive,
        requested_by_ref: "collab_principal:guest.sam".to_owned(),
        requested_by_label: "Sam (guest)".to_owned(),
        grant_expiry: ExpiryWindow {
            posture_class: ExpiryPostureClass::ExpiresAtFixedTime,
            expires_at_ref: Some("collab_deadline:terminal.pair.input.t1".to_owned()),
            expiry_label: "Expires in 15 minutes".to_owned(),
        },
        downgrade_path_class: DowngradePathClass::RevertToObserver,
        reapproval_required_on: vec![
            ReapprovalTriggerClass::OnReconnect,
            ReapprovalTriggerClass::OnDeviceHandoff,
            ReapprovalTriggerClass::OnSessionRestart,
        ],
        scope_summary: "Temporary terminal input for the shared terminal pane only.".to_owned(),
        headline_label: "Temporary terminal input".to_owned(),
        contract_doc_ref: INVITE_AND_AUTHORITY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn baseline_continuity() -> AuthorityContinuity {
    AuthorityContinuity {
        context_change_class: ContextChangeClass::NoneSteadyState,
        resumed_privileged_silently: false,
        post_change_grant_state_class: GrantStateClass::GrantedActive,
        reapproval_satisfied: false,
        local_lane_usable: true,
        available_actions: vec![
            ContinuityActionClass::RevokeGrant,
            ContinuityActionClass::ContinueLocalOnly,
            ContinuityActionClass::ExportAuthorityRecord,
        ],
        preserved_grant_ref: Some("authority_escrow:terminal.pair.input".to_owned()),
        continuity_summary: "Steady state; the active grant carries its reapproval triggers."
            .to_owned(),
    }
}

fn baseline_event() -> HistoryAttentionEvent {
    HistoryAttentionEvent {
        event_id: "collab_authority_event:terminal.pair.input.granted".to_owned(),
        attention_surface_class: AttentionSurfaceClass::DurableAttention,
        reason_code_class: ReasonCodeClass::ControlGrantedTemporary,
        durable: true,
        export_safe: true,
        event_summary: "Temporary terminal input granted; recorded in durable attention."
            .to_owned(),
    }
}

fn baseline_sheet() -> InviteAuthorityReviewSheet {
    InviteAuthorityReviewSheet {
        invite_authority_review_sheet_schema_version: INVITE_AUTHORITY_REVIEW_SHEET_SCHEMA_VERSION,
        record_kind: INVITE_AUTHORITY_REVIEW_SHEET_RECORD_KIND.to_owned(),
        sheet_id: "invite_authority_review_sheet:terminal.pair".to_owned(),
        sheet_summary: "Shared terminal invite and temporary terminal-input escrow.".to_owned(),
        invite_manifest: baseline_manifest(),
        authority_ticket: baseline_ticket(),
        authority_continuity: baseline_continuity(),
        history_event: baseline_event(),
        contract_doc_ref: INVITE_AND_AUTHORITY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

#[test]
fn baseline_sheet_validates() {
    baseline_sheet()
        .validate()
        .expect("baseline sheet validates");
}

#[test]
fn observer_only_invite_cannot_grant_terminal_control() {
    let mut sheet = baseline_sheet();
    // Drop the invite to observer-only, but keep the role consistent so the
    // failure is the capability mismatch on the requested scope, not the role.
    sheet.invite_manifest.invite_capability_class = InviteCapabilityClass::ObserverOnly;
    sheet.invite_manifest.offered_role_class = OfferedRoleClass::Observer;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::ControlRequestedWithoutCapability { .. }
    ));
}

#[test]
fn driver_candidate_role_requires_control_capability() {
    let mut sheet = baseline_sheet();
    sheet.invite_manifest.invite_capability_class = InviteCapabilityClass::ObserverWithFollow;
    // role stays driver_candidate, which observer_with_follow cannot back.
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::RoleCapabilityMismatch { .. }
    ));
}

#[test]
fn follow_handoff_is_allowed_by_follow_capability() {
    let mut sheet = baseline_sheet();
    sheet.invite_manifest.session_lane_class = SessionLaneClass::PresentationPreview;
    sheet.invite_manifest.offered_role_class = OfferedRoleClass::FollowViewer;
    sheet.invite_manifest.invite_capability_class = InviteCapabilityClass::ObserverWithFollow;
    sheet.authority_ticket.authority_scope_class = AuthorityScopeClass::FollowHandoff;
    sheet
        .validate()
        .expect("follow handoff validates under follow capability");
}

#[test]
fn control_capable_invite_must_be_bounded() {
    let mut sheet = baseline_sheet();
    sheet.invite_manifest.invite_expiry.posture_class = ExpiryPostureClass::PerpetualNoExpiry;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::InviteExpiryUnbounded { .. }
    ));
}

#[test]
fn untrusted_client_cannot_carry_recording() {
    let mut sheet = baseline_sheet();
    sheet.invite_manifest.client_class = ClientClass::UntrustedWeb;
    sheet.invite_manifest.retention_recording_posture_class =
        RetentionRecordingPostureClass::RecordingWithConsent;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::RetentionUnsafeForClient { .. }
    ));
}

#[test]
fn active_grant_must_be_bounded() {
    let mut sheet = baseline_sheet();
    sheet.authority_ticket.grant_expiry.posture_class = ExpiryPostureClass::PerpetualNoExpiry;
    sheet.authority_ticket.grant_expiry.expires_at_ref = None;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::GrantExpiryUnbounded { .. }
    ));
}

#[test]
fn active_grant_must_guard_silent_resume() {
    let mut sheet = baseline_sheet();
    // Drop the device-handoff trigger from an active grant.
    sheet.authority_ticket.reapproval_required_on = vec![
        ReapprovalTriggerClass::OnReconnect,
        ReapprovalTriggerClass::OnSessionRestart,
    ];
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::SilentResumeNotGuarded { .. }
    ));
}

#[test]
fn live_grant_needs_a_reapproval_trigger() {
    let mut sheet = baseline_sheet();
    sheet.authority_ticket.grant_state_class = GrantStateClass::FrozenPendingReapproval;
    sheet.authority_ticket.reapproval_required_on.clear();
    // Frozen is live but not privileged-active, so the missing-trigger guard
    // fires before the active-only silent-resume guard.
    sheet.history_event.reason_code_class = ReasonCodeClass::GrantFrozenPendingReapproval;
    sheet.authority_continuity.post_change_grant_state_class =
        GrantStateClass::FrozenPendingReapproval;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::NoReapprovalTriggers { .. }
    ));
}

#[test]
fn scope_must_match_invited_lane() {
    let mut sheet = baseline_sheet();
    // Terminal-input scope on a presentation invite.
    sheet.invite_manifest.session_lane_class = SessionLaneClass::PresentationPreview;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::ScopeLaneMismatch { .. }
    ));
}

#[test]
fn reason_code_must_match_grant_state() {
    let mut sheet = baseline_sheet();
    sheet.history_event.reason_code_class = ReasonCodeClass::ControlRevoked;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::ReasonCodeStateMismatch { .. }
    ));
}

#[test]
fn ticket_must_bind_to_invite() {
    let mut sheet = baseline_sheet();
    sheet.authority_ticket.invite_ref = "collab_invite:some.other".to_owned();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::TicketInviteRefMismatch { .. }
    ));
}

#[test]
fn reconnect_cannot_silently_resume_active_grant() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.context_change_class = ContextChangeClass::Reconnected;
    sheet.authority_continuity.post_change_grant_state_class = GrantStateClass::GrantedActive;
    sheet.authority_continuity.reapproval_satisfied = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::SilentPrivilegedResume { .. }
    ));
}

#[test]
fn reconnect_with_reapproval_is_allowed() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.context_change_class = ContextChangeClass::BrowserToDesktopHandoff;
    sheet.authority_continuity.post_change_grant_state_class = GrantStateClass::GrantedActive;
    sheet.authority_continuity.reapproval_satisfied = true;
    sheet
        .validate()
        .expect("reapproved resume after handoff validates");
}

#[test]
fn reconnect_can_freeze_instead_of_resuming() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.context_change_class = ContextChangeClass::AppRestart;
    sheet.authority_continuity.post_change_grant_state_class =
        GrantStateClass::FrozenPendingReapproval;
    sheet.authority_continuity.reapproval_satisfied = false;
    sheet.validate().expect("freeze after restart validates");
}

#[test]
fn silent_resume_flag_is_never_allowed() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.resumed_privileged_silently = true;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::SilentResumeNotAllowed { .. }
    ));
}

#[test]
fn local_lane_must_stay_usable() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.local_lane_usable = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::LocalLaneNotUsable { .. }
    ));
}

#[test]
fn revoked_grant_keeps_local_lane_and_validates() {
    let mut sheet = baseline_sheet();
    sheet.authority_ticket.grant_state_class = GrantStateClass::Revoked;
    sheet.history_event.reason_code_class = ReasonCodeClass::ControlRevoked;
    sheet.authority_continuity.context_change_class = ContextChangeClass::AppRestart;
    sheet.authority_continuity.post_change_grant_state_class = GrantStateClass::Revoked;
    sheet.authority_continuity.available_actions = vec![
        ContinuityActionClass::ContinueLocalOnly,
        ContinuityActionClass::ContinueObserverOnly,
        ContinuityActionClass::ExportAuthorityRecord,
    ];
    sheet
        .validate()
        .expect("revoked grant with usable local lane validates");
}

#[test]
fn continuity_needs_an_action() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.available_actions.clear();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::NoContinuityActions { .. }
    ));
}

#[test]
fn durable_event_must_be_durable_and_export_safe() {
    let mut sheet = baseline_sheet();
    sheet.history_event.durable = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::EventNotDurable { .. }
    ));

    let mut sheet = baseline_sheet();
    sheet.history_event.export_safe = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::EventNotExportSafe { .. }
    ));
}

#[test]
fn raw_url_in_owner_ref_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.invite_manifest.session_owner_ref = "https://example.com/u/alex".to_owned();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::RawRefLeak { .. }
    ));
}

#[test]
fn raw_email_in_requested_by_ref_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.authority_ticket.requested_by_ref = "sam@example.com".to_owned();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::RawRefLeak { .. }
    ));
}

#[test]
fn fixed_expiry_requires_a_deadline_ref() {
    let mut sheet = baseline_sheet();
    sheet.authority_ticket.grant_expiry.expires_at_ref = None;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::FixedExpiryMissingDeadline { .. }
    ));
}

#[test]
fn malformed_event_id_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.history_event.event_id = "event:wrong.prefix".to_owned();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::MalformedEventId { .. }
    ));
}

#[test]
fn preserved_grant_ref_must_match_ticket() {
    let mut sheet = baseline_sheet();
    sheet.authority_continuity.preserved_grant_ref =
        Some("authority_escrow:not.this.one".to_owned());
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::PreservedGrantRefMismatch { .. }
    ));
}

#[test]
fn render_plaintext_is_stable_and_mentions_key_axes() {
    let sheet = baseline_sheet();
    sheet.validate().expect("sheet validates");
    let block = sheet.render_plaintext();
    assert!(block.contains("invite_authority_review_sheet:terminal.pair"));
    assert!(block.contains("lane=shared_terminal"));
    assert!(block.contains("capability=observer_with_control_request"));
    assert!(block.contains("scope=terminal_input state=granted_active"));
    assert!(block.contains("reapproval on: on_reconnect, on_device_handoff, on_session_restart"));
    assert!(block.contains("reason=control_granted_temporary"));
}

#[test]
fn validate_sheets_rejects_duplicate_ids() {
    let sheet = baseline_sheet();
    let err = validate_sheets(&[sheet.clone(), sheet]).unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::MalformedSheetId { .. }
    ));
}

#[test]
fn all_lane_labels_are_distinct() {
    let classes = [
        SessionLaneClass::SharedTerminal,
        SessionLaneClass::SharedDebug,
        SessionLaneClass::PresentationPreview,
    ];
    let labels: BTreeSet<&str> = classes.iter().map(|c| c.label()).collect();
    assert_eq!(labels.len(), classes.len(), "lane labels must be distinct");
}

#[test]
fn reason_code_expected_for_state_covers_every_state() {
    let states = [
        GrantStateClass::Requested,
        GrantStateClass::GrantedActive,
        GrantStateClass::Expired,
        GrantStateClass::Revoked,
        GrantStateClass::FrozenPendingReapproval,
        GrantStateClass::Denied,
    ];
    for state in states {
        // Every state maps to a concrete export-safe reason code.
        let _ = ReasonCodeClass::expected_for_state(state).as_str();
    }
}
