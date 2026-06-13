//! Headless producer/validator for the M5 trust-class ladder packet.
//!
//! Usage:
//!
//! - `m5_trust_class_ladder` — emit the canonical support export JSON.
//! - `m5_trust_class_ladder --markdown` — emit the Markdown summary.
//! - `m5_trust_class_ladder --clean` — emit an all-trusted (no downgrade)
//!   packet for the clean fixture.
//! - `m5_trust_class_ladder --validate <packet.json>` — validate a packet and
//!   print its violation tokens; exits non-zero when invariants fail.

use std::fs;

use aureline_content_safety::{
    frozen_m5_trust_class_ladder_packet, project_m5_trust_class_ladder, M5TrustClass,
    M5TrustClassLadderPacket, M5TrustLadderSeed, M5TrustLadderSurface, M5TrustLadderSurfaceInput,
    M5TrustSignals,
};

fn clean_packet() -> M5TrustClassLadderPacket {
    let surface_inputs = M5TrustLadderSurface::ALL.map(|surface| M5TrustLadderSurfaceInput {
        surface,
        subject_ref: subject_ref_for(surface),
        requested_trust_class: requested_class_for(surface),
        active_content_present: requested_class_for(surface).is_active(),
        signals: M5TrustSignals::all_clear(),
    });
    project_m5_trust_class_ladder(&M5TrustLadderSeed {
        case_id: "case:m5-trust-class-ladder:all-trusted",
        surface_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

fn subject_ref_for(surface: M5TrustLadderSurface) -> &'static str {
    match surface {
        M5TrustLadderSurface::NotebookRichOutput => "notebook:cell:demo:out:2",
        M5TrustLadderSurface::DocsBrowserPanel => "docs:page:guide#embed",
        M5TrustLadderSurface::AiEvidenceViewer => "ai:evidence:review:finding:7",
        M5TrustLadderSurface::PipelineArtifactBrowser => "pipeline:run:128:artifact:logs",
        M5TrustLadderSurface::ProviderOverlay => "provider:policy:overlay:org",
        M5TrustLadderSurface::MarketplaceInstallReview => "marketplace:listing:demo@2.0.0",
        M5TrustLadderSurface::RemotePreviewTarget => "remote:preview:target:pr:128",
        M5TrustLadderSurface::StructuredCompareView => "review:diff:pr:128:file:3",
    }
}

/// The class each surface requests when every signal is clear. Embedded/review
/// surfaces stay at sanitized rich by construction; the rest request their
/// natural active class.
fn requested_class_for(surface: M5TrustLadderSurface) -> M5TrustClass {
    match surface {
        M5TrustLadderSurface::NotebookRichOutput => M5TrustClass::TrustedLocalActive,
        M5TrustLadderSurface::DocsBrowserPanel | M5TrustLadderSurface::RemotePreviewTarget => {
            M5TrustClass::IsolatedRemoteActive
        }
        M5TrustLadderSurface::AiEvidenceViewer
        | M5TrustLadderSurface::PipelineArtifactBrowser
        | M5TrustLadderSurface::ProviderOverlay
        | M5TrustLadderSurface::MarketplaceInstallReview
        | M5TrustLadderSurface::StructuredCompareView => M5TrustClass::SanitizedRich,
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_trust_class_ladder_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_trust_class_ladder_packet().render_markdown_summary()
            );
        }
        Some("--clean") => {
            println!("{}", clean_packet().export_safe_json());
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_trust_class_ladder --validate <packet.json>");
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
            let packet: M5TrustClassLadderPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} surfaces resolved, {} downgraded",
                    packet.surfaces.len(),
                    packet.downgraded_surface_count
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
                "usage: m5_trust_class_ladder [--markdown | --clean | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
