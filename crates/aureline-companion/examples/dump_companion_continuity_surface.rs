//! Conformance dump for the companion remote-preview, session-handoff,
//! light-remote-edit, and scoped collaboration-follow continuity surface.
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_companion_continuity_surface -- canonical
//! cargo run -p aureline-companion --example dump_companion_continuity_surface -- relay_down
//! cargo run -p aureline-companion --example dump_companion_continuity_surface -- host_inactive
//! cargo run -p aureline-companion --example dump_companion_continuity_surface -- scope_revoked
//! cargo run -p aureline-companion --example dump_companion_continuity_surface -- markdown
//! ```

use aureline_companion::add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio::*;

const PACKET_ID: &str = "companion-continuity-surface:stable:0001";
const PACKET_LABEL: &str =
    "Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> CompanionContinuitySurfacePacket {
    canonical_companion_continuity_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        CompanionContinuityProofFreshness {
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
            packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
                relay_available: false,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                collaboration_scope_intact: true,
                upstream_matrix_narrowed: false,
            });
        }
        "host_inactive" => {
            // No active desktop host session: every handoff requiring a live host can
            // no longer resolve exactly, remote-preview items report handoff continuity
            // unavailable, and the bounded light-remote-edit surface narrows because a
            // write can no longer be relayed.
            packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: false,
                trust_intact: true,
                collaboration_scope_intact: true,
                upstream_matrix_narrowed: false,
            });
        }
        "scope_revoked" => {
            // The shared collaboration scope was revoked: collaboration items narrow to
            // scope-revoked and the collaboration-follow surface narrows one step.
            packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                collaboration_scope_intact: false,
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
