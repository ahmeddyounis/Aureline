//! Headless emitter for the source-fidelity save lineage record.
//!
//! Reads a JSON envelope describing a staged-save participant risk review and
//! the open-time source-fidelity record, then prints the governed, export-safe
//! save-fidelity lineage record. This is the CLI / replay surface that proves
//! the same projection the editor save-status surface consumes.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "lineage_id": "lineage:example",
//!   "risk_review": { ...SaveParticipantRiskReview... },
//!   "source_fidelity": { ...SourceFidelityRecord... }
//! }
//! ```
//!
//! With `--lines`, prints the human-readable projection instead of JSON.

use std::io::{self, Read};

use aureline_editor::{
    project_save_fidelity_lineage, save_fidelity_lineage_lines, SaveFidelityLineageRecord,
};
use aureline_workspace::save::{SaveParticipantRiskReview, SourceFidelityRecord};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageEmitterEnvelope {
    lineage_id: String,
    risk_review: SaveParticipantRiskReview,
    source_fidelity: SourceFidelityRecord,
}

fn main() {
    let mut want_lines = false;
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_save_fidelity_lineage [--lines] [envelope.json]\n\
                     reads the lineage envelope from stdin when no path is given."
                );
                return;
            }
            other => path_arg = Some(other.to_owned()),
        }
    }

    let raw = match path_arg {
        Some(path) => std::fs::read_to_string(&path).unwrap_or_else(|err| {
            eprintln!("failed to read {path}: {err}");
            std::process::exit(2);
        }),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("stdin must be readable");
            buf
        }
    };

    let envelope: LineageEmitterEnvelope = serde_json::from_str(&raw).unwrap_or_else(|err| {
        eprintln!("failed to parse lineage envelope: {err}");
        std::process::exit(2);
    });

    let record: SaveFidelityLineageRecord = project_save_fidelity_lineage(
        envelope.lineage_id,
        &envelope.risk_review,
        &envelope.source_fidelity,
    );

    if want_lines {
        for line in save_fidelity_lineage_lines(&record) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("lineage record must serialize")
        );
    }
}
