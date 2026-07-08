//! Headless emitter for the M5 why-unavailable / source-language primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-why-unavailable-source-language-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-why-unavailable-source-language-primitive/`. The command-help row, the
//! menu-and-action row, the inline-status row, the settings-and-docs row, and the support
//! explanation export consumers read this matrix so a blocked action names its owner, reason, and
//! next safe action, and a localized surface preserves the source-language text, stable ID, and
//! canonical citation — never a generic disabled state and never an unsourced paraphrase.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_why_unavailable_source_language_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_why_unavailable_source_language_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_why_unavailable_source_language_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_why_unavailable_source_language_primitive -- fixture-menu-and-action-row-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_why_unavailable_source_language_primitive -- fixture-support-explanation-export-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_why_unavailable_source_language_primitive -- validate
//! ```

use aureline_learning::implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_with_owner_reason_next_safe_action_truth_and_citation_preserving_help_parity_across_claimed_m5_blocked_action_and_localized_surfaces::{
    seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed,
    seeded_m5_blocked_localized_row_packet,
    seeded_m5_blocked_localized_support_explanation_export_preview_narrowed,
    M5BlockedLocalizedRowPacket,
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
            let packet = seeded_m5_blocked_localized_row_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_blocked_localized_row_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_blocked_localized_row_packet().render_matrix_csv()
            );
        }
        Some("fixture-menu-and-action-row-beta-narrowed") => {
            let packet = seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-explanation-export-preview-narrowed") => {
            let packet = seeded_m5_blocked_localized_support_explanation_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_blocked_localized_row_packet(),
                seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed(),
                seeded_m5_blocked_localized_support_explanation_export_preview_narrowed(),
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

fn assert_valid(packet: &M5BlockedLocalizedRowPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "why unavailable source language primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
