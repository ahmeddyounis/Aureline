//! Unit coverage for the M5 task-event first-consumers packet: the canonical
//! record history, the replay-stable trace summaries, the per-surface
//! projections, and the CLI/headless and support exports.

use super::*;

fn stable() -> TaskEventFirstConsumersPacket {
    seeded_task_event_first_consumers_packet()
}

#[test]
fn seed_packet_validates_clean_and_is_stable() {
    let packet = stable();
    assert_eq!(packet.record_kind, TASK_EVENT_FIRST_CONSUMERS_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        TASK_EVENT_FIRST_CONSUMERS_SCHEMA_VERSION
    );
    assert!(
        packet.validate().is_empty(),
        "seed packet must validate clean: {:?}",
        packet.validate()
    );
    assert!(packet.is_stable());
    assert_eq!(packet.promotion_state.as_str(), "stable");
}

#[test]
fn every_emitting_lane_carries_canonical_records() {
    let packet = stable();
    for lane in TaskEventSurface::EMITTING {
        let count = packet
            .events
            .iter()
            .filter(|event| event.producer_lane == lane)
            .count();
        assert!(count > 0, "{} must carry canonical records", lane.as_str());
    }
}

#[test]
fn all_seven_surfaces_are_projected() {
    let packet = stable();
    assert_eq!(
        packet.surface_tokens(),
        vec![
            "notebook_run",
            "task_center",
            "test_session",
            "debug_session",
            "pipeline",
            "support_export",
            "cli_headless",
        ]
    );
}

#[test]
fn record_carries_every_spec_named_field() {
    let packet = stable();
    let event = packet
        .events
        .iter()
        .find(|event| event.event_id == "event:notebook:test")
        .expect("seed has the notebook structured test record");
    assert_eq!(event.workspace_id, "workspace:checkout");
    assert_eq!(event.target_id, "target:checkout:notebook");
    assert_eq!(event.source_kind.as_str(), "structured-output");
    assert_eq!(event.confidence.as_str(), "medium-high");
    assert_eq!(event.execution_context_id, "exec-context:local:checkout");
    assert_eq!(event.payload_kind.as_str(), "test");
    assert_eq!(event.raw_payload_ref, "raw:event:notebook:test");
    assert!(!event.provenance.adapter_id.trim().is_empty());
}

#[test]
fn source_and_payload_vocabularies_are_covered() {
    let packet = stable();
    assert_eq!(
        packet.source_kind_tokens(),
        vec![
            "native",
            "bsp",
            "bazel-bep",
            "structured-output",
            "heuristic-parser"
        ]
    );
    assert_eq!(
        packet.payload_kind_tokens(),
        vec![
            "lifecycle",
            "progress",
            "diagnostic",
            "test",
            "artifact",
            "debug"
        ]
    );
}

#[test]
fn replay_order_is_stable_under_reordering() {
    let mut input = current_stable_task_event_first_consumers_input();
    let forward = TaskEventFirstConsumersPacket::materialize(input.clone());
    input.events.reverse();
    let reversed = TaskEventFirstConsumersPacket::materialize(input);

    let forward_ids: Vec<&str> = forward
        .replay_ordered()
        .iter()
        .map(|event| event.event_id.as_str())
        .collect();
    let reversed_ids: Vec<&str> = reversed
        .replay_ordered()
        .iter()
        .map(|event| event.event_id.as_str())
        .collect();

    assert_eq!(forward_ids, reversed_ids, "replay order must be stable");
    assert_eq!(forward.replay_digest(), reversed.replay_digest());
    assert_eq!(forward.trace_summaries, reversed.trace_summaries);
}

#[test]
fn trace_window_is_a_contiguous_replay_slice() {
    let packet = stable();
    let full: Vec<&str> = packet
        .replay_ordered()
        .iter()
        .filter(|event| event.trace_id == "trace:task:build")
        .map(|event| event.event_id.as_str())
        .collect();
    let window: Vec<&str> = packet
        .trace_window("trace:task:build", 1, 2)
        .iter()
        .map(|event| event.event_id.as_str())
        .collect();
    assert_eq!(window, full[1..3].to_vec());
}

