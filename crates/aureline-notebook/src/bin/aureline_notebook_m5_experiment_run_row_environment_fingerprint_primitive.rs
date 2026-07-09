//! Headless emitter for the M5 experiment-run-row / environment-fingerprint controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/`, its matrix CSV,
//! the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-experiment-run-row-environment-fingerprint-controls/`. The experiment-run
//! dashboard and the environment-fingerprint surfaces read these components so one run row
//! names where a run came from, its commit or workspace revision, its execution origin, and
//! its outcome — and offers open / compare / export — and one fingerprint card names its
//! captured interpreter or kernel, package / toolchain summary, execution target, hardware /
//! profile class, and freshness, and offers inspect / export paths, so an imported, manually
//! attached, or unknown-origin run never reads as a first-party run and an uncaptured
//! environment never reads as captured before a compare or share.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- fixture-run-row-imported
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- fixture-fingerprint-card-uncaptured
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- validate
//! ```

use aureline_notebook::implement_experiment_run_rows_and_environment_fingerprint_cards_with_run_origin_code_revision_execution_target_and_outcome_truth_across_claimed_m5_notebook_and_data_surfaces::{
    seeded_experiment_run_row_environment_fingerprint_controls,
    seeded_experiment_run_row_environment_fingerprint_controls_fingerprint_card_uncaptured,
    seeded_experiment_run_row_environment_fingerprint_controls_run_row_imported,
    ExperimentRunRowEnvironmentFingerprintControlsPacket,
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
            let packet = seeded_experiment_run_row_environment_fingerprint_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_experiment_run_row_environment_fingerprint_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_experiment_run_row_environment_fingerprint_controls().render_matrix_csv()
            );
        }
        Some("fixture-run-row-imported") => {
            let packet =
                seeded_experiment_run_row_environment_fingerprint_controls_run_row_imported();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-fingerprint-card-uncaptured") => {
            let packet =
                seeded_experiment_run_row_environment_fingerprint_controls_fingerprint_card_uncaptured();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_experiment_run_row_environment_fingerprint_controls(),
                seeded_experiment_run_row_environment_fingerprint_controls_run_row_imported(),
                seeded_experiment_run_row_environment_fingerprint_controls_fingerprint_card_uncaptured(),
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
    packet: &ExperimentRunRowEnvironmentFingerprintControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "experiment run row fingerprint primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
