//! Headless emitter for the canonical completion-row model.
//!
//! Prints the one frozen, export-safe completion-row model — source-labeled,
//! commit-honest rows with their deterministic-versus-AI assist class, trust
//! weight, additional-edit/import cue, availability, and per-surface degraded
//! provider posture across the claimed editor families — that the editor shell,
//! Help/About, support export, and AI evidence surfaces all consume. With
//! `--lines`, prints the human-readable projection instead of JSON. The model
//! takes no input: it is the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_completion_rows          # JSON
//! cargo run --bin aureline_m5_completion_rows -- --lines
//! ```

use aureline_editor::{completion_row_model, completion_row_model_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_completion_rows [--lines]\n\
                     prints the canonical completion-row model as JSON, or the \
                     human-readable projection with --lines."
                );
                return;
            }
            other => {
                eprintln!("unexpected argument: {other}");
                std::process::exit(2);
            }
        }
    }

    let model = completion_row_model();

    if want_lines {
        for line in completion_row_model_lines(&model) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&model).expect("model must serialize")
        );
    }
}
