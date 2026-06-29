//! Headless emitter for the M5 assurance / governance / route-provenance governance matrix.
//!
//! The bin is the only mint-from-truth path for the published governance inventory checked in at
//! `artifacts/release/m5-assurance-route-governance-summary.json`, the rendered governance document
//! at `artifacts/release/m5-assurance-route-governance.md`, the machine-readable matrix at
//! `artifacts/release/m5-assurance-route-matrix.csv`, the release-grade parity proof under
//! `artifacts/release-proof/m5-assurance-route-governance/` (and its Markdown report), and the
//! per-state governance fixtures under `fixtures/release/m5-assurance-route/`. The matrix maps every
//! claimed M5 assurance-center, governance-dashboard, capability-boundary, route-hop,
//! approval-ticket, and event-provenance surface to its governed facets, owners, proof paths,
//! disclosed evidence classes, claimed postures, trust boundaries, and degraded-data behavior, and
//! auto-narrows a consumer's claim the moment any facet it reads goes stale or fails.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- governance
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- variant <canonical|stale|missing>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- validate
//! ```

use aureline_release::m5_assurance_route_governance::{
    seeded_m5_assurance_route_governance,
    seeded_m5_assurance_route_governance_missing_proof_blocked,
    seeded_m5_assurance_route_governance_stale_proof_narrowed, M5AssuranceRouteGovernance,
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
            let packet = seeded_m5_assurance_route_governance();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("governance") => {
            let packet = seeded_m5_assurance_route_governance();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_governance_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_assurance_route_governance();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_assurance_route_governance();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_assurance_route_governance();
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
            let packet = seeded_m5_assurance_route_governance();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_assurance_route_governance_stale_proof_narrowed())?;
            assert_packet_valid(&seeded_m5_assurance_route_governance_missing_proof_blocked())?;
            println!(
                "ok: assurance-route matrix valid ({} facets, {} consumers, {} state families)",
                packet.facets.len(),
                packet.consumers.len(),
                packet.state_families.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5AssuranceRouteGovernance, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_assurance_route_governance()),
        "stale" => Ok(seeded_m5_assurance_route_governance_stale_proof_narrowed()),
        "missing" => Ok(seeded_m5_assurance_route_governance_missing_proof_blocked()),
        other => Err(format!("unknown variant: {other} (canonical|stale|missing)").into()),
    }
}

fn assert_packet_valid(
    packet: &M5AssuranceRouteGovernance,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
