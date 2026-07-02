//! Headless emitter for the M5 discoverability-access-parity certification.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard, support export, and CSV
//! checked in under `artifacts/release/m5-discoverability-access-parity-proof/` and the markdown report
//! under `artifacts/commands/m5-discoverability-access-parity.md`, plus the protected fixtures under
//! `fixtures/commands/m5-discoverability-access-parity/`. Product UI, CLI, help, Support Center, the command
//! palette, menus, keybinding UI, and onboarding resolve the same access-parity rows — green/yellow/red
//! status, active waivers, and the exact conformance causes — through this proof rather than restating
//! non-pointer-reach, support-export-evidence, profile-stability, or release-evidence posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- validate
//! ```

use aureline_shell::m5_discoverability_access_parity::{
    seeded_m5_discoverability_access_parity_packet, validate_m5_access_parity_packet,
    AccessParityPacket, AccessParitySupportExport, M5_ACCESS_PARITY_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            let export =
                AccessParitySupportExport::from_packet(M5_ACCESS_PARITY_SUPPORT_EXPORT_ID, packet);
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_discoverability_access_parity_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &AccessParityPacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_access_parity_packet(packet) {
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
