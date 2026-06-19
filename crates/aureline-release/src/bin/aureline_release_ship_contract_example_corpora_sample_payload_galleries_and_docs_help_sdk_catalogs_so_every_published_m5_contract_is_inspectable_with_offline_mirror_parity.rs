//! In-product inspect surface for the M5 contract catalog.
//!
//! This is the headless inspect surface the acceptance anchor calls for: it
//! resolves the same catalog entry and sample payload gallery the Help/About,
//! SDK, docs, and support-export surfaces publish, from the one checked-in
//! catalog, with no live service.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full catalog as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity -- catalog
//!
//! # Inspect one family: its catalog entry plus the sample payload gallery
//! # the docs/SDK publication links to.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity -- inspect command_descriptors
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity -- support-export
//!
//! # Validate the checked-in catalog.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity -- validate
//! ```

use std::path::PathBuf;

use aureline_release::ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity::current_m5_contract_catalog;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("catalog") => {
            let catalog = current_m5_contract_catalog()?;
            println!("{}", serde_json::to_string_pretty(&catalog)?);
            Ok(())
        }
        Some("inspect") => {
            let family_id = args.get(1).ok_or("inspect requires a family id argument")?;
            inspect(family_id)
        }
        Some("support-export") => {
            let catalog = current_m5_contract_catalog()?;
            let projection = catalog.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("validate") => {
            let catalog = current_m5_contract_catalog()?;
            let violations = catalog.validate();
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

/// Resolves a family's catalog entry and its sample payload gallery, the same
/// pair the docs/SDK publication renders, and prints them as one inspect view.
fn inspect(family_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let catalog = current_m5_contract_catalog()?;
    let entry = catalog
        .family(family_id)
        .ok_or_else(|| format!("unknown family id: {family_id}"))?;

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let gallery_path = repo_root.join(&entry.example_gallery_ref);
    let gallery: serde_json::Value = match std::fs::read_to_string(&gallery_path) {
        Ok(raw) => serde_json::from_str(&raw)?,
        Err(_) => serde_json::Value::Null,
    };

    let view = serde_json::json!({
        "family_id": entry.family_id,
        "lifecycle_label": entry.lifecycle_label,
        "contract_identity": entry.contract_identity,
        "compatibility_note_ref": entry.compatibility_note_ref,
        "example_gallery_ref": entry.example_gallery_ref,
        "catalog_entry": entry,
        "gallery": gallery,
    });
    println!("{}", serde_json::to_string_pretty(&view)?);
    Ok(())
}
