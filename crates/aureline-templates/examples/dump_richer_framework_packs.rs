//! Conformance dump for the richer framework-pack lane catalog packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_richer_framework_packs -- canonical
//! cargo run -p aureline-templates --example dump_richer_framework_packs -- health_degraded
//! cargo run -p aureline-templates --example dump_richer_framework_packs -- generator_version_yanked
//! cargo run -p aureline-templates --example dump_richer_framework_packs -- markdown
//! ```

use aureline_templates::add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter::*;

const PACKET_ID: &str = "richer-framework-pack:stable:0001";
const PACKET_LABEL: &str =
    "Richer Framework Packs: Jupyter Adjacency, Terraform/Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";
const TERRAFORM_ROW: &str = "framework-lane-row:terraform.first_party:2026.06";
const NEST_ROW: &str = "framework-lane-row:nest.community_update:2026.05";

fn canonical() -> FrameworkLanePacket {
    canonical_richer_framework_pack(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        FrameworkLaneProofFreshness {
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
        "health_degraded" => {
            // A clean community pack whose archetype health check fails is degraded
            // and withheld behind its health banner.
            packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
                row_id: NEST_ROW.to_owned(),
                provenance_resolved: true,
                pack_version_current: true,
                generator_version_current: true,
                freshness_fresh: true,
                capability_verified: true,
                archetype_health_ok: false,
                origin_truth_verified: true,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "generator_version_yanked" => {
            // A first-party pack whose pinned generator version is yanked narrows to
            // stale and is withheld behind a freshness banner.
            packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
                row_id: TERRAFORM_ROW.to_owned(),
                provenance_resolved: true,
                pack_version_current: true,
                generator_version_current: false,
                freshness_fresh: true,
                capability_verified: true,
                archetype_health_ok: true,
                origin_truth_verified: true,
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
