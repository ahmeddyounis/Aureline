//! Headless emitter for the frozen M5 experiment-component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-experiment-component-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-experiment-components/`.
//! Notebook, experiment-dashboard, run-comparison, data-catalog, share-review, and CLI
//! surfaces read this matrix so one experiment run row names where a run came from and its
//! code revision, one dataset provenance card names what data was used, one artifact lineage
//! panel names its upstream and downstream, one run comparison table never implies
//! apples-to-apples without parity evidence, one environment fingerprint card names its
//! captured environment, one compare guard banner names why a comparison is guarded, one
//! sensitivity / sharing banner never exposes raw production-like data by default, and one
//! result summary card names its summary-versus-evidence-versus-raw export scope.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- fixture-run-comparison-table-beta-narrowed
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- fixture-sensitivity-sharing-banner-preview-narrowed
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- validate
//! ```

use aureline_notebook::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    seeded_m5_experiment_component_matrix,
    seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed,
    seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed,
    M5ExperimentComponentMatrixPacket,
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
            let packet = seeded_m5_experiment_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_experiment_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_experiment_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-run-comparison-table-beta-narrowed") => {
            let packet = seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-sensitivity-sharing-banner-preview-narrowed") => {
            let packet =
                seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_experiment_component_matrix(),
                seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed(),
                seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed(),
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
    packet: &M5ExperimentComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
