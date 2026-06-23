//! Headless emitter for the relation-navigation matrix.
//!
//! Prints the canonical matrix that freezes Aureline's relation-kind navigation —
//! navigation targets, reference occurrences, hierarchy edges, related-object
//! relations, rename-preview sets, and the relation/fallback vocabulary. The
//! search palette, editor assist, graph overlay, docs/help, AI context, review
//! workspace, support export, CLI/headless, and shell continuity surfaces render
//! this matrix instead of restating the relation-navigation contract by hand. With
//! `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_m5_relation_navigation            # JSON
//! cargo run -p aureline-navigation --example dump_m5_relation_navigation -- --lines
//! ```

use aureline_navigation::m5_relation_navigation::{
    relation_navigation_lines, relation_navigation_matrix,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let matrix = relation_navigation_matrix();
    matrix
        .validate()
        .expect("canonical relation-navigation matrix validates");

    if want_lines {
        for line in relation_navigation_lines(&matrix) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&matrix).expect("serialize relation-navigation matrix")
        );
    }
}
