//! Fixture-driven coverage for the M3 repository-acquisition beta lane.
//!
//! The first test replays every scenario fixture under
//! `fixtures/workspace/m3/repository_acquisition_and_bootstrap/`, projects
//! it, and asserts the closed acceptance truth. The second test proves the
//! Rust descriptors faithfully parse and round-trip the frozen
//! `source_locator` / `checkout_plan` / `bootstrap_queue_item` fixtures
//! under `fixtures/workspace/bootstrap_cases/`.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_workspace::{
    AcquisitionSurface, AcquisitionVerb, BootstrapQueueItemRecord, CheckoutModeClass,
    CheckoutPlanRecord, CredentialPostureClass, ExpectedCostBand, LfsPolicyClass,
    RepositoryAcquisitionBetaInputs, RepositoryAcquisitionBetaProjection, SourceLocatorRecord,
    SubmodulePolicyClass,
};

#[derive(Debug, Clone, Deserialize)]
struct ScenarioFixture {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    surface: String,
    locator: SourceLocatorRecord,
    plan: CheckoutPlanRecord,
    #[serde(default)]
    bootstrap_items: Vec<BootstrapQueueItemRecord>,
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
    acquisition_verb: String,
    checkout_mode: String,
    partial_or_sparse: bool,
    submodule_policy: String,
    lfs_policy: String,
    expected_cost_band: String,
    credential_posture: String,
    credential_reauth_required: bool,
    interrupted: bool,
    #[serde(default)]
    interrupted_branches: Vec<String>,
    manual_followup_count: usize,
    honesty_labels: Vec<String>,
    guardrails_all_hold: bool,
    surface_must_disclose: bool,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m3/repository_acquisition_and_bootstrap")
}

fn surface_from_token(token: &str) -> AcquisitionSurface {
    match token {
        "start_center" => AcquisitionSurface::StartCenter,
        "command_palette" => AcquisitionSurface::CommandPalette,
        "deep_link" => AcquisitionSurface::DeepLink,
        "cli_headless" => AcquisitionSurface::CliHeadless,
        "policy_guided_deployment" => AcquisitionSurface::PolicyGuidedDeployment,
        "support" => AcquisitionSurface::Support,
        other => panic!("unknown surface token {other}"),
    }
}

fn verb_token(verb: AcquisitionVerb) -> &'static str {
    verb.as_str()
}

fn mode_token(mode: CheckoutModeClass) -> &'static str {
    mode.as_str()
}

fn submodule_token(policy: SubmodulePolicyClass) -> &'static str {
    policy.as_str()
}

fn lfs_token(policy: LfsPolicyClass) -> &'static str {
    policy.as_str()
}

fn cost_token(band: ExpectedCostBand) -> &'static str {
    band.as_str()
}

fn credential_token(posture: CredentialPostureClass) -> &'static str {
    posture.as_str()
}

