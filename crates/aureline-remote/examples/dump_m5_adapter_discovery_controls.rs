//! Headless emitter for the M5 adapter-confidence-chip / discovery-diff-card controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-adapter-confidence-chip-discovery-diff-card-controls-proof/`, its matrix
//! CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-adapter-confidence-chip-discovery-diff-card-controls/`.
//!
//! ```text
//! cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls -- support-export
//! cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls -- report
//! cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls -- csv
//! cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls -- fixture-run-test-debug-beta-narrowed
//! cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls -- fixture-preview-preview-narrowed
//! cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls -- validate
//! ```

use aureline_remote::implement_the_m5_adapter_confidence_chip_and_discovery_diff_card_adapter_source_class_confidence_band_discovery_mode_downgrade_reason_target_identity_drift_changed_certainty_review_before_switch_and_no_higher_confidence_overwrite_primitive::{
    seeded_m5_adapter_discovery_controls,
    seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed,
    seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed,
    M5AdapterDiscoveryControlsPacket,
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
            let packet = seeded_m5_adapter_discovery_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_adapter_discovery_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_adapter_discovery_controls().render_matrix_csv()
            );
        }
        Some("fixture-run-test-debug-beta-narrowed") => {
            let packet = seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-preview-preview-narrowed") => {
            let packet = seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_adapter_discovery_controls(),
                seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed(),
                seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed(),
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
    packet: &M5AdapterDiscoveryControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
