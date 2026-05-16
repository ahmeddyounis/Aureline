//! Fixture-driven coverage for the M3 workset-scope beta truth lane.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use aureline_workspace::{
    BetaConsumerSurface, BroadActionClass, BroadActionDecision, ExcludedRootReason, ScopeClass,
    ScopeObservationInputs, WorksetArtifactRecord, WorksetScopeBetaSupportExport,
    WorksetScopeBetaTruth,
};

#[derive(Debug, Clone, Deserialize)]
struct BetaFixture {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    workspace: WorkspaceBlock,
    artifact: WorksetArtifactRecord,
    consumer_surfaces: Vec<String>,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureMeta {
    name: String,
    #[serde(default)]
    outside_root_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceBlock {
    workspace_root_refs: Vec<String>,
    #[serde(default)]
    workspace_root_labels: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    is_partial_scope: bool,
    outside_current_scope_marker_visible: bool,
    #[serde(default)]
    excluded_roots_count: Option<u32>,
    #[serde(default)]
    excluded_roots: Vec<ExcludedRootExpect>,
    lineage_length: u32,
    #[serde(default)]
    lineage_underlying_ref: Option<String>,
    admissions: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExcludedRootExpect {
    root_ref: String,
    reason: String,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/scope_truth")
}

fn load_fixtures() -> Vec<(std::path::PathBuf, BetaFixture)> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("scope-truth beta fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let parsed: BetaFixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
            (path, parsed)
        })
        .collect()
}

fn surface_from_token(token: &str) -> BetaConsumerSurface {
    match token {
        "search" => BetaConsumerSurface::Search,
        "graph" => BetaConsumerSurface::Graph,
        "refactor" => BetaConsumerSurface::Refactor,
        "ai" => BetaConsumerSurface::Ai,
        "export" => BetaConsumerSurface::Export,
        "support_packet" => BetaConsumerSurface::SupportPacket,
        other => panic!("unknown consumer surface token {other}"),
    }
}

fn action_from_token(token: &str) -> BroadActionClass {
    match token {
        "search_query" => BroadActionClass::SearchQuery,
        "graph_traversal" => BroadActionClass::GraphTraversal,
        "refactor_apply" => BroadActionClass::RefactorApply,
        "ai_apply" => BroadActionClass::AiApply,
        "export_artifact" => BroadActionClass::ExportArtifact,
        "support_archive" => BroadActionClass::SupportArchive,
        other => panic!("unknown broad action token {other}"),
    }
}

fn excluded_reason_from_token(token: &str) -> ExcludedRootReason {
    match token {
        "excluded_by_pattern" => ExcludedRootReason::ExcludedByPattern,
        "not_in_workset_root_list" => ExcludedRootReason::NotInWorksetRootList,
        "policy_hidden" => ExcludedRootReason::PolicyHidden,
        "unavailable" => ExcludedRootReason::Unavailable,
        other => panic!("unknown excluded-root reason token {other}"),
    }
}

fn project_truth(fixture: &BetaFixture, surface: BetaConsumerSurface) -> WorksetScopeBetaTruth {
    let inputs = ScopeObservationInputs {
        workspace_root_refs: &fixture.workspace.workspace_root_refs,
        workspace_root_labels: &fixture.workspace.workspace_root_labels,
        parent_artifact: None,
    };
    if fixture.expect.outside_current_scope_marker_visible {
        let outside_root = fixture
            .fixture
            .outside_root_ref
            .as_deref()
            .expect("outside-scope fixtures must declare outside_root_ref");
        fixture.artifact.project_beta_truth_outside_scope(
            surface,
            inputs,
            outside_root,
            "Quick-open jumped into a sibling repo without a widen review.",
            "mono:beta:test",
        )
    } else {
        fixture
            .artifact
            .project_beta_truth(surface, inputs, "mono:beta:test")
    }
}

#[test]
fn every_fixture_projects_a_valid_truth_per_consumer_surface() {
    for (path, fixture) in load_fixtures() {
        fixture
            .artifact
            .validate()
            .unwrap_or_else(|err| panic!("alpha artifact in {path:?} must validate: {err}"));
        for token in &fixture.consumer_surfaces {
            let surface = surface_from_token(token);
            let truth = project_truth(&fixture, surface);
            truth.validate().unwrap_or_else(|err| {
                panic!("beta truth for {token} in {path:?} must validate: {err}")
            });

            assert_eq!(
                truth.workset_ref, fixture.artifact.workset_id,
                "beta truth workset_ref mismatch in {path:?}",
            );
            assert_eq!(
                truth.stable_scope_id,
                fixture.artifact.stable_scope_id(),
                "beta truth stable_scope_id mismatch in {path:?}",
            );
            assert_eq!(
                truth.scope_class, fixture.artifact.scope_class,
                "beta truth scope_class mismatch in {path:?}",
            );
            assert_eq!(
                truth.is_partial_scope(),
                fixture.expect.is_partial_scope,
                "beta truth is_partial_scope mismatch in {path:?}",
            );
            assert_eq!(
                truth.outside_current_scope_marker_visible,
                fixture.expect.outside_current_scope_marker_visible,
                "beta truth outside marker mismatch in {path:?}",
            );
            assert_eq!(
                truth.lineage.len(),
                fixture.expect.lineage_length as usize,
                "beta truth lineage length mismatch in {path:?}",
            );
            if let Some(expected_underlying) = fixture.expect.lineage_underlying_ref.as_deref() {
                assert!(
                    truth
                        .lineage
                        .iter()
                        .any(|entry| entry.workset_ref == expected_underlying),
                    "beta truth lineage must include {expected_underlying} in {path:?}",
                );
            }
            if let Some(expected_count) = fixture.expect.excluded_roots_count {
                assert_eq!(
                    truth.excluded_roots.len() as u32,
                    expected_count,
                    "beta truth excluded_roots count mismatch in {path:?}",
                );
            }
            for expected in &fixture.expect.excluded_roots {
                let actual = truth
                    .excluded_roots
                    .iter()
                    .find(|entry| entry.root_ref == expected.root_ref)
                    .unwrap_or_else(|| {
                        panic!(
                            "beta truth missing excluded root {} in {path:?}",
                            expected.root_ref
                        )
                    });
                assert_eq!(
                    actual.reason,
                    excluded_reason_from_token(&expected.reason),
                    "beta truth excluded-root reason mismatch for {} in {path:?}",
                    expected.root_ref,
                );
            }
            for (action_token, expected_decision_token) in &fixture.expect.admissions {
                let action = action_from_token(action_token);
                let admission = truth
                    .admission_for(action)
                    .expect("admission row must exist");
                let actual = admission.decision.as_str();
                assert_eq!(
                    actual,
                    expected_decision_token.as_str(),
                    "beta truth admission for {} mismatch in {path:?}: expected {expected_decision_token} actual {actual}",
                    action_token,
                );
            }
        }
    }
}

#[test]
fn support_export_packet_bundles_every_surface_truth() {
    for (path, fixture) in load_fixtures() {
        if fixture.expect.outside_current_scope_marker_visible {
            continue;
        }
        let inputs = ScopeObservationInputs {
            workspace_root_refs: &fixture.workspace.workspace_root_refs,
            workspace_root_labels: &fixture.workspace.workspace_root_labels,
            parent_artifact: None,
        };
        let truths: Vec<WorksetScopeBetaTruth> = fixture
            .consumer_surfaces
            .iter()
            .map(|token| {
                fixture.artifact.project_beta_truth(
                    surface_from_token(token),
                    inputs.clone(),
                    "mono:beta:export",
                )
            })
            .collect();
        let packet = WorksetScopeBetaSupportExport::from_truths(truths, "mono:beta:export:packet")
            .unwrap_or_else(|err| panic!("support-export packet must validate in {path:?}: {err}"));
        assert_eq!(packet.artifact_workset_ref, fixture.artifact.workset_id);
        assert_eq!(
            packet.artifact_stable_scope_id,
            fixture.artifact.stable_scope_id()
        );
        assert_eq!(packet.truths.len(), fixture.consumer_surfaces.len());
        for token in &fixture.consumer_surfaces {
            assert!(
                packet.truth_for(surface_from_token(token)).is_some(),
                "support-export packet must include truth for {token} in {path:?}",
            );
        }
        let payload = serde_json::to_string(&packet).expect("packet must serialize");
        let parsed: WorksetScopeBetaSupportExport =
            serde_json::from_str(&payload).expect("packet must round-trip");
        assert_eq!(parsed, packet, "packet round-trip mismatch in {path:?}");
    }
}

#[test]
fn outside_scope_truths_block_every_destructive_class() {
    for (path, fixture) in load_fixtures() {
        if !fixture.expect.outside_current_scope_marker_visible {
            continue;
        }
        for token in &fixture.consumer_surfaces {
            let surface = surface_from_token(token);
            let truth = project_truth(&fixture, surface);
            for action in [
                BroadActionClass::RefactorApply,
                BroadActionClass::AiApply,
                BroadActionClass::ExportArtifact,
                BroadActionClass::SupportArchive,
            ] {
                let admission = truth.admission_for(action).expect("admission required");
                assert_eq!(
                    admission.decision,
                    BroadActionDecision::BlockedByOutsideScope,
                    "outside-scope truth must block {action:?} in {path:?}",
                );
            }
            assert!(
                truth.explain_note.is_some(),
                "outside-scope truth must carry an explain note in {path:?}",
            );
        }
    }
}

#[test]
fn fixtures_cover_every_beta_scope_class() {
    let fixtures = load_fixtures();
    let mut covered = std::collections::BTreeSet::new();
    for (_, fixture) in &fixtures {
        covered.insert(match fixture.artifact.scope_class {
            ScopeClass::CurrentRepo => "current_repo",
            ScopeClass::SelectedWorkset => "selected_workset",
            ScopeClass::SparseSlice => "sparse_slice",
            ScopeClass::FullWorkspace => "full_workspace",
            ScopeClass::PolicyLimitedView => "policy_limited_view",
        });
    }
    for required in ["full_workspace", "sparse_slice", "policy_limited_view"] {
        assert!(
            covered.contains(required),
            "missing fixture coverage for scope class {required}; got {covered:?}"
        );
    }
}

#[test]
fn fixture_names_are_unique() {
    let fixtures = load_fixtures();
    let mut names: Vec<String> = fixtures
        .iter()
        .map(|(_, f)| f.fixture.name.clone())
        .collect();
    names.sort();
    let unique = names.iter().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        names.len(),
        unique.len(),
        "fixture names must be unique; got {names:?}"
    );
}

#[test]
fn schema_uses_the_expected_record_kinds() {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/workspace/workset_scope_beta.schema.json");
    let payload = std::fs::read_to_string(&schema_path).expect("schema must read");
    let schema: Value = serde_json::from_str(&payload).expect("schema must parse");
    let kinds = schema["$defs"]["record_kind"]["enum"]
        .as_array()
        .expect("record_kind enum must be present");
    let kinds: std::collections::BTreeSet<&str> = kinds.iter().filter_map(|v| v.as_str()).collect();
    assert!(kinds.contains("workset_scope_beta_truth"));
    assert!(kinds.contains("workset_scope_beta_support_export"));
}
