//! Headless emitter for the per-family release-candidate graph: release
//! candidates, blocker/evidence-freshness rows, and scoped artifact-bundle cards
//! for every new M5 artifact family.
//!
//! Regenerates the checked-in artifact and the on-disk fixtures from the in-code
//! builder so the release center, support export, and diagnostics surfaces all
//! quote the same record renders as the embedded corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full graph as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family -- graph
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family -- support-export
//!
//! # Regenerate the checked-in artifact from the builder.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family -- emit-artifact \
//!   artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family -- emit-fixtures \
//!   fixtures/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family
//! ```

use std::path::PathBuf;

use aureline_release::implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family::{
    build_m5_family_release_graph, current_m5_family_release_graph,
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
        None | Some("graph") => {
            let graph = current_m5_family_release_graph()?;
            println!("{}", serde_json::to_string_pretty(&graph)?);
            Ok(())
        }
        Some("support-export") => {
            let graph = current_m5_family_release_graph()?;
            let projection = graph.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("validate") => {
            let graph = current_m5_family_release_graph()?;
            let violations = graph.validate();
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
            let graph = build_m5_family_release_graph();
            let violations = graph.validate();
            if !violations.is_empty() {
                return Err(format!("builder graph has {} violation(s)", violations.len()).into());
            }
            let mut json = serde_json::to_string_pretty(&graph)?;
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

    let graph = current_m5_family_release_graph()?;
    let violations = graph.validate();
    if !violations.is_empty() {
        return Err(format!("graph has {} violation(s)", violations.len()).into());
    }

    std::fs::write(
        base.join("graph.json"),
        serde_json::to_string_pretty(&graph)?,
    )?;

    let projection = graph.support_export_projection();
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&projection)?,
    )?;

    println!("emitted {} fixtures to {}", 2, base.display());
    Ok(())
}
