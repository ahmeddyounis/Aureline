//! Emits the seeded networked-surface transport-decision fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-missing-surface-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-bypass-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-silent-public-fallback-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-non-idempotent-replay-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-denied-no-reason-beta
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_decision_fixtures -- drill-stale-proof-beta
//! ```

use aureline_remote::{
    seeded_transport_decision_page, seeded_transport_decision_snapshot, NetworkedSurfaceClass,
    ProofFreshnessClass, TransportDecisionLogPage, TransportDecisionSupportExport,
    TransportOutcomeClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_transport_decision_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = TransportDecisionSupportExport::from_page(
                "remote:networked_surface_transport_decision:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-surface-preview") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            snapshot
                .decisions
                .retain(|d| d.surface != NetworkedSurfaceClass::AiGateway);
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:missing-surface",
                "Drill — required surface absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::RequestApiClient {
                    d.raw_private_material_excluded = false;
                }
            }
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-bypass-withdrawn") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::DatabaseCloudConnector {
                    d.no_bypass = false;
                }
            }
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:bypass",
                "Drill — governance bypass (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-silent-public-fallback-withdrawn") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::RegistryRead {
                    d.policy.no_silent_public_fallback = false;
                }
            }
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:silent-fallback",
                "Drill — silent public fall-through permitted (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-non-idempotent-replay-withdrawn") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::SyncOffboarding {
                    d.outcome = TransportOutcomeClass::OfflineDeferred;
                    d.outcome_token = TransportOutcomeClass::OfflineDeferred.as_str().to_owned();
                    d.action_is_idempotent = false;
                }
            }
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:non-idempotent-replay",
                "Drill — non-idempotent replay queued (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-denied-no-reason-beta") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::ProviderMutation {
                    d.outcome = TransportOutcomeClass::Denied;
                    d.outcome_token = TransportOutcomeClass::Denied.as_str().to_owned();
                    d.denial_reason = None;
                    d.denial_reason_token = String::new();
                }
            }
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:denied-no-reason",
                "Drill — denied without reason (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-stale-proof-beta") => {
            let mut snapshot = seeded_transport_decision_snapshot();
            for d in snapshot.decisions.iter_mut() {
                if d.surface == NetworkedSurfaceClass::DocsBrowserFetcher {
                    d.policy.trust_proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
                    d.policy.trust_proof_freshness_token =
                        ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
                }
            }
            let drill = TransportDecisionLogPage::new(
                "remote:networked_surface_transport_decision:drill:stale-proof",
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
