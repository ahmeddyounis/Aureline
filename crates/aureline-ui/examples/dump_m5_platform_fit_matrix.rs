//! Headless emitter for the frozen M5 platform-fit matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-platform-fit-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/platform/m5-desktop-fit/`. The macOS, Windows, and Linux desktop
//! surfaces, docs, and support read this matrix so command IDs stay stable while platform labels and
//! shortcut notation adapt, primary actions are never hidden in OS chrome alone, file / path / reveal /
//! save terminology matches the host, theme / contrast / accent / text-scale changes apply live or explain
//! their fallback, credential-store wording stays truthful and non-leaky, and IME / dead-key / dictation /
//! layout switching never corrupts text or trust fidelity.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- fixture-theme-contrast-live-change-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- fixture-input-method-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- validate
//! ```

use aureline_ui::m5_platform_fit_matrix::{
    seeded_m5_platform_fit_matrix, seeded_m5_platform_fit_matrix_input_method_preview_narrowed,
    seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed,
    M5PlatformFitMatrixPacket,
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
            let packet = seeded_m5_platform_fit_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_platform_fit_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_platform_fit_matrix().render_matrix_csv());
        }
        Some("fixture-theme-contrast-live-change-beta-narrowed") => {
            let packet = seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-input-method-preview-narrowed") => {
            let packet = seeded_m5_platform_fit_matrix_input_method_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_platform_fit_matrix(),
                seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed(),
                seeded_m5_platform_fit_matrix_input_method_preview_narrowed(),
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

fn assert_valid(packet: &M5PlatformFitMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
