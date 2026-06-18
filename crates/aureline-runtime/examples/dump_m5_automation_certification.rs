//! Headless inspector and regenerator for the M5 automation surface
//! certification matrix.
//!
//! Running the example with no argument regenerates the checked-in artifacts and
//! the fixture mutation cases from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- ai-evidence
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- incident-packet
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_automation_certification -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_automation_certification_input, seeded_automation_certification_packet,
    AutomationAuthoringPath, AutomationCertificationEvidenceSurface, AutomationCertificationPacket,
    AutomationCertificationPacketInput, AutomationSurface, AUTOMATION_CERTIFICATION_AI_EVIDENCE_ID,
    AUTOMATION_CERTIFICATION_CLI_HEADLESS_ID, AUTOMATION_CERTIFICATION_INCIDENT_PACKET_ID,
    AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF, AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/automation-certification";
const FIXTURE_DIR: &str = "fixtures/automation/m5/automation-certification";

/// Each case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 11] = [
    (
        "baseline_stable.json",
        "none",
        "Every claimed M5 notebook/request/package/test-debug/incident/AI automation surface authors in the declarative builder with reviewed parameters, a side-effect preview, durable history, scope-safe macros, and reused labels, so the certification index shows every surface shareable.",
    ),
    (
        "ad_hoc_authoring_blocks_stable.json",
        "ad_hoc_authoring",
        "The notebook automation surface authors automation in an ad-hoc feature dialog instead of the declarative recipe builder, so its certification claim is blocked.",
    ),
    (
        "missing_builder_evidence_blocks_stable.json",
        "missing_builder_evidence",
        "The request/API automation surface cites no upstream builder proof, so its builder-parity claim rests on no machine-readable evidence and is blocked.",
    ),
    (
        "unreviewed_parameters_block_stable.json",
        "unreviewed_parameters",
        "The package automation surface routes inputs without a typed parameter-review sheet, so an unreviewed input could reach an apply.",
    ),
    (
        "unsafe_secret_reference_blocks_stable.json",
        "unsafe_secret_reference",
        "The request/API automation surface inlines a secret instead of resolving a safe secret reference, so its parameter review is not secret-safe.",
    ),
    (
        "missing_side_effect_preview_blocks_stable.json",
        "missing_side_effect_preview",
        "The test/debug automation surface applies automation with no dry-run/explain side-effect preview, so predicted writes/process/network/remote effects are undisclosed before apply.",
    ),
    (
        "run_history_integrity_missing_blocks_stable.json",
        "run_history_integrity_missing",
        "The incident automation surface keeps no rerun-under-current-policy resolution in its run history, so a recorded run cannot be re-evaluated against the current policy.",
    ),
    (
        "macro_scope_unsafe_blocks_stable.json",
        "macro_scope_unsafe",
        "The notebook automation surface records macros that do not fail closed on a context/scope/supported-command mismatch, so a replayed macro could escape its declared scope.",
    ),
    (
        "label_reuse_broken_blocks_stable.json",
        "label_reuse_broken",
        "The AI-linked automation surface invents a label vocabulary instead of reusing the controlled safety-label set, so its labels drift from every other surface.",
    ),
    (
        "missing_surface_blocks_stable.json",
        "missing_surface",
        "The AI-linked automation surface is absent entirely, so the certification matrix would silently shrink to the surfaces that still happen to pass.",
    ),
    (
        "evidence_stale_narrows_below_stable.json",
        "evidence_stale",
        "The incident automation surface proof has aged past its freshness window, so its certification claim narrows below stable instead of staying shareable on aged proof.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_automation_certification_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(
            &packet.support_export(AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_ID, exported_at()),
        ),
        Some("ai-evidence") => print_json(&packet.evidence_join(
            AutomationCertificationEvidenceSurface::AiEvidence,
            AUTOMATION_CERTIFICATION_AI_EVIDENCE_ID,
            exported_at(),
        )),
        Some("incident-packet") => print_json(&packet.evidence_join(
            AutomationCertificationEvidenceSurface::IncidentPacket,
            AUTOMATION_CERTIFICATION_INCIDENT_PACKET_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(
            &packet.cli_headless_view(AUTOMATION_CERTIFICATION_CLI_HEADLESS_ID, exported_at()),
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

fn regenerate(root: &Path, packet: &AutomationCertificationPacket) {
    write_json(
        &root.join(AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(AUTOMATION_CERTIFICATION_SUPPORT_EXPORT_ID, exported_at()),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("ai_evidence.json"),
        &packet.evidence_join(
            AutomationCertificationEvidenceSurface::AiEvidence,
            AUTOMATION_CERTIFICATION_AI_EVIDENCE_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("incident_packet.json"),
        &packet.evidence_join(
            AutomationCertificationEvidenceSurface::IncidentPacket,
            AUTOMATION_CERTIFICATION_INCIDENT_PACKET_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(AUTOMATION_CERTIFICATION_CLI_HEADLESS_ID, exported_at()),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = AutomationCertificationPacket::materialize(mutated_input(mutation));
        let export = mutated.support_export(format!("support-export:{case_name}"), exported_at());
        let fixture = json!({
            "record_kind": "m5_automation_certification_case",
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
                "surface_tokens": mutated.surface_tokens(),
                "authoring_path_tokens": mutated.authoring_path_tokens(),
                "dimension_tokens": mutated.dimension_tokens(),
                "shareable_surfaces": mutated.certification_index.shareable_surfaces.clone(),
                "narrowed_surfaces": mutated.certification_index.narrowed_surfaces.clone(),
                "blocked_surfaces": mutated.certification_index.blocked_surfaces.clone(),
                "support_export_safe": export.is_export_safe(),
            }
        });
        write_json(&root.join(FIXTURE_DIR).join(file_name), &fixture);
    }
}

fn mutated_input(mutation: &str) -> AutomationCertificationPacketInput {
    let mut input = current_stable_automation_certification_input();
    match mutation {
        "none" => {}
        "ad_hoc_authoring" => {
            surface(&mut input, AutomationSurface::NotebookAutomation).authoring_path =
                AutomationAuthoringPath::AdHocFeatureDialog;
        }
        "missing_builder_evidence" => {
            surface(&mut input, AutomationSurface::RequestApiAutomation).evidence_refs = Vec::new();
        }
        "unreviewed_parameters" => {
            surface(&mut input, AutomationSurface::PackageAutomation).parameters_reviewed = false;
        }
        "unsafe_secret_reference" => {
            surface(&mut input, AutomationSurface::RequestApiAutomation).secret_references_safe =
                false;
        }
        "missing_side_effect_preview" => {
            surface(&mut input, AutomationSurface::TestDebugAutomation).side_effect_preview_shown =
                false;
        }
        "run_history_integrity_missing" => {
            surface(&mut input, AutomationSurface::IncidentAutomation).rerun_under_current_policy =
                false;
        }
        "macro_scope_unsafe" => {
            surface(&mut input, AutomationSurface::NotebookAutomation)
                .macro_fails_closed_on_mismatch = false;
        }
        "label_reuse_broken" => {
            surface(&mut input, AutomationSurface::AiLinkedAutomation).reuses_controlled_labels =
                false;
        }
        "missing_surface" => {
            input
                .surfaces
                .retain(|row| row.surface != AutomationSurface::AiLinkedAutomation);
        }
        "evidence_stale" => {
            let row = surface(&mut input, AutomationSurface::IncidentAutomation);
            row.proof_age_days = row.freshness_window_days + 10;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn surface(
    input: &mut AutomationCertificationPacketInput,
    surface: AutomationSurface,
) -> &mut aureline_runtime::AutomationSurfaceCertification {
    input
        .surfaces
        .iter_mut()
        .find(|row| row.surface == surface)
        .expect("surface present")
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
