//! Headless emitter for the M5 glossary-chip-card / safe-explanation-banner controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-glossary-chip-card-safe-explanation-banner-proof/`, its matrix CSV,
//! the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-glossary-chip-card-safe-explanation-banner-controls/`. The glossary surfaces
//! and the explanation surfaces read these controls so one glossary chip or card names exactly
//! what a term means and where its definition is cited from, and one safe explanation banner
//! names why a result is suggested or what a term means while keeping explain and do visibly
//! separate — never implying an apply-capable action or hidden authority.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- fixture-glossary-chip-card-uncited
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- fixture-safe-explanation-banner-explain-only
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- validate
//! ```

use aureline_learning::implement_glossary_chips_or_cards_and_safe_explanation_banners_with_cited_file_symbol_doc_truth_freshness_source_class_labels_and_explain_versus_do_separation_across_claimed_m5_learning_surfaces::{
    seeded_glossary_chip_card_safe_explanation_banner_controls,
    seeded_glossary_chip_card_safe_explanation_banner_controls_glossary_chip_card_uncited,
    seeded_glossary_chip_card_safe_explanation_banner_controls_safe_explanation_banner_explain_only,
    GlossaryChipCardSafeExplanationBannerControlsPacket,
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
            let packet = seeded_glossary_chip_card_safe_explanation_banner_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_glossary_chip_card_safe_explanation_banner_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_glossary_chip_card_safe_explanation_banner_controls().render_matrix_csv()
            );
        }
        Some("fixture-glossary-chip-card-uncited") => {
            let packet =
                seeded_glossary_chip_card_safe_explanation_banner_controls_glossary_chip_card_uncited(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-safe-explanation-banner-explain-only") => {
            let packet =
                seeded_glossary_chip_card_safe_explanation_banner_controls_safe_explanation_banner_explain_only(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_glossary_chip_card_safe_explanation_banner_controls(),
                seeded_glossary_chip_card_safe_explanation_banner_controls_glossary_chip_card_uncited(
                ),
                seeded_glossary_chip_card_safe_explanation_banner_controls_safe_explanation_banner_explain_only(
                ),
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
    packet: &GlossaryChipCardSafeExplanationBannerControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "glossary chip card safe explanation banner primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
