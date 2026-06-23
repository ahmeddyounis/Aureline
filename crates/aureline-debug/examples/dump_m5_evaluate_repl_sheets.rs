//! Headless emitter for the M5 evaluate/REPL sheet set.
//!
//! Prints the canonical, frozen set of typed evaluate/REPL review sheets and console
//! emissions. Every evaluation carries one posture pill that pins one purity class (pure,
//! unknown, may-mutate) and one approval disposition; an unknown or mutating expression
//! discloses its risk and requires approval before dispatch; a pending, denied, blocked,
//! or expired evaluation never permits dispatch and carries no result; an effectful
//! expression against an inspect-only context is blocked; and actor lineage names who
//! requested and reviewed it. Every console emission carries one pill that pins one
//! direction (user input vs target output) and one liveness (live vs replayed), so input
//! and output stay separate and a replayed line is never shown as live. With `--lines`,
//! prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_evaluate_repl_sheets \
//!   > fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_evaluate_repl_sheets -- --lines
//! ```

use aureline_debug::m5_evaluate_repl_sheets::{
    m5_evaluate_repl_sheet_lines, m5_evaluate_repl_sheet_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_evaluate_repl_sheet_set();
    set.validate()
        .expect("canonical m5 evaluate/REPL sheet set validates");

    if want_lines {
        for line in m5_evaluate_repl_sheet_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 evaluate/REPL sheet set")
        );
    }
}
