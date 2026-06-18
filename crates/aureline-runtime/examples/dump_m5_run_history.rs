//! Headless inspector and regenerator for the M5 run-history / evidence-panel
//! object and its first consumers.
//!
//! Running the example with no argument regenerates the checked-in run-history
//! artifacts and the worked-example run-history-evidence fixtures from the frozen
//! seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_run_history
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_run_history -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_run_history -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_run_history -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_run_history -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_run_history -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_run_history_first_consumers_input, seeded_imported_entry, seeded_run_history_entry,
    seeded_run_history_export_roundtrip, seeded_run_history_first_consumers_packet,
    seeded_run_history_panel, CurrentPolicyBlocker, OpenAsRecipeActionClass,
    RecipeBuilderEntrypoint, RerunActionClass, RunHistoryConsumerBinding, RunHistoryEntry,
    RunHistoryFirstConsumersInput, RunHistoryFirstConsumersPacket,
    RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_ID, RUN_HISTORY_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/run-history";
const FIXTURE_DIR: &str = "fixtures/automation/m5/run-history-evidence";

/// Each mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 8] = [
    (
        "run_history_stable.json",
        "none",
        "Every entrypoint binds a panel whose entries resolve run identity and layer, rerun resolves current policy, imported rows offer no rerun, macros offer no external rerun, open-as-recipe launders no capability, and no raw secret appears, so the packet is stable.",
    ),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
        "The package entrypoint is dropped, so a later surface could render a package history panel with no canonical run-history object; the packet blocks stable.",
    ),
    (
        "rerun_implies_cached_approval_blocks_stable.json",
        "rerun_implies_cached_approval",
        "The request run's blockers add no_blocker_present while still requiring a fresh approval, implying yesterday's approval is cached authority; the packet blocks stable.",
    ),
    (
        "macro_offers_external_rerun_blocks_stable.json",
        "macro_offers_external_rerun",
        "The notebook macro replay resolves to an extension/external rerun a recorded macro must never offer; the packet blocks stable.",
    ),
    (
        "capability_laundered_into_recipe_blocks_stable.json",
        "capability_laundered_into_recipe",
        "The headless-safe test run offers open-as-recipe macro promotion, lifting a capability into a recipe its layer does not admit; the packet blocks stable.",
    ),
    (
        "raw_secret_material_in_history_blocks_stable.json",
        "raw_secret_material_in_history",
        "The request run carries a raw secret value instead of an opaque broker handle, turning history into a shadow secret store; the packet blocks stable.",
    ),
    (
        "evidence_row_projection_inconsistent_blocks_stable.json",
        "evidence_row_projection_inconsistent",
        "The notebook evidence row quotes a rerun action that disagrees with the live entry, so a reviewer could trust a stale resolution; the packet blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The raw-secrets-never-appear invariant is set false, so a surface could leak a secret into history; the packet blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_run_history_first_consumers_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(
            &packet.support_export(RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_ID, exported_at()),
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

fn regenerate(root: &Path, packet: &RunHistoryFirstConsumersPacket) {
    // First-consumers packet and its projections.
    write_json(
        &root.join(RUN_HISTORY_FIRST_CONSUMERS_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(RUN_HISTORY_FIRST_CONSUMERS_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(RUN_HISTORY_FIRST_CONSUMERS_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    // Worked-example fixtures.
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("run_history_export_roundtrip.json"),
        &seeded_run_history_export_roundtrip(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("imported_row_blocks_rerun.json"),
        &seeded_imported_entry()
            .to_evidence_row("run-history:imported-runbook:1", "2026-06-18T00:01:00Z"),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("rerun_survives_history_and_support.json"),
        &survives_demonstration(),
    );

    // Mutation fixtures (the fail-closed gate cases).
    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = RunHistoryFirstConsumersPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_run_history_case",
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

/// Demonstrates that a run's identity and rerun resolution survive export, run
/// history, and support: the resolved rerun comes through the evidence row and a
/// re-import unchanged, and the rerun resolution is fresh (never cached approval).
fn survives_demonstration() -> serde_json::Value {
    let entry = seeded_run_history_entry(RecipeBuilderEntrypoint::RequestApi);
    let initial_rerun = entry.resolved_rerun_class().as_str().to_owned();
    let initial_digest = entry.entry_digest();

    let export = entry.export("export:survives-history:v1", "2026-06-18T00:03:00Z");
    let row = export.evidence_row.clone();
    let resolution = export.rerun_resolution.clone();
    let reimported = export.import();

    json!({
        "record_kind": "run_history_survival_demonstration",
        "schema_version": 1,
        "entry_id": entry.entry_id,
        "run_id": entry.run_identity.run_id,
        "initial_rerun": initial_rerun,
        "history_rerun": row.rerun_action_class.as_str(),
        "reimported_rerun": reimported.resolved_rerun_class().as_str(),
        "initial_digest": initial_digest,
        "history_digest": row.entry_digest,
        "export_digest": export.export_digest,
        "rerun_preserved": initial_rerun == row.rerun_action_class.as_str()
            && initial_rerun == reimported.resolved_rerun_class().as_str(),
        "digest_preserved": initial_digest == row.entry_digest
            && initial_digest == export.export_digest,
        "rerun_resolution_is_fresh": resolution.is_fresh(),
        "identity_and_rerun_preserved": export.identity_and_rerun_preserved(),
    })
}

fn mutated_input(mutation: &str) -> RunHistoryFirstConsumersInput {
    let mut input = current_run_history_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "rerun_implies_cached_approval" => {
            let mut entry = seeded_run_history_entry(RecipeBuilderEntrypoint::RequestApi);
            entry
                .current_policy_blockers
                .push(CurrentPolicyBlocker::NoBlockerPresent);
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, vec![entry]);
        }
        "macro_offers_external_rerun" => {
            let mut entries = seeded_run_history_panel(RecipeBuilderEntrypoint::Notebook);
            entries[1].current_policy_blockers =
                vec![CurrentPolicyBlocker::ExtensionOrExternalRunnerUnavailable];
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Notebook, entries);
        }
        "capability_laundered_into_recipe" => {
            let mut entry = seeded_run_history_entry(RecipeBuilderEntrypoint::TaskTestDebug);
            entry.open_as_recipe_action_class = OpenAsRecipeActionClass::AdmissibleMacroPromotable;
            rebuild_binding(
                &mut input,
                RecipeBuilderEntrypoint::TaskTestDebug,
                vec![entry],
            );
        }
        "raw_secret_material_in_history" => {
            let mut entry = seeded_run_history_entry(RecipeBuilderEntrypoint::RequestApi);
            entry.secret_reference_refs = vec!["raw:plaintext-token".to_owned()];
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, vec![entry]);
        }
        "evidence_row_projection_inconsistent" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook).evidence_rows[0]
                .rerun_action_class = RerunActionClass::BlockedReplayWindowExpired;
        }
        "invariant_violated" => {
            input.invariants.raw_secrets_never_appear_in_history = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn rebuild_binding(
    input: &mut RunHistoryFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    entries: Vec<RunHistoryEntry>,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(RunHistoryConsumerBinding::from_entries(
            entrypoint,
            entries,
            "mutated panel",
        ));
}

fn binding_mut(
    input: &mut RunHistoryFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
) -> &mut RunHistoryConsumerBinding {
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
