//! Headless emitter for the M5 handoff/digest set.
//!
//! Prints the canonical operator continuity packets — an outgoing-shift handoff, a
//! client-facing handoff, a daily operations digest, and a private night-shift
//! digest — that preserve, outside the live session, the same object identity,
//! grouping, freshness, ownership, redaction, unresolved questions, and
//! live-versus-cached-versus-mirrored-versus-snapshot truth the operator saw.
//! Digests group by object and severity before chronology; every packet reopens
//! onto the canonical object or a truthful placeholder, declares explicit scope and
//! boundary truth, and freezes a snapshot export that never flattens the
//! storage/freshness distinction. Shell UI, CLI/headless inspect,
//! incident/support/admin/managed consumers, and support export render this set
//! instead of restating the continuity contract by hand. With `--lines`, prints the
//! human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_handoff_digests            # JSON
//! cargo run -p aureline-support --example dump_m5_handoff_digests -- --lines
//! ```

use aureline_support::m5_handoff_digests::{handoff_digest_lines, handoff_digest_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = handoff_digest_set();
    set.validate()
        .expect("canonical handoff/digest set validates");

    if want_lines {
        for line in handoff_digest_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize handoff/digest set")
        );
    }
}
