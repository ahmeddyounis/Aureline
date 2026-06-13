//! Emits and validates the frozen M5 execution-surface resolution packet.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution
//! cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution -- markdown
//! cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution -- platform macos_desktop
//! cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution -- validate path/to/packet.json
//! ```
//!
//! With no argument it prints the canonical support export JSON. `markdown`
//! prints the deterministic Markdown summary. `platform <token>` prints the
//! single platform resolution (used to regenerate the per-platform fixtures).
//! `validate <path>` parses a packet, prints its violation tokens, and exits
//! non-zero when a resolved row fails to narrow or fail closed.

use std::fs;

use aureline_policy::{
    frozen_stable_m5_execution_surface_resolution_packet, M5ExecutionPlatform,
    M5ExecutionSurfaceResolutionPacket,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_stable_m5_execution_surface_resolution_packet().export_safe_json()
            );
        }
        Some("markdown") => {
            print!(
                "{}",
                frozen_stable_m5_execution_surface_resolution_packet().render_markdown_summary()
            );
        }
        Some("platform") => {
            let token = match args.get(1) {
                Some(token) => token,
                None => {
                    eprintln!(
                        "usage: dump_m5_execution_surface_resolution -- platform <platform_token>"
                    );
                    std::process::exit(2);
                }
            };
            let packet = frozen_stable_m5_execution_surface_resolution_packet();
            let resolution = packet
                .platform_resolutions
                .iter()
                .find(|resolution| resolution.platform.as_str() == token);
            match resolution {
                Some(resolution) => println!(
                    "{}",
                    serde_json::to_string_pretty(resolution)
                        .expect("platform resolution serializes")
                ),
                None => {
                    eprintln!("unknown platform token: {token}");
                    eprintln!(
                        "known platforms: {}",
                        M5ExecutionPlatform::ALL
                            .iter()
                            .map(|platform| platform.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    std::process::exit(2);
                }
            }
        }
        Some("validate") => {
            let path = match args.get(1) {
                Some(path) => path,
                None => {
                    eprintln!(
                        "usage: dump_m5_execution_surface_resolution -- validate <packet.json>"
                    );
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
            let packet: M5ExecutionSurfaceResolutionPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} platforms resolved",
                    packet.platform_resolutions.len()
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
                "usage: dump_m5_execution_surface_resolution [-- markdown | -- platform <token> | -- validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
