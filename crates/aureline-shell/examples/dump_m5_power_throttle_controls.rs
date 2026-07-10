//! Headless emitter for the M5 power-state / throttled-subsystem controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-power-state-throttled-subsystem-controls-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-power-state-throttled-subsystem-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_power_throttle_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_power_throttle_controls -- report
//! cargo run -p aureline-shell --example dump_m5_power_throttle_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_power_throttle_controls -- fixture-activity-center-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_power_throttle_controls -- fixture-diagnostics-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_power_throttle_controls -- validate
//! ```

use aureline_shell::implement_the_m5_power_state_indicator_and_throttled_subsystem_row_source_active_state_affected_subsystem_and_inspect_path_primitive::{
    seeded_m5_power_throttle_controls,
    seeded_m5_power_throttle_controls_activity_center_beta_narrowed,
    seeded_m5_power_throttle_controls_diagnostics_preview_narrowed, M5PowerThrottleControlsPacket,
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
            let packet = seeded_m5_power_throttle_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_power_throttle_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_power_throttle_controls().render_matrix_csv()
            );
        }
        Some("fixture-activity-center-beta-narrowed") => {
            let packet = seeded_m5_power_throttle_controls_activity_center_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-diagnostics-preview-narrowed") => {
            let packet = seeded_m5_power_throttle_controls_diagnostics_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_power_throttle_controls(),
                seeded_m5_power_throttle_controls_activity_center_beta_narrowed(),
                seeded_m5_power_throttle_controls_diagnostics_preview_narrowed(),
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

fn assert_valid(packet: &M5PowerThrottleControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
