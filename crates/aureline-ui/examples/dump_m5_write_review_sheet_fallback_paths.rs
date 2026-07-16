//! Headless emitter for the M5 write-review-sheet fallback-path packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-write-review-sheet-fallback-paths/`, its matrix CSV, the Markdown summary, and the
//! narrowed fixtures under `fixtures/editor/m5-write-review-sheet-fallback-paths/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_write_review_sheet_fallback_paths -- support-export
//! cargo run -p aureline-ui --example dump_m5_write_review_sheet_fallback_paths -- report
//! cargo run -p aureline-ui --example dump_m5_write_review_sheet_fallback_paths -- csv
//! cargo run -p aureline-ui --example dump_m5_write_review_sheet_fallback_paths -- fixture-precondition-notice-narrowed
//! cargo run -p aureline-ui --example dump_m5_write_review_sheet_fallback_paths -- fixture-export-redacted-narrowed
//! cargo run -p aureline-ui --example dump_m5_write_review_sheet_fallback_paths -- validate
//! ```

use aureline_ui::m5_write_review_sheet_fallback_paths::{
    seeded_m5_write_review_sheet_fallback_paths,
    seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed,
    seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed,
    M5WriteReviewSheetFallbackPathsPacket,
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
            let packet = seeded_m5_write_review_sheet_fallback_paths();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_write_review_sheet_fallback_paths().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_write_review_sheet_fallback_paths().render_matrix_csv()
            );
        }
        Some("fixture-precondition-notice-narrowed") => {
            let packet = seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-export-redacted-narrowed") => {
            let packet = seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_write_review_sheet_fallback_paths(),
                seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed(),
                seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed(),
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
    packet: &M5WriteReviewSheetFallbackPathsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "write-review-sheet fallback-path packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
