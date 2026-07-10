//! Headless emitter for the M5 coverage-summary-bar / coverage-overlay-marker primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-coverage-summary-overlay-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-coverage-summary-overlay-primitive/`. The coverage-report panel, the editor
//! gutter overlay, the CI coverage summary, the headless/CLI coverage surface, and the coverage
//! report export consumers read this matrix so one coverage-summary bar names its scope, metric
//! dimension, included run set, freshness, and imported/merged/live source note without
//! collapsing multi-run or imported evidence into one unlabeled percentage, and one
//! coverage-overlay marker preserves its exact covered/uncovered/partial/branch-missed/excluded/
//! unknown state, its changed-line emphasis, its source run-set identity, and a durable path
//! back to the evidence object that produced it.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_coverage_summary_overlay_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_coverage_summary_overlay_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_coverage_summary_overlay_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_coverage_summary_overlay_primitive -- fixture-report-panel-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_coverage_summary_overlay_primitive -- fixture-editor-gutter-overlay-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_coverage_summary_overlay_primitive -- validate
//! ```

use aureline_runtime::implement_coverage_summary_bars_and_coverage_overlay_markers_with_included_run_provenance_line_versus_branch_or_partial_truth_changed_file_emphasis_and_open_report_continuity_across_claimed_m5_test_surfaces::{
    seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed,
    seeded_m5_coverage_components_packet,
    seeded_m5_coverage_components_report_panel_preview_narrowed, M5CoverageComponentsPacket,
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
            let packet = seeded_m5_coverage_components_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_coverage_components_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_coverage_components_packet().render_matrix_csv()
            );
        }
        Some("fixture-report-panel-preview-narrowed") => {
            let packet = seeded_m5_coverage_components_report_panel_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-editor-gutter-overlay-beta-narrowed") => {
            let packet = seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_coverage_components_packet(),
                seeded_m5_coverage_components_report_panel_preview_narrowed(),
                seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed(),
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

fn assert_valid(packet: &M5CoverageComponentsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "coverage components primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
