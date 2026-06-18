//! Headless inspector and regenerator for the M5 build/test interop conformance
//! corpora and suite.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the four named fixture corpora from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- ai-evidence
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- incident-packet
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_interop_conformance -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_interop_conformance_input, seeded_interop_conformance_packet,
    BuildTestInteropConfidence, ConformanceCase, ConformanceEvidenceSurface, CorpusFamily,
    InteropArchetype, InteropConformancePacket, InteropConformancePacketInput, InteropCorpus,
    INTEROP_CONFORMANCE_AI_EVIDENCE_ID, INTEROP_CONFORMANCE_CLI_HEADLESS_ID,
    INTEROP_CONFORMANCE_INCIDENT_PACKET_ID, INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF,
    INTEROP_CONFORMANCE_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/interop-conformance";

/// Each case is (fixture-dir, file-name, mutation, scenario).
const CASES: [(&str, &str, &str, &str); 11] = [
    (
        "fixtures/tooling/m5/structured-output-junit-sarif",
        "baseline_stable.json",
        "none",
        "The canonical conformance suite runs the BSP, Bazel BEP/BES, structured-output JUnit/SARIF, and problem-matcher/heuristic corpora across every claimed M5 archetype with current proof, so the release-evidence binding shows stable interop truth.",
    ),
    (
        "fixtures/tooling/m5/structured-output-junit-sarif",
        "structured_raw_payload_not_retained_blocks_stable.json",
        "structured_raw_payload_not_retained",
        "A structured-output importer case drops its retained raw payload reference, so support and replay can no longer recover the original adapter payload.",
    ),
    (
        "fixtures/tooling/m5/structured-output-junit-sarif",
        "structured_export_parity_broken_blocks_stable.json",
        "structured_export_parity_broken",
        "A structured-output importer case breaks export parity, so the support/release/AI projections would lose source, confidence, or refs.",
    ),
    (
        "fixtures/tooling/m5/bsp-discovery",
        "bsp_capability_packet_missing_blocks_stable.json",
        "bsp_capability_packet_missing",
        "A BSP discovery case ran no capability handshake, so its negotiated capability state cannot be evidenced.",
    ),
    (
        "fixtures/tooling/m5/bsp-discovery",
        "bsp_archetype_coverage_missing_blocks_stable.json",
        "bsp_archetype_coverage_missing",
        "The BSP discovery corpus drops the JVM build-server archetype, so a claimed M5 profile that depends on BSP is no longer exercised.",
    ),
    (
        "fixtures/tooling/m5/bazel-bep-bes",
        "bazel_replay_unstable_blocks_stable.json",
        "bazel_replay_unstable",
        "A Bazel BEP/BES case stops replaying deterministically, so the canonical envelopes can no longer be re-derived.",
    ),
    (
        "fixtures/tooling/m5/bazel-bep-bes",
        "bazel_corpus_missing_blocks_stable.json",
        "bazel_corpus_missing",
        "The Bazel BEP/BES corpus is absent entirely, so the interop claim for Bazel monorepos would silently shrink.",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_fallback_reason_missing_blocks_stable.json",
        "heuristic_fallback_reason_missing",
        "A heuristic fallback case names no fallback reason, so a degraded capability is undisclosed at the reason level.",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_confidence_overclaim_blocks_stable.json",
        "heuristic_confidence_overclaim",
        "A heuristic fallback case claims more than low confidence, overclaiming the trust of a best-effort parser.",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_degraded_state_not_disclosed_blocks_stable.json",
        "heuristic_degraded_state_not_disclosed",
        "A heuristic fallback case hides its degraded capability state from the consumer surfaces.",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_evidence_stale_narrows_below_stable.json",
        "heuristic_evidence_stale",
        "The problem-matcher/heuristic corpus proof has aged past its freshness window, so the interop claim narrows below stable instead of staying green on aged proof.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_interop_conformance_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => {
            print_json(&packet.support_export(INTEROP_CONFORMANCE_SUPPORT_EXPORT_ID, exported_at()))
        }
        Some("ai-evidence") => print_json(&packet.evidence_join(
            ConformanceEvidenceSurface::AiEvidence,
            INTEROP_CONFORMANCE_AI_EVIDENCE_ID,
            exported_at(),
        )),
        Some("incident-packet") => print_json(&packet.evidence_join(
            ConformanceEvidenceSurface::IncidentPacket,
            INTEROP_CONFORMANCE_INCIDENT_PACKET_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(INTEROP_CONFORMANCE_CLI_HEADLESS_ID, exported_at()),
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

fn regenerate(root: &Path, packet: &InteropConformancePacket) {
    write_json(&root.join(INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF), packet);
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(INTEROP_CONFORMANCE_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("ai_evidence.json"),
        &packet.evidence_join(
            ConformanceEvidenceSurface::AiEvidence,
            INTEROP_CONFORMANCE_AI_EVIDENCE_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("incident_packet.json"),
        &packet.evidence_join(
            ConformanceEvidenceSurface::IncidentPacket,
            INTEROP_CONFORMANCE_INCIDENT_PACKET_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(INTEROP_CONFORMANCE_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (fixture_dir, file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = InteropConformancePacket::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_interop_conformance_case",
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
                "corpus_family_tokens": mutated.corpus_family_tokens(),
                "archetype_tokens": mutated.archetype_tokens(),
                "dimension_tokens": mutated.dimension_tokens(),
                "source_kind_tokens": mutated.source_kind_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(&root.join(fixture_dir).join(file_name), &fixture);
    }
}

fn mutated_input(mutation: &str) -> InteropConformancePacketInput {
    let mut input = current_stable_interop_conformance_input();
    match mutation {
        "none" => {}
        "structured_raw_payload_not_retained" => {
            first_case(&mut input, CorpusFamily::StructuredOutputJunitSarif)
                .raw_private_material_excluded = false;
        }
        "structured_export_parity_broken" => {
            first_case(&mut input, CorpusFamily::StructuredOutputJunitSarif)
                .export_parity_preserved = false;
        }
        "bsp_capability_packet_missing" => {
            first_case(&mut input, CorpusFamily::BspDiscovery).capability_packet_ref =
                String::new();
        }
        "bsp_archetype_coverage_missing" => {
            corpus(&mut input, CorpusFamily::BspDiscovery)
                .cases
                .retain(|case| case.archetype != InteropArchetype::JvmBuildServer);
        }
        "bazel_replay_unstable" => {
            first_case(&mut input, CorpusFamily::BazelBepBes).replay_stable = false;
        }
        "bazel_corpus_missing" => {
            input
                .corpora
                .retain(|c| c.family != CorpusFamily::BazelBepBes);
        }
        "heuristic_fallback_reason_missing" => {
            first_case(&mut input, CorpusFamily::ProblemMatcherHeuristic).fallback_reason = None;
        }
        "heuristic_confidence_overclaim" => {
            first_case(&mut input, CorpusFamily::ProblemMatcherHeuristic).observed_confidence =
                BuildTestInteropConfidence::High;
        }
        "heuristic_degraded_state_not_disclosed" => {
            first_case(&mut input, CorpusFamily::ProblemMatcherHeuristic)
                .degraded_state_disclosed = false;
        }
        "heuristic_evidence_stale" => {
            let corpus = corpus(&mut input, CorpusFamily::ProblemMatcherHeuristic);
            corpus.proof_age_days = corpus.freshness_window_days + 10;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn corpus(input: &mut InteropConformancePacketInput, family: CorpusFamily) -> &mut InteropCorpus {
    input
        .corpora
        .iter_mut()
        .find(|c| c.family == family)
        .expect("corpus present")
}

fn first_case(
    input: &mut InteropConformancePacketInput,
    family: CorpusFamily,
) -> &mut ConformanceCase {
    corpus(input, family)
        .cases
        .first_mut()
        .expect("corpus has a case")
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
