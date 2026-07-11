//! Headless emitter for the M5 repair-transaction-preview-card / rollback-class-strip controls
//! packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/`, its
//! matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_repair_preview_rollback_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_repair_preview_rollback_controls -- report
//! cargo run -p aureline-shell --example dump_m5_repair_preview_rollback_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_repair_preview_rollback_controls -- fixture-doctor-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_repair_preview_rollback_controls -- fixture-safe-mode-ui-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_repair_preview_rollback_controls -- validate
//! ```

use aureline_shell::implement_the_m5_repair_transaction_preview_card_and_rollback_class_strip_finding_ids_prerequisites_checkpoint_state_reversal_class_and_local_remote_managed_impact_primitive::{
    seeded_m5_repair_preview_rollback_controls,
    seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed,
    seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed,
    M5RepairPreviewRollbackControlsPacket,
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
            let packet = seeded_m5_repair_preview_rollback_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_repair_preview_rollback_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_repair_preview_rollback_controls().render_matrix_csv()
            );
        }
        Some("fixture-doctor-ui-beta-narrowed") => {
            let packet = seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-safe-mode-ui-preview-narrowed") => {
            let packet = seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_repair_preview_rollback_controls(),
                seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed(),
                seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed(),
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
    packet: &M5RepairPreviewRollbackControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
