//! Headless emitter for the frozen M5 shell-metric / density matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-shell-metric-density-proof/`, its matrix CSV, the Markdown design report,
//! and the narrowed fixtures under `fixtures/ui/m5-shell-metric-density/`. The desktop shell, editor,
//! review, notebook, and data surfaces read this matrix so the main workspace stays dominant, zones
//! honor declared minimum and recommended sizes, density changes presentation rather than information
//! architecture, responsive collapse preserves task identity and recovery-critical state, hit targets
//! never shrink below the supported minimum, and extension or embedded surfaces never invent private
//! widths that fracture the shell.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- fixture-responsive-geometry-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- fixture-collapse-priority-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- validate
//! ```

use aureline_ui::m5_shell_metric_density_matrix::{
    seeded_m5_shell_metric_density_matrix,
    seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed,
    seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed,
    M5ShellMetricDensityMatrixPacket,
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
            let packet = seeded_m5_shell_metric_density_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_shell_metric_density_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_shell_metric_density_matrix().render_matrix_csv()
            );
        }
        Some("fixture-responsive-geometry-beta-narrowed") => {
            let packet = seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-collapse-priority-preview-narrowed") => {
            let packet = seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_shell_metric_density_matrix(),
                seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed(),
                seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed(),
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
    packet: &M5ShellMetricDensityMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
