//! Headless inspector for the system-open and file-association intake report.
//!
//! The bin emits the same intake records consumed by the live shell entry
//! interstitials and Start Center, the Help/About and docs rails, and the
//! support inspector; the markdown artifact under
//! `artifacts/platform/m5-system-open-and-file-association.md`; the
//! support-export wrapper; the four per-incident case exports; and the CI gate
//! `tools/ci/m5/system_entry_check.py`. It is the only mint-from-truth path for
//! the JSON fixtures checked in under `fixtures/platform/m5-system-entry/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- case wrong_association
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- validate
//! ```

use aureline_shell::m5_system_entry::{
    seeded_system_entry_case_exports, seeded_system_entry_report, validate_system_entry_report,
    SystemEntrySupportExport, SYSTEM_ENTRY_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_system_entry_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export =
                SystemEntrySupportExport::from_report(SYSTEM_ENTRY_SUPPORT_EXPORT_ID, report);
            print_json(&export)?;
        }
        Some("cases") => {
            print_json(&seeded_system_entry_case_exports())?;
        }
        Some("case") => {
            let label = args.get(1).map(String::as_str).ok_or(
                "usage: aureline_shell_m5_system_entry case <wrong_association|moved_target|mixed_root|policy_blocked>",
            )?;
            let exports = seeded_system_entry_case_exports();
            let found = exports
                .into_iter()
                .find(|export| export.case_label == label)
                .ok_or_else(|| format!("unknown case label: {label}"))?;
            print_json(&found)?;
        }
        Some("report-md") => {
            print!("{}", report.render_markdown());
        }
        Some("compact") => {
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match validate_system_entry_report(&report) {
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
