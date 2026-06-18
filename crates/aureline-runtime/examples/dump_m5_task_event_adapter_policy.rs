//! Headless inspector and regenerator for the M5 task-event adapter-policy
//! baseline.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the BSP/BEP/native fixture corpus from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_adapter_policy
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_adapter_policy -- baseline
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_adapter_policy -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_adapter_policy -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_task_event_adapter_policy -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_task_event_adapter_policy_input, seeded_task_event_adapter_policy_baseline,
    BuildTestInteropConfidence, BuildTestInteropSourceKind, DowngradeReason,
    RawPayloadRetentionClass, TaskEventAdapterPolicyBaseline, TaskEventAdapterPolicyBaselineInput,
    TaskEventConsumer, TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF,
    TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR, TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/tooling/event-interop-baseline";

const CASES: [(&str, &str, &str); 8] = [
    (
        "baseline_stable.json",
        "none",
        "Canonical baseline freezes the native-first adapter ladder, the retention matrix, the downgrade vocabulary, the six consumer bindings, and the arbitration rows.",
    ),
    (
        "ladder_out_of_order_blocks_stable.json",
        "ladder_out_of_order",
        "Native and heuristic ranks are swapped so a heuristic parser claims the highest authority rung.",
    ),
    (
        "heuristic_overclaims_ceiling_blocks_stable.json",
        "heuristic_overclaims_ceiling",
        "The heuristic rung raises its confidence ceiling to high, letting a fallback masquerade as native truth.",
    ),
    (
        "retention_default_invalid_blocks_stable.json",
        "retention_default_invalid",
        "The heuristic source loses its single allowed default retention class.",
    ),
    (
        "downgrade_vocabulary_drift_blocks_stable.json",
        "downgrade_vocabulary_drop",
        "The replay-gap downgrade reason is dropped from the closed vocabulary.",
    ),
    (
        "consumer_binding_missing_blocks_stable.json",
        "consumer_binding_missing",
        "The notebook-run consumer no longer binds to the canonical envelope.",
    ),
    (
        "arbitration_shadow_not_downgraded_blocks_stable.json",
        "arbitration_shadow_not_downgraded",
        "A lower-priority shadow emission is presented without a visible downgrade.",
    ),
    (
        "arbitration_winner_not_highest_priority_blocks_stable.json",
        "arbitration_winner_swapped",
        "Imported structured output is presented as the winner over negotiated BSP truth.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let baseline = seeded_task_event_adapter_policy_baseline();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &baseline),
        Some("baseline") => print_json(&baseline),
        Some("support-export") => print_json(
            &baseline.support_export(TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_ID, exported_at()),
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

fn regenerate(root: &Path, baseline: &TaskEventAdapterPolicyBaseline) {
    write_json(
        &root.join(TASK_EVENT_ADAPTER_POLICY_BASELINE_ARTIFACT_REF),
        baseline,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &baseline.support_export(TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_ID, exported_at()),
    );
    let compact = baseline.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = TaskEventAdapterPolicyBaseline::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_task_event_adapter_policy_case",
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
                "source_kind_tokens": mutated.source_kind_tokens(),
                "consumer_tokens": mutated.consumer_tokens(),
                "downgrade_reason_tokens": mutated.downgrade_reason_tokens(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(
            &root
                .join(TASK_EVENT_ADAPTER_POLICY_FIXTURE_DIR)
                .join(file_name),
            &fixture,
        );
    }
}

fn mutated_input(mutation: &str) -> TaskEventAdapterPolicyBaselineInput {
    let mut input = current_stable_task_event_adapter_policy_input();
    match mutation {
        "none" => {}
        "ladder_out_of_order" => {
            for rung in &mut input.priority_ladder {
                if rung.source_kind == BuildTestInteropSourceKind::Native {
                    rung.priority_rank = 5;
                } else if rung.source_kind == BuildTestInteropSourceKind::HeuristicParser {
                    rung.priority_rank = 1;
                }
            }
        }
        "heuristic_overclaims_ceiling" => {
            for rung in &mut input.priority_ladder {
                if rung.source_kind == BuildTestInteropSourceKind::HeuristicParser {
                    rung.confidence_ceiling = BuildTestInteropConfidence::High;
                }
            }
        }
        "retention_default_invalid" => {
            for cell in &mut input.retention_matrix {
                if cell.source_kind == BuildTestInteropSourceKind::HeuristicParser
                    && cell.retention_class == RawPayloadRetentionClass::MetadataDigestOnly
                {
                    cell.is_default = false;
                }
            }
        }
        "downgrade_vocabulary_drop" => {
            input
                .downgrade_vocabulary
                .retain(|entry| entry.reason != DowngradeReason::ReplayGap);
        }
        "consumer_binding_missing" => {
            input
                .consumer_bindings
                .retain(|binding| binding.consumer != TaskEventConsumer::NotebookRun);
        }
        "arbitration_shadow_not_downgraded" => {
            let row = input
                .arbitration_rows
                .iter_mut()
                .find(|r| r.arbitration_id == "arbitration:native-over-heuristic")
                .expect("seed has the native-over-heuristic arbitration");
            let shadow = &mut row.shadow_events[0];
            shadow.downgraded = false;
            shadow.downgrade_reason = None;
        }
        "arbitration_winner_swapped" => {
            let row = input
                .arbitration_rows
                .iter_mut()
                .find(|r| r.arbitration_id == "arbitration:bsp-over-structured")
                .expect("seed has the bsp-over-structured arbitration");
            std::mem::swap(&mut row.winning_event, &mut row.shadow_events[0]);
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
