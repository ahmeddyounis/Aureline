//! Fixture-driven coverage for the M3 scaffold-safety beta lane.
//!
//! The first test replays every scenario fixture under
//! `fixtures/workspace/m3/scaffold_preflight_and_generation/`, projects it,
//! and asserts the closed acceptance truth. The remaining tests prove the
//! distinct generation verbs are covered and that every boundary record
//! round-trips through the Rust descriptors.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_workspace::{
    ScaffoldPlanRecord, ScaffoldRunRecord, ScaffoldSafetyBetaInputs, ScaffoldSafetyBetaProjection,
    ScaffoldSurface, TemplateGeneratorDescriptor,
};

#[derive(Debug, Clone, Deserialize)]
struct ScenarioFixture {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    surface: String,
    descriptor: TemplateGeneratorDescriptor,
    plan: ScaffoldPlanRecord,
    #[serde(default)]
    run: Option<ScaffoldRunRecord>,
    expect: ExpectedProjection,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureMeta {
    name: String,
    scenario: String,
    doc_sections: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedProjection {
    provider_class: String,
    signature_state: String,
    generation_verb: String,
    egress_posture: String,
    declared_side_effect_classes: Vec<String>,
    create_empty_available: bool,
    set_up_later_available: bool,
    rollback_boundary: String,
    rollback_automatic: bool,
    has_run: bool,
    #[serde(default)]
    run_outcome: Option<String>,
    honesty_labels: Vec<String>,
    guardrails_all_hold: bool,
    surface_must_disclose: bool,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m3/scaffold_preflight_and_generation")
}

fn surface_from_token(token: &str) -> ScaffoldSurface {
    match token {
        "start_center" => ScaffoldSurface::StartCenter,
        "command_palette" => ScaffoldSurface::CommandPalette,
        "generator_preview" => ScaffoldSurface::GeneratorPreview,
        "ai_assist" => ScaffoldSurface::AiAssist,
        "extension" => ScaffoldSurface::Extension,
        "cli_headless" => ScaffoldSurface::CliHeadless,
        "support" => ScaffoldSurface::Support,
        other => panic!("unknown surface token {other}"),
    }
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}

fn load_fixtures() -> Vec<(PathBuf, ScenarioFixture)> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("scaffold-safety beta fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let parsed: ScenarioFixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
            (path, parsed)
        })
        .collect()
}

