//! Headless emitter for the M5 review-scope-selector-state and rerun/outdated-freshness registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ai-review-scope-selector-and-rerun-state-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-ai-review-scope-selector-and-rerun-state-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- review-scope-selector-table
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- fixture-review-scope-selector-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- fixture-rerun-freshness-diff-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_scope_selector_and_rerun_state_registries -- validate
//! ```

use aureline_ui::m5_ai_review_scope_selector_and_rerun_state_registries::{
    seeded_m5_ai_review_scope_selector_and_rerun_state_registries,
    seeded_m5_ai_review_scope_selector_and_rerun_state_registries_rerun_freshness_diff_preview_narrowed,
    seeded_m5_ai_review_scope_selector_and_rerun_state_registries_review_scope_selector_beta_narrowed,
    M5AiReviewScopeSelectorAndRerunStateRegistriesPacket,
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
            let packet = seeded_m5_ai_review_scope_selector_and_rerun_state_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries().render_matrix_csv()
            );
        }
        Some("review-scope-selector-table") => {
            print!(
                "{}",
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries()
                    .render_review_scope_selector_table()
            );
        }
        Some("fixture-review-scope-selector-beta-narrowed") => {
            let packet =
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries_review_scope_selector_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-rerun-freshness-diff-preview-narrowed") => {
            let packet =
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries_rerun_freshness_diff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries(),
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries_review_scope_selector_beta_narrowed(),
                seeded_m5_ai_review_scope_selector_and_rerun_state_registries_rerun_freshness_diff_preview_narrowed(),
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
    packet: &M5AiReviewScopeSelectorAndRerunStateRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
