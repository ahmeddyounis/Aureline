//! Fixture-driven coverage for the efficiency-state surfaces.
//!
//! Each fixture under `fixtures/efficiency/states/` carries one canonical
//! [`EfficiencyStateSnapshot`] together with the diagnostics and support-export
//! projections that derive from it. This test round-trips every fixture back
//! through the typed surfaces, proving the checked-in projections never drift
//! from the code and that status, diagnostics, and support always agree on the
//! same object.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::efficiency::surfaces::{
    EfficiencyDiagnosticsProjection, EfficiencyStateSupportExport,
};
use aureline_shell::efficiency::EfficiencyStateSnapshot;

#[derive(Debug, Deserialize)]
struct SurfaceCase {
    workspace_id: String,
    snapshot: EfficiencyStateSnapshot,
    diagnostics: EfficiencyDiagnosticsProjection,
    support_export: EfficiencyStateSupportExport,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/efficiency/states")
}

#[test]
fn efficiency_state_surface_fixtures_agree_and_do_not_drift() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("efficiency states fixtures dir exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "efficiency state fixtures must exist");

    for path in fixtures {
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let case: SurfaceCase = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));

        assert_eq!(case.snapshot.workspace_id, case.workspace_id);

        // The stored projections must equal re-deriving them from the snapshot:
        // the diagnostics and support surfaces consume the same object.
        let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(&case.snapshot);
        let support_export = EfficiencyStateSupportExport::from_snapshot(&case.snapshot);
        assert_eq!(
            diagnostics, case.diagnostics,
            "diagnostics projection drifted in {path:?}"
        );
        assert_eq!(
            support_export, case.support_export,
            "support export drifted in {path:?}"
        );

        // What changed, why, and which subsystems agree across all surfaces.
        assert_eq!(diagnostics.active_state, case.snapshot.active_state);
        assert_eq!(support_export.active_state, case.snapshot.active_state);
        assert_eq!(diagnostics.source_of_change, case.snapshot.pressure_sources);
        assert_eq!(
            support_export.source_of_change,
            case.snapshot.pressure_sources
        );
        assert_eq!(
            diagnostics.affected_subsystem_count,
            case.snapshot.affected_subsystems.len()
        );
        assert_eq!(
            support_export.rows.len(),
            case.snapshot.affected_subsystems.len()
        );

        if let Some(status) = &case.snapshot.status {
            assert_eq!(status.active_state, case.snapshot.active_state);
            assert_eq!(status.pressure_sources, case.snapshot.pressure_sources);
        }

        // Support tooling can reconstruct the posture without logs or UI text.
        assert!(
            support_export.redaction_safe(),
            "export must be redaction-safe in {path:?}"
        );
        assert!(
            support_export.reconstructs_posture_without_logs(),
            "export must reconstruct posture without logs in {path:?}"
        );

        // The diagnostics row points operators at the matching support export.
        assert_eq!(diagnostics.support_export_ref, support_export.export_id);

        // Protected paths and durability are never narrowed away.
        assert!(case.snapshot.preserves_durability_truth());
        assert!(case.snapshot.hidden_pane_audit.passes_hidden_pane_policy);
    }
}
