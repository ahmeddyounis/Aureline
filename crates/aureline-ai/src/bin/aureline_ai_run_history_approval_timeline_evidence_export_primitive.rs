//! Headless emitter for the M5 AI run-history-row / approval-timeline-entry /
//! evidence-export-summary primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/`.
//! The run-history, evidence-packet, export, support, and replay surfaces all read this
//! primitive so one run-history row names its canonical run id, task, time, provider/model
//! route, and outcome with stable open/replay/export entry points, one approval-timeline
//! entry preserves its actor, scope, policy epoch, and expiry state, and one evidence /
//! export summary names its packet id, artifact classes, redaction posture, support
//! linkage, and export formats.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_run_history_approval_timeline_evidence_export_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_run_history_approval_timeline_evidence_export_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_run_history_approval_timeline_evidence_export_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_run_history_approval_timeline_evidence_export_primitive -- fixture-evidence-export-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_run_history_approval_timeline_evidence_export_primitive -- fixture-support-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_run_history_approval_timeline_evidence_export_primitive -- validate
//! ```

use aureline_ai::ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces::{
    seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed,
    seeded_m5_ai_run_history_export_primitive_packet,
    seeded_m5_ai_run_history_export_primitive_support_beta_narrowed,
    M5AiRunHistoryExportPrimitivePacket,
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
            let packet = seeded_m5_ai_run_history_export_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_run_history_export_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_run_history_export_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-evidence-export-preview-narrowed") => {
            let packet =
                seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-beta-narrowed") => {
            let packet = seeded_m5_ai_run_history_export_primitive_support_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_run_history_export_primitive_packet(),
                seeded_m5_ai_run_history_export_primitive_evidence_export_preview_narrowed(),
                seeded_m5_ai_run_history_export_primitive_support_beta_narrowed(),
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
    packet: &M5AiRunHistoryExportPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
