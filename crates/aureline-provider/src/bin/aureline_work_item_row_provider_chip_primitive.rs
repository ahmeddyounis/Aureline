//! Headless emitter for the M5 work-item-row / provider-chip-group controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-work-item-row-provider-chip-proof/`, its matrix
//! CSV, the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-work-item-row-provider-chip-controls/`. Issue-inbox, detail,
//! review, incident, support, and CLI work-item surfaces read these controls so
//! one work-item row names its canonical id, title, state, owner, priority, and
//! linked-change count with a derived state authority that lets a user tell
//! provider-authoritative state from local-only or blocked capability directly in
//! a list, and one provider chip group names its project or space scope, tenant
//! cue, and explicit read-only/comment-link/full-edit/offline-capture/policy-blocked
//! write posture.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- fixture-work-item-row-local-only
//! cargo run -q -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- fixture-provider-chip-offline-capture
//! cargo run -q -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- validate
//! ```

use aureline_provider::implement_work_item_rows_and_provider_chip_groups_with_canonical_id_owner_state_freshness_and_write_scope_truth::{
    seeded_work_item_row_provider_chip_controls,
    seeded_work_item_row_provider_chip_controls_provider_chip_offline_capture,
    seeded_work_item_row_provider_chip_controls_work_item_row_local_only,
    WorkItemRowProviderChipControlsPacket,
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
            let packet = seeded_work_item_row_provider_chip_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_work_item_row_provider_chip_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_work_item_row_provider_chip_controls().render_matrix_csv()
            );
        }
        Some("fixture-work-item-row-local-only") => {
            let packet = seeded_work_item_row_provider_chip_controls_work_item_row_local_only();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-provider-chip-offline-capture") => {
            let packet =
                seeded_work_item_row_provider_chip_controls_provider_chip_offline_capture();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_work_item_row_provider_chip_controls(),
                seeded_work_item_row_provider_chip_controls_work_item_row_local_only(),
                seeded_work_item_row_provider_chip_controls_provider_chip_offline_capture(),
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
    packet: &WorkItemRowProviderChipControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "work item row provider chip controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
