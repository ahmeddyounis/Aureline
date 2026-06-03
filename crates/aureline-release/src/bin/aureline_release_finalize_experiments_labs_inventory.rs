//! Headless emitter for the stable experiments/Labs inventory certification corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/release/m4/finalize-experiments-labs-inventory/` so the release
//! center, support export, and diagnostics surfaces all quote the same record
//! renders as the in-code corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full certification page as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_finalize_experiments_labs_inventory -- page
//!
//! # Print the CLI projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_finalize_experiments_labs_inventory -- cli
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_finalize_experiments_labs_inventory -- support-export
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_finalize_experiments_labs_inventory -- emit-fixtures \
//!   fixtures/release/m4/finalize-experiments-labs-inventory
//! ```

use std::path::PathBuf;

use aureline_release::finalize_experiments_labs_inventory::{
    audit_finalize_experiments_labs_inventory_page,
    project_cli_inventory,
    project_support_export,
    seeded_finalize_experiments_labs_inventory_page,
    validate_finalize_experiments_labs_inventory_page,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("page") => {
            let page = seeded_finalize_experiments_labs_inventory_page();
            println!("{}", serde_json::to_string_pretty(&page)?);
            Ok(())
        }
        Some("cli") => {
            let page = seeded_finalize_experiments_labs_inventory_page();
            let cli = project_cli_inventory(&page);
            println!("{}", serde_json::to_string_pretty(&cli)?);
            Ok(())
        }
        Some("support-export") => {
            let page = seeded_finalize_experiments_labs_inventory_page();
            let export = project_support_export("fixture:support-export:experiments-labs-inventory", &page);
            println!("{}", serde_json::to_string_pretty(&export)?);
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures requires a target directory argument")?;
            emit_fixtures(dir)
        }
        Some("audit") => {
            let page = seeded_finalize_experiments_labs_inventory_page();
            validate_finalize_experiments_labs_inventory_page(&page)?;
            let extra_defects = audit_finalize_experiments_labs_inventory_page(&page);
            if extra_defects.is_empty() {
                println!("audit: clean (no defects)");
            } else {
                println!(
                    "audit: {} extra defect(s)",
                    extra_defects.len()
                );
                for defect in &extra_defects {
                    println!("- {}: {}", defect.defect_kind, defect.description);
                }
            }
            Ok(())
        }
        Some(cmd) => Err(format!("unknown subcommand: {cmd}").into()),
    }
}

fn emit_fixtures(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base = PathBuf::from(dir);
    std::fs::create_dir_all(&base)?;

    let page = seeded_finalize_experiments_labs_inventory_page();
    validate_finalize_experiments_labs_inventory_page(&page)?;

    std::fs::write(
        base.join("certification_page.json"),
        serde_json::to_string_pretty(&page)?,
    )?;

    let cli = project_cli_inventory(&page);
    std::fs::write(
        base.join("cli_projection.json"),
        serde_json::to_string_pretty(&cli)?,
    )?;

    let export = project_support_export("fixture:support-export:experiments-labs-inventory", &page);
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&export)?,
    )?;

    println!("emitted {} fixtures to {}", 3, base.display());
    Ok(())
}
