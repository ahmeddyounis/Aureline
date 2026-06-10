//! Conformance dump for the signed template-registry packet.
//!
//! Prints one of the canonical export, the two checked-in narrowed fixtures, or
//! the Markdown summary, so the artifact and fixtures can be regenerated
//! deterministically from the canonical builder.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_signed_template_registry -- canonical
//! cargo run -p aureline-templates --example dump_signed_template_registry -- mirror_stale
//! cargo run -p aureline-templates --example dump_signed_template_registry -- signature_failed
//! cargo run -p aureline-templates --example dump_signed_template_registry -- markdown
//! ```

use aureline_templates::implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows::*;

const PACKET_ID: &str = "signed-template-registry:stable:0001";
const REGISTRY_LABEL: &str =
    "Signed Template Registry, Provenance/Mirror, and Template-Health Rows";
const MINTED_AT: &str = "2026-06-07T00:00:00Z";
const OFFICIAL_ROW: &str = "registry-row:official.rust.cli:2026.04";
const COMMUNITY_ROW: &str = "registry-row:community.python.data:2026.03";

fn canonical() -> SignedTemplateRegistryPacket {
    canonical_signed_template_registry(
        PACKET_ID.to_owned(),
        REGISTRY_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        SignedTemplateRegistryProofFreshness {
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
        "health_stale" => {
            // A healthy official row whose template-health checks aged out of cadence
            // narrows to stale-but-inspectable while staying offerable.
            packet.apply_downgrade_automation(&[SignedTemplateRegistryRowObservation {
                row_id: OFFICIAL_ROW.to_owned(),
                signature_valid: true,
                trust_root_resolved: true,
                mirror_fresh: true,
                health_current: false,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "signature_failed" => {
            // A community row whose signature failed verification is blocked, not hidden.
            packet.apply_downgrade_automation(&[SignedTemplateRegistryRowObservation {
                row_id: COMMUNITY_ROW.to_owned(),
                signature_valid: false,
                trust_root_resolved: true,
                mirror_fresh: true,
                health_current: true,
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
