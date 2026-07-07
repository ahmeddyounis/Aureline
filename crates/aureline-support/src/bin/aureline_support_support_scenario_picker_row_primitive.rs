//! Headless emitter for the M5 support-scenario-picker-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-support-scenario-picker-row-primitive-proof/`, its matrix CSV,
//! the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-support-scenario-picker-row-primitive/`. Project Doctor,
//! support-center, recovery-center, headless/CLI, and support-packet consumers read this
//! matrix so one support-scenario picker row names its stable scenario family, user-facing
//! symptom cue, claimed launch/deployment/profile scope, and bound Doctor finding family
//! with a scenario-coded start-diagnosis action that never loses a same-weight local-only
//! route.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-support --bin aureline_support_support_scenario_picker_row_primitive -- support-export
//! cargo run -q -p aureline-support --bin aureline_support_support_scenario_picker_row_primitive -- report
//! cargo run -q -p aureline-support --bin aureline_support_support_scenario_picker_row_primitive -- csv
//! cargo run -q -p aureline-support --bin aureline_support_support_scenario_picker_row_primitive -- fixture-recovery-center-intake-preview-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_support_scenario_picker_row_primitive -- fixture-headless-cli-intake-beta-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_support_scenario_picker_row_primitive -- validate
//! ```

use aureline_support::implement_support_scenario_picker_rows_and_seeded_symptom_scope_cues_with_start_diagnosis_parity_across_claimed_m5_support_intake_surfaces::{
    seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed,
    seeded_m5_support_scenario_picker_row_packet,
    seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed,
    M5ScenarioPickerRowPacket,
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
            let packet = seeded_m5_support_scenario_picker_row_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_support_scenario_picker_row_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_support_scenario_picker_row_packet().render_matrix_csv()
            );
        }
        Some("fixture-recovery-center-intake-preview-narrowed") => {
            let packet =
                seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-intake-beta-narrowed") => {
            let packet = seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_support_scenario_picker_row_packet(),
                seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed(),
                seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed(),
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

fn assert_valid(packet: &M5ScenarioPickerRowPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("picker row primitive failed validation: {}", tokens.join(",")).into())
    }
}
