//! Headless producer/validator for the M5 raw-versus-rendered handoff packet.
//!
//! Usage:
//!
//! - `m5_raw_rendered_handoff` — emit the canonical support export JSON.
//! - `m5_raw_rendered_handoff --markdown` — emit the Markdown summary.
//! - `m5_raw_rendered_handoff --clean` — emit a byte-identical (no divergence)
//!   handoff packet for the clean fixture.
//! - `m5_raw_rendered_handoff --validate <packet.json>` — validate a packet and
//!   print its violation tokens; exits non-zero when invariants fail.

use std::fs;

use aureline_content_safety::{
    frozen_m5_raw_rendered_handoff_packet, project_m5_raw_rendered_handoff,
    M5RawRenderedHandoffPacket, M5RawRenderedHandoffSeed, M5RawRenderedSurface,
    M5RawRenderedSurfaceInput, M5RenderTransform,
};

fn clean_packet() -> M5RawRenderedHandoffPacket {
    let inputs = M5RawRenderedSurface::ALL.map(|surface| M5RawRenderedSurfaceInput {
        surface,
        subject_ref: subject_ref_for(surface),
        render_transform: M5RenderTransform::NoTransform,
        raw_sample: "plain text",
        rendered_sample: "plain text",
    });
    project_m5_raw_rendered_handoff(&M5RawRenderedHandoffSeed {
        case_id: "case:m5-raw-rendered-handoff:clean",
        surface_inputs: inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

fn subject_ref_for(surface: M5RawRenderedSurface) -> &'static str {
    match surface {
        M5RawRenderedSurface::DocsRenderedPanel => "docs:page:guide#install",
        M5RawRenderedSurface::NotebookRenderedOutput => "notebook:cell:demo:out:2",
        M5RawRenderedSurface::AiSummaryEvidence => "ai:evidence:review:finding:7",
        M5RawRenderedSurface::ReviewStructuredDiff => "review:diff:pr:128:file:3",
        M5RawRenderedSurface::StructuredArtifactViewer => "artifact:json:report:9",
        M5RawRenderedSurface::MarketplaceInstallReview => "marketplace:listing:demo@2.0.0",
        M5RawRenderedSurface::PolicyReviewOverlay => "provider:policy:overlay:org",
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_raw_rendered_handoff_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_raw_rendered_handoff_packet().render_markdown_summary()
            );
        }
        Some("--clean") => {
            println!("{}", clean_packet().export_safe_json());
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_raw_rendered_handoff --validate <packet.json>");
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
            let packet: M5RawRenderedHandoffPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} surfaces in raw-rendered honesty",
                    packet.surfaces.len()
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
                "usage: m5_raw_rendered_handoff [--markdown | --clean | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
