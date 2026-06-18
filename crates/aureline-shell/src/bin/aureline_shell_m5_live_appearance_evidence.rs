//! Headless inspector for the M5 live-appearance change & evidence-linkage
//! report.
//!
//! The bin emits the same packet records consumed by the live release/evidence
//! center, the support-export wrapper, the docs page, the published report
//! artifact, and the integration that replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- coverage
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- live-build
//! ```
//!
//! The `live-build` subcommand prints the *actual* exact-build identity this
//! binary was compiled against (via `aureline_build_info`) — the value a runtime
//! would stamp into each capture attribution. Every other subcommand uses the
//! frozen seed so the checked-in fixtures stay reproducible.

use aureline_shell::live_appearance_evidence::{
    seeded_live_appearance_evidence_report, validate_live_appearance_evidence_report,
    LiveAppearanceEvidenceSupportExport, M5_LIVE_APPEARANCE_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_live_appearance_evidence_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("rows") => {
            print_json(&report.rows)?;
        }
        Some("coverage") => {
            print_json(&report.axis_platform_coverage)?;
        }
        Some("support-export") => {
            let export = LiveAppearanceEvidenceSupportExport::from_report(
                M5_LIVE_APPEARANCE_SUPPORT_EXPORT_ID,
                report,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_live_appearance_evidence_report(&report) {
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
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("markdown") => {
            print!("{}", report.render_markdown());
        }
        Some("live-build") => {
            println!("{}", aureline_build_info::exact_build_identity_ref());
            println!("{}", aureline_build_info::release_channel_class());
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
