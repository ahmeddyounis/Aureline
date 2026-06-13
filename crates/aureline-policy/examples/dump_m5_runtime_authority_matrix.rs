//! Emits and validates the frozen M5 runtime-authority matrix.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_matrix
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_matrix -- markdown
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_matrix -- validate path/to/packet.json
//! ```
//!
//! With no argument it prints the canonical support export JSON. `markdown`
//! prints the deterministic Markdown summary. `validate <path>` parses a packet,
//! prints its violation tokens, and exits non-zero when an underqualified or
//! self-issuing row fails to narrow.

use std::fs;

use aureline_policy::{
    frozen_stable_m5_runtime_authority_matrix_packet, M5RuntimeAuthorityMatrixPacket,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_stable_m5_runtime_authority_matrix_packet().export_safe_json()
            );
        }
        Some("markdown") => {
            print!(
                "{}",
                frozen_stable_m5_runtime_authority_matrix_packet().render_markdown_summary()
            );
        }
        Some("validate") => {
            let path = match args.get(1) {
                Some(path) => path,
                None => {
                    eprintln!("usage: dump_m5_runtime_authority_matrix -- validate <packet.json>");
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
            let packet: M5RuntimeAuthorityMatrixPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} executing surfaces qualified",
                    packet.surface_rows.len()
                );
            } else {
                for violation in &violations {
                    eprintln!("violation: {}", violation.as_str());
                }
                std::process::exit(1);
            }
        }
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            eprintln!(
                "usage: dump_m5_runtime_authority_matrix [-- markdown | -- validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
