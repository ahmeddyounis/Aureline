//! Headless emitter for the M5 shared-state-taxonomy component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-shared-state-taxonomy-component-consumer-proof/`, its matrix CSV, the
//! Markdown report, and the narrowed fixtures under
//! `fixtures/ui/m5-shared-state-taxonomy-component-consumers/`. Shell chrome, command / help,
//! search / dense collections, review / work-item flows, settings / capability prompts,
//! provider / offline-capture rows, test / watch surfaces, and support / recovery lanes read
//! this matrix so state semantics, state cause, consequence/recovery, and the accessibility label
//! stay one truth, and an incomplete or degraded state never masquerades as an exact, healthy
//! state.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_state_taxonomy_component_consumers -- support-export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_state_taxonomy_component_consumers -- report
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_state_taxonomy_component_consumers -- csv
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_state_taxonomy_component_consumers -- fixture-provider-offline-capture-beta-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_state_taxonomy_component_consumers -- fixture-test-watch-preview-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_state_taxonomy_component_consumers -- validate
//! ```

use aureline_design_system::add_shared_shell_command_search_review_settings_provider_test_and_support_consumers_so_state_taxonomy_components_keep_label_recovery_and_accessibility_parity_across_claimed_m5_profiles::{
    seeded_m5_state_component_consumer_packet,
    seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed,
    seeded_m5_state_component_consumer_test_watch_preview_narrowed, M5StateComponentConsumerPacket,
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
            let packet = seeded_m5_state_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_state_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_state_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-provider-offline-capture-beta-narrowed") => {
            let packet =
                seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-test-watch-preview-narrowed") => {
            let packet = seeded_m5_state_component_consumer_test_watch_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_state_component_consumer_packet(),
                seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed(),
                seeded_m5_state_component_consumer_test_watch_preview_narrowed(),
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

fn assert_valid(packet: &M5StateComponentConsumerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "state component consumer lane failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
