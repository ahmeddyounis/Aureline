//! Headless emitter for the M5 provenance-card register: per-family
//! About/Help/service-health provenance cards that converge one exact-build
//! identity with signature, attestation, SBOM, symbol, mirror, and rollback
//! state across every user-visible surface.
//!
//! Regenerates the checked-in artifact and the on-disk fixtures from the in-code
//! builder so the About, Help, release-center, service-health, and support/export
//! surfaces all quote the same record renders as the embedded corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts -- register
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts -- support-export
//!
//! # Regenerate the checked-in artifact from the builder.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts -- emit-artifact \
//!   artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts -- emit-fixtures \
//!   fixtures/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts
//! ```

use std::path::PathBuf;

use aureline_release::add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts::{
    build_m5_provenance_cards, current_m5_provenance_cards,
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
        None | Some("register") => {
            let register = current_m5_provenance_cards()?;
            println!("{}", serde_json::to_string_pretty(&register)?);
            Ok(())
        }
        Some("support-export") => {
            let register = current_m5_provenance_cards()?;
            let projection = register.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("validate") => {
            let register = current_m5_provenance_cards()?;
            let violations = register.validate();
            if violations.is_empty() {
                println!("validate: clean (no violations)");
            } else {
                println!("validate: {} violation(s)", violations.len());
                for v in &violations {
                    println!("- {v}");
                }
                std::process::exit(1);
            }
            Ok(())
        }
        Some("emit-artifact") => {
            let path = args
                .get(1)
                .ok_or("emit-artifact requires a target file argument")?;
            let register = build_m5_provenance_cards();
            let violations = register.validate();
            if !violations.is_empty() {
                return Err(
                    format!("builder register has {} violation(s)", violations.len()).into(),
                );
            }
            let mut json = serde_json::to_string_pretty(&register)?;
            json.push('\n');
            std::fs::write(path, json)?;
            println!("wrote artifact to {path}");
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures requires a target directory argument")?;
            emit_fixtures(dir)
        }
        Some(cmd) => Err(format!("unknown subcommand: {cmd}").into()),
    }
}

fn emit_fixtures(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base = PathBuf::from(dir);
    std::fs::create_dir_all(&base)?;

    let register = current_m5_provenance_cards()?;
    let violations = register.validate();
    if !violations.is_empty() {
        return Err(format!("register has {} violation(s)", violations.len()).into());
    }

    std::fs::write(
        base.join("register.json"),
        serde_json::to_string_pretty(&register)?,
    )?;

    let projection = register.support_export_projection();
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&projection)?,
    )?;

    println!("emitted {} fixtures to {}", 2, base.display());
    Ok(())
}
