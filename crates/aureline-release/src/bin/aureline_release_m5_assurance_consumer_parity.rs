//! Headless emitter for the M5 assurance consumer-parity model.
//!
//! The bin is the only mint-from-truth path for the converged inventory checked in at
//! `artifacts/public-truth/m5-assurance-consumer-parity.json`, the rendered overview document at
//! `artifacts/public-truth/m5-assurance-consumer-parity.md`, the machine-readable fact / consumer
//! matrix CSV at `artifacts/public-truth/m5-assurance-consumer-parity-facts.csv`, the release-grade
//! export proof under `artifacts/release/m5-assurance-export-proof/` (and its Markdown report and
//! refs-only export preview), and the per-state drill fixtures under
//! `fixtures/public-truth/m5-assurance-consumers/`. The model ingests the five M5 assurance / route
//! lanes and projects one fact set onto the About / help, procurement, evaluation, support, and
//! shiproom / public-truth surfaces so they can never restate the same trust facts independently.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- overview
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- export
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- variant <canonical|claim|governance|boundary|event>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- validate
//! ```

use aureline_release::m5_assurance_consumer_parity::{
    seeded_m5_assurance_consumer_parity,
    seeded_m5_assurance_consumer_parity_boundary_route_blocked,
    seeded_m5_assurance_consumer_parity_claim_narrowed,
    seeded_m5_assurance_consumer_parity_event_blocked,
    seeded_m5_assurance_consumer_parity_governance_blocked, M5AssuranceConsumerParity,
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
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("overview") => {
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_overview_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_facts_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("export") => {
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            println!("{}", packet.render_export_preview());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let view = packet
                .consumer_views
                .iter()
                .find(|v| v.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(view)?);
        }
        Some("validate") => {
            let packet = seeded_m5_assurance_consumer_parity();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_assurance_consumer_parity_claim_narrowed())?;
            assert_packet_valid(&seeded_m5_assurance_consumer_parity_governance_blocked())?;
            assert_packet_valid(&seeded_m5_assurance_consumer_parity_boundary_route_blocked())?;
            assert_packet_valid(&seeded_m5_assurance_consumer_parity_event_blocked())?;
            println!(
                "ok: consumer parity valid ({} facts, {} consumers, {} sources)",
                packet.facts.len(),
                packet.consumer_views.len(),
                packet.source_bindings.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5AssuranceConsumerParity, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_assurance_consumer_parity()),
        "claim" => Ok(seeded_m5_assurance_consumer_parity_claim_narrowed()),
        "governance" => Ok(seeded_m5_assurance_consumer_parity_governance_blocked()),
        "boundary" => Ok(seeded_m5_assurance_consumer_parity_boundary_route_blocked()),
        "event" => Ok(seeded_m5_assurance_consumer_parity_event_blocked()),
        other => Err(format!(
            "unknown variant: {other} (canonical|claim|governance|boundary|event)"
        )
        .into()),
    }
}

fn assert_packet_valid(
    packet: &M5AssuranceConsumerParity,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
