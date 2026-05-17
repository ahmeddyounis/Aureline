//! Headless inspector for the beta Start Center and workspace switcher.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- primary-actions
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- switcher-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- privacy-modes
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_start_center -- validate
//! ```

use aureline_shell::start_center::beta::{
    seeded_start_center_switcher_beta_page, validate_start_center_switcher_beta_page,
    validate_start_center_switcher_beta_support_export, StartCenterSwitcherBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_start_center_switcher_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("primary-actions") => print_json(&page.primary_actions)?,
        Some("rows") => print_json(&page.start_center_rows)?,
        Some("switcher-rows") => print_json(&page.workspace_switcher_rows)?,
        Some("privacy-modes") => print_json(&page.privacy_modes)?,
        Some("support-export") => {
            let export = StartCenterSwitcherBetaSupportExport::from_page(
                "support-export:start-center-switcher-beta:001",
                "2026-05-17T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => {
            let page_result = validate_start_center_switcher_beta_page(&page);
            let export = StartCenterSwitcherBetaSupportExport::from_page(
                "support-export:start-center-switcher-beta:001",
                "2026-05-17T00:00:00Z",
                page,
            );
            let export_result = validate_start_center_switcher_beta_support_export(&export);
            match (page_result, export_result) {
                (Ok(()), Ok(())) => println!("ok"),
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
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
