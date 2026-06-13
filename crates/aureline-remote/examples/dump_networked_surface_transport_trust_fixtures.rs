//! Emits the seeded networked-surface transport-trust fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- cli-view
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-missing-surface-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-private-key-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-ca-override-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-silent-downgrade-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-denied-no-reason-beta
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_trust_fixtures -- drill-missing-rotation-cue-beta
//! ```

use aureline_remote::{
    seeded_transport_trust_page, seeded_transport_trust_snapshot, HostProofStateClass,
    NetworkedSurfaceClass, RotationCueClass, TransportTrustPage, TrustOutcomeClass,
    TrustSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_transport_trust_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("cli-view") => print!("{}", page.render_cli_view()),
        Some("support-export") => {
            let export = TrustSupportExport::from_page(
                "remote:networked_surface_transport_trust:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-surface-preview") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            snapshot
                .records
                .retain(|r| r.surface != NetworkedSurfaceClass::AiGateway);
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:missing-surface",
                "Drill — required surface absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::RequestApiClient {
                    r.raw_trust_material_excluded = false;
                }
            }
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:raw-material",
                "Drill — raw trust material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-private-key-withdrawn") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::DatabaseCloudConnector {
                    r.private_key_material_excluded = false;
                }
            }
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:private-key",
                "Drill — raw private-key material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-ca-override-withdrawn") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::DocsBrowserFetcher {
                    r.ca_bundle.is_direct_ca_override = true;
                    r.no_direct_ca_override = false;
                }
            }
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:ca-override",
                "Drill — direct CA override shipped (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-silent-downgrade-withdrawn") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::RegistryRead {
                    r.no_silent_trust_downgrade = false;
                }
            }
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:silent-downgrade",
                "Drill — silent trust downgrade (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-denied-no-reason-beta") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::SyncOffboarding {
                    r.outcome = TrustOutcomeClass::DenyTrust;
                    r.outcome_token = TrustOutcomeClass::DenyTrust.as_str().to_owned();
                    r.denial_reason = None;
                    r.denial_reason_token = String::new();
                }
            }
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:denied-no-reason",
                "Drill — denied without reason (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-rotation-cue-beta") => {
            let mut snapshot = seeded_transport_trust_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == NetworkedSurfaceClass::DocsBrowserFetcher {
                    r.trust_root.rotation_cue = RotationCueClass::None;
                    r.trust_root.rotation_cue_token = RotationCueClass::None.as_str().to_owned();
                    r.host_proof.state = HostProofStateClass::KnownTofu;
                    r.host_proof.state_token = HostProofStateClass::KnownTofu.as_str().to_owned();
                    r.outcome = TrustOutcomeClass::Trusted;
                    r.outcome_token = TrustOutcomeClass::Trusted.as_str().to_owned();
                }
            }
            let drill = TransportTrustPage::new(
                "remote:networked_surface_transport_trust:drill:missing-cue",
                "Drill — rotation cue missing (beta)",
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
