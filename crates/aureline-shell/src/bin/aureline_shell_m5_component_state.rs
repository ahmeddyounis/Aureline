//! Headless inspector for the M5 component-state and token-inheritance audit.
//!
//! The bin emits the same audit records consumed by the live shell
//! design-QA inspector, the markdown audit under
//! `artifacts/ux/m5/component-state-audit/m5_component_state_audit.md`,
//! the support-export wrapper, and the CI gate
//! `tools/ci/m5/component_state_check.py`. It is the only mint-from-truth
//! path for the JSON fixtures checked in under
//! `fixtures/ux/m5/theme-token-consumers/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- validate
//! ```

use aureline_shell::m5_component_registry::{
    seeded_m5_component_state_audit, validate_m5_component_state_audit,
    M5ComponentStateSupportExport, M5_COMPONENT_STATE_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_m5_component_state_audit();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export = M5ComponentStateSupportExport::from_report(
                M5_COMPONENT_STATE_SUPPORT_EXPORT_ID,
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
        Some("validate") => match validate_m5_component_state_audit(&report) {
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
