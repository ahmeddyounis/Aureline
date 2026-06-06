//! Replay gate for stable source-locator, checkout-plan, bootstrap-result, and
//! queue truth across project-entry lanes.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    stabilize_source_locator_checkout_plan_bootstrap_result_and_queue, AcquisitionOutcomeClass,
    AcquisitionSurface, BootstrapCompletionState, BootstrapQueueItemRecord, CheckoutPlanRecord,
    SourceLocatorRecord, StableProjectEntryTruthInput, StableProjectEntryTruthRecord,
    SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_RECORD_KIND,
    SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct ScenarioFixture {
    surface: String,
    locator: SourceLocatorRecord,
    plan: CheckoutPlanRecord,
    #[serde(default)]
    bootstrap_items: Vec<BootstrapQueueItemRecord>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m3/repository_acquisition_and_bootstrap")
}

fn stable_fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/workspace/m4/stabilize-source-locator-checkout-plan-bootstrap-result-and-queue",
    )
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

fn load_scenarios() -> Vec<(String, ScenarioFixture)> {
    let dir = fixtures_dir();
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: ScenarioFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push((path.display().to_string(), fixture));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "expected acquisition fixtures");
    out
}

fn project(fixture: &ScenarioFixture) -> StableProjectEntryTruthRecord {
    stabilize_source_locator_checkout_plan_bootstrap_result_and_queue(
        StableProjectEntryTruthInput {
            locator: &fixture.locator,
            plan: &fixture.plan,
            bootstrap_items: &fixture.bootstrap_items,
            surface: surface_from_token(&fixture.surface),
        },
    )
    .expect("stable project-entry truth projects")
}

#[test]
fn every_project_entry_lane_projects_to_stable_truth() {
    let fixtures = load_scenarios();
    assert!(
        fixtures.len() >= 9,
        "scenario suite must keep broad entry-lane coverage"
    );

    let mut outcomes = Vec::new();
    for (name, fixture) in &fixtures {
        let record = project(fixture);
        assert_eq!(
            record.record_kind,
            SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_RECORD_KIND
        );
        assert_eq!(
            record.schema_version,
            SOURCE_LOCATOR_CHECKOUT_BOOTSTRAP_SCHEMA_VERSION
        );
        assert!(
            record.is_contract_valid(),
            "{name} must validate: {:?}",
            record.contract_findings()
        );
        assert!(record.default_support_export_safe, "{name}: support export");
        assert!(
            !record.credential_descriptor.raw_secret_present,
            "{name}: raw secrets must never be serialized"
        );
        assert_eq!(
            record.source_locator.source_locator_ref, fixture.locator.source_locator_id,
            "{name}: locator ref"
        );
        assert_eq!(
            record.checkout_plan.checkout_plan_ref, fixture.plan.checkout_plan_id,
            "{name}: plan ref"
        );
        assert_eq!(
            record.bootstrap_queue.len(),
            fixture.bootstrap_items.len(),
            "{name}: queue remains itemized"
        );
        for item in &record.bootstrap_queue {
            assert!(
                item.reviewable || !item.cancelable,
                "{name}: cancelable item {} must remain reviewable",
                item.item_ref
            );
            assert!(
                !item.evidence_ref.is_empty(),
                "{name}: item {} must carry evidence",
                item.item_ref
            );
        }
        if record.bootstrap_result.partial_authority {
            assert_ne!(
                record.bootstrap_result.completion_state,
                BootstrapCompletionState::Completed,
                "{name}: partial authority must not claim completed workspace"
            );
        }
        outcomes.extend(record.bootstrap_result.outcome_lineage);
    }

    for required in [
        AcquisitionOutcomeClass::Opened,
        AcquisitionOutcomeClass::Acquired,
        AcquisitionOutcomeClass::Mirrored,
        AcquisitionOutcomeClass::Imported,
        AcquisitionOutcomeClass::Resumed,
        AcquisitionOutcomeClass::PartiallyAcquired,
    ] {
        assert!(
            outcomes.contains(&required),
            "stable packet suite must distinguish {required:?}"
        );
    }
}

#[test]
fn interrupted_partial_mirror_fixture_stays_truthful() {
    let source_raw =
        std::fs::read_to_string(fixtures_dir().join("interrupted_mirror_clone_resume.json"))
            .expect("source fixture reads");
    let source: ScenarioFixture = serde_json::from_str(&source_raw).expect("source fixture parses");
    let projected = project(&source);

    let stable_raw = std::fs::read_to_string(
        stable_fixtures_dir().join("interrupted_partial_mirror_resume_packet.json"),
    )
    .expect("stable fixture reads");
    let fixture: StableProjectEntryTruthRecord =
        serde_json::from_str(&stable_raw).expect("stable fixture parses");

    assert_eq!(
        projected, fixture,
        "checked-in stable fixture must match the live projection"
    );
    assert!(
        fixture
            .bootstrap_result
            .outcome_lineage
            .contains(&AcquisitionOutcomeClass::Mirrored),
        "fixture must preserve mirror provenance"
    );
    assert!(
        fixture
            .bootstrap_result
            .outcome_lineage
            .contains(&AcquisitionOutcomeClass::PartiallyAcquired),
        "fixture must preserve partial acquisition"
    );
    assert_eq!(
        fixture.bootstrap_result.completion_state,
        BootstrapCompletionState::Interrupted,
        "interrupted sparse/partial acquisition must not flatten to success"
    );
}
