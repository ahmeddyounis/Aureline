//! Headless emitter for the M5 affected-install assessment panel primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-affected-install-panel-proof/`, its matrix CSV, the Markdown
//! report `artifacts/security/m5-affected-install-panel-primitive.md`, and the narrowed
//! fixtures under `fixtures/security/m5-affected-install-panel-primitive/`. Every M5
//! surface that has to answer "am I affected?" — update center, Help/About, support
//! bundle, and admin report — reads this primitive so the build / channel / install-mode
//! identity, the impacted components, the current exposure, the mitigation status, the
//! mirror freshness, and the attached rollback / repin / help actions stay consistent,
//! and so the support export reconstructs the assessment from one shared panel model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- fixture-managed-deployed-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- fixture-offline-bundle-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_affected_install_assessment_panel_primitive::{
    seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed,
    seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed,
    seeded_m5_affected_install_panel_primitive_packet, M5AffectedInstallPanelPacket,
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
            let packet = seeded_m5_affected_install_panel_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_affected_install_panel_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_affected_install_panel_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-managed-deployed-beta-narrowed") => {
            let packet =
                seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-bundle-preview-narrowed") => {
            let packet =
                seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_affected_install_panel_primitive_packet(),
                seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed(),
                seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed(),
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

fn assert_valid(packet: &M5AffectedInstallPanelPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
