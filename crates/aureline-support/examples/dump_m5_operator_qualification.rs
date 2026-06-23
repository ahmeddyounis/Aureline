//! Headless emitter for the operator-surface qualification packet.
//!
//! Prints the canonical packet that binds the M5 operator-surface truth sources
//! into one per-family claim verdict and auto-narrows a claimed operator family
//! when its ownership/freshness/continuity proof is stale or failing. About/help,
//! service-health, compatibility, release automation, and support export all
//! consume this packet instead of restating operator-surface quality claims by
//! hand. With `--lines`, prints the human-readable projection instead of JSON.
//! The packet takes no input: it reads the real in-code proof sources.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_operator_qualification            # JSON
//! cargo run -p aureline-support --example dump_m5_operator_qualification -- --lines
//! ```

use aureline_support::m5_operator_qualification::{
    operator_qualification_lines, operator_qualification_packet,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let packet = operator_qualification_packet();

    if want_lines {
        for line in operator_qualification_lines(&packet) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&packet).expect("serialize operator-qualification packet")
        );
    }
}
