//! Headless emitter for the M5 badge-aggregates bundle.
//!
//! Prints the canonical bundle that turns a durable-item corpus into deduped, per-scope
//! badge counts, coalesces repeated failures from one root cause into a single durable
//! object, projects one shared count across every badge-bearing surface, and emits stable
//! telemetry enums. The shell activity center, dock/taskbar badge, companion summaries,
//! operator dashboard, support export, Help/About, and CLI/headless consumers render this
//! bundle instead of re-deriving badge truth per surface. With `--lines`, prints the
//! human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_badge_aggregates            # JSON
//! cargo run -p aureline-activity --example dump_m5_badge_aggregates -- --lines
//! ```

use aureline_activity::m5_badge_aggregates::{badge_aggregates_bundle, badge_aggregates_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = badge_aggregates_bundle();
    bundle
        .validate()
        .expect("canonical badge-aggregates bundle validates");

    if want_lines {
        for line in badge_aggregates_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize badge-aggregates bundle")
        );
    }
}
