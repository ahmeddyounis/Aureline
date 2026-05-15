//! Headless inspector for the beta command-parity diff report.
//!
//! The bin emits the same report records consumed by the live shell
//! parity inspector, the markdown report under
//! `artifacts/ux/m3/command_parity_diff_report.md`, the support-export
//! wrapper, and the CI gate `tools/ci/m3/command_parity_check.py`. It
//! is the only mint-from-truth path for the JSON fixtures checked in
//! under `fixtures/commands/m3/command_parity/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- validate
//! ```

use aureline_shell::command_parity::{
    seeded_command_parity_diff_report, validate_command_parity_diff_report,
    BetaCommandParitySupportExport, COMMAND_PARITY_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_command_parity_diff_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export = BetaCommandParitySupportExport::from_report(
                COMMAND_PARITY_SUPPORT_EXPORT_ID,
                report,
            );
            print_json(&export)?;
        }
        Some("report-md") => {
            print!("{}", report.render_markdown());
        }
        Some("compact") => {
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match validate_command_parity_diff_report(&report) {
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