#[test]
fn cli_headless_view_explains_every_row() {
    let packet = stable();
    let view = packet.cli_headless_view(
        TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_ID,
        "2026-06-17T00:01:00Z",
    );
    assert_eq!(view.rows.len(), packet.events.len());
    assert!(view.every_row_explains());
    assert_eq!(view.replay_digest, packet.replay_digest());
    let shadow = view
        .rows
        .iter()
        .find(|row| row.event_id == "event:pipeline:diagnostic-shadow")
        .expect("view carries the heuristic shadow row");
    assert_eq!(shadow.source_kind, "heuristic-parser");
    assert_eq!(
        shadow.downgrade_reason.as_deref(),
        Some("heuristic_fallback")
    );
    assert!(shadow.explanation.contains("heuristic_fallback"));
}

#[test]
fn lane_without_records_blocks_stable() {
    let mut input = current_stable_task_event_first_consumers_input();
    input
        .events
        .retain(|event| event.producer_lane != TaskEventSurface::TestSession);
    let packet = TaskEventFirstConsumersPacket::materialize(input);
    assert_eq!(packet.promotion_state.as_str(), "blocks_stable");
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == EventBusFindingKind::LaneMissingCanonicalEvents));
}

#[test]
fn heuristic_overclaim_blocks_stable() {
    let mut input = current_stable_task_event_first_consumers_input();
    for event in &mut input.events {
        if event.source_kind == BuildTestEventSourceKind::HeuristicParser {
            event.confidence = BuildTestEventConfidence::High;
        }
    }
    let packet = TaskEventFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == EventBusFindingKind::EventConfidenceOverclaim));
    assert!(!packet.is_stable());
}

#[test]
fn payload_kind_mismatch_blocks_stable() {
    let mut input = current_stable_task_event_first_consumers_input();
    for event in &mut input.events {
        if event.event_id == "event:test:finished" {
            event.payload_kind = BuildTestPayloadKind::Lifecycle;
        }
    }
    let packet = TaskEventFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == EventBusFindingKind::EventPayloadKindMismatch));
}

#[test]
fn sequence_collision_blocks_stable() {
    let mut input = current_stable_task_event_first_consumers_input();
    for event in &mut input.events {
        if event.event_id == "event:task:progress" {
            event.sequence = 2;
        }
    }
    let packet = TaskEventFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == EventBusFindingKind::ReplaySequenceCollision));
}

#[test]
fn export_that_cannot_explain_blocks_stable() {
    let mut input = current_stable_task_event_first_consumers_input();
    for projection in &mut input.surface_projections {
        if projection.surface == TaskEventSurface::SupportExport {
            projection.explains_source_and_confidence = false;
        }
    }
    let packet = TaskEventFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == EventBusFindingKind::ExportCannotExplain));
}

#[test]
fn projection_that_drops_truth_blocks_stable() {
    let mut input = current_stable_task_event_first_consumers_input();
    for projection in &mut input.surface_projections {
        if projection.surface == TaskEventSurface::Pipeline {
            projection.preserves_confidence = false;
        }
    }
    let packet = TaskEventFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == EventBusFindingKind::SurfaceProjectionDropsTruth));
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = stable();
    let export = packet.support_export(
        TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
        "2026-06-17T00:01:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    let json = serde_json::to_string(&export).expect("serialize export");
    let round: TaskEventFirstConsumersSupportExport =
        serde_json::from_str(&json).expect("deserialize export");
    assert_eq!(round, export);
}

#[test]
fn finding_tokens_are_pinned() {
    assert_eq!(
        EventBusFindingKind::LaneMissingCanonicalEvents.as_str(),
        "lane_missing_canonical_events"
    );
    assert_eq!(
        EventBusFindingKind::ExportCannotExplain.as_str(),
        "export_cannot_explain"
    );
    assert_eq!(
        EventBusFindingKind::ReplaySequenceCollision.as_str(),
        "replay_sequence_collision"
    );
}
