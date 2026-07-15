//! Headless emitter for the M5 verified-input-manifest and sidecar-completeness-manifest registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-verified-input-manifest-and-sidecar-completeness-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-verified-input-manifest-and-sidecar-completeness-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- report
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- verified-input-manifest-table
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- fixture-verified-input-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- fixture-sidecar-completeness-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_verified_input_manifest_and_sidecar_completeness_registries -- validate
//! ```

use aureline_ui::m5_verified_input_manifest_and_sidecar_completeness_registries::{
    seeded_m5_verified_input_manifest_and_sidecar_completeness_registries,
    seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_sidecar_completeness_preview_narrowed,
    seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_verified_input_beta_narrowed,
    M5VerifiedInputSidecarCompletenessRegistriesPacket,
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
            let packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries()
                    .render_matrix_csv()
            );
        }
        Some("verified-input-manifest-table") => {
            print!(
                "{}",
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries()
                    .render_verified_input_manifest_table()
            );
        }
        Some("fixture-verified-input-beta-narrowed") => {
            let packet =
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_verified_input_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-sidecar-completeness-preview-narrowed") => {
            let packet =
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_sidecar_completeness_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries(),
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_verified_input_beta_narrowed(),
                seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_sidecar_completeness_preview_narrowed(),
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
    packet: &M5VerifiedInputSidecarCompletenessRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
