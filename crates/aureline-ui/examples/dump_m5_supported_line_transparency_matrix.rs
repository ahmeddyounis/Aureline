//! Headless emitter for the frozen M5 supported-line transparency matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-supported-line-transparency/`, its matrix CSV, the Markdown design report at
//! `artifacts/program/m5-supported-line-transparency-matrix.md`, the public-proof dashboard at
//! `dashboards/m5-supported-line-public-proof.json`, and the narrowed fixtures under
//! `fixtures/release/m5-supported-line-transparency/`. The release, help, docs, support, public-proof, and
//! partner/procurement surfaces read this matrix so no supported line stays green on stale external proof,
//! migration pain stays scored and versioned, ORR and correction history stays retained and archived, transparency
//! reports stay export-safe with no internal-only leakage, and support language never outruns current public
//! proof.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- fixture-orr-history-event-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- fixture-correction-train-archive-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_supported_line_transparency_matrix -- validate
//! ```

use aureline_ui::m5_supported_line_transparency_matrix::{
    seeded_m5_supported_line_transparency_matrix,
    seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed,
    seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed,
    M5SupportedLineTransparencyMatrixPacket,
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
            let packet = seeded_m5_supported_line_transparency_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_supported_line_transparency_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_supported_line_transparency_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_supported_line_transparency_matrix().render_dashboard_json()
            );
        }
        Some("fixture-orr-history-event-beta-narrowed") => {
            let packet =
                seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-correction-train-archive-preview-narrowed") => {
            let packet = seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_supported_line_transparency_matrix(),
                seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed(),
                seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed(),
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
    packet: &M5SupportedLineTransparencyMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
