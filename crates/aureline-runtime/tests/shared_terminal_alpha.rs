//! Fixture-driven coverage for the shared-terminal control-channel alpha
//! page.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    ControlRevocationCauseClass, LocalContinuityClass, ParticipantRoleClass,
    PresenterHandoffOutcomeClass, SharedTerminalAlphaPage, SharedTerminalAuditEventClass,
    SharedTerminalControlStateClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/shared_terminal_alpha/page.json")
}

fn load_page() -> SharedTerminalAlphaPage {
    let text = fs::read_to_string(fixture_path()).expect("read shared-terminal alpha fixture");
    serde_json::from_str(&text).expect("parse shared-terminal alpha fixture")
}

#[test]
fn alpha_fixture_validates() {
    let page = load_page();
    let report = page.validate();
    assert!(
        report.passed,
        "shared-terminal alpha fixture failed validation: {:#?}",
        report.findings
    );
}

#[test]
fn fixture_covers_all_four_control_states() {
    let page = load_page();
    let report = page.validate();
    for state in [
        SharedTerminalControlStateClass::ViewOnlyObserver,
        SharedTerminalControlStateClass::RequestControlPending,
        SharedTerminalControlStateClass::ActiveControlGrantee,
        SharedTerminalControlStateClass::ControlRevoked,
    ] {
        assert!(
            report.coverage.control_states.contains(&state),
            "missing control-state coverage: {state:?}"
        );
    }
}

#[test]
fn fixture_covers_presenter_handoff_outcomes() {
    let page = load_page();
    let report = page.validate();
    for outcome in [
        PresenterHandoffOutcomeClass::PresenterRoleAccepted,
        PresenterHandoffOutcomeClass::PresenterRoleAutoObserver,
    ] {
        assert!(
            report
                .coverage
                .presenter_handoff_outcomes
                .contains(&outcome),
            "missing presenter-handoff outcome coverage: {outcome:?}"
        );
    }
}

#[test]
fn fixture_covers_audit_events_for_control_changes_and_handoffs() {
    let page = load_page();
    let report = page.validate();
    for class in [
        SharedTerminalAuditEventClass::ControlActiveStarted,
        SharedTerminalAuditEventClass::ControlRevoked,
        SharedTerminalAuditEventClass::PresenterHandoffResolved,
        SharedTerminalAuditEventClass::AuditDenialEmitted,
    ] {
        assert!(
            report.coverage.audit_event_classes.contains(&class),
            "missing audit-event coverage: {class:?}"
        );
    }
}

#[test]
fn fixture_covers_local_continuity_classes() {
    let page = load_page();
    let report = page.validate();
    for class in [
        LocalContinuityClass::OwnerInputPreservedAfterGranteeRevoked,
        LocalContinuityClass::GranteeDemotedObserverNoInputInjection,
    ] {
        assert!(
            report.coverage.continuity_classes.contains(&class),
            "missing continuity-class coverage: {class:?}"
        );
    }
}

#[test]
fn active_control_states_cite_a_control_grant_ref() {
    let page = load_page();
    for state in &page.control_states {
        if state.control_state == SharedTerminalControlStateClass::ActiveControlGrantee
            || state.control_state == SharedTerminalControlStateClass::ControlRevoked
        {
            assert!(
                state.control_grant_ref.is_some(),
                "active/revoked state {} must cite a control_grant_ref",
                state.state_id
            );
        }
        if state.control_state == SharedTerminalControlStateClass::ViewOnlyObserver
            || state.control_state == SharedTerminalControlStateClass::RequestControlPending
        {
            assert!(
                state.control_grant_ref.is_none(),
                "view-only / pending state {} must not cite a control_grant_ref",
                state.state_id
            );
        }
    }
}

#[test]
fn revoked_states_cite_revocation_ref_and_cause() {
    let page = load_page();
    let revoked = page
        .control_states
        .iter()
        .find(|state| state.control_state == SharedTerminalControlStateClass::ControlRevoked)
        .expect("revoked state present");
    assert!(revoked.revocation_ref.is_some());
    assert!(matches!(
        revoked.revocation_cause,
        Some(ControlRevocationCauseClass::OwnerRevoked)
    ));
}

#[test]
fn participant_roles_resolve_locally() {
    let page = load_page();
    for state in &page.control_states {
        match state.participant_role {
            ParticipantRoleClass::SessionOwner
            | ParticipantRoleClass::Participant
            | ParticipantRoleClass::Approver
            | ParticipantRoleClass::Admin => {}
        }
    }
}

#[test]
fn local_continuity_invariants_hold_on_every_observation() {
    let page = load_page();
    assert!(
        !page.continuity_observations.is_empty(),
        "fixture must cover at least one continuity observation"
    );
    for observation in &page.continuity_observations {
        assert!(
            observation.local_terminal_continuity_preserved,
            "observation {} must preserve local terminal continuity",
            observation.observation_id
        );
        assert!(
            !observation.silent_authority_widening_taken,
            "observation {} must not silently widen authority",
            observation.observation_id
        );
        assert!(
            !observation.in_flight_input_replayed,
            "observation {} must not replay in-flight input",
            observation.observation_id
        );
    }
}

#[test]
fn support_export_omits_raw_and_in_flight_fields() {
    let page = load_page();
    let projection = page.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");
    assert_eq!(
        projection.record_kind,
        "shared_terminal_control_alpha_support_export"
    );
    assert!(!json.contains("raw_terminal_bytes"));
    assert!(!json.contains("raw_input_payload"));
    assert!(!json.contains("in_flight_input_replayed"));
    assert!(!json.contains("upstream_audit_event_ref"));
    assert_eq!(projection.state_summaries.len(), page.control_states.len());
    assert_eq!(
        projection.presenter_summaries.len(),
        page.presenter_handoffs.len()
    );
    assert_eq!(projection.audit_summaries.len(), page.audit_events.len());
    assert_eq!(
        projection.continuity_summaries.len(),
        page.continuity_observations.len()
    );
}

#[test]
fn editing_a_revoked_row_to_drop_revocation_ref_is_rejected_after_edit() {
    let mut page = load_page();
    let row = page
        .control_states
        .iter_mut()
        .find(|state| state.control_state == SharedTerminalControlStateClass::ControlRevoked)
        .expect("revoked state present");
    row.revocation_ref = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id
            == "shared_terminal_alpha.control_state_revocation_ref_missing"));
}

#[test]
fn audit_event_referencing_unknown_state_is_rejected_after_edit() {
    let mut page = load_page();
    let event = page
        .audit_events
        .iter_mut()
        .find(|event| event.event_class == SharedTerminalAuditEventClass::ControlActiveStarted)
        .expect("active-started audit present");
    event.control_state_ref =
        Some("shared_terminal_control_alpha.state.does_not_exist".to_string());
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "shared_terminal_alpha.audit_event_state_ref_unknown"));
}

#[test]
fn dropping_active_control_state_breaks_required_coverage_after_edit() {
    let mut page = load_page();
    page.control_states.retain(|state| {
        state.control_state != SharedTerminalControlStateClass::ActiveControlGrantee
    });
    page.audit_events
        .retain(|event| event.event_class != SharedTerminalAuditEventClass::ControlActiveStarted);
    page.presenter_handoffs.retain(|handoff| {
        handoff.destination_actor_ref.as_deref() != Some("actor.participant.driver.03")
    });
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "shared_terminal_alpha.coverage_control_state_missing"));
}
