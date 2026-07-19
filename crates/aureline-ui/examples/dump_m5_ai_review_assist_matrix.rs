//! Headless emitter for the frozen M5 AI-review-assist matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ai-review-publish-packets/`, its matrix CSV, the Markdown design report at
//! `artifacts/review/m5-ai-review-assist-components.md`, the AI-review-assist-health dashboard at
//! `dashboards/m5-ai-review-assist-health.json`, and the narrowed fixtures under
//! `fixtures/review/m5-ai-review-assist/`. The review, AI, provider, pending-review, help / docs, and support /
//! export surfaces read this matrix so an AI review finding never publishes or merges implicitly, no publish
//! hides whether output stays local or becomes a provider comment / suggested patch / check annotation, no
//! stale finding looks current after diff or instruction drift, and no local draft or evidence is lost when a
//! publish fails.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- fixture-publish-sheet-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- fixture-resolution-memory-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_assist_matrix -- validate
//! ```

use aureline_ui::m5_ai_review_assist_matrix::{
    seeded_m5_ai_review_assist_matrix,
    seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed,
    seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed,
    M5AiReviewAssistMatrixPacket,
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
            let packet = seeded_m5_ai_review_assist_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_review_assist_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_review_assist_matrix().render_matrix_csv()
            );
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_ai_review_assist_matrix().render_dashboard_json()
            );
        }
        Some("fixture-publish-sheet-beta-narrowed") => {
            let packet = seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-resolution-memory-preview-narrowed") => {
            let packet = seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_review_assist_matrix(),
                seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed(),
                seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed(),
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

fn assert_valid(packet: &M5AiReviewAssistMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
