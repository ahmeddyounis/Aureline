//! Headless emitter for the M5 toolchain-pin-row / precedence-inspector /
//! switch-review-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-toolchain-pin-switch-review-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-toolchain-pin-switch-review-primitive.md`,
//! and the narrowed fixtures under
//! `fixtures/ui/m5-toolchain-pin-switch-review-primitive/`. Every M5 environment
//! selector (the status-bar selector, the command-palette switcher, the settings
//! toolchain row, the interpreter picker, the SDK selector, the shell-profile picker,
//! the kernel picker, the runtime-target switcher, and the repair-panel selector)
//! reads this primitive so the target kind, the current selection, the winning scope
//! and source, the pin state, the shadowed layers, and the predicted switch blast
//! radius stay consistent, and so the support export reconstructs toolchain resolution
//! from one shared model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_toolchain_pin_switch_review_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_toolchain_pin_switch_review_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_toolchain_pin_switch_review_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_toolchain_pin_switch_review_primitive -- fixture-repair-panel-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_toolchain_pin_switch_review_primitive -- fixture-runtime-target-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_toolchain_pin_switch_review_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_toolchain_pin_row_precedence_inspector_and_switch_review_card_winning_scope_shadowed_layer_and_revert_or_repair_primitive::{
    seeded_m5_toolchain_pin_switch_review_primitive_packet,
    seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed,
    seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed,
    M5ToolchainPinSwitchReviewPrimitivePacket,
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
            let packet = seeded_m5_toolchain_pin_switch_review_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_toolchain_pin_switch_review_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_toolchain_pin_switch_review_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-repair-panel-beta-narrowed") => {
            let packet =
                seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-runtime-target-preview-narrowed") => {
            let packet =
                seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_toolchain_pin_switch_review_primitive_packet(),
                seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed(),
                seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed(),
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
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
