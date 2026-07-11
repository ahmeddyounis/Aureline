//! Headless emitter for the M5 compatibility-label-strip / publisher-continuity-row controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/`, its
//! matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-compatibility-label-strip-publisher-continuity-row-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_compatibility_publisher_continuity_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_compatibility_publisher_continuity_controls -- report
//! cargo run -p aureline-shell --example dump_m5_compatibility_publisher_continuity_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_compatibility_publisher_continuity_controls -- fixture-marketplace-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_compatibility_publisher_continuity_controls -- fixture-install-review-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_compatibility_publisher_continuity_controls -- validate
//! ```

use aureline_shell::implement_the_m5_compatibility_label_strip_and_publisher_continuity_row_host_version_range_manifest_schema_lifecycle_replacement_path_transfer_history_and_no_stale_certified_overclaim_primitive::{
    seeded_m5_compatibility_continuity_controls,
    seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed,
    seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed,
    M5CompatibilityContinuityControlsPacket,
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
            let packet = seeded_m5_compatibility_continuity_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_compatibility_continuity_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_compatibility_continuity_controls().render_matrix_csv()
            );
        }
        Some("fixture-marketplace-ui-beta-narrowed") => {
            let packet = seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-install-review-preview-narrowed") => {
            let packet = seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_compatibility_continuity_controls(),
                seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed(),
                seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed(),
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
    packet: &M5CompatibilityContinuityControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
