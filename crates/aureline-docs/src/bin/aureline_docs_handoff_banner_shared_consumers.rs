//! Headless emitter for the M5 docs handoff-banner / shared-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers/`, its matrix CSV, the
//! Markdown report, and the narrowed fixtures under
//! `fixtures/docs/m5/m5-docs-handoff-banner-and-shared-consumers/`. The docs browser, the
//! onboarding tour, the glossary card, the AI-evidence follow link, and the support/help
//! view read this banner so a handoff always explains its destination reason, its privacy
//! consequence, its return path, and why Aureline could not or should not satisfy the
//! request in-product, and never flattens the interaction into a raw URL jump that strips
//! source/version context.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_handoff_banner_shared_consumers -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_handoff_banner_shared_consumers -- report
//! cargo run -q -p aureline-docs --bin aureline_docs_handoff_banner_shared_consumers -- csv
//! cargo run -q -p aureline-docs --bin aureline_docs_handoff_banner_shared_consumers -- fixture-onboarding-tour-beta-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_handoff_banner_shared_consumers -- fixture-ai-evidence-preview-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_handoff_banner_shared_consumers -- validate
//! ```

use aureline_docs::add_browser_handoff_banners_and_shared_docs_browser_onboarding_glossary_ai_and_support_consumers::{
    seeded_m5_docs_handoff_consumer_ai_evidence_preview_narrowed,
    seeded_m5_docs_handoff_consumer_onboarding_tour_beta_narrowed,
    seeded_m5_docs_handoff_consumer_packet, M5DocsHandoffConsumerPacket,
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
            let packet = seeded_m5_docs_handoff_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_docs_handoff_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_docs_handoff_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-onboarding-tour-beta-narrowed") => {
            let packet = seeded_m5_docs_handoff_consumer_onboarding_tour_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-evidence-preview-narrowed") => {
            let packet = seeded_m5_docs_handoff_consumer_ai_evidence_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_docs_handoff_consumer_packet(),
                seeded_m5_docs_handoff_consumer_onboarding_tour_beta_narrowed(),
                seeded_m5_docs_handoff_consumer_ai_evidence_preview_narrowed(),
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

fn assert_valid(packet: &M5DocsHandoffConsumerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("lane failed validation: {}", tokens.join(",")).into())
    }
}
