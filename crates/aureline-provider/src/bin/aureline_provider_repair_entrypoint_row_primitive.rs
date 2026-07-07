//! Headless emitter for the M5 provider-settings repair-entrypoint row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-provider-settings-repair-entrypoint-row-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-provider-settings-repair-entrypoint-row/`. The provider-account row, project/
//! board mapping row, sync-behavior row, privacy/redaction row, and provider status bar consumers
//! read this matrix so one repair row names the real boundary (network/egress, auth, mapping, or
//! compatibility), links to the concrete repair entrypoint and the diagnostics that explain the
//! failure, and preserves queued work, cached-read continuity, and the reviewed export path —
//! never forcing a blind credential re-entry.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- fixture-sync-behavior-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- fixture-privacy-redaction-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- validate
//! ```

use aureline_provider::ship_provider_settings_repair_entrypoints_and_linked_diagnostics_so_network_egress_auth_compatibility_boundaries_stay_explicit_across_claimed_m5_provider_surfaces::{
    seeded_m5_provider_repair_entrypoint_packet,
    seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed,
    seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed,
    M5ProviderRepairEntrypointPacket,
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
            let packet = seeded_m5_provider_repair_entrypoint_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_provider_repair_entrypoint_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_provider_repair_entrypoint_packet().render_matrix_csv()
            );
        }
        Some("fixture-sync-behavior-beta-narrowed") => {
            let packet = seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-privacy-redaction-preview-narrowed") => {
            let packet = seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_provider_repair_entrypoint_packet(),
                seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed(),
                seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed(),
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
    packet: &M5ProviderRepairEntrypointPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "repair entrypoint row primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
