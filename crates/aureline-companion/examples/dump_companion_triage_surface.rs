//! Conformance dump for the companion notification triage, review queue, and
//! CI-status surface with exact desktop handoff.
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_companion_triage_surface -- canonical
//! cargo run -p aureline-companion --example dump_companion_triage_surface -- relay_down
//! cargo run -p aureline-companion --example dump_companion_triage_surface -- host_inactive
//! cargo run -p aureline-companion --example dump_companion_triage_surface -- markdown
//! ```

use aureline_companion::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::*;

const PACKET_ID: &str = "companion-triage-surface:stable:0001";
const PACKET_LABEL: &str = "Companion Notification Triage, Review Queues, and CI-Status Cards";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> CompanionTriageSurfacePacket {
    canonical_companion_triage_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        CompanionTriageProofFreshness {
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
            // The companion relay is unavailable: every section narrows one step and
            // every CI card goes stale, but the surface stays labeled, not hidden.
            packet.apply_companion_degradation(&CompanionSurfaceObservation {
                relay_available: false,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                upstream_matrix_narrowed: false,
            });
        }
        "host_inactive" => {
            // No active desktop host session: every handoff requiring a live host
            // can no longer resolve exactly and is marked unresolved instead.
            packet.apply_companion_degradation(&CompanionSurfaceObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: false,
                trust_intact: true,
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
