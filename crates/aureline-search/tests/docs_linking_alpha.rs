//! Fixture-driven coverage for docs-linked search rows.
//!
//! The fixtures under `fixtures/docs/symbol_linked_refs_alpha/` pin the first
//! docs/help lane that links product symbols to docs anchors, reuses planner
//! result IDs, opens citation evidence, and discloses stale examples or docs
//! suggestions without turning generated copy into authority.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{
    DocsLinkedSearchInputs, DocsLinkedSearchProjection, PlannerDataPath, PlannerResultTruthClass,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsLinkedSearchInputs,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    query_session_surface: String,
    result_set_surface: String,
    rows: Vec<ExpectedRow>,
}

#[derive(Debug, Deserialize)]
struct ExpectedRow {
    result_id: String,
    canonical_id: String,
    answered_by: String,
    result_truth_class: String,
    ranking_reason_classes: Vec<String>,
    partial_truth_causes: Vec<String>,
    exact_anchor_ref: String,
    doc_kind: String,
    source_class: String,
    source_version: String,
    locality: String,
    freshness: String,
    version_match_state: String,
    citation_availability: String,
    citation_drawer_opens_evidence_view: bool,
    project_vs_vendor_truth_cue: String,
    missing_anchor_downgrade_state: Option<String>,
    stale_example_trigger: Option<String>,
    stale_example_validation_freshness: Option<String>,
    suggestion_trigger: Option<String>,
    suggestion_evidence_state: Option<String>,
    suggestion_publish_boundary_state: Option<String>,
    suggestion_target_branch: Option<String>,
    suggestion_target_channel: Option<String>,
}

#[test]
fn docs_linking_alpha_cases_match_expected_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/docs/symbol_linked_refs_alpha");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one docs symbol-linked fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "docs_linking_alpha_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let projection = DocsLinkedSearchProjection::from_inputs(fixture.input);
        projection
            .validate_acceptance()
            .unwrap_or_else(|findings| panic!("fixture {path:?} failed validation: {findings:#?}"));

        assert_eq!(
            projection.query_session.surface.as_str(),
            fixture.expect.query_session_surface,
            "query surface mismatch in {path:?}"
        );
        assert_eq!(
            projection.result_set.surface.as_str(),
            fixture.expect.result_set_surface,
            "result-set surface mismatch in {path:?}"
        );
        assert_eq!(
            projection.rows.len(),
            fixture.expect.rows.len(),
            "row count mismatch in {path:?}"
        );

        for expected in &fixture.expect.rows {
            let row = projection
                .rows
                .iter()
                .find(|row| row.result_id == expected.result_id)
                .unwrap_or_else(|| {
                    panic!("missing docs-linked row {} in {path:?}", expected.result_id)
                });
            assert_eq!(row.canonical_id, expected.canonical_id);
            assert_data_path(row.answered_by, &expected.answered_by, &path);
            assert_eq!(row.result_truth_class.as_str(), expected.result_truth_class);
            assert_eq!(row.ranking_reason_classes, expected.ranking_reason_classes);
            assert_eq!(row.partial_truth_causes, expected.partial_truth_causes);
            assert_eq!(row.exact_anchor.exact_anchor_ref, expected.exact_anchor_ref);
            assert_eq!(row.doc_kind.as_str(), expected.doc_kind);
            assert_eq!(row.source_class.as_str(), expected.source_class);
            assert_eq!(row.exact_anchor.source_version, expected.source_version);
            assert_eq!(row.locality.as_str(), expected.locality);
            assert_eq!(row.freshness.as_str(), expected.freshness);
            assert_eq!(
                row.version_match_state.as_str(),
                expected.version_match_state
            );
            assert_eq!(
                row.citation_availability.as_str(),
                expected.citation_availability
            );
            assert_eq!(
                row.citation_drawer_hook.opens_evidence_view,
                expected.citation_drawer_opens_evidence_view
            );
            assert_eq!(
                row.project_vs_vendor_truth_cue.as_str(),
                expected.project_vs_vendor_truth_cue
            );
            assert_eq!(
                row.missing_anchor_downgrade_state
                    .map(|state| state.as_str().to_string()),
                expected.missing_anchor_downgrade_state
            );
            assert_eq!(
                row.stale_example_signal
                    .as_ref()
                    .map(|signal| signal.trigger_class.as_str().to_string()),
                expected.stale_example_trigger
            );
            assert_eq!(
                row.stale_example_signal
                    .as_ref()
                    .map(|signal| signal.validation_freshness.as_str().to_string()),
                expected.stale_example_validation_freshness
            );
            assert_eq!(
                row.suggestion_card
                    .as_ref()
                    .map(|card| card.trigger_class.as_str().to_string()),
                expected.suggestion_trigger
            );
            assert_eq!(
                row.suggestion_card
                    .as_ref()
                    .map(|card| card.evidence_state.as_str().to_string()),
                expected.suggestion_evidence_state
            );
            assert_eq!(
                row.suggestion_card
                    .as_ref()
                    .map(|card| card.publish_boundary_state.as_str().to_string()),
                expected.suggestion_publish_boundary_state
            );
            assert_eq!(
                row.suggestion_card
                    .as_ref()
                    .map(|card| card.target_branch.clone()),
                expected.suggestion_target_branch
            );
            assert_eq!(
                row.suggestion_card
                    .as_ref()
                    .map(|card| card.target_channel.clone()),
                expected.suggestion_target_channel
            );

            let support_row = projection
                .support_export
                .rows
                .iter()
                .find(|row| row.result_id == expected.result_id)
                .unwrap_or_else(|| {
                    panic!(
                        "support export missing docs-linked row {} in {path:?}",
                        expected.result_id
                    )
                });
            assert_eq!(support_row.exact_anchor_ref, expected.exact_anchor_ref);
            assert_eq!(support_row.source_class_token, expected.source_class);
            assert_eq!(support_row.freshness_token, expected.freshness);
            assert_eq!(
                support_row.citation_availability_token,
                expected.citation_availability
            );
            assert_eq!(
                support_row.ranking_reason_classes,
                expected.ranking_reason_classes
            );
        }
    }
}

fn assert_data_path(actual: PlannerDataPath, expected: &str, path: &Path) {
    assert_eq!(
        actual.as_str(),
        expected,
        "answered_by mismatch in {path:?}"
    );
}

#[test]
fn docs_result_truth_token_is_imported() {
    assert_eq!(PlannerResultTruthClass::Imported.as_str(), "imported");
}
