//! Fixture replay for the collaboration invite / observer-first join and
//! temporary authority-escrow review corpus published under
//! `fixtures/collab/m3/invite_and_authority_review/`.
//!
//! Positive cases MUST validate end-to-end; negative cases MUST fail validation
//! with a typed [`InviteAuthorityValidationError`].

use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::invite_review::{
    AuthorityScopeClass, ContextChangeClass, GrantStateClass, InviteAuthorityReviewSheet,
    InviteAuthorityValidationError, InviteCapabilityClass, SessionLaneClass,
};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/collab/m3/invite_and_authority_review")
}

fn load_sheet(rel: &str) -> InviteAuthorityReviewSheet {
    let path = fixture_root().join(rel);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read directory {}: {err}", dir.display()))
    {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out.sort();
    out
}

#[test]
fn all_positive_fixtures_validate() {
    let dir = fixture_root().join("positive");
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "expected positive fixtures under {}",
        dir.display()
    );
    for path in files {
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let sheet: InviteAuthorityReviewSheet = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        sheet.validate().unwrap_or_else(|err| {
            panic!(
                "positive fixture {} should validate but failed: {err}",
                path.display()
            )
        });
        let text = sheet.render_plaintext();
        assert!(
            text.contains(&sheet.sheet_id),
            "plaintext export must mention sheet id ({})",
            path.display()
        );
    }
}

#[test]
fn all_negative_fixtures_fail_validation() {
    let dir = fixture_root().join("negative");
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "expected negative fixtures under {}",
        dir.display()
    );
    for path in files {
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let sheet: InviteAuthorityReviewSheet = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        assert!(
            sheet.validate().is_err(),
            "negative fixture {} should fail validation but passed",
            path.display()
        );
    }
}

#[test]
fn observer_first_pending_shows_scope_before_accept() {
    let sheet = load_sheet("positive/observer_first_terminal_invite_pending.json");
    sheet.validate().expect("observer-first positive validates");
    // The invite never confers control directly — only the right to request it.
    assert_eq!(
        sheet.invite_manifest.invite_capability_class,
        InviteCapabilityClass::ObserverWithControlRequest
    );
    assert_eq!(
        sheet.authority_ticket.grant_state_class,
        GrantStateClass::Requested,
        "an observer-first sheet shows control as pending, not granted"
    );
    assert!(
        !sheet.authority_ticket.scope_summary.is_empty(),
        "the exact scope must be stated before accept"
    );
    assert_eq!(
        sheet.authority_ticket.authority_scope_class,
        AuthorityScopeClass::TerminalInput
    );
}

#[test]
fn debug_grant_is_bounded_and_guards_silent_resume() {
    let sheet = load_sheet("positive/debug_control_granted_temporary.json");
    sheet.validate().expect("debug grant positive validates");
    assert_eq!(
        sheet.invite_manifest.session_lane_class,
        SessionLaneClass::SharedDebug
    );
    assert_eq!(
        sheet.authority_ticket.authority_scope_class,
        AuthorityScopeClass::DebugStepControl
    );
    assert!(
        sheet
            .authority_ticket
            .grant_expiry
            .posture_class
            .is_bounded(),
        "a temporary grant must be bounded"
    );
}

#[test]
fn presenter_handoff_resumes_only_after_reapproval() {
    let sheet = load_sheet("positive/presenter_handoff_reapproved_after_reconnect.json");
    sheet
        .validate()
        .expect("presenter handoff positive validates");
    assert_eq!(
        sheet.authority_continuity.context_change_class,
        ContextChangeClass::Reconnected
    );
    assert_eq!(
        sheet.authority_continuity.post_change_grant_state_class,
        GrantStateClass::GrantedActive
    );
    assert!(
        sheet.authority_continuity.reapproval_satisfied,
        "an active grant after reconnect must be explicitly reapproved"
    );
}

#[test]
fn revoked_grant_keeps_local_lane_usable_after_restart() {
    let sheet = load_sheet("positive/revoked_grant_after_restart_local_preserved.json");
    sheet.validate().expect("revoked grant positive validates");
    assert_eq!(
        sheet.authority_ticket.grant_state_class,
        GrantStateClass::Revoked
    );
    assert_eq!(
        sheet.authority_continuity.context_change_class,
        ContextChangeClass::AppRestart
    );
    assert_eq!(
        sheet.authority_continuity.post_change_grant_state_class,
        GrantStateClass::Revoked,
        "a revoked grant must stay visibly revoked after restart"
    );
    assert!(
        sheet.authority_continuity.local_lane_usable,
        "the local shared-lane row must remain usable after revocation"
    );
}

#[test]
fn negative_control_without_capability_is_typed_error() {
    let sheet = load_sheet("negative/control_requested_without_capability.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::ControlRequestedWithoutCapability { .. }
    ));
}

#[test]
fn negative_perpetual_grant_is_typed_error() {
    let sheet = load_sheet("negative/perpetual_control_grant.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::GrantExpiryUnbounded { .. }
    ));
}

#[test]
fn negative_silent_resume_is_typed_error() {
    let sheet = load_sheet("negative/silent_resume_after_reconnect.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::SilentPrivilegedResume { .. }
    ));
}

#[test]
fn negative_missing_resume_guard_is_typed_error() {
    let sheet = load_sheet("negative/active_grant_missing_resume_guard.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::SilentResumeNotGuarded { .. }
    ));
}

#[test]
fn negative_scope_lane_mismatch_is_typed_error() {
    let sheet = load_sheet("negative/scope_lane_mismatch.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::ScopeLaneMismatch { .. }
    ));
}

#[test]
fn negative_local_lane_unusable_is_typed_error() {
    let sheet = load_sheet("negative/local_lane_unusable.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::LocalLaneNotUsable { .. }
    ));
}

#[test]
fn negative_reason_code_state_mismatch_is_typed_error() {
    let sheet = load_sheet("negative/reason_code_state_mismatch.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::ReasonCodeStateMismatch { .. }
    ));
}

#[test]
fn negative_ticket_invite_ref_mismatch_is_typed_error() {
    let sheet = load_sheet("negative/ticket_invite_ref_mismatch.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::TicketInviteRefMismatch { .. }
    ));
}

#[test]
fn negative_role_capability_mismatch_is_typed_error() {
    let sheet = load_sheet("negative/role_capability_mismatch.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::RoleCapabilityMismatch { .. }
    ));
}

#[test]
fn negative_unbounded_control_invite_is_typed_error() {
    let sheet = load_sheet("negative/unbounded_control_invite.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        InviteAuthorityValidationError::InviteExpiryUnbounded { .. }
    ));
}
