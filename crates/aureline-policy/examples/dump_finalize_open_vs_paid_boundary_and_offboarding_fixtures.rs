//! Emits the seeded open-vs-paid boundary and offboarding fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- usage-export-packets
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- offboarding-packets
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- drill-local-core-managed-withdrawn
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- drill-missing-offboarding-beta
//! cargo run -q -p aureline-policy --example dump_finalize_open_vs_paid_boundary_and_offboarding_fixtures -- drill-stale-export-schema-beta
//! ```

use aureline_policy::{
    seeded_open_vs_paid_boundary_input, seeded_open_vs_paid_boundary_page,
    CapabilityBoundaryClass, CapabilityFamilyClass,
    OpenVsPaidBoundaryPage, OpenVsPaidBoundarySupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_open_vs_paid_boundary_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = OpenVsPaidBoundarySupportExport::from_page(
                "policy:open-vs-paid-boundary:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("usage-export-packets") => {
            let packets: Vec<_> = page
                .rows
                .iter()
                .filter_map(|r| r.usage_export_packet.clone())
                .collect();
            print_json(&packets)?;
        }
        Some("offboarding-packets") => {
            let packets: Vec<_> = page
                .rows
                .iter()
                .filter_map(|r| r.offboarding_packet.clone())
                .collect();
            print_json(&packets)?;
        }
        Some("drill-local-core-managed-withdrawn") => {
            let mut input = seeded_open_vs_paid_boundary_input();
            if let Some(row) = input
                .capability_rows
                .iter_mut()
                .find(|r| r.capability_family == CapabilityFamilyClass::EditorCore)
            {
                row.boundary_class = CapabilityBoundaryClass::ManagedHosted;
            }
            let drill_page = OpenVsPaidBoundaryPage::new(
                "policy:open_vs_paid_boundary:drill:local-core-managed",
                "Drill — local core classified as managed (withdrawn)",
                "2026-06-01T00:00:00Z",
                input,
            );
            print_json(&drill_page)?;
        }
        Some("drill-missing-offboarding-beta") => {
            let mut input = seeded_open_vs_paid_boundary_input();
            if let Some(row) = input
                .capability_rows
                .iter_mut()
                .find(|r| r.capability_family == CapabilityFamilyClass::Collaboration)
            {
                row.offboarding_disclosed = false;
            }
            let drill_page = OpenVsPaidBoundaryPage::new(
                "policy:open_vs_paid_boundary:drill:missing-offboarding",
                "Drill — missing offboarding disclosure (beta)",
                "2026-06-01T00:00:00Z",
                input,
            );
            print_json(&drill_page)?;
        }
        Some("drill-stale-export-schema-beta") => {
            let mut input = seeded_open_vs_paid_boundary_input();
            input.usage_export_schema_version_current = false;
            let drill_page = OpenVsPaidBoundaryPage::new(
                "policy:open_vs_paid_boundary:drill:stale-export-schema",
                "Drill — stale usage-export schema version (beta)",
                "2026-06-01T00:00:00Z",
                input,
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
