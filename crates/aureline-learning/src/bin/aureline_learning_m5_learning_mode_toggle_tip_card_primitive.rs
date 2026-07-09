//! Headless emitter for the M5 learning-mode-toggle / tip-card controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-learning-mode-toggle-tip-card-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-learning-mode-toggle-tip-card-controls/`. The learning-mode panel and the
//! onboarding / tip surfaces read these controls so one learning-mode toggle names whether
//! learning is active, what scope it changes, and how to pause / snooze / reset it, and one
//! tip card names why it is relevant now and which stable command / file / docs deep link
//! backs the next step — never through an ephemeral coachmark or hidden routing.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- fixture-learning-mode-toggle-paused
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- fixture-tip-card-withheld
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- validate
//! ```

use aureline_learning::implement_learning_mode_toggles_and_tip_cards_with_user_workspace_scope_pause_snooze_reset_why_now_context_and_stable_command_file_docs_deep_link_truth_across_claimed_m5_onboarding_and_help_surfaces::{
    seeded_learning_mode_toggle_tip_card_controls,
    seeded_learning_mode_toggle_tip_card_controls_learning_mode_toggle_paused,
    seeded_learning_mode_toggle_tip_card_controls_tip_card_withheld,
    LearningModeToggleTipCardControlsPacket,
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
            let packet = seeded_learning_mode_toggle_tip_card_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_learning_mode_toggle_tip_card_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_learning_mode_toggle_tip_card_controls().render_matrix_csv()
            );
        }
        Some("fixture-learning-mode-toggle-paused") => {
            let packet =
                seeded_learning_mode_toggle_tip_card_controls_learning_mode_toggle_paused();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-tip-card-withheld") => {
            let packet = seeded_learning_mode_toggle_tip_card_controls_tip_card_withheld();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_learning_mode_toggle_tip_card_controls(),
                seeded_learning_mode_toggle_tip_card_controls_learning_mode_toggle_paused(),
                seeded_learning_mode_toggle_tip_card_controls_tip_card_withheld(),
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
    packet: &LearningModeToggleTipCardControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "learning mode toggle tip card primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
