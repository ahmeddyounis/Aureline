//! Headless emitter for the stable settings-definition-registry certification corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/settings/m4/finalize-settings-definition-registry/` so the
//! settings-definition registry, effective-configuration inspector, CLI,
//! diagnostics, support export, Help/About, migration review, and portable-state
//! artifacts all quote the same record renders as the in-code corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full certification page as JSON.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_finalize_settings_definition_registry -- page
//!
//! # Print the CLI projection as JSON.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_finalize_settings_definition_registry -- cli
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_finalize_settings_definition_registry -- support-export
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_finalize_settings_definition_registry -- emit-fixtures \
//!   fixtures/settings/m4/finalize-settings-definition-registry
//! ```

use std::path::PathBuf;

use aureline_settings::finalize_settings_definition_registry::{
    audit_finalize_settings_definition_registry_page, project_cli_inventory,
    project_support_export, seeded_finalize_settings_definition_registry_page,
    validate_finalize_settings_definition_registry_page,
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
            let page = seeded_finalize_settings_definition_registry_page();
            println!("{}", serde_json::to_string_pretty(&page)?);
            Ok(())
        }
        Some("cli") => {
            let page = seeded_finalize_settings_definition_registry_page();
            let cli = project_cli_inventory(&page);
            println!("{}", serde_json::to_string_pretty(&cli)?);
            Ok(())
        }
        Some("support-export") => {
            let page = seeded_finalize_settings_definition_registry_page();
            let export = project_support_export(
                "fixture:support-export:settings-definition-registry",
                &page,
            );
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
            let page = seeded_finalize_settings_definition_registry_page();
            validate_finalize_settings_definition_registry_page(&page)?;
            let extra_defects = audit_finalize_settings_definition_registry_page(&page);
            if extra_defects.is_empty() {
                println!("audit: clean (no defects)");
            } else {
                println!("audit: {} extra defect(s)", extra_defects.len());
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

    let page = seeded_finalize_settings_definition_registry_page();
    validate_finalize_settings_definition_registry_page(&page)?;

    std::fs::write(
        base.join("certification_page.json"),
        serde_json::to_string_pretty(&page)?,
    )?;

    let cli = project_cli_inventory(&page);
    std::fs::write(
        base.join("cli_projection.json"),
        serde_json::to_string_pretty(&cli)?,
    )?;

    let export =
        project_support_export("fixture:support-export:settings-definition-registry", &page);
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&export)?,
    )?;

    println!("emitted {} fixtures to {}", 3, base.display());
    Ok(())
}
