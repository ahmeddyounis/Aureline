//! Fixture-driven coverage for the hidden-surface render-suppression policy.
//!
//! Each fixture under `fixtures/efficiency/hidden-pane-audits/` carries one
//! scenario: the surfaces that requested work together with the suppression
//! audit, energy/thermal trace, and diagnostics projection the policy derives
//! from them. This test re-derives every projection from the stored surfaces,
//! proving the checked-in fixtures never drift from the code and that hidden or
//! off-screen surfaces shed work while resume stays correct.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::efficiency::hidden_surfaces::{
    HiddenSurfaceDiagnosticsProjection, HiddenSurfaceEnergyTrace, HiddenSurfaceInput,
    HiddenSurfaceSuppressionAudit, HiddenWorkChannel,
};
use aureline_shell::efficiency::EfficiencyState;

#[derive(Debug, Deserialize)]
struct HiddenSurfaceCaseFixture {
    scenario_id: String,
    efficiency_state: EfficiencyState,
    observed_at: String,
    window_label: String,
    surfaces: Vec<HiddenSurfaceInput>,
    audit: HiddenSurfaceSuppressionAudit,
    energy_trace: HiddenSurfaceEnergyTrace,
    diagnostics: HiddenSurfaceDiagnosticsProjection,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/efficiency/hidden-pane-audits")
}

#[test]
fn hidden_surface_audit_fixtures_agree_and_do_not_drift() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("hidden-pane audits fixtures dir exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "hidden-surface fixtures must exist");

    for path in fixtures {
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let case: HiddenSurfaceCaseFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));

        // Re-deriving from the surfaces reproduces the stored projections.
        let audit = HiddenSurfaceSuppressionAudit::for_surfaces(
            case.efficiency_state,
            &case.surfaces,
            &case.observed_at,
        );
        assert_eq!(audit, case.audit, "audit drifted in {path:?}");

        let trace = HiddenSurfaceEnergyTrace::from_audit(&audit, &case.window_label);
        assert_eq!(trace, case.energy_trace, "energy trace drifted in {path:?}");

        let diagnostics = HiddenSurfaceDiagnosticsProjection::from_audit(&audit);
        assert_eq!(
            diagnostics, case.diagnostics,
            "diagnostics drifted in {path:?}"
        );

        // Core invariants hold for every scenario.
        assert!(
            audit.passes_policy,
            "scenario {} kept hidden work alive ({path:?})",
            case.scenario_id
        );
        assert_eq!(audit.hidden_pane_violation_count, 0);
        assert!(
            audit.all_resumes_correct,
            "scenario {} broke resume continuity ({path:?})",
            case.scenario_id
        );
        assert!(audit.preserves_durability_truth());

        // No hidden surface committed decorative or speculative work, and every
        // hidden surface that owns a correctness channel keeps it above zero.
        for decision in &audit.decisions {
            if !decision.hidden {
                continue;
            }
            for channel in &decision.channels {
                if channel.correctness_critical {
                    if channel.requested_units > 0 {
                        assert!(
                            channel.committed_units >= 1,
                            "hidden correctness channel dropped to zero in {path:?}"
                        );
                    }
                } else {
                    assert_eq!(
                        channel.committed_units, 0,
                        "hidden surface kept {} alive in {path:?}",
                        channel.channel
                    );
                }
            }
            assert!(decision.resume.restores_without_rerun);
            assert!(decision.resume.restores_without_cache_corruption);
        }

        // The energy/thermal trace attributes every saved unit to a class.
        let attributed: u32 = audit
            .saved_by_class
            .iter()
            .map(|saving| saving.saved_units_total)
            .sum();
        assert_eq!(attributed, audit.total_saved_units);
        assert_eq!(trace.total_saved_units, audit.total_saved_units);

        // The coarse hidden-pane render policy agrees with the per-class audit.
        assert!(
            audit
                .as_hidden_pane_render_audit()
                .passes_hidden_pane_policy
        );

        // Paint is always suppressed for hidden surfaces.
        for decision in audit.decisions.iter().filter(|d| d.hidden) {
            let paint = decision
                .channels
                .iter()
                .find(|c| c.channel == HiddenWorkChannel::Paint.as_str())
                .expect("paint channel present");
            assert_eq!(paint.committed_units, 0);
        }
    }
}
