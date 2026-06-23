//! Headless emitter for the relation-continuity corpus.
//!
//! Prints the canonical corpus that freezes how Aureline keeps symbol navigation and
//! rename evidence relation-aware across replay, drift, and return-context restoration:
//! every peek, temporary reveal, open-in-split, back/forward-history, and recent-location
//! entry preserves its relation kind, origin surface, return anchor, and
//! current-versus-captured target truth; a remapped, drifted, missing-target,
//! scope-unavailable, or archived entry keeps its drift state, reason, and recovery
//! choices visible and never silently jumps; and every entry and rename-evidence row
//! carries a replay-safe target id with its evidence class named. With `--lines`, prints
//! the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_relation_continuity            # JSON
//! cargo run -p aureline-navigation --example dump_relation_continuity -- --lines
//! ```

use aureline_navigation::relation_continuity::{
    relation_continuity_lines, relation_continuity_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = relation_continuity_set();
    set.validate()
        .expect("canonical relation-continuity corpus validates");

    if want_lines {
        for line in relation_continuity_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize relation-continuity corpus")
        );
    }
}
