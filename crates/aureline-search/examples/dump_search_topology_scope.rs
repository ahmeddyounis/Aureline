//! Conformance dump for topology-aware search scope.
//!
//! Prints the canonical export-safe [`SearchTopologyScopePacket`] as deterministic
//! JSON. The packet propagates the canonical [`aureline_git`] search-scope surface
//! bindings, so every topology limit (omitted slice, unfetched objects, shallow
//! boundary, uninitialized submodule, pointer-only assets, generated/vendor roots)
//! surfaces as an explicit search-scope row instead of a flat zero-result state.
//!
//! The optional first argument selects the variant:
//!
//! * (no argument) — every root projected against its own active root
//! * `cross-root` — every root projected against the `main` active root, so a
//!   non-active root surfaces as wrong-target-root or a nested boundary
//!
//! Pass `--markdown` for the summary form. The default JSON form is the source of
//! the checked-in artifact.

use aureline_git::{current_git_topology_first_consumers_map, TopologyConsumerSurface};
use aureline_search::SearchTopologyScopePacket;

const STAMP: &str = "2026-06-17T00:00:00Z";

fn main() {
    let map = current_git_topology_first_consumers_map().expect("canonical topology map validates");
    let variant = std::env::args().nth(1).unwrap_or_default();
    let (packet_id, export_id, bindings) = if variant == "cross-root" {
        let bindings = map
            .roots
            .iter()
            .map(|root| {
                root.project(
                    TopologyConsumerSurface::SearchScope,
                    "main",
                    format!("binding-search_scope-{}-active-main", root.root_id),
                )
            })
            .collect();
        (
            "search-topology-scope:cross-root:0001",
            "search-topology-scope-export:cross-root:0001",
            bindings,
        )
    } else {
        let bindings = map
            .surface_bindings
            .into_iter()
            .filter(|binding| binding.surface == TopologyConsumerSurface::SearchScope)
            .collect();
        (
            "search-topology-scope:0001",
            "search-topology-scope-export:0001",
            bindings,
        )
    };
    let packet =
        SearchTopologyScopePacket::from_search_bindings(packet_id, STAMP, export_id, bindings);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "search scope packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
