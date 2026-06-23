//! Headless emitter for the M5 breakpoint-spec set.
//!
//! Prints the canonical, frozen set of typed breakpoint specs and mapping-state pills.
//! Every breakpoint carries one canonical pill that pins one verification state
//! (pending, verified, unbound, unsupported, policy-blocked) and one mapping state
//! (exact, misaligned, needs-remap, unmapped); a green confirmed-stop icon renders
//! only when a breakpoint is verified, exact, and not replay-only; identity survives
//! rename/reformat/import or degrades to an explicit needs-remap; a lexical fallback
//! never poses as exact; and notebook and replay views keep stable cell and frame
//! identity. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_breakpoint_specs \
//!   > fixtures/debug/m5_breakpoint_specs/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_breakpoint_specs -- --lines
//! ```

use aureline_debug::m5_breakpoint_specs::{m5_breakpoint_spec_lines, m5_breakpoint_spec_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_breakpoint_spec_set();
    set.validate()
        .expect("canonical m5 breakpoint-spec set validates");

    if want_lines {
        for line in m5_breakpoint_spec_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 breakpoint-spec set")
        );
    }
}
