//! Conformance dump for the runbook execution surface (execution rows, deviation
//! notes, export bundles, and browser or vendor-console handoff).
//!
//! Prints either the canonical surface export, a degraded fixture, or the Markdown
//! summary, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the first-consumer surface builder.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- canonical
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- relay_down
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- host_inactive
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- attribution_lost
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- export_incomplete
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- external_unreachable
//! cargo run -p aureline-companion --example dump_runbook_execution_surface -- markdown
//! ```

use aureline_companion::implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth::*;

const PACKET_ID: &str = "runbook-execution-surface:stable:0001";
const PACKET_LABEL: &str =
    "Runbook Execution Rows, Deviation Notes, Export Bundles, and Browser or Vendor-Console Handoff";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn canonical() -> RunbookExecutionSurfacePacket {
    canonical_runbook_execution_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        RunbookExecutionProofFreshness {
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
            packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
                relay_available: false,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: true,
                export_complete: true,
                external_reachable: true,
                upstream_matrix_narrowed: false,
            });
        }
        "host_inactive" => {
            // No active desktop host session: every desktop handoff requiring a live
            // host can no longer resolve exactly, and the execution-row section narrows
            // because an approved action can no longer be relayed.
            packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: false,
                trust_intact: true,
                incident_attribution_intact: true,
                export_complete: true,
                external_reachable: true,
                upstream_matrix_narrowed: false,
            });
        }
        "attribution_lost" => {
            // Incident attribution was lost: execution rows and deviation notes narrow
            // to unattributed and their sections narrow one step.
            packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: false,
                export_complete: true,
                external_reachable: true,
                upstream_matrix_narrowed: false,
            });
        }
        "export_incomplete" => {
            // The export is incomplete: every ready bundle narrows to partial, gaps are
            // labeled, and the export-bundle section narrows one step.
            packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: true,
                export_complete: false,
                external_reachable: true,
                upstream_matrix_narrowed: false,
            });
        }
        "external_unreachable" => {
            // The browser/vendor console is unreachable: every external handoff narrows
            // to unresolved while the local desktop fallback stays exact, and the
            // external-handoff section narrows one step.
            packet.apply_runbook_execution_degradation(&RunbookExecutionObservation {
                relay_available: true,
                proof_fresh: true,
                host_session_active: true,
                trust_intact: true,
                incident_attribution_intact: true,
                export_complete: true,
                external_reachable: false,
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
