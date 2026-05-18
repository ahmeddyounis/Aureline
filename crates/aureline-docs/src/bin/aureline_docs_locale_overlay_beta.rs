//! Headless inspector for translated-pack locale overlays.
//!
//! The binary emits deterministic manifest, surface projection, and support
//! export records consumed by fixtures, docs, and release evidence.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- manifest
//! cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- surfaces
//! cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- validate
//! ```

use aureline_docs::{
    seeded_translated_pack_locale_overlay_contract,
    seeded_translated_pack_locale_overlay_support_export,
    seeded_translated_pack_locale_overlay_surface_projection,
    validate_seeded_translated_pack_locale_overlay,
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
        Some("manifest") | None => print_json(&seeded_translated_pack_locale_overlay_contract())?,
        Some("surfaces") => print_json(&seeded_translated_pack_locale_overlay_surface_projection())?,
        Some("support-export") => {
            print_json(&seeded_translated_pack_locale_overlay_support_export())?
        }
        Some("validate") => match validate_seeded_translated_pack_locale_overlay() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!("{} {}: {}", finding.row_ref, finding.check_id, finding.message);
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
