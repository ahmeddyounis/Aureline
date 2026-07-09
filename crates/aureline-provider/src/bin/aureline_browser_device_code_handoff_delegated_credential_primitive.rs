//! Headless emitter for the M5 browser-handoff / delegated-credential controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-browser-device-code-handoff-delegated-credential-proof/`,
//! its matrix CSV, the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-browser-device-code-handoff-delegated-credential-controls/`. Sign-in,
//! provider, remote-attach, support, and CLI surfaces read these controls so one
//! browser-or-device-code handoff card names its provider / org, its auth-handoff flow
//! kind, its fallback state, its local continuity, its device code / expiry, and why a
//! safer boundary is preferred with a derived handoff-boundary class that never blurs
//! system-browser / device-code / local capture into one generic sign-in state, and one
//! delegated-credential row names its source identity, target scope, storage class,
//! expiration, and policy owner with a derived identity origin that never lets a
//! forwarded, remote-vault-held, or service-issued identity read as locally stored.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- fixture-handoff-local-capture
//! cargo run -q -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- fixture-delegated-forwarded-identity
//! cargo run -q -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- validate
//! ```

use aureline_provider::implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows_with_handoff_boundary_and_delegated_identity_origin_truth::{
    seeded_browser_handoff_delegated_credential_controls,
    seeded_browser_handoff_delegated_credential_controls_delegated_forwarded_identity,
    seeded_browser_handoff_delegated_credential_controls_handoff_local_capture,
    BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket,
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
            let packet = seeded_browser_handoff_delegated_credential_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_browser_handoff_delegated_credential_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_browser_handoff_delegated_credential_controls().render_matrix_csv()
            );
        }
        Some("fixture-handoff-local-capture") => {
            let packet =
                seeded_browser_handoff_delegated_credential_controls_handoff_local_capture();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-delegated-forwarded-identity") => {
            let packet =
                seeded_browser_handoff_delegated_credential_controls_delegated_forwarded_identity();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_browser_handoff_delegated_credential_controls(),
                seeded_browser_handoff_delegated_credential_controls_handoff_local_capture(),
                seeded_browser_handoff_delegated_credential_controls_delegated_forwarded_identity(),
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
    packet: &BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "browser handoff delegated credential controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
