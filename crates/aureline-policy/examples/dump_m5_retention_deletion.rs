//! Headless emitter for the M5 retention/deletion bundle.
//!
//! Prints the rendered retention/deletion matrices — each claimed managed
//! artifact family with its data class, local-only versus hosted location, default
//! retention, export and delete routes, owner, and governing schema, plus a
//! current immediate/deferred/blocked delete outcome linked to destruction
//! receipts, privacy-request cases, holds, and partial-delete reasons — bound back
//! to the frozen admin-plane matrix and rendered across the managed-cloud,
//! self-hosted, sovereign/air-gapped, and mirrored/offline profiles. Shell admin
//! center, CLI/headless inspect, Help/About, support export, and procurement
//! consumers render this bundle instead of restating retention truth by hand. With
//! `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_retention_deletion            # JSON
//! cargo run -p aureline-policy --example dump_m5_retention_deletion -- --lines
//! ```

use aureline_policy::m5_retention_deletion::{retention_deletion_bundle, retention_deletion_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = retention_deletion_bundle();
    bundle
        .validate()
        .expect("canonical retention/deletion bundle validates");

    if want_lines {
        for line in retention_deletion_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize retention/deletion bundle")
        );
    }
}
