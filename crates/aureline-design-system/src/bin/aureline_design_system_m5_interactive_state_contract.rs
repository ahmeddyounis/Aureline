//! Headless emitter for the M5 interactive-state-contract primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-interactive-state-contract-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-interactive-state-contract-primitive/`. The push button, icon button, menu item,
//! pane splitter, and quick-action card read this matrix so one interactive-state contract names
//! the derived presentation posture, the non-color cues that carry the state beyond hue, and the
//! interaction routes it is reachable through — with a stable hit target, no interaction-breaking
//! layout shift, and a keyboard-visible focus ring on every claimed control.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_interactive_state_contract -- support-export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_interactive_state_contract -- report
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_interactive_state_contract -- csv
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_interactive_state_contract -- fixture-pane-splitter-beta-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_interactive_state_contract -- fixture-quick-action-card-preview-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_interactive_state_contract -- validate
//! ```

use aureline_design_system::implement_default_hover_focus_visible_pressed_state_contracts_with_no_color_only_and_no_layout_shift_rules_across_claimed_m5_controls_and_pane_affordances::{
    seeded_m5_interactive_state_contract_packet,
    seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed,
    seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed,
    M5InteractiveStateContractPacket,
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
            let packet = seeded_m5_interactive_state_contract_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_interactive_state_contract_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_interactive_state_contract_packet().render_matrix_csv()
            );
        }
        Some("fixture-pane-splitter-beta-narrowed") => {
            let packet = seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-quick-action-card-preview-narrowed") => {
            let packet = seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_interactive_state_contract_packet(),
                seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed(),
                seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed(),
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
    packet: &M5InteractiveStateContractPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "interactive state contract primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
