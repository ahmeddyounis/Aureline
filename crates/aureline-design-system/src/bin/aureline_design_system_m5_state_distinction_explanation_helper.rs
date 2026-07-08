//! Headless emitter for the M5 state-distinction explanation helper primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-state-distinction-explanation-helper-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-state-distinction-explanation-helper-primitive/`. The onboarding / help surface,
//! the blocked-action row, the settings row, the activity row, and the workspace-entry surface read
//! this matrix so one explanation helper keeps `current` distinct from `selected`, `read-only`
//! distinct from `disabled`, `locked` distinct from `disabled`, and `pending` distinct from
//! `loading`, teaches each distinction in place, invents no one-off language, and keeps
//! contextual-teaching and blocked-action help aligned with the same component-state truth — on
//! every claimed consumer surface.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_state_distinction_explanation_helper -- support-export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_state_distinction_explanation_helper -- report
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_state_distinction_explanation_helper -- csv
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_state_distinction_explanation_helper -- fixture-blocked-action-beta-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_state_distinction_explanation_helper -- fixture-workspace-entry-preview-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_state_distinction_explanation_helper -- validate
//! ```

use aureline_design_system::implement_current_vs_selected_read_only_vs_disabled_locked_vs_disabled_and_pending_vs_loading_state_explanation_helpers_across_claimed_m5_onboarding_blocked_action_settings_activity_and_workspace_surfaces::{
    seeded_m5_state_explanation_blocked_action_beta_narrowed, seeded_m5_state_explanation_packet,
    seeded_m5_state_explanation_workspace_entry_preview_narrowed, M5StateExplanationPacket,
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
            let packet = seeded_m5_state_explanation_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_state_explanation_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_state_explanation_packet().render_matrix_csv()
            );
        }
        Some("fixture-blocked-action-beta-narrowed") => {
            let packet = seeded_m5_state_explanation_blocked_action_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-workspace-entry-preview-narrowed") => {
            let packet = seeded_m5_state_explanation_workspace_entry_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_state_explanation_packet(),
                seeded_m5_state_explanation_blocked_action_beta_narrowed(),
                seeded_m5_state_explanation_workspace_entry_preview_narrowed(),
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

fn assert_valid(packet: &M5StateExplanationPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "state distinction explanation primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
