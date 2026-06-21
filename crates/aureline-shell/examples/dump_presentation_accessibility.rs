//! Emits the seeded presentation-accessibility fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_presentation_accessibility -- corpus
//! cargo run -q -p aureline-shell --example dump_presentation_accessibility -- support-export
//! ```
//!
//! The `corpus` output is the literal source of
//! `fixtures/presentation/a11y-and-motion/accessibility-corpus.json`; the
//! `support-export` output is the source of the sibling
//! `accessibility-support-export.json`.

use aureline_shell::presentation::a11y::{
    presentation_a11y_support_export, seeded_presentation_a11y_corpus,
    validate_presentation_a11y_corpus,
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
    let corpus = seeded_presentation_a11y_corpus();
    validate_presentation_a11y_corpus(&corpus)
        .map_err(|err| format!("seeded corpus failed validation: {err:?}"))?;

    let json = match mode.as_str() {
        "corpus" => serde_json::to_string_pretty(&corpus)?,
        "support-export" => {
            let export = presentation_a11y_support_export(
                "support-export:presentation-a11y:001",
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
