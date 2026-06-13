//! Emits the seeded networked-surface transport-automation fixtures.
//!
//! This is the CLI/headless consumer of the transport audit and automation
//! layer: the same canonical denial vocabulary, network-activity filters,
//! route/origin history joins, and redaction-safe automation packets the
//! product surfaces show are emitted here as machine-readable JSON, and the
//! CLI activity view (`activity-cli`) renders each record through the shared
//! field catalog so headless output quotes identical denial codes and field
//! names.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- records
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- joins
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- facets
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- activity-cli
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- denied-filter
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- drill-missing-surface-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- drill-bypass-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- drill-non-idempotent-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- drill-denied-no-code-beta
//! cargo run -q -p aureline-remote --example dump_networked_surface_transport_automation_fixtures -- drill-disposition-mismatch-beta
//! ```

use aureline_remote::{
    seeded_transport_automation_page, seeded_transport_automation_snapshot,
    ActivityDispositionClass, ActivityFilter, NetworkedSurfaceClass as SurfaceClass,
    TransportAutomationPage, TransportAutomationSupportExport, TransportDenialClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_transport_automation_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("records") => print_json(&page.activity_snapshot.records)?,
        Some("joins") => print_json(&page.route_origin_joins)?,
        Some("facets") => print_json(&page.filter_facets)?,
        Some("support-export") => {
            let export = TransportAutomationSupportExport::from_page(
                "remote:networked_surface_transport_automation:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("activity-cli") => {
            // Headless activity view: print the shared field catalog as
            // `key=value` lines per record, proving CLI/support/product parity.
            for record in &page.activity_snapshot.records {
                println!("# {} ({})", record.surface_label, record.disposition_token);
                for line in record.render_cli_lines() {
                    println!("{line}");
                }
                println!();
            }
        }
        Some("denied-filter") => {
            // Headless network-activity filter: every denied action across all
            // surfaces, quoting the canonical denial code.
            let denied = page
                .filter(&ActivityFilter::all().with_disposition(ActivityDispositionClass::Denied));
            for record in denied {
                println!("{} {}", record.surface_token, record.denial_code_token);
            }
        }
        Some("drill-missing-surface-preview") => {
            let mut snapshot = seeded_transport_automation_snapshot();
            snapshot
                .records
                .retain(|r| r.surface != SurfaceClass::AiGateway);
            let drill = TransportAutomationPage::new(
                "remote:networked_surface_transport_automation:drill:missing-surface",
                "Drill — required surface absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_transport_automation_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == SurfaceClass::DocsBrowserFetcher {
                    r.raw_private_material_excluded = false;
                }
            }
            let drill = TransportAutomationPage::new(
                "remote:networked_surface_transport_automation:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-bypass-withdrawn") => {
            let mut snapshot = seeded_transport_automation_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == SurfaceClass::RequestApiClient {
                    r.no_bypass = false;
                }
            }
            let drill = TransportAutomationPage::new(
                "remote:networked_surface_transport_automation:drill:bypass",
                "Drill — shared-governance bypass (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-non-idempotent-withdrawn") => {
            let mut snapshot = seeded_transport_automation_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == SurfaceClass::CompanionHandoff {
                    r.action_is_idempotent = false;
                }
            }
            let drill = TransportAutomationPage::new(
                "remote:networked_surface_transport_automation:drill:non-idempotent",
                "Drill — non-idempotent action queued for replay (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-denied-no-code-beta") => {
            let mut snapshot = seeded_transport_automation_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == SurfaceClass::ProviderMutation {
                    r.denial_code = TransportDenialClass::None;
                    r.denial_code_token = TransportDenialClass::None.as_str().to_owned();
                }
            }
            let drill = TransportAutomationPage::new(
                "remote:networked_surface_transport_automation:drill:denied-no-code",
                "Drill — denied without a canonical code (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-disposition-mismatch-beta") => {
            let mut snapshot = seeded_transport_automation_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.surface == SurfaceClass::AiGateway {
                    // An allowed action carrying a denial code is inconsistent.
                    r.denial_code = TransportDenialClass::CaUntrusted;
                    r.denial_code_token = TransportDenialClass::CaUntrusted.as_str().to_owned();
                }
            }
            let drill = TransportAutomationPage::new(
                "remote:networked_surface_transport_automation:drill:disposition-mismatch",
                "Drill — disposition/denial-code mismatch (beta)",
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
