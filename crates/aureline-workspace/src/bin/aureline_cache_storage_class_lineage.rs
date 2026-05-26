//! Headless emitter for the cache / storage-class governance lineage
//! record.
//!
//! Reads a JSON envelope carrying the cache / storage inputs (and
//! optional inspection hooks) and prints the governed, export-safe
//! cache / storage-class lineage record. This is the CLI / replay
//! surface that proves the same projection the workspace cache /
//! storage status surface consumes.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "posture_id": "posture:example",
//!   "inputs": { ...CacheStorageClassInputs... },
//!   "inspection_hooks": [ ...CacheStorageInspectionHook... ]
//! }
//! ```
//!
//! `inspection_hooks` is optional; when omitted the default hook set
//! is used. With `--lines`, prints the human-readable projection
//! instead of JSON.

use std::io::{self, Read};

use aureline_workspace::{
    cache_storage_class_lineage_lines, default_cache_storage_inspection_hooks,
    project_cache_storage_class_lineage_with_hooks, CacheStorageClassInputs,
    CacheStorageClassLineageRecord, CacheStorageInspectionHook,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageEmitterEnvelope {
    posture_id: String,
    inputs: CacheStorageClassInputs,
    #[serde(default)]
    inspection_hooks: Option<Vec<CacheStorageInspectionHook>>,
}

fn main() {
    let mut want_lines = false;
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_cache_storage_class_lineage [--lines] [envelope.json]\n\
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
        .unwrap_or_else(default_cache_storage_inspection_hooks);

    let record: CacheStorageClassLineageRecord = project_cache_storage_class_lineage_with_hooks(
        envelope.posture_id,
        &envelope.inputs,
        hooks,
    );

    if want_lines {
        for line in cache_storage_class_lineage_lines(&record) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("lineage record must serialize")
        );
    }
}
