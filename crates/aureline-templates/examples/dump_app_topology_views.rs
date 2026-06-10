//! Conformance dump for the route-explorer, component-tree, and app-topology view packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_app_topology_views -- canonical
//! cargo run -p aureline-templates --example dump_app_topology_views -- origin_unknown
//! cargo run -p aureline-templates --example dump_app_topology_views -- derivation_degraded
//! cargo run -p aureline-templates --example dump_app_topology_views -- markdown
//! ```

use aureline_templates::ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth::*;

const PACKET_ID: &str = "app-topology:stable:0001";
const PACKET_LABEL: &str =
    "Route Explorers, Component Trees, and App-Topology Views with Authored/Generated/Runtime-Only Truth";
const MINTED_AT: &str = "2026-06-08T00:00:00Z";
const AUTHORED_ROUTE: &str = "app-topology-row:route.authored.dashboard:2026.06";
const ZONE_COMPONENT: &str = "app-topology-row:component.authored_in_zone.user_card:2026.06";

fn canonical() -> AppTopologyPacket {
    canonical_app_topology(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        AppTopologyProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let mut packet = canonical();
    match which.as_str() {
        "canonical" => {}
        "markdown" => {
            print!("{}", packet.render_markdown_summary());
            return;
        }
        "origin_unknown" => {
            // An authored route whose origin can no longer be resolved is blocked
            // and labeled origin-unknown rather than shown as authored truth.
            packet.apply_downgrade_automation(&[AppTopologyRowObservation {
                row_id: AUTHORED_ROUTE.to_owned(),
                origin_resolved: false,
                generator_version_current: true,
                scan_fresh: true,
                derivation_verified: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "derivation_degraded" => {
            // An authored-in-generated-zone component whose derivation cannot be
            // verified is degraded and withheld behind its derivation banner.
            packet.apply_downgrade_automation(&[AppTopologyRowObservation {
                row_id: ZONE_COMPONENT.to_owned(),
                origin_resolved: true,
                generator_version_current: true,
                scan_fresh: true,
                derivation_verified: false,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
    println!("{}", packet.export_safe_json());
}
