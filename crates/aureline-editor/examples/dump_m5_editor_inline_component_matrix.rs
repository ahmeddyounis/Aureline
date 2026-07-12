//! Headless emitter for the frozen M5 editor-inline component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-editor-inline-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/ui/m5-editor-inline-components/`. Editor, diff/merge, review,
//! notebook, AI, diagnostics, and support surfaces read this matrix so one editor tab names its state
//! and context, one gutter layers markers without color alone, one diagnostic decoration names
//! severity and freshness, one code-action chip distinguishes exact from inferred fixes, one diff view
//! names every change kind, one review thread names anchor and resolution truth, one AI message card
//! names source, confidence, and actions, and one evidence timeline stays inspectable.
//!
//! ```text
//! cargo run -p aureline-editor --example dump_m5_editor_inline_component_matrix -- support-export
//! cargo run -p aureline-editor --example dump_m5_editor_inline_component_matrix -- report
//! cargo run -p aureline-editor --example dump_m5_editor_inline_component_matrix -- csv
//! cargo run -p aureline-editor --example dump_m5_editor_inline_component_matrix -- fixture-diff-view-beta-narrowed
//! cargo run -p aureline-editor --example dump_m5_editor_inline_component_matrix -- fixture-review-thread-preview-narrowed
//! cargo run -p aureline-editor --example dump_m5_editor_inline_component_matrix -- validate
//! ```

use aureline_editor::m5_editor_inline_component_matrix::{
    seeded_m5_editor_inline_component_matrix,
    seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed,
    seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed,
    M5EditorInlineComponentMatrixPacket,
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
            let packet = seeded_m5_editor_inline_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_editor_inline_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_editor_inline_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-diff-view-beta-narrowed") => {
            let packet = seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-thread-preview-narrowed") => {
            let packet = seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_editor_inline_component_matrix(),
                seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed(),
                seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed(),
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
    packet: &M5EditorInlineComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
