//! Headless emitter for the M5 incident-snapshot-card / desktop-handoff-sheet controls.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-incident-snapshot-card-desktop-handoff-sheet-proof/`, its matrix CSV,
//! the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls/`. The
//! incident-awareness and desktop-handoff UIs read this packet so the first glance at an
//! incident names the service, run, severity, latest status, and freshness, and every handoff
//! names the exact object that opens on desktop before a tap.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- support-export
//! cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- report
//! cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- csv
//! cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- fixture-incident-snapshot-card-stale
//! cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- fixture-desktop-handoff-sheet-not-openable
//! cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- validate
//! ```

use aureline_companion::implement_incident_snapshot_cards_and_desktop_handoff_sheets_with_service_run_identity_severity_status_target_identity_auth_tenant_reminder_and_open_on_desktop_truth::{
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls,
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls_desktop_handoff_sheet_not_openable,
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls_incident_snapshot_card_stale,
    IncidentSnapshotCardDesktopHandoffSheetControlsPacket,
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
        Some("support-export") | None => {
            let packet = seeded_incident_snapshot_card_desktop_handoff_sheet_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls().render_matrix_csv()
            );
        }
        Some("fixture-incident-snapshot-card-stale") => {
            let packet =
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls_incident_snapshot_card_stale();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-desktop-handoff-sheet-not-openable") => {
            let packet =
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls_desktop_handoff_sheet_not_openable();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls(),
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls_incident_snapshot_card_stale(),
                seeded_incident_snapshot_card_desktop_handoff_sheet_controls_desktop_handoff_sheet_not_openable(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &IncidentSnapshotCardDesktopHandoffSheetControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls failed validation: {}", tokens.join(",")).into())
    }
}
