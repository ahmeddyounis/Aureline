//! Headless inspector for the native-desktop system-entry, handler-ownership,
//! reopen, and OS-notification matrix.
//!
//! The bin emits the same matrix records consumed by the live shell platform
//! inspector, Help/About and docs rails, and support inspector, the markdown
//! matrix under `artifacts/platform/m5-native-desktop-matrix.md`, the
//! support-export wrapper, and the CI gate
//! `tools/ci/m5/native_desktop_check.py`. It is the only mint-from-truth path
//! for the JSON fixtures checked in under
//! `fixtures/platform/m5_os_entry_and_reopen/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- validate
//! ```

use aureline_shell::m5_native_desktop::{
    seeded_native_desktop_matrix, validate_native_desktop_matrix, NativeDesktopSupportExport,
    NATIVE_DESKTOP_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_native_desktop_matrix();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export =
                NativeDesktopSupportExport::from_report(NATIVE_DESKTOP_SUPPORT_EXPORT_ID, report);
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
        Some("validate") => match validate_native_desktop_matrix(&report) {
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
