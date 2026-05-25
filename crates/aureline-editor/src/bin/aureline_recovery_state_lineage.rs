//! Headless emitter for the recovery-state lineage record.
//!
//! Reads a JSON envelope describing a dirty-buffer autosave journal entry, the
//! observed buffer undo/redo grouping, and the local-history actor-lineage
//! packet, then prints the governed, export-safe recovery-state lineage record.
//! This is the CLI / replay surface that proves the same projection the editor
//! recovery-status surface consumes.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "lineage_id": "lineage:example",
//!   "buffer_recovery": { ...AutosaveJournalEntryRecord... },
//!   "undo_groups": [ ...UndoGroupObservation... ],
//!   "local_history": { ...LocalHistoryAlphaPacket... }
//! }
//! ```
//!
//! With `--lines`, prints the human-readable projection instead of JSON.

use std::io::{self, Read};

use aureline_editor::{
    project_recovery_state_lineage, recovery_state_lineage_lines, RecoveryStateLineageRecord,
    UndoGroupObservation,
};
use aureline_history::LocalHistoryAlphaPacket;
use aureline_recovery::crash_journal::AutosaveJournalEntryRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageEmitterEnvelope {
    lineage_id: String,
    buffer_recovery: AutosaveJournalEntryRecord,
    #[serde(default)]
    undo_groups: Vec<UndoGroupObservation>,
    local_history: LocalHistoryAlphaPacket,
}

fn main() {
    let mut want_lines = false;
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_recovery_state_lineage [--lines] [envelope.json]\n\
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

    let record: RecoveryStateLineageRecord = project_recovery_state_lineage(
        envelope.lineage_id,
        &envelope.buffer_recovery,
        &envelope.undo_groups,
        &envelope.local_history,
    );

    if want_lines {
        for line in recovery_state_lineage_lines(&record) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("lineage record must serialize")
        );
    }
}
