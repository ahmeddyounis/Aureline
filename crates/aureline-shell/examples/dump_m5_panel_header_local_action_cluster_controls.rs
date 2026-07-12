//! Headless emitter for the M5 panel-header and local-action-cluster controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-panel-header-local-action-cluster-controls-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-panel-header-local-action-cluster-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- report
//! cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- fixture-shell-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- fixture-support-export-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- validate
//! ```

use aureline_shell::implement_the_m5_panel_header_and_local_action_cluster_stable_title_overflow_rule_source_freshness_cue_and_command_backed_action_primitive::{
    seeded_m5_panel_header_local_action_cluster_controls,
    seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed,
    seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed,
    M5PanelControlsPacket,
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
            let packet = seeded_m5_panel_header_local_action_cluster_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_panel_header_local_action_cluster_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_panel_header_local_action_cluster_controls().render_matrix_csv()
            );
        }
        Some("fixture-shell-ui-beta-narrowed") => {
            let packet =
                seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-export-preview-narrowed") => {
            let packet =
                seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_panel_header_local_action_cluster_controls(),
                seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed(),
                seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed(),
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

fn assert_valid(packet: &M5PanelControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
