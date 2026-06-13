//! Emits the seeded M5 transport-governance certification fixtures.
//!
//! This is the CLI/headless consumer of the transport-governance certification
//! layer: the same per-profile certification verdicts, per-dimension proof
//! cells, evidence bindings, and narrowing the product surfaces show are emitted
//! here as machine-readable JSON, and the cell CLI view (`cells-cli`) renders
//! each cell through the shared field catalog so headless output quotes
//! identical state and evidence tokens.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- bindings
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- profiles
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- cells-cli
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- drill-stale-narrowed
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- drill-missing-continuity-held
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- drill-missing-profile-held
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_m5_transport_governance_certification_fixtures -- drill-fallthrough-withdrawn
//! ```

use aureline_remote::{
    seeded_m5_transport_governance_certification_page,
    seeded_m5_transport_governance_certification_snapshot, CertificationCellStateClass,
    CertificationDimensionClass, CertificationProfileClass, CertificationSupportExport,
    M5TransportGovernanceCertificationPage, ProofFreshnessClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_m5_transport_governance_certification_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("bindings") => print_json(&page.dimension_bindings)?,
        Some("profiles") => print_json(&page.certification_snapshot.profiles)?,
        Some("support-export") => {
            let export = CertificationSupportExport::from_page(
                "remote:m5_transport_governance_certification:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("cells-cli") => {
            // Headless certification view: print the shared field catalog as
            // `key=value` lines per cell, proving CLI/support/product parity.
            for profile in &page.certification_snapshot.profiles {
                println!("# {} ({})", profile.profile_label, profile.profile_token);
                for cell in &profile.cells {
                    for line in cell.render_cli_lines() {
                        println!("{line}");
                    }
                    println!();
                }
            }
        }
        Some("drill-stale-narrowed") => {
            let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
            for profile in snapshot.profiles.iter_mut() {
                if profile.profile == CertificationProfileClass::SelfHosted {
                    for cell in profile.cells.iter_mut() {
                        if cell.dimension == CertificationDimensionClass::TrustStore {
                            cell.state = CertificationCellStateClass::Stale;
                            cell.state_token =
                                CertificationCellStateClass::Stale.as_str().to_owned();
                            cell.freshness = ProofFreshnessClass::ExpiredBeyondWindow;
                            cell.freshness_token =
                                ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
                        }
                    }
                }
            }
            let drill = M5TransportGovernanceCertificationPage::new(
                "remote:m5_transport_governance_certification:drill:stale",
                "Drill — stale trust proof (narrowed)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-continuity-held") => {
            let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
            for profile in snapshot.profiles.iter_mut() {
                if profile.profile == CertificationProfileClass::Managed {
                    profile
                        .cells
                        .retain(|c| c.dimension != CertificationDimensionClass::MirrorOffline);
                }
            }
            let drill = M5TransportGovernanceCertificationPage::new(
                "remote:m5_transport_governance_certification:drill:missing-continuity",
                "Drill — missing mirror/offline continuity (held back)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-profile-held") => {
            let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
            snapshot
                .profiles
                .retain(|p| p.profile != CertificationProfileClass::AirGapped);
            let drill = M5TransportGovernanceCertificationPage::new(
                "remote:m5_transport_governance_certification:drill:missing-profile",
                "Drill — required profile absent (held back)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
            for profile in snapshot.profiles.iter_mut() {
                if profile.profile == CertificationProfileClass::LocalOss {
                    profile.raw_private_material_excluded = false;
                }
            }
            let drill = M5TransportGovernanceCertificationPage::new(
                "remote:m5_transport_governance_certification:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-fallthrough-withdrawn") => {
            let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
            for profile in snapshot.profiles.iter_mut() {
                if profile.profile == CertificationProfileClass::AirGapped {
                    profile.no_silent_public_fallthrough = false;
                }
            }
            let drill = M5TransportGovernanceCertificationPage::new(
                "remote:m5_transport_governance_certification:drill:fallthrough",
                "Drill — silent mirror-to-public fallthrough (withdrawn)",
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
