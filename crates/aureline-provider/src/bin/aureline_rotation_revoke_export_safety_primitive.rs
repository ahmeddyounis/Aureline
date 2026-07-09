//! Headless emitter for the M5 rotation/revoke / export-safety controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-rotation-revoke-export-safety-proof/`, its matrix CSV, the
//! Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-rotation-revoke-export-safety-controls/`. Recovery, audit, support,
//! export, and CLI surfaces read these controls so one rotation/revoke-event row names its
//! credential class, its prior / new lifecycle state, its impacted running sessions /
//! queued jobs / remembered decisions, its recovery next step, and its audit event with a
//! derived continuity class that never lets a revoked or expired credential read as still
//! usable, and one export-safety banner states raw credentials are excluded by default from
//! profiles, support bundles, handoff packets, recipes, and portable workspace exports with
//! a derived redaction posture that never leaves credential exclusion to implication.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- fixture-revoke-event-impacted-workflows
//! cargo run -q -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- fixture-export-banner-raw-excluded
//! cargo run -q -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- validate
//! ```

use aureline_provider::implement_rotation_revoke_event_rows_and_export_safety_banners_with_impacted_workflow_remembered_decision_and_raw_secret_excluded_continuity_truth::{
    seeded_rotation_revoke_export_safety_controls,
    seeded_rotation_revoke_export_safety_controls_export_banner_raw_excluded,
    seeded_rotation_revoke_export_safety_controls_revoke_event_impacted_workflows,
    RotationRevokeExportSafetyControlsPacket,
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
            let packet = seeded_rotation_revoke_export_safety_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_rotation_revoke_export_safety_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_rotation_revoke_export_safety_controls().render_matrix_csv()
            );
        }
        Some("fixture-revoke-event-impacted-workflows") => {
            let packet =
                seeded_rotation_revoke_export_safety_controls_revoke_event_impacted_workflows();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-export-banner-raw-excluded") => {
            let packet = seeded_rotation_revoke_export_safety_controls_export_banner_raw_excluded();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_rotation_revoke_export_safety_controls(),
                seeded_rotation_revoke_export_safety_controls_revoke_event_impacted_workflows(),
                seeded_rotation_revoke_export_safety_controls_export_banner_raw_excluded(),
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
    packet: &RotationRevokeExportSafetyControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "rotation revoke export safety controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
