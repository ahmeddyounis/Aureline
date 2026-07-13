//! Headless emitter for the M5 shell-metric and minimum-size registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-shell-metric-and-minimum-size-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- report
//! cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- fixture-editor-ui-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- fixture-data-ui-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- validate
//! ```

use aureline_ui::m5_shell_metric_and_minimum_size_registries::{
    seeded_m5_shell_metric_minimum_size_registries,
    seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed,
    seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed,
    M5ShellMetricRegistriesPacket,
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
            let packet = seeded_m5_shell_metric_minimum_size_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_shell_metric_minimum_size_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_shell_metric_minimum_size_registries().render_matrix_csv()
            );
        }
        Some("fixture-editor-ui-beta-narrowed") => {
            let packet = seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-data-ui-preview-narrowed") => {
            let packet = seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_shell_metric_minimum_size_registries(),
                seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed(),
                seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed(),
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

fn assert_valid(packet: &M5ShellMetricRegistriesPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
