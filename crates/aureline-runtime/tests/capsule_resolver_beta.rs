//! Integration coverage for the beta environment-capsule resolver.
//!
//! The test replays the checked-in fixtures under
//! [`fixtures/runtime/m3/environment_capsules/`] end-to-end:
//!
//! 1. Each resolution case fixture points at a checked-in workspace under
//!    `workspaces/<scenario>/`. The integration test resolves the workspace
//!    through the canonical beta resolver and asserts the primary source,
//!    drift state, capsule id, prebuild reuse posture, parsed-source
//!    confidence, and conflict notes.
//! 2. Each drift case fixture mints a baseline from one workspace and a
//!    fresh resolution from another; the drift evaluator MUST classify the
//!    outcome and report drift rows / added / removed sources matching the
//!    fixture.
//! 3. The canonical source-coverage manifest fixture round-trips through
//!    serde so reviewer evidence and the runtime emit the same record shape.
//! 4. A support-export packet is built from the conflict drill so reviewers
//!    can verify the packet round-trips and carries the parsed sources,
//!    precedence ladder, and drift evaluations the resolver emitted.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    evaluate_capsule_drift, CapsuleBetaSourceBaseline, CapsuleBetaSourceClass,
    EnvironmentCapsuleBetaCoverageManifest, EnvironmentCapsuleBetaResolver,
    EnvironmentCapsuleBetaSupportExport, ProjectArchetypeHint,
    ENVIRONMENT_CAPSULE_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("environment_capsules")
}

fn archetype_for_token(token: &str) -> ProjectArchetypeHint {
    match token {
        "web_application" => ProjectArchetypeHint::WebApplication,
        "web_frontend_library" => ProjectArchetypeHint::WebFrontendLibrary,
        "backend_service" => ProjectArchetypeHint::BackendService,
        "cli_tool" => ProjectArchetypeHint::CliTool,
        "library_or_sdk" => ProjectArchetypeHint::LibraryOrSdk,
        "data_or_ml_workbench" => ProjectArchetypeHint::DataOrMlWorkbench,
        "mobile_application" => ProjectArchetypeHint::MobileApplication,
        "embedded_or_firmware" => ProjectArchetypeHint::EmbeddedOrFirmware,
        "monorepo_root_with_workspaces" => ProjectArchetypeHint::MonorepoRootWithWorkspaces,
        "monorepo_member_workspace" => ProjectArchetypeHint::MonorepoMemberWorkspace,
        "documentation_site" => ProjectArchetypeHint::DocumentationSite,
        "infrastructure_or_pipeline" => ProjectArchetypeHint::InfrastructureOrPipeline,
        "extension_or_plugin" => ProjectArchetypeHint::ExtensionOrPlugin,
        "archetype_class_unknown_requires_review" => {
            ProjectArchetypeHint::ArchetypeClassUnknownRequiresReview
        }
        other => panic!("unknown archetype hint token: {other}"),
    }
}

#[derive(Debug, Deserialize)]
struct ResolutionCase {
    record_kind: String,
    schema_version: u32,
    workspace_dir: String,
    archetype_hint: String,
    expect: ResolutionExpect,
}

