//! Headless emitter for the M5 dump/mapping/restore set.
//!
//! Prints the canonical, frozen set of typed dump/core-file/source-map/symbol artifact
//! strips and restored-layout records. Every strip carries one pill that pins one shared
//! mapping fidelity (exact, approximate, symbol-only, unresolved, imported,
//! mismatched-build), one build-match outcome, and one source class; a precise source link
//! renders only for an exact mapping backed by an exact-build match; an imported or
//! build-mismatched strip never renders it; core-file/crash-dump/open-replay/open-inspect-only
//! entrypoints stay distinct from importing a symbol or source-map artifact. Every restored
//! layout names whether the prior process/session is gone, inspect-only, reconnect-required,
//! or manually relaunchable and never implies live continuity or reacquired process
//! authority. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_dump_mapping_restore \
//!   > fixtures/debug/m5_dump_mapping_restore/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_dump_mapping_restore -- --lines
//! ```

use aureline_debug::m5_dump_mapping_restore::{
    m5_dump_mapping_restore_lines, m5_dump_mapping_restore_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_dump_mapping_restore_set();
    set.validate()
        .expect("canonical m5 dump/mapping/restore set validates");

    if want_lines {
        for line in m5_dump_mapping_restore_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 dump/mapping/restore set")
        );
    }
}
