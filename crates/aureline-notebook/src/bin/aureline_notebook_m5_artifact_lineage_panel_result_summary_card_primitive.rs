//! Headless emitter for the M5 artifact-lineage-panel / result-summary-card controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-artifact-lineage-panel-result-summary-card-controls/`. The lineage and
//! result-summary surfaces read these components so one lineage panel names its artifact
//! identity, artifact kind, producing run ID, generator step, environment / model fingerprint,
//! saved scope, and lineage state — and offers open-artifact / trace-to-run / export-lineage —
//! and one summary card names its headline metrics, artifact count, freshness, support / report
//! scope, include-raw toggle, provenance note, and summary-versus-evidence-versus-raw handoff
//! choice, and offers review / share-summary-only paths, so a generated artifact never reads as
//! an anonymous attachment and a raw payload is never included by default.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- fixture-lineage-panel-broken
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- fixture-summary-card-raw-payload
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- validate
//! ```

use aureline_notebook::implement_artifact_lineage_panels_and_result_summary_cards_with_producing_run_identity_stale_diverged_notes_include_raw_toggles_and_export_boundary_truth_across_claimed_m5_experiment_surfaces::{
    seeded_artifact_lineage_panel_result_summary_card_controls,
    seeded_artifact_lineage_panel_result_summary_card_controls_lineage_panel_broken,
    seeded_artifact_lineage_panel_result_summary_card_controls_summary_card_raw_payload,
    ArtifactLineagePanelResultSummaryCardControlsPacket,
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
            let packet = seeded_artifact_lineage_panel_result_summary_card_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_artifact_lineage_panel_result_summary_card_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_artifact_lineage_panel_result_summary_card_controls().render_matrix_csv()
            );
        }
        Some("fixture-lineage-panel-broken") => {
            let packet =
                seeded_artifact_lineage_panel_result_summary_card_controls_lineage_panel_broken();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-summary-card-raw-payload") => {
            let packet =
                seeded_artifact_lineage_panel_result_summary_card_controls_summary_card_raw_payload(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_artifact_lineage_panel_result_summary_card_controls(),
                seeded_artifact_lineage_panel_result_summary_card_controls_lineage_panel_broken(),
                seeded_artifact_lineage_panel_result_summary_card_controls_summary_card_raw_payload(
                ),
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
    packet: &ArtifactLineagePanelResultSummaryCardControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "artifact lineage summary card primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
