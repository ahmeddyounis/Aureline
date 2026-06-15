//! Headless emitter for the M5 artifact-graph promotion-ledger register:
//! per-family promotion timelines, immutable-digest joins, release-center/headless
//! reconstruction parity, and break-glass event capture.
//!
//! Regenerates the checked-in artifact and the on-disk fixtures from the in-code
//! builder so the release center, support export, audit, and diagnostics surfaces
//! all quote the same record renders as the embedded corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs -- register
//!
//! # Print the audit/postmortem support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs -- support-export
//!
//! # Regenerate the checked-in artifact from the builder.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs -- emit-artifact \
//!   artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs -- emit-fixtures \
//!   fixtures/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs
//! ```

use std::path::PathBuf;

use aureline_release::implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs::{
    build_m5_artifact_graph_promotion_ledger, current_m5_artifact_graph_promotion_ledger,
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
            let register = current_m5_artifact_graph_promotion_ledger()?;
            println!("{}", serde_json::to_string_pretty(&register)?);
            Ok(())
        }
        Some("support-export") => {
            let register = current_m5_artifact_graph_promotion_ledger()?;
            let projection = register.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("validate") => {
            let register = current_m5_artifact_graph_promotion_ledger()?;
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
            let register = build_m5_artifact_graph_promotion_ledger();
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

    let register = current_m5_artifact_graph_promotion_ledger()?;
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
