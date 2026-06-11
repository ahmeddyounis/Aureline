//! Conformance dump for the managed sync maturity surface (snapshot classes,
//! conflict review, device registry, and end-to-end encrypted storage).
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- canonical
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- provider_down
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- sync_uninspectable
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- residency_unverified
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- device_trust_narrowed
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- admin_continuity_lost
//! cargo run -p aureline-companion --example dump_managed_sync_maturity_surface -- markdown
//! ```

use aureline_companion::ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage::*;

const PACKET_ID: &str = "managed-sync-maturity-surface:stable:0001";
const PACKET_LABEL: &str =
    "Managed Sync Maturity: Snapshot Classes, Conflict Review, Device Registry, and End-to-End Encrypted Storage";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> ManagedSyncMaturitySurfacePacket {
    canonical_managed_sync_maturity_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        ManagedSyncProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn healthy() -> ManagedSyncObservation {
    ManagedSyncObservation {
        sync_provider_available: true,
        proof_fresh: true,
        admin_continuity_available: true,
        residency_and_encryption_verified: true,
        sync_inspectable: true,
        device_trust_intact: true,
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
        "provider_down" => {
            // The managed sync provider is unavailable: every section narrows one step
            // and every live/cached item goes stale, labeled rather than hidden.
            packet.apply_managed_sync_degradation(&ManagedSyncObservation {
                sync_provider_available: false,
                ..healthy()
            });
        }
        "sync_uninspectable" => {
            // Sync can no longer be inspected: every snapshot, conflict, and device
            // record narrows to unreconcilable and the three managed-sync sections
            // narrow one step.
            packet.apply_managed_sync_degradation(&ManagedSyncObservation {
                sync_inspectable: false,
                ..healthy()
            });
        }
        "residency_unverified" => {
            // The residency/encryption claim is unverified: every verified encryption
            // claim downgrades to claimed-unverified, is labeled, and the
            // encrypted-storage section narrows one step.
            packet.apply_managed_sync_degradation(&ManagedSyncObservation {
                residency_and_encryption_verified: false,
                ..healthy()
            });
        }
        "device_trust_narrowed" => {
            // Device trust narrowed: trusted devices narrow to pending-approval and the
            // conflict-review and device-registry sections narrow one step.
            packet.apply_managed_sync_degradation(&ManagedSyncObservation {
                device_trust_intact: false,
                ..healthy()
            });
        }
        "admin_continuity_lost" => {
            // Managed-tenant admin continuity is unavailable: the device-registry and
            // encrypted-storage sections narrow one step.
            packet.apply_managed_sync_degradation(&ManagedSyncObservation {
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
