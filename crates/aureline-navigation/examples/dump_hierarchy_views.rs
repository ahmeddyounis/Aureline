//! Headless emitter for the hierarchy-views corpus.
//!
//! Prints the canonical corpus that freezes how Aureline turns a set of hierarchy
//! edges into a typed call/type/override/ownership view: edges grouped by a
//! direct/transitive/inferred/runtime-observed legend, current-versus-captured scope
//! counts, explicitly named missing scopes, provider attribution, freshness and
//! confidence, an ambiguity/disambiguation state that gates jumps when the root
//! competes, stable open/peek/split/expand/export actions, and consumer projections
//! that never flatten the view into a single opaque tree. With `--lines`, prints the
//! human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_hierarchy_views            # JSON
//! cargo run -p aureline-navigation --example dump_hierarchy_views -- --lines
//! ```

use aureline_navigation::hierarchy_views::{hierarchy_views_lines, hierarchy_views_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = hierarchy_views_set();
    set.validate()
        .expect("canonical hierarchy-views corpus validates");

    if want_lines {
        for line in hierarchy_views_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize hierarchy-views corpus")
        );
    }
}
