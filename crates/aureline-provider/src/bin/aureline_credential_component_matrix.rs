//! Headless emitter for the frozen M5 credential component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-credential-component-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-credential-components/`.
//! Sign-in, provider, registry, request, remote, package, release, help, and support
//! surfaces read this matrix so one credential-state row names where a secret is stored and
//! whether a handle-only path exists, one secret-access-prompt sheet names its reveal
//! posture and auth-handoff class, one vault/keychain picker names the store it will write
//! to, one credential-store-capability row names what the store can do, one browser/device-
//! code handoff card names the handoff in flight, one delegated-credential row names which
//! identity is forwarded or delegated, one rotation/revoke-event row names what rotation or
//! revoke will impact, and one export-safety banner names what an export will and will not
//! reveal.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- support-export
//! cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- report
//! cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- csv
//! cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- fixture-browser-device-code-handoff-card-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- fixture-export-safety-banner-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- validate
//! ```

use aureline_provider::freeze_the_m5_credential_component_matrix::{
    seeded_m5_credential_component_matrix,
    seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed,
    seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed,
    M5CredentialComponentMatrixPacket,
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
            let packet = seeded_m5_credential_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_credential_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_credential_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-browser-device-code-handoff-card-beta-narrowed") => {
            let packet =
                seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-export-safety-banner-preview-narrowed") => {
            let packet = seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_credential_component_matrix(),
                seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed(),
                seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed(),
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
    packet: &M5CredentialComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
