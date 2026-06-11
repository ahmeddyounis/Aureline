//! Conformance dump for the incident workspace surface (headers, evidence
//! timelines, resource slices, and runbook packets).
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_incident_workspace_surface -- canonical
//! cargo run -p aureline-companion --example dump_incident_workspace_surface -- relay_down
//! cargo run -p aureline-companion --example dump_incident_workspace_surface -- host_inactive
//! cargo run -p aureline-companion --example dump_incident_workspace_surface -- attribution_lost
//! cargo run -p aureline-companion --example dump_incident_workspace_surface -- evidence_incomplete
//! cargo run -p aureline-companion --example dump_incident_workspace_surface -- markdown
//! ```

use aureline_companion::add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets::*;

const PACKET_ID: &str = "incident-workspace-surface:stable:0001";
const PACKET_LABEL: &str =
    "Incident Workspace Headers, Evidence Timelines, Resource Slices, and Runbook Packets";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> IncidentWorkspaceSurfacePacket {
    canonical_incident_workspace_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        IncidentWorkspaceProofFreshness {
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
            // every live/cached item goes stale, labeled rather than hidden.
            packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
                relay_available: false,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: true,
                evidence_complete: true,
                upstream_matrix_narrowed: false,
            });
        }
        "host_inactive" => {
            // No active desktop host session: every handoff requiring a live host can
            // no longer resolve exactly, and the runbook section narrows because an
            // approved action can no longer be relayed.
            packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: false,
                trust_intact: true,
                incident_attribution_intact: true,
                evidence_complete: true,
                upstream_matrix_narrowed: false,
            });
        }
        "attribution_lost" => {
            // Incident attribution was lost: headers and evidence spans narrow to
            // unattributed and their sections narrow one step.
            packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: false,
                evidence_complete: true,
                upstream_matrix_narrowed: false,
            });
        }
        "evidence_incomplete" => {
            // Evidence is incomplete: every present span narrows to partial, gaps are
            // labeled, and the evidence-timeline section narrows one step.
            packet.apply_incident_workspace_degradation(&IncidentWorkspaceObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: true,
                evidence_complete: false,
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
