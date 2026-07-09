//! Headless emitter for the M5 credential component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-credential-component-consumer-proof/`, its matrix CSV, the Markdown
//! report, and the narrowed fixtures under `fixtures/ui/m5-credential-component-consumers/`.
//! Credential settings, the request auth surface, database attach, registry/provider auth,
//! release publish, remote attach, the AI model provider, Help / docs, the support / export
//! desk, and the export packet read this matrix so storage mode, credential class, handle-only
//! reveal posture, forwarded/delegated identity, expiry, and raw-secret-excluded export safety
//! stay one truth, and an expired/revoked, forwarded/delegated, or session-only credential never
//! masquerades as a usable, locally stored one.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_credential_component_consumers -- support-export
//! cargo run -q -p aureline-provider --bin aureline_credential_component_consumers -- report
//! cargo run -q -p aureline-provider --bin aureline_credential_component_consumers -- csv
//! cargo run -q -p aureline-provider --bin aureline_credential_component_consumers -- fixture-registry-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_credential_component_consumers -- fixture-database-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_credential_component_consumers -- validate
//! ```

use aureline_provider::add_shared_settings_request_database_registry_release_remote_ai_help_support_and_export_consumers_so_credential_components_keep_storage_scope_expiry_and_export_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_credential_component_consumer_database_preview_narrowed,
    seeded_m5_credential_component_consumer_packet,
    seeded_m5_credential_component_consumer_registry_beta_narrowed,
    M5CredentialComponentConsumerPacket,
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
            let packet = seeded_m5_credential_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_credential_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_credential_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-registry-beta-narrowed") => {
            let packet = seeded_m5_credential_component_consumer_registry_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-database-preview-narrowed") => {
            let packet = seeded_m5_credential_component_consumer_database_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_credential_component_consumer_packet(),
                seeded_m5_credential_component_consumer_registry_beta_narrowed(),
                seeded_m5_credential_component_consumer_database_preview_narrowed(),
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
    packet: &M5CredentialComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "credential component consumer lane failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
