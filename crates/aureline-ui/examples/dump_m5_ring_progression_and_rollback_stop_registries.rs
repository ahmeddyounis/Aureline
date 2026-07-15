//! Headless emitter for the M5 cohort-descriptor and cohort-evidence-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-ring-progression-and-rollback-stop-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-ring-progression-and-rollback-stop-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- ring-progression-table
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- fixture-ring-progression-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- fixture-rollback-stop-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- validate
//! ```

use aureline_ui::m5_ring_progression_and_rollback_stop_registries::{
    seeded_m5_ring_progression_and_rollback_stop_registries,
    seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed,
    seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed,
    M5RingProgressionRollbackStopRegistriesPacket,
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
            let packet = seeded_m5_ring_progression_and_rollback_stop_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ring_progression_and_rollback_stop_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ring_progression_and_rollback_stop_registries().render_matrix_csv()
            );
        }
        Some("ring-progression-table") => {
            print!(
                "{}",
                seeded_m5_ring_progression_and_rollback_stop_registries()
                    .render_ring_progression_table()
            );
        }
        Some("fixture-ring-progression-beta-narrowed") => {
            let packet =
                seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-rollback-stop-preview-narrowed") => {
            let packet =
                seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ring_progression_and_rollback_stop_registries(),
                seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed(),
                seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed(),
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
    packet: &M5RingProgressionRollbackStopRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
