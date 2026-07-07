//! Headless emitter for the M5 support-intake / escalation component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-support-intake-escalation-component-consumer-proof/`, its matrix
//! CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ui/m5-support-intake-escalation-component-consumers/`. Project Doctor
//! results, the safe-mode recovery flow, the extension-bisect recovery flow, the support
//! center, Help / docs, and the CLI / headless export desk all read this adoption lane so
//! one shared set of support-intake and escalation components keeps scenario-code,
//! packet-id, redaction-class, and approved-repair language aligned — never a parallel
//! local variant.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_consumers -- support-export
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_consumers -- report
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_consumers -- csv
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_consumers -- fixture-bisect-preview-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_consumers -- fixture-docs-help-beta-narrowed
//! cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_consumers -- validate
//! ```

use aureline_support::add_shared_doctor_safe_mode_bisect_support_center_docs_help_and_export_consumers_so_support_intake_components_keep_scenario_code_repair_lineage_and_redaction_parity_across_claimed_m5_profiles::{
    seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed,
    seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed,
    seeded_m5_support_intake_escalation_component_consumer_packet,
    M5SupportIntakeComponentConsumerPacket,
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
            let packet = seeded_m5_support_intake_escalation_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_support_intake_escalation_component_consumer_packet()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_support_intake_escalation_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-bisect-preview-narrowed") => {
            let packet =
                seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-docs-help-beta-narrowed") => {
            let packet =
                seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_support_intake_escalation_component_consumer_packet(),
                seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed(),
                seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed(),
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
    packet: &M5SupportIntakeComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("consumer lane failed validation: {}", tokens.join(",")).into())
    }
}
