//! Headless emitter for the M5 contextual-tip-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-contextual-tip-card-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-contextual-tip-card-primitive/`. The first-run onboarding panel, guided-tour
//! overlay, command-palette hint, inline editor tip, and support tip export consumers read
//! this matrix so one contextual tip card names why it is relevant now, the concrete next
//! action, the stable command that backs it, and whether it is delivered, snoozed, or withheld
//! for quiet hours / presentation mode / a recent dismissal — teaching in place, staying
//! reversible, and never bypassing the trust limits of the action it teaches.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- fixture-command-palette-hint-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- fixture-support-tip-export-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- validate
//! ```

use aureline_learning::implement_contextual_tip_cards_with_why_now_relevance_concrete_next_action_stable_command_reference_and_try_open_docs_snooze_dismiss_actions_that_respect_quiet_hours_presentation_mode_and_recent_dismissals_across_claimed_m5_learnability_surfaces::{
    seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed,
    seeded_m5_contextual_tip_card_packet,
    seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed, M5ContextualTipCardPacket,
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
            let packet = seeded_m5_contextual_tip_card_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_contextual_tip_card_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_contextual_tip_card_packet().render_matrix_csv()
            );
        }
        Some("fixture-command-palette-hint-beta-narrowed") => {
            let packet = seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-tip-export-preview-narrowed") => {
            let packet = seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_contextual_tip_card_packet(),
                seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed(),
                seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed(),
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

fn assert_valid(packet: &M5ContextualTipCardPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "contextual tip card primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
