//! Headless emitter for the M5 admin-plane matrix.
//!
//! Prints the canonical matrix that freezes Aureline's local admin plane —
//! effective policy, policy diff, decision history, locked-state explanation,
//! retention/deletion, offboarding, procurement/verification, and endpoint
//! posture. Shell admin center, CLI/headless inspect, Help/About, support
//! export, commercial/procurement, release evidence, and managed-service
//! consumers render this matrix instead of restating the admin-plane contract by
//! hand. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_admin_plane            # JSON
//! cargo run -p aureline-policy --example dump_m5_admin_plane -- --lines
//! ```

use aureline_policy::m5_admin_plane::{admin_plane_lines, admin_plane_matrix};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let matrix = admin_plane_matrix();
    matrix
        .validate()
        .expect("canonical admin-plane matrix validates");

    if want_lines {
        for line in admin_plane_lines(&matrix) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&matrix).expect("serialize admin-plane matrix")
        );
    }
}
