//! Headless emitter for the M5 provider offline-capture / privacy-redaction row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-provider-offline-capture-privacy-redaction-row-primitive-proof/`, its
//! matrix CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-provider-offline-capture-privacy-redaction-row-primitive/`. The
//! offline-capture panel, privacy/redaction panel, provider status bar, headless/CLI capture
//! surface, and support privacy export consumers read this matrix so one offline-capture row
//! names its packet destination, queued-draft count, redaction default, and publish-later
//! behavior with export/clear parity, and one privacy/redaction row states its copied/exported
//! fields, support-bundle treatment, telemetry limit, and policy source with a reviewed
//! escalation — never erasing prepared handoff state or leaking past the metadata-safe
//! boundary.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_provider_offline_capture_privacy_row_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_provider_offline_capture_privacy_row_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_provider_offline_capture_privacy_row_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_provider_offline_capture_privacy_row_primitive -- fixture-offline-capture-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_offline_capture_privacy_row_primitive -- fixture-privacy-redaction-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_offline_capture_privacy_row_primitive -- validate
//! ```

use aureline_provider::implement_offline_capture_rows_and_privacy_redaction_rows_with_packet_destination_queued_draft_count_export_clear_actions_and_metadata_safe_boundary_truth_across_claimed_m5_provider_workflows::{
    seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed,
    seeded_m5_provider_offline_privacy_row_packet,
    seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed,
    M5ProviderOfflinePrivacyRowPacket,
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
            let packet = seeded_m5_provider_offline_privacy_row_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_provider_offline_privacy_row_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_provider_offline_privacy_row_packet().render_matrix_csv()
            );
        }
        Some("fixture-offline-capture-beta-narrowed") => {
            let packet = seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-privacy-redaction-preview-narrowed") => {
            let packet =
                seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_provider_offline_privacy_row_packet(),
                seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed(),
                seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed(),
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
    packet: &M5ProviderOfflinePrivacyRowPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "offline/privacy row primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
