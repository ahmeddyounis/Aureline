//! Tests proving active sessions stay correct and attributable under pressure,
//! that optional work sheds first, and that any material downgrade is warned about
//! before it applies — never silently.

use super::*;
use crate::efficiency::governance::M5_EFFICIENCY_GOVERNANCE_MATRIX_REF;
use crate::efficiency::surfaces::seeded_efficiency_state_snapshots;

#[test]
fn nominal_posture_preserves_every_session_and_sheds_nothing() {
    let posture = SessionPressurePosture::for_state(
        "ws:nominal",
        EfficiencyState::Nominal,
        &[EfficiencyPressureSource::AcPower],
        0,
        "2026-06-20T14:00:00Z",
    );
    assert!(!posture.behavior_changed);
    assert_eq!(posture.sessions.len(), ActiveSessionKind::ALL.len());
    for session in &posture.sessions {
        assert_eq!(session.continuity_action, "preserve_active");
        assert!(!session.behavior_changed);
        assert!(session.warning.is_none());
        // No assist sheds under nominal.
        assert!(session.assists.iter().all(|assist| !assist.shed));
    }
    assert!(posture.preserves_active_session_correctness());
}

#[test]
fn every_pressured_posture_keeps_active_runs_correct_and_attributable() {
    for case in seeded_session_pressure_cases() {
        let posture = &case.posture;
        assert!(
            posture.preserves_active_session_correctness(),
            "{} regressed active-session correctness",
            case.case_id
        );
        assert!(!posture.any_session_silently_killed);
        assert!(!posture.any_session_replayed);
        for session in &posture.sessions {
            assert!(session.correctness_preserved);
            assert!(session.user_authority_preserved);
            assert!(session.attributable);
            assert!(session.never_silently_killed);
            assert!(session.never_replayed);
            assert!(!session.protected_authority.is_empty());
        }
        // Durability and protected interactions are never narrowed away.
        for protected in ["typing", "save", "local_navigation"] {
            assert!(posture
                .protected_interactions_preserved
                .contains(&protected.to_owned()));
        }
        assert!(posture.durability_preserved);
    }
}

#[test]
fn optional_assists_shed_before_active_behavior_regresses() {
    for case in seeded_session_pressure_cases() {
        let posture = &case.posture;
        assert!(
            posture.optional_work_sheds_first(),
            "{} did not shed optional work first",
            case.case_id
        );
        // Every session changed behavior under a pressured posture, and the change
        // came from shedding optional assists, not from regressing the run.
        assert!(posture.behavior_changed, "{} changed nothing", case.case_id);
        for session in &posture.sessions {
            assert!(session.behavior_changed);
            assert!(session.optional_assists_all_shed());
            for assist in &session.assists {
                assert!(assist.is_optional);
                assert!(assist.shed);
            }
            // The run itself stays correct regardless of how much optional work shed.
            assert!(session.correctness_preserved);
        }
    }
}

#[test]
fn debug_task_and_kernel_authority_is_never_downgraded() {
    // Even under critical-battery protect-core, debug authority, an active task's
    // completion, and a kernel's in-memory state are protected paths: they only
    // shed optional assists and never reach a material downgrade.
    let critical = seeded_session_pressure_cases()
        .into_iter()
        .find(|case| case.case_id == "critical-battery")
        .expect("critical-battery case exists");
    for token in ["debug_session", "active_task_run", "notebook_kernel"] {
        let session = critical
            .posture
            .session_for(token)
            .unwrap_or_else(|| panic!("session {token} present"));
        assert_eq!(
            session.continuity_action, "shed_optional_assists",
            "{token} was downgraded under protect-core"
        );
        assert!(session.warning.is_none(), "{token} proposed a downgrade");
    }
}

