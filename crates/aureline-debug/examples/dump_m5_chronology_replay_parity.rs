//! Headless emitter for the M5 chronology/replay/parity set.
//!
//! Prints the canonical, frozen set of typed chronology-capability descriptors, replay
//! sessions, timeline bookmarks, notebook-kernel capability descriptors, cell-frame links,
//! and restart/reconnect consequence records. Every descriptor carries one support pill
//! that pins one support class (supported, limited, unavailable, policy-blocked) and one
//! timeline state, derived only from its own backend; a replay session is always
//! inspect-only and names the capture it reconstructs; a timeline bookmark is bound to one
//! capture/session/target identity and survives support export and restore review; a
//! restart/reconnect consequence itemizes — per variables, queued cells, debug state,
//! breakpoints, and transient outputs — what was preserved, lost, invalidated, or left
//! stale; and a frame-to-cell link renders exact only when its mapping is exact and
//! supported. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_chronology_replay_parity \
//!   > fixtures/debug/m5_chronology_replay_parity/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_chronology_replay_parity -- --lines
//! ```

use aureline_debug::m5_chronology_replay_parity::{
    m5_chronology_replay_parity_lines, m5_chronology_replay_parity_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_chronology_replay_parity_set();
    set.validate()
        .expect("canonical m5 chronology/replay/parity set validates");

    if want_lines {
        for line in m5_chronology_replay_parity_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 chronology/replay/parity set")
        );
    }
}
