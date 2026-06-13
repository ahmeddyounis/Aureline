//! Emits and validates the frozen M5 runtime-authority certification packet.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- markdown
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture all-certified
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture with-missing
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture with-stale
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture with-unsupported
//! cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- validate path/to/packet.json
//! ```
//!
//! With no argument it prints the canonical support export JSON. `markdown`
//! prints the deterministic Markdown summary. `fixture <name>` prints one of the
//! checked certification fixtures. `validate <path>` parses a packet, prints its
//! violation tokens, and exits non-zero when a surface keeps a claim it cannot
//! prove, a narrowed surface silently widens its qualification, or an
//! unsupported backend does not fail closed.

use std::fs;

use aureline_policy::{
    build_certification_packet, certified_entries, entries_with_missing_proof_surface,
    entries_with_stale_proof_surface, entries_with_unsupported_backend_surface,
    frozen_stable_m5_runtime_authority_certification_packet, M5RuntimeAuthorityCertificationPacket,
};

fn fixture_packet(name: &str) -> Option<M5RuntimeAuthorityCertificationPacket> {
    match name {
        "all-certified" => Some(build_certification_packet(
            "m5-runtime-authority-certification:fixture:all-certified",
            "M5 Runtime-Authority Certification — all surfaces certified",
            certified_entries(),
        )),
        "with-missing" => Some(build_certification_packet(
            "m5-runtime-authority-certification:fixture:with-missing",
            "M5 Runtime-Authority Certification — surface narrowed by missing proof",
            entries_with_missing_proof_surface(),
        )),
        "with-stale" => Some(build_certification_packet(
            "m5-runtime-authority-certification:fixture:with-stale",
            "M5 Runtime-Authority Certification — surface narrowed by stale proof",
            entries_with_stale_proof_surface(),
        )),
        "with-unsupported" => Some(build_certification_packet(
            "m5-runtime-authority-certification:fixture:with-unsupported",
            "M5 Runtime-Authority Certification — surface failed closed by unsupported backend",
            entries_with_unsupported_backend_surface(),
        )),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_stable_m5_runtime_authority_certification_packet().export_safe_json()
            );
        }
        Some("markdown") => {
            print!(
                "{}",
                frozen_stable_m5_runtime_authority_certification_packet().render_markdown_summary()
            );
        }
        Some("fixture") => {
            let name = match args.get(1) {
                Some(name) => name,
                None => {
                    eprintln!(
                        "usage: dump_m5_runtime_authority_certification -- fixture <all-certified|with-missing|with-stale|with-unsupported>"
                    );
                    std::process::exit(2);
                }
            };
            match fixture_packet(name) {
                Some(packet) => {
                    let violations = packet.validate();
                    if !violations.is_empty() {
                        for violation in &violations {
                            eprintln!("violation: {}", violation.as_str());
                        }
                        std::process::exit(1);
                    }
                    println!("{}", packet.export_safe_json());
                }
                None => {
                    eprintln!("unknown fixture: {name}");
                    std::process::exit(2);
                }
            }
        }
        Some("validate") => {
            let path = match args.get(1) {
                Some(path) => path,
                None => {
                    eprintln!(
                        "usage: dump_m5_runtime_authority_certification -- validate <packet.json>"
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
            let packet: M5RuntimeAuthorityCertificationPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} certified, {} narrowed across {} surfaces",
                    packet.certified_count(),
                    packet.narrowed_count(),
                    packet.entries.len()
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
                "usage: dump_m5_runtime_authority_certification [-- markdown | -- fixture <name> | -- validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
