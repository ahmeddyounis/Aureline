//! Headless inspector for the M5 appearance-object certification capstone.
//!
//! The bin emits the same packet records consumed by the release/evidence
//! center, the support-export wrapper, the docs page, the published report
//! artifact, and the integration that replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- surfaces
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- index
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- live-build
//! ```
//!
//! The `live-build` subcommand prints the *actual* exact-build identity this
//! binary was compiled against (via `aureline_build_info`) — the value a runtime
//! would certify against. Every other subcommand uses the frozen seed so the
//! checked-in fixtures stay reproducible.

use aureline_shell::appearance_object_certification::{
    seeded_appearance_object_certification_report, validate_appearance_object_certification_report,
    AppearanceObjectCertificationSupportExport, M5_APPEARANCE_CERT_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_appearance_object_certification_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("surfaces") => {
            print_json(&report.surfaces)?;
        }
        Some("index") => {
            print_json(&report.object_model_index)?;
        }
        Some("support-export") => {
            let export = AppearanceObjectCertificationSupportExport::from_report(
                M5_APPEARANCE_CERT_SUPPORT_EXPORT_ID,
                report,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_appearance_object_certification_report(&report) {
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
