//! Headless emitter for the frozen M5 historical-reference matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-historical-evidence/`, its matrix CSV, the Markdown design report at
//! `artifacts/program/m5-historical-reference-matrix.md`, the historical-evidence-health dashboard at
//! `dashboards/m5-historical-evidence-health.json`, and the narrowed fixtures under
//! `fixtures/recovery/m5-historical-snapshots/`. The shell, help / docs, support, review / incident,
//! runbook-archive, and companion / export surfaces read this matrix so archived and imported / offline
//! evidence never looks live, writable, or current by omission, no live target is reopened from a snapshot
//! without validating identity, trust, route, and authority, and no expired / removed artifact is dead-linked
//! when metadata, provenance, or a safe cleanup state can be shown.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- fixture-imported-offline-route-evidence-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- fixture-review-incident-snapshot-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- validate
//! ```

use aureline_ui::m5_historical_reference_matrix::{
    seeded_m5_historical_reference_matrix,
    seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed,
    seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed,
    M5HistoricalReferenceMatrixPacket,
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
            let packet = seeded_m5_historical_reference_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_historical_reference_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_historical_reference_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_historical_reference_matrix().render_dashboard_json()
            );
        }
        Some("fixture-imported-offline-route-evidence-beta-narrowed") => {
            let packet =
                seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-incident-snapshot-preview-narrowed") => {
            let packet =
                seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_historical_reference_matrix(),
                seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed(
                ),
                seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed(),
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

fn assert_valid(
    packet: &M5HistoricalReferenceMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
