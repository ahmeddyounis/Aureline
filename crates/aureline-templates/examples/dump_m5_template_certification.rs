//! Conformance dump for the M5 template, scaffold, framework-pack, and
//! archetype-health certification packet.
//!
//! Prints either the canonical certification export, one of the two checked-in
//! narrowed/blocked fixtures, or the Markdown summary, so the artifact and
//! fixtures can be regenerated deterministically from the first-consumer
//! certification.
//!
//! ```text
//! cargo run -p aureline-templates --example dump_m5_template_certification -- canonical
//! cargo run -p aureline-templates --example dump_m5_template_certification -- framework_pack_blocked
//! cargo run -p aureline-templates --example dump_m5_template_certification -- archetype_health_stale
//! cargo run -p aureline-templates --example dump_m5_template_certification -- markdown
//! ```

use aureline_templates::certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile::*;

const PACKET_ID: &str = "m5-template-certification:certified:0001";
const PACKET_LABEL: &str =
    "M5 Template Registry, Scaffold Planner, Framework Packs, and Archetype Health Certification";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> M5TemplateCertificationPacket {
    certify_from_current_exports(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        M5TemplateCertificationProofFreshness {
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
        "framework_pack_blocked" => {
            // The framework-pack-header profile's evidence packet fails validation,
            // so it is blocked from promotion rather than shipped greener than the proof.
            packet.apply_downgrade_automation(&[M5TemplateCertificationProfileObservation {
                profile: M5TemplateProfile::FrameworkPackHeader,
                evidence_valid: false,
                proof_fresh: true,
                upstream_narrowed: false,
            }]);
        }
        "archetype_health_stale" => {
            // The archetype-health profile's proof went stale, so a certified profile
            // narrows to narrowed_certified while its proof_fresh flag flips to false.
            packet.apply_downgrade_automation(&[M5TemplateCertificationProfileObservation {
                profile: M5TemplateProfile::ArchetypeHealthBundle,
                evidence_valid: true,
                proof_fresh: false,
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
