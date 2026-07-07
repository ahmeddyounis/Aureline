//! Headless emitter for the M5 provider-account / offline-capture component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-provider-account-offline-capture-component-consumer-proof/`, its
//! matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ui/m5-provider-account-offline-capture-component-consumers/`. Work-item detail,
//! status-transition review, issue intake, Help / docs, the support / export desk, and the
//! browser-handoff flow read this matrix so account state, destination mapping, queued-draft
//! state, and redaction posture stay one truth, and cached or offline-captured state never
//! masquerades as provider-committed state.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_provider_component_consumers -- support-export
//! cargo run -q -p aureline-provider --bin aureline_provider_component_consumers -- report
//! cargo run -q -p aureline-provider --bin aureline_provider_component_consumers -- csv
//! cargo run -q -p aureline-provider --bin aureline_provider_component_consumers -- fixture-browser-handoff-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_component_consumers -- fixture-issue-intake-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_component_consumers -- validate
//! ```

use aureline_provider::add_shared_work_item_status_transition_help_support_and_export_consumers_so_provider_account_and_offline_capture_components_keep_account_mapping_sync_and_redaction_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed,
    seeded_m5_provider_component_consumer_issue_intake_preview_narrowed,
    seeded_m5_provider_component_consumer_packet, M5ProviderComponentConsumerPacket,
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
            let packet = seeded_m5_provider_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_provider_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_provider_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-browser-handoff-beta-narrowed") => {
            let packet = seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-issue-intake-preview-narrowed") => {
            let packet = seeded_m5_provider_component_consumer_issue_intake_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_provider_component_consumer_packet(),
                seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed(),
                seeded_m5_provider_component_consumer_issue_intake_preview_narrowed(),
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
    packet: &M5ProviderComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "provider component consumer lane failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
