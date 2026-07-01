//! Headless emitter for the M5 window-lifecycle-safety proof.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard, support
//! export, and CSV checked in under `artifacts/release/m5-window-lifecycle-safety-proof/`
//! and the markdown report under `artifacts/shell/m5-window-lifecycle-safety.md`, plus the
//! protected fixtures under `fixtures/ui/m5-window-lifecycle-safety/`. Shell / windowing /
//! layout / status automation, release-center, docs/help, and support exports resolve the
//! same lifecycle rows — green/yellow/red status, active waivers, and the exact lifecycle
//! causes — through this proof rather than restating drag-verb, close-orphan, or reopen
//! posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- validate
//! ```

use aureline_shell::m5_window_lifecycle_safety::{
    seeded_m5_window_lifecycle_safety_packet, validate_m5_window_lifecycle_safety_packet,
    WindowLifecyclePacket, WindowLifecycleSupportExport,
    M5_WINDOW_LIFECYCLE_SAFETY_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            let export = WindowLifecycleSupportExport::from_packet(
                M5_WINDOW_LIFECYCLE_SAFETY_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_window_lifecycle_safety_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &WindowLifecyclePacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_window_lifecycle_safety_packet(packet) {
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
