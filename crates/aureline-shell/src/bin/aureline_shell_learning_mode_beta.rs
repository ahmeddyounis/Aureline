//! Headless inspector for beta guided tours and learning mode.
//!
//! The binary emits deterministic manifest, surface projection, and support
//! export records consumed by fixtures, docs, and release evidence.
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_learning_mode_beta -- manifest
//! cargo run -q -p aureline-shell --bin aureline_shell_learning_mode_beta -- surfaces
//! cargo run -q -p aureline-shell --bin aureline_shell_learning_mode_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_learning_mode_beta -- validate
//! ```

use aureline_commands::registry::seeded_registry;
use aureline_shell::learning_mode::{
    seeded_learning_mode_beta_manifest, seeded_learning_mode_beta_support_export,
    seeded_learning_mode_beta_surface_projection, validate_seeded_learning_mode_beta,
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
        Some("manifest") | None => print_json(&seeded_learning_mode_beta_manifest())?,
        Some("surfaces") => print_json(&seeded_learning_mode_beta_surface_projection())?,
        Some("support-export") => print_json(&seeded_learning_mode_beta_support_export())?,
        Some("validate") => match validate_seeded_learning_mode_beta(seeded_registry()) {
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
