//! Headless emitter for the M5 settings-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-settings-row-proof/`, its matrix CSV, the Markdown
//! report `artifacts/components/m5-settings-row-primitive.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-settings-row-primitive/`. Every M5
//! config-bearing surface (admin, trust, AI, network, execution, extension, and
//! update/config) reads this primitive so effective value, configured value,
//! source pill, lock reason, and diff / open-source-detail behavior stay
//! consistent, and so the support export reconstructs effective-value truth from
//! one shared row model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_settings_row_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_settings_row_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_settings_row_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_settings_row_primitive -- fixture-admin-enterprise-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_settings_row_primitive -- fixture-update-channel-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_settings_row_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_settings_row_effective_value_source_pill_and_lock_state_primitive::{
    seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed,
    seeded_m5_settings_row_primitive_packet,
    seeded_m5_settings_row_primitive_update_channel_preview_narrowed,
    M5SettingsRowPrimitivePacket,
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
            let packet = seeded_m5_settings_row_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_settings_row_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_settings_row_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-admin-enterprise-beta-narrowed") => {
            let packet = seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-update-channel-preview-narrowed") => {
            let packet = seeded_m5_settings_row_primitive_update_channel_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_settings_row_primitive_packet(),
                seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed(),
                seeded_m5_settings_row_primitive_update_channel_preview_narrowed(),
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

fn assert_valid(packet: &M5SettingsRowPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
