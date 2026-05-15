use std::path::Path;

use aureline_docs::{DocsPack, DocsSearchIndex};
use aureline_search::{
    PlannedSearchResult, PlannerDataPath, PlannerPathReadiness, PlannerPathSnapshot,
    PlannerRankingReason, PlannerResultTruthClass, ScopeClass, SearchPlannerAlpha,
    SearchPlannerInputs, SearchQuerySession, SearchSurface, SEARCH_PLANNER_ALPHA_VERSION,
};

fn repo_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

#[test]
fn docs_index_query_projects_to_planner_docs_results() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("fixture docs pack loads");
    let index = DocsSearchIndex::from_pack("ws-docs-alpha", "docs-index:alpha:01", pack)
        .with_partial_truth(["docs_pack_subset"]);
    let query_result = index.query("setup");
    let snapshot = PlannerPathSnapshot::from_docs_search_query_result(
        "search:snapshot:docs:index:01",
        &query_result,
    );
    let session = SearchQuerySession::for_local_text(
        "search:session:docs:index:01",
        SearchSurface::DocsSearch,
        "setup",
        ScopeClass::CurrentRepo,
        "Current repo",
        SEARCH_PLANNER_ALPHA_VERSION,
        "partial",
        "mono:docs-index:01",
    );

    let output = SearchPlannerAlpha::plan(SearchPlannerInputs {
        query_session: session,
        planner_pass_id: "search:planner:docs:index:01".to_string(),
        result_set_id: "search:result_set:docs:index:01".to_string(),
        planner_version: SEARCH_PLANNER_ALPHA_VERSION.to_string(),
        observed_at: "mono:docs-index:02".to_string(),
        path_snapshots: vec![snapshot],
    });

    let row = output
        .result_set
        .rows
        .iter()
        .find(|row| row.canonical_id == "docs-anchor:bundle:typescript-web:start")
        .expect("docs anchor row appears");

    assert_docs_row(row);
    assert_eq!(row.readiness_state, PlannerPathReadiness::Partial);
    assert_eq!(
        row.partial_truth_causes,
        vec!["docs_pack_subset".to_string()]
    );
    assert_eq!(
        output.query_session.index_epoch.as_deref(),
        Some("docs-index:alpha:01")
    );
}

fn assert_docs_row(row: &PlannedSearchResult) {
    assert_eq!(
        row.result_id,
        "search:planned:docs_search:docs-anchor:bundle:typescript-web:start"
    );
    assert_eq!(row.answered_by, PlannerDataPath::Docs);
    assert_eq!(row.truth_class, PlannerResultTruthClass::Imported);
    assert_eq!(
        row.ranking_reasons,
        vec![
            PlannerRankingReason::DocsAnchorMatch,
            PlannerRankingReason::CitationAvailable
        ]
    );
    assert_eq!(row.target_kind.as_str(), "docs_anchor");
    assert_eq!(
        row.symbol_ref.as_deref(),
        Some("id:docs-reopen:launch-bundle:typescript-web:start")
    );
}
