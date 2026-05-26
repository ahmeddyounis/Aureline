//! Headless emitter for the entry hardening lineage record.
//!
//! Reads a JSON envelope describing a `ProjectEntryReviewRequest` and an
//! optional pre-commit inspection-hook set, then prints the governed,
//! export-safe entry hardening lineage record. This is the CLI / replay
//! surface that proves the same projection workspace status surfaces consume.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "posture_id": "posture:example",
//!   "request": { ...ProjectEntryReviewRequest... },
//!   "inspection_hooks": [ ...EntryInspectionHook... ]
//! }
//! ```
//!
//! `inspection_hooks` is optional; when omitted the default
//! review_entry / inspect_collision / inspect_handoff / inspect_failure_repair
//! / export hook set is used. With `--lines`, prints the human-readable
//! projection instead of JSON.

use std::io::{self, Read};

use aureline_workspace::{
    build_project_entry_review, default_entry_hardening_inspection_hooks,
    entry_hardening_lineage_lines, project_entry_hardening_lineage_with_hooks,
    EntryHardeningInspectionHook, EntryHardeningLineageRecord, ProjectEntryReviewRequest,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageEmitterEnvelope {
    posture_id: String,
    request: ProjectEntryReviewRequest,
    #[serde(default)]
    inspection_hooks: Option<Vec<EntryHardeningInspectionHook>>,
}

fn main() {
    let mut want_lines = false;
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_entry_hardening_lineage [--lines] [envelope.json]\n\
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

    let hooks = envelope
        .inspection_hooks
        .unwrap_or_else(default_entry_hardening_inspection_hooks);

    let entry_review = build_project_entry_review(envelope.request);
    let record: EntryHardeningLineageRecord =
        project_entry_hardening_lineage_with_hooks(envelope.posture_id, &entry_review, hooks);

    if want_lines {
        for line in entry_hardening_lineage_lines(&record) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("lineage record must serialize")
        );
    }
}
