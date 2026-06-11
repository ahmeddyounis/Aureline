//! Conformance dump for the companion-safe redaction, local-core continuity, and offline
//! packet-flow surface across the support and incident lanes.
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown summary,
//! so the checked-in artifact and fixtures can be regenerated deterministically from the
//! first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- canonical
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- managed_service_degraded
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- redaction_proof_lost
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- packet_assembler_down
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- completeness_unverified
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- incident_attribution_lost
//! cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- markdown
//! ```

use aureline_companion::ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes::*;

const PACKET_ID: &str = "redaction-continuity-offline-packet-surface:stable:0001";
const PACKET_LABEL: &str =
    "Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flows Across Support and Incident Lanes";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> RedactionContinuitySurfacePacket {
    canonical_redaction_continuity_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        RedactionProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
        },
    )
}

fn healthy() -> RedactionDegradationObservation {
    RedactionDegradationObservation {
        redaction_proof_available: true,
        packet_assembler_available: true,
        proof_fresh: true,
        completeness_verified: true,
        incident_attribution_available: true,
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
            packet.apply_redaction_degradation(&RedactionDegradationObservation {
                managed_service_available: false,
                ..healthy()
            });
        }
        "redaction_proof_lost" => {
            // Redaction proof is unavailable: every verified redaction downgrades to
            // claimed-unverified, is labeled, a redacted-summary class narrows to
            // reference-only, and the redaction-policy section narrows one step.
            packet.apply_redaction_degradation(&RedactionDegradationObservation {
                redaction_proof_available: false,
                ..healthy()
            });
        }
        "packet_assembler_down" => {
            // The offline packet assembler is unavailable: every provider-assembled packet
            // narrows to unavailable while the local-first path remains.
            packet.apply_redaction_degradation(&RedactionDegradationObservation {
                packet_assembler_available: false,
                ..healthy()
            });
        }
        "completeness_unverified" => {
            // Completeness is unverified: every verified completeness claim downgrades to
            // claimed-unverified, is labeled, and the packet sections narrow one step.
            packet.apply_redaction_degradation(&RedactionDegradationObservation {
                completeness_verified: false,
                ..healthy()
            });
        }
        "incident_attribution_lost" => {
            // Incident attribution is unavailable: every attributed incident packet narrows
            // its attribution, is labeled, and the incident-packet section narrows one step.
            packet.apply_redaction_degradation(&RedactionDegradationObservation {
                incident_attribution_available: false,
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
