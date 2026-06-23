//! Headless emitter for the M5 admin-plane render bundle.
//!
//! Prints the rendered admin-plane surfaces — the effective-policy view,
//! policy-diff sheet, locked-state explanations, and endpoint-posture card —
//! bound back to the frozen admin-plane matrix and rendered across the
//! managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline
//! profiles. Shell admin center, CLI/headless inspect, Help/About, support
//! export, and release evidence render this bundle instead of restating policy or
//! endpoint state by hand. With `--lines`, prints the human-readable projection
//! instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_admin_render            # JSON
//! cargo run -p aureline-policy --example dump_m5_admin_render -- --lines
//! ```

use aureline_policy::m5_admin_render::{admin_render_bundle, admin_render_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = admin_render_bundle();
    bundle
        .validate()
        .expect("canonical admin-plane render bundle validates");

    if want_lines {
        for line in admin_render_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize admin-plane render bundle")
        );
    }
}
