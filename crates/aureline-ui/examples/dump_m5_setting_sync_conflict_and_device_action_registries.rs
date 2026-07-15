//! Headless emitter for the M5 sync-conflict and device-action registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-setting-sync-conflict-and-device-action-registries-proof/`, its matrix CSV, the Markdown
//! summary, and the narrowed fixtures under
//! `fixtures/config/m5-setting-sync-conflict-and-device-action-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- report
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- conflict-table
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- fixture-sync-conflict-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- fixture-device-action-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- validate
//! ```

use aureline_ui::m5_setting_sync_conflict_and_device_action_registries::{
    seeded_m5_setting_sync_conflict_and_device_action_registries,
    seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed,
    seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed,
    M5SettingSyncConflictDeviceActionRegistriesPacket,
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
            let packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_setting_sync_conflict_and_device_action_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_setting_sync_conflict_and_device_action_registries().render_matrix_csv()
            );
        }
        Some("conflict-table") => {
            print!(
                "{}",
                seeded_m5_setting_sync_conflict_and_device_action_registries()
                    .render_conflict_table()
            );
        }
        Some("fixture-sync-conflict-beta-narrowed") => {
            let packet =
                seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-device-action-preview-narrowed") => {
            let packet =
                seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_setting_sync_conflict_and_device_action_registries(),
                seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed(),
                seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed(),
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
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
