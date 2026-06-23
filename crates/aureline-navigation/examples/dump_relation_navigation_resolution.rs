//! Headless emitter for the relation-resolution corpus.
//!
//! Prints the canonical corpus that freezes how Aureline resolves a Go to
//! Definition / Declaration / Implementation command: it keeps the three relation
//! kinds distinct, opens a disambiguation set instead of guessing when multiple
//! candidates could change behavior, and never silently aliases one relation kind
//! for another — it discloses a fallback or reports the command unavailable. With
//! `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_relation_navigation_resolution            # JSON
//! cargo run -p aureline-navigation --example dump_relation_navigation_resolution -- --lines
//! ```

use aureline_navigation::relation_resolution::{
    relation_resolution_lines, relation_resolution_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = relation_resolution_set();
    set.validate()
        .expect("canonical relation-resolution corpus validates");

    if want_lines {
        for line in relation_resolution_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize relation-resolution corpus")
        );
    }
}
