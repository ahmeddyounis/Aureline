//! Conformance dump for the usage-export and offboarding packages, grace-window state,
//! org-switch semantics, and deletion/export honesty surface.
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- canonical
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- managed_service_degraded
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- export_assembler_down
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- completeness_unverified
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- deletion_service_down
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- admin_continuity_lost
//! cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- markdown
//! ```

use aureline_companion::implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho::*;

const PACKET_ID: &str = "usage-export-offboarding-surface:stable:0001";
const PACKET_LABEL: &str =
    "Usage-Export and Offboarding Packages, Grace-Window State, Org-Switch Semantics, and Deletion/Export Honesty";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> UsageExportOffboardingSurfacePacket {
    canonical_usage_export_offboarding_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        OffboardingProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn healthy() -> OffboardingDegradationObservation {
    OffboardingDegradationObservation {
        export_assembler_available: true,
        deletion_service_available: true,
        proof_fresh: true,
        completeness_verified: true,
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
            // The managed service is degraded: every section narrows one step and every
            // live/cached item goes stale while local paths remain and local work stays
            // preserved.
            packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
                managed_service_available: false,
                ..healthy()
            });
        }
        "export_assembler_down" => {
            // The managed export assembler is unavailable: every provider-assembled
            // package narrows to unavailable while the local-first path remains.
            packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
                export_assembler_available: false,
                ..healthy()
            });
        }
        "completeness_unverified" => {
            // Completeness is unverified: every verified completeness claim downgrades to
            // claimed-unverified, is labeled, and the package sections narrow one step.
            packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
                completeness_verified: false,
                ..healthy()
            });
        }
        "deletion_service_down" => {
            // The deletion service is unavailable: every closing grace window is held
            // open again, widening the reversible window, and the grace-window section
            // narrows one step.
            packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
                deletion_service_available: false,
                ..healthy()
            });
        }
        "admin_continuity_lost" => {
            // Managed-tenant admin continuity is unavailable: the offboarding-package,
            // grace-window, and org-switch sections narrow one step.
            packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
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
