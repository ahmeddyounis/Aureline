//! Headless emitter for the frozen M5 workspace-trust-repair component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-workspace-trust-repair-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-workspace-trust-repair-components/`.
//! Workspace-trust, settings, Project Doctor, safe-mode, and guided-repair surfaces read this
//! matrix so one workspace-trust banner names grant source and trust scope, one trust-fact grid
//! names grant source, scope, capability, and root together, one trust-elevation sheet names the
//! grant source and scope change, one restricted-capability row names exactly which capability is
//! narrowed, one root-trust strip names per-root trust, one repair-transaction preview card names
//! targets, checkpoint, and reversal, one rollback-class strip names the reversal class and
//! checkpoint, and one repair-result receipt row names the outcome and any manual follow-up.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- support-export
//! cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- report
//! cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- csv
//! cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- fixture-trust-elevation-sheet-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- fixture-repair-transaction-preview-card-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix::{
    seeded_m5_workspace_trust_repair_component_matrix,
    seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed,
    seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed,
    M5WorkspaceTrustRepairComponentMatrixPacket,
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
            let packet = seeded_m5_workspace_trust_repair_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_workspace_trust_repair_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_workspace_trust_repair_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-trust-elevation-sheet-beta-narrowed") => {
            let packet =
                seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-repair-transaction-preview-card-preview-narrowed") => {
            let packet =
                seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_workspace_trust_repair_component_matrix(),
                seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed(),
                seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed(),
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
    packet: &M5WorkspaceTrustRepairComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
