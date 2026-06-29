//! Headless emitter for the M5 update / support-lifecycle governance matrix.
//!
//! The bin is the only mint-from-truth path for the published governance inventory checked in at
//! `artifacts/release/m5-update-lifecycle-summary.json`, the rendered governance document at
//! `artifacts/release/m5-update-lifecycle-governance.md`, the machine-readable matrix at
//! `artifacts/release/m5-update-lifecycle-matrix.csv`, the release-grade parity proof under
//! `artifacts/release-proof/m5-update-lifecycle/` (and its Markdown report), and the per-state
//! governance fixtures under `fixtures/release/m5-update-center/`. The matrix maps every claimed
//! M5 update / help / support-lifecycle surface to its governed facets, owners, proof paths,
//! disclosed artifact classes, channel scope, profiles, and stale-data behavior, and auto-narrows
//! a consumer's claim the moment any facet it reads goes stale or fails.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- governance
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- variant <canonical|stale|missing>
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- validate
//! ```

use aureline_release::m5_update_lifecycle::{
    seeded_m5_update_lifecycle, seeded_m5_update_lifecycle_missing_proof_blocked,
    seeded_m5_update_lifecycle_stale_proof_narrowed, M5UpdateLifecycleGovernance,
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
            let packet = seeded_m5_update_lifecycle();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("governance") => {
            let packet = seeded_m5_update_lifecycle();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_governance_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_update_lifecycle();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_update_lifecycle();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_update_lifecycle();
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
            let packet = seeded_m5_update_lifecycle();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_update_lifecycle_stale_proof_narrowed())?;
            assert_packet_valid(&seeded_m5_update_lifecycle_missing_proof_blocked())?;
            println!(
                "ok: update-lifecycle matrix valid ({} facets, {} consumers, {} state families)",
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

fn parse_variant(token: &str) -> Result<M5UpdateLifecycleGovernance, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_update_lifecycle()),
        "stale" => Ok(seeded_m5_update_lifecycle_stale_proof_narrowed()),
        "missing" => Ok(seeded_m5_update_lifecycle_missing_proof_blocked()),
        other => Err(format!("unknown variant: {other} (canonical|stale|missing)").into()),
    }
}

fn assert_packet_valid(
    packet: &M5UpdateLifecycleGovernance,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
