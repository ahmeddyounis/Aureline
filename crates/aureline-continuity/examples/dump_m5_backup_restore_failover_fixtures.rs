//! Emits the canonical backup/restore/failover continuity fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- registry
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-generic-dr-text-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-not-exercised-disclosure-missing-beta
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-scope-not-exercised-preview
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-restore-identity-undeclared-beta
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-drill-never-run-preview
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-drill-evidence-stale-beta
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-sovereign-hidden-vendor-failover-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_backup_restore_failover_fixtures -- case-packet-evidence-missing-preview
//! ```

use aureline_continuity::{
    seeded_backup_restore_failover_input, seeded_backup_restore_failover_page,
    BackupRestoreFailoverInput, BackupRestoreFailoverPacketEntry, BackupRestoreFailoverPage,
    BackupRestoreFailoverSupportExport, DrillEvidenceStateClass, RestoreFailoverHostingClass,
    RestoreIdentityClass, RestoreScope, ScopeExercisedClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_backup_restore_failover_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("registry") => print_json(&page.registry)?,
        Some("support-export") => {
            let export = BackupRestoreFailoverSupportExport::from_page(
                "continuity:backup-restore-failover:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-generic-dr-text-withdrawn") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(
                &mut input,
                "continuity-brf:managed-workspace-backup",
                |packet| {
                    packet.generic_dr_text_only = true;
                },
            );
            print_json(&case_page(
                "continuity:backup-restore-failover:case:generic-dr-text",
                "Case - a backup packet relies on generic 'DR tested' text (withdrawn)",
                input,
            ))?;
        }
        Some("case-not-exercised-disclosure-missing-beta") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(&mut input, "continuity-brf:self-hosted-restore", |packet| {
                packet.restore_scope.not_exercised_note = String::new();
            });
            print_json(&case_page(
                "continuity:backup-restore-failover:case:not-exercised-disclosure-missing",
                "Case - a partial drill omits what restored narrower than normal (beta)",
                input,
            ))?;
        }
        Some("case-scope-not-exercised-preview") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(
                &mut input,
                "continuity-brf:managed-relay-failover",
                |packet| {
                    packet.restore_scope =
                        RestoreScope::new(ScopeExercisedClass::NotExercised, Vec::new(), "");
                },
            );
            print_json(&case_page(
                "continuity:backup-restore-failover:case:scope-not-exercised",
                "Case - a managed failover packet exercised nothing (preview)",
                input,
            ))?;
        }
        Some("case-restore-identity-undeclared-beta") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(
                &mut input,
                "continuity-brf:managed-workspace-backup",
                |packet| {
                    packet.restore_identity = RestoreIdentityClass::NotApplicable;
                    packet.restore_identity_token =
                        RestoreIdentityClass::NotApplicable.as_str().to_owned();
                },
            );
            print_json(&case_page(
                "continuity:backup-restore-failover:case:restore-identity-undeclared",
                "Case - a managed backup packet declares no restore identity (beta)",
                input,
            ))?;
        }
        Some("case-drill-never-run-preview") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(
                &mut input,
                "continuity-brf:managed-workspace-backup",
                |packet| {
                    packet.drill.evidence_state = DrillEvidenceStateClass::NeverRun;
                    packet.drill.evidence_state_token =
                        DrillEvidenceStateClass::NeverRun.as_str().to_owned();
                    packet.drill.last_drill_at = String::new();
                },
            );
            print_json(&case_page(
                "continuity:backup-restore-failover:case:drill-never-run",
                "Case - a managed backup drill has never been run (preview)",
                input,
            ))?;
        }
        Some("case-drill-evidence-stale-beta") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(
                &mut input,
                "continuity-brf:sovereign-snapshot-replication",
                |packet| {
                    packet.drill.evidence_state = DrillEvidenceStateClass::StaleNeedsDrill;
                    packet.drill.evidence_state_token =
                        DrillEvidenceStateClass::StaleNeedsDrill.as_str().to_owned();
                },
            );
            print_json(&case_page(
                "continuity:backup-restore-failover:case:drill-evidence-stale",
                "Case - a sovereign snapshot drill has aged out under its freshness SLO (beta)",
                input,
            ))?;
        }
        Some("case-sovereign-hidden-vendor-failover-withdrawn") => {
            let mut input = seeded_backup_restore_failover_input();
            with_packet(
                &mut input,
                "continuity-brf:sovereign-snapshot-replication",
                |packet| {
                    packet.restore_failover_hosting = RestoreFailoverHostingClass::VendorOperated;
                    packet.restore_failover_hosting_token =
                        RestoreFailoverHostingClass::VendorOperated
                            .as_str()
                            .to_owned();
                    packet.external_dependency_disclosed = false;
                },
            );
            print_json(&case_page(
                "continuity:backup-restore-failover:case:sovereign-hidden-vendor-failover",
                "Case - a sovereign packet hides a vendor-operated failover lane (withdrawn)",
                input,
            ))?;
        }
        Some("case-packet-evidence-missing-preview") => {
            let mut input = seeded_backup_restore_failover_input();
            input
                .packets
                .retain(|packet| packet.packet_id != "continuity-brf:managed-relay-failover");
            print_json(&case_page(
                "continuity:backup-restore-failover:case:packet-evidence-missing",
                "Case - a claimed resilience row carries no continuity packet (preview)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_packet(
    input: &mut BackupRestoreFailoverInput,
    packet_id: &str,
    mutate: impl FnOnce(&mut BackupRestoreFailoverPacketEntry),
) {
    let packet = input
        .packets
        .iter_mut()
        .find(|packet| packet.packet_id == packet_id)
        .unwrap_or_else(|| panic!("missing seeded packet: {packet_id}"));
    mutate(packet);
}

fn case_page(
    page_id: &str,
    page_label: &str,
    input: BackupRestoreFailoverInput,
) -> BackupRestoreFailoverPage {
    BackupRestoreFailoverPage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
