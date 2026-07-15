//! Headless emitter for the frozen M5 settings-governance matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-settings-governance-proof/`, its matrix CSV, the Markdown design report under
//! `artifacts/config/`, and the narrowed fixtures under `fixtures/config/m5-settings-runtime/`. The
//! settings, shell, diagnostics, admin, docs, and support surfaces read this matrix so setting definition
//! and effective resolution stay separately inspectable, stable setting IDs are never recycled, scoped
//! writes never widen into a broader scope, sync never silently overwrites local authoritative state, and
//! kill-switch or policy-disable causes stay self-explaining.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_settings_governance_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_settings_governance_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_settings_governance_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_settings_governance_matrix -- fixture-sync-scope-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_settings_governance_matrix -- fixture-rollout-capability-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_settings_governance_matrix -- validate
//! ```

use aureline_ui::m5_settings_governance_matrix::{
    seeded_m5_settings_governance_matrix,
    seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed,
    seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed,
    M5SettingsGovernanceMatrixPacket,
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
            let packet = seeded_m5_settings_governance_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_settings_governance_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_settings_governance_matrix().render_matrix_csv()
            );
        }
        Some("fixture-sync-scope-beta-narrowed") => {
            let packet = seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-rollout-capability-preview-narrowed") => {
            let packet = seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_settings_governance_matrix(),
                seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed(),
                seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed(),
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
    packet: &M5SettingsGovernanceMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
