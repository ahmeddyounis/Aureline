//! Headless emitter for the canonical assist-descriptor model.
//!
//! Prints the one frozen, export-safe decoration / code-lens / inlay-hint
//! descriptor model — typed descriptors plus their resolved precedence,
//! suppression reasons, and accessibility truth across editor surfaces — that
//! the editor shell, Help/About, support export, and AI evidence surfaces all
//! consume. With `--lines`, prints the human-readable projection instead of
//! JSON. The model takes no input: it is the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_assist_descriptors          # JSON
//! cargo run --bin aureline_m5_assist_descriptors -- --lines
//! ```

use aureline_editor::{assist_descriptor_model, assist_descriptor_model_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_assist_descriptors [--lines]\n\
                     prints the canonical assist-descriptor model as JSON, or the \
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

    let model = assist_descriptor_model();

    if want_lines {
        for line in assist_descriptor_model_lines(&model) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&model).expect("model must serialize")
        );
    }
}
