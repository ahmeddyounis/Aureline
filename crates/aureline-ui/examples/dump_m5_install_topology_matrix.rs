//! Headless emitter for the frozen M5 install-topology matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-install-topology-proof/`, its matrix CSV, the Markdown design report, and the
//! narrowed fixtures under `fixtures/install/m5-delivery-topologies/`. The About, update, diagnostics,
//! admin, docs, and support surfaces read this matrix so binary placement and updater ownership stay
//! inspectable, portable mode never spills machine-global durable state, stable and preview channels never
//! corrupt one another, silent and managed flows preserve diagnostics and repair / verify truth, rollback
//! targets the full artifact graph, and rollout rings keep promotion and rollback evidence per ring.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_install_topology_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_install_topology_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_install_topology_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_install_topology_matrix -- fixture-side-by-side-channel-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_install_topology_matrix -- fixture-offline-airgap-bundle-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_install_topology_matrix -- validate
//! ```

use aureline_ui::m5_install_topology_matrix::{
    seeded_m5_install_topology_matrix,
    seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed,
    seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed,
    M5InstallTopologyMatrixPacket,
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
            let packet = seeded_m5_install_topology_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_install_topology_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_install_topology_matrix().render_matrix_csv()
            );
        }
        Some("fixture-side-by-side-channel-beta-narrowed") => {
            let packet = seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-airgap-bundle-preview-narrowed") => {
            let packet = seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_install_topology_matrix(),
                seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed(),
                seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed(),
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

fn assert_valid(packet: &M5InstallTopologyMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
