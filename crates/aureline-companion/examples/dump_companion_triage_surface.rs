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
//! cargo run -p aureline-companion --example dump_companion_triage_surface -- emit-fixtures .
//! ```

use aureline_companion::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::*;
use std::error::Error;
use std::path::Path;

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

fn write_packet(path: &Path, packet: &CompanionTriageSurfacePacket) -> Result<(), Box<dyn Error>> {
    assert!(
        packet.validate().is_empty(),
        "dump packet failed validation: {:?}",
        packet.validate()
    );
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, format!("{}\n", packet.export_safe_json()))?;
    println!("wrote {}", path.display());
    Ok(())
}

fn emit_fixtures(root: &Path) -> Result<(), Box<dyn Error>> {
    let canonical_packet = canonical();
    write_packet(
        &root.join(
            "artifacts/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/support_export.json",
        ),
        &canonical_packet,
    )?;

    let fixture_root = root.join(
        "fixtures/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff",
    );
    let mut relay_down = canonical();
    relay_down.apply_companion_degradation(&CompanionSurfaceObservation {
        relay_available: false,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        upstream_matrix_narrowed: false,
    });
    write_packet(
        &fixture_root.join("relay_unavailable_surface.json"),
        &relay_down,
    )?;

    let mut host_inactive = canonical();
    host_inactive.apply_companion_degradation(&CompanionSurfaceObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: false,
        trust_intact: true,
        upstream_matrix_narrowed: false,
    });
    write_packet(
        &fixture_root.join("host_inactive_surface.json"),
        &host_inactive,
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    if which == "emit-fixtures" {
        let root = std::env::args().nth(2).unwrap_or_else(|| ".".to_owned());
        return emit_fixtures(Path::new(&root));
    }
    let mut packet = canonical();
    match which.as_str() {
        "canonical" => {}
        "markdown" => {
            print!("{}", packet.render_markdown_summary());
            return Ok(());
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
    Ok(())
}
