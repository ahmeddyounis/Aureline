//! Headless inspector and regenerator for the M5 automation builder /
//! parameter-review / dry-run-explain / run-history / macro-recorder / safety-label
//! contract baseline.
//!
//! Running the example with no argument regenerates the checked-in baseline
//! artifacts, the worked-example recipe-macro fixtures, and the baseline mutation
//! fixtures from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline -- safety-labels
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_automation_contract_baseline -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_automation_contract_baseline_input, seeded_automation_contract_baseline_packet,
    seeded_dry_run_explain_packet, seeded_macro_session_discarded,
    seeded_macro_session_stopped_promotable, seeded_parameter_review_sheet,
    seeded_recipe_builder_session_blocked, seeded_recipe_builder_session_preview_ready,
    AutomationContractBaselinePacket, AutomationObjectFamily, AutomationSafetyLabelId,
    AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_ID, AUTOMATION_CONTRACT_BASELINE_PACKET_ARTIFACT_REF,
    AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_ID, AUTOMATION_SAFETY_LABEL_MANIFEST_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/automation-contract-baseline";
const RECIPE_MACRO_FIXTURE_DIR: &str = "fixtures/automation/m5/recipe-macro";
const BASELINE_FIXTURE_DIR: &str = "fixtures/automation/m5/automation-contract-baseline";

/// Each baseline mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 7] = [
    (
        "baseline_stable.json",
        "none",
        "Every automation object family is bound, the whole safety-label vocabulary is reused, and every invariant holds, so the baseline is stable.",
    ),
    (
        "missing_object_family_blocks_stable.json",
        "missing_object_family",
        "The macro-recorder family is dropped, so a later surface could invent a feature-local macro runner; the baseline blocks stable.",
    ),
    (
        "family_missing_evidence_hook_blocks_stable.json",
        "family_missing_evidence_hook",
        "The recipe-builder family cites no evidence hook, so its records would have no inspectable lineage; the baseline blocks stable.",
    ),
    (
        "family_missing_consumer_surface_blocks_stable.json",
        "family_missing_consumer_surface",
        "The run-history family names no consumer surface, so no surface is bound to read it; the baseline blocks stable.",
    ),
    (
        "safety_label_set_incomplete_blocks_stable.json",
        "safety_label_set_incomplete",
        "The network-call safety label is dropped, so the reuse vocabulary is partial and surfaces would diverge; the baseline blocks stable.",
    ),
    (
        "safety_label_miscategorized_blocks_stable.json",
        "safety_label_miscategorized",
        "The writes-files effect disclosure is miscategorized as an admissibility cue, so the label kinds drift; the baseline blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The rerun-re-resolves-current-context invariant is set false, so reruns could replay stale authority; the baseline blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_automation_contract_baseline_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(&packet.support_export(
            AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_ID, exported_at()),
        ),
        Some("safety-labels") => print_json(
            &packet.safety_label_manifest(AUTOMATION_SAFETY_LABEL_MANIFEST_ID, exported_at()),
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

fn regenerate(root: &Path, packet: &AutomationContractBaselinePacket) {
    // Baseline packet and its projections.
    write_json(
        &root.join(AUTOMATION_CONTRACT_BASELINE_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(
            AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("safety_label_manifest.json"),
        &packet.safety_label_manifest(AUTOMATION_SAFETY_LABEL_MANIFEST_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    // Worked-example recipe-macro fixtures.
    write_json(
        &root
            .join(RECIPE_MACRO_FIXTURE_DIR)
            .join("recipe_builder_session_preview_ready.json"),
        &seeded_recipe_builder_session_preview_ready(),
    );
    write_json(
        &root
            .join(RECIPE_MACRO_FIXTURE_DIR)
            .join("recipe_builder_session_blocked.json"),
        &seeded_recipe_builder_session_blocked(),
    );
    write_json(
        &root
            .join(RECIPE_MACRO_FIXTURE_DIR)
            .join("parameter_review_sheet.json"),
        &seeded_parameter_review_sheet(),
    );
    write_json(
        &root
            .join(RECIPE_MACRO_FIXTURE_DIR)
            .join("dry_run_explain_packet.json"),
        &seeded_dry_run_explain_packet(),
    );
    write_json(
        &root
            .join(RECIPE_MACRO_FIXTURE_DIR)
            .join("macro_session_stopped_promotable.json"),
        &seeded_macro_session_stopped_promotable(),
    );
    write_json(
        &root
            .join(RECIPE_MACRO_FIXTURE_DIR)
            .join("macro_session_discarded.json"),
        &seeded_macro_session_discarded(),
    );

    // Baseline mutation fixtures (the fail-closed gate cases).
    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = AutomationContractBaselinePacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_automation_contract_baseline_case",
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
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>(),
                "family_tokens": mutated.family_tokens(),
                "safety_label_tokens": mutated.safety_label_tokens(),
                "is_stable": mutated.is_stable(),
            }
        });
        write_json(&root.join(BASELINE_FIXTURE_DIR).join(file_name), &fixture);
    }
}

fn mutated_input(mutation: &str) -> aureline_runtime::AutomationContractBaselineInput {
    let mut input = current_automation_contract_baseline_input();
    match mutation {
        "none" => {}
        "missing_object_family" => {
            input
                .object_families
                .retain(|binding| binding.family != AutomationObjectFamily::MacroRecorder);
        }
        "family_missing_evidence_hook" => {
            family_mut(&mut input, AutomationObjectFamily::RecipeBuilder)
                .evidence_hook_refs
                .clear();
        }
        "family_missing_consumer_surface" => {
            family_mut(&mut input, AutomationObjectFamily::RunHistory)
                .consumer_surfaces
                .clear();
        }
        "safety_label_set_incomplete" => {
            input
                .safety_labels
                .retain(|label| label.label_id != AutomationSafetyLabelId::NetworkCall);
        }
        "safety_label_miscategorized" => {
            input
                .safety_labels
                .iter_mut()
                .find(|label| label.label_id == AutomationSafetyLabelId::WritesFiles)
                .expect("writes_files present")
                .label_kind = aureline_runtime::SafetyLabelKind::AdmissibilityCue;
        }
        "invariant_violated" => {
            input
                .invariants
                .reruns_reresolve_current_context_never_replay_stale_authority = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn family_mut(
    input: &mut aureline_runtime::AutomationContractBaselineInput,
    family: AutomationObjectFamily,
) -> &mut aureline_runtime::ObjectFamilyBinding {
    input
        .object_families
        .iter_mut()
        .find(|binding| binding.family == family)
        .expect("family present")
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
