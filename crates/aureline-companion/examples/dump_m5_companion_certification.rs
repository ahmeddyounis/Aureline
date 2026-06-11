//! Conformance dump for the M5 companion certification packet.
//!
//! Prints either the canonical certification export, a degraded fixture, or the
//! Markdown summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer certification builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_m5_companion_certification -- canonical
//! cargo run -p aureline-companion --example dump_m5_companion_certification -- proof_stale
//! cargo run -p aureline-companion --example dump_m5_companion_certification -- sync_evidence_invalid
//! cargo run -p aureline-companion --example dump_m5_companion_certification -- residency_unverified
//! cargo run -p aureline-companion --example dump_m5_companion_certification -- markdown
//! ```

use aureline_companion::certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile::*;
use aureline_companion::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::M5CompanionMatrixLane;

const PACKET_ID: &str = "m5-companion-certification:stable:0001";
const PACKET_LABEL: &str = "M5 Companion Lane Certification On Every Marketed Profile";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> M5CompanionCertificationPacket {
    canonical_m5_companion_certification(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        M5CompanionCertificationProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
    .expect("canonical certification builds from the frozen matrix")
}

fn observation(lane: M5CompanionMatrixLane) -> CompanionCertificationObservation {
    CompanionCertificationObservation {
        lane,
        evidence_valid: true,
        proof_fresh: true,
        provider_or_admin_available: true,
        residency_and_encryption_verified: true,
        upstream_matrix_narrowed: false,
    }
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
        "proof_stale" => {
            // Every lane's proof goes stale: each headline claim and certified row
            // narrows one step and every row is forced to a labeled stale state.
            let observations = M5CompanionMatrixLane::ALL
                .into_iter()
                .map(|lane| CompanionCertificationObservation {
                    proof_fresh: false,
                    ..observation(lane)
                })
                .collect::<Vec<_>>();
            packet.apply_downgrade_automation(&observations);
        }
        "sync_evidence_invalid" => {
            // The managed-sync lane's evidence fails validation, so the lane is held
            // and every certified profile row withheld rather than shipped greener.
            packet.apply_downgrade_automation(&[CompanionCertificationObservation {
                evidence_valid: false,
                ..observation(M5CompanionMatrixLane::ManagedSync)
            }]);
        }
        "residency_unverified" => {
            // The residency/encryption lane's E2EE claim could not be verified, so the
            // lane narrows one step on the managed profiles where it is certified.
            packet.apply_downgrade_automation(&[CompanionCertificationObservation {
                residency_and_encryption_verified: false,
                ..observation(M5CompanionMatrixLane::ResidencyEncryption)
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
