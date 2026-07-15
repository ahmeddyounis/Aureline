//! Headless emitter for the M5 regression-asset and incident-close registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-regression-asset-and-incident-close-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-regression-asset-and-incident-close-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- report
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- regression-asset-table
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- fixture-regression-asset-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- fixture-incident-close-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- validate
//! ```

use aureline_ui::m5_regression_asset_and_incident_close_registries::{
    seeded_m5_regression_asset_and_incident_close_registries,
    seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed,
    seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed,
    M5RegressionAssetIncidentCloseRegistriesPacket,
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
            let packet = seeded_m5_regression_asset_and_incident_close_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_regression_asset_and_incident_close_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_regression_asset_and_incident_close_registries().render_matrix_csv()
            );
        }
        Some("regression-asset-table") => {
            print!(
                "{}",
                seeded_m5_regression_asset_and_incident_close_registries()
                    .render_regression_asset_table()
            );
        }
        Some("fixture-regression-asset-beta-narrowed") => {
            let packet =
                seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-incident-close-preview-narrowed") => {
            let packet =
                seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_regression_asset_and_incident_close_registries(),
                seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed(),
                seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed(),
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
    packet: &M5RegressionAssetIncidentCloseRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
