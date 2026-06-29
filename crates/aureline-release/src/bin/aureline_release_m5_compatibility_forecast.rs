//! Headless emitter for the compatibility-forecast sheet and migration-assistant task rows.
//!
//! The bin is the only mint-from-truth path for the published forecast inventory checked in at
//! `artifacts/release/m5-compatibility-forecast.json`, the release-grade migration-assistant parity
//! proof under `artifacts/release/m5-migration-assistant-proof/` (and its Markdown report), the
//! machine-readable per-task CSV export at `artifacts/release/m5-migration-tasks.csv`, and the
//! per-state drill fixtures under `fixtures/release/compatibility-forecast/`. It forecasts, before
//! restart or rollout widening, how a staged M5 update drifts the qualified subjects — certified
//! archetypes, extension SDK / manifest ranges, remote-agent skew, and public export / schema readers —
//! on the stable / beta / preview / LTS lines, routes every drift to a concrete migration task, and
//! labels out-of-window and speculative inputs honestly instead of raising them as hard failures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- variant <canonical|review|hold|out-of-window>
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_compatibility_forecast -- validate
//! ```

use aureline_release::m5_compatibility_forecast::{
    seeded_m5_compatibility_forecast_sheet, seeded_m5_compatibility_forecast_sheet_hold,
    seeded_m5_compatibility_forecast_sheet_out_of_window,
    seeded_m5_compatibility_forecast_sheet_review, CompatibilityForecastSheet,
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
            let packet = seeded_m5_compatibility_forecast_sheet();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_compatibility_forecast_sheet();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_compatibility_forecast_sheet();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_task_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_compatibility_forecast_sheet();
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
            let packet = seeded_m5_compatibility_forecast_sheet();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_compatibility_forecast_sheet_review())?;
            assert_packet_valid(&seeded_m5_compatibility_forecast_sheet_hold())?;
            assert_packet_valid(&seeded_m5_compatibility_forecast_sheet_out_of_window())?;
            println!(
                "ok: compatibility forecast valid ({} subjects, {} tasks, {} consumers)",
                packet.subjects.len(),
                packet.migration_tasks.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<CompatibilityForecastSheet, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_compatibility_forecast_sheet()),
        "review" => Ok(seeded_m5_compatibility_forecast_sheet_review()),
        "hold" => Ok(seeded_m5_compatibility_forecast_sheet_hold()),
        "out-of-window" => Ok(seeded_m5_compatibility_forecast_sheet_out_of_window()),
        other => Err(format!(
            "unknown variant: {other} (canonical|review|hold|out-of-window)"
        )
        .into()),
    }
}

fn assert_packet_valid(
    packet: &CompatibilityForecastSheet,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
