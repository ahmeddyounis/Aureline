//! Headless emitter for the M5 operator-surface matrix.
//!
//! Prints the canonical matrix that freezes Aureline's operator-facing
//! overview, triage, action-plan, handoff, shift-digest, service-ownership,
//! runbook-step, maintenance, failover, and embedded-boundary surfaces. Shell
//! UI, CLI/headless inspect, incident/support/admin/release/managed-service,
//! and companion/browser consumers render this matrix instead of restating the
//! operator-surface contract by hand. With `--lines`, prints the human-readable
//! projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_operator_surfaces            # JSON
//! cargo run -p aureline-support --example dump_m5_operator_surfaces -- --lines
//! ```

use aureline_support::m5_operator_surfaces::{operator_surface_lines, operator_surface_matrix};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let matrix = operator_surface_matrix();
    matrix
        .validate()
        .expect("canonical operator-surface matrix validates");

    if want_lines {
        for line in operator_surface_lines(&matrix) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&matrix).expect("serialize operator-surface matrix")
        );
    }
}
