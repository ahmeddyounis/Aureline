//! Headless inspector for the beta migration-wizard projection.
//!
//! The bin emits the same wizard records consumed by the live shell,
//! by the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- mapping-report
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- unsupported-gaps
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- compare-actions
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- undo-actions
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- stage-history
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- compact
//! ```

use aureline_shell::migration_wizard::{
    seeded_migration_wizard_page, validate_migration_wizard_page, MigrationWizardSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_migration_wizard_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("mapping-report") => {
            print_json(&page.mapping_report)?;
        }
        Some("unsupported-gaps") => {
            print_json(&page.mapping_report.unsupported_gaps)?;
        }
        Some("compare-actions") => {
            print_json(&page.compare_actions)?;
        }
        Some("undo-actions") => {
            print_json(&page.undo_actions)?;
        }
        Some("stage-history") => {
            print_json(&page.stage_history)?;
        }
        Some("rollback-checkpoint") => {
            print_json(&page.rollback_checkpoint)?;
        }
        Some("support-export") => {
            let export = MigrationWizardSupportExport::from_page(
                "support-export:migration-wizard:001",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_migration_wizard_page(&page) {
            Ok(()) => {
                println!("ok");
            }
            Err(errors) => {
                for err in &errors {
                    eprintln!(
                        "error: {}",
                        serde_json::to_string(err).unwrap_or_else(|_| format!("{err:?}"))
                    );
                }
                std::process::exit(3);
            }
        },
        Some("compact") => {
            for line in page.compact_lines() {
                println!("{line}");
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
