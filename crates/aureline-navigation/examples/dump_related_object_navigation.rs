//! Headless emitter for the related-object navigation corpus.
//!
//! Prints the canonical corpus that freezes how Aureline turns the related links for an
//! anchor into a typed, source-attributed panel: route, component, test, doc, owner, and
//! generated-artifact links grouped by a graph-derived/framework-derived/curated/runtime-
//! derived legend, each with its fallback mode, freshness, proof, and scope; an anchor
//! context and parity that say whether notebook, diff, docs-linked, and generated-artifact
//! surfaces can reuse the relation semantics; current-versus-captured counts; a
//! disambiguation path that gates jumps; stable open/peek/split/reveal/export actions; and
//! consumer projections that never flatten the panel into generic links. With `--lines`,
//! prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_related_object_navigation            # JSON
//! cargo run -p aureline-navigation --example dump_related_object_navigation -- --lines
//! ```

use aureline_navigation::related_object_navigation::{
    related_object_navigation_lines, related_object_navigation_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = related_object_navigation_set();
    set.validate()
        .expect("canonical related-object navigation corpus validates");

    if want_lines {
        for line in related_object_navigation_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize related-object navigation corpus")
        );
    }
}
