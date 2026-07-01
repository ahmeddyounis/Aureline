//! Headless emitter for the M5 shell-zone occupancy & declared-slot routing proof.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard,
//! support export, and CSV checked in under
//! `artifacts/release/m5-shell-occupancy-proof/` and the markdown report under
//! `artifacts/shell/m5-shell-zone-occupancy.md`, plus the protected fixtures under
//! `fixtures/ui/m5-shell-occupancy/`. Shell / windowing / layout / release
//! automation, release-center, docs/help, and support exports resolve the same
//! occupancy rows — green/yellow/red status, active waivers, and the exact
//! occupancy causes — through this proof rather than restating slot, placeholder, or
//! route posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- validate
//! ```

use aureline_shell::m5_shell_zone_occupancy::{
    seeded_m5_shell_occupancy_packet, validate_m5_shell_occupancy_packet, ShellOccupancyPacket,
    ShellOccupancySupportExport, M5_SHELL_OCCUPANCY_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            let export = ShellOccupancySupportExport::from_packet(
                M5_SHELL_OCCUPANCY_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_shell_occupancy_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &ShellOccupancyPacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_shell_occupancy_packet(packet) {
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
