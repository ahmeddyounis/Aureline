//! Emits the canonical continuity-proof freshness SLO dashboard fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- case-managed-backup-breached-hold
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- case-relay-packet-missing-hold
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- case-owner-signoff-missing-beta
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- case-no-rerun-path-beta
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_freshness_slo_fixtures -- case-local-core-stays-green
//! ```

use aureline_continuity::{
    seeded_continuity_freshness_slo_dashboard, seeded_continuity_freshness_slo_input,
    ContinuityFreshnessRow, ContinuityFreshnessSloDashboard, ContinuityFreshnessSloInput,
    ContinuityFreshnessSloState, ContinuityFreshnessSloSupportExport, RerunAutomationClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dashboard = seeded_continuity_freshness_slo_dashboard();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&dashboard)?,
        Some("summary") => print_json(&dashboard.summary)?,
        Some("support-export") => {
            let export = ContinuityFreshnessSloSupportExport::from_dashboard(
                "continuity:freshness-slo:support-export:fixture-001",
                "2026-06-19T00:00:00Z",
                dashboard,
            );
            print_json(&export)?;
        }
        Some("case-managed-backup-breached-hold") => {
            let mut input = seeded_continuity_freshness_slo_input();
            with_row(&mut input, "continuity-row:managed-cloud-sync", |row| {
                set_state(row, ContinuityFreshnessSloState::Breached);
                row.proof_packet.captured_at = Some("2026-01-01".to_owned());
            });
            print_json(&case_dashboard(
                "continuity:freshness-slo:case:managed-backup-breached",
                "Case - a managed backup packet breached its freshness SLO (hold)",
                input,
            ))?;
        }
        Some("case-relay-packet-missing-hold") => {
            let mut input = seeded_continuity_freshness_slo_input();
            with_row(&mut input, "continuity-row:managed-relay-failover", |row| {
                set_state(row, ContinuityFreshnessSloState::Missing);
                row.proof_packet.captured_at = None;
                row.proof_packet.evidence_refs = Vec::new();
            });
            print_json(&case_dashboard(
                "continuity:freshness-slo:case:relay-packet-missing",
                "Case - a managed relay row has no captured continuity packet (hold)",
                input,
            ))?;
        }
        Some("case-owner-signoff-missing-beta") => {
            let mut input = seeded_continuity_freshness_slo_input();
            with_row(&mut input, "continuity-row:self-hosted-restore", |row| {
                row.owner_signoff_present = false;
            });
            print_json(&case_dashboard(
                "continuity:freshness-slo:case:owner-signoff-missing",
                "Case - a self-hosted restore row lacks a current drill-owner sign-off (beta)",
                input,
            ))?;
        }
        Some("case-no-rerun-path-beta") => {
            let mut input = seeded_continuity_freshness_slo_input();
            with_row(
                &mut input,
                "continuity-row:sovereign-airgap-snapshot",
                |row| {
                    row.rerun.rerun_class = RerunAutomationClass::NoRerunPath;
                    row.rerun.rerun_class_token =
                        RerunAutomationClass::NoRerunPath.as_str().to_owned();
                },
            );
            print_json(&case_dashboard(
                "continuity:freshness-slo:case:no-rerun-path",
                "Case - a sovereign snapshot row has no rerun path to refresh evidence (beta)",
                input,
            ))?;
        }
        Some("case-local-core-stays-green") => {
            let mut input = seeded_continuity_freshness_slo_input();
            with_row(&mut input, "continuity-row:managed-cloud-sync", |row| {
                set_state(row, ContinuityFreshnessSloState::Breached);
                row.proof_packet.captured_at = Some("2026-01-01".to_owned());
            });
            print_json(&case_dashboard(
                "continuity:freshness-slo:case:local-core-stays-green",
                "Case - a managed row goes stale but the local-core lane stays green (hold)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn set_state(row: &mut ContinuityFreshnessRow, state: ContinuityFreshnessSloState) {
    row.proof_packet.slo_state = state;
    row.proof_packet.slo_state_token = state.as_str().to_owned();
}

fn with_row(
    input: &mut ContinuityFreshnessSloInput,
    row_id: &str,
    mutate: impl FnOnce(&mut ContinuityFreshnessRow),
) {
    let row = input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded row: {row_id}"));
    mutate(row);
}

fn case_dashboard(
    dashboard_id: &str,
    dashboard_label: &str,
    input: ContinuityFreshnessSloInput,
) -> ContinuityFreshnessSloDashboard {
    ContinuityFreshnessSloDashboard::new(
        dashboard_id,
        dashboard_label,
        "2026-06-19T00:00:00Z",
        input,
    )
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
