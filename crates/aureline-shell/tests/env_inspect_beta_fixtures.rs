//! Shell-side parity coverage for the env-inspect contract.
//!
//! The runtime integration test in
//! `/crates/aureline-runtime/tests/env_inspect_beta.rs` pins the canonical
//! snapshot shape per seeded scenario. This shell-side test proves the
//! chrome panel projection consumes that snapshot verbatim: same row
//! sequence, same labels, same value tokens, same degradation severities.
//! Together the two tests close the UI / CLI / support-export parity loop
//! the env-inspect acceptance bullets require.

use aureline_runtime::{
    seeded_env_inspect_snapshot, EnvInspectDegradationSeverity, EnvInspectSeededScenario,
};
use aureline_shell::env_inspect::EnvInspectPanelProjection;

#[test]
fn panel_projection_matches_canonical_snapshot_for_every_scenario() {
    for scenario in EnvInspectSeededScenario::ALL {
        let snapshot = seeded_env_inspect_snapshot(scenario);
        let panel = EnvInspectPanelProjection::from_snapshot(snapshot.clone());

        let panel_paths: Vec<&str> = panel.rows().map(|row| row.field_path.as_str()).collect();
        let snapshot_paths: Vec<&str> = snapshot
            .core_fields
            .iter()
            .map(|field| field.field_path.as_str())
            .collect();
        assert_eq!(
            panel_paths, snapshot_paths,
            "{}: panel field-path sequence must match snapshot",
            scenario.as_str()
        );

        let panel_labels: Vec<&str> = panel.rows().map(|row| row.label.as_str()).collect();
        let snapshot_labels: Vec<&str> = snapshot
            .core_fields
            .iter()
            .map(|field| field.label.as_str())
            .collect();
        assert_eq!(
            panel_labels, snapshot_labels,
            "{}: panel labels must match snapshot",
            scenario.as_str()
        );

        let panel_values: Vec<Option<&str>> = panel
            .rows()
            .map(|row| row.value_token.as_deref())
            .collect();
        let snapshot_values: Vec<Option<&str>> = snapshot
            .core_fields
            .iter()
            .map(|field| field.value_token.as_deref())
            .collect();
        assert_eq!(
            panel_values, snapshot_values,
            "{}: panel values must match snapshot",
            scenario.as_str()
        );

        let panel_severities: Vec<EnvInspectDegradationSeverity> = panel
            .degradation_banners
            .iter()
            .map(|banner| banner.severity)
            .collect();
        let snapshot_severities: Vec<EnvInspectDegradationSeverity> = snapshot
            .degradation_labels
            .iter()
            .map(|label| label.severity)
            .collect();
        assert_eq!(
            panel_severities, snapshot_severities,
            "{}: panel degradation severities must match snapshot",
            scenario.as_str()
        );

        assert_eq!(
            panel.requires_review_before_dispatch,
            snapshot.requires_review_before_dispatch(),
            "{}: review posture must match",
            scenario.as_str()
        );
        assert_eq!(
            panel.blocks_dispatch,
            snapshot.blocks_dispatch(),
            "{}: blocking posture must match",
            scenario.as_str()
        );
    }
}

#[test]
fn panel_projection_preserves_boundary_cue_posture() {
    // Every non-local scenario MUST surface the boundary cue; the local
    // scenario MUST NOT. Boundary cue truth lives on the canonical
    // execution context, never re-derived by chrome.
    for scenario in EnvInspectSeededScenario::ALL {
        let snapshot = seeded_env_inspect_snapshot(scenario);
        let panel = EnvInspectPanelProjection::from_snapshot(snapshot.clone());
        assert_eq!(
            panel.boundary_cue_visible, snapshot.boundary_cue_visible,
            "{}: boundary cue must match",
            scenario.as_str()
        );
        let expects_cue = !matches!(scenario, EnvInspectSeededScenario::LocalTerminal);
        assert_eq!(
            panel.boundary_cue_visible, expects_cue,
            "{}: boundary cue posture mismatch",
            scenario.as_str()
        );
    }
}
