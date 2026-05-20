//! Headless inspector for the bounded voice-preview surface.
//!
//! The bin emits the same records consumed by the live shell voice
//! preview inspector, the published markdown under
//! `artifacts/ux/m3/voice_preview_beta.md`, the support-export wrapper,
//! and the CI gate `tools/ci/m3/voice_preview_check.py`. It is the only
//! mint-from-truth path for the JSON fixtures checked in under
//! `fixtures/ux/m3/voice_preview_and_privacy/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- validate
//! ```

use aureline_shell::voice::{
    seeded_voice_preview_beta_page, validate_voice_preview_beta_page, VoicePreviewSupportExport,
    VOICE_PREVIEW_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_voice_preview_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("support-export") => {
            let export =
                VoicePreviewSupportExport::from_page(VOICE_PREVIEW_SUPPORT_EXPORT_ID, page);
            print_json(&export)?;
        }
        Some("report-md") => {
            print!("{}", page.render_markdown());
        }
        Some("compact") => {
            for line in page.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match validate_voice_preview_beta_page(&page) {
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
