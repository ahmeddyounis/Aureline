//! Headless emitter for the M5 line-ai_review_finding_record and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ai-review-finding-and-scope-source-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-ai-review-finding-and-scope-source-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- ai-review-finding-record-table
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- fixture-ai-review-finding-record-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- fixture-scope-source-diff-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_finding_and_scope_source_registries -- validate
//! ```

use aureline_ui::m5_ai_review_finding_and_scope_source_registries::{
    seeded_m5_ai_review_finding_and_scope_source_registries,
    seeded_m5_ai_review_finding_and_scope_source_registries_ai_review_finding_record_beta_narrowed,
    seeded_m5_ai_review_finding_and_scope_source_registries_scope_source_diff_preview_narrowed,
    M5AiReviewFindingAndScopeSourceRegistriesPacket,
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
            let packet = seeded_m5_ai_review_finding_and_scope_source_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_review_finding_and_scope_source_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_review_finding_and_scope_source_registries().render_matrix_csv()
            );
        }
        Some("ai-review-finding-record-table") => {
            print!(
                "{}",
                seeded_m5_ai_review_finding_and_scope_source_registries()
                    .render_ai_review_finding_record_table()
            );
        }
        Some("fixture-ai-review-finding-record-beta-narrowed") => {
            let packet =
                seeded_m5_ai_review_finding_and_scope_source_registries_ai_review_finding_record_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-scope-source-diff-preview-narrowed") => {
            let packet =
                seeded_m5_ai_review_finding_and_scope_source_registries_scope_source_diff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_review_finding_and_scope_source_registries(),
                seeded_m5_ai_review_finding_and_scope_source_registries_ai_review_finding_record_beta_narrowed(),
                seeded_m5_ai_review_finding_and_scope_source_registries_scope_source_diff_preview_narrowed(),
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
    packet: &M5AiReviewFindingAndScopeSourceRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
