//! Headless emitter for the M5 docs-search-bar / scope-switcher primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/`, its
//! matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/`. Docs-browser,
//! onboarding, AI citation-follow, support/help, and CLI surfaces read this
//! primitive so one search bar names its corpus and provider, one scope switcher
//! names its version scope, and every narrowed, offline, mirror-only, or
//! policy-limited lookup degrades to a self-contained banner rather than empty
//! results with no explanation.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- report
//! cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- csv
//! cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- fixture-onboarding-lookup-beta-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- fixture-ai-citation-follow-preview-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- validate
//! ```

use aureline_docs::implement_docs_search_bars_and_scope_switchers_with_corpus_provider_and_cached_live_state_truth::{
    seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed,
    seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed,
    seeded_m5_docs_search_primitive_packet, M5DocsSearchPrimitivePacket,
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
            let packet = seeded_m5_docs_search_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_docs_search_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_docs_search_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-onboarding-lookup-beta-narrowed") => {
            let packet = seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-citation-follow-preview-narrowed") => {
            let packet = seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_docs_search_primitive_packet(),
                seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed(),
                seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed(),
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

fn assert_valid(packet: &M5DocsSearchPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
