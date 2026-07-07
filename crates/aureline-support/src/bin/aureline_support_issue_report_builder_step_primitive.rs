//! Headless emitter for the M5 issue-report-builder-step primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-support-issue-report-builder-step-primitive-proof/`, its matrix
//! CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-support-issue-report-builder-step-primitive/`. Support-center,
//! recovery-center, Doctor-handoff, headless/CLI, and support-packet consumers read this
//! matrix so one issue-report builder step names its human-readable summary, ordered
//! reproduction steps, selected and excluded evidence classes, their data-risk class,
//! redaction posture, and per-class local-boundary disposition without ever collapsing the
//! draft into one opaque blob or losing a same-weight local-only preview.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-support --bin aureline_support_issue_report_builder_step_primitive -- support-export
//! cargo run -q -p aureline-support --bin aureline_support_issue_report_builder_step_primitive -- report
//! cargo run -q -p aureline-support --bin aureline_support_issue_report_builder_step_primitive -- csv
//! cargo run -q -p aureline-support --bin aureline_support_issue_report_builder_step_primitive -- fixture-recovery-center-builder-preview-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_issue_report_builder_step_primitive -- fixture-headless-cli-builder-beta-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_issue_report_builder_step_primitive -- validate
//! ```

use aureline_support::implement_issue_report_builder_steps_and_evidence_class_selectors_with_included_excluded_redaction_repro_and_local_only_preview_truth_across_claimed_m5_support_flows::{
    seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed,
    seeded_m5_issue_report_builder_step_packet,
    seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed,
    M5IssueReportBuilderStepPacket,
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
            let packet = seeded_m5_issue_report_builder_step_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_issue_report_builder_step_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_issue_report_builder_step_packet().render_matrix_csv()
            );
        }
        Some("fixture-recovery-center-builder-preview-narrowed") => {
            let packet =
                seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-builder-beta-narrowed") => {
            let packet = seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_issue_report_builder_step_packet(),
                seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed(),
                seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed(),
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

fn assert_valid(packet: &M5IssueReportBuilderStepPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "builder step primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
