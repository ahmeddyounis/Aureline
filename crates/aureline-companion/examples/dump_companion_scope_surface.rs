//! Conformance dump for the companion session-follow and incident-awareness
//! surface with bounded read/write scope and stale-state honesty.
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_companion_scope_surface -- canonical
//! cargo run -p aureline-companion --example dump_companion_scope_surface -- relay_down
//! cargo run -p aureline-companion --example dump_companion_scope_surface -- host_inactive
//! cargo run -p aureline-companion --example dump_companion_scope_surface -- attribution_lost
//! cargo run -p aureline-companion --example dump_companion_scope_surface -- markdown
//! ```

use aureline_companion::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty::*;

const PACKET_ID: &str = "companion-scope-surface:stable:0001";
const PACKET_LABEL: &str = "Companion Session-Follow and Incident-Awareness Surfaces";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> CompanionScopeSurfacePacket {
    canonical_companion_scope_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        CompanionScopeProofFreshness {
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
        "relay_down" => {
            // The companion relay is unavailable: every surface narrows one step and
            // every live/cached item goes stale, labeled rather than hidden.
            packet.apply_companion_scope_degradation(&CompanionScopeObservation {
                relay_available: false,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: true,
                upstream_matrix_narrowed: false,
            });
        }
        "host_inactive" => {
            // No active desktop host session: every handoff requiring a live host
            // can no longer resolve exactly, and the bounded light-edit surface
            // narrows because a write can no longer be relayed.
            packet.apply_companion_scope_degradation(&CompanionScopeObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: false,
                trust_intact: true,
                incident_attribution_intact: true,
                upstream_matrix_narrowed: false,
            });
        }
        "attribution_lost" => {
            // Incident attribution was lost: incident items narrow to unattributed
            // and the incident-awareness surface narrows one step.
            packet.apply_companion_scope_degradation(&CompanionScopeObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: false,
                upstream_matrix_narrowed: false,
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
