//! Headless producer/validator for the frozen M5 content-integrity certification.
//!
//! Usage:
//!
//! - `m5_content_integrity_certification` — emit the canonical support export JSON.
//! - `m5_content_integrity_certification --markdown` — emit the Markdown summary.
//! - `m5_content_integrity_certification --validate <packet.json>` — validate a
//!   packet and print its violation tokens; exits non-zero when the certification
//!   disagrees with its own proofs or a required dimension is missing.

use std::fs;

use aureline_content_safety::{
    frozen_m5_content_integrity_certification_packet, M5ContentIntegrityCertificationPacket,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_content_integrity_certification_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_content_integrity_certification_packet().render_markdown_summary()
            );
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_content_integrity_certification --validate <packet.json>");
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
            let packet: M5ContentIntegrityCertificationPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} families certified ({} narrowed)",
                    packet.family_rows.len(),
                    packet.summary.narrowed_families
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
                "usage: m5_content_integrity_certification [--markdown | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
