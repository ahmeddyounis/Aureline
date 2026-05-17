//! Headless inspector for the enterprise drill baseline page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-packets
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-backup-restore
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-failover
//! cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-key-rotation
//! ```

use aureline_shell::enterprise_drill_baseline::{
    seeded_enterprise_drill_baseline_page, validate_enterprise_drill_baseline_page,
    EnterpriseDrillBaselinePage, EnterpriseDrillBaselineRenderSummary,
    EnterpriseDrillBaselineSupportExport, EnterpriseDrillKindClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_enterprise_drill_baseline_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("drill-packets") => print_json(&page.drill_packets)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => {
            let summary = EnterpriseDrillBaselineRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("support-export") => {
            let export = EnterpriseDrillBaselineSupportExport::from_page(
                "support-export:enterprise-drill-baseline:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-backup-restore") => {
            print_drills_by_kind(&page, EnterpriseDrillKindClass::BackupRestore)?
        }
        Some("drill-failover") => print_drills_by_kind(&page, EnterpriseDrillKindClass::Failover)?,
        Some("drill-key-rotation") => {
            print_drills_by_kind(&page, EnterpriseDrillKindClass::KeyRotation)?
        }
        Some("validate") => match validate_enterprise_drill_baseline_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} subject_id={} field={} note={}",
                        defect.defect_kind_token, defect.subject_id, defect.field, defect.note,
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

fn print_drills_by_kind(
    page: &EnterpriseDrillBaselinePage,
    kind: EnterpriseDrillKindClass,
) -> Result<(), Box<dyn std::error::Error>> {
    let drills: Vec<_> = page
        .drill_packets
        .iter()
        .filter(|packet| packet.drill_kind == kind)
        .collect();
    print_json(&drills)?;
    Ok(())
}
