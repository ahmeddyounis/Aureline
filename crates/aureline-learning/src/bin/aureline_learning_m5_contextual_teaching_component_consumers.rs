//! Headless emitter for the M5 contextual-teaching component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-contextual-teaching-component-consumer-proof/`, its matrix CSV, the
//! Markdown report, and the narrowed fixtures under
//! `fixtures/ui/m5-contextual-teaching-component-consumers/`. First-run onboarding, the
//! migration importer, keybinding / leader help, command docs, the Help pane, and the
//! localized support packet read this matrix so command binding, migration mapping,
//! blocked-action explanation, and source-language citation stay one truth, and partial or
//! unsupported state never masquerades as exact teaching parity.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_consumers -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_consumers -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_consumers -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_consumers -- fixture-migration-importer-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_consumers -- fixture-help-pane-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_consumers -- validate
//! ```

use aureline_learning::add_shared_onboarding_help_importer_keybinding_modal_command_doc_consumers_so_contextual_teaching_components_keep_mapping_enablement_source_language_truth_aligned_across_claimed_m5_profiles::{
    seeded_m5_teaching_component_consumer_help_pane_preview_narrowed,
    seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed,
    seeded_m5_teaching_component_consumer_packet, M5TeachingComponentConsumerPacket,
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
            let packet = seeded_m5_teaching_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_teaching_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_teaching_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-migration-importer-beta-narrowed") => {
            let packet = seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-help-pane-preview-narrowed") => {
            let packet = seeded_m5_teaching_component_consumer_help_pane_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_teaching_component_consumer_packet(),
                seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed(),
                seeded_m5_teaching_component_consumer_help_pane_preview_narrowed(),
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
    packet: &M5TeachingComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "teaching component consumer lane failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
