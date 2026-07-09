//! Headless emitter for the frozen M5 learning-component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-learning-component-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-learning-components/`.
//! First-run, guided-tour, learning-mode, glossary, inline-help, and CLI help surfaces
//! read this matrix so one learning-mode toggle names its state and scope, one tip card
//! names its cited source and dismissal state, one guided exercise step names its state and
//! that it never hides an apply, one glossary chip or card names its cited source and
//! citation state, one safe explanation banner names its explain-versus-do boundary, and
//! one progress marker names that its progress is user-owned and default-local.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- fixture-learning-mode-toggle-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- fixture-progress-marker-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- validate
//! ```

use aureline_learning::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    seeded_m5_learning_component_matrix,
    seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed,
    seeded_m5_learning_component_matrix_progress_marker_preview_narrowed,
    M5LearningComponentMatrixPacket,
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
            let packet = seeded_m5_learning_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_learning_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_learning_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-learning-mode-toggle-beta-narrowed") => {
            let packet = seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-progress-marker-preview-narrowed") => {
            let packet = seeded_m5_learning_component_matrix_progress_marker_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_learning_component_matrix(),
                seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed(),
                seeded_m5_learning_component_matrix_progress_marker_preview_narrowed(),
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
    packet: &M5LearningComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
