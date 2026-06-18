//! Headless inspector and regenerator for the M5 task-event first-consumers
//! packet.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the event-envelope fixture corpus from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_envelope_bus
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_envelope_bus -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_envelope_bus -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_envelope_bus -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_envelope_bus -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_envelope_bus -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_task_event_first_consumers_input, seeded_task_event_first_consumers_packet,
    BuildTestInteropConfidence, BuildTestInteropPayloadKind, BuildTestInteropSourceKind,
    TaskEventFirstConsumersPacket, TaskEventFirstConsumersPacketInput, TaskEventSurface,
    TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_ID, TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR,
    TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF, TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/event-envelope-first-consumers";

const CASES: [(&str, &str, &str); 8] = [
    (
        "baseline_stable.json",
        "none",
        "Canonical packet binds the native-first record history, the replay-stable trace summaries, and the seven consumer-surface projections.",
    ),
    (
        "lane_missing_events_blocks_stable.json",
        "lane_missing_test_session",
        "The test session lane loses every canonical record and would fall back to log-only event truth.",
    ),
    (
        "heuristic_overclaims_blocks_stable.json",
        "heuristic_overclaims",
        "The pipeline heuristic shadow raises its confidence to high, masquerading as native truth.",
    ),
    (
        "payload_kind_mismatch_blocks_stable.json",
        "payload_kind_mismatch",
        "A finished test record mislabels its payload class as lifecycle.",
    ),
    (
        "replay_sequence_collision_blocks_stable.json",
        "sequence_collision",
        "Two records in one trace share a sequence number so replay order is ambiguous.",
    ),
    (
        "downgrade_inconsistent_blocks_stable.json",
        "downgrade_inconsistent",
        "The heuristic shadow keeps its downgrade reason but drops the visible downgrade flag.",
    ),
    (
        "projection_drops_truth_blocks_stable.json",
        "projection_drops_truth",
        "The pipeline surface projection stops preserving confidence.",
    ),
    (
        "export_cannot_explain_blocks_stable.json",
        "export_cannot_explain",
        "The support-export surface can no longer explain source and confidence without parsing.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_task_event_first_consumers_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(
            &packet.support_export(TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_ID, exported_at()),
        ),
        Some("compact") => {
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match packet.validate() {
            findings if findings.is_empty() => println!("ok"),
            findings => {
                for finding in &findings {
                    eprintln!("error: {}", finding.finding_kind.as_str());
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            std::process::exit(2);
        }
    }
}

fn regenerate(root: &Path, packet: &TaskEventFirstConsumersPacket) {
    write_json(
        &root.join(TASK_EVENT_FIRST_CONSUMERS_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(TASK_EVENT_FIRST_CONSUMERS_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(TASK_EVENT_FIRST_CONSUMERS_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = TaskEventFirstConsumersPacket::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_task_event_first_consumers_case",
            "schema_version": 1,
            "case_name": case_name,
            "scenario": scenario,
            "mutation": mutation,
            "expect": {
                "promotion_state": mutated.promotion_state.as_str(),
                "validation_finding_count": mutated.validation_findings.len(),
                "expected_finding_kinds": mutated
                    .validation_findings
                    .iter()
                    .map(|f| f.finding_kind.as_str())
                    .collect::<Vec<_>>(),
                "surface_tokens": mutated.surface_tokens(),
                "source_kind_tokens": mutated.source_kind_tokens(),
                "payload_kind_tokens": mutated.payload_kind_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(
            &root
                .join(TASK_EVENT_FIRST_CONSUMERS_FIXTURE_DIR)
                .join(file_name),
            &fixture,
        );
    }
}

fn mutated_input(mutation: &str) -> TaskEventFirstConsumersPacketInput {
    let mut input = current_stable_task_event_first_consumers_input();
    match mutation {
        "none" => {}
        "lane_missing_test_session" => {
            input
                .events
                .retain(|event| event.producer_lane != TaskEventSurface::TestSession);
        }
        "heuristic_overclaims" => {
            for event in &mut input.events {
                if event.source_kind == BuildTestInteropSourceKind::HeuristicParser {
                    event.confidence = BuildTestInteropConfidence::High;
                }
            }
        }
        "payload_kind_mismatch" => {
            for event in &mut input.events {
                if event.event_id == "event:test:finished" {
                    event.payload_kind = BuildTestInteropPayloadKind::Lifecycle;
                }
            }
        }
        "sequence_collision" => {
            for event in &mut input.events {
                if event.event_id == "event:task:progress" {
                    event.sequence = 2;
                }
            }
        }
        "downgrade_inconsistent" => {
            for event in &mut input.events {
                if event.event_id == "event:pipeline:diagnostic-shadow" {
                    event.downgraded = false;
                }
            }
        }
        "projection_drops_truth" => {
            for projection in &mut input.surface_projections {
                if projection.surface == TaskEventSurface::Pipeline {
                    projection.preserves_confidence = false;
                }
            }
        }
        "export_cannot_explain" => {
            for projection in &mut input.surface_projections {
                if projection.surface == TaskEventSurface::SupportExport {
                    projection.explains_source_and_confidence = false;
                }
            }
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn exported_at() -> &'static str {
    "2026-06-17T00:01:00Z"
}

fn print_json(value: &impl serde::Serialize) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("serialize JSON")
    );
}

fn write_json(path: &PathBuf, value: &impl serde::Serialize) {
    ensure_parent(path);
    let payload = serde_json::to_string_pretty(value).expect("serialize JSON");
    std::fs::write(path, format!("{payload}\n")).expect("write JSON");
}

fn write_text(path: &PathBuf, body: &str) {
    ensure_parent(path);
    std::fs::write(path, format!("{body}\n")).expect("write text");
}

fn ensure_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create artifact directory");
    }
}
