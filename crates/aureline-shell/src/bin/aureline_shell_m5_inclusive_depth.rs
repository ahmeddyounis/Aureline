//! Headless inspector for the M5 accessibility-and-locale qualification audit.
//!
//! The bin emits the same audit records consumed by the live shell
//! accessibility/locale inspector, docs/help rails, and support inspector, the
//! markdown audit under
//! `artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md`, the
//! support-export wrapper, and the CI gate
//! `tools/ci/m5/inclusive_depth_check.py`. It is the only mint-from-truth path
//! for the JSON fixtures checked in under
//! `fixtures/a11y/m5_ime_bidi_pseudoloc/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- validate
//! ```

use aureline_shell::m5_inclusive_depth::{
    seeded_m5_inclusive_depth_audit, validate_m5_inclusive_depth, M5InclusiveSupportExport,
    M5_INCLUSIVE_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_m5_inclusive_depth_audit();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export =
                M5InclusiveSupportExport::from_report(M5_INCLUSIVE_SUPPORT_EXPORT_ID, report);
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
        Some("validate") => match validate_m5_inclusive_depth(&report) {
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
