//! Emits the canonical operator/support continuity summary fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- case-generic-wording-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- case-locality-undisclosed-beta
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- case-evidence-stale-beta
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- case-evidence-missing-preview
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- case-admin-leak-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures -- case-local-core-stays-green
//! ```

use aureline_continuity::{
    seeded_operator_support_continuity_input, seeded_operator_support_continuity_page,
    ContinuityRowSummary, OperatorSupportContinuityInput, OperatorSupportContinuityPage,
    OperatorSupportContinuitySupportExport, SummaryRedaction,
};

// The outage and evidence enums are re-exported from the outage taxonomy module.
use aureline_continuity::m5_control_plane_vs_data_plane_outage::OutageEvidenceStateClass;
use aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix::LocalityClass;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_operator_support_continuity_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = OperatorSupportContinuitySupportExport::from_page(
                "continuity:operator-support:support-export:fixture-001",
                "2026-06-19T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-generic-wording-withdrawn") => {
            let mut input = seeded_operator_support_continuity_input();
            with_summary(
                &mut input,
                "continuity:operator-support:managed-relay",
                |s| {
                    s.outage.status_phrasing = "Service degraded.".to_owned();
                },
            );
            print_json(&case_page(
                "continuity:operator-support:case:generic-wording",
                "Case - a degraded row uses generic wording and is withheld",
                input,
            ))?;
        }
        Some("case-locality-undisclosed-beta") => {
            let mut input = seeded_operator_support_continuity_input();
            with_summary(
                &mut input,
                "continuity:operator-support:managed-cloud-sync",
                |s| {
                    s.posture.storage_locality = LocalityClass::Undisclosed;
                    s.posture.storage_locality_token =
                        LocalityClass::Undisclosed.as_str().to_owned();
                },
            );
            print_json(&case_page(
                "continuity:operator-support:case:locality-undisclosed",
                "Case - a managed row stops disclosing storage locality (beta)",
                input,
            ))?;
        }
        Some("case-evidence-stale-beta") => {
            let mut input = seeded_operator_support_continuity_input();
            with_summary(
                &mut input,
                "continuity:operator-support:self-hosted-restore",
                |s| set_evidence(s, OutageEvidenceStateClass::StaleNeedsRefresh),
            );
            print_json(&case_page(
                "continuity:operator-support:case:evidence-stale",
                "Case - a self-hosted row's backing evidence is stale (beta)",
                input,
            ))?;
        }
        Some("case-evidence-missing-preview") => {
            let mut input = seeded_operator_support_continuity_input();
            with_summary(
                &mut input,
                "continuity:operator-support:sovereign-airgap",
                |s| set_evidence(s, OutageEvidenceStateClass::Missing),
            );
            print_json(&case_page(
                "continuity:operator-support:case:evidence-missing",
                "Case - a sovereign row's backing evidence is missing (preview)",
                input,
            ))?;
        }
        Some("case-admin-leak-withdrawn") => {
            let mut input = seeded_operator_support_continuity_input();
            with_summary(
                &mut input,
                "continuity:operator-support:managed-cloud-sync",
                |s| {
                    s.redaction = SummaryRedaction::new(false, true);
                },
            );
            print_json(&case_page(
                "continuity:operator-support:case:admin-leak",
                "Case - a summary carries admin-only routing and is withheld",
                input,
            ))?;
        }
        Some("case-local-core-stays-green") => {
            let mut input = seeded_operator_support_continuity_input();
            with_summary(
                &mut input,
                "continuity:operator-support:managed-cloud-sync",
                |s| set_evidence(s, OutageEvidenceStateClass::Missing),
            );
            print_json(&case_page(
                "continuity:operator-support:case:local-core-stays-green",
                "Case - a managed row loses evidence but the local-core summary stays stable",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn set_evidence(summary: &mut ContinuityRowSummary, state: OutageEvidenceStateClass) {
    summary.evidence.evidence_state = state;
    summary.evidence.evidence_state_token = state.as_str().to_owned();
    if state == OutageEvidenceStateClass::Missing {
        summary.evidence.last_refreshed_at = String::new();
    }
}

fn with_summary(
    input: &mut OperatorSupportContinuityInput,
    summary_id: &str,
    mutate: impl FnOnce(&mut ContinuityRowSummary),
) {
    let summary = input
        .summaries
        .iter_mut()
        .find(|summary| summary.summary_id == summary_id)
        .unwrap_or_else(|| panic!("missing seeded summary: {summary_id}"));
    mutate(summary);
}

fn case_page(
    page_id: &str,
    page_label: &str,
    input: OperatorSupportContinuityInput,
) -> OperatorSupportContinuityPage {
    OperatorSupportContinuityPage::new(page_id, page_label, "2026-06-19T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
