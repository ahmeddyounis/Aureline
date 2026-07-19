//! Headless emitter for the M5 publication-certification register, publish-target
//! and mirror/offline parity posture, and downgrade automation.
//!
//! Mints the on-disk fixtures under
//! `fixtures/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows/`
//! so the release center, support export, and diagnostics surfaces all quote the
//! same record renders as the in-code corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_artifact_graph_publication_exact -- register
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_artifact_graph_publication_exact -- support-export
//!
//! # Print the computed summary and promotion verdict as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_artifact_graph_publication_exact -- computed
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_artifact_graph_publication_exact -- emit-fixtures \
//!   fixtures/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows
//! ```

use std::path::PathBuf;

use aureline_release::certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows::current_m5_publication_cert_register;

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
            let reg = current_m5_publication_cert_register()?;
            println!("{}", serde_json::to_string_pretty(&reg)?);
            Ok(())
        }
        Some("support-export") => {
            let reg = current_m5_publication_cert_register()?;
            let projection = reg.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("computed") => {
            let reg = current_m5_publication_cert_register()?;
            let value = serde_json::json!({
                "decision": reg.computed_promotion_decision().as_str(),
                "blocking_rule_ids": reg.computed_blocking_rule_ids(),
                "blocking_claim_ids": reg.computed_blocking_entry_ids(),
                "summary": reg.computed_summary(),
            });
            println!("{}", serde_json::to_string_pretty(&value)?);
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures requires a target directory argument")?;
            emit_fixtures(dir)
        }
        Some("validate") => {
            let reg = current_m5_publication_cert_register()?;
            let violations = reg.validate();
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

    let reg = current_m5_publication_cert_register()?;
    let violations = reg.validate();
    if !violations.is_empty() {
        return Err(format!("register has {} violation(s)", violations.len()).into());
    }

    std::fs::write(
        base.join("register.json"),
        serde_json::to_string_pretty(&reg)?,
    )?;

    let projection = reg.support_export_projection();
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&projection)?,
    )?;

    println!("emitted {} fixtures to {}", 2, base.display());
    Ok(())
}
