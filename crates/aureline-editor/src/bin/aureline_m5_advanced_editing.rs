//! Headless emitter for the canonical advanced-editing micro-surface model.
//!
//! Prints the one frozen, export-safe model that binds selection-summary strips,
//! multi-cursor / column-edit semantics, fold-state risk markers, and minimap /
//! overview-ruler parity into a single orientation contract across the claimed M5
//! advanced editors — selection semantics a user can always read (exact / normalized
//! / primary-only / blocked), folded regions that advertise hidden diagnostics,
//! conflicts, and trust warnings instead of appearing clean, and overview aids that
//! stay aligned with the main editor and degrade honestly — that the editor shell,
//! Help/About, support export, and AI evidence surfaces all consume. With `--lines`,
//! prints the human-readable projection instead of JSON. The model takes no input:
//! it is the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_advanced_editing          # JSON
//! cargo run --bin aureline_m5_advanced_editing -- --lines
//! ```

use aureline_editor::{advanced_editing_model, advanced_editing_model_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_advanced_editing [--lines]\n\
                     prints the canonical advanced-editing micro-surface model as JSON, or the \
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

    let model = advanced_editing_model();

    if want_lines {
        for line in advanced_editing_model_lines(&model) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&model).expect("model must serialize")
        );
    }
}
