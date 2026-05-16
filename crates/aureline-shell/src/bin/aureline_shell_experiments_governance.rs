//! Headless inspector for the beta experiments / flags / Labs governance
//! UI projection.
//!
//! The bin emits the same records consumed by the live shell, the
//! support-export wrapper, and the integration test that replays the
//! checked-in inventory.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- badges
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- cli
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- validate
//! ```

use aureline_shell::experiments_governance::{
    project_labs_governance_beta_cli, project_labs_governance_beta_support_export,
    seeded_experiments_governance_beta_page, validate_labs_governance_beta_page,
    validate_labs_governance_beta_support_export, ExperimentsGovernanceRenderSummary,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_experiments_governance_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("rows") => {
            print_json(&page.rows)?;
        }
        Some("badges") => {
            print_json(&page.badges)?;
        }
        Some("summary") => {
            let summary = ExperimentsGovernanceRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("cli") => {
            let cli = project_labs_governance_beta_cli(&page);
            print_json(&cli)?;
        }
        Some("support-export") => {
            let export = project_labs_governance_beta_support_export(
                "support-export:experiments-labs-governance-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => {
            let page_errors = validate_labs_governance_beta_page(&page);
            let export = project_labs_governance_beta_support_export(
                "support-export:experiments-labs-governance-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            let export_errors = validate_labs_governance_beta_support_export(&export);
            match (page_errors, export_errors) {
                (Ok(()), Ok(())) => {
                    println!("ok");
                }
                (page_result, export_result) => {
                    if let Err(errors) = page_result {
                        for err in errors {
                            eprintln!("page error: {err}");
                        }
                    }
                    if let Err(errors) = export_result {
                        for err in errors {
                            eprintln!("support-export error: {err}");
                        }
                    }
                    std::process::exit(3);
                }
            }
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
