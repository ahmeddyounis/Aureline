//! Headless producer/validator for the M5 mutation-path fix-flow packet.
//!
//! Usage:
//!
//! - `m5_mutation_path_fix_flow` — emit the canonical support export JSON.
//! - `m5_mutation_path_fix_flow --markdown` — emit the Markdown summary.
//! - `m5_mutation_path_fix_flow --clean` — emit an all-clean-no-findings packet
//!   for the clean fixture.
//! - `m5_mutation_path_fix_flow --validate <packet.json>` — validate a packet
//!   and print its violation tokens; exits non-zero when invariants fail.

use std::fs;

use aureline_content_safety::{
    frozen_m5_mutation_path_fix_flow_packet, project_m5_mutation_path_fix_flow, M5MutationPath,
    M5MutationPathFixFlowPacket, M5MutationPathFixFlowSeed, M5MutationPathInput,
};

fn clean_packet() -> M5MutationPathFixFlowPacket {
    let path_inputs = M5MutationPath::ALL.map(|path| M5MutationPathInput {
        path,
        artifact_ref: artifact_ref_for(path),
        raw_content_ref: raw_content_ref_for(path),
        content_excerpt: clean_content_for(path),
        suppression: None,
    });
    project_m5_mutation_path_fix_flow(&M5MutationPathFixFlowSeed {
        case_id: "case:m5-mutation-path-fix-flow:all-clean",
        path_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

fn artifact_ref_for(path: M5MutationPath) -> &'static str {
    match path {
        M5MutationPath::Save => "docs:page:runbook",
        M5MutationPath::Format => "notebook:cell:7",
        M5MutationPath::OrganizeImports => "structured:artifact:config",
        M5MutationPath::AiApply => "ai:evidence:patch-1",
    }
}

fn raw_content_ref_for(path: M5MutationPath) -> &'static str {
    match path {
        M5MutationPath::Save => "docs:page:runbook:raw",
        M5MutationPath::Format => "notebook:cell:7:raw",
        M5MutationPath::OrganizeImports => "structured:artifact:config:raw",
        M5MutationPath::AiApply => "ai:evidence:patch-1:raw",
    }
}

fn clean_content_for(path: M5MutationPath) -> &'static str {
    match path {
        M5MutationPath::Save => "title: Allow egress",
        M5MutationPath::Format => "value = build_id",
        M5MutationPath::OrganizeImports => "import payload",
        M5MutationPath::AiApply => "fix payload now",
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_mutation_path_fix_flow_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_mutation_path_fix_flow_packet().render_markdown_summary()
            );
        }
        Some("--clean") => {
            println!("{}", clean_packet().export_safe_json());
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_mutation_path_fix_flow --validate <packet.json>");
                    std::process::exit(2);
                }
            };
            let raw = match fs::read_to_string(path) {
                Ok(raw) => raw,
                Err(err) => {
                    eprintln!("failed to read {path}: {err}");
                    std::process::exit(1);
                }
            };
            let packet: M5MutationPathFixFlowPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} paths resolved, {} with findings, {} suppressed",
                    packet.paths.len(),
                    packet.paths_with_findings_count,
                    packet.suppressed_path_count
                );
            } else {
                for violation in &violations {
                    eprintln!("violation: {}", violation.as_str());
                }
                std::process::exit(1);
            }
        }
        Some(other) => {
            eprintln!("unknown argument: {other}");
            eprintln!(
                "usage: m5_mutation_path_fix_flow [--markdown | --clean | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
