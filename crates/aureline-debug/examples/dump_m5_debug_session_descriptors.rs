//! Headless emitter for the M5 debug-session descriptor set.
//!
//! Prints the canonical, frozen set of typed debug-session and attach-target
//! descriptors. Launch, attach, core-file, replay, and inspect-only appear as five
//! distinct session modes and command/result objects; each attach target carries
//! its identity, local/remote/container/managed boundary, mutability, privilege
//! class, adapter ref/version, adapter drift, and build/artifact identity, which
//! every session echoes from picker to active session to export packet; adapter
//! drift, reconnect-required, inspect-only, and unsupported-skew are first-class
//! labels; and a restored session reopens layout and history but never silently
//! reattaches. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_debug_session_descriptors \
//!   > fixtures/debug/m5_debug_session_descriptors/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_debug_session_descriptors -- --lines
//! ```

use aureline_debug::m5_debug_session_descriptors::{
    m5_debug_session_descriptor_lines, m5_debug_session_descriptor_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_debug_session_descriptor_set();
    set.validate()
        .expect("canonical m5 debug-session descriptor set validates");

    if want_lines {
        for line in m5_debug_session_descriptor_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 debug-session descriptor set")
        );
    }
}
