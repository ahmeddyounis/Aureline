//! Headless inspector for the managed-boundary, org-switch, seat-quota,
//! grace-window, and offboarding beta surface.
//!
//! The bin loads the published boundary manifest, projects it through the
//! shell module, and emits the same audited records consumed by product
//! surfaces (org-switch / seat-quota / grace-window / offboarding flows),
//! the support-export wrapper, and the reviewer fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- org-switch-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- seat-quota-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- grace-window-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- offboarding-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- render-summary
//! cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- validate
//! ```

use aureline_shell::managed_boundary::{
    seeded_managed_boundary_beta_page, validate_managed_boundary_beta_page,
    ManagedBoundaryBetaRenderSummary, ManagedBoundaryBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_managed_boundary_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("org-switch-rows") => print_json(&page.org_switch_projection())?,
        Some("seat-quota-rows") => print_json(&page.seat_quota_projection())?,
        Some("grace-window-rows") => print_json(&page.grace_window_projection())?,
        Some("offboarding-rows") => print_json(&page.offboarding_projection())?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("render-summary") => {
            let summary = ManagedBoundaryBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("support-export") => {
            let export = ManagedBoundaryBetaSupportExport::from_page(
                "support-export:managed-boundary:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_managed_boundary_beta_page(&page) {
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
