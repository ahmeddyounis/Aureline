use aureline_language::{IncrementalParseBuffer, ParseRequest, TextEdit};
use aureline_search::{
    PlannerDataPath, PlannerFreshnessClass, PlannerPathDecisionClass, PlannerPathReadiness,
    PlannerPathSnapshot, PlannerResultTruthClass, PlannerUnavailableReason, ScopeClass,
    SearchPlannerAlpha, SearchPlannerInputs, SearchQuerySession, SearchSurface,
    SemanticFallbackState, SEARCH_PLANNER_ALPHA_VERSION,
};

#[test]
fn symbol_search_consumes_language_symbol_snapshot_as_structural_fallback() {
    let source = [
        "export function renderGreeting(name: string): string {",
        "  return `Hello, ${name}`;",
        "}",
        "",
    ]
    .join("\n");
    let mut buffer = IncrementalParseBuffer::open_with_default_registry(
        ParseRequest::foreground_file(
            "parse-session:search-consumer:initial",
            "doc:src/render.ts",
            "buffer:src/render.ts",
            1,
            "language:typescript",
            "2026-05-13T00:00:00Z",
        ),
        source.clone(),
    );
    let edit_start = source
        .find("Hello, ${name}")
        .expect("fixture source contains edit target");
    let update = buffer
        .apply_edit(
            TextEdit::replace(
                "edit:search-consumer:typing",
                edit_start,
                edit_start + "Hello, ${name}".len(),
                "Hello, ${name}!",
            ),
            "parse-session:search-consumer:edit",
            "src/render.ts",
            "2026-05-13T00:00:01Z",
        )
        .expect("incremental edit should parse");

    let structural_snapshot = PlannerPathSnapshot::from_symbol_snapshot(
        "planner:snapshot:structural:symbol-snapshot",
        &update.symbol_snapshot,
    );
    let graph_snapshot = PlannerPathSnapshot {
        path_kind: PlannerDataPath::GraphBacked,
        snapshot_id: "planner:snapshot:graph:warming".into(),
        readiness: PlannerPathReadiness::Warming,
        freshness: PlannerFreshnessClass::Unknown,
        index_epoch: None,
        graph_epoch: Some("graph:epoch:warming".into()),
        unavailable_reason: Some(PlannerUnavailableReason::GraphWarming),
        partial_truth_causes: vec!["graph_index_warming".into()],
        rows: Vec::new(),
    };
    let query_session = SearchQuerySession::for_local_text(
        "search:session:symbol-snapshot",
        SearchSurface::SymbolSearch,
        "renderGreeting",
        ScopeClass::CurrentRepo,
        "Current repo",
        SEARCH_PLANNER_ALPHA_VERSION,
        "warming",
        "mono:symbol-snapshot:0001",
    );

    let output = SearchPlannerAlpha::plan(SearchPlannerInputs {
        query_session,
        planner_pass_id: "planner:pass:symbol-snapshot".into(),
        result_set_id: "planner:results:symbol-snapshot".into(),
        planner_version: SEARCH_PLANNER_ALPHA_VERSION.into(),
        observed_at: "mono:symbol-snapshot:0002".into(),
        path_snapshots: vec![graph_snapshot, structural_snapshot],
    });

    assert_eq!(
        output.planner_pass.semantic_fallback_state,
        SemanticFallbackState::GraphUnavailableStructuralFallback
    );
    assert_eq!(
        output.planner_pass.path_decisions[1].decision_class,
        PlannerPathDecisionClass::SelectedFallback
    );
    let row = output
        .result_set
        .rows
        .first()
        .expect("structural symbol row should feed symbol search");
    assert_eq!(row.title, "renderGreeting");
    assert_eq!(row.answered_by, PlannerDataPath::Structural);
    assert_eq!(row.truth_class, PlannerResultTruthClass::Heuristic);
    assert_eq!(row.relative_path.as_deref(), Some("src/render.ts"));
}
