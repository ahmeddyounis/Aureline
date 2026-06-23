//! Headless emitter for the M5 frame-mapping and variable/watch snapshot set.
//!
//! Prints the canonical, frozen set of typed frame mappings and value snapshots. Every
//! frame carries one pill that pins one mapping fidelity (exact, approximate,
//! symbol-only, unmapped) and one build-match outcome; a precise source link renders
//! only for an exact mapping backed by an exact-build match; current-frame identity is
//! preserved per thread; a source-map mapping always discloses; a lost mapping degrades
//! to an explicit unmapped frame; and async/runtime boundaries stay visible. Every value
//! snapshot — variable or watch, live session, notebook cell, or replay capture — carries
//! one disclosure pill that pins one of live, captured, stale, unavailable, or redacted.
//! With `--lines`, prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_frame_variable_snapshots \
//!   > fixtures/debug/m5_frame_variable_snapshots/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_frame_variable_snapshots -- --lines
//! ```

use aureline_debug::m5_frame_variable_snapshots::{
    m5_frame_variable_snapshot_lines, m5_frame_variable_snapshot_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_frame_variable_snapshot_set();
    set.validate()
        .expect("canonical m5 frame/variable snapshot set validates");

    if want_lines {
        for line in m5_frame_variable_snapshot_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 frame/variable snapshot set")
        );
    }
}
