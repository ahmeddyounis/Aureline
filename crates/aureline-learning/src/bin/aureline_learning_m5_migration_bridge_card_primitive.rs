//! Headless emitter for the M5 migration-bridge-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-migration-bridge-card-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-migration-bridge-card-primitive/`. The migration report panel, import diff
//! row, first-run switch summary, keybinding migration notice, and support migration export
//! consumers read this matrix so one migration bridge card names the old path or shortcut, the
//! new command or surface, the exact migration mapping honesty class (exact / native / bridge /
//! shimmed / partial / unsupported), the affected scope, any unsupported edge cases, and
//! whether the import can be reviewed or undone — never letting an approximated or partial
//! behavior masquerade as exact parity, and never leaving a durable import change without an
//! available undo / review action.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- fixture-keybinding-migration-notice-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- fixture-support-migration-export-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- validate
//! ```

use aureline_learning::ship_migration_bridge_cards_with_old_path_new_command_mapping_native_bridge_shimmed_partial_states_and_undo_import_parity_across_claimed_m5_importer_and_migration_surfaces::{
    seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed,
    seeded_m5_migration_bridge_card_packet,
    seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed,
    M5MigrationBridgeCardPacket,
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
            let packet = seeded_m5_migration_bridge_card_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_migration_bridge_card_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_migration_bridge_card_packet().render_matrix_csv()
            );
        }
        Some("fixture-keybinding-migration-notice-beta-narrowed") => {
            let packet =
                seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-migration-export-preview-narrowed") => {
            let packet =
                seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_migration_bridge_card_packet(),
                seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed(),
                seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed(),
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

fn assert_valid(packet: &M5MigrationBridgeCardPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "migration bridge card primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
