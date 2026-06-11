//! Headless inspector for the M5 command-parity and discoverability audit.
//!
//! The bin emits the same audit records consumed by the live shell
//! discoverability inspector, the markdown audit under
//! `artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md`,
//! the support-export wrapper, and the CI gate
//! `tools/ci/m5/command_parity_check.py`. It is the only mint-from-truth
//! path for the JSON fixtures checked in under
//! `fixtures/ux/m5/command-parity/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- validate
//! ```

use aureline_shell::m5_command_registry::{
    seeded_m5_command_parity_audit, validate_m5_command_parity_audit, M5CommandParitySupportExport,
    M5_COMMAND_PARITY_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_m5_command_parity_audit();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export = M5CommandParitySupportExport::from_report(
                M5_COMMAND_PARITY_SUPPORT_EXPORT_ID,
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
        Some("validate") => match validate_m5_command_parity_audit(&report) {
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
