//! Headless emitter for the M5 work-item component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-work-item-component-consumer-proof/`, its matrix CSV, the Markdown
//! report, and the narrowed fixtures under `fixtures/ui/m5-work-item-component-consumers/`. The
//! issue inbox, the work-item detail, the review workspace, the incident workspace, Help /
//! docs, the support / export desk, and the offline export packet read this matrix so canonical
//! identity, provider authority, local-versus-provider state, linked context, the side-effect
//! preview, and publish-later continuity stay one truth, and queued-local or offline-captured
//! state never masquerades as provider-committed state.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_consumers -- support-export
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_consumers -- report
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_consumers -- csv
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_consumers -- fixture-incident-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_consumers -- fixture-review-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_consumers -- validate
//! ```

use aureline_provider::add_shared_inbox_detail_review_incident_help_support_and_export_consumers_so_work_item_components_keep_provider_freshness_and_offline_handoff_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_work_item_component_consumer_incident_beta_narrowed,
    seeded_m5_work_item_component_consumer_packet,
    seeded_m5_work_item_component_consumer_review_preview_narrowed,
    M5WorkItemComponentConsumerPacket,
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
            let packet = seeded_m5_work_item_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_work_item_component_consumer_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_work_item_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-incident-beta-narrowed") => {
            let packet = seeded_m5_work_item_component_consumer_incident_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-preview-narrowed") => {
            let packet = seeded_m5_work_item_component_consumer_review_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_work_item_component_consumer_packet(),
                seeded_m5_work_item_component_consumer_incident_beta_narrowed(),
                seeded_m5_work_item_component_consumer_review_preview_narrowed(),
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
    packet: &M5WorkItemComponentConsumerPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "work-item component consumer lane failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
