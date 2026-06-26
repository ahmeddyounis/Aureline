//! Headless emitter for the M5 live-announcement grammar catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-live-announcement-proof/` and the narrowed fixtures under
//! `fixtures/a11y/m5-announcements/`. Shell, editor, terminal, notebook, data,
//! review, notifications, and help surfaces narrate M5 dynamic events through this
//! grammar so announcements carry concise meaning with stable message ids,
//! polite/assertive channel rules, coalescing budgets, and durable fallbacks rather
//! than per-surface improvised prose.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- fixture-proof-stale-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- fixture-live-region-unavailable-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_announcement_grammar -- validate
//! ```

use aureline_shell::announcement_grammar::{
    seeded_m5_announcement_grammar_catalog,
    seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed,
    seeded_m5_announcement_grammar_catalog_proof_stale_narrowed,
    M5AnnouncementGrammarCatalogPacket,
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
            let packet = seeded_m5_announcement_grammar_catalog();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_announcement_grammar_catalog().render_markdown_summary()
            );
        }
        Some("fixture-proof-stale-narrowed") => {
            let packet = seeded_m5_announcement_grammar_catalog_proof_stale_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-live-region-unavailable-narrowed") => {
            let packet = seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_announcement_grammar_catalog(),
                seeded_m5_announcement_grammar_catalog_proof_stale_narrowed(),
                seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed(),
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
    packet: &M5AnnouncementGrammarCatalogPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "announcement grammar catalog failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
