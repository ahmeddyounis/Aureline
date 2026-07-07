//! Headless emitter for the M5 inline-result-marker primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-inline-result-marker-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-inline-result-marker-primitive/`. The editor-gutter marker, the editor
//! inline marker, the notebook-cell marker, the headless/CLI marker, and the marker-report
//! export consumers read this matrix so one inline result marker names its
//! pass/fail/error/timeout verdict, stability-or-flaky chip, imported/live origin class,
//! last-result freshness, source-mapping fidelity, target/environment shorthand, and
//! attempt lineage without ever letting an imported, stale, or approximately-mapped run
//! read as a current live-local result.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_inline_result_marker_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_inline_result_marker_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_inline_result_marker_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_inline_result_marker_primitive -- fixture-notebook-cell-marker-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_inline_result_marker_primitive -- fixture-headless-cli-marker-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_inline_result_marker_primitive -- validate
//! ```

use aureline_runtime::implement_inline_result_markers_with_live_versus_imported_versus_stale_stability_chips_open_recent_attempts_and_target_env_shorthand_across_claimed_m5_editors_and_notebook_views::{
    seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed,
    seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed,
    seeded_m5_inline_result_marker_packet, M5InlineResultMarkerPacket,
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
            let packet = seeded_m5_inline_result_marker_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_inline_result_marker_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_inline_result_marker_packet().render_matrix_csv()
            );
        }
        Some("fixture-notebook-cell-marker-preview-narrowed") => {
            let packet = seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-marker-beta-narrowed") => {
            let packet = seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_inline_result_marker_packet(),
                seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed(),
                seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed(),
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

fn assert_valid(packet: &M5InlineResultMarkerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "inline marker primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
