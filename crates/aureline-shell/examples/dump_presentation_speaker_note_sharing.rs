//! Emits the seeded presentation speaker-note sharing fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_presentation_speaker_note_sharing -- corpus
//! cargo run -q -p aureline-shell --example dump_presentation_speaker_note_sharing -- support-export
//! ```
//!
//! The `corpus` output is the literal source of
//! `fixtures/presentation/speaker-note-sharing/speaker-note-sharing-corpus.json`;
//! the `support-export` output is the source of the sibling
//! `speaker-note-sharing-support-export.json`.

use aureline_shell::presentation::speaker_notes::{
    seeded_speaker_note_sharing_corpus, speaker_note_sharing_support_export,
    validate_speaker_note_sharing_corpus,
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
    let corpus = seeded_speaker_note_sharing_corpus();
    validate_speaker_note_sharing_corpus(&corpus)
        .map_err(|err| format!("seeded corpus failed validation: {err:?}"))?;

    let json = match mode.as_str() {
        "corpus" => serde_json::to_string_pretty(&corpus)?,
        "support-export" => {
            let export = speaker_note_sharing_support_export(
                "support-export:presentation-speaker-note-sharing:001",
                "2026-06-20T00:00:00Z",
                &corpus,
            );
            let violations = export.validate();
            if !violations.is_empty() {
                return Err(format!("support export failed validation: {violations:?}").into());
            }
            serde_json::to_string_pretty(&export)?
        }
        other => return Err(format!("unknown mode: {other}").into()),
    };
    println!("{json}");
    Ok(())
}
