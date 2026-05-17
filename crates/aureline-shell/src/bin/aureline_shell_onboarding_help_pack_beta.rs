//! Headless inspector for the beta onboarding/help-pack manifest.
//!
//! The binary emits the deterministic manifest, surface projection, and
//! support export consumed by fixtures, docs, and release evidence.
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- manifest
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- surfaces
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- validate
//! ```

use aureline_commands::registry::seeded_registry;
use aureline_shell::help_packs::onboarding_beta::{
    seeded_onboarding_help_pack_beta_manifest, seeded_onboarding_help_pack_beta_support_export,
    seeded_onboarding_help_pack_beta_surface_projection, validate_seeded_onboarding_help_pack_beta,
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
        Some("manifest") | None => print_json(&seeded_onboarding_help_pack_beta_manifest())?,
        Some("surfaces") => print_json(&seeded_onboarding_help_pack_beta_surface_projection())?,
        Some("support-export") => print_json(&seeded_onboarding_help_pack_beta_support_export())?,
        Some("validate") => match validate_seeded_onboarding_help_pack_beta(seeded_registry()) {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!("{}: {}", finding.row_ref, finding.message);
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
