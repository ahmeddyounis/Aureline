//! Headless emitter for the M5 resolution-memory-row and finding-lifecycle-transition registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-ai-review-resolution-memory-and-finding-lifecycle-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-ai-review-resolution-memory-and-finding-lifecycle-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- report
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- resolution-memory-row-table
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- fixture-resolution-memory-row-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- fixture-finding-lifecycle-transition-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_ai_review_resolution_memory_and_finding_lifecycle_registries -- validate
//! ```

use aureline_ui::m5_ai_review_resolution_memory_and_finding_lifecycle_registries::{
    seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries,
    seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_finding_lifecycle_transition_preview_narrowed,
    seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_resolution_memory_row_beta_narrowed,
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacket,
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
            let packet = seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries()
                    .render_matrix_csv()
            );
        }
        Some("resolution-memory-row-table") => {
            print!(
                "{}",
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries()
                    .render_resolution_memory_row_table()
            );
        }
        Some("fixture-resolution-memory-row-beta-narrowed") => {
            let packet =
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_resolution_memory_row_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-finding-lifecycle-transition-preview-narrowed") => {
            let packet =
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_finding_lifecycle_transition_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries(),
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_resolution_memory_row_beta_narrowed(),
                seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_finding_lifecycle_transition_preview_narrowed(),
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
    packet: &M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
