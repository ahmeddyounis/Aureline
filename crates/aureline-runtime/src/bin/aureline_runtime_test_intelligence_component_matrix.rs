//! Headless emitter for the frozen M5 test-intelligence component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-test-intelligence-component-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-test-intelligence-components/`. Editor overlay, coverage report,
//! test tree, review, CI summary, and CLI surfaces read this matrix so one
//! coverage-summary bar names its scope and metric, one coverage-overlay marker names
//! its state and emphasis, one flaky-state badge names its classification and
//! confidence, one retry-history row names its outcome and rerun scope, one
//! snapshot-review card names its baseline and diff state, one coverage-import/merge
//! sheet names its source and merge resolution, and one test-generation suggestion
//! card names its assumptions and apply scope.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- fixture-flaky-state-badge-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- fixture-coverage-import-merge-sheet-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- validate
//! ```

use aureline_runtime::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    seeded_m5_test_intelligence_component_matrix,
    seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed,
    seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed,
    M5TestIntelligenceComponentMatrixPacket,
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
            let packet = seeded_m5_test_intelligence_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_test_intelligence_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_test_intelligence_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-flaky-state-badge-beta-narrowed") => {
            let packet =
                seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-coverage-import-merge-sheet-preview-narrowed") => {
            let packet =
                seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_test_intelligence_component_matrix(),
                seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed(),
                seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed(),
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
    packet: &M5TestIntelligenceComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
