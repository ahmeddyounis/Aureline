//! Fixture-driven coverage for the hot-set indexing scheduler alpha.
//!
//! Each fixture under `fixtures/search/hot_set_alpha/` describes a workspace
//! warm-up state, hot-set signals, a quick-open query, and the exact first
//! useful navigation projection. The tests assert that quick open receives a
//! usable lexical index before full indexing completes and that fallback
//! reasons stay visible when hot-set inputs are absent.

use std::path::Path;

use serde::Deserialize;

use aureline_reactive_state::ReadinessLabel;
use aureline_search::{
    HotSetCandidate, IndexSchedulerAlpha, IndexSchedulerInputs, LexicalQuery, ScopeClass,
};
use aureline_workspace::WorkspaceLifecycleState;

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
    full_index_complete: bool,
    scope_class: String,
    scope_label: String,
    query: String,
    discovered_files: Vec<String>,
    hot_set_candidates: Vec<HotSetCandidate>,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    readiness_state: String,
    lexical_readiness_class: String,
    first_useful_paths: Vec<String>,
    plan_partial_truth_causes: Vec<String>,
    shell_partial_truth_causes: Vec<String>,
    quick_open_total_rows: usize,
    quick_open_rows: Vec<String>,
    fallback_reason: Option<String>,
    quick_open_blocked_by_full_index: bool,
    edit_blocked_by_index_warmup: bool,
    path_explanations: Vec<ExpectedPathExplanation>,
    symbol_explanations: Vec<ExpectedSymbolExplanation>,
}

#[derive(Debug, Deserialize)]
struct ExpectedPathExplanation {
    relative_path: String,
    input_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSymbolExplanation {
    symbol_ref: String,
    input_classes: Vec<String>,
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

#[test]
fn hot_set_scheduler_cases_match_expected_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/hot_set_alpha");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one hot_set_alpha fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "hot_set_alpha_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let inputs = IndexSchedulerInputs::for_fixture(
            fixture.workspace_id.clone(),
            fixture.observed_at.clone(),
            parse_lifecycle(&fixture.lifecycle_state),
            parse_readiness_label(&fixture.readiness_label),
            fixture.discovered_files.clone(),
            fixture.hot_set_candidates.clone(),
            fixture.full_index_complete,
        );
        let output = IndexSchedulerAlpha::schedule(inputs.clone());
        assert_eq!(
            output.plan.readiness_state.as_str(),
            fixture.expect.readiness_state,
            "readiness_state mismatch in {path:?}"
        );
        assert_eq!(
            output.lexical_index.readiness().as_str(),
            fixture.expect.lexical_readiness_class,
            "lexical readiness mismatch in {path:?}"
        );
        assert_eq!(
            output.navigation_snapshot.first_useful_paths, fixture.expect.first_useful_paths,
            "first_useful_paths mismatch in {path:?}"
        );
        assert_eq!(
            output.navigation_snapshot.partial_truth_causes,
            fixture.expect.plan_partial_truth_causes,
            "plan partial causes mismatch in {path:?}"
        );
        assert_eq!(
            output.navigation_snapshot.fallback_reason, fixture.expect.fallback_reason,
            "fallback reason mismatch in {path:?}"
        );
        assert_eq!(
            output.navigation_snapshot.quick_open_blocked_by_full_index,
            fixture.expect.quick_open_blocked_by_full_index,
            "quick-open responsiveness mismatch in {path:?}"
        );
        assert_eq!(
            output.navigation_snapshot.edit_blocked_by_index_warmup,
            fixture.expect.edit_blocked_by_index_warmup,
            "edit responsiveness mismatch in {path:?}"
        );

        for expected in &fixture.expect.path_explanations {
            let explanation = output
                .plan
                .why_path_is_hot(&expected.relative_path)
                .unwrap_or_else(|| {
                    panic!(
                        "missing hot-set path explanation for {} in {path:?}",
                        expected.relative_path
                    )
                });
            let actual_classes: Vec<&str> = explanation
                .input_classes
                .iter()
                .map(|class| class.as_str())
                .collect();
            let expected_classes: Vec<&str> =
                expected.input_classes.iter().map(String::as_str).collect();
            assert_eq!(
                actual_classes, expected_classes,
                "path explanation class mismatch for {} in {path:?}",
                expected.relative_path
            );
        }

        for expected in &fixture.expect.symbol_explanations {
            let explanation = output
                .plan
                .why_symbol_is_hot(&expected.symbol_ref)
                .unwrap_or_else(|| {
                    panic!(
                        "missing hot-set symbol explanation for {} in {path:?}",
                        expected.symbol_ref
                    )
                });
            let actual_classes: Vec<&str> = explanation
                .input_classes
                .iter()
                .map(|class| class.as_str())
                .collect();
            let expected_classes: Vec<&str> =
                expected.input_classes.iter().map(String::as_str).collect();
            assert_eq!(
                actual_classes, expected_classes,
                "symbol explanation class mismatch for {} in {path:?}",
                expected.symbol_ref
            );
        }

        let quick_open = IndexSchedulerAlpha::quick_open_snapshot(
            inputs,
            parse_scope_class(&fixture.scope_class),
            fixture.scope_label.clone(),
            LexicalQuery::new(fixture.query.clone()),
            fixture.observed_at.clone(),
        );
        assert_eq!(
            quick_open.shell.readiness_class, fixture.expect.lexical_readiness_class,
            "quick-open shell readiness mismatch in {path:?}"
        );
        assert_eq!(
            quick_open.shell.partial_truth_causes, fixture.expect.shell_partial_truth_causes,
            "quick-open shell partial causes mismatch in {path:?}"
        );
        assert_eq!(
            quick_open.shell.total_rows, fixture.expect.quick_open_total_rows,
            "quick-open total rows mismatch in {path:?}"
        );
        let actual_rows: Vec<String> = quick_open
            .shell
            .groups
            .iter()
            .flat_map(|group| group.items.iter())
            .map(|item| item.relative_path.clone())
            .collect();
        assert_eq!(
            actual_rows, fixture.expect.quick_open_rows,
            "quick-open rows mismatch in {path:?}"
        );
    }
}
