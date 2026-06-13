//! Emits and validates the frozen M5 child-envelope derivation packet.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_m5_child_envelope_derivation
//! cargo run -q -p aureline-policy --example dump_m5_child_envelope_derivation -- markdown
//! cargo run -q -p aureline-policy --example dump_m5_child_envelope_derivation -- fixture all-nominal
//! cargo run -q -p aureline-policy --example dump_m5_child_envelope_derivation -- fixture all-narrowed
//! cargo run -q -p aureline-policy --example dump_m5_child_envelope_derivation -- fixture mixed
//! cargo run -q -p aureline-policy --example dump_m5_child_envelope_derivation -- validate path/to/packet.json
//! ```
//!
//! With no argument it prints the canonical support export JSON. `markdown`
//! prints the deterministic Markdown summary. `fixture <name>` prints one of the
//! checked derivation fixtures. `validate <path>` parses a packet, prints its
//! violation tokens, and exits non-zero when a child widens its parent, inherits
//! the raw OS environment, or runs unconfined when its backend is missing.

use std::fs;

use aureline_policy::{
    build_derivation_packet, frozen_stable_m5_child_envelope_derivation_packet,
    narrowed_derivations, nominal_derivations, M5ChildEnvelopeDerivationPacket, M5NestedLaunchLane,
};

fn fixture_packet(name: &str) -> Option<M5ChildEnvelopeDerivationPacket> {
    match name {
        "all-nominal" => Some(build_derivation_packet(
            "m5-child-envelope-derivation:fixture:all-nominal",
            "M5 Child-Envelope Derivations — all nominal",
            nominal_derivations(),
        )),
        "all-narrowed" => Some(build_derivation_packet(
            "m5-child-envelope-derivation:fixture:all-narrowed",
            "M5 Child-Envelope Derivations — all narrowed",
            narrowed_derivations(),
        )),
        "mixed" => {
            // A nominal derivation per lane plus a narrowed derivation for the
            // untrusted AI lane and the debug lane.
            let mut derivations = nominal_derivations();
            let narrowed = narrowed_derivations();
            for derivation in narrowed {
                if matches!(
                    derivation.lane,
                    M5NestedLaunchLane::Ai | M5NestedLaunchLane::Debug
                ) {
                    derivations.push(derivation);
                }
            }
            Some(build_derivation_packet(
                "m5-child-envelope-derivation:fixture:mixed",
                "M5 Child-Envelope Derivations — mixed",
                derivations,
            ))
        }
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None => {
            println!(
                "{}",
                frozen_stable_m5_child_envelope_derivation_packet().export_safe_json()
            );
        }
        Some("markdown") => {
            print!(
                "{}",
                frozen_stable_m5_child_envelope_derivation_packet().render_markdown_summary()
            );
        }
        Some("fixture") => {
            let name = match args.get(1) {
                Some(name) => name,
                None => {
                    eprintln!(
                        "usage: dump_m5_child_envelope_derivation -- fixture <all-nominal|all-narrowed|mixed>"
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
                    eprintln!("usage: dump_m5_child_envelope_derivation -- validate <packet.json>");
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
            let packet: M5ChildEnvelopeDerivationPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} child-envelope derivations",
                    packet.derivations.len()
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
                "usage: dump_m5_child_envelope_derivation [-- markdown | -- fixture <name> | -- validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
