//! Headless inspector and regenerator for the M5 adapter hierarchy negotiation
//! baseline.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the BSP/BEP/heuristic-fallback fixture corpus from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_hierarchy_negotiation
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_hierarchy_negotiation -- baseline
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_hierarchy_negotiation -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_hierarchy_negotiation -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_adapter_hierarchy_negotiation -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_adapter_hierarchy_negotiation_input,
    seeded_adapter_hierarchy_negotiation_baseline, AdapterNegotiationBaseline,
    AdapterNegotiationBaselineInput, BuildTestAdapterCapabilityState, BuildTestInteropConfidence,
    CapabilityNegotiation, DisclosureSurface, Ecosystem, NegotiatedCapability,
    ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF, ADAPTER_NEGOTIATION_FIXTURE_DIR,
    ADAPTER_NEGOTIATION_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/adapter-negotiation";

const CASES: [(&str, &str, &str); 8] = [
    (
        "baseline_stable.json",
        "none",
        "Canonical baseline resolves every ecosystem in native-first order, names the fallback reason for each skipped rung, and discloses unsupported capabilities and drift across all four surfaces.",
    ),
    (
        "lower_priority_displaces_higher_blocks_stable.json",
        "lower_priority_displaces_higher",
        "The JVM native adapter is reachable and capable, yet the lower-priority BSP adapter is still presented as the winner.",
    ),
    (
        "heuristic_overclaims_confidence_blocks_stable.json",
        "heuristic_overclaims_confidence",
        "The generic heuristic resolution raises its confidence to high, letting a last-resort parser read as native truth.",
    ),
    (
        "fallback_not_downgraded_blocks_stable.json",
        "fallback_not_downgraded",
        "The pytest structured-import resolution drops its visible downgrade flag.",
    ),
    (
        "unsupported_capability_unnamed_blocks_stable.json",
        "unsupported_capability_unnamed",
        "The pytest resolution hides an unsupported capability instead of naming it explicitly.",
    ),
    (
        "skip_reason_missing_blocks_stable.json",
        "skip_reason_missing",
        "The Bazel resolution skips the native rung without recording why it was passed over.",
    ),
    (
        "drift_not_visible_blocks_stable.json",
        "drift_not_visible",
        "A capability-drift signal is recorded but not surfaced before it can degrade trust.",
    ),
    (
        "disclosure_surface_missing_blocks_stable.json",
        "disclosure_surface_missing",
        "The AI-evidence surface no longer discloses the negotiation outcome.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let baseline = seeded_adapter_hierarchy_negotiation_baseline();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &baseline),
        Some("baseline") => print_json(&baseline),
        Some("support-export") => print_json(
            &baseline.support_export(ADAPTER_NEGOTIATION_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("compact") => {
            for line in baseline.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match baseline.validate() {
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

fn regenerate(root: &Path, baseline: &AdapterNegotiationBaseline) {
    write_json(
        &root.join(ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF),
        baseline,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &baseline.support_export(ADAPTER_NEGOTIATION_SUPPORT_EXPORT_ID, exported_at()),
    );
    let compact = baseline.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = AdapterNegotiationBaseline::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_adapter_hierarchy_negotiation_case",
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
                "ecosystem_tokens": mutated.ecosystem_tokens(),
                "selected_source_kind_tokens": mutated.selected_source_kind_tokens(),
                "fallback_class_tokens": mutated.fallback_class_tokens(),
                "drift_class_tokens": mutated.drift_class_tokens(),
                "disclosure_surface_tokens": mutated.disclosure_surface_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(
            &root.join(ADAPTER_NEGOTIATION_FIXTURE_DIR).join(file_name),
            &fixture,
        );
    }
}

fn mutated_input(mutation: &str) -> AdapterNegotiationBaselineInput {
    let mut input = current_stable_adapter_hierarchy_negotiation_input();
    match mutation {
        "none" => {}
        "lower_priority_displaces_higher" => {
            let gradle = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::GradleJvm)
                .expect("seed has the gradle resolution");
            let native = gradle
                .candidate_ladder
                .iter_mut()
                .find(|c| c.priority_rank == 1)
                .expect("seed has the native rung");
            native.available = true;
            native.capabilities = vec![CapabilityNegotiation {
                capability: NegotiatedCapability::LifecycleEvents,
                state: BuildTestAdapterCapabilityState::Negotiated,
                capability_packet_ref: "capability-packet:gradle_jvm:native:lifecycle_events"
                    .to_owned(),
                note: "native is reachable".to_owned(),
            }];
        }
        "heuristic_overclaims_confidence" => {
            let generic = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::Generic)
                .expect("seed has the generic resolution");
            generic.confidence = BuildTestInteropConfidence::High;
        }
        "fallback_not_downgraded" => {
            let pytest = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::PythonPytest)
                .expect("seed has the pytest resolution");
            pytest.downgraded = false;
        }
        "unsupported_capability_unnamed" => {
            let pytest = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::PythonPytest)
                .expect("seed has the pytest resolution");
            pytest
                .unsupported_capabilities
                .retain(|c| *c != NegotiatedCapability::TargetGraph);
        }
        "skip_reason_missing" => {
            let bazel = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::Bazel)
                .expect("seed has the bazel resolution");
            let native = bazel
                .candidate_ladder
                .iter_mut()
                .find(|c| c.priority_rank == 1)
                .expect("seed has the native rung");
            native.skip_reason = None;
            bazel
                .fallback_reasons
                .retain(|r| r.adapter_id != "adapter:bazel:native");
        }
        "drift_not_visible" => {
            input.drift_signals[0].visible_before_trust_loss = false;
        }
        "disclosure_surface_missing" => {
            input
                .disclosure_surfaces
                .retain(|b| b.surface != DisclosureSurface::AiEvidence);
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
