//! Fixture-driven coverage for the energy/thermal efficiency lab.
//!
//! Each fixture under `fixtures/efficiency/lab/` carries one claimed M5 desktop
//! profile together with the lab trace, Project Doctor report, and support export
//! the canonical efficiency-state runtime derives from it. This test re-runs every
//! profile through the runtime, proving the checked-in fixtures never drift from
//! the code, and asserts the promotion gates, content-free guarantees, and
//! Doctor/support parity the low-power claim depends on.

use std::path::Path;

use aureline_shell::efficiency::energy_lab::{
    run_lab_case, EfficiencyLabCase, EFFICIENCY_DOCTOR_PROBE_ID,
};
use aureline_shell::efficiency::governance::OverridePosture;
use aureline_shell::efficiency::EfficiencyState;

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/efficiency/lab")
}

fn lab_fixtures() -> Vec<std::path::PathBuf> {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("efficiency lab fixtures dir exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    fixtures
}

#[test]
fn lab_fixtures_agree_and_do_not_drift() {
    let fixtures = lab_fixtures();
    assert!(!fixtures.is_empty(), "efficiency lab fixtures must exist");

    for path in fixtures {
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let case: EfficiencyLabCase = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));

        // Re-running the profile through the runtime reproduces the stored case.
        let rebuilt = run_lab_case(case.profile.clone());
        assert_eq!(rebuilt, case, "lab case drifted in {path:?}");

        // Every promotion gate holds for the seeded profiles.
        assert!(
            case.trace.promotion_gates_pass(),
            "profile {} failed a promotion gate ({path:?})",
            case.profile.profile_id
        );
        assert!(case.trace.protected_paths_held);
        assert!(case.trace.hidden_panes_passed);
        assert!(case.trace.every_slowdown_explained);
        assert!(case.trace.trace_is_content_free);

        // One step per injection; one transition per step.
        assert_eq!(case.trace.steps.len(), case.profile.injections.len());
        assert_eq!(case.trace.transitions.len(), case.profile.injections.len());

        // Every reduced surface carries a content-free reason.
        for step in &case.trace.steps {
            assert!(
                step.protected_paths_held(),
                "step {} narrowed a protected path ({path:?})",
                step.step_index
            );
            assert!(step.every_slowdown_explained());
            for reason in &step.slowdown_explanations {
                assert!(reason.content_free);
                assert!(!reason.names_user_content);
                assert!(!reason.why_label.is_empty());
                assert!(!reason.what_stays_correct.is_empty());
            }
        }

        // The Doctor report names all four contract fields and resolves them.
        let doctor = &case.doctor_report;
        assert_eq!(doctor.probe_id, EFFICIENCY_DOCTOR_PROBE_ID);
        assert_eq!(doctor.current_state, case.trace.final_state);
        assert!(doctor.names_state_transitions_subsystems_and_override());
        assert!(EfficiencyState::from_token(&doctor.current_state).is_some());
        assert!(OverridePosture::from_token(&doctor.override_posture).is_some());
        assert!(doctor.durability_preserved);
        assert!(doctor.hidden_pane_passes_policy);

        // The support export is metadata-only and reconstructs without logs.
        let support = &case.support_export;
        assert!(support.redaction_safe());
        assert!(support.reconstructs_posture_without_logs());
        assert!(!support.raw_provider_payloads_exported);
        assert!(!support.raw_secret_values_exported);
        assert!(!support.names_user_content);
        assert!(!support.ui_text_scrape_required);

        // Doctor and support agree with each other and with the trace.
        assert_eq!(doctor.trace_ref, case.trace.trace_id);
        assert_eq!(support.trace_ref, case.trace.trace_id);
        assert_eq!(doctor.support_export_ref, support.export_id);
        assert_eq!(doctor.override_posture, support.override_posture);
        assert_eq!(doctor.current_state, support.current_state);
        assert_eq!(doctor.recent_transitions, support.recent_transitions);
    }
}
