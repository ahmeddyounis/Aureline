//! Headless emitter for the M5 related-evidence / offline-handoff controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-related-evidence-offline-handoff-proof/`, its matrix CSV, the
//! Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-related-evidence-offline-handoff-controls/`. Work-item detail, review,
//! incident, support, and CLI surfaces read these controls so one related-evidence card
//! summarizes its linked context summary-first with derived freshness and an open-detail
//! action, and one offline-handoff packet card shows its type, included content, redaction
//! state, and publish-later target, staying visible, retryable, and exportable after
//! failure without implying provider acceptance.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- fixture-related-evidence-summary-first
//! cargo run -q -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- fixture-offline-packet-publish-failed
//! cargo run -q -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- validate
//! ```

use aureline_provider::implement_related_evidence_cards_and_offline_handoff_packet_cards_with_summary_first_evidence_redaction_state_publish_later_target_and_copy_export_retry_truth::{
    seeded_related_evidence_offline_handoff_controls,
    seeded_related_evidence_offline_handoff_controls_offline_packet_publish_failed,
    seeded_related_evidence_offline_handoff_controls_related_evidence_summary_first,
    EvidenceHandoffControlsPacket,
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
            let packet = seeded_related_evidence_offline_handoff_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_related_evidence_offline_handoff_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_related_evidence_offline_handoff_controls().render_matrix_csv()
            );
        }
        Some("fixture-related-evidence-summary-first") => {
            let packet =
                seeded_related_evidence_offline_handoff_controls_related_evidence_summary_first();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-packet-publish-failed") => {
            let packet =
                seeded_related_evidence_offline_handoff_controls_offline_packet_publish_failed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_related_evidence_offline_handoff_controls(),
                seeded_related_evidence_offline_handoff_controls_related_evidence_summary_first(),
                seeded_related_evidence_offline_handoff_controls_offline_packet_publish_failed(),
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

fn assert_valid(packet: &EvidenceHandoffControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "related evidence offline handoff controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
