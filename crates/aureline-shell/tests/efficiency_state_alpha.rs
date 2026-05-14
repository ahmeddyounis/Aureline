//! Fixture-driven coverage for efficiency-state shell hooks.
//!
//! The fixtures under `fixtures/performance/efficiency_state_alpha/` exercise
//! one pressure drill and one hidden-pane render audit through the same
//! runtime records the shell status bar, durable activity center, and benchmark
//! packet consume.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::activity_center::alpha::ActivityCenterAlphaRuntime;
use aureline_shell::efficiency::{
    EfficiencyPressureSource, EfficiencyState, EfficiencyStateRuntime, EfficiencyStateSnapshot,
    HiddenPaneRenderAudit, ProtectedSurfaceClass, RenderVisibilityInput, VisibilityState,
    WorkloadAction, WorkloadFamily,
};

#[derive(Debug, Deserialize)]
struct EfficiencyFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    workspace_id: String,
    observed_at: String,
    transition_reason: String,
    active_state: EfficiencyState,
    pressure_sources: Vec<EfficiencyPressureSource>,
    workloads: Vec<FixtureWorkload>,
    render_surfaces: Vec<FixtureSurface>,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureWorkload {
    workload: WorkloadFamily,
    expected_action: WorkloadAction,
}

#[derive(Debug, Deserialize)]
struct FixtureSurface {
    surface_id: String,
    surface_class: ProtectedSurfaceClass,
    visibility_state: VisibilityState,
    requested_paint_count: u32,
    requested_animation_tick_count: u32,
    correctness_polling_required: bool,
}

#[derive(Debug, Deserialize)]
struct FixtureExpect {
    status_visible: bool,
    affected_capability_count: usize,
    hidden_pane_render_violation_count: u32,
    protected_interactions: Vec<String>,
    capability_ids: Vec<String>,
    activity_row_for_indexing: bool,
    durability_preserved: bool,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/performance/efficiency_state_alpha")
}

#[test]
fn efficiency_state_alpha_fixtures_project_status_activity_and_render_truth() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("fixtures dir exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with("_case.json"))
        })
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "efficiency fixtures must exist");

    for path in fixtures {
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: EfficiencyFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "efficiency_state_alpha_case");
        assert_eq!(fixture.schema_version, 1);
        assert!(
            !fixture.case_name.trim().is_empty(),
            "fixture case name must be present in {path:?}"
        );

        let source = *fixture
            .pressure_sources
            .first()
            .expect("fixture must include a pressure source");
        let mut runtime = EfficiencyStateRuntime::new();
        runtime.transition_to(
            fixture.active_state,
            source,
            fixture.transition_reason.clone(),
            fixture.observed_at.clone(),
        );

        let decisions = fixture
            .workloads
            .iter()
            .map(|workload| {
                let decision =
                    runtime.decide_workload(workload.workload, source, fixture.observed_at.clone());
                assert_eq!(
                    decision.action,
                    workload.expected_action.as_str(),
                    "workload action mismatch for {:?} in {:?}",
                    workload.workload,
                    path
                );
                decision
            })
            .collect::<Vec<_>>();

        let render_decisions = fixture
            .render_surfaces
            .iter()
            .map(|surface| {
                runtime.decide_render(RenderVisibilityInput {
                    surface_id: surface.surface_id.clone(),
                    surface_class: surface.surface_class,
                    visibility_state: surface.visibility_state,
                    requested_paint_count: surface.requested_paint_count,
                    requested_animation_tick_count: surface.requested_animation_tick_count,
                    correctness_polling_required: surface.correctness_polling_required,
                })
            })
            .collect::<Vec<_>>();
        let audit = HiddenPaneRenderAudit::from_decisions(&render_decisions);
        assert_eq!(
            audit.hidden_pane_render_violation_count,
            fixture.expect.hidden_pane_render_violation_count,
            "hidden-pane violations mismatch in {path:?}"
        );
        assert!(audit.passes_hidden_pane_policy);

        let snapshot = EfficiencyStateSnapshot::from_decisions(
            fixture.workspace_id.clone(),
            fixture.active_state,
            fixture.pressure_sources.clone(),
            true,
            decisions.clone(),
            audit,
            fixture.observed_at.clone(),
        );
        assert_eq!(snapshot.status.is_some(), fixture.expect.status_visible);
        assert_eq!(
            snapshot.throttled_capabilities.len(),
            fixture.expect.affected_capability_count
        );
        assert_eq!(
            snapshot.protected_interactions_preserved,
            fixture.expect.protected_interactions
        );
        assert_eq!(
            snapshot
                .throttled_capabilities
                .iter()
                .map(|row| row.capability_id.clone())
                .collect::<Vec<_>>(),
            fixture.expect.capability_ids
        );
        assert_eq!(
            snapshot.preserves_durability_truth(),
            fixture.expect.durability_preserved
        );

        if fixture.expect.activity_row_for_indexing {
            let indexing_decision = decisions
                .iter()
                .find(|decision| decision.workload_id == WorkloadFamily::IndexingRefresh.as_str())
                .expect("fixture expected an indexing decision");
            let row = indexing_decision
                .indexing_activity_row(&fixture.workspace_id, &fixture.observed_at)
                .expect("indexing decision maps to activity row");
            let mut activity = ActivityCenterAlphaRuntime::in_memory();
            activity.record_row(row).expect("record activity row");
            let activity_snapshot = activity.snapshot();
            assert_eq!(activity_snapshot.len(), 1);
            let activity_row = &activity_snapshot.rows[0];
            assert!(activity_row.summary_label.contains("Indexing refresh"));
            assert!(activity_row.has_exact_reopen_identity());
            assert!(activity_row.is_support_exportable());
        }
    }
}
