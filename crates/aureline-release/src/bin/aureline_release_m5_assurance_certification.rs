//! Headless emitter for the M5 assurance certification packet.
//!
//! The bin is the only mint-from-truth path for the published certification inventory checked in at
//! `artifacts/public-truth/m5-assurance-certification.json`, the rendered certification document at
//! `artifacts/public-truth/m5-assurance-certification.md`, the machine-readable grid CSV at
//! `artifacts/public-truth/m5-assurance-certification-grid.csv`, the release-grade parity proof under
//! `artifacts/release/m5-assurance-certification-proof/` (and its Markdown report), and the per-state
//! packet fixtures under `fixtures/public-truth/m5-assurance-certification/`. It qualifies every
//! claimed M5 managed / self-hosted / regulated / sovereign profile against the assurance,
//! governance, boundary-route, and event-provenance contract and narrows or blocks a profile claim
//! deterministically when the backing governance proof is stale, drifting, or missing.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- document
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- variant <all-certified|stale-narrowed|missing-blocked>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- validate
//! ```

use aureline_release::m5_assurance_certification::{
    seeded_m5_assurance_certification, seeded_m5_assurance_certification_missing_proof_blocked,
    seeded_m5_assurance_certification_stale_proof_narrowed, M5AssuranceCertification,
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
        Some("registry") | Some("proof") | None => {
            let packet = seeded_m5_assurance_certification();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("document") => {
            let packet = seeded_m5_assurance_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_certification_markdown());
        }
        Some("markdown") => {
            let packet = seeded_m5_assurance_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_assurance_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_grid_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_assurance_certification();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let consumer = packet
                .consumers
                .iter()
                .find(|c| c.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(consumer)?);
        }
        Some("validate") => {
            let packet = seeded_m5_assurance_certification();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_assurance_certification_stale_proof_narrowed())?;
            assert_packet_valid(&seeded_m5_assurance_certification_missing_proof_blocked())?;
            println!(
                "ok: assurance certification valid ({} profiles, {} consumers)",
                packet.profiles.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5AssuranceCertification, Box<dyn std::error::Error>> {
    match token {
        "all-certified" | "" => Ok(seeded_m5_assurance_certification()),
        "stale-narrowed" => Ok(seeded_m5_assurance_certification_stale_proof_narrowed()),
        "missing-blocked" => Ok(seeded_m5_assurance_certification_missing_proof_blocked()),
        other => Err(format!(
            "unknown variant: {other} (all-certified|stale-narrowed|missing-blocked)"
        )
        .into()),
    }
}

fn assert_packet_valid(
    packet: &M5AssuranceCertification,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
