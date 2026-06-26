//! Headless emitter for the frozen M5 accessibility-bridge, live-announcement,
//! focus-return, and non-visual dynamic-surface matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-dynamic-surfaces/` and the narrowed fixtures under
//! `fixtures/a11y/m5-dynamic-surfaces/`. Release, help, docs, and support automation
//! read this matrix so claimed M5 dynamic surfaces cannot harden
//! screen-reader/keyboard-complete claims without a governed assistive-tech source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_surface_a11y_matrix -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_surface_a11y_matrix -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_surface_a11y_matrix -- fixture-bridge-unavailable
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_surface_a11y_matrix -- fixture-dense-summary-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_surface_a11y_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix::{
    seeded_m5_dynamic_surface_a11y_matrix,
    seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable,
    seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed,
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
            let packet = seeded_m5_dynamic_surface_a11y_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_dynamic_surface_a11y_matrix().render_markdown_summary()
            );
        }
        Some("fixture-bridge-unavailable") => {
            let packet = seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-dense-summary-narrowed") => {
            let packet = seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_dynamic_surface_a11y_matrix(),
                seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable(),
                seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed(),
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
    packet: &aureline_shell::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix::M5DynamicSurfaceA11yMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
