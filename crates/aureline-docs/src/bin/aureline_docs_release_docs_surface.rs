//! Headless emitter for the release-docs maintenance contract and its fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_release_docs_surface -- contract
//! cargo run -q -p aureline-docs --bin aureline_docs_release_docs_surface -- projection
//! cargo run -q -p aureline-docs --bin aureline_docs_release_docs_surface -- review-packet
//! cargo run -q -p aureline-docs --bin aureline_docs_release_docs_surface -- surface release-docs-surface:readme:installed-stable
//! cargo run -q -p aureline-docs --bin aureline_docs_release_docs_surface -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_release_docs_surface -- validate
//! ```

use aureline_docs::{
    seeded_release_docs_maintenance_contract, seeded_release_docs_review_packet,
    seeded_release_docs_surface_projection, validate_seeded_release_docs_maintenance,
};
use serde::Serialize;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("contract") | None => emit_contract()?,
        Some("projection") => emit_projection()?,
        Some("review-packet") => emit_review_packet()?,
        Some("surface") => emit_surface(args.get(1).map(String::as_str))?,
        Some("summary") => emit_summary(),
        Some("validate") => validate_contract(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_contract() -> Result<(), Box<dyn std::error::Error>> {
    print_json(&seeded_release_docs_maintenance_contract())
}

fn emit_projection() -> Result<(), Box<dyn std::error::Error>> {
    print_json(&seeded_release_docs_surface_projection())
}

fn emit_review_packet() -> Result<(), Box<dyn std::error::Error>> {
    print_json(&seeded_release_docs_review_packet())
}

fn emit_surface(surface_id: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let surface_id = surface_id.ok_or("surface id is required")?;
    let contract = seeded_release_docs_maintenance_contract();
    let surface = contract
        .surface(surface_id)
        .ok_or_else(|| format!("unknown surface: {surface_id}"))?;
    print_json(surface)
}

fn emit_summary() {
    let contract = seeded_release_docs_maintenance_contract();
    let projection = contract.surface_projection();
    println!("# Release-docs maintenance surfaces");
    println!();
    println!("- Contract: `{}`", contract.contract_id);
    println!("- Version: `{}`", contract.contract_version_ref);
    println!("- Surfaces: {}", projection.coverage.surface_count);
    println!(
        "- Compare entries: {}",
        projection.coverage.compare_entry_count
    );
    println!(
        "- Integration anchors: {}",
        projection.coverage.integration_anchor_count
    );
    println!();
    for surface in &contract.surfaces {
        println!(
            "- `{}` — {} / evidence `{}` / boundary `{}`",
            surface.surface_id,
            surface.artifact_kind.as_str(),
            surface.evidence_scope.as_str(),
            surface.publish_boundary_state.as_str()
        );
        println!("  - {}", surface.active_scope_summary);
    }
}

fn validate_contract() {
    match validate_seeded_release_docs_maintenance() {
        Ok(()) => println!("ok"),
        Err(findings) => {
            for finding in &findings {
                eprintln!(
                    "{}: {} ({})",
                    finding.check_id, finding.message, finding.row_ref
                );
            }
            std::process::exit(3);
        }
    }
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
