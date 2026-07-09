//! Headless emitter for the frozen M5 work-item component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-work-item-component-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-work-item-components/`. Issue,
//! task, incident, review, support, and CLI work-item surfaces read this matrix so one
//! work-item row names its identity, authority, and local state, one provider-chip group
//! names who owns the object, one relation strip names the linked branch/review/test
//! context, one sync-pending pill names what is only local and not yet published, one
//! status-transition sheet previews the side effects before a write, one related-evidence
//! card names its provenance, and one offline-handoff card names where a deferred change
//! will land and what export will reveal.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- support-export
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- report
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- csv
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- fixture-status-transition-sheet-beta-narrowed
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- fixture-offline-handoff-packet-card-preview-narrowed
//! cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- validate
//! ```

use aureline_provider::freeze_the_m5_work_item_component_matrix::{
    seeded_m5_work_item_component_matrix,
    seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed,
    seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed,
    M5WorkItemComponentMatrixPacket,
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
            let packet = seeded_m5_work_item_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_work_item_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_work_item_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-status-transition-sheet-beta-narrowed") => {
            let packet =
                seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-handoff-packet-card-preview-narrowed") => {
            let packet =
                seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_work_item_component_matrix(),
                seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed(),
                seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed(),
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
    packet: &M5WorkItemComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
