//! Headless emitter for the M5 prompt-composer-header / context-attachment-pill
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/`.
//! The inline assistant, the side panel, the patch draft, the handoff surface, and the
//! CLI / support export all read this primitive so one composer header names its mode,
//! scope, route/provider/model, and budget band, and one attachment pill names its exact
//! object identity, kind, trust state, freshness, and bounded open/remove actions.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- support-export
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- report
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- csv
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- fixture-patch-draft-preview-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- fixture-handoff-beta-narrowed
//! cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- validate
//! ```

use aureline_ai::implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces::{
    seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed,
    seeded_m5_prompt_composer_header_pill_packet,
    seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed,
    M5PromptComposerHeaderPillPacket,
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
            let packet = seeded_m5_prompt_composer_header_pill_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_prompt_composer_header_pill_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_prompt_composer_header_pill_packet().render_matrix_csv()
            );
        }
        Some("fixture-patch-draft-preview-narrowed") => {
            let packet = seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-handoff-beta-narrowed") => {
            let packet = seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_prompt_composer_header_pill_packet(),
                seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed(),
                seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed(),
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
    packet: &M5PromptComposerHeaderPillPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
