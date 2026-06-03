//! Headless emitter for the workspace archetype readiness preflight record.
//!
//! Reads a JSON envelope describing an underlying `AdmissionCheckpointRouteRecord`
//! plus metadata, then prints the governed, export-safe stable preflight record.
//! This is the CLI / replay surface that proves the same projection shell,
//! diagnostics, and support surfaces consume.
//!
//! Input envelope (stdin, or a single file-path argument):
//!
//! ```json
//! {
//!   "record_id": "preflight:example",
//!   "as_of": "2026-06-02T12:00:00Z",
//!   "underlying": { ...AdmissionCheckpointRouteRecord... },
//!   "diagnostics_export_ref": "aureline://diagnostics/example",
//!   "support_export_ref": "aureline://support-export/example",
//!   "evidence_refs": ["aureline://artifact/example"],
//!   "narrative_refs": ["aureline://doc/example"]
//! }
//! ```
//!
//! With `--lines`, prints the human-readable projection instead of JSON.
//! With `--corpus`, prints the deterministic claimed-stable matrix as a JSON
//! array.

use std::io::{self, Read};

use aureline_workspace::{
    AdmissionCheckpointRouteRecord, PreflightInput, WorkspaceArchetypeReadinessPreflightRecord,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PreflightEmitterEnvelope {
    record_id: String,
    as_of: String,
    underlying: AdmissionCheckpointRouteRecord,
    diagnostics_export_ref: String,
    support_export_ref: String,
    #[serde(default)]
    evidence_refs: Vec<String>,
    #[serde(default)]
    narrative_refs: Vec<String>,
}

fn main() {
    let mut want_lines = false;
    let mut want_corpus = false;
    let mut path_arg: Option<String> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--corpus" => want_corpus = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_stabilize_workspace_archetype_detection_readiness_preflight \
                     [--lines] [--corpus] [envelope.json]\n\
                     reads the preflight envelope from stdin when no path is given.\n\
                     --corpus emits the deterministic claimed-stable matrix."
                );
                return;
            }
            other => path_arg = Some(other.to_owned()),
        }
    }

    if want_corpus {
        let corpus = aureline_workspace::workspace_archetype_readiness_preflight_corpus();
        let records: Vec<WorkspaceArchetypeReadinessPreflightRecord> =
            corpus.iter().map(|s| s.record()).collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&records).expect("corpus must serialize")
        );
        return;
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

    let envelope: PreflightEmitterEnvelope = serde_json::from_str(&raw).unwrap_or_else(|err| {
        eprintln!("failed to parse preflight envelope: {err}");
        std::process::exit(2);
    });

    let record = WorkspaceArchetypeReadinessPreflightRecord::build(PreflightInput {
        record_id: envelope.record_id,
        as_of: envelope.as_of,
        underlying: envelope.underlying,
        diagnostics_export_ref: envelope.diagnostics_export_ref,
        support_export_ref: envelope.support_export_ref,
        evidence_refs: envelope.evidence_refs,
        narrative_refs: envelope.narrative_refs,
    })
    .unwrap_or_else(|err| {
        eprintln!("preflight record build failed: {err}");
        std::process::exit(2);
    });

    if want_lines {
        for line in record.support_export_lines() {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("preflight record must serialize")
        );
    }
}
