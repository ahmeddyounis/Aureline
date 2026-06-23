//! Headless emitter for the relation-navigation qualification certification.
//!
//! Prints the canonical certification that binds Aureline's relation-kind
//! navigation and rename-preview truth into M5 promotion: target-kind honesty,
//! references/access-kind truth, hierarchy proof classes, related-object
//! attribution, rename-preview completeness, and continuity/replay fidelity, each
//! certified on the search/navigation, graph/topology, docs/help, and editor-assist
//! surfaces. Every claim is derived from its proof state and freshness, so a stale
//! or failing proof narrows or withdraws the affected claim automatically. About,
//! Help, search/navigation, support, compatibility, release-truth, and public-truth
//! surfaces consume this certification instead of restating relation-navigation
//! quality by hand. With `--lines`, prints the human-readable projection instead of
//! JSON.
//!
//! ```sh
//! cargo run -p aureline-navigation --example dump_relation_navigation_qualification            # JSON
//! cargo run -p aureline-navigation --example dump_relation_navigation_qualification -- --lines
//! ```

use aureline_navigation::relation_navigation_qualification::{
    relation_navigation_qualification, relation_navigation_qualification_lines,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let cert = relation_navigation_qualification();
    cert.validate()
        .expect("canonical relation-navigation qualification certification validates");

    if want_lines {
        for line in relation_navigation_qualification_lines(&cert) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&cert)
                .expect("serialize relation-navigation qualification certification")
        );
    }
}
