//! Headless emitter for the update-center summary objects.
//!
//! The bin is the only mint-from-truth path for the published summary inventory checked in at
//! `artifacts/release/m5-update-center-summary.json`, the release-grade parity proof under
//! `artifacts/release/m5-update-center-summary-proof/` (and its Markdown report), the
//! machine-readable artifact-class delta export at
//! `artifacts/release/m5-update-center-summary-delta.csv`, and the per-state summary fixtures under
//! `fixtures/release/update-center-summary/`. It summarizes every claimed M5 artifact family — desktop
//! app, extension, docs pack, policy bundle, framework pack, runtime / toolchain — with its current
//! and target version, verification / restart / rollback truth, and release-data state, and
//! auto-narrows a consumer the moment any family it reads goes stale or loses live data.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- variant <canonical|stale|not-provided>
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- validate
//! ```

use aureline_release::m5_update_summary::{
    seeded_m5_update_center_summary, seeded_m5_update_center_summary_not_provided_blocked,
    seeded_m5_update_center_summary_stale_data_narrowed, M5UpdateCenterSummary,
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
            let packet = seeded_m5_update_center_summary();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_update_center_summary();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_update_center_summary();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_delta_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_update_center_summary();
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
            let packet = seeded_m5_update_center_summary();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_update_center_summary_stale_data_narrowed())?;
            assert_packet_valid(&seeded_m5_update_center_summary_not_provided_blocked())?;
            println!(
                "ok: update-center summary valid ({} families, {} delta rows, {} consumers)",
                packet.entries.len(),
                packet.summary.total_delta_rows,
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5UpdateCenterSummary, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_update_center_summary()),
        "stale" => Ok(seeded_m5_update_center_summary_stale_data_narrowed()),
        "not-provided" => Ok(seeded_m5_update_center_summary_not_provided_blocked()),
        other => Err(format!("unknown variant: {other} (canonical|stale|not-provided)").into()),
    }
}

fn assert_packet_valid(packet: &M5UpdateCenterSummary) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
