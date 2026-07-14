//! Headless emitter for the M5 portable-mode state-containment and diagnostics registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-portable-mode-state-containment-and-diagnostics-proof/`, its matrix CSV, the Markdown
//! summary, and the narrowed fixtures under
//! `fixtures/install/m5-portable-mode-state-containment-and-diagnostics/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- support-export
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- report
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- csv
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- root-inventory-table
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- fixture-side-by-side-channel-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- fixture-offline-airgap-bundle-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- validate
//! ```

use aureline_ui::m5_portable_mode_state_containment_and_diagnostics::{
    seeded_m5_portable_mode_state_containment_and_diagnostics,
    seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed,
    seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed,
    M5PortableModeStateContainmentAndDiagnosticsPacket,
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
            let packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_portable_mode_state_containment_and_diagnostics()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_portable_mode_state_containment_and_diagnostics().render_matrix_csv()
            );
        }
        Some("root-inventory-table") => {
            print!(
                "{}",
                seeded_m5_portable_mode_state_containment_and_diagnostics()
                    .render_portable_root_inventory_table()
            );
        }
        Some("fixture-side-by-side-channel-beta-narrowed") => {
            let packet =
                seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-airgap-bundle-preview-narrowed") => {
            let packet =
                seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_portable_mode_state_containment_and_diagnostics(),
                seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed(),
                seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed(),
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
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
