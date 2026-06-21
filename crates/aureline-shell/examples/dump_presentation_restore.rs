//! Emits the seeded presentation-restore fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_presentation_restore -- corpus
//! cargo run -q -p aureline-shell --example dump_presentation_restore -- support-export
//! ```
//!
//! The `corpus` output is the literal source of
//! `fixtures/presentation/restore-no-rerun/restore-report-corpus.json`; the
//! `support-export` output is the source of the sibling
//! `restore-report-support-export.json`.

use aureline_shell::presentation::presentation_restore::{
    presentation_restore_support_export, seeded_presentation_restore_corpus,
    validate_presentation_restore_corpus,
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
    let corpus = seeded_presentation_restore_corpus();
    validate_presentation_restore_corpus(&corpus)
        .map_err(|err| format!("seeded corpus failed validation: {err:?}"))?;

    let json = match mode.as_str() {
        "corpus" => serde_json::to_string_pretty(&corpus)?,
        "support-export" => {
            let export = presentation_restore_support_export(
                "support-export:presentation-restore:001",
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
