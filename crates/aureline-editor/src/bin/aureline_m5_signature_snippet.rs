//! Headless emitter for the canonical signature-help and snippet-session model.
//!
//! Prints the one frozen, export-safe model that binds signature-help cards and
//! snippet-session strips into a single typing-loop contract across the claimed
//! editor families — active overload / parameter, placeholder count and exit
//! path, IME / multi-cursor coherence, no-hidden-side-effects disclosure, and
//! per-surface blocked / degraded reasons — that the editor shell, Help/About,
//! support export, and AI evidence surfaces all consume. With `--lines`, prints
//! the human-readable projection instead of JSON. The model takes no input: it is
//! the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_signature_snippet          # JSON
//! cargo run --bin aureline_m5_signature_snippet -- --lines
//! ```

use aureline_editor::{signature_snippet_model, signature_snippet_model_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_signature_snippet [--lines]\n\
                     prints the canonical signature-help / snippet-session model as JSON, or the \
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

    let model = signature_snippet_model();

    if want_lines {
        for line in signature_snippet_model_lines(&model) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&model).expect("model must serialize")
        );
    }
}
