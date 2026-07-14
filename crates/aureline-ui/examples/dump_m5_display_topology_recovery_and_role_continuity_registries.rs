//! Headless emitter for the M5 display-topology-recovery bounds-recovery and role-continuity registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-display-topology-recovery-and-role-continuity-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-display-topology-recovery-and-role-continuity-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- report
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- bounds-recovery-table
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- fixture-dpi-rescale-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- fixture-reduced-fidelity-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- validate
//! ```

use aureline_ui::m5_display_topology_recovery_and_role_continuity_registries::{
    seeded_m5_display_topology_recovery_and_role_continuity_registries,
    seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed,
    seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed,
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
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
            let packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_display_topology_recovery_and_role_continuity_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_display_topology_recovery_and_role_continuity_registries()
                    .render_matrix_csv()
            );
        }
        Some("bounds-recovery-table") => {
            print!(
                "{}",
                seeded_m5_display_topology_recovery_and_role_continuity_registries()
                    .render_bounds_recovery_table()
            );
        }
        Some("fixture-dpi-rescale-beta-narrowed") => {
            let packet =
                seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-reduced-fidelity-preview-narrowed") => {
            let packet =
                seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_display_topology_recovery_and_role_continuity_registries(),
                seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed(),
                seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed(),
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
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
