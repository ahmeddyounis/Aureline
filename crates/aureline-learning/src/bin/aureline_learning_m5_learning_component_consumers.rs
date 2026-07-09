//! Headless emitter for the M5 learning component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-learning-component-consumer-proof/`, its matrix CSV, the Markdown report,
//! and the narrowed fixtures under `fixtures/ui/m5-learning-component-consumers/`. Onboarding,
//! migration, contextual help, the docs / browser surface, the feature-family tour, the companion
//! handoff, and the support / export packet read this matrix so citation, source-class, progress /
//! privacy, and explain-versus-do stay one truth, and an uncited or unavailable source never
//! masquerades as a live, cited one.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_consumers -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_consumers -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_consumers -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_consumers -- fixture-docs-browser-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_consumers -- fixture-companion-handoff-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_consumers -- validate
//! ```

use aureline_learning::add_shared_onboarding_migration_contextual_help_docs_browser_feature_family_tour_companion_handoff_and_support_export_consumers_so_learning_components_keep_citation_privacy_and_progress_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed,
    seeded_m5_learning_component_consumer_docs_browser_beta_narrowed,
    seeded_m5_learning_component_consumer_packet, M5LearningComponentConsumerPacket,
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
            let packet = seeded_m5_learning_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_learning_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_learning_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-docs-browser-beta-narrowed") => {
            let packet = seeded_m5_learning_component_consumer_docs_browser_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-companion-handoff-preview-narrowed") => {
            let packet = seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_learning_component_consumer_packet(),
                seeded_m5_learning_component_consumer_docs_browser_beta_narrowed(),
                seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed(),
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
    packet: &M5LearningComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "learning component consumer lane failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
