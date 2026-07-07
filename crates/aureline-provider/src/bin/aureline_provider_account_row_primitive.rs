//! Headless emitter for the M5 provider-account-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-provider-account-row-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-provider-account-row-primitive/`. The account-settings panel, provider
//! status bar, connection picker, headless/CLI accounts surface, and support account
//! export consumers read this matrix so one provider-account row names its provider
//! identity, connection state, tenant/org scope, effective write scope, and token/session
//! freshness with a derived access capability that lets a user tell whether Aureline can
//! read live, write, or only inspect a cached read — and offers sign-in / retry / remove
//! without forcing blind credential re-entry.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- fixture-connection-picker-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- fixture-headless-cli-accounts-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- validate
//! ```

use aureline_provider::implement_provider_account_rows_with_signed_in_limited_scope_stale_session_offline_cached_policy_blocked_truth_and_sign_in_retry_remove_parity_across_claimed_m5_provider_surfaces::{
    seeded_m5_provider_account_row_connection_picker_preview_narrowed,
    seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed,
    seeded_m5_provider_account_row_packet, M5ProviderAccountRowPacket,
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
            let packet = seeded_m5_provider_account_row_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_provider_account_row_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_provider_account_row_packet().render_matrix_csv()
            );
        }
        Some("fixture-connection-picker-preview-narrowed") => {
            let packet = seeded_m5_provider_account_row_connection_picker_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-accounts-beta-narrowed") => {
            let packet = seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_provider_account_row_packet(),
                seeded_m5_provider_account_row_connection_picker_preview_narrowed(),
                seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed(),
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

fn assert_valid(packet: &M5ProviderAccountRowPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "account row primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
