//! Headless emitter for the M5 mention-resolver / slash-command-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/`.
//! The inline composer, the command palette, the automation recipe, the CLI / headless
//! surface, and the support export all read this primitive so one mention row names the
//! exact stable object an `@`-mention binds to and surfaces ambiguity before send, and one
//! slash-command row names its stable command id, capability class, help path, availability
//! posture, disabled-state explanation, and approval semantics.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_mention_resolver_slash_command_row_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_mention_resolver_slash_command_row_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_mention_resolver_slash_command_row_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_mention_resolver_slash_command_row_primitive -- fixture-automation-recipe-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_mention_resolver_slash_command_row_primitive -- fixture-cli-headless-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_mention_resolver_slash_command_row_primitive -- validate
//! ```

use aureline_ai::ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces::{
    seeded_m5_mention_slash_command_cli_headless_beta_narrowed,
    seeded_m5_mention_slash_command_automation_recipe_preview_narrowed,
    seeded_m5_mention_slash_command_packet, M5MentionSlashCommandPacket,
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
            let packet = seeded_m5_mention_slash_command_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_mention_slash_command_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_mention_slash_command_packet().render_matrix_csv()
            );
        }
        Some("fixture-automation-recipe-preview-narrowed") => {
            let packet = seeded_m5_mention_slash_command_automation_recipe_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-cli-headless-beta-narrowed") => {
            let packet = seeded_m5_mention_slash_command_cli_headless_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_mention_slash_command_packet(),
                seeded_m5_mention_slash_command_automation_recipe_preview_narrowed(),
                seeded_m5_mention_slash_command_cli_headless_beta_narrowed(),
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

fn assert_valid(packet: &M5MentionSlashCommandPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
