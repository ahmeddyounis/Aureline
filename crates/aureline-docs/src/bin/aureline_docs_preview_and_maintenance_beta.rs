//! Headless inspector for docs preview-and-maintenance records.
//!
//! The binary emits deterministic manifest, surface projection, and review
//! packet records consumed by fixtures, docs, and release evidence.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- manifest
//! cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- surfaces
//! cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- review-packet
//! cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- validate
//! ```

use aureline_docs::{
    seeded_docs_preview_and_maintenance_contract,
    seeded_docs_preview_and_maintenance_review_packet,
    seeded_docs_preview_and_maintenance_surface_projection,
    validate_seeded_docs_preview_and_maintenance,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("manifest") | None => print_json(&seeded_docs_preview_and_maintenance_contract())?,
        Some("surfaces") => print_json(&seeded_docs_preview_and_maintenance_surface_projection())?,
        Some("review-packet") => print_json(&seeded_docs_preview_and_maintenance_review_packet())?,
        Some("validate") => match validate_seeded_docs_preview_and_maintenance() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!(
                        "{} {}: {}",
                        finding.row_ref, finding.check_id, finding.message
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
