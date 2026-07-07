//! Headless emitter for the frozen M5 test-explorer / watch / triage component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-test-explorer-watch-triage-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-test-explorer-watch-triage-components/`. Test explorer, editor
//! gutter, status bar, run panel, CI summary, and CLI test surfaces read this matrix so
//! one test-tree row names its identity class and imported/live origin, one inline
//! result marker names its verdict and freshness, one session-summary bar names its
//! outcome and attempt lineage, one watch-mode banner names its fidelity and why it
//! degraded, one failure-triage panel names its category and disposition, one
//! quarantine-review sheet names its ownership and release impact, and one
//! environment-matrix card names its target and environment.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- fixture-watch-mode-banner-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- fixture-quarantine-review-sheet-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- validate
//! ```

use aureline_runtime::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    seeded_m5_test_explorer_watch_triage_component_matrix,
    seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed,
    seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed,
    M5TestExplorerWatchTriageComponentMatrixPacket,
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
            let packet = seeded_m5_test_explorer_watch_triage_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_test_explorer_watch_triage_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_test_explorer_watch_triage_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-watch-mode-banner-beta-narrowed") => {
            let packet =
                seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-quarantine-review-sheet-preview-narrowed") => {
            let packet =
                seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_test_explorer_watch_triage_component_matrix(),
                seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed(),
                seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed(),
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
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
