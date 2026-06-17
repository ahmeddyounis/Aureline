//! Headless inspector for the M5 OS-attention parity audit.
//!
//! The bin emits the same audit records consumed by the live shell notification
//! router / dock-badge / taskbar-progress / About surfaces, the markdown audit
//! under `artifacts/ux/m5/os-notification-and-reopen.md`, the support-export
//! wrapper, and the CI gate
//! `tools/ci/m5/os_notifications_and_badges_check.py`. It is the only
//! mint-from-truth path for the JSON fixtures checked in under
//! `fixtures/ux/m5_os_notifications_and_badges/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- validate
//! ```

use aureline_shell::m5_os_notifications_and_badges::{
    seeded_m5_os_attention_report, validate_m5_os_attention_report, M5OsAttentionSupportExport,
    M5_OS_ATTENTION_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_m5_os_attention_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export =
                M5OsAttentionSupportExport::from_report(M5_OS_ATTENTION_SUPPORT_EXPORT_ID, report);
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
        Some("validate") => match validate_m5_os_attention_report(&report) {
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
