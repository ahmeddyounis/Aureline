//! Headless emitter for the M5 restricted-capability-row / narrowed-capability-summary controls
//! packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-restricted-capability-row-narrowed-capability-summary-controls-proof/`, its
//! matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-restricted-capability-row-narrowed-capability-summary-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_restricted_capability_row_narrowed_capability_summary_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_restricted_capability_row_narrowed_capability_summary_controls -- report
//! cargo run -p aureline-shell --example dump_m5_restricted_capability_row_narrowed_capability_summary_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_restricted_capability_row_narrowed_capability_summary_controls -- fixture-workspace-trust-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_restricted_capability_row_narrowed_capability_summary_controls -- fixture-safe-mode-ui-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_restricted_capability_row_narrowed_capability_summary_controls -- validate
//! ```

use aureline_shell::implement_the_m5_restricted_capability_row_and_narrowed_capability_summary_blocked_action_families_still_safe_actions_restriction_reason_and_command_backed_recovery_primitive::{
    seeded_m5_restricted_capability_controls,
    seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed,
    seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed,
    M5RestrictedCapabilityControlsPacket,
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
            let packet = seeded_m5_restricted_capability_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_restricted_capability_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_restricted_capability_controls().render_matrix_csv()
            );
        }
        Some("fixture-workspace-trust-ui-beta-narrowed") => {
            let packet =
                seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-safe-mode-ui-preview-narrowed") => {
            let packet = seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_restricted_capability_controls(),
                seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed(),
                seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed(),
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
    packet: &M5RestrictedCapabilityControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
