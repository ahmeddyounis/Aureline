//! Headless emitter for the M5 line-historical_snapshot_descriptor and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-historical-snapshot-descriptor-and-change-diff-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-historical-snapshot-descriptor-and-change-diff-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- report
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- historical-snapshot-descriptor-table
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- fixture-historical-snapshot-descriptor-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- fixture-descriptor-change-diff-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- validate
//! ```

use aureline_ui::m5_historical_snapshot_descriptor_and_change_diff_registries::{
    seeded_m5_historical_snapshot_descriptor_and_change_diff_registries,
    seeded_m5_historical_snapshot_descriptor_and_change_diff_registries_descriptor_change_diff_preview_narrowed,
    seeded_m5_historical_snapshot_descriptor_and_change_diff_registries_historical_snapshot_descriptor_beta_narrowed,
    M5HistoricalSnapshotDescriptorChangeDiffRegistriesPacket,
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
            let packet = seeded_m5_historical_snapshot_descriptor_and_change_diff_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries()
                    .render_matrix_csv()
            );
        }
        Some("historical-snapshot-descriptor-table") => {
            print!(
                "{}",
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries()
                    .render_historical_snapshot_descriptor_table()
            );
        }
        Some("fixture-historical-snapshot-descriptor-beta-narrowed") => {
            let packet =
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries_historical_snapshot_descriptor_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-descriptor-change-diff-preview-narrowed") => {
            let packet =
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries_descriptor_change_diff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries(),
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries_historical_snapshot_descriptor_beta_narrowed(),
                seeded_m5_historical_snapshot_descriptor_and_change_diff_registries_descriptor_change_diff_preview_narrowed(),
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
    packet: &M5HistoricalSnapshotDescriptorChangeDiffRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
