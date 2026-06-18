//! Headless inspector and regenerator for the M5 automation-label parity packet.
//!
//! Running the example with no argument regenerates the checked-in parity
//! artifacts and the fail-closed mutation fixtures from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_label_parity
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_label_parity -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_label_parity -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_label_parity -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_label_parity -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_label_parity -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_label_parity_input, seeded_label_parity_packet, AutomationSafetyLabelId,
    LabelParityInput, LabelParityPacket, LabelSurfaceClass, ProjectedLabel,
    LABEL_PARITY_CLI_HEADLESS_ID, LABEL_PARITY_PACKET_ARTIFACT_REF, LABEL_PARITY_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/label-parity";
const FIXTURE_DIR: &str = "fixtures/automation/m5/label-parity";

/// Each mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 7] = [
    (
        "label_parity_stable.json",
        "none",
        "Every claimed command projects its label set to every surface with canonical stable ids and display tokens, every state-preservation guarantee holds, and every invariant holds, so the packet is stable.",
    ),
    (
        "missing_surface_projection_blocks_stable.json",
        "missing_surface",
        "The docs/help surface is dropped for one command, so a docs page could omit the automation posture; the packet blocks stable.",
    ),
    (
        "surface_label_drift_blocks_stable.json",
        "surface_label_drift",
        "The command-palette row adds a label the command source does not carry, drifting the surface set away from one source; the packet blocks stable.",
    ),
    (
        "synonym_display_token_blocks_stable.json",
        "synonym_display_token",
        "The release/public-truth surface renames Writes files to a surface-local synonym; the packet blocks stable.",
    ),
    (
        "effect_disclosure_dropped_blocks_stable.json",
        "effect_disclosure_dropped",
        "The docs/help surface drops the Writes files side-effect label, hiding a material effect; the packet blocks stable.",
    ),
    (
        "stable_id_not_preserved_blocks_stable.json",
        "stable_id_not_preserved",
        "The support-export surface does not preserve stable ids on downgrade, so an exported label would lose its identity; the packet blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The no-surface-invents-synonyms invariant is set false, so a surface could fork its own label terms; the packet blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_label_parity_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => {
            print_json(&packet.support_export(LABEL_PARITY_SUPPORT_EXPORT_ID, exported_at()))
        }
        Some("cli-headless") => {
            print_json(&packet.cli_headless_view(LABEL_PARITY_CLI_HEADLESS_ID, exported_at()))
        }
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

fn regenerate(root: &Path, packet: &LabelParityPacket) {
    write_json(&root.join(LABEL_PARITY_PACKET_ARTIFACT_REF), packet);
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(LABEL_PARITY_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(LABEL_PARITY_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = LabelParityPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_automation_label_parity_case",
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
                "command_verbs": mutated.command_verbs(),
                "is_stable": mutated.is_stable(),
            }
        });
        write_json(&root.join(FIXTURE_DIR).join(file_name), &fixture);
    }
}

fn mutated_input(mutation: &str) -> LabelParityInput {
    let mut input = current_label_parity_input();
    match mutation {
        "none" => {}
        "missing_surface" => {
            input.command_rows[0]
                .surface_projections
                .retain(|projection| projection.surface != LabelSurfaceClass::DocsHelp);
        }
        "surface_label_drift" => {
            projection_mut(&mut input, 2, LabelSurfaceClass::CommandPaletteRow)
                .projected_labels
                .push(ProjectedLabel::canonical(
                    AutomationSafetyLabelId::MacroSafe,
                ));
        }
        "synonym_display_token" => {
            let projection = projection_mut(&mut input, 0, LabelSurfaceClass::ReleasePublicTruth);
            let label = projection
                .projected_labels
                .iter_mut()
                .find(|label| label.label_id == AutomationSafetyLabelId::WritesFiles)
                .expect("writes_files label");
            label.display_token = "Writes to disk".to_owned();
        }
        "effect_disclosure_dropped" => {
            projection_mut(&mut input, 0, LabelSurfaceClass::DocsHelp)
                .projected_labels
                .retain(|label| label.label_id != AutomationSafetyLabelId::WritesFiles);
        }
        "stable_id_not_preserved" => {
            projection_mut(&mut input, 4, LabelSurfaceClass::SupportExport)
                .preserves_stable_ids_on_downgrade = false;
        }
        "invariant_violated" => {
            input.invariants.no_surface_invents_synonyms = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn projection_mut(
    input: &mut LabelParityInput,
    command_index: usize,
    surface: LabelSurfaceClass,
) -> &mut aureline_runtime::SurfaceLabelProjection {
    input.command_rows[command_index]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == surface)
        .expect("surface projection present")
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
