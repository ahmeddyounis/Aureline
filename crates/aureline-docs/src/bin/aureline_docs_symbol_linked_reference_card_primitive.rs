//! Headless emitter for the M5 symbol-linked reference-card primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/docs/m5/m5-symbol-linked-reference-card-primitive/`, its matrix CSV, the
//! Markdown report, and the narrowed fixtures under
//! `fixtures/docs/m5/m5-symbol-linked-reference-card-primitive/`. Editor hover/peek,
//! docs-browser, AI-explanation, onboarding, and support surfaces read this primitive
//! so one reference card keeps the initiating file/symbol code anchor visible, names
//! how strong the symbol linkage is (an exact symbol match, a nearby version match, a
//! project-specific override, or a keyword fallback), and never shows a cached,
//! mirrored, or stale cited revision as live.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_symbol_linked_reference_card_primitive -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_symbol_linked_reference_card_primitive -- report
//! cargo run -q -p aureline-docs --bin aureline_docs_symbol_linked_reference_card_primitive -- csv
//! cargo run -q -p aureline-docs --bin aureline_docs_symbol_linked_reference_card_primitive -- fixture-onboarding-reference-beta-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_symbol_linked_reference_card_primitive -- fixture-ai-explanation-preview-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_symbol_linked_reference_card_primitive -- validate
//! ```

use aureline_docs::implement_docs_symbol_linked_reference_cards_with_code_anchor_and_exact_nearby_project_or_keyword_fallback_truth::{
    seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed,
    seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed,
    seeded_m5_reference_card_primitive_packet, M5DocsReferenceCardPrimitivePacket,
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
            let packet = seeded_m5_reference_card_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_reference_card_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_reference_card_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-onboarding-reference-beta-narrowed") => {
            let packet = seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-explanation-preview-narrowed") => {
            let packet = seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_reference_card_primitive_packet(),
                seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed(),
                seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed(),
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
    packet: &M5DocsReferenceCardPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
