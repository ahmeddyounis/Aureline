//! Headless emitter for the M5 failure-triage-panel / quarantine-review-sheet /
//! environment-matrix-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-failure-triage-quarantine-environment-primitive-proof/`, its matrix
//! CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-failure-triage-quarantine-environment-primitive/`. The test-explorer triage
//! view, the editor inline triage, the notebook triage view, the run-panel triage, and the
//! quality report export consumers read this matrix so one failure-triage panel names its
//! failure category, disposition, confidence, origin, recent attempts, and deltas and only
//! escalates to suppression once evidence context is present, one quarantine-review sheet keeps
//! a suppressed test visible with its owner/expiry/release impact and a restore action, and one
//! environment-matrix card compares target/runtime/toolchain/build compatibility without
//! implying safe equivalence.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- fixture-notebook-triage-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- fixture-editor-inline-triage-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- validate
//! ```

use aureline_runtime::implement_failure_triage_panels_quarantine_review_sheets_and_environment_matrix_cards_with_assertion_diff_summaries_recent_attempts_env_build_runtime_deltas_owner_expiry_release_impact_and_rerun_debug_review_parity_across_claimed_m5_quality_surfaces::{
    seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed,
    seeded_m5_quality_triage_status_notebook_triage_preview_narrowed,
    seeded_m5_quality_triage_status_packet, M5QualityTriageStatusPacket,
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
            let packet = seeded_m5_quality_triage_status_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_quality_triage_status_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_quality_triage_status_packet().render_matrix_csv()
            );
        }
        Some("fixture-notebook-triage-preview-narrowed") => {
            let packet = seeded_m5_quality_triage_status_notebook_triage_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-editor-inline-triage-beta-narrowed") => {
            let packet = seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_quality_triage_status_packet(),
                seeded_m5_quality_triage_status_notebook_triage_preview_narrowed(),
                seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed(),
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

fn assert_valid(packet: &M5QualityTriageStatusPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "quality triage status primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
