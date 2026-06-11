//! Conformance dump for the customer-managed-key and storage selection flows,
//! region/residency cues, and degraded managed-service continuity surface.
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- canonical
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- managed_service_degraded
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- key_management_down
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- storage_provider_down
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- residency_unverified
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- admin_continuity_lost
//! cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- markdown
//! ```

use aureline_companion::add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont::*;

const PACKET_ID: &str = "key-storage-residency-continuity-surface:stable:0001";
const PACKET_LABEL: &str =
    "Customer-Managed-Key and Storage Selection Flows, Region/Residency Cues, and Degraded Managed-Service Continuity";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> KeyStorageResidencyContinuitySurfacePacket {
    canonical_key_storage_residency_continuity_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        ResidencyProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn healthy() -> ResidencyContinuityObservation {
    ResidencyContinuityObservation {
        key_management_available: true,
        storage_provider_available: true,
        proof_fresh: true,
        residency_verified: true,
        encryption_verified: true,
        admin_continuity_available: true,
        managed_service_available: true,
        host_session_active: true,
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
        "managed_service_degraded" => {
            // The managed service is degraded: every section narrows one step, every
            // live/cached item goes stale, and every non-local continuity capability is
            // marked degraded while local work stays preserved.
            packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
                managed_service_available: false,
                ..healthy()
            });
        }
        "key_management_down" => {
            // The key-management service is unavailable: every non-local key-custody
            // option narrows to require admin approval and the local-only key fallback
            // remains available.
            packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
                key_management_available: false,
                ..healthy()
            });
        }
        "storage_provider_down" => {
            // The managed storage provider is unavailable: every non-local-fallback
            // storage option narrows to require admin approval and the local-first
            // storage fallback remains available.
            packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
                storage_provider_available: false,
                ..healthy()
            });
        }
        "residency_unverified" => {
            // The residency claim is unverified: every verified residency pin downgrades
            // to claimed-unverified, is labeled, and the residency-cue and storage
            // sections narrow one step.
            packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
                residency_verified: false,
                ..healthy()
            });
        }
        "admin_continuity_lost" => {
            // Managed-tenant admin continuity is unavailable: the key-custody,
            // residency-cue, and continuity sections narrow one step.
            packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
                admin_continuity_available: false,
                ..healthy()
            });
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
