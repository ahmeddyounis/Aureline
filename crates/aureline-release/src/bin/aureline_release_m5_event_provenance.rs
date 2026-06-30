//! Headless emitter for the M5 event-provenance inspector.
//!
//! The bin is the only mint-from-truth path for the published event-provenance inventory checked in
//! at `artifacts/public-truth/m5-event-provenance.json`, the rendered overview document at
//! `artifacts/public-truth/m5-event-provenance.md`, the machine-readable event / facet matrix at
//! `artifacts/public-truth/m5-event-provenance-events.csv`, the release-grade parity proof under
//! `artifacts/public-truth/m5-event-provenance-proof/` (and its Markdown report), the exported
//! redaction-safe preview, and the per-state drill fixtures under
//! `fixtures/public-truth/m5-event-provenance/`. The inspector explains, for each queued or
//! replayable M5 action, where the event came from, what drifted since it was planned, and whether
//! replaying it is still safe, so a facet can never read safer than its proof.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- overview
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- export
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- variant <canonical|provenance-stale|drift-region|drift-tenant|reapproval-required|reapproval-blocked>
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- event <action-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_event_provenance -- validate
//! ```

use aureline_release::m5_event_provenance::{
    seeded_m5_event_provenance, seeded_m5_event_provenance_drift_region_narrowed,
    seeded_m5_event_provenance_drift_tenant_blocked,
    seeded_m5_event_provenance_provenance_stale_narrowed,
    seeded_m5_event_provenance_reapproval_blocked,
    seeded_m5_event_provenance_reapproval_required_narrowed, M5EventProvenance,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("registry") | None => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("overview") => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_overview_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_events_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("export") => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            println!("{}", packet.render_export_preview());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("event") => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let event = packet
                .deferred_events
                .iter()
                .find(|e| e.action.as_str() == token)
                .ok_or_else(|| format!("unknown action token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(event)?);
        }
        Some("validate") => {
            let packet = seeded_m5_event_provenance();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_event_provenance_provenance_stale_narrowed())?;
            assert_packet_valid(&seeded_m5_event_provenance_drift_region_narrowed())?;
            assert_packet_valid(&seeded_m5_event_provenance_drift_tenant_blocked())?;
            assert_packet_valid(&seeded_m5_event_provenance_reapproval_required_narrowed())?;
            assert_packet_valid(&seeded_m5_event_provenance_reapproval_blocked())?;
            println!(
                "ok: event provenance valid ({} events)",
                packet.deferred_events.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5EventProvenance, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_event_provenance()),
        "provenance-stale" => Ok(seeded_m5_event_provenance_provenance_stale_narrowed()),
        "drift-region" => Ok(seeded_m5_event_provenance_drift_region_narrowed()),
        "drift-tenant" => Ok(seeded_m5_event_provenance_drift_tenant_blocked()),
        "reapproval-required" => Ok(seeded_m5_event_provenance_reapproval_required_narrowed()),
        "reapproval-blocked" => Ok(seeded_m5_event_provenance_reapproval_blocked()),
        other => Err(format!(
            "unknown variant: {other} (canonical|provenance-stale|drift-region|drift-tenant|reapproval-required|reapproval-blocked)"
        )
        .into()),
    }
}

fn assert_packet_valid(packet: &M5EventProvenance) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
