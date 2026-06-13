//! Emits and validates the frozen M5 authority-lifecycle ledger packet.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_m5_authority_lifecycle_ledger
//! cargo run -q -p aureline-policy --example dump_m5_authority_lifecycle_ledger -- markdown
//! cargo run -q -p aureline-policy --example dump_m5_authority_lifecycle_ledger -- fixture full
//! cargo run -q -p aureline-policy --example dump_m5_authority_lifecycle_ledger -- fixture with-issued
//! cargo run -q -p aureline-policy --example dump_m5_authority_lifecycle_ledger -- fixture with-expired
//! cargo run -q -p aureline-policy --example dump_m5_authority_lifecycle_ledger -- validate path/to/packet.json
//! ```
//!
//! With no argument it prints the canonical support export JSON. `markdown`
//! prints the deterministic Markdown summary. `fixture <name>` prints one of the
//! checked lifecycle fixtures. `validate <path>` parses a packet, prints its
//! violation tokens, and exits non-zero when a grant self-issues authority, an
//! invalidation's trigger drifts from its named dimension, or a lifecycle state
//! is incoherent with its recorded events.

use std::fs;

use aureline_policy::{
    active_entries, build_lifecycle_ledger_packet, expired_entries,
    frozen_stable_m5_authority_lifecycle_ledger_packet, invalidated_entries, issued_entries,
    revoked_entries, M5AuthorityLifecycleLedgerPacket,
};

fn base_entries() -> Vec<aureline_policy::M5AuthorityLedgerEntry> {
    let mut entries = active_entries();
    entries.extend(invalidated_entries());
    entries.extend(revoked_entries());
    entries
}

fn fixture_packet(name: &str) -> Option<M5AuthorityLifecycleLedgerPacket> {
    match name {
        "full" => Some(build_lifecycle_ledger_packet(
            "m5-authority-lifecycle-ledger:fixture:full",
            "M5 Authority-Lifecycle Ledger — full issue-use-revoke",
            base_entries(),
        )),
        "with-issued" => {
            let mut entries = base_entries();
            entries.extend(issued_entries());
            Some(build_lifecycle_ledger_packet(
                "m5-authority-lifecycle-ledger:fixture:with-issued",
                "M5 Authority-Lifecycle Ledger — with an issued-but-unused grant",
                entries,
            ))
        }
        "with-expired" => {
            let mut entries = base_entries();
            entries.extend(expired_entries());
            Some(build_lifecycle_ledger_packet(
                "m5-authority-lifecycle-ledger:fixture:with-expired",
                "M5 Authority-Lifecycle Ledger — with an expired grant",
                entries,
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
                frozen_stable_m5_authority_lifecycle_ledger_packet().export_safe_json()
            );
        }
        Some("markdown") => {
            print!(
                "{}",
                frozen_stable_m5_authority_lifecycle_ledger_packet().render_markdown_summary()
            );
        }
        Some("fixture") => {
            let name = match args.get(1) {
                Some(name) => name,
                None => {
                    eprintln!(
                        "usage: dump_m5_authority_lifecycle_ledger -- fixture <full|with-issued|with-expired>"
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
                        "usage: dump_m5_authority_lifecycle_ledger -- validate <packet.json>"
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
            let packet: M5AuthorityLifecycleLedgerPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!(
                    "ok: {} authority-lifecycle ledger entries",
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
                "usage: dump_m5_authority_lifecycle_ledger [-- markdown | -- fixture <name> | -- validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
