//! Fixture-driven coverage for the shared search planner alpha.
//!
//! Each fixture under `fixtures/search/planner_alpha/` describes one query
//! session, the lexical/structural/cached/graph-backed snapshots available to
//! the planner, and the exact path decisions and result explanations expected
//! for quick open, file search, or symbol search.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{
    PlannerDataPath, PlannerPathDecisionClass, PlannerResultTruthClass, SearchPlannerAlpha,
    SearchPlannerInputs,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SearchPlannerInputs,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    query_session_readiness_state: String,
    planner_readiness_state: String,
    semantic_fallback_state: String,
    path_decisions: Vec<ExpectedDecision>,
    rows: Vec<ExpectedRow>,
}

#[derive(Debug, Deserialize)]
struct ExpectedDecision {
    path_kind: String,
    snapshot_id: String,
    decision_class: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedRow {
    canonical_id: String,
    answered_by: String,
    answer_role: String,
    truth_class: String,
    readiness_state: String,
    ranking_reasons: Vec<String>,
    partial_truth_causes: Vec<String>,
    degraded_by_missing_graph_or_language: bool,
    fallback_reason: Option<String>,
    contributions: Vec<ExpectedContribution>,
}

#[derive(Debug, Deserialize)]
struct ExpectedContribution {
    path_kind: String,
    role: String,
}

#[test]
fn planner_alpha_cases_match_expected_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/planner_alpha");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one planner_alpha fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "planner_alpha_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let output = SearchPlannerAlpha::plan(fixture.input);

        assert_eq!(
            output.query_session.readiness_state, fixture.expect.query_session_readiness_state,
            "query session readiness mismatch in {path:?}"
        );
        assert_eq!(
            output.planner_pass.readiness_state.as_str(),
            fixture.expect.planner_readiness_state,
            "planner readiness mismatch in {path:?}"
        );
        assert_eq!(
            output.planner_pass.semantic_fallback_state.as_str(),
            fixture.expect.semantic_fallback_state,
            "semantic fallback mismatch in {path:?}"
        );

        for expected in &fixture.expect.path_decisions {
            let actual = output
                .planner_pass
                .path_decisions
                .iter()
                .find(|decision| {
                    decision.path_kind.as_str() == expected.path_kind
                        && decision.snapshot_id == expected.snapshot_id
                })
                .unwrap_or_else(|| {
                    panic!(
                        "missing path decision {} {} in {path:?}",
                        expected.path_kind, expected.snapshot_id
                    )
                });
            assert_eq!(
                actual.decision_class.as_str(),
                expected.decision_class,
                "decision mismatch for {} in {path:?}",
                expected.snapshot_id
            );
        }

        assert_eq!(
            output.result_set.rows.len(),
            fixture.expect.rows.len(),
            "result row count mismatch in {path:?}"
        );

        for expected in &fixture.expect.rows {
            let actual = output
                .result_set
                .rows
                .iter()
                .find(|row| row.canonical_id == expected.canonical_id)
                .unwrap_or_else(|| {
                    panic!(
                        "missing planned result {} in {path:?}",
                        expected.canonical_id
                    )
                });

            assert_enum_token(
                actual.answered_by,
                expected.answered_by.as_str(),
                "answered_by",
                &path,
            );
            assert_decision_token(
                actual.answer_role,
                expected.answer_role.as_str(),
                "answer_role",
                &path,
            );
            assert_truth_token(
                actual.truth_class,
                expected.truth_class.as_str(),
                "truth_class",
                &path,
            );
            assert_eq!(
                actual.readiness_state.as_str(),
                expected.readiness_state,
                "row readiness mismatch for {} in {path:?}",
                expected.canonical_id
            );
            assert_eq!(
                actual.ranking_reason_tokens(),
                expected
                    .ranking_reasons
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<&str>>(),
                "ranking reasons mismatch for {} in {path:?}",
                expected.canonical_id
            );
            assert_eq!(
                actual.partial_truth_cause_tokens(),
                expected
                    .partial_truth_causes
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<&str>>(),
                "partial truth causes mismatch for {} in {path:?}",
                expected.canonical_id
            );
            assert_eq!(
                actual.explanation.degraded_by_missing_graph_or_language,
                expected.degraded_by_missing_graph_or_language,
                "degraded flag mismatch for {} in {path:?}",
                expected.canonical_id
            );
            assert_eq!(
                actual
                    .explanation
                    .fallback_reason
                    .map(|reason| reason.as_str().to_string()),
                expected.fallback_reason,
                "fallback reason mismatch for {} in {path:?}",
                expected.canonical_id
            );

            assert_eq!(
                actual.contributions.len(),
                expected.contributions.len(),
                "contribution count mismatch for {} in {path:?}",
                expected.canonical_id
            );
            for expected_contribution in &expected.contributions {
                let contribution = actual
                    .contributions
                    .iter()
                    .find(|contribution| {
                        contribution.path_kind.as_str() == expected_contribution.path_kind
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "missing contribution {} for {} in {path:?}",
                            expected_contribution.path_kind, expected.canonical_id
                        )
                    });
                assert_eq!(
                    contribution.role.as_str(),
                    expected_contribution.role,
                    "contribution role mismatch for {} in {path:?}",
                    expected_contribution.path_kind
                );
            }
        }
    }
}

fn assert_enum_token(actual: PlannerDataPath, expected: &str, field: &str, path: &std::path::Path) {
    assert_eq!(actual.as_str(), expected, "{field} mismatch in {path:?}");
}

fn assert_decision_token(
    actual: PlannerPathDecisionClass,
    expected: &str,
    field: &str,
    path: &std::path::Path,
) {
    assert_eq!(actual.as_str(), expected, "{field} mismatch in {path:?}");
}

fn assert_truth_token(
    actual: PlannerResultTruthClass,
    expected: &str,
    field: &str,
    path: &std::path::Path,
) {
    assert_eq!(actual.as_str(), expected, "{field} mismatch in {path:?}");
}
