//! Headless emitter for the M5 attention-routing matrix.
//!
//! Prints the canonical matrix that freezes Aureline's attention routing —
//! notification envelopes, durable activity objects, badge aggregates, fanout
//! receipts, routing context, privacy classes, and action/retention semantics.
//! The shell activity center, OS notifications, dock/taskbar badge, browser and
//! mobile companions, operator dashboard, support export, Help/About, and
//! CLI/headless consumers render this matrix instead of restating the
//! attention-routing contract by hand. With `--lines`, prints the human-readable
//! projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_attention_routing            # JSON
//! cargo run -p aureline-activity --example dump_m5_attention_routing -- --lines
//! ```

use aureline_activity::m5_attention_routing::{attention_routing_lines, attention_routing_matrix};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let matrix = attention_routing_matrix();
    matrix
        .validate()
        .expect("canonical attention-routing matrix validates");

    if want_lines {
        for line in attention_routing_lines(&matrix) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&matrix).expect("serialize attention-routing matrix")
        );
    }
}
