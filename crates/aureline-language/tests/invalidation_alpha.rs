use std::collections::BTreeSet;
use std::path::Path;

use aureline_language::{
    CacheStatusClass, IncrementalParseBuffer, IncrementalParseInvalidationRecord,
    InvalidationDecisionClass, ParseLifecycleStateClass, ParseQualityClass, ParseRequest,
    ParseRequestClass, SymbolSnapshotRecord, TextEdit,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    cases: Vec<Case>,
    benchmark_expectations: BenchmarkExpectations,
}

#[derive(Debug, Deserialize)]
struct Case {
    label: String,
    language_id: String,
    workspace_relative_path: String,
    initial_source_lines: Vec<String>,
    edit: EditFixture,
    expected_workload: String,
    expected_symbols_after: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EditFixture {
    edit_id: String,
    find_text: String,
    replacement_text: String,
}

#[derive(Debug, Deserialize)]
struct BenchmarkExpectations {
    required_workloads: Vec<String>,
}

#[test]
fn editor_edits_reuse_tree_and_export_symbol_snapshots() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "language_invalidation_alpha_cases");
    assert_eq!(fixture.schema_version, 1);

    let mut observed_workloads = BTreeSet::new();
    for case in &fixture.cases {
        let initial_source = case.initial_source_lines.join("\n");
        let mut buffer = IncrementalParseBuffer::open_with_default_registry(
            initial_request(case),
            initial_source.clone(),
        );
        assert!(
            buffer.latest_parse_session().has_current_full_tree(),
            "{} initial parse should produce a current full tree",
            case.label
        );

        let edit_start = initial_source
            .find(&case.edit.find_text)
            .unwrap_or_else(|| panic!("fixture edit target missing for {}", case.label));
        let edit = TextEdit::replace(
            case.edit.edit_id.clone(),
            edit_start,
            edit_start + case.edit.find_text.len(),
            case.edit.replacement_text.clone(),
        );
        let update = buffer
            .apply_edit(
                edit,
                format!("parse-session:test:{}:edit", case.label),
                case.workspace_relative_path.clone(),
                "2026-05-13T00:00:01Z",
            )
            .unwrap_or_else(|err| panic!("apply edit for {}: {err}", case.label));

        assert_eq!(
            update.parse_session.parse_request_class,
            ParseRequestClass::VisibleEditIncremental
        );
        assert_eq!(
            update.parse_session.parse_state.parse_lifecycle_state_class,
            ParseLifecycleStateClass::Completed
        );
        assert_eq!(
            update.parse_session.parse_state.parse_quality_class,
            ParseQualityClass::FullTree
        );
        assert_eq!(
            update.invalidation_record.decision_class,
            InvalidationDecisionClass::ReusePreviousTree
        );
        assert_eq!(
            update.invalidation_record.cache_status_class,
            CacheStatusClass::InvalidatedByEdit
        );
        assert_eq!(
            update.invalidation_record.edit_operations[0]
                .workload_class
                .as_str(),
            case.expected_workload
        );
        assert_eq!(
            update
                .invalidation_record
                .benchmark_sample
                .workload_class
                .as_str(),
            case.expected_workload
        );
        assert!(
            update
                .invalidation_record
                .benchmark_sample
                .reused_previous_tree
        );
        assert!(update.symbol_snapshot.state.search_consumable);
        assert!(update.symbol_snapshot.export_policy.raw_source_excluded);

        for expected_symbol in &case.expected_symbols_after {
            assert!(
                update
                    .symbol_snapshot
                    .symbol_named(expected_symbol)
                    .is_some(),
                "{} missing symbol {} after edit",
                case.label,
                expected_symbol
            );
        }

        let invalidation_json = serde_json::to_string(&update.invalidation_record)
            .expect("invalidation record serializes");
        let invalidation_round_trip: IncrementalParseInvalidationRecord =
            serde_json::from_str(&invalidation_json).expect("invalidation record deserializes");
        assert_eq!(invalidation_round_trip, update.invalidation_record);

        let symbol_json =
            serde_json::to_string(&update.symbol_snapshot).expect("symbol snapshot serializes");
        let symbol_round_trip: SymbolSnapshotRecord =
            serde_json::from_str(&symbol_json).expect("symbol snapshot deserializes");
        assert_eq!(symbol_round_trip, update.symbol_snapshot);

        observed_workloads.insert(
            update
                .invalidation_record
                .benchmark_sample
                .workload_class
                .as_str()
                .to_string(),
        );
    }

    for workload in fixture.benchmark_expectations.required_workloads {
        assert!(
            observed_workloads.contains(&workload),
            "benchmark workload {workload} should be exercised"
        );
    }
}

fn initial_request(case: &Case) -> ParseRequest {
    ParseRequest::foreground_file(
        format!("parse-session:test:{}:initial", case.label),
        format!("doc:{}", case.workspace_relative_path),
        format!("buffer:{}", case.label),
        1,
        case.language_id.clone(),
        "2026-05-13T00:00:00Z",
    )
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/invalidation_alpha/incremental_parse_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
