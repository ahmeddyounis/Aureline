//! Headless producer/validator for the M5 safe-preview limited-mode packet.
//!
//! Usage:
//!
//! - `m5_safe_preview_limited_mode` — emit the canonical support export JSON.
//! - `m5_safe_preview_limited_mode --markdown` — emit the Markdown summary.
//! - `m5_safe_preview_limited_mode --clean` — emit an all-small (no guarded
//!   render) packet for the clean fixture.
//! - `m5_safe_preview_limited_mode --validate <packet.json>` — validate a packet
//!   and print its violation tokens; exits non-zero when invariants fail.

use std::fs;

use aureline_content_safety::{
    frozen_m5_safe_preview_limited_mode_packet, project_m5_safe_preview_limited_mode,
    M5LimitedModeArtifactFamily, M5LimitedModeArtifactInput, M5LimitedModeSeed,
    M5SafePreviewLimitedModePacket,
};

fn clean_packet() -> M5SafePreviewLimitedModePacket {
    let artifact_inputs =
        M5LimitedModeArtifactFamily::ALL.map(|family| M5LimitedModeArtifactInput {
            family,
            subject_ref: subject_ref_for(family),
            canonical_source_ref: canonical_source_for(family),
            byte_size: 6_000,
            line_count: 180,
            // Logs are raw captures; the rest are inherently generated. None is
            // oversized or expensive, so nothing needs a guarded render.
            generated: family != M5LimitedModeArtifactFamily::BuildLog,
            full_render_is_expensive: false,
            active_content_present: false,
        });
    project_m5_safe_preview_limited_mode(&M5LimitedModeSeed {
        case_id: "case:m5-safe-preview-limited-mode:all-small",
        artifact_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

fn subject_ref_for(family: M5LimitedModeArtifactFamily) -> &'static str {
    match family {
        M5LimitedModeArtifactFamily::BuildLog => "pipeline:run:128:log:build",
        M5LimitedModeArtifactFamily::DependencyLockfile => "workspace:lockfile:Cargo.lock",
        M5LimitedModeArtifactFamily::TestSnapshot => "test:snapshot:ui:home:1",
        M5LimitedModeArtifactFamily::DistributionBundle => "release:bundle:app@2.0.0",
        M5LimitedModeArtifactFamily::EvidencePacket => "incident:evidence:packet:42",
        M5LimitedModeArtifactFamily::GeneratedArtifact => "codegen:output:api_client.rs",
    }
}

fn canonical_source_for(family: M5LimitedModeArtifactFamily) -> &'static str {
    match family {
        M5LimitedModeArtifactFamily::BuildLog => "pipeline:run:128",
        M5LimitedModeArtifactFamily::DependencyLockfile => "workspace:manifest:Cargo.toml",
        M5LimitedModeArtifactFamily::TestSnapshot => "test:case:ui_home_renders",
        M5LimitedModeArtifactFamily::DistributionBundle => "build:inputs:app@2.0.0",
        M5LimitedModeArtifactFamily::EvidencePacket => "incident:42:underlying-records",
        M5LimitedModeArtifactFamily::GeneratedArtifact => "codegen:spec:openapi.yaml",
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_safe_preview_limited_mode_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_safe_preview_limited_mode_packet().render_markdown_summary()
            );
        }
        Some("--clean") => {
            println!("{}", clean_packet().export_safe_json());
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_safe_preview_limited_mode --validate <packet.json>");
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
            let packet: M5SafePreviewLimitedModePacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} artifacts resolved, {} in limited mode, {} guarded",
                    packet.artifacts.len(),
                    packet.limited_mode_artifact_count,
                    packet.guarded_render_count
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
                "usage: m5_safe_preview_limited_mode [--markdown | --clean | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
