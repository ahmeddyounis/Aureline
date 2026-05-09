use std::fs;
use std::path::PathBuf;

use aureline_telemetry::hot_path_metrics::HotPathMetricsRecord;

#[test]
fn hot_path_trace_reference_fixture_contains_required_milestones() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture_path = repo_root.join("fixtures/perf/hot_path_trace_reference.json");
    let raw = fs::read_to_string(&fixture_path).expect("fixture should be readable");
    let record: HotPathMetricsRecord =
        serde_json::from_str(&raw).expect("fixture should parse as HotPathMetricsRecord");

    record
        .validate_minimum_required()
        .expect("fixture should include required milestones");

    assert!(
        record.counters.process_start_marks >= 1,
        "expected at least one process_start mark"
    );
    assert!(
        record.counters.spans_errored >= 1,
        "expected at least one errored span for failure-drill coverage"
    );
}

#[test]
fn fixture_validation_fails_when_required_milestone_missing() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture_path = repo_root.join("fixtures/perf/hot_path_trace_reference.json");
    let raw = fs::read_to_string(&fixture_path).expect("fixture should be readable");
    let mut record: HotPathMetricsRecord =
        serde_json::from_str(&raw).expect("fixture should parse as HotPathMetricsRecord");

    record.events.retain(|ev| {
        ev.journey_segment_id.as_ref() != "seg.input_to_paint.ui_dispatch.keystroke_to_paint"
    });

    let err = record
        .validate_minimum_required()
        .expect_err("expected validation failure");
    assert!(
        err.missing
            .iter()
            .any(|id| *id == "seg.input_to_paint.ui_dispatch.keystroke_to_paint"),
        "expected keystroke_to_paint milestone to be missing"
    );
}
