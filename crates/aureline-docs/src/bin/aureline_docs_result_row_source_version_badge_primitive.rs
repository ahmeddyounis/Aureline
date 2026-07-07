//! Headless emitter for the M5 docs-result-row / source-version-badge primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/`, its
//! matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/`.
//! Docs-browser, AI-answer, onboarding, support, and CLI surfaces read this primitive
//! so one result row names its kind, source provider, version scope, symbol-match
//! confidence, and freshness, one source/version badge distinguishes local/project
//! docs from upstream/vendor docs, and every materially overridden ranking degrades to
//! a self-contained rank-reason disclosure rather than a silent reorder.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- report
//! cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- csv
//! cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- fixture-onboarding-reference-beta-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- fixture-ai-citation-preview-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- validate
//! ```

use aureline_docs::implement_docs_result_rows_and_source_or_version_badges_with_result_kind_provider_version_scope_and_freshness_truth::{
    seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed,
    seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed,
    seeded_m5_docs_result_row_primitive_packet, M5DocsResultRowPrimitivePacket,
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
            let packet = seeded_m5_docs_result_row_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_docs_result_row_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_docs_result_row_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-onboarding-reference-beta-narrowed") => {
            let packet = seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-citation-preview-narrowed") => {
            let packet = seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_docs_result_row_primitive_packet(),
                seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed(),
                seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed(),
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

fn assert_valid(packet: &M5DocsResultRowPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
