//! Headless emitter for the M5 line-bundle_refresh_audit and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- report
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- bundle-refresh-audit-table
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- fixture-bundle-refresh-audit-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- fixture-shipping-line-drift-report-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries -- validate
//! ```

use aureline_ui::m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries::{
    seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries,
    seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_bundle_refresh_audit_beta_narrowed,
    seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_shipping_line_drift_report_preview_narrowed,
    M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket,
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
            let packet =
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries()
                    .render_matrix_csv()
            );
        }
        Some("bundle-refresh-audit-table") => {
            print!(
                "{}",
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries()
                    .render_bundle_refresh_audit_table()
            );
        }
        Some("fixture-bundle-refresh-audit-beta-narrowed") => {
            let packet =
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_bundle_refresh_audit_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-shipping-line-drift-report-preview-narrowed") => {
            let packet =
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_shipping_line_drift_report_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries(),
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_bundle_refresh_audit_beta_narrowed(),
                seeded_m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries_shipping_line_drift_report_preview_narrowed(),
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
    packet: &M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