#[test]
fn every_scenario_projects_to_expected_beta_truth() {
    let fixtures = load_fixtures();
    assert!(
        fixtures.len() >= 9,
        "scaffold-safety beta fixture suite must keep at least 9 scenarios (found {})",
        fixtures.len()
    );

    for (path, fixture) in &fixtures {
        let surface = surface_from_token(&fixture.surface);
        let projection = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &fixture.descriptor,
            plan: &fixture.plan,
            run: fixture.run.as_ref(),
            surface,
        })
        .unwrap_or_else(|err| panic!("fixture {} failed to project: {err}", fixture.fixture.name));

        assert!(
            !fixture.fixture.scenario.is_empty(),
            "{path:?}: scenario must not be empty"
        );
        assert!(
            !fixture.fixture.doc_sections.is_empty(),
            "{path:?}: doc_sections must not be empty"
        );

        let name = &fixture.fixture.name;
        let e = &fixture.expect;

        assert_eq!(
            projection.provider_class.as_str(),
            e.provider_class,
            "{name}: provider_class"
        );
        assert_eq!(
            projection.signature_state.as_str(),
            e.signature_state,
            "{name}: signature_state"
        );
        assert_eq!(
            projection.generation_verb.as_str(),
            e.generation_verb,
            "{name}: generation_verb"
        );
        assert_eq!(
            projection.egress_posture.as_str(),
            e.egress_posture,
            "{name}: egress_posture"
        );

        let observed_side_effects: Vec<String> = sorted(
            projection
                .declared_side_effects
                .classes
                .iter()
                .map(|c| c.as_str().to_string())
                .collect(),
        );
        assert_eq!(
            observed_side_effects,
            sorted(e.declared_side_effect_classes.clone()),
            "{name}: declared_side_effect_classes"
        );

        assert_eq!(
            projection.setup_handoff.create_empty_available, e.create_empty_available,
            "{name}: create_empty_available"
        );
        assert_eq!(
            projection.setup_handoff.set_up_later_available, e.set_up_later_available,
            "{name}: set_up_later_available"
        );
        assert_eq!(
            projection.setup_handoff.rollback_boundary.as_str(),
            e.rollback_boundary,
            "{name}: rollback_boundary"
        );
        assert_eq!(
            projection.setup_handoff.rollback_automatic, e.rollback_automatic,
            "{name}: rollback_automatic"
        );

        assert_eq!(
            projection.run_summary.is_some(),
            e.has_run,
            "{name}: has_run"
        );
        let observed_outcome = projection
            .run_summary
            .as_ref()
            .map(|r| r.outcome_class.as_str().to_string());
        assert_eq!(observed_outcome, e.run_outcome, "{name}: run_outcome");

        let observed_labels: Vec<String> = sorted(
            projection
                .honesty_labels
                .iter()
                .map(|l| l.as_str().to_string())
                .collect(),
        );
        assert_eq!(
            observed_labels,
            sorted(e.honesty_labels.clone()),
            "{name}: honesty_labels"
        );

        assert_eq!(
            projection.guardrails.all_hold(),
            e.guardrails_all_hold,
            "{name}: guardrails_all_hold"
        );
        assert_eq!(
            projection.surface_must_disclose_generation(),
            e.surface_must_disclose,
            "{name}: surface_must_disclose"
        );

        // The projection always binds descriptor and plan refs.
        assert_eq!(
            projection.descriptor_ref, fixture.descriptor.descriptor_id,
            "{name}: descriptor ref"
        );
        assert_eq!(
            projection.scaffold_plan_ref, fixture.plan.scaffold_plan_id,
            "{name}: plan ref"
        );

        // A claimed beta plan never writes before review, and a present run
        // keeps generated output plain workspace content.
        assert!(
            projection.guardrails.no_writes_before_review,
            "{name}: every fixture plan guards no-writes-before-review"
        );
        if let Some(run) = projection.run_summary.as_ref() {
            assert!(
                run.plain_file_authority,
                "{name}: run keeps plain-file authority"
            );
            assert!(
                run.no_hidden_project_database,
                "{name}: run keeps no hidden project database"
            );
            assert!(
                !run.generated_lineage_ref.is_empty(),
                "{name}: run binds a lineage ref"
            );
        }

        // The projection round-trips through its serialized shape.
        let round_trip = serde_json::to_value(&projection)
            .and_then(serde_json::from_value::<ScaffoldSafetyBetaProjection>)
            .expect("projection must round-trip");
        assert_eq!(round_trip, projection, "{name}: projection round-trip");

        // Each input record round-trips at the struct level.
        let rt_descriptor = serde_json::to_value(&fixture.descriptor)
            .and_then(serde_json::from_value::<TemplateGeneratorDescriptor>)
            .expect("descriptor must round-trip");
        assert_eq!(
            rt_descriptor, fixture.descriptor,
            "{name}: descriptor round-trip"
        );
        let rt_plan = serde_json::to_value(&fixture.plan)
            .and_then(serde_json::from_value::<ScaffoldPlanRecord>)
            .expect("plan must round-trip");
        assert_eq!(rt_plan, fixture.plan, "{name}: plan round-trip");
        if let Some(run) = &fixture.run {
            let rt_run = serde_json::to_value(run)
                .and_then(serde_json::from_value::<ScaffoldRunRecord>)
                .expect("run must round-trip");
            assert_eq!(&rt_run, run, "{name}: run round-trip");
        }
    }
}

#[test]
fn distinct_generation_verbs_are_all_represented() {
    let fixtures = load_fixtures();
    let mut verbs: Vec<String> = fixtures
        .iter()
        .map(|(_, f)| f.expect.generation_verb.clone())
        .collect();
    verbs.sort();
    verbs.dedup();
    for required in [
        "create_project",
        "generate_into_existing",
        "update_regenerate",
    ] {
        assert!(
            verbs.iter().any(|v| v == required),
            "scaffold fixture suite must cover the distinct verb {required}; covered: {verbs:?}"
        );
    }
}

#[test]
fn ai_and_extension_generation_is_governed() {
    let fixtures = load_fixtures();
    let governed: Vec<&ScenarioFixture> = fixtures
        .iter()
        .map(|(_, f)| f)
        .filter(|f| {
            matches!(
                f.expect.provider_class.as_str(),
                "ai_assisted" | "extension_provided"
            )
        })
        .collect();
    assert!(
        governed.len() >= 2,
        "scaffold fixture suite must cover AI-assisted and extension-provided generation"
    );
    for fixture in governed {
        let projection = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
            descriptor: &fixture.descriptor,
            plan: &fixture.plan,
            run: fixture.run.as_ref(),
            surface: surface_from_token(&fixture.surface),
        })
        .expect("governed projection");
        assert!(
            projection.guardrails.ai_extension_uses_governed_surface,
            "{}: AI / extension generation reuses the governed surface",
            fixture.fixture.name
        );
        assert!(
            projection.guardrails.no_undeclared_hooks_or_bootstrap,
            "{}: AI / extension generation cannot invent undeclared hooks or bootstrap",
            fixture.fixture.name
        );
        assert!(
            projection.surface_must_disclose_generation(),
            "{}: AI / extension generation is always disclosed",
            fixture.fixture.name
        );
    }
}
