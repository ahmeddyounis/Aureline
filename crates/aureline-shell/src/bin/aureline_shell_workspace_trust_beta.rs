//! Headless inspector for the beta workspace-trust audit projection.
//!
//! The bin emits the same audited records consumed by the shell trust center,
//! support-export wrapper, diagnostics views, and fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- validate
//! ```

use aureline_auth::{CapabilityAuthorityClass, LaunchWedgeCapabilityFamily};
use aureline_shell::workspace_trust_beta::{
    audit_workspace_trust_beta_rows, seeded_workspace_trust_beta_page,
    validate_workspace_trust_beta_page, WorkspaceTrustBetaPage, WorkspaceTrustBetaRenderSummary,
    WorkspaceTrustBetaSummary, WorkspaceTrustBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_workspace_trust_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("support-rows") => print_json(&page.support_rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = WorkspaceTrustBetaSupportExport::from_page(
                "support-export:workspace-trust-beta:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = WorkspaceTrustBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-trust-bypass") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| row.surface_family == LaunchWedgeCapabilityFamily::TasksRun)
                .ok_or("seeded page must include tasks_run")?;
            row.restricted_authority = CapabilityAuthorityClass::Allowed;
            row.restricted_authority_token = CapabilityAuthorityClass::Allowed.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-support-drift") => {
            let mut page = page;
            let support = page
                .support_rows
                .iter_mut()
                .find(|row| row.surface_family_token == "ai_apply_mutation")
                .ok_or("seeded page must include ai_apply_mutation support row")?;
            support.restricted_authority_token = "allowed".to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-public-fallback") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|row| {
                    row.surface_family == LaunchWedgeCapabilityFamily::ConnectedProviderToolCall
                })
                .ok_or("seeded page must include connected_provider_tool_call")?;
            row.no_public_endpoint_fallback = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_workspace_trust_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} field={} note={}",
                        defect.defect_kind_token, defect.row_id, defect.field, defect.note,
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

fn rebuild_with_defects(mut page: WorkspaceTrustBetaPage) -> WorkspaceTrustBetaPage {
    page.defects = audit_workspace_trust_beta_rows(&page.rows, &page.support_rows);
    page.summary =
        WorkspaceTrustBetaSummary::from_rows(&page.rows, &page.support_rows, &page.defects);
    page
}
