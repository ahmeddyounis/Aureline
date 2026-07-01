//! Headless emitter for the M5 owning-window routing proof.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard,
//! support export, and CSV checked in under
//! `artifacts/release/m5-owning-window-routing-proof/` and the markdown report under
//! `artifacts/shell/m5-owning-window-routing.md`, plus the protected fixtures under
//! `fixtures/ui/m5-owning-window-routing/`. Shell / windowing / notification / release
//! automation, release-center, docs/help, and support exports resolve the same routing
//! rows — green/yellow/red status, active waivers, and the exact routing causes —
//! through this proof rather than restating dialog-binding, reopen, focus, or
//! OS-notification posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- validate
//! ```

use aureline_shell::m5_owning_window_routing::{
    seeded_m5_owning_window_routing_packet, validate_m5_owning_window_routing_packet,
    RoutingContinuityPacket, RoutingContinuitySupportExport,
    M5_OWNING_WINDOW_ROUTING_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            let export = RoutingContinuitySupportExport::from_packet(
                M5_OWNING_WINDOW_ROUTING_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_owning_window_routing_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &RoutingContinuityPacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_owning_window_routing_packet(packet) {
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
