//! Headless inspector for the M5 start-center / switcher parity packet.
//!
//! The bin emits the same packet records consumed by the live shell, by the
//! support-export wrapper, by the docs page, and by the integration that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- diagnostics
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- coverage
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- markdown
//! ```

use aureline_shell::m5_start_center_and_switcher::{
    seeded_m5_start_center_and_switcher_packet, validate_m5_start_center_and_switcher_packet,
    M5StartCenterSwitcherSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_m5_start_center_and_switcher_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => {
            print_json(&packet)?;
        }
        Some("rows") => {
            print_json(&packet.rows)?;
        }
        Some("diagnostics") => {
            print_json(&packet.diagnostics)?;
        }
        Some("coverage") => {
            print_json(&packet.surface_class_coverage)?;
        }
        Some("support-export") => {
            let export = M5StartCenterSwitcherSupportExport::from_packet(
                "support-export:m5-start-center-and-switcher:001",
                packet,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_m5_start_center_and_switcher_packet(&packet) {
            Ok(()) => {
                println!("ok");
            }
            Err(errors) => {
                for err in &errors {
                    eprintln!(
                        "error: {}",
                        serde_json::to_string(err).unwrap_or_else(|_| format!("{err:?}"))
                    );
                }
                std::process::exit(3);
            }
        },
        Some("compact") => {
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("markdown") => {
            print!("{}", packet.render_markdown());
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
