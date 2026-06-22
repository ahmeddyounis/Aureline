//! Headless emitter for the canonical constrained-file and degraded-provider
//! assist-narrowing model.
//!
//! Prints the one frozen, export-safe model that states how every editor assist
//! micro-surface narrows, downgrades, blocks, or routes elsewhere when file state
//! or provider certainty means Aureline cannot safely offer the same completion /
//! hint / hover / refactor behavior it offers on an ordinary source file —
//! per-state, per-channel degraded-state verdicts with inspectable reasons and
//! next-safe-action routes, degraded-provider cases, and consumer-surface proofs —
//! that the editor shell, Help/About, support export, and AI evidence surfaces all
//! consume. With `--lines`, prints the human-readable projection instead of JSON.
//! The model takes no input: it is the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_constrained_assist          # JSON
//! cargo run --bin aureline_m5_constrained_assist -- --lines
//! ```

use aureline_editor::{constrained_assist_model, constrained_assist_model_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_constrained_assist [--lines]\n\
                     prints the canonical constrained-file / degraded-provider assist-narrowing \
                     model as JSON, or the human-readable projection with --lines."
                );
                return;
            }
            other => {
                eprintln!("unexpected argument: {other}");
                std::process::exit(2);
            }
        }
    }

    let model = constrained_assist_model();

    if want_lines {
        for line in constrained_assist_model_lines(&model) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&model).expect("model must serialize")
        );
    }
}
