//! Headless emitter for the frozen M5 prompt-composer-component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/`.
//! Inline, panel, patch-review, branch-agent, help, and companion composer surfaces
//! read this matrix so one composer header names its mode, scope, and route, one
//! attachment pill names its object identity and trust, one mention resolver names
//! its resolution state, one slash-command row names its availability and gate, one
//! budget strip names its budget posture and omitted context, one tainted-context
//! warning names its source and severity, one draft-state row names its locality
//! and retention, one attachment-stale banner names its staleness reason, and one
//! send-review control names its send posture and review requirement.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- fixture-tainted-context-warning-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- fixture-send-review-control-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- validate
//! ```

use aureline_ai::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    seeded_m5_prompt_composer_component_matrix,
    seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed,
    seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed,
    M5PromptComposerComponentMatrixPacket,
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
            let packet = seeded_m5_prompt_composer_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_prompt_composer_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_prompt_composer_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-tainted-context-warning-beta-narrowed") => {
            let packet =
                seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-send-review-control-preview-narrowed") => {
            let packet =
                seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_prompt_composer_component_matrix(),
                seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed(),
                seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed(),
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
    packet: &M5PromptComposerComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
