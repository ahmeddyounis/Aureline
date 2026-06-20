//! Emits the canonical continuity certification fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- case-backup-drill-stale-narrows
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- case-restore-identity-missing-narrows
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- case-freshness-breached-narrows
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- case-mirror-offline-missing-narrows
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- case-profile-mismatch-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_certification_fixtures -- case-local-core-stays-certified
//! ```

use aureline_continuity::{
    seeded_continuity_certification_input, seeded_continuity_certification_report,
    CertificationDimension, CertificationEvidenceState, CertifiedRow, ContinuityCertificationInput,
    ContinuityCertificationReport, ContinuityCertificationSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_continuity_certification_report();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&report)?,
        Some("summary") => print_json(&report.summary)?,
        Some("support-export") => {
            let export = ContinuityCertificationSupportExport::from_report(
                "continuity:certification:support-export:fixture-001",
                "2026-06-19T00:00:00Z",
                report,
            );
            print_json(&export)?;
        }
        Some("case-backup-drill-stale-narrows") => {
            let mut input = seeded_continuity_certification_input();
            set_state(
                &mut input,
                "continuity-row:managed-cloud-sync",
                CertificationDimension::BackupRestoreFailover,
                CertificationEvidenceState::Stale,
            );
            print_json(&case_report(
                "continuity:certification:case:backup-drill-stale",
                "Case - a managed backup drill is stale and the row narrows to beta",
                input,
            ))?;
        }
        Some("case-restore-identity-missing-narrows") => {
            let mut input = seeded_continuity_certification_input();
            set_state(
                &mut input,
                "continuity-row:self-hosted-restore",
                CertificationDimension::RestoreIdentityPartialLoss,
                CertificationEvidenceState::Missing,
            );
            print_json(&case_report(
                "continuity:certification:case:restore-identity-missing",
                "Case - a self-hosted row lacks restore-identity disclosure and narrows to preview",
                input,
            ))?;
        }
        Some("case-freshness-breached-narrows") => {
            let mut input = seeded_continuity_certification_input();
            set_state(
                &mut input,
                "continuity-row:managed-relay-failover",
                CertificationDimension::DrillFreshnessSlo,
                CertificationEvidenceState::Stale,
            );
            print_json(&case_report(
                "continuity:certification:case:freshness-breached",
                "Case - a managed relay proof packet breached its freshness SLO and the row narrows",
                input,
            ))?;
        }
        Some("case-mirror-offline-missing-narrows") => {
            let mut input = seeded_continuity_certification_input();
            set_state(
                &mut input,
                "continuity-row:sovereign-airgap-snapshot",
                CertificationDimension::MirrorOfflineContinuity,
                CertificationEvidenceState::Missing,
            );
            print_json(&case_report(
                "continuity:certification:case:mirror-offline-missing",
                "Case - a sovereign air-gapped row lacks mirror/offline continuity and narrows to preview",
                input,
            ))?;
        }
        Some("case-profile-mismatch-withdrawn") => {
            let mut input = seeded_continuity_certification_input();
            set_state(
                &mut input,
                "continuity-row:sovereign-airgap-snapshot",
                CertificationDimension::LocalityTenantKey,
                CertificationEvidenceState::ProfileMismatched,
            );
            print_json(&case_report(
                "continuity:certification:case:profile-mismatch",
                "Case - a sovereign row's locality contradicts its profile and the claim is withdrawn",
                input,
            ))?;
        }
        Some("case-local-core-stays-certified") => {
            let mut input = seeded_continuity_certification_input();
            set_state(
                &mut input,
                "continuity-row:managed-cloud-sync",
                CertificationDimension::BackupRestoreFailover,
                CertificationEvidenceState::Missing,
            );
            print_json(&case_report(
                "continuity:certification:case:local-core-stays-certified",
                "Case - a managed row narrows but the local-core lane stays certified",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn set_state(
    input: &mut ContinuityCertificationInput,
    row_id: &str,
    dimension: CertificationDimension,
    state: CertificationEvidenceState,
) {
    let row = with_row(input, row_id);
    let cell = row
        .evidence
        .iter_mut()
        .find(|cell| cell.dimension == dimension)
        .unwrap_or_else(|| panic!("missing dimension {dimension:?} on {row_id}"));
    cell.state = state;
    cell.state_token = state.as_str().to_owned();
    if !state.requires_evidence_ref() {
        cell.evidence_ref.clear();
    }
}

fn with_row<'a>(input: &'a mut ContinuityCertificationInput, row_id: &str) -> &'a mut CertifiedRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded row: {row_id}"))
}

fn case_report(
    report_id: &str,
    report_label: &str,
    input: ContinuityCertificationInput,
) -> ContinuityCertificationReport {
    ContinuityCertificationReport::new(report_id, report_label, "2026-06-19T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
