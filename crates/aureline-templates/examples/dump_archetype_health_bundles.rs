//! Conformance dump for the certified-archetype health-check bundle packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_archetype_health_bundles -- canonical
//! cargo run -p aureline-templates --example dump_archetype_health_bundles -- health_unknown
//! cargo run -p aureline-templates --example dump_archetype_health_bundles -- fix_forward_unavailable
//! cargo run -p aureline-templates --example dump_archetype_health_bundles -- markdown
//! ```

use aureline_templates::ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance::*;

const PACKET_ID: &str = "archetype-health:stable:0001";
const PACKET_LABEL: &str =
    "Certified-Archetype Health-Check Bundles, Stack Diagnostics, and Fix-Forward Guidance";
const MINTED_AT: &str = "2026-06-08T00:00:00Z";
const SERVICE_HEALTHY: &str = "archetype-health-row:rust.axum.service.certified.healthy:2026.06";
const WEBAPP_ADVISORIES: &str = "archetype-health-row:ts.next.webapp.certified.advisories:2026.06";

fn canonical() -> ArchetypeHealthBundlePacket {
    canonical_archetype_health_bundles(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        ArchetypeHealthProofFreshness {
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
        "health_unknown" => {
            // A certified service bundle whose health verdict can no longer be
            // determined is blocked rather than presented as a confident verdict.
            packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
                row_id: SERVICE_HEALTHY.to_owned(),
                certification_verified: true,
                health_determinable: false,
                diagnostics_available: true,
                fix_guidance_available: true,
                bundle_fresh: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "fix_forward_unavailable" => {
            // A certified web-app bundle whose fix-forward guidance could not be
            // produced is labeled rather than hidden, and stays offered.
            packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
                row_id: WEBAPP_ADVISORIES.to_owned(),
                certification_verified: true,
                health_determinable: true,
                diagnostics_available: true,
                fix_guidance_available: false,
                bundle_fresh: true,
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
