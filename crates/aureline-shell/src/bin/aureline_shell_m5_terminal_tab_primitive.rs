//! Headless emitter for the M5 terminal-tab / header-strip primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-terminal-tab-proof/`, its matrix CSV, the Markdown
//! report `artifacts/components/m5-terminal-tab-primitive.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-terminal-tab-primitive/`. Every M5
//! terminal-console consumer (the terminal panel, the notebook console, the
//! request console, the preview dev-server console, and the incident shell) reads
//! this primitive so session title, host boundary, shell-integration quality,
//! cwd-or-transcript state, and shared-control truth stay consistent, and so the
//! support export reconstructs boundary and liveness truth from one shared tab
//! model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_terminal_tab_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_terminal_tab_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_terminal_tab_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_terminal_tab_primitive -- fixture-incident-shell-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_terminal_tab_primitive -- fixture-preview-dev-server-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_terminal_tab_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_terminal_tab_and_header_strip_boundary_liveness_and_shared_control_primitive::{
    seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed,
    seeded_m5_terminal_tab_primitive_packet,
    seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed,
    M5TerminalTabPrimitivePacket,
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
            let packet = seeded_m5_terminal_tab_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_terminal_tab_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_terminal_tab_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-incident-shell-beta-narrowed") => {
            let packet = seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-preview-dev-server-preview-narrowed") => {
            let packet = seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_terminal_tab_primitive_packet(),
                seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed(),
                seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed(),
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

fn assert_valid(packet: &M5TerminalTabPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
