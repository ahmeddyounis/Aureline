//! Emits and validates the frozen M5 approval-ticket ledger packet.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_m5_approval_ticket_ledger
//! cargo run -q -p aureline-policy --example dump_m5_approval_ticket_ledger -- markdown
//! cargo run -q -p aureline-policy --example dump_m5_approval_ticket_ledger -- fixture all-valid
//! cargo run -q -p aureline-policy --example dump_m5_approval_ticket_ledger -- fixture expiry-replay
//! cargo run -q -p aureline-policy --example dump_m5_approval_ticket_ledger -- fixture epoch-binding
//! cargo run -q -p aureline-policy --example dump_m5_approval_ticket_ledger -- validate path/to/packet.json
//! ```
//!
//! With no argument it prints the canonical support export JSON. `markdown`
//! prints the deterministic Markdown summary. `fixture <name>` prints one of the
//! checked narrowed ledger fixtures. `validate <path>` parses a packet, prints
//! its violation tokens, and exits non-zero when a ticket fails to bind a
//! dimension, omits a deny reason, or widens authority offline.

use std::fs;

use aureline_policy::{
    build_ledger_packet, denied_tickets, frozen_stable_m5_approval_ticket_ledger_packet,
    valid_tickets, M5ApprovalTicketLedgerPacket,
};

fn fixture_packet(name: &str) -> Option<M5ApprovalTicketLedgerPacket> {
    match name {
        "all-valid" => Some(build_ledger_packet(
            "m5-approval-ticket-ledger:fixture:all-valid",
            "M5 Approval-Ticket Ledger — all local-first valid",
            valid_tickets(),
        )),
        "expiry-replay" => {
            let mut tickets = valid_tickets();
            let mut denied = denied_tickets();
            // Keep only the expiry and replay denials.
            denied.truncate(2);
            tickets.extend(denied);
            Some(build_ledger_packet(
                "m5-approval-ticket-ledger:fixture:expiry-replay",
                "M5 Approval-Ticket Ledger — expiry and replay denied",
                tickets,
            ))
        }
        "epoch-binding" => {
            let mut tickets = valid_tickets();
            let denied = denied_tickets();
            // Keep only the epoch-superseded and binding-mismatch denials.
            tickets.extend(denied.into_iter().skip(2));
            Some(build_ledger_packet(
                "m5-approval-ticket-ledger:fixture:epoch-binding",
                "M5 Approval-Ticket Ledger — epoch and binding denied",
                tickets,
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
                frozen_stable_m5_approval_ticket_ledger_packet().export_safe_json()
            );
        }
        Some("markdown") => {
            print!(
                "{}",
                frozen_stable_m5_approval_ticket_ledger_packet().render_markdown_summary()
            );
        }
        Some("fixture") => {
            let name = match args.get(1) {
                Some(name) => name,
                None => {
                    eprintln!(
                        "usage: dump_m5_approval_ticket_ledger -- fixture <all-valid|expiry-replay|epoch-binding>"
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
                    eprintln!("usage: dump_m5_approval_ticket_ledger -- validate <packet.json>");
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
            let packet: M5ApprovalTicketLedgerPacket = match serde_json::from_str(&raw) {
                Ok(packet) => packet,
                Err(err) => {
                    eprintln!("failed to parse {path}: {err}");
                    std::process::exit(1);
                }
            };
            let violations = packet.validate();
            if violations.is_empty() {
                println!("ok: {} approval tickets", packet.tickets.len());
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
                "usage: dump_m5_approval_ticket_ledger [-- markdown | -- fixture <name> | -- validate <packet.json>]"
            );
            std::process::exit(2);
        }
    }
}
