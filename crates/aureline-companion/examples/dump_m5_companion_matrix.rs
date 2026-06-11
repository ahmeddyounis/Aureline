//! Conformance dump for the frozen M5 companion, incident, sync, residency, and
//! offboarding matrix.
//!
//! Prints either the canonical matrix export, a narrowed/withheld fixture, or the
//! Markdown summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer matrix builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_m5_companion_matrix -- canonical
//! cargo run -p aureline-companion --example dump_m5_companion_matrix -- sync_withheld
//! cargo run -p aureline-companion --example dump_m5_companion_matrix -- residency_narrowed
//! cargo run -p aureline-companion --example dump_m5_companion_matrix -- markdown
//! ```

use aureline_companion::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::*;

const PACKET_ID: &str = "m5-companion-matrix:stable:0001";
const PACKET_LABEL: &str = "M5 Companion, Incident, Sync, Residency, and Offboarding Matrix";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> M5CompanionMatrixPacket {
    canonical_m5_companion_matrix(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        M5CompanionMatrixProofFreshness {
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
        "sync_withheld" => {
            // The managed-sync lane's evidence packet fails validation, so the lane is
            // held and its rollout withheld rather than shipped greener than the proof.
            packet.apply_downgrade_automation(&[M5CompanionMatrixLaneObservation {
                lane: M5CompanionMatrixLane::ManagedSync,
                evidence_valid: false,
                proof_fresh: true,
                provider_or_admin_available: true,
                residency_and_encryption_verified: true,
                upstream_narrowed: false,
            }]);
        }
        "residency_narrowed" => {
            // The residency lane's E2EE claim could not be verified, so a Beta lane
            // narrows to Preview and its staged rollout narrows to early access.
            packet.apply_downgrade_automation(&[M5CompanionMatrixLaneObservation {
                lane: M5CompanionMatrixLane::ResidencyEncryption,
                evidence_valid: true,
                proof_fresh: true,
                provider_or_admin_available: true,
                residency_and_encryption_verified: false,
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
