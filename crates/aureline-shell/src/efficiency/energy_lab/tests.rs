//! Unit coverage for the energy/thermal lab traces, Doctor reports, and exports.

use super::*;

#[test]
fn seeded_lab_cases_cover_every_profile_class() {
    let cases = seeded_lab_cases();
    assert_eq!(cases.len(), seed_lab_profiles().len());
    for class in LabProfileClass::ALL {
        assert!(
            cases
                .iter()
                .any(|case| case.profile.profile_class == class.as_str()),
            "missing lab profile for class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_seeded_trace_passes_its_promotion_gates() {
    for case in seeded_lab_cases() {
        assert!(
            case.trace.promotion_gates_pass(),
            "profile {} failed a promotion gate",
            case.profile.profile_id
        );
        assert!(case.trace.protected_paths_held);
        assert!(case.trace.hidden_panes_passed);
        assert!(case.trace.every_slowdown_explained);
        assert!(case.trace.trace_is_content_free);
    }
}

#[test]
fn trace_records_one_step_per_injection_and_a_transition_per_step() {
    for case in seeded_lab_cases() {
        assert_eq!(case.trace.steps.len(), case.profile.injections.len());
        assert_eq!(case.trace.transitions.len(), case.profile.injections.len());
        for (index, step) in case.trace.steps.iter().enumerate() {
            assert_eq!(step.step_index, index);
            // The recorded transition lands at the step's active state.
            assert_eq!(step.transition.new_state, step.active_state);
        }
    }
}

#[test]
fn pressured_steps_explain_every_throttled_subsystem_content_free() {
    let case = run_lab_case(
        seed_lab_profiles()
            .into_iter()
            .find(|profile| profile.profile_id == "thermal-workstation")
            .expect("thermal profile seeded"),
    );
    let pressured = case
        .trace
        .steps
        .iter()
        .find(|step| step.active_state == EfficiencyState::ThermalConstrained.as_str())
        .expect("thermal step present");
    assert!(pressured.behavior_changed);
    assert!(!pressured.throttled_subsystems.is_empty());
    assert!(pressured.every_slowdown_explained());
    for reason in &pressured.slowdown_explanations {
        assert!(reason.content_free);
        assert!(!reason.names_user_content);
        assert!(!reason.why_label.is_empty());
        assert!(!reason.what_stays_correct.is_empty());
    }
}

#[test]
fn doctor_report_names_state_transitions_subsystems_and_override() {
    for case in seeded_lab_cases() {
        let report = &case.doctor_report;
        assert_eq!(report.probe_id, EFFICIENCY_DOCTOR_PROBE_ID);
        assert_eq!(report.current_state, case.trace.final_state);
        assert!(report.names_state_transitions_subsystems_and_override());
        // The Doctor report points operators at the full state surface.
        assert_eq!(report.primary_command_id, EFFICIENCY_INSPECT_COMMAND_ID);
        assert_eq!(report.opens_surface_ref, EFFICIENCY_DETAILS_SURFACE_REF);
        assert!(report.durability_preserved);
    }
}

#[test]
fn critical_battery_profile_reports_protect_core_degraded() {
    let case = run_lab_case(
        seed_lab_profiles()
            .into_iter()
            .find(|profile| profile.profile_id == "critical-battery-field")
            .expect("critical-battery profile seeded"),
    );
    // The run ends in recovery, but the protect-core transition is in the history.
    assert!(case
        .doctor_report
        .recent_transitions
        .iter()
        .any(|transition| transition.new_state == EfficiencyState::ProtectCore.as_str()));
    // Protect-core is not overridable while it is active.
    let protect_core = case
        .trace
        .steps
        .iter()
        .find(|step| step.active_state == EfficiencyState::ProtectCore.as_str())
        .expect("protect-core step present");
    assert_eq!(protect_core.override_posture, "not_overridable");
}

#[test]
fn policy_capped_profile_reports_policy_blocked_override() {
    let case = run_lab_case(
        seed_lab_profiles()
            .into_iter()
            .find(|profile| profile.profile_id == "policy-managed-fleet")
            .expect("policy profile seeded"),
    );
    assert_eq!(case.doctor_report.override_posture, "policy_blocked");
    assert_eq!(case.support_export.override_posture, "policy_blocked");
}

#[test]
fn support_export_is_redaction_safe_and_reconstructs_without_logs() {
    for case in seeded_lab_cases() {
        let export = &case.support_export;
        assert!(export.redaction_safe());
        assert!(export.reconstructs_posture_without_logs());
        assert!(!export.raw_provider_payloads_exported);
        assert!(!export.raw_secret_values_exported);
        assert!(!export.names_user_content);
        // The export quotes the same transition history the Doctor report does.
        assert_eq!(
            export.recent_transitions,
            case.doctor_report.recent_transitions
        );
    }
}

#[test]
fn doctor_and_support_agree_with_the_trace_they_derive_from() {
    for case in seeded_lab_cases() {
        assert_eq!(case.doctor_report.trace_ref, case.trace.trace_id);
        assert_eq!(case.support_export.trace_ref, case.trace.trace_id);
        assert_eq!(
            case.doctor_report.support_export_ref,
            case.support_export.export_id
        );
        assert_eq!(
            case.doctor_report.override_posture,
            case.support_export.override_posture
        );
        assert_eq!(
            case.doctor_report.current_state,
            case.support_export.current_state
        );
    }
}

#[test]
fn trace_round_trips_through_serde() {
    for case in seeded_lab_cases() {
        let json = serde_json::to_string(&case).expect("case serializes");
        let restored: EfficiencyLabCase = serde_json::from_str(&json).expect("case deserializes");
        assert_eq!(restored, case);
    }
}
