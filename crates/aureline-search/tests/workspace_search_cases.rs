//! Fixture-driven coverage for the workspace lexical search shell.
//!
//! Each fixture under `fixtures/search/workspace_search_cases/` describes a
//! seeded workspace lifecycle, scope, file list, and query, paired with the
//! exact card projection the shell MUST render. The test loads every
//! fixture and asserts the projection round-trips, so the protected-row
//! truth vocabulary cannot drift without a fixture update.

use std::path::Path;

use serde::Deserialize;

use aureline_reactive_state::ReadinessLabel;
use aureline_search::{
    lexical::index::{LexicalIndexInputs, LexicalIndexState},
    lexical::query::{run_query, LexicalQuery},
    lexical::scope::ScopeClass,
    lexical::shell::LexicalShell,
};
use aureline_workspace::{WorkspaceLifecycleState, WorkspaceReadinessInputs};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    #[serde(default)]
    scenario: String,
    workspace_id: String,
    observed_at: String,
    lifecycle_state: String,
    readiness_label: String,
    scope_class: String,
    #[serde(default)]
    workset_name: Option<String>,
    files: Vec<String>,
    query: String,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    readiness_class: String,
    readiness_banner: String,
    scope_chip_label: String,
    partial_truth_causes: Vec<String>,
    total_rows: usize,
    groups: Vec<ExpectedGroup>,
}

#[derive(Debug, Deserialize)]
struct ExpectedGroup {
    source_class: String,
    label: String,
    items: Vec<ExpectedItem>,
}

#[derive(Debug, Deserialize)]
struct ExpectedItem {
    relative_path: String,
    match_kind: String,
}

fn parse_lifecycle(token: &str) -> WorkspaceLifecycleState {
    match token {
        "discovered" => WorkspaceLifecycleState::Discovered,
        "trust_evaluating" => WorkspaceLifecycleState::TrustEvaluating,
        "opening" => WorkspaceLifecycleState::Opening,
        "partially_ready" => WorkspaceLifecycleState::PartiallyReady,
        "ready" => WorkspaceLifecycleState::Ready,
        "degraded" => WorkspaceLifecycleState::Degraded,
        "closing" => WorkspaceLifecycleState::Closing,
        "closed" => WorkspaceLifecycleState::Closed,
        other => panic!("unknown lifecycle_state token: {other}"),
    }
}

fn parse_readiness_label(token: &str) -> ReadinessLabel {
    match token {
        "exact" => ReadinessLabel::Exact,
        "imported" => ReadinessLabel::Imported,
        "heuristic" => ReadinessLabel::Heuristic,
        "stale" => ReadinessLabel::Stale,
        "partial" => ReadinessLabel::Partial,
        "unavailable" => ReadinessLabel::Unavailable,
        "out_of_scope" => ReadinessLabel::OutOfScope,
        other => panic!("unknown readiness_label token: {other}"),
    }
}

fn parse_scope_class(token: &str) -> ScopeClass {
    match token {
        "current_repo" => ScopeClass::CurrentRepo,
        "selected_workset" => ScopeClass::SelectedWorkset,
        "sparse_slice" => ScopeClass::SparseSlice,
        "full_workspace" => ScopeClass::FullWorkspace,
        "policy_limited_view" => ScopeClass::PolicyLimitedView,
        other => panic!("unknown scope_class token: {other}"),
    }
}

fn project_scope_label(scope: ScopeClass, workset_name: Option<&str>) -> String {
    match scope {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => {
            scope.chip_label_family().to_string()
        }
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView => {
            match workset_name {
                Some(name) if !name.trim().is_empty() => {
                    format!("{} · {}", scope.chip_label_family(), name)
                }
                _ => scope.chip_label_family().to_string(),
            }
        }
    }
}

#[test]
fn workspace_search_cases_match_expected_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/workspace_search_cases");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one workspace_search_case fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "workspace_search_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let lifecycle = parse_lifecycle(&fixture.lifecycle_state);
        let readiness_label = parse_readiness_label(&fixture.readiness_label);
        let scope = parse_scope_class(&fixture.scope_class);

        let watcher_token: Option<&'static str> = match lifecycle {
            WorkspaceLifecycleState::Ready => Some("healthy"),
            WorkspaceLifecycleState::PartiallyReady => Some("warming"),
            WorkspaceLifecycleState::Degraded => Some("degraded"),
            _ => None,
        };
        let inputs = LexicalIndexInputs {
            readiness_inputs: WorkspaceReadinessInputs {
                workspace_id: fixture.workspace_id.clone(),
                lifecycle_state_token: lifecycle.as_str(),
                watcher_health_token: watcher_token,
                hot_index_ready: matches!(lifecycle, WorkspaceLifecycleState::Ready),
                command_graph_ready: matches!(lifecycle, WorkspaceLifecycleState::Ready),
                observed_at: fixture.observed_at.clone(),
            },
            readiness_label,
            files: fixture.files.clone(),
            scope: None,
        };
        let index = LexicalIndexState::from_inputs(inputs);

        let scope_label = project_scope_label(scope, fixture.workset_name.as_deref());
        let mut shell = LexicalShell::with_empty_query(scope, scope_label.clone(), index.clone());
        shell.set_query(LexicalQuery::new(fixture.query.clone()));

        // Parity check: running the query directly through `run_query`
        // produces the same result the shell holds, so the shell does
        // not silently mutate the projection.
        let direct = run_query(&index, &LexicalQuery::new(fixture.query.clone()));
        assert_eq!(
            shell.results(),
            &direct,
            "shell projection must equal direct query in {path:?} ({})",
            fixture.case_name
        );

        let results = shell.results();
        assert_eq!(
            results.readiness.as_str(),
            fixture.expect.readiness_class,
            "readiness_class mismatch in {path:?}"
        );
        assert_eq!(
            results.readiness.banner_label(),
            fixture.expect.readiness_banner,
            "readiness_banner mismatch in {path:?}"
        );
        assert_eq!(
            scope_label, fixture.expect.scope_chip_label,
            "scope_chip_label mismatch in {path:?}"
        );

        let mut got_causes = results.partial_truth_causes.clone();
        got_causes.sort();
        let mut expected_causes = fixture.expect.partial_truth_causes.clone();
        expected_causes.sort();
        assert_eq!(
            got_causes, expected_causes,
            "partial_truth_causes mismatch in {path:?}"
        );

        assert_eq!(
            results.total_rows, fixture.expect.total_rows,
            "total_rows mismatch in {path:?}"
        );

        assert_eq!(
            results.groups.len(),
            fixture.expect.groups.len(),
            "group count mismatch in {path:?}"
        );

        for (got_group, expected_group) in results.groups.iter().zip(fixture.expect.groups.iter()) {
            assert_eq!(
                got_group.source_class.as_str(),
                expected_group.source_class,
                "group source_class mismatch in {path:?}"
            );
            assert_eq!(
                got_group.label, expected_group.label,
                "group label mismatch in {path:?}"
            );
            assert_eq!(
                got_group.items.len(),
                expected_group.items.len(),
                "group {} item count mismatch in {path:?}",
                got_group.label
            );
            for (got_item, expected_item) in got_group.items.iter().zip(expected_group.items.iter())
            {
                assert_eq!(
                    got_item.relative_path, expected_item.relative_path,
                    "row path mismatch in {path:?}"
                );
                assert_eq!(
                    got_item.match_kind.as_str(),
                    expected_item.match_kind,
                    "row match_kind mismatch in {path:?}"
                );
            }
        }
    }
}
