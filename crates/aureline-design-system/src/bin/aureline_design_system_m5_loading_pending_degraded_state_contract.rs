//! Headless emitter for the M5 loading / pending / warning-error / degraded state-block contract
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-loading-pending-degraded-state-contract-primitive-proof/`, its matrix CSV,
//! the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-loading-pending-degraded-state-contract-primitive/`. The form, background job
//! row, banner, card, dense row, and review sheet read this matrix so one degraded-state contract
//! keeps `loading` distinct from `pending`, `warning` distinct from `error`, and `error` distinct
//! from `degraded`, attributes a pending action to the user action that triggered it, and preserves
//! submission lineage, what-still-works, and the next safe action — on every claimed workflow
//! surface.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- support-export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- report
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- csv
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- fixture-banner-beta-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- fixture-review-sheet-preview-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- validate
//! ```

use aureline_design_system::implement_loading_pending_warning_error_and_degraded_state_blocks_with_submission_lineage_health_and_recovery_truth_across_claimed_m5_workflows::{
    seeded_m5_degraded_state_contract_banner_beta_narrowed,
    seeded_m5_degraded_state_contract_packet,
    seeded_m5_degraded_state_contract_review_sheet_preview_narrowed,
    M5DegradedStateContractPacket,
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
            let packet = seeded_m5_degraded_state_contract_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_degraded_state_contract_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_degraded_state_contract_packet().render_matrix_csv()
            );
        }
        Some("fixture-banner-beta-narrowed") => {
            let packet = seeded_m5_degraded_state_contract_banner_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-sheet-preview-narrowed") => {
            let packet = seeded_m5_degraded_state_contract_review_sheet_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_degraded_state_contract_packet(),
                seeded_m5_degraded_state_contract_banner_beta_narrowed(),
                seeded_m5_degraded_state_contract_review_sheet_preview_narrowed(),
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

fn assert_valid(packet: &M5DegradedStateContractPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "degraded state contract primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
