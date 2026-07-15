//! Headless emitter for the M5 line-deferral_backlog and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-stable-line-deferral-backlog-and-correction-conversion-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-stable-line-deferral-backlog-and-correction-conversion-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- report
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- deferral-backlog-table
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- fixture-deferral-backlog-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- fixture-correction-conversion-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_deferral_backlog_and_correction_conversion_registries -- validate
//! ```

use aureline_ui::m5_stable_line_deferral_backlog_and_correction_conversion_registries::{
    seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries,
    seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries_correction_conversion_preview_narrowed,
    seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries_deferral_backlog_beta_narrowed,
    M5StableLineDeferralBacklogCorrectionConversionRegistriesPacket,
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
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries()
                    .render_matrix_csv()
            );
        }
        Some("deferral-backlog-table") => {
            print!(
                "{}",
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries()
                    .render_deferral_backlog_table()
            );
        }
        Some("fixture-deferral-backlog-beta-narrowed") => {
            let packet =
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries_deferral_backlog_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-correction-conversion-preview-narrowed") => {
            let packet =
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries_correction_conversion_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries(),
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries_deferral_backlog_beta_narrowed(),
                seeded_m5_stable_line_deferral_backlog_and_correction_conversion_registries_correction_conversion_preview_narrowed(),
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
    packet: &M5StableLineDeferralBacklogCorrectionConversionRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
