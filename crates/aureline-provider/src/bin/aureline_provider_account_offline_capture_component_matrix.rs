//! Headless emitter for the frozen M5 provider-account / offline-capture component
//! matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-provider-account-offline-capture-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-provider-account-offline-capture-components/`. Issue, review,
//! incident, support, provider-settings, and CLI provider surfaces read this matrix so
//! one provider-account row names its connection state and tenant scope, one project/
//! board mapping row names where a publish will land and how that mapping was derived,
//! one sync-behavior row names its sync mode and effective write scope, one
//! offline-capture row names what remains queued locally, and one privacy-redaction row
//! names what support and export will reveal.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- support-export
//! cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- report
//! cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- csv
//! cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- fixture-offline-capture-row-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- fixture-privacy-redaction-row-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- validate
//! ```

use aureline_provider::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    seeded_m5_provider_account_offline_capture_component_matrix,
    seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed,
    seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed,
    M5ProviderAccountOfflineComponentMatrixPacket,
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
            let packet = seeded_m5_provider_account_offline_capture_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_provider_account_offline_capture_component_matrix()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_provider_account_offline_capture_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-offline-capture-row-beta-narrowed") => {
            let packet =
                seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-privacy-redaction-row-preview-narrowed") => {
            let packet =
                seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_provider_account_offline_capture_component_matrix(),
                seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed(),
                seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed(),
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
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
