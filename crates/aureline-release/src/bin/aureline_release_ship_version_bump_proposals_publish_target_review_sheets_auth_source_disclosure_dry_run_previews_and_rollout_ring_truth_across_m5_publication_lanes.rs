//! Headless emitter for the M5 publication review-sheet register: version-bump
//! proposals, publish-target review sheets, auth-source disclosure, dry-run
//! previews, and rollout-ring truth for every M5 publication lane.
//!
//! Regenerates the checked-in artifact and the on-disk fixtures from the in-code
//! builder so the release center, support export, and diagnostics surfaces all
//! quote the same record renders as the embedded corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_version_bump_proposals_publish -- register
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_version_bump_proposals_publish -- support-export
//!
//! # Regenerate the checked-in artifact from the builder.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_version_bump_proposals_publish -- emit-artifact \
//!   artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_version_bump_proposals_publish -- emit-fixtures \
//!   fixtures/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes
//! ```

use std::path::PathBuf;

use aureline_release::ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes::{
    build_publication_review_register, current_publication_review_register,
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
            let register = current_publication_review_register()?;
            println!("{}", serde_json::to_string_pretty(&register)?);
            Ok(())
        }
        Some("support-export") => {
            let register = current_publication_review_register()?;
            let projection = register.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("validate") => {
            let register = current_publication_review_register()?;
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
            let register = build_publication_review_register();
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

    let register = current_publication_review_register()?;
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
