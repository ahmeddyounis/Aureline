//! Emits the seeded cross-client presentation follow-state fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_presentation_follow_state -- corpus
//! cargo run -q -p aureline-shell --example dump_presentation_follow_state -- support-export
//! ```
//!
//! The `corpus` output is the literal source of
//! `fixtures/presentation/browser-and-companion-follow/follow-state-truth-corpus.json`;
//! the `support-export` output is the source of the sibling
//! `follow-state-truth-support-export.json`.

use aureline_shell::presentation::follow_state::{
    follow_state_support_export, seeded_follow_state_corpus, validate_follow_state_corpus,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "corpus".to_owned());
    let corpus = seeded_follow_state_corpus();
    validate_follow_state_corpus(&corpus)
        .map_err(|err| format!("seeded corpus failed validation: {err:?}"))?;

    let json = match mode.as_str() {
        "corpus" => serde_json::to_string_pretty(&corpus)?,
        "support-export" => {
            let export = follow_state_support_export(
                "support-export:presentation-follow-state:001",
                "2026-06-20T00:00:00Z",
                &corpus,
            );
            serde_json::to_string_pretty(&export)?
        }
        other => return Err(format!("unknown mode: {other}").into()),
    };
    println!("{json}");
    Ok(())
}
