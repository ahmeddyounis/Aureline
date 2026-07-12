//! Headless emitter for the M5 activation-budget-band / installed-state-diagnostics-card controls
//! packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/`,
//! its matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_activation_budget_diagnostics_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_activation_budget_diagnostics_controls -- report
//! cargo run -p aureline-shell --example dump_m5_activation_budget_diagnostics_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_activation_budget_diagnostics_controls -- fixture-marketplace-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_activation_budget_diagnostics_controls -- fixture-install-review-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_activation_budget_diagnostics_controls -- validate
//! ```

use aureline_shell::implement_the_m5_activation_budget_band_and_installed_state_diagnostics_card_cold_warm_activation_buckets_trigger_classes_throttling_quarantine_reasons_and_disable_retry_parity_primitive::{
    seeded_m5_activation_diagnostics_controls,
    seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed,
    seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed,
    M5ActivationDiagnosticsControlsPacket,
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
            let packet = seeded_m5_activation_diagnostics_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_activation_diagnostics_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_activation_diagnostics_controls().render_matrix_csv()
            );
        }
        Some("fixture-marketplace-ui-beta-narrowed") => {
            let packet = seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-install-review-preview-narrowed") => {
            let packet =
                seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_activation_diagnostics_controls(),
                seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed(),
                seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed(),
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
    packet: &M5ActivationDiagnosticsControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
