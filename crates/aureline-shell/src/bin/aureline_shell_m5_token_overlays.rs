//! Headless inspector for the M5 token-overlay round-trip audit.
//!
//! The bin emits the same records consumed by the live shell appearance
//! inspector, the markdown audit under
//! `artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md`,
//! the support-export wrapper, and the CI gate
//! `tools/ci/m5/token_overlay_check.py`. It is the only mint-from-truth path for
//! the JSON fixtures checked in under
//! `fixtures/ux/m5/token-overlay-sync-import/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- validate
//! ```

use aureline_shell::token_overlays::{
    seeded_token_overlay_portability, validate_token_overlay_portability,
    TokenOverlaySupportExport, TOKEN_OVERLAY_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_token_overlay_portability();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("support-export") => {
            let export =
                TokenOverlaySupportExport::from_report(TOKEN_OVERLAY_SUPPORT_EXPORT_ID, report);
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
        Some("validate") => match validate_token_overlay_portability(&report) {
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