#[derive(Debug, Deserialize)]
struct ResolutionExpect {
    primary_source_token: Option<String>,
    drift_state: String,
    prebuild_reuse_state: String,
    capsule_id: String,
    expected_source_classes: Vec<String>,
    expected_confidence_by_class: BTreeMap<String, String>,
    conflict_notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DriftCase {
    record_kind: String,
    schema_version: u32,
    baseline_workspace_dir: String,
    fresh_workspace_dir: String,
    archetype_hint: String,
    expect: DriftExpect,
}

#[derive(Debug, Deserialize)]
struct DriftExpect {
    outcome_token: String,
    drift_source_classes: Vec<String>,
    added_sources: Vec<String>,
    removed_sources: Vec<String>,
}

fn classes_to_tokens(classes: &[CapsuleBetaSourceClass]) -> Vec<String> {
    classes.iter().map(|c| c.as_str().to_owned()).collect()
}

fn assert_resolution_case(case_path: &Path) {
    let payload = std::fs::read_to_string(case_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", case_path.display()));
    let case: ResolutionCase = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse {}: {err}", case_path.display()));
    assert_eq!(case.record_kind, "environment_capsule_beta_case");
    assert_eq!(case.schema_version, 1);

    let workspace = fixture_root().join(&case.workspace_dir);
    let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
    let resolution =
        resolver.resolve_workspace(&workspace, archetype_for_token(&case.archetype_hint));

    assert_eq!(
        resolution.primary_source_token, case.expect.primary_source_token,
        "{}: primary_source_token mismatch",
        case.workspace_dir
    );
    assert_eq!(
        resolution.drift_state.as_str(),
        case.expect.drift_state,
        "{}: drift_state mismatch",
        case.workspace_dir
    );
    assert_eq!(
        resolution.prebuild_reuse_state.as_str(),
        case.expect.prebuild_reuse_state,
        "{}: prebuild_reuse_state mismatch",
        case.workspace_dir
    );
    assert_eq!(
        resolution.environment_capsule_ref.capsule_id, case.expect.capsule_id,
        "{}: capsule_id mismatch",
        case.workspace_dir
    );

    let actual_classes: Vec<String> = resolution
        .sources
        .iter()
        .map(|s| s.source_class_token.clone())
        .collect();
    for expected in &case.expect.expected_source_classes {
        assert!(
            actual_classes.contains(expected),
            "{}: missing source class {} (got {:?})",
            case.workspace_dir,
            expected,
            actual_classes
        );
    }
    assert_eq!(
        actual_classes.len(),
        case.expect.expected_source_classes.len(),
        "{}: unexpected source classes (got {:?}, expected {:?})",
        case.workspace_dir,
        actual_classes,
        case.expect.expected_source_classes,
    );

    for (class, confidence) in &case.expect.expected_confidence_by_class {
        let src = resolution
            .sources
            .iter()
            .find(|s| s.source_class_token == *class)
            .unwrap_or_else(|| {
                panic!(
                    "{}: missing source class {} in confidence map",
                    case.workspace_dir, class
                )
            });
        assert_eq!(
            src.confidence_token, *confidence,
            "{}: confidence mismatch for {}",
            case.workspace_dir, class
        );
    }

    let actual_conflicts: Vec<String> = resolution
        .conflict_notes
        .iter()
        .map(|n| n.as_str().to_owned())
        .collect();
    assert_eq!(
        actual_conflicts, case.expect.conflict_notes,
        "{}: conflict_notes mismatch",
        case.workspace_dir
    );

    if let Some(primary) = case.expect.primary_source_token.as_deref() {
        let winners: Vec<String> = resolution
            .precedence
            .iter()
            .filter(|row| row.winner)
            .map(|row| row.source_class_token.clone())
            .collect();
        assert_eq!(
            winners,
            vec![primary.to_owned()],
            "{}: precedence winner mismatch",
            case.workspace_dir
        );
    } else {
        for row in &resolution.precedence {
            assert!(!row.winner, "no source must win when primary is null");
        }
    }
}

fn assert_drift_case(case_path: &Path) {
    let payload = std::fs::read_to_string(case_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", case_path.display()));
    let case: DriftCase = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse {}: {err}", case_path.display()));
    assert_eq!(case.record_kind, "environment_capsule_beta_drift_case");
    assert_eq!(case.schema_version, 1);

    let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
    let baseline_workspace = fixture_root().join(&case.baseline_workspace_dir);
    let fresh_workspace = fixture_root().join(&case.fresh_workspace_dir);
    let archetype = archetype_for_token(&case.archetype_hint);
    let baseline_resolution = resolver.resolve_workspace(&baseline_workspace, archetype);
    let baseline = CapsuleBetaSourceBaseline::from_resolution(&baseline_resolution);
    let fresh_resolution = resolver.resolve_workspace(&fresh_workspace, archetype);

    let evaluation = evaluate_capsule_drift(&baseline, &fresh_resolution);
    assert_eq!(
        evaluation.outcome_token, case.expect.outcome_token,
        "{}: drift outcome mismatch",
        case.baseline_workspace_dir
    );
    let drift_classes: Vec<String> = evaluation
        .drift_rows
        .iter()
        .map(|row| row.source_class_token.clone())
        .collect();
    for expected in &case.expect.drift_source_classes {
        assert!(
            drift_classes.contains(expected),
            "{}: missing drift class {} (got {:?})",
            case.baseline_workspace_dir,
            expected,
            drift_classes
        );
    }
    let added_tokens = classes_to_tokens(&evaluation.added_sources);
    assert_eq!(
        added_tokens, case.expect.added_sources,
        "{}: added_sources mismatch",
        case.baseline_workspace_dir
    );
    let removed_tokens = classes_to_tokens(&evaluation.removed_sources);
    assert_eq!(
        removed_tokens, case.expect.removed_sources,
        "{}: removed_sources mismatch",
        case.baseline_workspace_dir
    );
}

#[test]
fn every_resolution_case_replays_through_the_beta_resolver() {
    for case_name in [
        "devcontainer_only_case.json",
        "devcontainer_with_compose_case.json",
        "compose_only_case.json",
        "nix_flake_case.json",
        "conflict_devcontainer_nix_compose_case.json",
        "empty_workspace_case.json",
    ] {
        let case_path = fixture_root().join(case_name);
        assert_resolution_case(&case_path);
    }
}

#[test]
fn drift_cases_classify_against_typed_outcomes() {
    for case_name in ["drift_after_edit_case.json", "source_added_drift_case.json"] {
        let case_path = fixture_root().join(case_name);
        assert_drift_case(&case_path);
    }
}

#[test]
fn coverage_manifest_fixture_round_trips_through_serde() {
    let path = fixture_root().join("beta_source_coverage.json");
    let payload = std::fs::read_to_string(&path).expect("read coverage manifest fixture");
    let fixture: EnvironmentCapsuleBetaCoverageManifest =
        serde_json::from_str(&payload).expect("parse coverage manifest fixture");
    assert_eq!(
        fixture.record_kind,
        ENVIRONMENT_CAPSULE_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );
    let canonical = EnvironmentCapsuleBetaCoverageManifest::canonical(
        "environment-capsule-beta:canonical",
        "2026-05-15T00:00:00Z",
    );
    assert_eq!(fixture, canonical);
    assert!(canonical.covers_every_source_class());
}

#[test]
fn conflict_drill_support_export_round_trips_with_drift_evaluations() {
    let resolver = EnvironmentCapsuleBetaResolver::default_read_only();
    let baseline_workspace = fixture_root().join("workspaces/devcontainer_only");
    let fresh_workspace = fixture_root().join("workspaces/conflict_devcontainer_nix_compose");
    let baseline_resolution =
        resolver.resolve_workspace(&baseline_workspace, ProjectArchetypeHint::WebApplication);
    let baseline = CapsuleBetaSourceBaseline::from_resolution(&baseline_resolution);
    let fresh_resolution =
        resolver.resolve_workspace(&fresh_workspace, ProjectArchetypeHint::BackendService);
    let drift = evaluate_capsule_drift(&baseline, &fresh_resolution);

    let packet = EnvironmentCapsuleBetaSupportExport::new(
        "environment-capsule-beta:packet",
        "2026-05-15T00:00:00Z",
        fresh_resolution.clone(),
        vec![drift.clone()],
    );
    let json = serde_json::to_string(&packet).expect("serialize packet");
    let round: EnvironmentCapsuleBetaSupportExport =
        serde_json::from_str(&json).expect("deserialize packet");
    assert_eq!(round, packet);
    assert_eq!(
        round.record_kind,
        ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(round.coverage_manifest.covers_every_source_class());
    assert!(round.drift_evaluations.iter().any(|eval| eval.is_drifted()));
    assert_eq!(
        round.resolution.primary_source_token.as_deref(),
        Some("devcontainer")
    );
    assert!(round
        .resolution
        .conflict_notes
        .iter()
        .any(|note| note.as_str() == "overridden_by_higher_precedence"));
}
