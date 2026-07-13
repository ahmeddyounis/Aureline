//! Headless emitter for the frozen M5 visual-foundation matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-visual-foundations-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/ui/m5-visual-foundations/`. The shell, editor, review, data, and
//! docs surfaces read this matrix so brand / interactive / neutral / status palettes stay distinct and
//! never rely on hue alone, syntax / diff / chart palettes never collide with diagnostics, typography and
//! font stacks stay stable, geometry stays density-aware, and hit targets never shrink below supported
//! minima.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- fixture-typography-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- fixture-chart-token-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- validate
//! ```

use aureline_ui::m5_visual_foundation_matrix::{
    seeded_m5_visual_foundation_matrix,
    seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed,
    seeded_m5_visual_foundation_matrix_typography_beta_narrowed, M5VisualFoundationMatrixPacket,
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
            let packet = seeded_m5_visual_foundation_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_visual_foundation_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_visual_foundation_matrix().render_matrix_csv()
            );
        }
        Some("fixture-typography-beta-narrowed") => {
            let packet = seeded_m5_visual_foundation_matrix_typography_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-chart-token-preview-narrowed") => {
            let packet = seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_visual_foundation_matrix(),
                seeded_m5_visual_foundation_matrix_typography_beta_narrowed(),
                seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed(),
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

fn assert_valid(packet: &M5VisualFoundationMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
