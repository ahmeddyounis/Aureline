//! Headless inspector and regenerator for the M5 parameter-review object and its
//! first consumers.
//!
//! Running the example with no argument regenerates the checked-in
//! parameter-review artifacts and the worked-example fixtures from the frozen
//! seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_parameter_review
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_parameter_review -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_parameter_review -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_parameter_review -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_parameter_review -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_parameter_review -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_parameter_review_first_consumers_input, seeded_consumer_sheet,
    seeded_parameter_review_export_roundtrip, seeded_parameter_review_first_consumers_packet,
    seeded_secret_reference_sheet, ParameterReviewBuilder, ParameterReviewConsumerBinding,
    ParameterReviewFirstConsumersInput, ParameterReviewFirstConsumersPacket, ParameterSourceLayer,
    ParameterValueState, RecipeBuilderEntrypoint, SaveToScope,
    PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    PARAMETER_REVIEW_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/parameter-review";
const FIXTURE_DIR: &str = "fixtures/automation/m5/parameter-review";

/// Each mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 7] = [
    (
        "parameter_review_stable.json",
        "none",
        "Every entrypoint binds a typed sheet with explicit source layers, secret references, and allowed save scopes, and every invariant holds, so the packet is stable.",
    ),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
        "The package entrypoint is dropped, so a later surface could gather package inputs through a private form; the packet blocks stable.",
    ),
    (
        "raw_secret_blocks_stable.json",
        "raw_secret",
        "The request bearer token drops its broker reference and claims a resolved value, so a secret could land as a raw literal; the packet blocks stable.",
    ),
    (
        "save_scope_not_allowed_blocks_stable.json",
        "save_scope_not_allowed",
        "The notebook output directory chooses a save scope outside its allowed set, hiding where a remembered value would persist; the packet blocks stable.",
    ),
    (
        "source_layer_unspecified_blocks_stable.json",
        "source_layer_unspecified",
        "The incident reference hides in a generic control with no source layer, so its provenance is ambiguous; the packet blocks stable.",
    ),
    (
        "sheet_projection_inconsistent_blocks_stable.json",
        "sheet_projection_inconsistent",
        "The notebook frozen-sheet projection disagrees with the live parameter verdict, so a consumer could read a stale review; the packet blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The secret-values-are-references invariant is set false, so a surface could store a raw secret; the packet blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_parameter_review_first_consumers_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(&packet.support_export(
            PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(&packet.cli_headless_view(
            PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_ID,
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

fn regenerate(root: &Path, packet: &ParameterReviewFirstConsumersPacket) {
    // First-consumers packet and its projections.
    write_json(
        &root.join(PARAMETER_REVIEW_FIRST_CONSUMERS_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(
            PARAMETER_REVIEW_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(
            PARAMETER_REVIEW_FIRST_CONSUMERS_CLI_HEADLESS_ID,
            exported_at(),
        ),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    // Worked-example fixtures.
    write_json(
        &root.join(FIXTURE_DIR).join("sheet_export_roundtrip.json"),
        &seeded_parameter_review_export_roundtrip(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("secret_reference_held_sheet.json"),
        &seeded_secret_reference_sheet().to_sheet_record(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("rerun_preserves_provenance.json"),
        &rerun_demonstration(),
    );

    // Mutation fixtures (the fail-closed gate cases).
    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = ParameterReviewFirstConsumersPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_parameter_review_first_consumers_case",
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

/// Demonstrates that a rerun preserves every parameter's source layer and
/// redaction posture and never re-materializes a raw secret.
fn rerun_demonstration() -> serde_json::Value {
    let sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::RequestApi);
    let initial_source_layers = sheet.source_layer_tokens();
    let initial_redaction_classes = sheet.redaction_class_tokens();

    // A rerun re-projects the same sheet; provenance must come through unchanged.
    let export = sheet.export("export:rerun-review:v1", "2026-06-18T00:03:00Z");
    let reran = export.import();
    let rerun_source_layers = reran.source_layer_tokens();
    let rerun_redaction_classes = reran.redaction_class_tokens();

    json!({
        "record_kind": "parameter_review_rerun_demonstration",
        "schema_version": 1,
        "sheet_id": sheet.sheet_id,
        "initial_source_layers": initial_source_layers,
        "rerun_source_layers": rerun_source_layers,
        "initial_redaction_classes": initial_redaction_classes,
        "rerun_redaction_classes": rerun_redaction_classes,
        "secret_reference_count": sheet.secret_reference_count(),
        "source_layers_preserved": initial_source_layers == rerun_source_layers,
        "redaction_preserved": initial_redaction_classes == rerun_redaction_classes,
        "provenance_preserved": export.provenance_preserved(),
    })
}

fn mutated_input(mutation: &str) -> ParameterReviewFirstConsumersInput {
    let mut input = current_parameter_review_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "raw_secret" => {
            let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::RequestApi);
            let token = parameter_mut(&mut sheet, "bearer_token");
            token.secret_reference = None;
            token.value_state = ParameterValueState::DefaultValue;
            replace_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, &sheet);
        }
        "save_scope_not_allowed" => {
            let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Notebook);
            parameter_mut(&mut sheet, "output_dir").chosen_save_scope = SaveToScope::User;
            replace_binding(&mut input, RecipeBuilderEntrypoint::Notebook, &sheet);
        }
        "source_layer_unspecified" => {
            let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Incident);
            parameter_mut(&mut sheet, "incident_ref").source_layer =
                ParameterSourceLayer::UnspecifiedGenericControl;
            replace_binding(&mut input, RecipeBuilderEntrypoint::Incident, &sheet);
        }
        "sheet_projection_inconsistent" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook)
                .sheet_record
                .rows[0]
                .verdict_class = aureline_runtime::ParameterReviewVerdictClass::Blocked;
        }
        "invariant_violated" => {
            input.invariants.secret_values_are_references_not_raw = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn parameter_mut<'a>(
    sheet: &'a mut ParameterReviewBuilder,
    parameter_name: &str,
) -> &'a mut aureline_runtime::ReviewedParameter {
    sheet
        .parameters
        .iter_mut()
        .find(|parameter| parameter.parameter_name == parameter_name)
        .expect("parameter present")
}

fn replace_binding(
    input: &mut ParameterReviewFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    sheet: &ParameterReviewBuilder,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(ParameterReviewConsumerBinding::from_builder(sheet));
}

fn binding_mut(
    input: &mut ParameterReviewFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
) -> &mut ParameterReviewConsumerBinding {
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
