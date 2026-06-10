//! Conformance dump for the framework-pack header, freshness-chip, and banner packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_framework_pack_headers -- canonical
//! cargo run -p aureline-templates --example dump_framework_pack_headers -- provenance_unknown
//! cargo run -p aureline-templates --example dump_framework_pack_headers -- capability_degraded
//! cargo run -p aureline-templates --example dump_framework_pack_headers -- markdown
//! ```

use aureline_templates::implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners::*;

const PACKET_ID: &str = "framework-pack:stable:0001";
const PACKET_LABEL: &str =
    "Framework-Pack Headers, Pack Version/Freshness Chips, and Capability/Downgrade Banners";
const MINTED_AT: &str = "2026-06-07T00:00:00Z";
const CLEAN_ROW: &str = "framework-pack-row:rust_axum.first_party:2026.05";
const PARTIAL_ROW: &str = "framework-pack-row:node_nest.community_update:2026.05";

fn canonical() -> FrameworkPackPacket {
    canonical_framework_pack(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        FrameworkPackProofFreshness {
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
        "provenance_unknown" => {
            // A clean first-party pack whose provenance can no longer be verified
            // is blocked and labeled provenance-unknown rather than offered.
            packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
                row_id: CLEAN_ROW.to_owned(),
                provenance_resolved: false,
                pack_version_current: true,
                freshness_fresh: true,
                capability_verified: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "capability_degraded" => {
            // A partial-capability pack whose capability cannot be verified is
            // degraded and withheld behind its capability banner.
            packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
                row_id: PARTIAL_ROW.to_owned(),
                provenance_resolved: true,
                pack_version_current: true,
                freshness_fresh: true,
                capability_verified: false,
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
