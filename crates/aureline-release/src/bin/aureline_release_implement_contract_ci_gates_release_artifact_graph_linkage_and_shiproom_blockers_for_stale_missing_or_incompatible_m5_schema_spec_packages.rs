//! In-product inspect surface for the M5 contract-health register.
//!
//! This headless surface resolves the same register, CI gates, and shiproom
//! blocker decision the dashboard and Help page publish, from the one checked-in
//! register, with no live service.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_contract_ci_gates_release -- register
//!
//! # Inspect one family: its gates, graph linkage, and blocker decision.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_contract_ci_gates_release -- inspect task_event_envelope
//!
//! # Print the shiproom blocker projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_contract_ci_gates_release -- shiproom
//!
//! # Print the promotion gate decision and exit non-zero when promotion is held.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_contract_ci_gates_release -- gate
//!
//! # Validate the checked-in register.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_implement_contract_ci_gates_release -- validate
//! ```

use aureline_release::implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages::{
    current_m5_contract_health_register, BlockerDecision,
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
            let register = current_m5_contract_health_register()?;
            println!("{}", serde_json::to_string_pretty(&register)?);
            Ok(())
        }
        Some("inspect") => {
            let family_id = args.get(1).ok_or("inspect requires a family id argument")?;
            let register = current_m5_contract_health_register()?;
            let row = register
                .row(family_id)
                .ok_or_else(|| format!("unknown family id: {family_id}"))?;
            println!("{}", serde_json::to_string_pretty(row)?);
            Ok(())
        }
        Some("shiproom") => {
            let register = current_m5_contract_health_register()?;
            let projection = register.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("gate") => {
            // The promotion-gate surface: print the decision and exit non-zero
            // when contract health holds promotion, so CI and release tooling can
            // fail directly from this register.
            let register = current_m5_contract_health_register()?;
            println!("{}", serde_json::to_string_pretty(&register.blockers)?);
            if register.blockers.decision == BlockerDecision::Hold {
                std::process::exit(1);
            }
            Ok(())
        }
        Some("validate") => {
            let register = current_m5_contract_health_register()?;
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
