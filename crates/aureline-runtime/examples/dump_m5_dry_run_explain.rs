//! Headless inspector and regenerator for the M5 dry-run/explain preview object
//! and its first consumers.
//!
//! Running the example with no argument regenerates the checked-in
//! dry-run/explain artifacts and the worked-example side-effect-preview fixtures
//! from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_dry_run_explain
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_dry_run_explain -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_dry_run_explain -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_dry_run_explain -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_dry_run_explain -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_dry_run_explain -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_dry_run_explain_first_consumers_input, seeded_blocked_preview,
    seeded_dry_run_explain_consumer_preview, seeded_dry_run_explain_export_roundtrip,
    seeded_dry_run_explain_first_consumers_packet, DryRunExplainConsumerBinding,
    DryRunExplainFirstConsumersInput, DryRunExplainFirstConsumersPacket, DryRunExplainPreview,
    DryRunOutcomeClass, DryRunSideEffectClass, PreviewedAction, RecipeBuilderEntrypoint,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/dry-run-explain";
const FIXTURE_DIR: &str = "fixtures/automation/m5/side-effect-preview";

/// Each mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 7] = [
    (
        "dry_run_explain_stable.json",
        "none",
        "Every entrypoint binds a preview whose predicted writes are declared, process/network/remote actions are labeled, trust/policy blockers are visible, and the frozen projection agrees with the live actions, so the packet is stable.",
    ),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
        "The package entrypoint is dropped, so a later surface could run a package automation with no side-effect preview; the packet blocks stable.",
    ),
    (
        "predicted_write_not_declared_blocks_stable.json",
        "predicted_write_not_declared",
        "The notebook export action claims a predicted write but declares no write, hiding what it would touch; the packet blocks stable.",
    ),
    (
        "mutating_action_mislabeled_read_only_blocks_stable.json",
        "mutating_action_mislabeled_read_only",
        "The package lockfile write is relabeled as a read-only inspection while keeping its write, presenting a mutation as safe; the packet blocks stable.",
    ),
    (
        "outcome_projection_inconsistent_blocks_stable.json",
        "outcome_projection_inconsistent",
        "The request preview's frozen outcome claims it would apply while the live actions need approval, so a reviewer could trust a stale outcome; the packet blocks stable.",
    ),
    (
        "safety_label_projection_inconsistent_blocks_stable.json",
        "safety_label_projection_inconsistent",
        "The notebook preview's frozen label union drops the writes-files label its actions imply, so a write could read as safe; the packet blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The predicted-writes-are-explicit invariant is set false, so a surface could imply a write instead of declaring it; the packet blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_dry_run_explain_first_consumers_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(&packet.support_export(
            DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(&packet.cli_headless_view(
            DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_ID,
            exported_at(),
        )),
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

fn regenerate(root: &Path, packet: &DryRunExplainFirstConsumersPacket) {
    // First-consumers packet and its projections.
    write_json(
        &root.join(DRY_RUN_EXPLAIN_FIRST_CONSUMERS_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(
            DRY_RUN_EXPLAIN_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(
            DRY_RUN_EXPLAIN_FIRST_CONSUMERS_CLI_HEADLESS_ID,
            exported_at(),
        ),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    // Worked-example fixtures.
    write_json(
        &root.join(FIXTURE_DIR).join("preview_export_roundtrip.json"),
        &seeded_dry_run_explain_export_roundtrip(),
    );
    write_json(
        &root.join(FIXTURE_DIR).join("blocked_preview_packet.json"),
        &seeded_blocked_preview().to_packet_record(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("preview_survives_history_and_support.json"),
        &survives_demonstration(),
    );

    // Mutation fixtures (the fail-closed gate cases).
    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = DryRunExplainFirstConsumersPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_dry_run_explain_case",
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
                "entrypoint_tokens": mutated.entrypoint_tokens(),
                "is_stable": mutated.is_stable(),
            }
        });
        write_json(&root.join(FIXTURE_DIR).join(file_name), &fixture);
    }
}

/// Demonstrates that a preview result survives export, run history, and support:
/// the outcome and digest come through the run-history row and a re-import
/// unchanged.
fn survives_demonstration() -> serde_json::Value {
    let preview = seeded_dry_run_explain_consumer_preview(RecipeBuilderEntrypoint::RequestApi);
    let initial_outcome = preview.dry_run_outcome_class().as_str().to_owned();
    let initial_digest = preview.preview_digest();

    let export = preview.export("export:survives-history:v1", "2026-06-18T00:03:00Z");
    let history = export.run_history_row.clone();
    let reimported = export.import();

    json!({
        "record_kind": "dry_run_preview_survival_demonstration",
        "schema_version": 1,
        "preview_id": preview.preview_id,
        "initial_outcome": initial_outcome,
        "history_outcome": history.dry_run_outcome_class.as_str(),
        "reimported_outcome": reimported.dry_run_outcome_class().as_str(),
        "initial_digest": initial_digest,
        "history_digest": history.preview_digest,
        "export_digest": export.export_digest,
        "outcome_preserved": initial_outcome == history.dry_run_outcome_class.as_str()
            && initial_outcome == reimported.dry_run_outcome_class().as_str(),
        "digest_preserved": initial_digest == history.preview_digest
            && initial_digest == export.export_digest,
        "side_effects_preserved": export.side_effects_preserved(),
    })
}

fn mutated_input(mutation: &str) -> DryRunExplainFirstConsumersInput {
    let mut input = current_dry_run_explain_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "predicted_write_not_declared" => {
            let mut preview =
                seeded_dry_run_explain_consumer_preview(RecipeBuilderEntrypoint::Notebook);
            action_mut(&mut preview, "step:write-export")
                .predicted_writes
                .clear();
            replace_binding(&mut input, RecipeBuilderEntrypoint::Notebook, &preview);
        }
        "mutating_action_mislabeled_read_only" => {
            let mut preview =
                seeded_dry_run_explain_consumer_preview(RecipeBuilderEntrypoint::Package);
            action_mut(&mut preview, "step:resolve-update").side_effect_class =
                DryRunSideEffectClass::ReadOnlyInspection;
            replace_binding(&mut input, RecipeBuilderEntrypoint::Package, &preview);
        }
        "outcome_projection_inconsistent" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::RequestApi)
                .packet_record
                .dry_run_outcome_class = DryRunOutcomeClass::WouldApply;
        }
        "safety_label_projection_inconsistent" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook)
                .packet_record
                .aggregate_safety_labels
                .pop();
        }
        "invariant_violated" => {
            input.invariants.predicted_writes_are_explicit_before_apply = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn action_mut<'a>(preview: &'a mut DryRunExplainPreview, step_id: &str) -> &'a mut PreviewedAction {
    preview
        .actions
        .iter_mut()
        .find(|action| action.step_id == step_id)
        .expect("action present")
}

fn replace_binding(
    input: &mut DryRunExplainFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    preview: &DryRunExplainPreview,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(DryRunExplainConsumerBinding::from_preview(preview));
}

fn binding_mut(
    input: &mut DryRunExplainFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
) -> &mut DryRunExplainConsumerBinding {
    input
        .consumer_bindings
        .iter_mut()
        .find(|binding| binding.entrypoint == entrypoint)
        .expect("entrypoint present")
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
