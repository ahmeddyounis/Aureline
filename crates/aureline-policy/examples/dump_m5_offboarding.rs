//! Headless emitter for the M5 offboarding bundle.
//!
//! Prints the rendered offboarding wizards — the ordered review, export,
//! transfer, confirm, delete, and local-continuation checkpoints for seat loss,
//! cancellation, deprovision, org switch, and plan downgrade, each with its scope,
//! managed copies remaining, transfer owner, deletion schedule, confirmation gate,
//! and typed recovery — bound back to the frozen admin-plane matrix and rendered
//! across the managed-cloud, self-hosted, sovereign/air-gapped, and
//! mirrored/offline profiles. Shell admin center, CLI/headless inspect,
//! Help/About, support export, and procurement consumers render this bundle
//! instead of restating offboarding truth by hand. With `--lines`, prints the
//! human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_offboarding            # JSON
//! cargo run -p aureline-policy --example dump_m5_offboarding -- --lines
//! ```

use aureline_policy::m5_offboarding::{offboarding_bundle, offboarding_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = offboarding_bundle();
    bundle
        .validate()
        .expect("canonical offboarding bundle validates");

    if want_lines {
        for line in offboarding_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize offboarding bundle")
        );
    }
}
