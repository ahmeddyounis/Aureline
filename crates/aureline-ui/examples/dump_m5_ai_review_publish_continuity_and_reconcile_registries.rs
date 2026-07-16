//! Headless emitter for the M5 publish-later-draft and compare-reconcile-review registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ai-review-publish-continuity-and-reconcile-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-ai-review-publish-continuity-and-reconcile-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- publish-later-draft-table
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- fixture-publish-later-draft-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- fixture-compare-reconcile-review-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_publish_continuity_and_reconcile_registries -- validate
//! ```

use aureline_ui::m5_ai_review_publish_continuity_and_reconcile_registries::{
    seeded_m5_ai_review_publish_continuity_and_reconcile_registries,
    seeded_m5_ai_review_publish_continuity_and_reconcile_registries_compare_reconcile_review_preview_narrowed,
    seeded_m5_ai_review_publish_continuity_and_reconcile_registries_publish_later_draft_beta_narrowed,
    M5AiReviewPublishContinuityAndReconcileRegistriesPacket,
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
            let packet = seeded_m5_ai_review_publish_continuity_and_reconcile_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries()
                    .render_matrix_csv()
            );
        }
        Some("publish-later-draft-table") => {
            print!(
                "{}",
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries()
                    .render_publish_later_draft_table()
            );
        }
        Some("fixture-publish-later-draft-beta-narrowed") => {
            let packet =
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries_publish_later_draft_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-compare-reconcile-review-preview-narrowed") => {
            let packet =
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries_compare_reconcile_review_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries(),
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries_publish_later_draft_beta_narrowed(),
                seeded_m5_ai_review_publish_continuity_and_reconcile_registries_compare_reconcile_review_preview_narrowed(),
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
    packet: &M5AiReviewPublishContinuityAndReconcileRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
