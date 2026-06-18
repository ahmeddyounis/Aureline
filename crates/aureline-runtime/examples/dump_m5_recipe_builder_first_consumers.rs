//! Headless inspector and regenerator for the M5 recipe-builder object and its
//! first consumers.
//!
//! Running the example with no argument regenerates the checked-in first-consumers
//! artifacts and the worked-example recipe-builder fixtures from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_recipe_builder_first_consumers
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_recipe_builder_first_consumers -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_recipe_builder_first_consumers -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_recipe_builder_first_consumers -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_recipe_builder_first_consumers -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_recipe_builder_first_consumers -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_recipe_builder_first_consumers_input, seeded_blocked_recipe_builder,
    seeded_recipe_builder_export_roundtrip, seeded_recipe_builder_first_consumers_packet,
    RecipeBuilderConsumerBinding, RecipeBuilderEntrypoint, RecipeBuilderFirstConsumersPacket,
    RecipeBuilderStateClass, ReorderGesture, RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    RECIPE_BUILDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/recipe-builder-first-consumers";
const FIXTURE_DIR: &str = "fixtures/automation/m5/recipe-builder";

/// Each mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 6] = [
    (
        "first_consumers_stable.json",
        "none",
        "Every first-consumer entrypoint binds the canonical builder, every step keeps its command identity and copy-CLI/open-docs parity, and every invariant holds, so the packet is stable.",
    ),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
        "The package entrypoint is dropped, so a later surface could invent a feature-local recipe wizard; the packet blocks stable.",
    ),
    (
        "non_declarative_manifest_blocks_stable.json",
        "non_declarative_manifest",
        "The notebook builder targets a shell-script manifest instead of the declarative recipe manifest; the packet blocks stable.",
    ),
    (
        "ui_only_step_not_blocked_blocks_stable.json",
        "ui_only_step_not_blocked",
        "A builder cites a UI-only command but reads as preview-ready rather than blocked, hiding inadmissible authority; the packet blocks stable.",
    ),
    (
        "cli_docs_parity_broken_blocks_stable.json",
        "cli_docs_parity_broken",
        "A step's copy-CLI no longer cites its canonical verb, so the CLI and docs would point at different commands; the packet blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The builder-reuses-command-truth invariant is set false, so a surface could fork private form state; the packet blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_recipe_builder_first_consumers_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(&packet.support_export(
            RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(&packet.cli_headless_view(
            RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
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

fn regenerate(root: &Path, packet: &RecipeBuilderFirstConsumersPacket) {
    // First-consumers packet and its projections.
    write_json(
        &root.join(RECIPE_BUILDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(
            RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(
            RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
            exported_at(),
        ),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    // Worked-example recipe-builder fixtures.
    write_json(
        &root.join(FIXTURE_DIR).join("builder_export_roundtrip.json"),
        &seeded_recipe_builder_export_roundtrip(),
    );
    write_json(
        &root.join(FIXTURE_DIR).join("blocked_builder_session.json"),
        &seeded_blocked_recipe_builder().to_session_record(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("reorder_preserves_identity.json"),
        &reorder_demonstration(),
    );

    // Mutation fixtures (the fail-closed gate cases).
    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = RecipeBuilderFirstConsumersPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_recipe_builder_first_consumers_case",
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

/// Demonstrates that a drag and the equivalent keyboard moves converge on the
/// same step order, with both reorder logs preserved.
fn reorder_demonstration() -> serde_json::Value {
    let initial = aureline_runtime::seeded_consumer_builder(RecipeBuilderEntrypoint::TaskTestDebug);

    let mut dragged = initial.clone();
    dragged
        .reorder("step:rerun-failed", ReorderGesture::DragToIndex(0))
        .expect("drag rerun-failed to front");

    let mut keyed = initial.clone();
    keyed
        .reorder("step:rerun-failed", ReorderGesture::KeyboardMoveUp)
        .expect("keyboard move rerun-failed up");

    json!({
        "record_kind": "recipe_builder_reorder_demonstration",
        "schema_version": 1,
        "builder_id": initial.builder_id,
        "initial_order": initial.step_order(),
        "drag_result_order": dragged.step_order(),
        "keyboard_result_order": keyed.step_order(),
        "orders_match": dragged.step_order() == keyed.step_order(),
        "step_identity_preserved": dragged.steps.len() == initial.steps.len(),
        "drag_reorder_log": dragged.reorder_log,
        "keyboard_reorder_log": keyed.reorder_log,
    })
}

fn mutated_input(mutation: &str) -> aureline_runtime::RecipeBuilderFirstConsumersInput {
    let mut input = current_recipe_builder_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "non_declarative_manifest" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook)
                .session_record
                .manifest_target_schema_ref =
                "schemas/automation/shell_script.schema.json".to_owned();
        }
        "ui_only_step_not_blocked" => {
            let mut forged =
                RecipeBuilderConsumerBinding::from_builder(&seeded_blocked_recipe_builder());
            forged.builder_state_class = RecipeBuilderStateClass::PreviewReady;
            forged.session_record.builder_state_class = RecipeBuilderStateClass::PreviewReady;
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::TaskTestDebug);
            input.consumer_bindings.push(forged);
        }
        "cli_docs_parity_broken" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook).copy_cli_lines[0] =
                "aureline command run wrong.verb".to_owned();
        }
        "invariant_violated" => {
            input
                .invariants
                .builder_reuses_command_truth_not_private_form_state = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn binding_mut(
    input: &mut aureline_runtime::RecipeBuilderFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
) -> &mut RecipeBuilderConsumerBinding {
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
