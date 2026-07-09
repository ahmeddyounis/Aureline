//! Headless emitter for the M5 secret-access-prompt / store-capability controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-secret-access-prompt-store-capability-proof/`, its
//! matrix CSV, the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-secret-access-prompt-store-capability-controls/`. Credential
//! settings, secret-prompt, vault-picker, support, and CLI surfaces read these
//! controls so one secret-access prompt names its actor, purpose, requested scope,
//! and raw-secret-versus-handle-only posture with a derived handle-availability class
//! that surfaces a handle-only path wherever one exists — never nudging a user toward
//! raw-secret sprawl — and one credential-store-capability row names its store type,
//! verification state, portability / export posture, and platform limitations with a
//! derived trust class that never lets an unverified or unsupported store read as
//! "securely stored".
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- fixture-secret-access-prompt-raw-reveal
//! cargo run -q -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- fixture-store-capability-unverified
//! cargo run -q -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- validate
//! ```

use aureline_provider::implement_secret_access_prompt_sheets_and_credential_store_capability_rows_with_actor_scope_handle_only_and_session_fallback_truth::{
    seeded_secret_access_prompt_store_capability_controls,
    seeded_secret_access_prompt_store_capability_controls_secret_access_prompt_raw_reveal,
    seeded_secret_access_prompt_store_capability_controls_store_capability_unverified,
    SecretAccessPromptStoreCapabilityControlsPacket,
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
            let packet = seeded_secret_access_prompt_store_capability_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_secret_access_prompt_store_capability_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_secret_access_prompt_store_capability_controls().render_matrix_csv()
            );
        }
        Some("fixture-secret-access-prompt-raw-reveal") => {
            let packet =
                seeded_secret_access_prompt_store_capability_controls_secret_access_prompt_raw_reveal();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-store-capability-unverified") => {
            let packet =
                seeded_secret_access_prompt_store_capability_controls_store_capability_unverified();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_secret_access_prompt_store_capability_controls(),
                seeded_secret_access_prompt_store_capability_controls_secret_access_prompt_raw_reveal(),
                seeded_secret_access_prompt_store_capability_controls_store_capability_unverified(),
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
    packet: &SecretAccessPromptStoreCapabilityControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "secret access prompt store capability controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