fn load_fixtures() -> Vec<(PathBuf, ScenarioFixture)> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("repository_acquisition beta fixtures dir must exist")
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
        "acquisition beta fixture suite must keep at least 9 scenarios (found {})",
        fixtures.len()
    );

    for (path, fixture) in &fixtures {
        let surface = surface_from_token(&fixture.surface);
        let projection = RepositoryAcquisitionBetaProjection::project(
            RepositoryAcquisitionBetaInputs {
                locator: &fixture.locator,
                plan: &fixture.plan,
                bootstrap_items: &fixture.bootstrap_items,
                surface,
            },
        )
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

        assert_eq!(verb_token(projection.acquisition_verb), e.acquisition_verb, "{name}: acquisition_verb");
        assert_eq!(mode_token(projection.checkout_shape.mode), e.checkout_mode, "{name}: checkout_mode");
        assert_eq!(projection.checkout_shape.partial_or_sparse, e.partial_or_sparse, "{name}: partial_or_sparse");
        assert_eq!(submodule_token(projection.checkout_shape.submodule_policy), e.submodule_policy, "{name}: submodule_policy");
        assert_eq!(lfs_token(projection.checkout_shape.lfs_policy), e.lfs_policy, "{name}: lfs_policy");
        assert_eq!(cost_token(projection.expected_cost_band), e.expected_cost_band, "{name}: expected_cost_band");
        assert_eq!(credential_token(projection.credential_posture.posture_class), e.credential_posture, "{name}: credential_posture");
        assert_eq!(projection.credential_posture.reauth_required, e.credential_reauth_required, "{name}: credential_reauth_required");

        assert_eq!(projection.interrupted_recovery.is_some(), e.interrupted, "{name}: interrupted");
        let observed_branches: Vec<String> = projection
            .interrupted_recovery
            .as_ref()
            .map(|r| r.branches.iter().map(|b| b.as_str().to_string()).collect())
            .unwrap_or_default();
        assert_eq!(observed_branches, e.interrupted_branches, "{name}: interrupted_branches");

        assert_eq!(projection.manual_followups.len(), e.manual_followup_count, "{name}: manual_followup_count");

        let observed_labels: Vec<String> = projection
            .honesty_labels
            .iter()
            .map(|l| l.as_str().to_string())
            .collect();
        assert_eq!(observed_labels, e.honesty_labels, "{name}: honesty_labels");

        assert_eq!(projection.guardrails.all_hold(), e.guardrails_all_hold, "{name}: guardrails_all_hold");
        assert_eq!(projection.surface_must_disclose_acquisition(), e.surface_must_disclose, "{name}: surface_must_disclose");

        // The evidence packet always joins locator + plan and stays
        // export-safe, and every enqueued item is attributed.
        assert_eq!(projection.evidence_packet.source_locator_ref, fixture.locator.source_locator_id, "{name}: evidence locator ref");
        assert_eq!(projection.evidence_packet.checkout_plan_ref, fixture.plan.checkout_plan_id, "{name}: evidence plan ref");
        assert!(projection.evidence_packet.export_safe, "{name}: evidence export_safe");
        assert!(projection.evidence_packet.every_item_attributed, "{name}: every_item_attributed");

        // The projection round-trips through its serialized shape.
        let round_trip = serde_json::to_value(&projection)
            .and_then(serde_json::from_value::<RepositoryAcquisitionBetaProjection>)
            .expect("projection must round-trip");
        assert_eq!(round_trip, projection, "{name}: projection round-trip");

        // Each input record round-trips at the struct level.
        let rt_locator = serde_json::to_value(&fixture.locator)
            .and_then(serde_json::from_value::<SourceLocatorRecord>)
            .expect("locator must round-trip");
        assert_eq!(rt_locator, fixture.locator, "{name}: locator round-trip");
        let rt_plan = serde_json::to_value(&fixture.plan)
            .and_then(serde_json::from_value::<CheckoutPlanRecord>)
            .expect("plan must round-trip");
        assert_eq!(rt_plan, fixture.plan, "{name}: plan round-trip");
        for item in &fixture.bootstrap_items {
            let rt_item = serde_json::to_value(item)
                .and_then(serde_json::from_value::<BootstrapQueueItemRecord>)
                .expect("bootstrap item must round-trip");
            assert_eq!(&rt_item, item, "{name}: bootstrap item round-trip");
            assert!(item.is_well_formed_blocked_item(), "{name}: bootstrap item blocker/repair contract");
        }
    }
}

#[test]
fn distinct_verbs_are_all_represented() {
    let fixtures = load_fixtures();
    let mut verbs: Vec<String> = fixtures
        .iter()
        .map(|(_, f)| f.expect.acquisition_verb.clone())
        .collect();
    verbs.sort();
    verbs.dedup();
    for required in ["open_local", "clone", "import", "open_archive", "resume"] {
        assert!(
            verbs.iter().any(|v| v == required),
            "acquisition fixture suite must cover the distinct verb {required}; covered: {verbs:?}"
        );
    }
}

#[test]
fn descriptors_round_trip_the_frozen_bootstrap_case_fixtures() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/bootstrap_cases");
    let entries = std::fs::read_dir(&dir).expect("bootstrap_cases dir must exist");
    let mut locators = 0usize;
    let mut plans = 0usize;
    let mut items = 0usize;

    for entry in entries {
        let path = entry.expect("dir entry must read").path();
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let payload = std::fs::read_to_string(&path).expect("fixture must read");

        if file_name.ends_with("__locator.json") {
            let record: SourceLocatorRecord = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("{path:?} must parse as locator: {err}"));
            let rt = serde_json::to_value(&record)
                .and_then(serde_json::from_value::<SourceLocatorRecord>)
                .expect("locator must round-trip");
            assert_eq!(rt, record, "{path:?}: locator round-trip");
            locators += 1;
        } else if file_name.ends_with("__plan.json") {
            let record: CheckoutPlanRecord = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("{path:?} must parse as plan: {err}"));
            let rt = serde_json::to_value(&record)
                .and_then(serde_json::from_value::<CheckoutPlanRecord>)
                .expect("plan must round-trip");
            assert_eq!(rt, record, "{path:?}: plan round-trip");
            plans += 1;
        } else if file_name.contains("__bootstrap_") {
            let record: BootstrapQueueItemRecord = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("{path:?} must parse as bootstrap item: {err}"));
            let rt = serde_json::to_value(&record)
                .and_then(serde_json::from_value::<BootstrapQueueItemRecord>)
                .expect("bootstrap item must round-trip");
            assert_eq!(rt, record, "{path:?}: bootstrap item round-trip");
            assert!(
                record.is_well_formed_blocked_item(),
                "{path:?}: bootstrap item blocker/repair contract"
            );
            items += 1;
        }
    }

    assert!(locators >= 5, "expected the frozen locator fixtures (found {locators})");
    assert!(plans >= 5, "expected the frozen plan fixtures (found {plans})");
    assert!(items >= 5, "expected the frozen bootstrap-item fixtures (found {items})");
}
