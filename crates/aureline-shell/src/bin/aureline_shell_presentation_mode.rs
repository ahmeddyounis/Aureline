//! Headless inspector for the beta presentation-mode corpus.
//!
//! The bin emits the same governed presentation sessions and reversible overlay
//! projections consumed by the live shell, by the support-export wrapper, and
//! by the integration test that replays the checked-in fixtures under
//! `fixtures/help/m3/presentation_mode/`. It is the only mint-from-truth path
//! for those fixtures, so the JSON cannot drift from the Rust types.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- corpus
//! cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- sessions
//! cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- restore-outcomes
//! cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- validate
//! ```

use aureline_shell::presentation_mode::{
    seeded_presentation_mode_corpus, validate_presentation_mode_corpus,
    PresentationModeSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let corpus = seeded_presentation_mode_corpus();

    match args.first().map(String::as_str) {
        Some("corpus") | None => {
            print_json(&corpus)?;
        }
        Some("sessions") => {
            print_json(&corpus.sessions)?;
        }
        Some("restore-outcomes") => {
            print_json(&corpus.restore_outcomes)?;
        }
        Some("summary") => {
            print_json(&corpus.summary)?;
        }
        Some("support-export") => {
            let export = PresentationModeSupportExport::from_corpus(
                "support-export:presentation-mode-beta:001",
                "2026-05-20T00:00:00Z",
                &corpus,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_presentation_mode_corpus(&corpus) {
            Ok(()) => println!("ok"),
            Err(errors) => {
                for err in errors {
                    eprintln!("corpus error: {err}");
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
