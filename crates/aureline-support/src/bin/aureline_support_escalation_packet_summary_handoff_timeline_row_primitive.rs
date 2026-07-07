//! Headless emitter for the M5 escalation-packet-summary / handoff-timeline-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-support-escalation-packet-summary-handoff-timeline-row-primitive-proof/`,
//! its matrix CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-support-escalation-packet-summary-handoff-timeline-row-primitive/`.
//! Support-center, recovery-center, Doctor-handoff, headless/CLI, and support-packet
//! consumers read this matrix so one escalation-packet summary keeps its packet id,
//! scenario code, finding / crash lineage, repair attempts, redaction posture, build /
//! profile identity, and destination legible with explicit confirm / cancel actions, and
//! one handoff-timeline row keeps its event identity, owner, current owner, related
//! evidence, and next expected step legible over time.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- support-export
//! cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- report
//! cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- csv
//! cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- fixture-recovery-center-handoff-preview-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- fixture-headless-cli-escalation-beta-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- validate
//! ```

use aureline_support::implement_escalation_packet_summaries_and_handoff_timeline_rows_with_packet_id_scenario_code_finding_repair_lineage_owner_destination_and_next_step_truth_across_claimed_m5_support_lanes::{
    seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed,
    seeded_m5_escalation_handoff_packet,
    seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed,
    M5EscalationHandoffPacket,
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
            let packet = seeded_m5_escalation_handoff_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_escalation_handoff_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_escalation_handoff_packet().render_matrix_csv()
            );
        }
        Some("fixture-recovery-center-handoff-preview-narrowed") => {
            let packet = seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-escalation-beta-narrowed") => {
            let packet = seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_escalation_handoff_packet(),
                seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed(),
                seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed(),
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

fn assert_valid(packet: &M5EscalationHandoffPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "escalation handoff primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
