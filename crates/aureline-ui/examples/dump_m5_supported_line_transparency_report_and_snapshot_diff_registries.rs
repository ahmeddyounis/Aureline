//! Headless emitter for the M5 line-transparency_report and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-supported-line-transparency-report-and-snapshot-diff-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-supported-line-transparency-report-and-snapshot-diff-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- report
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- transparency-report-table
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- fixture-transparency-report-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- fixture-snapshot-diff-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- validate
//! ```

use aureline_ui::m5_supported_line_transparency_report_and_snapshot_diff_registries::{
    seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries,
    seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_snapshot_diff_preview_narrowed,
    seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_transparency_report_beta_narrowed,
    M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket,
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
            let packet =
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries()
                    .render_matrix_csv()
            );
        }
        Some("transparency-report-table") => {
            print!(
                "{}",
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries()
                    .render_transparency_report_table()
            );
        }
        Some("fixture-transparency-report-beta-narrowed") => {
            let packet =
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_transparency_report_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-snapshot-diff-preview-narrowed") => {
            let packet =
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_snapshot_diff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries(),
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_transparency_report_beta_narrowed(),
                seeded_m5_supported_line_transparency_report_and_snapshot_diff_registries_snapshot_diff_preview_narrowed(),
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
    packet: &M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
