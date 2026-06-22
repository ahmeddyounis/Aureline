//! Headless emitter for the M5 response-pane set.
//!
//! Prints the canonical response-pane set — the service-ownership / on-call
//! strips, the runbook-guided response panes with their computed mutating-step
//! preview/approval admission, and the local-outage continuity views — as one
//! support-export-safe record bound to the operator-surface matrix. Shell UI,
//! CLI/headless inspect, incident/support/managed consumers, and support export
//! render this set instead of restating service ownership, step authority, or
//! outage continuity by hand. With `--lines`, prints the human-readable
//! projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_response_panes            # JSON
//! cargo run -p aureline-support --example dump_m5_response_panes -- --lines
//! ```

use aureline_support::m5_response_panes::{response_pane_lines, response_pane_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = response_pane_set();
    set.validate()
        .expect("canonical response-pane set validates");

    if want_lines {
        for line in response_pane_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize response-pane set")
        );
    }
}
