//! Headless emitter for the M5 publish-to-review-sheet and publish-scope-decision registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ai-review-publish-sheet-and-scope-decision-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-ai-review-publish-sheet-and-scope-decision-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- publish-to-review-sheet-table
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- fixture-publish-to-review-sheet-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- fixture-publish-scope-decision-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_sheet_and_scope_decision_registries -- validate
//! ```

use aureline_ui::m5_ai_review_publish_sheet_and_scope_decision_registries::{
    seeded_m5_ai_review_publish_sheet_and_scope_decision_registries,
    seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_scope_decision_preview_narrowed,
    seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_to_review_sheet_beta_narrowed,
    M5AiReviewPublishSheetAndScopeDecisionRegistriesPacket,
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
            let packet = seeded_m5_ai_review_publish_sheet_and_scope_decision_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries()
                    .render_matrix_csv()
            );
        }
        Some("publish-to-review-sheet-table") => {
            print!(
                "{}",
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries()
                    .render_publish_to_review_sheet_table()
            );
        }
        Some("fixture-publish-to-review-sheet-beta-narrowed") => {
            let packet =
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_to_review_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-publish-scope-decision-preview-narrowed") => {
            let packet =
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_scope_decision_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries(),
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_to_review_sheet_beta_narrowed(),
                seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_scope_decision_preview_narrowed(),
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
    packet: &M5AiReviewPublishSheetAndScopeDecisionRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
