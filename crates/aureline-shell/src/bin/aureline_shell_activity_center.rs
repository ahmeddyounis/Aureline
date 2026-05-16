//! Headless inspector for the beta activity-center projection.
//!
//! The bin emits the same beta records consumed by the live shell, by
//! the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- badges
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- validate
//! ```

use aureline_shell::activity_center::beta::{
    seeded_activity_center_beta_page, validate_activity_center_beta_page,
    validate_activity_center_beta_support_export, ActivityCenterBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_activity_center_beta_page();

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
        Some("support-export") => {
            let export = ActivityCenterBetaSupportExport::from_page(
                "support-export:activity-center-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => {
            let page_errors = validate_activity_center_beta_page(&page);
            let export = ActivityCenterBetaSupportExport::from_page(
                "support-export:activity-center-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            let export_errors = validate_activity_center_beta_support_export(&export);
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
