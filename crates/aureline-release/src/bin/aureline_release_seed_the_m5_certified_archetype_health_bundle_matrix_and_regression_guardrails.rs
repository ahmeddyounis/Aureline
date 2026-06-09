//! Headless emitter for the M5 certified-archetype health-bundle matrix and regression guardrails.
//!
//! Mints the on-disk fixtures under
//! `fixtures/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails/`
//! so the release center, support export, and diagnostics surfaces all quote the same record
//! renders as the in-code corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full matrix as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails -- matrix
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails -- support-export
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails -- emit-fixtures \
//!   fixtures/release/m5/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails
//! ```

use std::path::PathBuf;

use aureline_release::seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails::
    current_m5_health_bundle_matrix;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("matrix") => {
            let m = current_m5_health_bundle_matrix()?;
            println!("{}", serde_json::to_string_pretty(&m)?);
            Ok(())
        }
        Some("support-export") => {
            let m = current_m5_health_bundle_matrix()?;
            let projection = m.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures requires a target directory argument")?;
            emit_fixtures(dir)
        }
        Some("validate") => {
            let m = current_m5_health_bundle_matrix()?;
            let violations = m.validate();
            if violations.is_empty() {
                println!("validate: clean (no violations)");
            } else {
                println!("validate: {} violation(s)", violations.len());
                for v in &violations {
                    println!("- {}", v);
                }
                std::process::exit(1);
            }
            Ok(())
        }
        Some(cmd) => Err(format!("unknown subcommand: {cmd}").into()),
    }
}

fn emit_fixtures(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base = PathBuf::from(dir);
    std::fs::create_dir_all(&base)?;

    let m = current_m5_health_bundle_matrix()?;
    let violations = m.validate();
    if !violations.is_empty() {
        return Err(format!("matrix has {} violation(s)", violations.len()).into());
    }

    std::fs::write(base.join("matrix.json"), serde_json::to_string_pretty(&m)?)?;

    let projection = m.support_export_projection();
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&projection)?,
    )?;

    println!("emitted {} fixtures to {}", 2, base.display());
    Ok(())
}
