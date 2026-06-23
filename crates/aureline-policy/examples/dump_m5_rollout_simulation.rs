//! Headless emitter for the M5 rollout-simulation bundle.
//!
//! Prints the dry-run rollout simulations — policy imports, promotions, bundle
//! rollouts, mirror-source changes, trust-root changes, and route/egress
//! expansions previewed before they widen privilege or feature access — bound back
//! to the frozen admin-plane matrix and simulated across the managed-cloud,
//! self-hosted, sovereign/air-gapped, and mirrored/offline profiles. Shell admin
//! center, CLI/headless inspect, Help/About, support export, and release evidence
//! render this bundle instead of re-deriving rollout impact by hand. With
//! `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_rollout_simulation            # JSON
//! cargo run -p aureline-policy --example dump_m5_rollout_simulation -- --lines
//! ```

use aureline_policy::m5_rollout_simulation::{rollout_simulation_bundle, rollout_simulation_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = rollout_simulation_bundle();
    bundle
        .validate()
        .expect("canonical rollout-simulation bundle validates");

    if want_lines {
        for line in rollout_simulation_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize rollout-simulation bundle")
        );
    }
}
