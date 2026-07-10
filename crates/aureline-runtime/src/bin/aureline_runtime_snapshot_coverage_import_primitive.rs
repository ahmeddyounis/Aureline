//! Headless emitter for the M5 snapshot-review-card / coverage-import-merge-sheet primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-snapshot-coverage-import-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-snapshot-coverage-import-primitive/`. The snapshot review panel, the editor
//! snapshot diff, the coverage-import / merge panel, the headless/CLI review surface, and the
//! review export consumers read this matrix so one snapshot / golden review card names its
//! artifact kind, baseline identity, diff count, render/raw fallback mode, and scope without
//! collapsing to a blind Accept all, and one coverage-import / merge sheet exposes its included
//! and excluded runs, commit/build identity, stale-or-incompatible warnings, and line-versus-branch
//! support before any merged number is treated as exact current truth.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_snapshot_coverage_import_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_snapshot_coverage_import_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_snapshot_coverage_import_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_snapshot_coverage_import_primitive -- fixture-snapshot-review-panel-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_snapshot_coverage_import_primitive -- fixture-coverage-import-merge-panel-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_snapshot_coverage_import_primitive -- validate
//! ```

use aureline_runtime::implement_snapshot_or_golden_review_cards_and_coverage_import_merge_sheets_with_artifact_baseline_identity_raw_or_text_fallback_shard_inclusion_truth_and_stale_or_incompatible_warnings_across_claimed_m5_review_surfaces::{
    seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed,
    seeded_m5_snapshot_merge_components_packet,
    seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed,
    M5SnapshotMergeComponentsPacket,
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
            let packet = seeded_m5_snapshot_merge_components_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_snapshot_merge_components_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_snapshot_merge_components_packet().render_matrix_csv()
            );
        }
        Some("fixture-snapshot-review-panel-preview-narrowed") => {
            let packet =
                seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-coverage-import-merge-panel-beta-narrowed") => {
            let packet =
                seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_snapshot_merge_components_packet(),
                seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed(),
                seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed(),
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
    packet: &M5SnapshotMergeComponentsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "snapshot merge components primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
