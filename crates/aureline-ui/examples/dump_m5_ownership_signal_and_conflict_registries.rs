//! Headless emitter for the M5 line-ownership_signal_row and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ownership-signal-and-conflict-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-ownership-signal-and-conflict-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- ownership-signal-row-table
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- fixture-ownership-signal-row-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- fixture-owner-conflict-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ownership_signal_and_conflict_registries -- validate
//! ```

use aureline_ui::m5_ownership_signal_and_conflict_registries::{
    seeded_m5_ownership_signal_and_conflict_registries,
    seeded_m5_ownership_signal_and_conflict_registries_owner_conflict_preview_narrowed,
    seeded_m5_ownership_signal_and_conflict_registries_ownership_signal_row_beta_narrowed,
    M5OwnershipSignalAndConflictRegistriesPacket,
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
            let packet = seeded_m5_ownership_signal_and_conflict_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ownership_signal_and_conflict_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ownership_signal_and_conflict_registries().render_matrix_csv()
            );
        }
        Some("ownership-signal-row-table") => {
            print!(
                "{}",
                seeded_m5_ownership_signal_and_conflict_registries()
                    .render_ownership_signal_row_table()
            );
        }
        Some("fixture-ownership-signal-row-beta-narrowed") => {
            let packet =
                seeded_m5_ownership_signal_and_conflict_registries_ownership_signal_row_beta_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-owner-conflict-preview-narrowed") => {
            let packet =
                seeded_m5_ownership_signal_and_conflict_registries_owner_conflict_preview_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ownership_signal_and_conflict_registries(),
                seeded_m5_ownership_signal_and_conflict_registries_ownership_signal_row_beta_narrowed(),
                seeded_m5_ownership_signal_and_conflict_registries_owner_conflict_preview_narrowed(),
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
    packet: &M5OwnershipSignalAndConflictRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
