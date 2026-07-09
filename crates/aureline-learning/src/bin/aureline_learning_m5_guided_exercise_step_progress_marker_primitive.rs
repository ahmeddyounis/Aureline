//! Headless emitter for the M5 guided-exercise-step / progress-marker controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-guided-exercise-step-progress-marker-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-guided-exercise-step-progress-marker-controls/`. The exercise surfaces and
//! the progress surfaces read these controls so one guided exercise step names exactly what
//! to act on, what counts as success, how to hint / reveal / reset / skip it, and whether a
//! mutating lesson runs in a sandbox or behind a preview, and one progress marker names how
//! much is completed and remaining and how to resume / reset / export it — with progress
//! staying user-owned, default-local, and never silently shared beyond the supported scope.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- fixture-guided-exercise-step-retryable
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- fixture-progress-marker-reset
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- validate
//! ```

use aureline_learning::implement_guided_exercise_steps_and_progress_markers_with_target_object_success_criteria_hint_reveal_reset_skip_sandbox_or_preview_preference_and_privacy_bounded_resume_export_truth_across_claimed_m5_learnability_lanes::{
    seeded_guided_exercise_step_progress_marker_controls,
    seeded_guided_exercise_step_progress_marker_controls_guided_exercise_step_retryable,
    seeded_guided_exercise_step_progress_marker_controls_progress_marker_reset,
    GuidedExerciseStepProgressMarkerControlsPacket,
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
            let packet = seeded_guided_exercise_step_progress_marker_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_guided_exercise_step_progress_marker_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_guided_exercise_step_progress_marker_controls().render_matrix_csv()
            );
        }
        Some("fixture-guided-exercise-step-retryable") => {
            let packet =
                seeded_guided_exercise_step_progress_marker_controls_guided_exercise_step_retryable(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-progress-marker-reset") => {
            let packet =
                seeded_guided_exercise_step_progress_marker_controls_progress_marker_reset();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_guided_exercise_step_progress_marker_controls(),
                seeded_guided_exercise_step_progress_marker_controls_guided_exercise_step_retryable(
                ),
                seeded_guided_exercise_step_progress_marker_controls_progress_marker_reset(),
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
    packet: &GuidedExerciseStepProgressMarkerControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "guided exercise step progress marker primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
