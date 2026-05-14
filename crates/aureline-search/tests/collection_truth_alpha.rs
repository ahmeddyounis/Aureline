//! Fixture-driven coverage for shared collection truth.
//!
//! The cases under `fixtures/search/collection_truth_alpha/` exercise the
//! first search-like consumer of the dense collection contract: typed filter
//! ASTs, saved-view portability, hidden narrowing chips, result-scope counts,
//! and stable selection truth.

use std::path::{Path, PathBuf};

use aureline_search::{
    CollectionCountStatus, CollectionSurfaceFamily, CollectionViewAlphaRecord, SavedCollectionView,
    SearchCollectionViewInputs, SearchQuerySession, SearchScopeCountsRecord,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CollectionTruthFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    input: CollectionTruthInput,
    expect: CollectionTruthExpect,
}

#[derive(Debug, Deserialize)]
struct CollectionTruthInput {
    collection_view_id: String,
    query_session: SearchQuerySession,
    saved_view: SavedCollectionView,
    search_counts: SearchScopeCountsRecord,
    result_identity_refs: Vec<String>,
    selected_result_ids: Vec<String>,
    blocked_result_ids: Vec<String>,
    hidden_selected_count: u64,
}

#[derive(Debug, Deserialize)]
struct CollectionTruthExpect {
    surface_family: CollectionSurfaceFamily,
    hidden_narrowing_count: usize,
    filter_chip_count: usize,
    selected_count: u64,
    blocked_count: u64,
    hidden_count: u64,
    all_matching_status: CollectionCountStatus,
    saved_view_degraded: bool,
    saved_view_findings: usize,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/collection_truth_alpha")
}

fn collection_view_for(fixture: &CollectionTruthFixture) -> CollectionViewAlphaRecord {
    CollectionViewAlphaRecord::from_search_results(SearchCollectionViewInputs {
        collection_view_id: fixture.input.collection_view_id.clone(),
        query_session: fixture.input.query_session.clone(),
        filter_ast: fixture.input.saved_view.filter_ast.clone(),
        saved_view_id_ref: Some(fixture.input.saved_view.saved_view_id.clone()),
        search_counts: fixture.input.search_counts.clone(),
        result_identity_refs: fixture.input.result_identity_refs.clone(),
        selected_result_ids: fixture.input.selected_result_ids.clone(),
        blocked_result_ids: fixture.input.blocked_result_ids.clone(),
        hidden_selected_count: fixture.input.hidden_selected_count,
    })
}

#[test]
fn protected_collection_truth_fixtures_project_search_views() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "collection truth fixtures must exist");

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CollectionTruthFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "collection_truth_alpha_case");
        assert_eq!(fixture.schema_version, 1);

        let saved_view_findings = fixture.input.saved_view.validate_portability();
        assert_eq!(
            saved_view_findings.len(),
            fixture.expect.saved_view_findings,
            "{}: saved-view finding count mismatch: {saved_view_findings:?}",
            fixture.case_name
        );
        assert_eq!(
            fixture.input.saved_view.is_degraded(),
            fixture.expect.saved_view_degraded,
            "{}: saved-view degraded state mismatch",
            fixture.case_name
        );

        let view = collection_view_for(&fixture);
        assert_eq!(
            view.surface_family, fixture.expect.surface_family,
            "{}: surface family mismatch",
            fixture.case_name
        );
        assert!(view.surfaces_hidden_narrowing());
        assert_eq!(
            view.hidden_narrowing_labels.len(),
            fixture.expect.hidden_narrowing_count,
            "{}: hidden narrowing count mismatch",
            fixture.case_name
        );
        assert_eq!(
            view.filter_chips.len(),
            fixture.expect.filter_chip_count,
            "{}: filter chip count mismatch",
            fixture.case_name
        );
        assert_eq!(
            view.counters.selected.value,
            Some(fixture.expect.selected_count),
            "{}: selected count mismatch",
            fixture.case_name
        );
        assert_eq!(
            view.counters.blocked.value,
            Some(fixture.expect.blocked_count),
            "{}: blocked count mismatch",
            fixture.case_name
        );
        assert_eq!(
            view.counters.hidden.value,
            Some(fixture.expect.hidden_count),
            "{}: hidden count mismatch",
            fixture.case_name
        );
        assert_eq!(
            view.counters.all_matching.status, fixture.expect.all_matching_status,
            "{}: all-matching status mismatch",
            fixture.case_name
        );
        assert_eq!(
            view.selection_state.hidden_selected_count, fixture.input.hidden_selected_count,
            "{}: hidden selected count mismatch",
            fixture.case_name
        );
        assert!(view
            .selection_state
            .accessibility_summary
            .contains("hidden selected"));
    }
}
