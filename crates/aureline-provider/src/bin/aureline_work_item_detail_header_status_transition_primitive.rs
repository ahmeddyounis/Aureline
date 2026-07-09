//! Headless emitter for the M5 detail-header / status-transition-sheet controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-work-item-detail-header-status-transition-proof/`, its matrix
//! CSV, the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-work-item-detail-header-status-transition-controls/`. Work-item
//! detail, review, incident, support, and CLI surfaces read these controls so one
//! detail header states provider space, canonical id, title, state, owner, derived
//! write scope and freshness, and an open-external escape hatch, and one
//! status-transition sheet previews its mutations, linked context, notification side
//! effects, permission scope, and confirm/export/cancel behavior before any publish.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- fixture-detail-header-local-draft
//! cargo run -q -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- fixture-status-transition-publish-now
//! cargo run -q -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- validate
//! ```

use aureline_provider::implement_work_item_detail_headers_and_status_transition_sheets_with_provider_boundary_side_effect_permission_scope_and_confirm_export_cancel_truth::{
    seeded_detail_header_transition_controls,
    seeded_detail_header_transition_controls_detail_header_local_draft,
    seeded_detail_header_transition_controls_status_transition_publish_now,
    DetailHeaderTransitionControlsPacket,
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
            let packet = seeded_detail_header_transition_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_detail_header_transition_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_detail_header_transition_controls().render_matrix_csv()
            );
        }
        Some("fixture-detail-header-local-draft") => {
            let packet = seeded_detail_header_transition_controls_detail_header_local_draft();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-status-transition-publish-now") => {
            let packet = seeded_detail_header_transition_controls_status_transition_publish_now();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_detail_header_transition_controls(),
                seeded_detail_header_transition_controls_detail_header_local_draft(),
                seeded_detail_header_transition_controls_status_transition_publish_now(),
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
    packet: &DetailHeaderTransitionControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "detail header transition controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
