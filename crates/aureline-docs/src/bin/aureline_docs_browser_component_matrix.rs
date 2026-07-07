//! Headless emitter for the frozen M5 docs-browser component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/`,
//! its matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/`.
//! Docs-browser, help-center, onboarding, AI-context, search-palette, hover-peek,
//! support, and admin surfaces read this matrix so one search bar names its
//! corpus, one scope switcher names its version scope, one result row states its
//! match state and override reason, one symbol card names its anchor, one badge
//! names its provider and freshness, one docs-pack row states its pin/mirror/
//! offline/quarantine state, one stale-example row states its staleness, and one
//! handoff banner states exactly why Aureline handed off to a browser.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_component_matrix -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_component_matrix -- report
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_component_matrix -- csv
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_component_matrix -- fixture-stale-example-finding-row-beta-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_component_matrix -- fixture-handoff-banner-preview-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_browser_component_matrix -- validate
//! ```

use aureline_docs::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    seeded_m5_docs_browser_component_matrix,
    seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed,
    seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed,
    M5DocsBrowserMatrixPacket,
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
            let packet = seeded_m5_docs_browser_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_docs_browser_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_docs_browser_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-stale-example-finding-row-beta-narrowed") => {
            let packet =
                seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-handoff-banner-preview-narrowed") => {
            let packet = seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_docs_browser_component_matrix(),
                seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed(),
                seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed(),
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

fn assert_valid(packet: &M5DocsBrowserMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
