//! Headless producer/validator for the M5 trust-decision identity packet.
//!
//! Usage:
//!
//! - `m5_trust_decision_identity` — emit the canonical support export JSON.
//! - `m5_trust_decision_identity --markdown` — emit the Markdown summary.
//! - `m5_trust_decision_identity --clean` — emit an all-clean-identities packet
//!   for the clean fixture.
//! - `m5_trust_decision_identity --validate <packet.json>` — validate a packet
//!   and print its violation tokens; exits non-zero when invariants fail.

use std::fs;

use aureline_content_safety::{
    frozen_m5_trust_decision_identity_packet, project_m5_trust_decision_identity,
    M5TrustDecisionIdentityInput, M5TrustDecisionIdentityPacket, M5TrustDecisionIdentitySeed,
    M5TrustDecisionSurface,
};

fn clean_packet() -> M5TrustDecisionIdentityPacket {
    let surface_inputs = M5TrustDecisionSurface::ALL.map(|surface| M5TrustDecisionIdentityInput {
        surface,
        subject_ref: subject_ref_for(surface),
        raw_identity_ref: raw_identity_ref_for(surface),
        identity_text: identity_text_for(surface),
    });
    project_m5_trust_decision_identity(&M5TrustDecisionIdentitySeed {
        case_id: "case:m5-trust-decision-identity:all-clean",
        surface_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

fn subject_ref_for(surface: M5TrustDecisionSurface) -> &'static str {
    match surface {
        M5TrustDecisionSurface::PublisherPackageName => "marketplace:listing:formatter@3.1.0",
        M5TrustDecisionSurface::RemoteHostLabel => "remote:attach:build-01",
        M5TrustDecisionSurface::CollaboratorIdentity => "collab:invite:9f2",
        M5TrustDecisionSurface::RouteShare => "route:share:dashboard",
        M5TrustDecisionSurface::PolicyReview => "policy:review:egress-allow-list",
    }
}

fn raw_identity_ref_for(surface: M5TrustDecisionSurface) -> &'static str {
    match surface {
        M5TrustDecisionSurface::PublisherPackageName => {
            "marketplace:publisher:formatter@3.1.0:identity"
        }
        M5TrustDecisionSurface::RemoteHostLabel => "remote:host:build-01:identity",
        M5TrustDecisionSurface::CollaboratorIdentity => "collab:invite:9f2:identity",
        M5TrustDecisionSurface::RouteShare => "route:share:dashboard:target",
        M5TrustDecisionSurface::PolicyReview => "policy:review:egress-allow-list:text",
    }
}

fn identity_text_for(surface: M5TrustDecisionSurface) -> &'static str {
    match surface {
        M5TrustDecisionSurface::PublisherPackageName => "Aureline Labs",
        M5TrustDecisionSurface::RemoteHostLabel => "build-01.internal",
        M5TrustDecisionSurface::CollaboratorIdentity => "Dana Okonomou",
        M5TrustDecisionSurface::RouteShare => "team-observability",
        M5TrustDecisionSurface::PolicyReview => "Allow egress to gpc.example",
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_trust_decision_identity_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_trust_decision_identity_packet().render_markdown_summary()
            );
        }
        Some("--clean") => {
            println!("{}", clean_packet().export_safe_json());
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_trust_decision_identity --validate <packet.json>");
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
            let packet: M5TrustDecisionIdentityPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} surfaces resolved, {} suspicious",
                    packet.surfaces.len(),
                    packet.suspicious_surface_count
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
                "usage: m5_trust_decision_identity [--markdown | --clean | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
