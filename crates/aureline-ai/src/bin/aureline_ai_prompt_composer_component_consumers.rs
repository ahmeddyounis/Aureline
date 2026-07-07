//! Headless emitter for the M5 prompt-composer-component-consumer adoption lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/m5-prompt-composer-component-consumer-proof/`, its matrix CSV, the
//! Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/m5-prompt-composer-component-consumers/`. The inline / panel
//! composer, patch review, the branch-agent console, the docs/help surface, and the
//! companion composer all read this adoption matrix so the reusable prompt-composer
//! header, attachment pill, mention resolver, slash-command row, budget / size strip,
//! tainted-context warning, draft-state row, attachment-stale banner, and split-send
//! review control keep locality, route/provider/model, approval, and taint truth
//! aligned, and auto-narrow their claim language whenever the surrounding workflow
//! becomes review-only, handoff-only, offline / mirrored, or companion-scoped.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- fixture-branch-agent-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- fixture-companion-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- validate
//! ```

use aureline_ai::add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers::{
    seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed,
    seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed,
    seeded_m5_prompt_composer_component_consumer_packet, M5ComposerComponentConsumerPacket,
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
            let packet = seeded_m5_prompt_composer_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_prompt_composer_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_prompt_composer_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-branch-agent-beta-narrowed") => {
            let packet = seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-companion-preview-narrowed") => {
            let packet = seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_prompt_composer_component_consumer_packet(),
                seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed(),
                seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed(),
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
    packet: &M5ComposerComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("consumer matrix failed validation: {}", tokens.join(",")).into())
    }
}
