//! Headless emitter for the frozen M5 marketplace-install component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-marketplace-install-proof/`, its matrix CSV, the Markdown design report,
//! and the narrowed fixtures under `fixtures/ui/m5-marketplace-install-components/`. Marketplace,
//! extension-manager, registry-admin, install-review, and help surfaces read this matrix so one
//! marketplace result row names source class and compatibility, one detail fact grid names every
//! marketplace fact together, one compatibility-label strip names range and host model, one
//! permission-manifest summary names posture and transitive widening, one activation-budget band
//! names the budget band, one install/update/disable/rollback review sheet names disable scope and
//! rollback compatibility, one publisher-continuity row names transfer and deprecation, and one
//! installed-state diagnostics card names quarantine history and health.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_marketplace_install_component_matrix -- support-export
//! cargo run -p aureline-shell --example dump_m5_marketplace_install_component_matrix -- report
//! cargo run -p aureline-shell --example dump_m5_marketplace_install_component_matrix -- csv
//! cargo run -p aureline-shell --example dump_m5_marketplace_install_component_matrix -- fixture-compatibility-label-strip-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_marketplace_install_component_matrix -- fixture-install-review-sheet-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_marketplace_install_component_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    seeded_m5_marketplace_install_component_matrix,
    seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed,
    seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed,
    M5MarketplaceInstallComponentMatrixPacket,
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
            let packet = seeded_m5_marketplace_install_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_marketplace_install_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_marketplace_install_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-compatibility-label-strip-beta-narrowed") => {
            let packet =
                seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-install-review-sheet-preview-narrowed") => {
            let packet =
                seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_marketplace_install_component_matrix(),
                seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed(),
                seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed(),
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
    packet: &M5MarketplaceInstallComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
