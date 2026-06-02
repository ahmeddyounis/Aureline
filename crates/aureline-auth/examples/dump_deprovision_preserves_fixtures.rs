//! Emits the seeded deprovision-preserves beta fixtures.
//!
//! The example is a one-shot helper that prints either the seeded page,
//! rows, defects, support-export, summary, or a named failure drill so
//! reviewer-facing fixtures can be regenerated:
//!
//! ```sh
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- page
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- rows
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- defects
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- support-export
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- summary
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- drill-silent-purge
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- drill-blocking-exit
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- drill-missing-export-opportunity
//! cargo run -q -p aureline-auth --example dump_deprovision_preserves_fixtures -- drill-affordance-without-notice
//! ```

use aureline_auth::{
    seeded_deprovision_preserves_beta_page, DeprovisionPreservesBetaPage,
    DeprovisionPreservesBetaSupportExport, LocalWorkPreservationClass, ManagedExitEventClass,
    OrgAffordanceClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_deprovision_preserves_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = DeprovisionPreservesBetaSupportExport::from_page(
                "auth:deprovision-preserves-export:stable:0001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-silent-purge") => {
            let mut rows = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.exit_event_token == ManagedExitEventClass::Deprovision.as_str())
            {
                row.local_work_survival.local_editing_token =
                    LocalWorkPreservationClass::SilentlyPurged
                        .as_str()
                        .to_owned();
            }
            let dirty = DeprovisionPreservesBetaPage::new(
                "auth:deprovision_preserves:drill-silent-purge",
                "Drill: local editing silently purged on deprovision (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&dirty)?;
        }
        Some("drill-blocking-exit") => {
            let mut rows = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.exit_event_token == ManagedExitEventClass::SignOut.as_str())
            {
                row.local_work_survival.local_editing_token =
                    LocalWorkPreservationClass::PreservedReadOnly
                        .as_str()
                        .to_owned();
            }
            let dirty = DeprovisionPreservesBetaPage::new(
                "auth:deprovision_preserves:drill-blocking-exit",
                "Drill: managed exit blocks local editing (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&dirty)?;
        }
        Some("drill-missing-export-opportunity") => {
            let mut rows = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.exit_event_token == ManagedExitEventClass::OrgSwitch.as_str())
            {
                row.local_work_survival.prior_export_opportunity = false;
            }
            let dirty = DeprovisionPreservesBetaPage::new(
                "auth:deprovision_preserves:drill-missing-export-opportunity",
                "Drill: org-switch row missing prior export opportunity (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&dirty)?;
        }
        Some("drill-affordance-without-notice") => {
            let mut rows = page.rows;
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.exit_event_token == ManagedExitEventClass::SeatLoss.as_str())
            {
                row.org_affordance.collab_session_token =
                    OrgAffordanceClass::RemovedWithoutNotice.as_str().to_owned();
            }
            let dirty = DeprovisionPreservesBetaPage::new(
                "auth:deprovision_preserves:drill-affordance-without-notice",
                "Drill: collab session removed without notice on seat-loss (beta)",
                "2026-06-01T00:00:00Z",
                rows,
            );
            print_json(&dirty)?;
        }
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            std::process::exit(2);
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
