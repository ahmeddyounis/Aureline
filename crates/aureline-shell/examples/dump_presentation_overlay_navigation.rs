//! Emits the seeded presentation overlay/navigation binding fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-shell --example dump_presentation_overlay_navigation -- corpus
//! cargo run -q -p aureline-shell --example dump_presentation_overlay_navigation -- support-export
//! ```
//!
//! The `corpus` output is the literal source of
//! `fixtures/presentation/overlay-and-waypoint/overlay-navigation-corpus.json`.

use aureline_shell::presentation::{
    seeded_overlay_navigation_corpus, validate_overlay_navigation_corpus,
    PresentationOverlayBindingSupportExport,
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
    let corpus = seeded_overlay_navigation_corpus();
    validate_overlay_navigation_corpus(&corpus)
        .map_err(|err| format!("seeded corpus failed validation: {err:?}"))?;

    let json = match mode.as_str() {
        "corpus" => serde_json::to_string_pretty(&corpus)?,
        "support-export" => {
            let export = PresentationOverlayBindingSupportExport::from_corpus(
                "support-export:presentation-overlay-binding:001",
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
