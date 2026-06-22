//! Headless emitter for the M5 triage-inbox set.
//!
//! Prints the canonical triage inboxes — incident, support, and admin — as
//! reason-bearing rows over many canonical objects, each carrying its
//! reason-for-attention, priority/SLA, source/provider, local-versus-shared/
//! deferred state, freshness, computed no-silent-green state, batch-review path,
//! and canonical open-detail route, with the shared filter/group/order
//! vocabulary, the frozen default-view handoff bundles, and the batch-review
//! previews. Shell UI, CLI/headless inspect, incident/support/admin consumers,
//! and support export render this set instead of restating the triage contract by
//! hand. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_triage_inbox            # JSON
//! cargo run -p aureline-support --example dump_m5_triage_inbox -- --lines
//! ```

use aureline_support::m5_triage_inbox::{triage_inbox_lines, triage_inbox_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = triage_inbox_set();
    set.validate()
        .expect("canonical triage-inbox set validates");

    if want_lines {
        for line in triage_inbox_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize triage-inbox set")
        );
    }
}
