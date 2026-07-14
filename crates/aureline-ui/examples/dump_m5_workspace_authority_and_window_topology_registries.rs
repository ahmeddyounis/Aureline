//! Headless emitter for the M5 workspace-authority and window-topology registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-workspace-authority-and-window-topology-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-workspace-authority-and-window-topology-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- report
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- workspace-ownership-table
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- fixture-multi-window-shared-authority-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- fixture-auxiliary-window-topology-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_workspace_authority_and_window_topology_registries -- validate
//! ```

use aureline_ui::m5_workspace_authority_and_window_topology_registries::{
    seeded_m5_workspace_authority_and_window_topology_registries,
    seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed,
    seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed,
    M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
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
            let packet = seeded_m5_workspace_authority_and_window_topology_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_workspace_authority_and_window_topology_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_workspace_authority_and_window_topology_registries().render_matrix_csv()
            );
        }
        Some("workspace-ownership-table") => {
            print!(
                "{}",
                seeded_m5_workspace_authority_and_window_topology_registries()
                    .render_workspace_ownership_table()
            );
        }
        Some("fixture-multi-window-shared-authority-beta-narrowed") => {
            let packet =
                seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-auxiliary-window-topology-preview-narrowed") => {
            let packet =
                seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_workspace_authority_and_window_topology_registries(),
                seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed(),
                seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed(),
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
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
