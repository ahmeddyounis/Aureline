//! Headless inspector and regenerator for the M5 event-interop tooling-profile
//! certification matrix.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the fixture mutation cases from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- ai-evidence
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- incident-packet
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_event_interop_certification -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_event_interop_certification_input, seeded_event_interop_certification_packet,
    BuildTestInteropConfidence, CertificationEvidenceSurface, ConsumerTruthSource,
    EventInteropCertificationPacket, EventInteropCertificationPacketInput, ToolingProfile,
    EVENT_INTEROP_CERTIFICATION_AI_EVIDENCE_ID, EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_ID,
    EVENT_INTEROP_CERTIFICATION_INCIDENT_PACKET_ID,
    EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF, EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/event-interop-certification";
const FIXTURE_DIR: &str = "fixtures/tooling/m5/event-interop-certification";

/// Each case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 12] = [
    (
        "baseline_stable.json",
        "none",
        "Every claimed M5 run/test/debug/pipeline/notebook/coverage profile reads the canonical event envelope with current proof, so the certification index shows every profile claimable.",
    ),
    (
        "private_session_history_blocks_stable.json",
        "private_session_history",
        "The task center run profile reconstructs truth from a forked private session history instead of the canonical event envelope, so its interop claim is blocked.",
    ),
    (
        "missing_evidence_ref_blocks_stable.json",
        "missing_evidence_ref",
        "The test session profile cites no upstream evidence packet, so its claim rests on no machine-readable proof and is blocked.",
    ),
    (
        "adapter_hierarchy_missing_blocks_stable.json",
        "adapter_hierarchy_missing",
        "The debug session profile evidences no native-first capability handshake, so its adapter hierarchy cannot be certified.",
    ),
    (
        "confidence_overclaim_blocks_stable.json",
        "confidence_overclaim",
        "The pipeline overlay profile claims more than low confidence for its imported heuristic path, overclaiming a best-effort fallback.",
    ),
    (
        "fallback_reason_missing_blocks_stable.json",
        "fallback_reason_missing",
        "The pipeline overlay profile names no fallback reason for its degraded imported path, so a degraded capability is undisclosed at the reason level.",
    ),
    (
        "raw_payload_not_retained_blocks_stable.json",
        "raw_payload_not_retained",
        "The coverage intelligence profile drops its retained raw payload, so support and replay can no longer recover the original adapter payload.",
    ),
    (
        "replay_unstable_blocks_stable.json",
        "replay_unstable",
        "The notebook run profile stops replaying deterministically, so the canonical envelopes can no longer be re-derived.",
    ),
    (
        "export_parity_broken_blocks_stable.json",
        "export_parity_broken",
        "The task center run profile breaks export parity, so the support/release/AI projections would lose source, confidence, or refs.",
    ),
    (
        "degraded_state_not_disclosed_blocks_stable.json",
        "degraded_state_not_disclosed",
        "The pipeline overlay profile hides its degraded capability state from the consumer surfaces.",
    ),
    (
        "missing_profile_blocks_stable.json",
        "missing_profile",
        "The coverage intelligence profile is absent entirely, so the certification matrix would silently shrink to the profiles that still happen to pass.",
    ),
    (
        "evidence_stale_narrows_below_stable.json",
        "evidence_stale",
        "The coverage intelligence profile proof has aged past its freshness window, so its interop claim narrows below stable instead of staying green on aged proof.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_event_interop_certification_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(
            &packet.support_export(EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("ai-evidence") => print_json(&packet.evidence_join(
            CertificationEvidenceSurface::AiEvidence,
            EVENT_INTEROP_CERTIFICATION_AI_EVIDENCE_ID,
            exported_at(),
        )),
        Some("incident-packet") => print_json(&packet.evidence_join(
            CertificationEvidenceSurface::IncidentPacket,
            EVENT_INTEROP_CERTIFICATION_INCIDENT_PACKET_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_ID, exported_at()),
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

fn regenerate(root: &Path, packet: &EventInteropCertificationPacket) {
    write_json(
        &root.join(EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("ai_evidence.json"),
        &packet.evidence_join(
            CertificationEvidenceSurface::AiEvidence,
            EVENT_INTEROP_CERTIFICATION_AI_EVIDENCE_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("incident_packet.json"),
        &packet.evidence_join(
            CertificationEvidenceSurface::IncidentPacket,
            EVENT_INTEROP_CERTIFICATION_INCIDENT_PACKET_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = EventInteropCertificationPacket::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_event_interop_certification_case",
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
                "profile_tokens": mutated.profile_tokens(),
                "source_kind_tokens": mutated.source_kind_tokens(),
                "consumer_truth_source_tokens": mutated.consumer_truth_source_tokens(),
                "dimension_tokens": mutated.dimension_tokens(),
                "claimable_profiles": mutated.certification_index.claimable_profiles.clone(),
                "narrowed_profiles": mutated.certification_index.narrowed_profiles.clone(),
                "blocked_profiles": mutated.certification_index.blocked_profiles.clone(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(&root.join(FIXTURE_DIR).join(file_name), &fixture);
    }
}

fn mutated_input(mutation: &str) -> EventInteropCertificationPacketInput {
    let mut input = current_stable_event_interop_certification_input();
    match mutation {
        "none" => {}
        "private_session_history" => {
            profile(&mut input, ToolingProfile::TaskCenterRun).consumer_truth_source =
                ConsumerTruthSource::PrivateSessionHistory;
        }
        "missing_evidence_ref" => {
            profile(&mut input, ToolingProfile::TestSession).evidence_refs = Vec::new();
        }
        "adapter_hierarchy_missing" => {
            profile(&mut input, ToolingProfile::DebugSession).capability_packet_ref = String::new();
        }
        "confidence_overclaim" => {
            profile(&mut input, ToolingProfile::PipelineOverlay).observed_confidence =
                BuildTestInteropConfidence::High;
        }
        "fallback_reason_missing" => {
            profile(&mut input, ToolingProfile::PipelineOverlay).fallback_reason = None;
        }
        "raw_payload_not_retained" => {
            profile(&mut input, ToolingProfile::CoverageIntelligence)
                .raw_private_material_excluded = false;
        }
        "replay_unstable" => {
            profile(&mut input, ToolingProfile::NotebookRun).replay_stable = false;
        }
        "export_parity_broken" => {
            profile(&mut input, ToolingProfile::TaskCenterRun).export_parity_preserved = false;
        }
        "degraded_state_not_disclosed" => {
            profile(&mut input, ToolingProfile::PipelineOverlay).degraded_state_disclosed = false;
        }
        "missing_profile" => {
            input
                .profiles
                .retain(|row| row.profile != ToolingProfile::CoverageIntelligence);
        }
        "evidence_stale" => {
            let row = profile(&mut input, ToolingProfile::CoverageIntelligence);
            row.proof_age_days = row.freshness_window_days + 10;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn profile(
    input: &mut EventInteropCertificationPacketInput,
    profile: ToolingProfile,
) -> &mut aureline_runtime::ToolingProfileCertification {
    input
        .profiles
        .iter_mut()
        .find(|row| row.profile == profile)
        .expect("profile present")
}

fn exported_at() -> &'static str {
    "2026-06-18T00:01:00Z"
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
