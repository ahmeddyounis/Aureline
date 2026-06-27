//! Headless emitter for the M5 runbook certification packet.
//!
//! The bin is the only mint-from-truth path for the certification support export and
//! Markdown proof checked in under `artifacts/release/m5-runbook-proof/`, the published
//! inventory at `artifacts/runbooks/m5-runbook-certification.json`, and the stale /
//! missing-proof drill fixtures under `fixtures/runbooks/m5-certification-drills/`. The
//! certification binds the six runbook proof lanes — governance, sources, steps,
//! executions, handoffs, and companion — to the product rows that claim runbook-backed
//! behavior, and Help/About, the shiproom, support exports, and the incident/operator
//! surfaces all consume this one qualification: a claimed row either maps to current
//! lane proofs or is auto-narrowed / blocked before Stable promotion, with the gap
//! named rather than hidden.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- support-export
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- fixture-stale-proof-narrowed
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- fixture-missing-proof-blocked
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- row <row-id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- validate
//! ```

use aureline_runbooks::m5_runbook_certification::{
    seeded_m5_runbook_certification_packet,
    seeded_m5_runbook_certification_packet_missing_proof_blocked,
    seeded_m5_runbook_certification_packet_stale_proof_narrowed, M5RunbookCertificationPacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_runbook_certification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_certification_packet().render_markdown_summary()
            );
        }
        Some("fixture-stale-proof-narrowed") => {
            let packet = seeded_m5_runbook_certification_packet_stale_proof_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-missing-proof-blocked") => {
            let packet = seeded_m5_runbook_certification_packet_missing_proof_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("row") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let packet = seeded_m5_runbook_certification_packet();
            let row = packet
                .row(id)
                .ok_or_else(|| format!("unknown row id: {id}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(row).expect("row serializes")
            );
        }
        Some("validate") => {
            for packet in [
                seeded_m5_runbook_certification_packet(),
                seeded_m5_runbook_certification_packet_stale_proof_narrowed(),
                seeded_m5_runbook_certification_packet_missing_proof_blocked(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5RunbookCertificationPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "certification packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
