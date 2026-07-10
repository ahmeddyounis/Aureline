//! Headless emitter for the frozen M5 efficiency component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-efficiency-components-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-efficiency-components/`. Shell
//! status, activity-center, diagnostics, Help/About, support, and policy-aware settings
//! surfaces read this matrix so one power-state indicator names the source of change and active
//! state, one throttled-subsystem row names which work slowed, one background-work row/banner
//! names slowed-versus-paused work explicitly, one per-workspace override sheet names override
//! availability and its policy owner, one resume-summary card names the resumed-work backlog,
//! and one stale-result continuity note keeps stale context across resume.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- support-export
//! cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- report
//! cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- csv
//! cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- fixture-override-sheet-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- fixture-stale-result-note-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::{
    seeded_m5_efficiency_component_matrix,
    seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed,
    seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed,
    M5EfficiencyComponentMatrixPacket,
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
            let packet = seeded_m5_efficiency_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_efficiency_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_efficiency_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-override-sheet-beta-narrowed") => {
            let packet = seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-stale-result-note-preview-narrowed") => {
            let packet = seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_efficiency_component_matrix(),
                seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed(),
                seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed(),
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
    packet: &M5EfficiencyComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
