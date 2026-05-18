//! Headless inspector for the scoped companion and desktop-handoff beta
//! contract.
//!
//! The bin emits the same audited records consumed by the live shell, docs,
//! fixtures, support export, and release-evidence audit.
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- validate
//! ```

use aureline_shell::companion_handoff::{
    audit_companion_scope_beta_rows, seeded_companion_scope_beta_page,
    validate_companion_scope_beta_page, ApprovalOwnerSurfaceClass, CompanionScopeBetaPage,
    CompanionScopeBetaSupportExport, CompanionScopeBetaSupportRow, CompanionWorkflowClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_companion_scope_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("support-rows") => print_json(&page.support_rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = CompanionScopeBetaSupportExport::from_page(
                "support-export:companion-scope:001",
                "2026-05-18T17:05:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-stale-label-missing") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.workflow == CompanionWorkflowClass::CiStatus)
                .ok_or("seeded page must include a CI status row")?;
            row.freshness.stale_or_offline_label_visible = false;
            row.labels.freshness_label_visible = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-companion-owns-protected-approval") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.workflow == CompanionWorkflowClass::ReviewTriage)
                .ok_or("seeded page must include a review triage row")?;
            row.authority.companion_can_grant_approval = true;
            row.authority.step_up.approval_owner_surface =
                ApprovalOwnerSurfaceClass::CompanionRequestOnly;
            row.authority.step_up.approval_owner_surface_token =
                ApprovalOwnerSurfaceClass::CompanionRequestOnly
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_companion_scope_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} field={} note={}",
                        defect.defect_kind_token, defect.row_id, defect.field, defect.note
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

fn rebuild_with_defects(mut page: CompanionScopeBetaPage) -> CompanionScopeBetaPage {
    page.support_rows = page
        .rows
        .iter()
        .map(CompanionScopeBetaSupportRow::from_row)
        .collect();
    page.defects = audit_companion_scope_beta_rows(&page.rows, &page.support_rows);
    page.summary.defect_count = page.defects.len();
    page
}
