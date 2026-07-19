//! In-product inspect surface for the M5 interchange-conformance register.
//!
//! This headless surface resolves the same register, validators, and promotion decision the
//! conformance report and Help page publish, from the one checked-in register, with no live
//! service.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_import_export_validators_cross -- register
//!
//! # Inspect one family: its validator, runner, dimensions, and decision.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_import_export_validators_cross -- inspect support_bundles
//!
//! # Print the support/export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_import_export_validators_cross -- support
//!
//! # Print the promotion decision and exit non-zero when promotion is held.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_import_export_validators_cross -- gate
//!
//! # Validate the checked-in register.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_add_import_export_validators_cross -- validate
//! ```

use aureline_release::add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families::{
    current_m5_interchange_conformance_register, DecisionState,
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
            let register = current_m5_interchange_conformance_register()?;
            println!("{}", serde_json::to_string_pretty(&register)?);
            Ok(())
        }
        Some("inspect") => {
            let family_id = args.get(1).ok_or("inspect requires a family id argument")?;
            let register = current_m5_interchange_conformance_register()?;
            let row = register
                .row(family_id)
                .ok_or_else(|| format!("unknown family id: {family_id}"))?;
            println!("{}", serde_json::to_string_pretty(row)?);
            Ok(())
        }
        Some("support") => {
            let register = current_m5_interchange_conformance_register()?;
            let projection = register.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("gate") => {
            // The promotion-gate surface: print the decision and exit non-zero when
            // interchange conformance holds promotion, so CI and release tooling can fail
            // directly from this register.
            let register = current_m5_interchange_conformance_register()?;
            println!("{}", serde_json::to_string_pretty(&register.blockers)?);
            if register.blockers.decision == DecisionState::Hold {
                std::process::exit(1);
            }
            Ok(())
        }
        Some("validate") => {
            let register = current_m5_interchange_conformance_register()?;
            let violations = register.validate();
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
