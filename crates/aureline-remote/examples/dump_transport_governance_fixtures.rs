//! Emits the seeded transport governance fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- drill-missing-lane-preview
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_transport_governance_fixtures -- drill-no-local-core-continuity-beta
//! ```

use aureline_remote::{
    seeded_transport_governance_page, seeded_transport_policy_snapshot, EgressLaneClass,
    TransportGovernancePage, TransportGovernanceSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_transport_governance_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = TransportGovernanceSupportExport::from_page(
                "remote:transport_governance:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-lane-preview") => {
            // Remove the AI lane to trigger a missing-required-lane preview defect.
            let mut snapshot = seeded_transport_policy_snapshot();
            snapshot
                .records
                .retain(|r| r.lane != EgressLaneClass::Ai);
            let drill_page = TransportGovernancePage::new(
                "remote:transport_governance:drill:missing-lane",
                "Drill — required lane absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-raw-material-withdrawn") => {
            // Place raw private material on the update lane to trigger withdrawal.
            let mut snapshot = seeded_transport_policy_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.lane == EgressLaneClass::Update {
                    rec.raw_private_material_excluded = false;
                }
            }
            let drill_page = TransportGovernancePage::new(
                "remote:transport_governance:drill:raw-material-withdrawn",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-no-local-core-continuity-beta") => {
            // Remove local-core-continuity flag from marketplace to trigger beta.
            let mut snapshot = seeded_transport_policy_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.lane == EgressLaneClass::Marketplace {
                    rec.local_core_continuity_allowed = false;
                }
            }
            let drill_page = TransportGovernancePage::new(
                "remote:transport_governance:drill:no-local-core-continuity",
                "Drill — no local core continuity declared on marketplace (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
