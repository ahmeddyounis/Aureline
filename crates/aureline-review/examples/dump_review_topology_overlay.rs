//! Conformance dump for topology-aware mutation-review overlays.
//!
//! Prints the canonical export-safe [`ReviewTopologyOverlayPacket`] as
//! deterministic JSON. The packet models a proposed mutation set that spans a
//! parent root, its submodule child, and a nested independent repo — the classic
//! ambient-bulk-mutation hazard — and shows the cross-root preview guard that
//! keeps the apply preview-first and opt-in.
//!
//! The optional first argument selects the variant:
//!
//! * (no argument) — the multi-root parent/submodule/nested set
//! * `single` — a single active root that needs no cross-root preview
//!
//! Pass `--markdown` for the summary form. The default JSON form is the source of
//! the checked-in artifact.

use aureline_git::{current_git_topology_first_consumers_map, TopologyRootDescriptor};
use aureline_review::ReviewTopologyOverlayPacket;

const STAMP: &str = "2026-06-17T00:00:00Z";

fn roots_named(ids: &[&str]) -> Vec<TopologyRootDescriptor> {
    let map = current_git_topology_first_consumers_map().expect("canonical topology map validates");
    ids.iter()
        .map(|id| {
            map.roots
                .iter()
                .find(|root| root.root_id == *id)
                .unwrap_or_else(|| panic!("root {id} present in canonical map"))
                .clone()
        })
        .collect()
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "single" => ReviewTopologyOverlayPacket::from_descriptors(
            "review-topology-overlay:single:0001",
            STAMP,
            "review-topology-overlay-export:single:0001",
            "main",
            roots_named(&["main"]),
        ),
        _ => ReviewTopologyOverlayPacket::from_descriptors(
            "review-topology-overlay:0001",
            STAMP,
            "review-topology-overlay-export:0001",
            "main",
            roots_named(&["main", "submodule", "nested"]),
        ),
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "review overlay packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
