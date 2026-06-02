//! Emits the seeded effective-policy stabilize fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- drill-raw-private-material-withdrawn
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- drill-missing-change-class-preview
//! cargo run -q -p aureline-policy --example dump_effective_policy_stabilize_fixtures -- drill-exception-preview-links-missing
//! ```

use aureline_policy::simulation::{
    audit_policy_simulation_beta_page, seeded_policy_simulation_beta_page,
};
use aureline_policy::{
    seeded_effective_policy_stabilize_page, EffectivePolicyStabilizePage,
    EffectivePolicyStabilizeSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_effective_policy_stabilize_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = EffectivePolicyStabilizeSupportExport::from_page(
                "policy:effective-policy-stabilize:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-private-material-withdrawn") => {
            let mut beta_page = seeded_policy_simulation_beta_page();
            beta_page.exceptions[0].raw_justification_excluded = false;
            beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
            let drill_page = EffectivePolicyStabilizePage::new(
                "policy:effective_policy_stabilize:drill:raw-private-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                beta_page,
            );
            print_json(&drill_page)?;
        }
        Some("drill-missing-change-class-preview") => {
            let mut beta_page = seeded_policy_simulation_beta_page();
            beta_page
                .simulations
                .retain(|s| s.request.change_class_token != "policy_bundle_change");
            beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
            let drill_page = EffectivePolicyStabilizePage::new(
                "policy:effective_policy_stabilize:drill:missing-change-class",
                "Drill — required change class missing (preview)",
                "2026-06-01T00:00:00Z",
                beta_page,
            );
            print_json(&drill_page)?;
        }
        Some("drill-exception-preview-links-missing") => {
            let mut beta_page = seeded_policy_simulation_beta_page();
            for sim in beta_page.simulations.iter_mut() {
                sim.exception_preview_refs.clear();
            }
            beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
            let drill_page = EffectivePolicyStabilizePage::new(
                "policy:effective_policy_stabilize:drill:exception-preview-links-missing",
                "Drill — exception preview links missing (beta)",
                "2026-06-01T00:00:00Z",
                beta_page,
            );
            print_json(&drill_page)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
