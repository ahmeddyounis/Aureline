//! Headless emitter for the M5 fanout-receipts bundle.
//!
//! Prints the canonical bundle that mints one durable, privacy-safe fanout receipt per
//! cross-client destination — the native OS notification and the browser and mobile
//! companions — when an attention object is fanned out. The shell activity center, OS
//! notifications, companion summaries, operator dashboard, support export, Help/About, and
//! CLI/headless consumers render this bundle instead of reimplementing cross-client
//! delivery truth per surface. With `--lines`, prints the human-readable projection instead
//! of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_fanout_receipts            # JSON
//! cargo run -p aureline-activity --example dump_m5_fanout_receipts -- --lines
//! ```

use aureline_activity::m5_fanout_receipts::{fanout_receipts_bundle, fanout_receipts_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = fanout_receipts_bundle();
    bundle
        .validate()
        .expect("canonical fanout-receipts bundle validates");

    if want_lines {
        for line in fanout_receipts_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize fanout-receipts bundle")
        );
    }
}
