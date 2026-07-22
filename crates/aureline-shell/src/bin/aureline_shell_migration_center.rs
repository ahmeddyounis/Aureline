//! Headless inspector for the beta migration center / learnability surface.
//!
//! Emits the same records consumed by the live shell, the support-export
//! wrapper, and the integration test that replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- sections
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- entries
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- drill-account-detour
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- drill-missing-command-id
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- drill-review-overdue
//! cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- emit-fixtures fixtures/ux/m3/migration_center
//! ```

use std::path::{Path, PathBuf};

use aureline_shell::migration_center::{
    audit_migration_center_rows, seeded_migration_center_page, validate_migration_center_page,
    FreshnessClass, KeyboardReachClass, MigrationCenterPage, MigrationCenterSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_migration_center_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("sections") => {
            print_json(&page.sections)?;
        }
        Some("entries") => {
            print_json(&page.entries)?;
        }
        Some("support-rows") => {
            print_json(&page.support_rows)?;
        }
        Some("defects") => {
            print_json(&page.defects)?;
        }
        Some("summary") => {
            print_json(&page.summary)?;
        }
        Some("support-export") => {
            let export = MigrationCenterSupportExport::from_page(
                "support-export:migration-center-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            )
            .map_err(|defects| {
                format!(
                    "migration-center support export blocked by {} validation defect(s)",
                    defects.len()
                )
            })?;
            print_json(&export)?;
        }
        Some("validate") => match validate_migration_center_page(&page) {
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
        Some("drill-account-detour") => {
            let mut page = page;
            page.entries[0].requires_account_detour = true;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-missing-command-id") => {
            let mut page = page;
            let idx = page
                .entries
                .iter()
                .position(|e| {
                    e.keyboard_reach == KeyboardReachClass::KeyboardFirstCommandInvocation
                })
                .ok_or("seeded page must include a keyboard-first entry")?;
            page.entries[idx].command_id = None;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-review-overdue") => {
            let mut page = page;
            page.entries[0].learnability_claim.freshness_class = FreshnessClass::ReviewOverdue;
            page.entries[0].learnability_claim.freshness_class_token =
                FreshnessClass::ReviewOverdue.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures <dir> requires a target directory")?;
            emit_fixtures(PathBuf::from(dir), &page)?;
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

fn rebuild_with_defects(mut page: MigrationCenterPage) -> MigrationCenterPage {
    page.defects = audit_migration_center_rows(&page.sections, &page.entries, &page.support_rows);
    page.summary.defect_count = page.defects.len();
    page
}

fn emit_fixtures(
    dir: PathBuf,
    page: &MigrationCenterPage,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    write_json(&dir.join("page.json"), page)?;
    write_json(&dir.join("sections.json"), &page.sections)?;
    write_json(&dir.join("entries.json"), &page.entries)?;
    write_json(&dir.join("support_rows.json"), &page.support_rows)?;
    write_json(&dir.join("defects.json"), &page.defects)?;
    write_json(&dir.join("summary.json"), &page.summary)?;
    let support_export = MigrationCenterSupportExport::from_page(
        "support-export:migration-center-beta:001",
        "2026-05-15T00:00:00Z",
        page.clone(),
    )
    .map_err(|defects| {
        format!(
            "migration-center support export blocked by {} validation defect(s)",
            defects.len()
        )
    })?;
    write_json(&dir.join("support_export.json"), &support_export)?;
    Ok(())
}

fn write_json<T: serde::Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}
