//! Fixture-driven coverage for the result-identity / ranking-reasons /
//! partiality-class contract.
//!
//! Each fixture under `fixtures/search/result_identity_cases/` describes a
//! seeded workspace lifecycle, scope, file list, and query, paired with the
//! exact identity packet the lexical query path MUST attach to every row.
//! The test loads every fixture and asserts the projection round-trips, so
//! the protected truth vocabulary cannot drift without a fixture update.

use std::path::Path;

use serde::Deserialize;

use aureline_reactive_state::ReadinessLabel;
use aureline_search::{
    lexical::index::{LexicalIndexInputs, LexicalIndexState},
    lexical::query::{run_query, LexicalQuery},
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
    rows: Vec<ExpectedRow>,
}

#[derive(Debug, Deserialize)]
struct ExpectedRow {
    relative_path: String,
    source_class: String,
    match_kind: String,
    result_id: String,
    ranking_reasons: Vec<String>,
    partiality_class: String,
    partiality_row_badge: String,
    must_show_row_caveat: bool,
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

#[test]
fn result_identity_cases_match_expected_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/result_identity_cases");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one result_identity_case fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "result_identity_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let lifecycle = parse_lifecycle(&fixture.lifecycle_state);
        let readiness_label = parse_readiness_label(&fixture.readiness_label);

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
        let results = run_query(&index, &LexicalQuery::new(fixture.query.clone()));

        assert_eq!(
            results.readiness.as_str(),
            fixture.expect.readiness_class,
            "readiness_class mismatch in {path:?}"
        );

        let actual_rows: Vec<_> = results.groups.iter().flat_map(|g| g.items.iter()).collect();

        assert_eq!(
            actual_rows.len(),
            fixture.expect.rows.len(),
            "row count mismatch in {path:?}: actual={actual_rows:#?}",
        );

        for expected in &fixture.expect.rows {
            let actual = actual_rows
                .iter()
                .find(|r| {
                    r.relative_path == expected.relative_path
                        && r.source_class.as_str() == expected.source_class
                })
                .unwrap_or_else(|| {
                    panic!(
                        "missing row for {} on lane {} in {path:?}",
                        expected.relative_path, expected.source_class
                    )
                });

            assert_eq!(
                actual.match_kind.as_str(),
                expected.match_kind,
                "match_kind mismatch for {} in {path:?}",
                expected.relative_path
            );
            assert_eq!(
                actual.identity.result_id, expected.result_id,
                "result_id mismatch for {} in {path:?}",
                expected.relative_path
            );
            let actual_reasons: Vec<&'static str> = actual.identity.ranking_reason_tokens();
            assert_eq!(
                actual_reasons,
                expected
                    .ranking_reasons
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
                "ranking_reasons mismatch for {} in {path:?}",
                expected.relative_path
            );
            assert_eq!(
                actual.identity.partiality_class.as_str(),
                expected.partiality_class,
                "partiality_class mismatch for {} in {path:?}",
                expected.relative_path
            );
            assert_eq!(
                actual.identity.partiality_class.row_badge(),
                expected.partiality_row_badge,
                "partiality_row_badge mismatch for {} in {path:?}",
                expected.relative_path
            );
            assert_eq!(
                actual.identity.must_show_row_caveat(),
                expected.must_show_row_caveat,
                "must_show_row_caveat mismatch for {} in {path:?}",
                expected.relative_path
            );
            assert_eq!(
                actual.identity.workspace_id, fixture.workspace_id,
                "workspace_id mismatch on identity for {} in {path:?}",
                expected.relative_path
            );
        }
    }
}

#[test]
fn result_id_is_stable_across_re_materializations() {
    // The same row resolved twice must keep the same result_id; this protects
    // selection survival, support replay, and quick-open dedup against
    // ranking-pass churn.
    let inputs = LexicalIndexInputs {
        readiness_inputs: WorkspaceReadinessInputs {
            workspace_id: "ws-stable".to_string(),
            lifecycle_state_token: WorkspaceLifecycleState::Ready.as_str(),
            watcher_health_token: Some("healthy"),
            hot_index_ready: true,
            command_graph_ready: true,
            observed_at: "mono:1".to_string(),
        },
        readiness_label: ReadinessLabel::Exact,
        files: vec!["src/main.rs".to_string()],
        scope: None,
    };
    let index = LexicalIndexState::from_inputs(inputs);
    let first = run_query(&index, &LexicalQuery::new("main.rs"));
    let second = run_query(&index, &LexicalQuery::new("main.rs"));
    assert_eq!(
        first.groups[0].items[0].identity.result_id,
        second.groups[0].items[0].identity.result_id,
    );
}
