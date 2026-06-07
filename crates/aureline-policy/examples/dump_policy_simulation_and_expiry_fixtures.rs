//! Emits the seeded policy simulation, exception, approval, and expiry fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- review-packet
//! cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- support-export
//! ```

use aureline_policy::simulation::{
    audit_policy_simulation_beta_page, seeded_policy_simulation_beta_page,
};
use aureline_policy::{
    seeded_policy_simulation_and_expiry_page, PolicySimulationAndExpiryPage,
    PolicySimulationAndExpirySupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_policy_simulation_and_expiry_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("simulation-views") => print_json(&page.simulation_views)?,
        Some("exception-sheets") => print_json(&page.exception_preview_sheets)?,
        Some("approval-history") => print_json(&page.approval_history_rows)?,
        Some("diff-summaries") => print_json(&page.policy_diff_summaries)?,
        Some("expiry-banners") => print_json(&page.expiry_banners)?,
        Some("review-packet") => print_json(&page.review_packet)?,
        Some("summary") => print_json(&page.summary)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = PolicySimulationAndExpirySupportExport::from_page(
                "policy:simulation-expiry:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-raw-private-material-withdrawn") => {
            let mut beta_page = seeded_policy_simulation_beta_page();
            beta_page.exceptions[0].raw_justification_excluded = false;
            beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
            let drill_page = PolicySimulationAndExpiryPage::from_policy_simulation_page(
                "policy:simulation-expiry:drill:raw-private-material",
                "Drill - raw private material exposed",
                "2026-06-01T00:00:00Z",
                beta_page,
            );
            print_json(&drill_page)?;
        }
        Some("drill-indefinite-approval-needs-review") => {
            let mut drill_page = page;
            drill_page.approval_history_rows[0].expires_at.clear();
            drill_page.approval_history_rows[0].bounded_by_expiry = false;
            drill_page.defects =
                aureline_policy::audit_policy_simulation_and_expiry_page(&drill_page);
            drill_page.summary = aureline_policy::PolicySimulationAndExpirySummary::from_page_parts(
                &drill_page.simulation_views,
                &drill_page.exception_preview_sheets,
                &drill_page.approval_history_rows,
                &drill_page.policy_diff_summaries,
                &drill_page.expiry_banners,
                &drill_page.defects,
            );
            print_json(&drill_page)?;
        }
        Some("drill-missing-projection-needs-review") => {
            let mut drill_page = page;
            drill_page.review_packet.surface_projections.pop();
            drill_page.review_packet.surface_projection_tokens.pop();
            drill_page.defects =
                aureline_policy::audit_policy_simulation_and_expiry_page(&drill_page);
            drill_page.summary = aureline_policy::PolicySimulationAndExpirySummary::from_page_parts(
                &drill_page.simulation_views,
                &drill_page.exception_preview_sheets,
                &drill_page.approval_history_rows,
                &drill_page.policy_diff_summaries,
                &drill_page.expiry_banners,
                &drill_page.defects,
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
