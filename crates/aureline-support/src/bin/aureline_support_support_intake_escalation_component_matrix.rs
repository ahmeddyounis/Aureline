//! Headless emitter for the frozen M5 support-intake / escalation component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-support-intake-escalation-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-support-intake-escalation-components/`. Project Doctor,
//! support-center, report-builder, escalation-desk, recovery-center, Help, and admin
//! surfaces read this matrix so one support-scenario picker row names its scenario
//! family, incident scope, and bound Doctor finding family, one issue-report builder
//! step names which evidence it selects and omits, one escalation-packet summary names
//! its destination and redaction, one handoff-timeline row names its stage and next
//! human step, and one unsafe-fix blocked note names why a fix is blocked and which
//! repair class is approved instead.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- support-export
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- report
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- csv
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- fixture-escalation-packet-summary-beta-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- fixture-unsafe-fix-blocked-note-preview-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- validate
//! ```

use aureline_support::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    seeded_m5_support_intake_escalation_component_matrix,
    seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed,
    seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed,
    M5SupportIntakeEscalationComponentMatrixPacket,
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
            let packet = seeded_m5_support_intake_escalation_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_support_intake_escalation_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_support_intake_escalation_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-escalation-packet-summary-beta-narrowed") => {
            let packet =
                seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-unsafe-fix-blocked-note-preview-narrowed") => {
            let packet =
                seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_support_intake_escalation_component_matrix(),
                seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed(),
                seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed(),
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
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
