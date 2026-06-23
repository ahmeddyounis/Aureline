//! Headless emitter for the references-pane corpus.
//!
//! Prints the canonical corpus that freezes how Aureline turns a Find References
//! result into a typed pane: occurrences grouped by access kind, current-versus-
//! captured scope counts, an evidence class naming whether the set is semantic,
//! framework-derived, runtime-observed, imported, or a lexical fallback,
//! generated/external/test-only labels, stable open/peek/split/export actions, and
//! consumer projections that never flatten the set into generic hits. With
//! `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_reference_panes            # JSON
//! cargo run -p aureline-navigation --example dump_reference_panes -- --lines
//! ```

use aureline_navigation::reference_panes::{reference_panes_lines, reference_panes_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = reference_panes_set();
    set.validate()
        .expect("canonical references-pane corpus validates");

    if want_lines {
        for line in reference_panes_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize references-pane corpus")
        );
    }
}
