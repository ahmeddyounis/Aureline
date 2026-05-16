use std::path::Path;

use aureline_runtime::{
    lane_for_event, TaskConsumerSurfaceClass, TaskDegradationReason, TaskEventBetaCoverageManifest,
    TaskEventBetaLane, TaskEventKind, TaskEventStream, TaskWedgeClass,
    TASK_EVENT_BETA_COVERAGE_MANIFEST_RECORD_KIND, TASK_EVENT_SCHEMA_VERSION,
    TASK_EVENT_STREAM_RECORD_KIND,
};

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/runtime/task_event_beta")
}

fn read_fixture(name: &str) -> String {
    let path = fixture_dir().join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()))
}

#[test]
fn checked_in_manifest_matches_canonical_runtime_manifest() {
    let payload = read_fixture("beta_lane_coverage.json");
    let fixture: TaskEventBetaCoverageManifest =
        serde_json::from_str(&payload).expect("parse beta lane coverage manifest");

    assert_eq!(
        fixture.record_kind,
        TASK_EVENT_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );
    assert_eq!(fixture.task_event_schema_version, TASK_EVENT_SCHEMA_VERSION);

    let canonical = TaskEventBetaCoverageManifest::canonical(
        fixture.manifest_id.clone(),
        fixture.generated_at.clone(),
    );
    assert_eq!(canonical, fixture);
}

#[test]
fn review_and_ai_stream_uses_one_event_grammar_across_lanes() {
    let payload = read_fixture("review_and_ai_stream.json");
    let fixture_stream: TaskEventStream =
        serde_json::from_str(&payload).expect("parse review-and-ai stream");

    assert_eq!(fixture_stream.record_kind, TASK_EVENT_STREAM_RECORD_KIND);
    assert_eq!(
        fixture_stream.task_event_schema_version,
        TASK_EVENT_SCHEMA_VERSION
    );

    let stream = TaskEventStream::from_events(
        fixture_stream.stream_id.clone(),
        fixture_stream.workspace_id.clone(),
        fixture_stream.trace_id.clone(),
        fixture_stream.events.clone(),
    )
    .expect("replay review-and-ai stream");

    let mut lanes_seen: Vec<TaskEventBetaLane> = Vec::new();
    let mut wedges_seen: Vec<TaskWedgeClass> = Vec::new();
    for event in &stream.events {
        let lane = lane_for_event(event);
        if !lanes_seen.contains(&lane) {
            lanes_seen.push(lane);
        }
        if !wedges_seen.contains(&event.identity.wedge) {
            wedges_seen.push(event.identity.wedge);
        }
        assert!(event.raw_envelope.matches_event(event));
    }
    assert!(lanes_seen.contains(&TaskEventBetaLane::Review));
    assert!(lanes_seen.contains(&TaskEventBetaLane::Ai));
    assert!(wedges_seen.contains(&TaskWedgeClass::Review));
    assert!(wedges_seen.contains(&TaskWedgeClass::AiTool));

    let manifest =
        TaskEventBetaCoverageManifest::canonical("task-event-beta:test", "2026-05-15T00:00:00Z");
    assert!(manifest.unclaimed_wedges(&stream).is_empty());
}

#[test]
fn degraded_state_is_visible_as_typed_event_not_console_string() {
    let payload = read_fixture("review_and_ai_stream.json");
    let fixture_stream: TaskEventStream =
        serde_json::from_str(&payload).expect("parse review-and-ai stream");

    let stream = TaskEventStream::from_events(
        fixture_stream.stream_id.clone(),
        fixture_stream.workspace_id.clone(),
        fixture_stream.trace_id.clone(),
        fixture_stream.events.clone(),
    )
    .expect("replay review-and-ai stream");

    let degraded_event = stream
        .events
        .iter()
        .find(|event| event.event_kind == TaskEventKind::DegradedStateReported)
        .expect("stream emits a typed degraded_state_reported event");
    assert_eq!(
        degraded_event.payload.degradation_reason(),
        Some(TaskDegradationReason::AdapterCapabilityDropped)
    );

    let shell = stream.shell_projection();
    let shell_degraded = shell
        .iter()
        .find(|row| row.event_kind == TaskEventKind::DegradedStateReported)
        .expect("shell projection carries the typed degraded row");
    assert!(shell_degraded.needs_attention);
    assert_eq!(
        shell_degraded.degradation_reason,
        Some(TaskDegradationReason::AdapterCapabilityDropped)
    );

    let support = stream.support_export(
        "support-export:task-event-beta:degraded",
        "2026-05-15T00:12:00Z",
    );
    let support_degraded = support
        .events
        .iter()
        .find(|row| row.event_kind == TaskEventKind::DegradedStateReported)
        .expect("support export retains the typed degraded row");
    assert_eq!(
        support_degraded.degradation_reason,
        Some(TaskDegradationReason::AdapterCapabilityDropped)
    );
}

#[test]
fn one_event_stream_feeds_every_consumer_surface() {
    let payload = read_fixture("review_and_ai_stream.json");
    let fixture_stream: TaskEventStream =
        serde_json::from_str(&payload).expect("parse review-and-ai stream");

    let stream = TaskEventStream::from_events(
        fixture_stream.stream_id.clone(),
        fixture_stream.workspace_id.clone(),
        fixture_stream.trace_id.clone(),
        fixture_stream.events.clone(),
    )
    .expect("replay review-and-ai stream");

    let shell = stream.shell_projection();
    let activity = stream.activity_projection();
    let support = stream.support_export("support-export:task-event-beta", "2026-05-15T00:12:00Z");

    assert_eq!(shell.len(), stream.events.len());
    assert!(activity.len() >= 2);
    assert_eq!(support.events.len(), stream.events.len());
    assert_eq!(support.raw_envelopes.len(), stream.events.len());
    for surface in [
        TaskConsumerSurfaceClass::Shell,
        TaskConsumerSurfaceClass::ActivityCenter,
        TaskConsumerSurfaceClass::SupportBundleExport,
    ] {
        assert!(support.consumer_surfaces.contains(&surface));
    }
}
