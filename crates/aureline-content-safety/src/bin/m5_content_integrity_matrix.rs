//! Headless producer/validator for the frozen M5 content-integrity matrix.
//!
//! Usage:
//!
//! - `m5_content_integrity_matrix` — emit the canonical support export JSON.
//! - `m5_content_integrity_matrix --markdown` — emit the Markdown summary.
//! - `m5_content_integrity_matrix --validate <packet.json>` — validate a packet
//!   and print its violation tokens; exits non-zero when narrowed rows are
//!   underqualified.

use std::fs;

use aureline_content_safety::{
    frozen_stable_m5_content_integrity_matrix_packet, M5ContentIntegrityMatrixPacket,
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_stable_m5_content_integrity_matrix_packet().export_safe_json()
            );
        }
        Some("--markdown") => {
            print!(
                "{}",
                frozen_stable_m5_content_integrity_matrix_packet().render_markdown_summary()
            );
        }
        Some("--validate") => {
            let path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("usage: m5_content_integrity_matrix --validate <packet.json>");
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
            let packet: M5ContentIntegrityMatrixPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!("ok: {} families qualified", packet.family_rows.len());
            } else {
                for violation in &violations {
                    eprintln!("violation: {}", violation.as_str());
                }
                std::process::exit(1);
            }
        }
        Some(other) => {
            eprintln!("unknown argument: {other}");
            eprintln!("usage: m5_content_integrity_matrix [--markdown | --validate <packet.json>]");
            std::process::exit(2);
        }
    }
}
