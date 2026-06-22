//! Headless emitter for the canonical hover-card and documentation-peek model.
//!
//! Prints the one frozen, export-safe model that binds transient hovercards,
//! documentation peeks, and pinned / open-in-tab / open-in-split peek promotion
//! into a single contextual-inspection contract across the claimed inspection
//! contexts — symbol / anchor identity that never silently retargets, source /
//! provider / freshness provenance, mapping quality, raw-versus-rendered truth, and
//! inline non-live state disclosure — that the editor shell, Help/About, support
//! export, and AI evidence surfaces all consume. With `--lines`, prints the
//! human-readable projection instead of JSON. The model takes no input: it is the
//! single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_hover_peek          # JSON
//! cargo run --bin aureline_m5_hover_peek -- --lines
//! ```

use aureline_editor::{hover_peek_model, hover_peek_model_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_hover_peek [--lines]\n\
                     prints the canonical hover-card / documentation-peek model as JSON, or the \
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

    let model = hover_peek_model();

    if want_lines {
        for line in hover_peek_model_lines(&model) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&model).expect("model must serialize")
        );
    }
}
