//! Emits the seeded networked-surface transport matrix fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- drill-missing-surface-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- drill-silent-public-fallback-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- drill-non-idempotent-replay-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_matrix_fixtures -- drill-stale-proof-beta
//! ```

use aureline_remote::{
    seeded_networked_surface_matrix_page, seeded_networked_surface_matrix_snapshot,
    NetworkedSurfaceClass, NetworkedSurfaceMatrixSupportExport,
    NetworkedSurfaceTransportMatrixPage, ProofFreshnessClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_networked_surface_matrix_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = NetworkedSurfaceMatrixSupportExport::from_page(
                "remote:networked_surface_transport_matrix:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-surface-preview") => {
            let mut snapshot = seeded_networked_surface_matrix_snapshot();
            snapshot
                .records
                .retain(|r| r.surface != NetworkedSurfaceClass::AiGateway);
            let drill = NetworkedSurfaceTransportMatrixPage::new(
                "remote:networked_surface_transport_matrix:drill:missing-surface",
                "Drill — required surface absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_networked_surface_matrix_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.surface == NetworkedSurfaceClass::RequestApiClient {
                    rec.raw_private_material_excluded = false;
                }
            }
            let drill = NetworkedSurfaceTransportMatrixPage::new(
                "remote:networked_surface_transport_matrix:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-silent-public-fallback-withdrawn") => {
            let mut snapshot = seeded_networked_surface_matrix_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.surface == NetworkedSurfaceClass::RegistryRead {
                    rec.no_silent_public_fallback = false;
                }
            }
            let drill = NetworkedSurfaceTransportMatrixPage::new(
                "remote:networked_surface_transport_matrix:drill:silent-fallback",
                "Drill — silent public fall-through permitted (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-non-idempotent-replay-withdrawn") => {
            let mut snapshot = seeded_networked_surface_matrix_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.surface == NetworkedSurfaceClass::SyncOffboarding {
                    rec.offline_deferral_allowed = true;
                    rec.replay_idempotent_only = false;
                }
            }
            let drill = NetworkedSurfaceTransportMatrixPage::new(
                "remote:networked_surface_transport_matrix:drill:non-idempotent-replay",
                "Drill — non-idempotent replay queued (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-stale-proof-beta") => {
            let mut snapshot = seeded_networked_surface_matrix_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.surface == NetworkedSurfaceClass::DocsBrowserFetcher {
                    rec.proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
                    rec.proof_freshness_token =
                        ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
                }
            }
            let drill = NetworkedSurfaceTransportMatrixPage::new(
                "remote:networked_surface_transport_matrix:drill:stale-proof",
                "Drill — stale proof beyond window (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
