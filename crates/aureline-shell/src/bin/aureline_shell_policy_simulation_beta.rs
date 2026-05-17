//! Headless inspector for the policy simulation, exception, and memory beta page.
//!
//! The bin emits the same audited records consumed by admin, support export,
//! diagnostics, and shell rendering summaries.

use aureline_shell::policy_simulation_beta::{
    audit_policy_simulation_beta_page, seeded_policy_simulation_beta_page,
    validate_policy_simulation_beta_page, PolicySimulationBetaPage, PolicySimulationRenderSummary,
    PolicySimulationSummary, PolicySimulationSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_policy_simulation_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("simulations") => print_json(&page.simulations)?,
        Some("affected-surfaces") => {
            let surfaces: Vec<_> = page
                .simulations
                .iter()
                .flat_map(|simulation| simulation.affected_surfaces.iter())
                .collect();
            print_json(&surfaces)?;
        }
        Some("exceptions") => print_json(&page.exceptions)?,
        Some("remembered-decisions") => print_json(&page.remembered_decisions)?,
        Some("action-time-policy") => print_json(&page.action_time_policy_states)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => {
            let summary = PolicySimulationRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("support-export") => {
            let export = PolicySimulationSupportExport::from_page(
                "support-export:policy-simulation:001",
                "2026-05-17T19:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-expiry-omitted") => {
            let mut page = page;
            page.exceptions[0].time_horizon.expires_at.clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-memory-drift-unexplained") => {
            let mut page = page;
            page.remembered_decisions[0].invalidation_reasons.clear();
            page.remembered_decisions[0]
                .invalidation_reason_tokens
                .clear();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-current-only-support-export") => {
            let mut page = page;
            page.action_time_policy_states[0].preserves_historical_truth = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_policy_simulation_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} subject_id={} field={} note={}",
                        defect.defect_kind_token, defect.subject_id, defect.field, defect.note,
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

fn rebuild_with_defects(mut page: PolicySimulationBetaPage) -> PolicySimulationBetaPage {
    page.defects = audit_policy_simulation_beta_page(&page);
    page.summary = PolicySimulationSummary::from_records(
        &page.simulations,
        &page.exceptions,
        &page.remembered_decisions,
        &page.action_time_policy_states,
        &page.defects,
    );
    page
}
