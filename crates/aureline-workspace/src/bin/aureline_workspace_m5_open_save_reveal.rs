//! Headless inspector for the open/save/reveal path-truth report.
//!
//! The bin emits the same flow records consumed by the live save/reveal
//! affordances, the Help/About and docs rails, and the support inspector; the
//! markdown artifact under `artifacts/platform/m5-open-save-reveal.md`; the
//! support-export wrapper; the four per-incident case exports; and the CI gate
//! `tools/ci/m5/open_save_reveal_check.py`. It is the only mint-from-truth path
//! for the JSON fixtures checked in under
//! `fixtures/platform/m5-open-save-reveal/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- report
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- support-export
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- cases
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- case network_share_alias
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- report-md
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- compact
//! cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- validate
//! ```

use aureline_workspace::m5_open_save_reveal::{
    seeded_open_save_reveal_case_exports, seeded_open_save_reveal_report,
    validate_open_save_reveal_report, OpenSaveRevealSupportExport,
    OPEN_SAVE_REVEAL_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_open_save_reveal_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export = OpenSaveRevealSupportExport::from_report(
                OPEN_SAVE_REVEAL_SUPPORT_EXPORT_ID,
                report,
            );
            print_json(&export)?;
        }
        Some("cases") => {
            print_json(&seeded_open_save_reveal_case_exports())?;
        }
        Some("case") => {
            let label = args.get(1).map(String::as_str).ok_or(
                "usage: aureline_workspace_m5_open_save_reveal case <missing_canonical_target|network_share_alias|generated_output|read_only_destination>",
            )?;
            let exports = seeded_open_save_reveal_case_exports();
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
        Some("validate") => match validate_open_save_reveal_report(&report) {
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
