//! Headless emitter for the large-file posture record.
//!
//! Reads a JSON envelope describing the at-open large-file classification
//! observation and the limited-mode file record, then prints the governed,
//! export-safe large-file posture record. This is the CLI / replay surface that
//! proves the same projection the editor large-file status surface consumes.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "posture_id": "posture:example",
//!   "classification": { ...LargeFileClassificationObservation... },
//!   "limited_mode": { ...LimitedModeFileRecord... },
//!   "inspection_hooks": [ ...InspectionHook... ]
//! }
//! ```
//!
//! `inspection_hooks` is optional; when omitted the default compare / checkpoint
//! / export / repair hook set is used. With `--lines`, prints the human-readable
//! projection instead of JSON.

use std::io::{self, Read};

use aureline_editor::large_file_mode::LimitedModeFileRecord;
use aureline_editor::{
    default_large_file_inspection_hooks, large_file_posture_lines,
    project_large_file_posture_with_hooks, InspectionHook, LargeFileClassificationObservation,
    LargeFilePostureRecord,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PostureEmitterEnvelope {
    posture_id: String,
    classification: LargeFileClassificationObservation,
    limited_mode: LimitedModeFileRecord,
    #[serde(default)]
    inspection_hooks: Option<Vec<InspectionHook>>,
}

fn main() {
    let mut want_lines = false;
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_large_file_posture [--lines] [envelope.json]\n\
                     reads the posture envelope from stdin when no path is given."
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

    let envelope: PostureEmitterEnvelope = serde_json::from_str(&raw).unwrap_or_else(|err| {
        eprintln!("failed to parse posture envelope: {err}");
        std::process::exit(2);
    });

    let hooks = envelope
        .inspection_hooks
        .unwrap_or_else(default_large_file_inspection_hooks);

    let record: LargeFilePostureRecord = project_large_file_posture_with_hooks(
        envelope.posture_id,
        &envelope.classification,
        &envelope.limited_mode,
        hooks,
    );

    if want_lines {
        for line in large_file_posture_lines(&record) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("posture record must serialize")
        );
    }
}
