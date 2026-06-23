//! Headless emitter for the M5 procurement bundle.
//!
//! Prints the rendered procurement / verification packets, renewal / trial /
//! seat-change summary cards, and admin-handoff packets — each with its deployment
//! mode, supported export paths, billing/owner scope, validity window and signature
//! posture, evidence refs, residual dependencies, reused canonical objects, and
//! support/renewal handoff data — bound back to the frozen admin-plane matrix and
//! rendered across the managed-cloud, self-hosted, sovereign/air-gapped, and
//! mirrored/offline profiles. Commercial/procurement, Help/About, support export,
//! release evidence, and managed-service consumers render this bundle instead of
//! restating procurement truth by hand. With `--lines`, prints the human-readable
//! projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_procurement            # JSON
//! cargo run -p aureline-policy --example dump_m5_procurement -- --lines
//! ```

use aureline_policy::m5_procurement::{procurement_bundle, procurement_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = procurement_bundle();
    bundle
        .validate()
        .expect("canonical procurement bundle validates");

    if want_lines {
        for line in procurement_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize procurement bundle")
        );
    }
}
