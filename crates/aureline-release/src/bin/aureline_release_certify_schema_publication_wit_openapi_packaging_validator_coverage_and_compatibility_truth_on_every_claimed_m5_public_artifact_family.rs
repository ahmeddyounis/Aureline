//! In-product inspect surface for the M5 public-contract certification register.
//!
//! This headless surface resolves the same certification packet, pillars, and promotion
//! decision the report, shiproom dashboard, and Help page publish, from the one checked-in
//! register, with no live service. It is the surface claim-publication, release-center,
//! support-center, and SDK/docs publication share.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full certification register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_schema_publication_wit_openapi -- register
//!
//! # Inspect one family: its pillars, certification state, reasons, and decision.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_schema_publication_wit_openapi -- inspect task_event_envelope
//!
//! # Print the support/export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_schema_publication_wit_openapi -- support
//!
//! # Print the promotion decision and exit non-zero when certification holds promotion.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_schema_publication_wit_openapi -- gate
//!
//! # Validate the checked-in register.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_certify_schema_publication_wit_openapi -- validate
//! ```

use aureline_release::certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family::{
    current_m5_public_contract_certification_register, DecisionState,
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
            let register = current_m5_public_contract_certification_register()?;
            println!("{}", serde_json::to_string_pretty(&register)?);
            Ok(())
        }
        Some("inspect") => {
            let family_id = args.get(1).ok_or("inspect requires a family id argument")?;
            let register = current_m5_public_contract_certification_register()?;
            let row = register
                .row(family_id)
                .ok_or_else(|| format!("unknown family id: {family_id}"))?;
            println!("{}", serde_json::to_string_pretty(row)?);
            Ok(())
        }
        Some("support") => {
            let register = current_m5_public_contract_certification_register()?;
            let projection = register.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("gate") => {
            // The promotion-gate surface: print the decision and exit non-zero when
            // certification holds promotion, so CI and release tooling can fail directly from
            // this register.
            let register = current_m5_public_contract_certification_register()?;
            println!("{}", serde_json::to_string_pretty(&register.promotion)?);
            if register.promotion.decision == DecisionState::Hold {
                std::process::exit(1);
            }
            Ok(())
        }
        Some("validate") => {
            let register = current_m5_public_contract_certification_register()?;
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
