//! Headless inspector for the native-desktop per-desktop-profile qualification
//! family and its auto-narrowing claim packet.
//!
//! The bin emits the same qualification records consumed by the live shell
//! platform inspector, Help/About, install/update, docs, and support rails, the
//! markdown matrix under
//! `artifacts/platform/m5-native-desktop-qualification/`, the support-export
//! wrapper, the shiproom claim packet under
//! `artifacts/shiproom/m5-native-desktop-claim-packet/`, and the CI gate
//! `tools/ci/m5/native_desktop_qualification_check.py`. It is the only
//! mint-from-truth path for the JSON fixtures checked in under
//! `fixtures/platform/m5-native-desktop-qualification/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- claim-packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- claim-packet-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- validate
//! ```

use aureline_shell::m5_native_desktop_qualification::{
    seeded_native_desktop_qualification, seeded_qualification_claim_packet,
    seeded_qualification_support_export, validate_qualification_report,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_native_desktop_qualification();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            print_json(&seeded_qualification_support_export())?;
        }
        Some("claim-packet") => {
            print_json(&seeded_qualification_claim_packet())?;
        }
        Some("report-md") => {
            print!("{}", report.render_markdown());
        }
        Some("claim-packet-md") => {
            print!("{}", seeded_qualification_claim_packet().render_markdown());
        }
        Some("compact") => {
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match validate_qualification_report(&report) {
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
