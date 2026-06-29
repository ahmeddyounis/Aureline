//! Headless emitter for the M5 governance dashboard.
//!
//! The bin is the only mint-from-truth path for the published governance-dashboard inventory checked
//! in at `artifacts/public-truth/m5-governance-dashboard.json`, the rendered overview document at
//! `artifacts/public-truth/m5-governance-dashboard.md`, the machine-readable fitness-tile matrix at
//! `artifacts/public-truth/m5-governance-dashboard-tiles.csv`, the release-grade parity proof under
//! `artifacts/public-truth/m5-governance-dashboard-proof/` (and its Markdown report), the exported
//! evaluation packet, and the per-state drill fixtures under
//! `fixtures/public-truth/m5-governance-dashboard/`. The dashboard turns Aureline's protected fitness
//! functions, nightly governance runs, accepted waivers, service ownership, and decision rights into
//! freshness-aware tiles and cards whose state is derived from the inputs backing them, so a tile can
//! never read greener than its proof.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- overview
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- evaluation
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- variant <canonical|warning|stale|waiver|expired|missing>
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- tile <function-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_governance_dashboard -- validate
//! ```

use aureline_release::m5_governance_dashboard::{
    seeded_m5_governance_dashboard, seeded_m5_governance_dashboard_evidence_stale_narrowed,
    seeded_m5_governance_dashboard_missing_evidence_blocked,
    seeded_m5_governance_dashboard_waiver_active_narrowed,
    seeded_m5_governance_dashboard_waiver_expired_blocked, seeded_m5_governance_dashboard_warning,
    M5GovernanceDashboard,
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
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("overview") => {
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_overview_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_tiles_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("evaluation") => {
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            println!("{}", packet.render_evaluation_packet());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("tile") => {
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let tile = packet
                .fitness_tiles
                .iter()
                .find(|t| t.function.as_str() == token)
                .ok_or_else(|| format!("unknown fitness-function token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(tile)?);
        }
        Some("validate") => {
            let packet = seeded_m5_governance_dashboard();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_governance_dashboard_warning())?;
            assert_packet_valid(&seeded_m5_governance_dashboard_evidence_stale_narrowed())?;
            assert_packet_valid(&seeded_m5_governance_dashboard_waiver_active_narrowed())?;
            assert_packet_valid(&seeded_m5_governance_dashboard_waiver_expired_blocked())?;
            assert_packet_valid(&seeded_m5_governance_dashboard_missing_evidence_blocked())?;
            println!(
                "ok: governance dashboard valid ({} tiles, {} services, {} decisions, {} profiles)",
                packet.fitness_tiles.len(),
                packet.service_cards.len(),
                packet.decision_right_cards.len(),
                packet.overviews.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5GovernanceDashboard, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_governance_dashboard()),
        "warning" => Ok(seeded_m5_governance_dashboard_warning()),
        "stale" => Ok(seeded_m5_governance_dashboard_evidence_stale_narrowed()),
        "waiver" => Ok(seeded_m5_governance_dashboard_waiver_active_narrowed()),
        "expired" => Ok(seeded_m5_governance_dashboard_waiver_expired_blocked()),
        "missing" => Ok(seeded_m5_governance_dashboard_missing_evidence_blocked()),
        other => Err(format!(
            "unknown variant: {other} (canonical|warning|stale|waiver|expired|missing)"
        )
        .into()),
    }
}

fn assert_packet_valid(packet: &M5GovernanceDashboard) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
