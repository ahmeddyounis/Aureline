//! Headless emitter for the M5 command-surface parity certification.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard, support export, and CSV
//! checked in under `artifacts/release/m5-command-surface-parity-proof/` and the markdown report under
//! `artifacts/commands/m5-command-surface-parity.md`, plus the protected fixtures under
//! `fixtures/commands/m5-command-surface-parity/`. Product UI, CLI, help, Support Center, the command
//! palette, and AI automation resolve the same certification rows — green/yellow/red status, active
//! waivers, and the exact conformance causes — through this proof rather than restating
//! canonical-projection, target-guard, route-parity, or support-export-parity posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- validate
//! ```

use aureline_shell::m5_command_surface_parity::{
    seeded_m5_command_surface_parity_packet, validate_m5_command_surface_parity_packet,
    CommandSurfaceParityPacket, CommandSurfaceParitySupportExport,
    M5_COMMAND_SURFACE_PARITY_SUPPORT_EXPORT_ID,
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
        Some("packet") | None => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            let export = CommandSurfaceParitySupportExport::from_packet(
                M5_COMMAND_SURFACE_PARITY_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_command_surface_parity_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &CommandSurfaceParityPacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_command_surface_parity_packet(packet) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let tokens: Vec<String> = errors
                .iter()
                .map(|error| serde_json::to_string(error).unwrap_or_default())
                .collect();
            Err(format!("packet failed validation: {}", tokens.join(",")).into())
        }
    }
}
