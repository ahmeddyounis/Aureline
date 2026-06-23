//! Headless emitter for the M5 debug-contracts matrix.
//!
//! Prints the canonical, frozen matrix that names Aureline's M5 debugger object
//! families — debug session, attach target, breakpoint spec, frame mapping,
//! variable/watch snapshot, evaluate request/result, console emission, chronology
//! capability, replay session, and notebook-debug parity — pins one controlled
//! vocabulary across session modes, breakpoint/mapping states, variable freshness,
//! evaluate purity, mapping fidelity, and restore/reattach posture, and maps each
//! object to the proof packet that keeps it current. Notebook, profiler, incident,
//! support, AI, and core debug surfaces consume this matrix instead of restating
//! debug truth by hand. With `--lines`, prints the human-readable projection
//! instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_debug_contracts \
//!   > fixtures/debug/m5_debug_contracts/canonical_matrix.json
//! cargo run -p aureline-debug --example dump_m5_debug_contracts -- --lines
//! ```

use aureline_debug::m5_debug_contracts::{m5_debug_contracts_lines, m5_debug_contracts_matrix};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let matrix = m5_debug_contracts_matrix();
    matrix
        .validate()
        .expect("canonical m5 debug-contracts matrix validates");

    if want_lines {
        for line in m5_debug_contracts_lines(&matrix) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&matrix).expect("serialize m5 debug-contracts matrix")
        );
    }
}
