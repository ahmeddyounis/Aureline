//! Headless inspector and regenerator for the M5 cross-surface event-reuse
//! packet.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the consumer-parity fixture corpus from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- ai-evidence
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- incident-packet
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_cross_surface_event_reuse -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_cross_surface_event_reuse_input, seeded_cross_surface_event_reuse_packet,
    ConsumerSurface, CrossSurfaceEventReusePacket, CrossSurfaceEventReusePacketInput,
    CrossSurfaceFlowKind, ReuseEvidenceSurface, CROSS_SURFACE_EVENT_REUSE_AI_EVIDENCE_ID,
    CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_ID, CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR,
    CROSS_SURFACE_EVENT_REUSE_INCIDENT_PACKET_ID, CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF,
    CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/cross-surface-event-reuse";

const CASES: [(&str, &str, &str); 7] = [
    (
        "baseline_stable.json",
        "none",
        "Canonical packet binds one shared execution history to every major M5 consumer and proves the reopen, export, rerun-review, and evidence-link flows all point back to the same authoritative event objects.",
    ),
    (
        "consumer_reconstructs_from_logs_blocks_stable.json",
        "consumer_reconstructs_from_logs",
        "The coverage/flaky/snapshot surface reconstructs its own history from rendered logs instead of reading the shared event objects.",
    ),
    (
        "consumer_forks_history_blocks_stable.json",
        "consumer_forks_history",
        "The test tree forks a private session history instead of reading the one shared execution history.",
    ),
    (
        "consumer_rewrites_ids_blocks_stable.json",
        "consumer_rewrites_ids",
        "The notebook run surface rewrites stable event/trace ids, so the same run no longer joins across surfaces.",
    ),
    (
        "consumer_binding_missing_blocks_stable.json",
        "consumer_binding_missing",
        "A claimed consumer surface has no binding, so the reuse contract would silently shrink.",
    ),
    (
        "flow_target_missing_blocks_stable.json",
        "flow_target_missing",
        "A reopen flow points at an authoritative event id that is not in the shared history.",
    ),
    (
        "flow_drops_provenance_blocks_stable.json",
        "flow_drops_provenance",
        "An evidence-link flow drops provenance across the surface boundary.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_cross_surface_event_reuse_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(
            &packet.support_export(CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("ai-evidence") => print_json(&packet.evidence_join(
            ReuseEvidenceSurface::AiEvidence,
            CROSS_SURFACE_EVENT_REUSE_AI_EVIDENCE_ID,
            exported_at(),
        )),
        Some("incident-packet") => print_json(&packet.evidence_join(
            ReuseEvidenceSurface::IncidentPacket,
            CROSS_SURFACE_EVENT_REUSE_INCIDENT_PACKET_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_ID, exported_at()),
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

fn regenerate(root: &Path, packet: &CrossSurfaceEventReusePacket) {
    write_json(
        &root.join(CROSS_SURFACE_EVENT_REUSE_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("ai_evidence.json"),
        &packet.evidence_join(
            ReuseEvidenceSurface::AiEvidence,
            CROSS_SURFACE_EVENT_REUSE_AI_EVIDENCE_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("incident_packet.json"),
        &packet.evidence_join(
            ReuseEvidenceSurface::IncidentPacket,
            CROSS_SURFACE_EVENT_REUSE_INCIDENT_PACKET_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = CrossSurfaceEventReusePacket::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_cross_surface_event_reuse_case",
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
                "consumer_surface_tokens": mutated.consumer_surface_tokens(),
                "flow_kind_tokens": mutated.flow_kind_tokens(),
                "source_kind_tokens": mutated.source_kind_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(
            &root
                .join(CROSS_SURFACE_EVENT_REUSE_FIXTURE_DIR)
                .join(file_name),
            &fixture,
        );
    }
}

fn mutated_input(mutation: &str) -> CrossSurfaceEventReusePacketInput {
    let mut input = current_stable_cross_surface_event_reuse_input();
    match mutation {
        "none" => {}
        "consumer_reconstructs_from_logs" => {
            for binding in &mut input.consumer_bindings {
                if binding.surface == ConsumerSurface::CoverageFlakySnapshot {
                    binding.reconstructs_from_logs = true;
                }
            }
        }
        "consumer_forks_history" => {
            for binding in &mut input.consumer_bindings {
                if binding.surface == ConsumerSurface::TestTree {
                    binding.reads_shared_history = false;
                }
            }
        }
        "consumer_rewrites_ids" => {
            for binding in &mut input.consumer_bindings {
                if binding.surface == ConsumerSurface::NotebookRun {
                    binding.preserves_stable_ids = false;
                }
            }
        }
        "consumer_binding_missing" => {
            input
                .consumer_bindings
                .retain(|binding| binding.surface != ConsumerSurface::NotebookRun);
        }
        "flow_target_missing" => {
            for flow in &mut input.cross_surface_flows {
                if flow.flow_kind == CrossSurfaceFlowKind::Reopen {
                    flow.authoritative_event_id = "event:does-not-exist".to_owned();
                }
            }
        }
        "flow_drops_provenance" => {
            for flow in &mut input.cross_surface_flows {
                if flow.flow_kind == CrossSurfaceFlowKind::EvidenceLink {
                    flow.preserves_provenance = false;
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
