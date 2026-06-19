//! Emits the canonical locality, tenant/key-mode, and continuity-drill matrix fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- drill-managed-restore-drill-stale-beta
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- drill-drill-never-run-preview
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- drill-sovereign-hidden-vendor-failover-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- drill-locality-undisclosed-beta
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- drill-local-only-overclaimed-preview
//! cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- drill-partial-loss-undisclosed-beta
//! ```

use aureline_continuity::{
    seeded_continuity_claim_matrix_input, seeded_continuity_claim_matrix_page,
    ContinuityClaimMatrixInput, ContinuityClaimMatrixPage, ContinuityClaimMatrixSupportExport,
    ContinuityPacketFamilyClass, DrillEvidenceStateClass, LocalityClass, PartialLossClass,
    RestoreFailoverHostingClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_continuity_claim_matrix_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = ContinuityClaimMatrixSupportExport::from_page(
                "continuity:claim-matrix:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-managed-restore-drill-stale-beta") => {
            let mut input = seeded_continuity_claim_matrix_input();
            with_row(&mut input, "continuity-row:managed-cloud-sync", |row| {
                row.drill.evidence_state = DrillEvidenceStateClass::StaleNeedsDrill;
                row.drill.evidence_state_token =
                    DrillEvidenceStateClass::StaleNeedsDrill.as_str().to_owned();
            });
            print_json(&drill_page(
                "continuity:claim-matrix:drill:managed-restore-drill-stale",
                "Drill - stale managed backup drill evidence (beta)",
                input,
            ))?;
        }
        Some("drill-drill-never-run-preview") => {
            let mut input = seeded_continuity_claim_matrix_input();
            with_row(&mut input, "continuity-row:managed-cloud-sync", |row| {
                row.drill.evidence_state = DrillEvidenceStateClass::NeverRun;
                row.drill.evidence_state_token =
                    DrillEvidenceStateClass::NeverRun.as_str().to_owned();
                row.drill.last_drill_at = String::new();
            });
            print_json(&drill_page(
                "continuity:claim-matrix:drill:drill-never-run",
                "Drill - managed continuity drill never run (preview)",
                input,
            ))?;
        }
        Some("drill-sovereign-hidden-vendor-failover-withdrawn") => {
            let mut input = seeded_continuity_claim_matrix_input();
            with_row(
                &mut input,
                "continuity-row:sovereign-airgap-snapshot",
                |row| {
                    row.restore_failover_hosting = RestoreFailoverHostingClass::VendorOperated;
                    row.restore_failover_hosting_token =
                        RestoreFailoverHostingClass::VendorOperated
                            .as_str()
                            .to_owned();
                    row.external_dependency_disclosed = false;
                },
            );
            print_json(&drill_page(
                "continuity:claim-matrix:drill:sovereign-hidden-vendor-failover",
                "Drill - sovereign row hides a vendor-operated failover lane (withdrawn)",
                input,
            ))?;
        }
        Some("drill-locality-undisclosed-beta") => {
            let mut input = seeded_continuity_claim_matrix_input();
            with_row(&mut input, "continuity-row:managed-relay-failover", |row| {
                row.locality.processing_locality = LocalityClass::Undisclosed;
                row.locality.processing_locality_token =
                    LocalityClass::Undisclosed.as_str().to_owned();
            });
            print_json(&drill_page(
                "continuity:claim-matrix:drill:locality-undisclosed",
                "Drill - managed relay row hides processing locality (beta)",
                input,
            ))?;
        }
        Some("drill-local-only-overclaimed-preview") => {
            let mut input = seeded_continuity_claim_matrix_input();
            with_row(&mut input, "continuity-row:local-desktop-core", |row| {
                row.continuity_packet_family = ContinuityPacketFamilyClass::Backup;
                row.continuity_packet_family_token =
                    ContinuityPacketFamilyClass::Backup.as_str().to_owned();
            });
            print_json(&drill_page(
                "continuity:claim-matrix:drill:local-only-overclaimed",
                "Drill - local-only row claims a managed backup family (preview)",
                input,
            ))?;
        }
        Some("drill-partial-loss-undisclosed-beta") => {
            let mut input = seeded_continuity_claim_matrix_input();
            with_row(&mut input, "continuity-row:self-hosted-restore", |row| {
                row.partial_loss = PartialLossClass::Undisclosed;
                row.partial_loss_token = PartialLossClass::Undisclosed.as_str().to_owned();
            });
            print_json(&drill_page(
                "continuity:claim-matrix:drill:partial-loss-undisclosed",
                "Drill - self-hosted restore row hides partial-loss behavior (beta)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_row(
    input: &mut ContinuityClaimMatrixInput,
    row_id: &str,
    mutate: impl FnOnce(&mut aureline_continuity::ContinuityClaimRow),
) {
    let row = input
        .claim_rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded row: {row_id}"));
    mutate(row);
}

fn drill_page(
    page_id: &str,
    page_label: &str,
    input: ContinuityClaimMatrixInput,
) -> ContinuityClaimMatrixPage {
    ContinuityClaimMatrixPage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
