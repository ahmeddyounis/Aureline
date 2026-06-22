//! Headless emitter for the canonical editor-assist matrix.
//!
//! Prints the one frozen, export-safe editor-assist micro-surface matrix that
//! the editor shell, Help/About, support export, and AI evidence surfaces all
//! consume. With `--lines`, prints the human-readable projection instead of
//! JSON. The matrix takes no input: it is the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_editor_assist          # JSON
//! cargo run --bin aureline_m5_editor_assist -- --lines
//! ```

use aureline_editor::{editor_assist_matrix, editor_assist_matrix_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_editor_assist [--lines]\n\
                     prints the canonical editor-assist matrix as JSON, or the \
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

    let matrix = editor_assist_matrix();

    if want_lines {
        for line in editor_assist_matrix_lines(&matrix) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&matrix).expect("matrix must serialize")
        );
    }
}
