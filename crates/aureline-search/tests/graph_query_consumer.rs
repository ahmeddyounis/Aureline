use aureline_graph::{GraphQueryRequest, GraphStore};
use aureline_graph_proto::all_scenarios;
use aureline_search::{
    PlannerDataPath, PlannerPathDecisionClass, PlannerPathReadiness, PlannerPathSnapshot,
    PlannerRankingReason, PlannerResultTruthClass, ScopeClass, SearchPlannerAlpha,
    SearchPlannerInputs, SearchQuerySession, SearchSurface, SemanticFallbackState,
    SEARCH_PLANNER_ALPHA_VERSION,
};

#[test]
fn search_planner_consumes_graph_query_envelope_without_private_rows() {
    let graph = all_scenarios()
        .into_iter()
        .find(|scenario| scenario.label == "local_root_workspace")
        .expect("local graph scenario exists")
        .graph;
    let store = GraphStore::persist_snapshot(graph).expect("scenario graph must validate");
    let envelope = store.query(GraphQueryRequest::symbol_lookup(
        "q:search:symbol:greet",
        "ws:aureline",
        "greet",
    ));

    let graph_snapshot =
        PlannerPathSnapshot::from_graph_query_envelope("planner:graph:symbol:greet", &envelope);
    let query_session = SearchQuerySession::for_local_text(
        "search:session:graph:symbol:greet",
        SearchSurface::SymbolSearch,
        "greet",
        ScopeClass::CurrentRepo,
        "Current repo",
        SEARCH_PLANNER_ALPHA_VERSION,
        "warming",
        "mono:search:graph:0001",
    );

    let output = SearchPlannerAlpha::plan(SearchPlannerInputs {
        query_session,
        planner_pass_id: "planner:pass:graph:symbol:greet".into(),
        result_set_id: "planner:results:graph:symbol:greet".into(),
        planner_version: SEARCH_PLANNER_ALPHA_VERSION.into(),
        observed_at: "mono:search:graph:0002".into(),
        path_snapshots: vec![graph_snapshot],
    });

    assert_eq!(
        output.query_session.graph_epoch.as_deref(),
        Some(store.workspace_graph_id())
    );
    assert_eq!(
        output.planner_pass.semantic_fallback_state,
        SemanticFallbackState::GraphBacked
    );
    assert_eq!(
        output.planner_pass.path_decisions[0].decision_class,
        PlannerPathDecisionClass::SelectedPrimary
    );

    let row = output
        .result_set
        .rows
        .first()
        .expect("graph-backed symbol result is planned");
    assert_eq!(row.canonical_id, "node:symbol:greet_fn");
    assert_eq!(row.answered_by, PlannerDataPath::GraphBacked);
    assert_eq!(row.truth_class, PlannerResultTruthClass::GraphBacked);
    assert_eq!(row.readiness_state, PlannerPathReadiness::Ready);
    assert_eq!(row.relative_path.as_deref(), Some("src/lib.rs"));
    assert_eq!(row.symbol_ref.as_deref(), Some("aureline::greet"));
    assert_eq!(
        row.ranking_reasons,
        vec![PlannerRankingReason::GraphExactSymbol]
    );
    assert!(!row.explanation.degraded_by_missing_graph_or_language);
}
