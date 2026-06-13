//! Emits the seeded networked-surface transport-explainability fixtures.
//!
//! This is the CLI/headless consumer of the explainability layer: the same
//! posture inspectors, recent network-event ledger, and per-action explain
//! sheets the product surfaces show are emitted here as machine-readable JSON,
//! and the CLI explain view (`explain-cli`) renders the per-action sheets
//! through the shared field catalog so headless output quotes identical decision
//! codes and field names.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- posture-inspectors
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- event-ledger
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- explain-sheets
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- explain-cli
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- drill-missing-surface-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- drill-bypass-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- drill-denied-no-reason-beta
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_explainability_fixtures -- drill-stale-proof-beta
//! ```

use aureline_remote::{
    seeded_transport_explainability_page, seeded_transport_explainability_snapshot,
    NetworkedSurfaceClass, ProofFreshnessClass, TransportExplainabilityPage,
    TransportExplainabilitySupportExport, TransportOutcomeClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_transport_explainability_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("posture-inspectors") => print_json(&page.posture_inspectors)?,
        Some("event-ledger") => print_json(&page.event_ledger)?,
        Some("explain-sheets") => print_json(&page.explain_sheets)?,
        Some("support-export") => {
            let export = TransportExplainabilitySupportExport::from_page(
                "remote:networked_surface_transport_explainability:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("explain-cli") => {
            // Headless explain view: print the shared field catalog as
            // `key=value` lines per action, proving CLI/support/product parity.
            for sheet in &page.explain_sheets {
                println!("# {} ({})", sheet.surface_label, sheet.disposition_token);
                for line in sheet.render_cli_lines() {
                    println!("{line}");
                }
                println!();
            }
        }
        Some("drill-missing-surface-preview") => {
            let mut snapshot = seeded_transport_explainability_snapshot();
            snapshot
                .decisions
                .retain(|d| d.surface != NetworkedSurfaceClass::AiGateway);
            let drill = TransportExplainabilityPage::from_decision_snapshot(
                "remote:networked_surface_transport_explainability:drill:missing-surface",
                "Drill — required surface absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_transport_explainability_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::DatabaseCloudConnector {
                    d.raw_private_material_excluded = false;
                }
            }
            let drill = TransportExplainabilityPage::from_decision_snapshot(
                "remote:networked_surface_transport_explainability:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-bypass-withdrawn") => {
            let mut snapshot = seeded_transport_explainability_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::ProviderMutation {
                    d.no_bypass = false;
                }
            }
            let drill = TransportExplainabilityPage::from_decision_snapshot(
                "remote:networked_surface_transport_explainability:drill:bypass",
                "Drill — governance bypass (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-denied-no-reason-beta") => {
            let mut snapshot = seeded_transport_explainability_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::ProviderMutation {
                    d.outcome = TransportOutcomeClass::Denied;
                    d.outcome_token = TransportOutcomeClass::Denied.as_str().to_owned();
                    d.denial_reason = None;
                    d.denial_reason_token = String::new();
                }
            }
            let drill = TransportExplainabilityPage::from_decision_snapshot(
                "remote:networked_surface_transport_explainability:drill:denied-no-reason",
                "Drill — denied without reason (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-stale-proof-beta") => {
            let mut snapshot = seeded_transport_explainability_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::DocsBrowserFetcher {
                    d.policy.trust_proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
                    d.policy.trust_proof_freshness_token =
                        ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
                }
            }
            let drill = TransportExplainabilityPage::from_decision_snapshot(
                "remote:networked_surface_transport_explainability:drill:stale-proof",
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
