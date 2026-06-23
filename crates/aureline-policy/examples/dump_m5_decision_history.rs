//! Headless emitter for the M5 decision-history bundle.
//!
//! Prints the rendered decision-history timelines and audit-event explorers —
//! material allow/deny/quota/force-disable/publish-scope decisions with stable
//! decision codes, distinguished actor classes, policy epochs, affected scope,
//! time, explanation links, and dual machine/plain-language export — bound back
//! to the frozen admin-plane matrix and rendered across the managed-cloud,
//! self-hosted, sovereign/air-gapped, and mirrored/offline profiles. Shell admin
//! center, CLI/headless inspect, support export, procurement, and managed-service
//! consumers render this bundle instead of scraping logs by hand. With `--lines`,
//! prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_decision_history            # JSON
//! cargo run -p aureline-policy --example dump_m5_decision_history -- --lines
//! ```

use aureline_policy::m5_decision_history::{decision_history_bundle, decision_history_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = decision_history_bundle();
    bundle
        .validate()
        .expect("canonical decision-history bundle validates");

    if want_lines {
        for line in decision_history_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize decision-history bundle")
        );
    }
}
