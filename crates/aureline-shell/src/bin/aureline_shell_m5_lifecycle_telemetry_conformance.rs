//! Headless emitter for the M5 lifecycle-telemetry-conformance proof.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard, support export, and
//! CSV checked in under `artifacts/release/m5-lifecycle-telemetry-conformance-proof/` and the markdown
//! report under `artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md`, plus the protected
//! fixtures under `fixtures/state/m5-lifecycle-telemetry-conformance/`. Product UI, CLI, diagnostics,
//! Support Center, telemetry, Shiproom, release-center, and docs/help resolve the same certification
//! rows — green/yellow/red status, active waivers, and the exact conformance causes — through this
//! proof rather than restating enum-emission, transition-event, ui-export-parity, or
//! shared-contract-consumption posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- validate
//! ```

use aureline_shell::m5_lifecycle_telemetry_conformance::{
    seeded_m5_lifecycle_telemetry_conformance_packet,
    validate_m5_lifecycle_telemetry_conformance_packet, TelemetryConformancePacket,
    TelemetryConformanceSupportExport, M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            let export = TelemetryConformanceSupportExport::from_packet(
                M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &TelemetryConformancePacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_lifecycle_telemetry_conformance_packet(packet) {
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