#[test]
fn material_downgrade_is_always_warned_before_it_applies() {
    for case in seeded_session_pressure_cases() {
        assert!(
            case.posture.warns_before_material_downgrade(),
            "{} downgraded without a scope-accurate warning",
            case.case_id
        );
        for session in &case.posture.sessions {
            let Some(warning) = &session.warning else {
                continue;
            };
            assert_eq!(session.continuity_action, "warn_before_downgrade");
            assert!(warning.shown_before_change);
            assert!(!warning.silent);
            assert!(warning.user_keeps_authority);
            assert!(!warning.what_changes.is_empty());
            assert!(!warning.what_stays_correct.is_empty());
            assert!(SessionDowngradeKind::from_token(&warning.downgrade_kind).is_some());
            assert_eq!(warning.inspect_command_id, EFFICIENCY_INSPECT_COMMAND_ID);
            // The warning always offers an inspect path; an override is offered only
            // where it is set.
            assert!(warning
                .actions
                .iter()
                .any(|action| action == "Open efficiency details"));
        }
    }
}

#[test]
fn protect_core_warns_capture_and_remote_but_keeps_them_alive() {
    let critical = seeded_session_pressure_cases()
        .into_iter()
        .find(|case| case.case_id == "critical-battery")
        .expect("critical-battery case exists");
    // Critical battery is not user-overridable, so the warning is shown but the
    // reduction cannot be declined — yet the session is still never killed.
    for token in ["trace_capture", "long_running_capture", "remote_attach"] {
        let session = critical
            .posture
            .session_for(token)
            .unwrap_or_else(|| panic!("session {token} present"));
        let warning = session
            .warning
            .as_ref()
            .unwrap_or_else(|| panic!("{token} warns before downgrade"));
        assert_eq!(warning.override_posture, "not_overridable");
        assert!(!warning.override_allowed);
        assert!(session.never_silently_killed);
        assert!(session.never_replayed);
    }
}

#[test]
fn battery_saver_lets_the_user_keep_capture_full_fidelity() {
    // On battery the override is session-only, but a plain battery-saver posture
    // does not materially downgrade any live run, so no warning is emitted and the
    // sessions only shed optional assists.
    let battery = seeded_session_pressure_cases()
        .into_iter()
        .find(|case| case.case_id == "battery-saver")
        .expect("battery-saver case exists");
    assert_eq!(
        battery.posture.override_posture,
        "user_override_session_only"
    );
    for session in &battery.posture.sessions {
        assert_eq!(session.continuity_action, "shed_optional_assists");
        assert!(session.warning.is_none());
    }
}

#[test]
fn recovery_resumes_assists_in_stages_without_touching_the_run() {
    let recovery = seeded_session_pressure_cases()
        .into_iter()
        .find(|case| case.case_id == "recovery")
        .expect("recovery case exists");
    assert_eq!(recovery.posture.recovery_state, "staged_resume");
    for session in &recovery.posture.sessions {
        assert_eq!(session.continuity_action, "staged_resume");
        assert!(session.warning.is_none());
        for assist in &session.assists {
            assert_eq!(assist.action, "staged_resume");
        }
        assert!(session.correctness_preserved);
    }
}

#[test]
fn posture_projects_from_the_canonical_snapshot() {
    // The posture re-derived from a snapshot equals the seeded case for the same
    // workspace, and shares the snapshot's state, cause, override posture, recovery
    // state, and governance binding so the surfaces never disagree.
    let cases = seeded_session_pressure_cases();
    for snapshot in seeded_efficiency_state_snapshots() {
        let case = cases
            .iter()
            .find(|case| case.workspace_id == snapshot.workspace_id)
            .unwrap_or_else(|| {
                panic!("session-pressure case for {} exists", snapshot.workspace_id)
            });
        let projected = SessionPressurePosture::from_snapshot(&snapshot);
        assert_eq!(
            projected, case.posture,
            "posture drifted from the snapshot for {}",
            snapshot.workspace_id
        );
        assert_eq!(projected.active_state, snapshot.active_state);
        assert_eq!(projected.source_of_change, snapshot.pressure_sources);
        assert_eq!(projected.override_posture, snapshot.override_posture);
        assert_eq!(projected.recovery_state, snapshot.recovery_state);
        assert_eq!(
            projected.governance.matrix_ref,
            M5_EFFICIENCY_GOVERNANCE_MATRIX_REF
        );
    }
}
