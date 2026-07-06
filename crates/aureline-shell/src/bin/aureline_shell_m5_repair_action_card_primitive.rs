//! Headless emitter for the M5 repair-action-card / repair-preview-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-repair-action-card-proof/`, its matrix CSV, the Markdown report
//! `artifacts/components/m5-repair-action-card-primitive.md`, and the narrowed fixtures
//! under `fixtures/ui/m5-repair-action-card-primitive/`. Every M5 recovery surface (the
//! Project Doctor panel, the Doctor repair card, the guided repair wizard, the
//! support-bundle repair row, the environment repair prompt, the toolchain repair card,
//! the remote-host repair card, the repair preview sheet, and the activity-center repair
//! entry) reads this primitive so what a repair changes, what it leaves untouched, where
//! it runs, and how reversible it is stay consistent, and so the support export
//! reconstructs the same repair explanation from one shared model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_repair_action_card_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_repair_action_card_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_repair_action_card_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_repair_action_card_primitive -- fixture-remote-host-repair-card-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_repair_action_card_primitive -- fixture-repair-preview-sheet-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_repair_action_card_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_repair_action_card_and_repair_preview_row_impact_scope_target_boundary_and_reversal_class_primitive::{
    seeded_m5_repair_action_card_primitive_packet,
    seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed,
    seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed,
    M5RepairActionCardPrimitivePacket,
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
            let packet = seeded_m5_repair_action_card_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_repair_action_card_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_repair_action_card_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-remote-host-repair-card-beta-narrowed") => {
            let packet =
                seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-repair-preview-sheet-preview-narrowed") => {
            let packet =
                seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_repair_action_card_primitive_packet(),
                seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed(),
                seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed(),
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
    packet: &M5RepairActionCardPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
