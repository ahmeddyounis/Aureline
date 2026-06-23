//! Headless emitter for the M5 attention-actions bundle.
//!
//! Prints the canonical bundle that implements Aureline's distinct attention-action
//! semantics: the dismiss, snooze, acknowledge, mute, and resolve action definitions,
//! a representative corpus of durable attention items, and every applied outcome with
//! its retention, badge, exact reopen continuity, cross-client propagation, and
//! support-export explanation. The shell activity center, OS notifications,
//! companion summaries, operator dashboard, support export, Help/About, and
//! CLI/headless consumers render this bundle instead of reimplementing attention
//! actions per surface. With `--lines`, prints the human-readable projection instead
//! of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_attention_actions            # JSON
//! cargo run -p aureline-activity --example dump_m5_attention_actions -- --lines
//! ```

use aureline_activity::m5_attention_actions::{attention_actions_bundle, attention_actions_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = attention_actions_bundle();
    bundle
        .validate()
        .expect("canonical attention-actions bundle validates");

    if want_lines {
        for line in attention_actions_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize attention-actions bundle")
        );
    }
}
