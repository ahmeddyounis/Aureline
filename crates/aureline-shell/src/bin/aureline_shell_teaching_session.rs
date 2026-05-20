//! Headless inspector for the beta teaching/classroom conformance corpus.
//!
//! The bin emits the same governed teaching sessions and role-aware affordance
//! projections consumed by the live shell, by the support-export wrapper, and by
//! the integration test that replays the checked-in fixtures under
//! `fixtures/help/m3/teaching_classroom/`. It is the only mint-from-truth path
//! for those fixtures, so the JSON cannot drift from the Rust types.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- corpus
//! cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- sessions
//! cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- restore-outcomes
//! cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- validate
//! ```

use aureline_shell::teaching_session::{
    seeded_teaching_classroom_corpus, validate_teaching_classroom_corpus,
    TeachingClassroomSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let corpus = seeded_teaching_classroom_corpus();

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
            let export = TeachingClassroomSupportExport::from_corpus(
                "support-export:teaching-classroom-beta:001",
                "2026-05-20T00:00:00Z",
                &corpus,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_teaching_classroom_corpus(&corpus) {
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
