//! Headless emitter for the M5 assurance center.
//!
//! The bin is the only mint-from-truth path for the published assurance-center inventory checked in
//! at `artifacts/public-truth/m5-assurance-center.json`, the rendered overview document at
//! `artifacts/public-truth/m5-assurance-center.md`, the machine-readable claim / control matrix at
//! `artifacts/public-truth/m5-assurance-center-claims.csv`, the release-grade parity proof under
//! `artifacts/public-truth/m5-assurance-center-proof/` (and its Markdown report), the exported
//! evaluation packet, and the per-state drill fixtures under
//! `fixtures/public-truth/m5-assurance-center/`. The center turns Aureline's regulated, sovereign,
//! air-gapped, telemetry, residency, key-ownership, and local-first continuity claims into
//! inspectable cards whose active state is derived from the controls backing them, so a card can
//! never read stronger than its proof.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- overview
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- evaluation
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- variant <canonical|waiver|stale|missing>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- claim <subject-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- validate
//! ```

use aureline_release::m5_assurance_center::{
    seeded_m5_assurance_center, seeded_m5_assurance_center_missing_evidence_blocked,
    seeded_m5_assurance_center_stale_evidence_narrowed, seeded_m5_assurance_center_waiver_narrowed,
    M5AssuranceCenter,
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
        Some("registry") | None => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("overview") => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_overview_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_claims_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("evaluation") => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            println!("{}", packet.render_evaluation_packet());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("claim") => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let card = packet
                .claim_cards
                .iter()
                .find(|c| c.subject.as_str() == token)
                .ok_or_else(|| format!("unknown claim subject token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(card)?);
        }
        Some("validate") => {
            let packet = seeded_m5_assurance_center();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_assurance_center_waiver_narrowed())?;
            assert_packet_valid(&seeded_m5_assurance_center_stale_evidence_narrowed())?;
            assert_packet_valid(&seeded_m5_assurance_center_missing_evidence_blocked())?;
            println!(
                "ok: assurance center valid ({} claims, {} controls, {} profiles)",
                packet.claim_cards.len(),
                packet.control_proof_rows.len(),
                packet.overviews.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5AssuranceCenter, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_assurance_center()),
        "waiver" => Ok(seeded_m5_assurance_center_waiver_narrowed()),
        "stale" => Ok(seeded_m5_assurance_center_stale_evidence_narrowed()),
        "missing" => Ok(seeded_m5_assurance_center_missing_evidence_blocked()),
        other => Err(format!("unknown variant: {other} (canonical|waiver|stale|missing)").into()),
    }
}

fn assert_packet_valid(packet: &M5AssuranceCenter) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
