//! Conformance dump for topology-aware AI context assembly.
//!
//! Prints the canonical export-safe [`AiTopologyContextPacket`] as deterministic
//! JSON. The packet propagates the canonical [`aureline_git`] AI-context surface
//! bindings, so the model only treats complete, in-scope, hydrated slices as
//! authoritative, and every cross-root or topology-limited slice stays explicitly
//! disclosed.
//!
//! The optional first argument selects the variant:
//!
//! * (no argument) — every root projected against its own active root
//! * `cross-root` — every root projected against the `main` active root, so a
//!   non-active slice surfaces as a cross-root boundary that is never admitted
//!
//! Pass `--markdown` for the summary form. The default JSON form is the source of
//! the checked-in artifact.

use aureline_ai::AiTopologyContextPacket;
use aureline_git::{current_git_topology_first_consumers_map, TopologyConsumerSurface};

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
                    TopologyConsumerSurface::AiContext,
                    "main",
                    format!("binding-ai_context-{}-active-main", root.root_id),
                )
            })
            .collect();
        (
            "ai-topology-context:cross-root:0001",
            "ai-topology-context-export:cross-root:0001",
            bindings,
        )
    } else {
        let bindings = map
            .surface_bindings
            .into_iter()
            .filter(|binding| binding.surface == TopologyConsumerSurface::AiContext)
            .collect();
        (
            "ai-topology-context:0001",
            "ai-topology-context-export:0001",
            bindings,
        )
    };
    let packet =
        AiTopologyContextPacket::from_ai_context_bindings(packet_id, STAMP, export_id, bindings);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "ai context packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
