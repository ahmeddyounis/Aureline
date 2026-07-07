//! Headless emitter for the M5 test-tree-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-test-tree-row-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-test-tree-row-primitive/`. The test-explorer tree, editor-gutter tree,
//! run-panel tree, headless/CLI tree, and test-report export consumers read this matrix so
//! one test-tree row names its item class, stable identity, current state, last-result
//! freshness, imported/live origin, target/environment shorthand, parameterized-case
//! count, and mute/quarantine state with an exact rerun scope that never lets an imported
//! or partial-discovery item read as a current live result.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_tree_row_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_tree_row_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_tree_row_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_tree_row_primitive -- fixture-run-panel-tree-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_tree_row_primitive -- fixture-headless-cli-tree-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_tree_row_primitive -- validate
//! ```

use aureline_runtime::implement_test_tree_rows_with_suite_template_case_notebook_imported_result_distinction_parameterized_counts_freshness_target_chip_and_mute_quarantine_truth_across_claimed_m5_test_surfaces::{
    seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed, seeded_m5_test_tree_row_packet,
    seeded_m5_test_tree_row_run_panel_tree_preview_narrowed, M5TestTreeRowPacket,
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
            let packet = seeded_m5_test_tree_row_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_test_tree_row_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_test_tree_row_packet().render_matrix_csv());
        }
        Some("fixture-run-panel-tree-preview-narrowed") => {
            let packet = seeded_m5_test_tree_row_run_panel_tree_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-tree-beta-narrowed") => {
            let packet = seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_test_tree_row_packet(),
                seeded_m5_test_tree_row_run_panel_tree_preview_narrowed(),
                seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed(),
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

fn assert_valid(packet: &M5TestTreeRowPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("tree row primitive failed validation: {}", tokens.join(",")).into())
    }
}
