//! Headless inspector and regenerator for the M5 replay bundle.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the replay-bundle fixture corpus from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- ai-evidence
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- incident-packet
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_replay_bundles -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_replay_bundle_input, seeded_replay_bundle, BuildTestInteropSourceKind,
    RawPayloadRetentionClass, ReplayBundle, ReplayBundleInput, ReplayJoinSurface,
    REPLAY_BUNDLE_AI_EVIDENCE_ID, REPLAY_BUNDLE_CLI_HEADLESS_ID, REPLAY_BUNDLE_FIXTURE_DIR,
    REPLAY_BUNDLE_INCIDENT_PACKET_ID, REPLAY_BUNDLE_PACKET_ARTIFACT_REF,
    REPLAY_BUNDLE_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/raw-plus-normalized-replay";
const METADATA_REF: &str = "raw:event:task:queued";

const CASES: [(&str, &str, &str); 7] = [
    (
        "baseline_stable.json",
        "none",
        "Canonical bundle binds the normalized history to the typed raw-payload lineage, joins both halves into the replay, support, incident, and AI surfaces, and proves replay stays stable under every required delivery anomaly.",
    ),
    (
        "lineage_entry_missing_blocks_stable.json",
        "lineage_entry_missing",
        "A normalized event cites a raw-payload reference that no lineage entry backs, breaking the raw-to-normalized chain.",
    ),
    (
        "raw_payload_unbounded_blocks_stable.json",
        "raw_payload_unbounded",
        "A metadata-only lineage entry retains far more than its byte bound, violating the typed, bounded retention guardrail.",
    ),
    (
        "retention_exposes_secret_blocks_stable.json",
        "retention_exposes_secret",
        "The approval-gated debug payload is marked support- and AI-safe, which would let an export expose a secret-bearing raw payload.",
    ),
    (
        "lineage_source_mismatch_blocks_stable.json",
        "lineage_source_mismatch",
        "A lineage entry's source kind disagrees with the normalized event that cites it, flattening provenance.",
    ),
    (
        "join_missing_blocks_stable.json",
        "join_missing",
        "The incident-packet join projection is absent, so an execution surface would lose the raw-to-normalized join.",
    ),
    (
        "join_drops_redaction_blocks_stable.json",
        "join_drops_redaction",
        "The AI evidence join stops honoring raw-payload redaction, which would expose gated references.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let bundle = seeded_replay_bundle();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &bundle),
        Some("packet") => print_json(&bundle),
        Some("support-export") => {
            print_json(&bundle.support_export(REPLAY_BUNDLE_SUPPORT_EXPORT_ID, exported_at()))
        }
        Some("ai-evidence") => print_json(&bundle.evidence_join(
            ReplayJoinSurface::AiEvidence,
            REPLAY_BUNDLE_AI_EVIDENCE_ID,
            exported_at(),
        )),
        Some("incident-packet") => print_json(&bundle.evidence_join(
            ReplayJoinSurface::IncidentPacket,
            REPLAY_BUNDLE_INCIDENT_PACKET_ID,
            exported_at(),
        )),
        Some("cli-headless") => {
            print_json(&bundle.cli_headless_view(REPLAY_BUNDLE_CLI_HEADLESS_ID, exported_at()))
        }
        Some("compact") => {
            for line in bundle.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match bundle.validate() {
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

fn regenerate(root: &Path, bundle: &ReplayBundle) {
    write_json(&root.join(REPLAY_BUNDLE_PACKET_ARTIFACT_REF), bundle);
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &bundle.support_export(REPLAY_BUNDLE_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("ai_evidence.json"),
        &bundle.evidence_join(
            ReplayJoinSurface::AiEvidence,
            REPLAY_BUNDLE_AI_EVIDENCE_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("incident_packet.json"),
        &bundle.evidence_join(
            ReplayJoinSurface::IncidentPacket,
            REPLAY_BUNDLE_INCIDENT_PACKET_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &bundle.cli_headless_view(REPLAY_BUNDLE_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = bundle.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = ReplayBundle::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_replay_bundle_case",
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
                "retention_class_tokens": mutated.retention_class_tokens(),
                "source_kind_tokens": mutated.source_kind_tokens(),
                "failure_mode_tokens": mutated.failure_mode_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(
            &root.join(REPLAY_BUNDLE_FIXTURE_DIR).join(file_name),
            &fixture,
        );
    }
}

fn mutated_input(mutation: &str) -> ReplayBundleInput {
    let mut input = current_stable_replay_bundle_input();
    match mutation {
        "none" => {}
        "lineage_entry_missing" => {
            input
                .raw_lineage
                .retain(|entry| entry.raw_payload_ref != METADATA_REF);
        }
        "raw_payload_unbounded" => {
            for entry in &mut input.raw_lineage {
                if entry.raw_payload_ref == METADATA_REF {
                    entry.payload_byte_len = 1_000_000;
                }
            }
        }
        "retention_exposes_secret" => {
            for entry in &mut input.raw_lineage {
                if entry.retention_class == RawPayloadRetentionClass::SupportApprovalRequired {
                    entry.support_export_safe = true;
                    entry.ai_evidence_safe = true;
                }
            }
        }
        "lineage_source_mismatch" => {
            for entry in &mut input.raw_lineage {
                if entry.raw_payload_ref == METADATA_REF {
                    entry.source_kind = BuildTestInteropSourceKind::Bsp;
                }
            }
        }
        "join_missing" => {
            input
                .join_projections
                .retain(|projection| projection.surface != ReplayJoinSurface::IncidentPacket);
        }
        "join_drops_redaction" => {
            for projection in &mut input.join_projections {
                if projection.surface == ReplayJoinSurface::AiEvidence {
                    projection.honors_retention_redaction = false;
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
