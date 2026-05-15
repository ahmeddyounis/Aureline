//! Headless inspector for the first-run task-success packet.
//!
//! The bin emits the same packet records consumed by the live shell,
//! by the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- telemetry
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- markdown
//! ```

use aureline_shell::onboarding_metrics::{
    seeded_first_run_task_success_packet, validate_first_run_task_success_packet,
    FirstRunTaskSuccessSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_first_run_task_success_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => {
            print_json(&packet)?;
        }
        Some("rows") => {
            print_json(&packet.rows)?;
        }
        Some("summary") => {
            print_json(&packet.state_summary)?;
        }
        Some("telemetry") => {
            print_json(&packet.telemetry_capture)?;
        }
        Some("support-export") => {
            let export = FirstRunTaskSuccessSupportExport::from_packet(
                "support-export:first-run-task-success:001",
                packet,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_first_run_task_success_packet(&packet) {
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
