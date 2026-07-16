//! Headless emitter for the frozen M5 retired-state matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-retirements/`, its matrix CSV, the Markdown design report at
//! `artifacts/program/m5-retired-state-matrix.md`, the retired-surface-health dashboard at
//! `dashboards/m5-retired-surface-health.json`, and the narrowed fixtures under
//! `fixtures/release/m5-retired-state/`. The release, help, docs, support, marketplace, install/update, and
//! partner/procurement surfaces read this matrix so no retired surface disappears without a tombstone,
//! successor pointer, or archival route, no retired class stays selectable in a new-install / new-tenant /
//! marketplace / upgrade flow, and last-supported docs / schemas / evidence survive until support-note
//! closure and export-safe archive handoff are complete.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- fixture-registry-visible-package-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- fixture-managed-tenant-feature-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_retired_state_matrix -- validate
//! ```

use aureline_ui::m5_retired_state_matrix::{
    seeded_m5_retired_state_matrix,
    seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed,
    seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed,
    M5RetiredStateMatrixPacket,
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
            let packet = seeded_m5_retired_state_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_retired_state_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_retired_state_matrix().render_matrix_csv());
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_retired_state_matrix().render_dashboard_json()
            );
        }
        Some("fixture-registry-visible-package-beta-narrowed") => {
            let packet = seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-managed-tenant-feature-preview-narrowed") => {
            let packet = seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_retired_state_matrix(),
                seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed(),
                seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed(),
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

fn assert_valid(packet: &M5RetiredStateMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
