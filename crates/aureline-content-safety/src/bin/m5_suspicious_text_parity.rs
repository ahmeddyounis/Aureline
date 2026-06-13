//! Headless producer/validator for the M5 suspicious-text detector parity packet.
//!
//! Usage:
//!
//! - `m5_suspicious_text_parity` — emit the canonical support export JSON.
//! - `m5_suspicious_text_parity --markdown` — emit the Markdown summary.
//! - `m5_suspicious_text_parity --clean` — emit a clean-content parity packet
//!   (no suspicious findings) for the clean fixture.
//! - `m5_suspicious_text_parity --validate <packet.json>` — validate a packet
//!   and print its violation tokens; exits non-zero when invariants fail.

use std::fs;

use aureline_content_safety::{
    frozen_m5_suspicious_text_parity_packet, project_m5_suspicious_text_parity,
    M5SuspiciousTextParityPacket, M5SuspiciousTextParitySeed,
};

fn clean_packet() -> M5SuspiciousTextParityPacket {
    project_m5_suspicious_text_parity(&M5SuspiciousTextParitySeed {
        case_id: "case:m5-suspicious-text-parity:clean",
        content: "let payload = \"ok\";\nlet admin = username;\n",
        subject_refs: [
            "notebook:cell:demo:out:1",
            "docs:page:demo#install",
            "marketplace:listing:demo@1.2.0",
            "remote:host:demo.example.dev",
            "collab:share:demo:thread:1",
            "ai:evidence:demo:finding:1",
            "provider:overlay:demo:policy",
        ],
        minted_at: "2026-06-10T00:00:00Z",
    })
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_m5_suspicious_text_parity_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_m5_suspicious_text_parity_packet().render_markdown_summary()
            );
        }
        Some("--clean") => {
            println!("{}", clean_packet().export_safe_json());
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_suspicious_text_parity --validate <packet.json>");
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
            let packet: M5SuspiciousTextParityPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!("ok: {} surfaces in parity", packet.surfaces.len());
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
                "usage: m5_suspicious_text_parity [--markdown | --clean | --validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
