//! Headless inspector for the M5 appearance-session runtime audit.
//!
//! The bin emits the same records consumed by the live shell appearance
//! inspector, the markdown audit under
//! `artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md`,
//! the support-export wrapper, and the CI gate
//! `tools/ci/m5/appearance_session_check.py`. It is the only mint-from-truth
//! path for the JSON fixtures checked in under
//! `fixtures/ux/m5/live-appearance-change/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- validate
//! ```

use aureline_shell::appearance_session::{
    seeded_appearance_session_runtime, validate_appearance_session_runtime,
    AppearanceSessionSupportExport, APPEARANCE_SESSION_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_appearance_session_runtime();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export = AppearanceSessionSupportExport::from_report(
                APPEARANCE_SESSION_SUPPORT_EXPORT_ID,
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
        Some("validate") => match validate_appearance_session_runtime(&report) {
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
