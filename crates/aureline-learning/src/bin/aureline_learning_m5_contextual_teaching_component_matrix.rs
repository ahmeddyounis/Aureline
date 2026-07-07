//! Headless emitter for the frozen M5 contextual-teaching component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-contextual-teaching-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under
//! `fixtures/ui/m5-contextual-teaching-components/`. First-run, guided-tour,
//! command-palette, migration-report, inline-help, and CLI help surfaces read this matrix
//! so one contextual tip card names its command binding and dismissal state, one migration
//! bridge card names how an imported behavior maps and where it came from, one
//! sequence-help strip names its current sequence state, one why-unavailable row names the
//! owner, reason, and next safe action, and one source-language fallback surface names its
//! locale and preserves its citation.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- fixture-migration-bridge-card-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- fixture-source-language-fallback-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- validate
//! ```

use aureline_learning::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    seeded_m5_contextual_teaching_component_matrix,
    seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed,
    seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed,
    M5ContextualTeachingComponentMatrixPacket,
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
            let packet = seeded_m5_contextual_teaching_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_contextual_teaching_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_contextual_teaching_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-migration-bridge-card-beta-narrowed") => {
            let packet =
                seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-source-language-fallback-preview-narrowed") => {
            let packet =
                seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_contextual_teaching_component_matrix(),
                seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed(),
                seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed(),
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
    packet: &M5ContextualTeachingComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
