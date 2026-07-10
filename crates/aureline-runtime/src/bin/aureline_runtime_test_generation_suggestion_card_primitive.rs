//! Headless emitter for the M5 test-generation-suggestion-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-test-generation-suggestion-card-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-test-generation-suggestion-card-primitive/`. The suggestion review panel, the
//! editor inline suggestion, the test-tree suggestion, the headless/CLI suggestion, and the
//! suggestion export consumers read this matrix so one test-generation suggestion card names its
//! trigger source, target symbols/files, uncovered-path/bug context, assumption summary, separated
//! review classes, and generated file count, and never offers an apply-capable action that
//! understates its churn or drops its diff-first preview and rollback.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_generation_suggestion_card_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_generation_suggestion_card_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_generation_suggestion_card_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_generation_suggestion_card_primitive -- fixture-suggestion-review-panel-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_generation_suggestion_card_primitive -- fixture-editor-suggestion-inline-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_generation_suggestion_card_primitive -- validate
//! ```

use aureline_runtime::implement_test_generation_suggestion_cards_with_uncovered_path_or_bug_trigger_truth_assumption_summaries_helper_fixture_snapshot_separation_sandbox_validation_and_diff_first_apply_parity_across_claimed_m5_ai_test_flows::{
    seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed,
    seeded_m5_suggestion_card_components_packet,
    seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed,
    M5SuggestionCardComponentsPacket,
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
            let packet = seeded_m5_suggestion_card_components_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_suggestion_card_components_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_suggestion_card_components_packet().render_matrix_csv()
            );
        }
        Some("fixture-suggestion-review-panel-preview-narrowed") => {
            let packet =
                seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-editor-suggestion-inline-beta-narrowed") => {
            let packet =
                seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_suggestion_card_components_packet(),
                seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed(),
                seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed(),
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
    packet: &M5SuggestionCardComponentsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "suggestion card components primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
