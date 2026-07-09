//! Headless emitter for the M5 credential-state-row / vault-picker controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-credential-state-row-vault-picker-proof/`, its
//! matrix CSV, the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-credential-state-row-vault-picker-controls/`. Credential
//! settings, secret-prompt, vault-picker, support, and CLI surfaces read these
//! controls so one credential-state row names its storage mode, source class,
//! target boundary, and expiry/rotation/revoke lifecycle with a derived health
//! state that lets a user tell where authority lives and what boundary it applies
//! to without reading logs or provider docs, and one vault/keychain picker names
//! its available source, access scope, reveal policy, and a derived portability
//! note that never normalizes raw-secret handling.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- fixture-credential-state-row-revoked
//! cargo run -q -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- fixture-vault-picker-export-blocked
//! cargo run -q -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- validate
//! ```

use aureline_provider::implement_credential_state_rows_and_vault_or_keychain_pickers_with_source_target_boundary_expiry_portability_and_rotate_revoke_test_truth::{
    seeded_credential_state_row_vault_picker_controls,
    seeded_credential_state_row_vault_picker_controls_credential_state_row_revoked,
    seeded_credential_state_row_vault_picker_controls_vault_picker_export_blocked,
    CredentialStateRowVaultPickerControlsPacket,
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
            let packet = seeded_credential_state_row_vault_picker_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_credential_state_row_vault_picker_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_credential_state_row_vault_picker_controls().render_matrix_csv()
            );
        }
        Some("fixture-credential-state-row-revoked") => {
            let packet =
                seeded_credential_state_row_vault_picker_controls_credential_state_row_revoked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-vault-picker-export-blocked") => {
            let packet =
                seeded_credential_state_row_vault_picker_controls_vault_picker_export_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_credential_state_row_vault_picker_controls(),
                seeded_credential_state_row_vault_picker_controls_credential_state_row_revoked(),
                seeded_credential_state_row_vault_picker_controls_vault_picker_export_blocked(),
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
    packet: &CredentialStateRowVaultPickerControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "credential state row vault picker controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
