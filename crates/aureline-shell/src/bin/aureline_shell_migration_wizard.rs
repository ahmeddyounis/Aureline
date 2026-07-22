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
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- rollback-requirement
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- header
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- issue-template
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- emit-fixtures fixtures/migration/m3/migration_wizard
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_wizard -- compact
//! ```

use aureline_shell::migration_wizard::{
    seeded_migration_wizard_page, validate_migration_wizard_page,
    MigrationWizardIssueTemplateExport, MigrationWizardPage, MigrationWizardSupportExport,
};
use std::path::Path;

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
        Some("rollback-requirement") => {
            print_json(&page.rollback_requirement)?;
        }
        Some("header") => {
            print_json(&page.header)?;
        }
        Some("support-export") => {
            let export = MigrationWizardSupportExport::from_page(
                "support-export:migration-wizard:001",
                page,
            )
            .map_err(|errors| format!("wizard support export validation failed: {errors:?}"))?;
            print_json(&export)?;
        }
        Some("issue-template") => {
            let export = MigrationWizardSupportExport::from_page(
                "support-export:migration-wizard:001",
                page,
            )
            .map_err(|errors| format!("wizard support export validation failed: {errors:?}"))?;
            let issue = MigrationWizardIssueTemplateExport::from_support_export(&export);
            print_json(&issue)?;
        }
        Some("emit-fixtures") => {
            let output_dir = args
                .get(1)
                .ok_or("emit-fixtures requires an output directory")?;
            emit_fixtures(Path::new(output_dir), &page)?;
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

fn emit_fixtures(
    output_dir: &Path,
    page: &MigrationWizardPage,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_dir)?;
    let export = MigrationWizardSupportExport::from_page(
        "support-export:migration-wizard:001",
        page.clone(),
    )
    .map_err(|errors| format!("wizard support export validation failed: {errors:?}"))?;
    let issue = MigrationWizardIssueTemplateExport::from_support_export(&export);
    for (filename, value) in [
        ("page.json", serde_json::to_value(page)?),
        (
            "mapping_report.json",
            serde_json::to_value(&page.mapping_report)?,
        ),
        (
            "unsupported_gaps.json",
            serde_json::to_value(&page.mapping_report.unsupported_gaps)?,
        ),
        (
            "compare_actions.json",
            serde_json::to_value(&page.compare_actions)?,
        ),
        (
            "undo_actions.json",
            serde_json::to_value(&page.undo_actions)?,
        ),
        (
            "stage_history.json",
            serde_json::to_value(&page.stage_history)?,
        ),
        (
            "rollback_requirement.json",
            serde_json::to_value(&page.rollback_requirement)?,
        ),
        ("support_export.json", serde_json::to_value(&export)?),
        ("issue_template.json", serde_json::to_value(&issue)?),
    ] {
        let mut payload = serde_json::to_string_pretty(&value)?;
        payload.push('\n');
        std::fs::write(output_dir.join(filename), payload)?;
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
