//! Headless emitter for the frozen M5 decision-feedback component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-decision-feedback-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/ui/m5-decision-feedback-components/`. The shell, entry, trust,
//! review, repair, and notification surfaces read this matrix so one badge expands into plain language,
//! one popover keeps safe focus return, one dialog names rationale/scope/explicit actions, one banner
//! stays scoped and actionable, one toast never becomes the only durable truth, one empty state explains
//! purpose/emptiness/next action, one loading state preserves partial data, and one consequence block
//! names its blast radius and rollback posture.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_decision_feedback_component_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_decision_feedback_component_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_decision_feedback_component_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_decision_feedback_component_matrix -- fixture-dialog-sheet-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_decision_feedback_component_matrix -- fixture-loading-state-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_decision_feedback_component_matrix -- validate
//! ```

use aureline_ui::m5_decision_feedback_component_matrix::{
    seeded_m5_decision_feedback_component_matrix,
    seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed,
    seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed,
    M5DecisionFeedbackComponentMatrixPacket,
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
            let packet = seeded_m5_decision_feedback_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_decision_feedback_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_decision_feedback_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-dialog-sheet-beta-narrowed") => {
            let packet = seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-loading-state-preview-narrowed") => {
            let packet =
                seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_decision_feedback_component_matrix(),
                seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed(),
                seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed(),
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
    packet: &M5DecisionFeedbackComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
