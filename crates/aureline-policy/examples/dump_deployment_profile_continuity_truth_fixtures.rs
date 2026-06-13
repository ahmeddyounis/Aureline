//! Emits the canonical deployment-profile continuity fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-hidden-self-hosted-dependency-withdrawn
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-mirror-freshness-gap-beta
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-surface-reuse-gap-beta
//! cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-missing-local-safe-fallback-preview
//! ```

use aureline_policy::{
    seeded_deployment_profile_continuity_input, seeded_deployment_profile_continuity_page,
    DeploymentProfileClass, DeploymentProfileContinuityPage,
    DeploymentProfileContinuitySupportExport, FactFamilyClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_deployment_profile_continuity_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = DeploymentProfileContinuitySupportExport::from_page(
                "policy:deployment-profile-continuity:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-hidden-self-hosted-dependency-withdrawn") => {
            let mut input = seeded_deployment_profile_continuity_input();
            input
                .residual_dependency_rows
                .retain(|row| row.dependency_ref != "dependency:self-hosted:model-gateway");
            let drill = DeploymentProfileContinuityPage::new(
                "policy:deployment-profile-continuity:drill:hidden-self-hosted-dependency",
                "Drill - hidden self-hosted vendor dependency (withdrawn)",
                "2026-06-01T00:00:00Z",
                input,
            );
            print_json(&drill)?;
        }
        Some("drill-mirror-freshness-gap-beta") => {
            let mut input = seeded_deployment_profile_continuity_input();
            input
                .mirror_freshness_cards
                .retain(|card| card.profile != DeploymentProfileClass::AirGapped);
            let drill = DeploymentProfileContinuityPage::new(
                "policy:deployment-profile-continuity:drill:mirror-freshness-gap",
                "Drill - missing air-gapped mirror freshness (beta)",
                "2026-06-01T00:00:00Z",
                input,
            );
            print_json(&drill)?;
        }
        Some("drill-surface-reuse-gap-beta") => {
            let mut input = seeded_deployment_profile_continuity_input();
            let row = input
                .surface_reuse_rows
                .iter_mut()
                .find(|row| row.fact_family == FactFamilyClass::ResidualDependency)
                .expect("residual dependency surface reuse row");
            row.surface_visibility.help = false;
            let drill = DeploymentProfileContinuityPage::new(
                "policy:deployment-profile-continuity:drill:surface-reuse-gap",
                "Drill - residual dependency surface reuse gap (beta)",
                "2026-06-01T00:00:00Z",
                input,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-local-safe-fallback-preview") => {
            let mut input = seeded_deployment_profile_continuity_input();
            input
                .local_safe_fallback_cards
                .retain(|card| card.profile != DeploymentProfileClass::AirGapped);
            let drill = DeploymentProfileContinuityPage::new(
                "policy:deployment-profile-continuity:drill:missing-local-safe-fallback",
                "Drill - missing air-gapped local-safe fallback (preview)",
                "2026-06-01T00:00:00Z",
                input,
            );
            print_json(&drill)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
