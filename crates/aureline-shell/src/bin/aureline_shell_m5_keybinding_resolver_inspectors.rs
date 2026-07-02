//! Headless emitter for the M5 keybinding resolver inspection certification.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard, support export, and CSV
//! checked in under `artifacts/release/m5-keybinding-resolver-inspectors-proof/` and the markdown report
//! under `artifacts/commands/m5-keybinding-resolver-inspectors.md`, plus the protected fixtures under
//! `fixtures/commands/m5-keybinding-resolver-inspectors/`. Product UI, CLI, help, Support Center, the
//! command palette, the keybinding UI, and migration tooling resolve the same inspection rows —
//! green/yellow/red status, active waivers, and the exact conformance causes — through this proof rather
//! than restating resolver-inspection, bridge-outcome, leader-sequence, or resolver-export posture by
//! hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- validate
//! ```

use aureline_shell::m5_keybinding_resolver_inspectors::{
    seeded_m5_keybinding_resolver_inspectors_packet,
    validate_m5_keybinding_resolver_inspectors_packet, ResolverInspectorPacket,
    ResolverInspectorSupportExport, M5_RESOLVER_INSPECTORS_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            let export = ResolverInspectorSupportExport::from_packet(
                M5_RESOLVER_INSPECTORS_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_keybinding_resolver_inspectors_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &ResolverInspectorPacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_keybinding_resolver_inspectors_packet(packet) {
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
