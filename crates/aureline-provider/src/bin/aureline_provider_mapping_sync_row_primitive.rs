//! Headless emitter for the M5 provider mapping / sync-behavior row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-provider-mapping-sync-behavior-row-primitive-proof/`, its matrix
//! CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-provider-mapping-sync-behavior-row-primitive/`. The mapping-picker panel,
//! sync-behavior panel, provider status bar, headless/CLI mappings surface, and support
//! mapping export consumers read this matrix so one project/board mapping row names its
//! destination, origin, inherited/local/policy scope, and lock note with change/reset
//! parity, and one sync-behavior row separates read-only-metadata, comment/link,
//! status-transition, and offline-capture-only behaviors with a visible local-draft queue —
//! never one ambiguous `synced` label.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_provider_mapping_sync_row_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_provider_mapping_sync_row_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_provider_mapping_sync_row_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_provider_mapping_sync_row_primitive -- fixture-sync-behavior-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_mapping_sync_row_primitive -- fixture-headless-cli-mappings-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_mapping_sync_row_primitive -- validate
//! ```

use aureline_provider::ship_project_or_board_mapping_rows_and_sync_behavior_rows_with_inherited_local_policy_scope_read_only_comment_transition_sync_modes_and_change_reset_parity_across_claimed_m5_provider_lanes::{
    seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed,
    seeded_m5_provider_mapping_sync_row_packet,
    seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed,
    M5ProviderMappingSyncRowPacket,
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
            let packet = seeded_m5_provider_mapping_sync_row_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_provider_mapping_sync_row_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_provider_mapping_sync_row_packet().render_matrix_csv()
            );
        }
        Some("fixture-sync-behavior-preview-narrowed") => {
            let packet = seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-mappings-beta-narrowed") => {
            let packet = seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_provider_mapping_sync_row_packet(),
                seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed(),
                seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed(),
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

fn assert_valid(packet: &M5ProviderMappingSyncRowPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "mapping/sync row primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
