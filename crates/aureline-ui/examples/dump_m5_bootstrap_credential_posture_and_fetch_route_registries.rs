//! Headless emitter for the M5 credential-posture and fetch-route registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-bootstrap-credential-posture-and-fetch-route-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- report
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- credential-posture-table
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- fixture-air-gap-bundle-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- fixture-managed-snapshot-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_bootstrap_credential_posture_and_fetch_route_registries -- validate
//! ```

use aureline_ui::m5_bootstrap_credential_posture_and_fetch_route_registries::{
    seeded_m5_bootstrap_credential_posture_and_fetch_route_registries,
    seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed,
    seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed,
    M5CredentialPostureFetchRouteRegistriesPacket,
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
            let packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries()
                    .render_matrix_csv()
            );
        }
        Some("credential-posture-table") => {
            print!(
                "{}",
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries()
                    .render_credential_posture_table()
            );
        }
        Some("fixture-air-gap-bundle-beta-narrowed") => {
            let packet =
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-managed-snapshot-preview-narrowed") => {
            let packet =
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries(),
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed(),
                seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed(),
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
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
