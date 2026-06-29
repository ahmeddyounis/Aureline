//! Headless emitter for the service-health communication packet.
//!
//! The bin is the only mint-from-truth path for the published packet inventory checked in at
//! `artifacts/release/m5-service-health-communication.json`, the release-grade stale-release-data
//! parity proof under `artifacts/release/m5-stale-release-data-proof/` (and its Markdown report), the
//! machine-readable per-card CSV export at `artifacts/release/m5-service-health-communication.csv`, and
//! the per-state packet fixtures under `fixtures/release/service-health-and-admin-notes/`. It surfaces,
//! per service tier — local machine, remote target, enterprise control plane, and optional
//! vendor-hosted service — the health state, the release-data state of the data shown for it, the
//! source-age truth, and the local-safe continuation statement, and, per propagated admin note —
//! channel, mirror, and deployment change — the same release-data vocabulary, without making
//! downgraded data look live or implying a remote / vendor outage makes local editing unsafe.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- variant <canonical|vendor-outage|mirror-note|local-only>
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- validate
//! ```

use aureline_release::m5_service_health_communication::{
    seeded_m5_service_health_communication, seeded_m5_service_health_communication_local_only,
    seeded_m5_service_health_communication_mirror_note,
    seeded_m5_service_health_communication_vendor_outage, ServiceHealthCommunication,
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
        Some("registry") | Some("proof") | None => {
            let packet = seeded_m5_service_health_communication();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_service_health_communication();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_service_health_communication();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_card_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_service_health_communication();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let consumer = packet
                .consumers
                .iter()
                .find(|c| c.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(consumer)?);
        }
        Some("validate") => {
            let packet = seeded_m5_service_health_communication();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_service_health_communication_vendor_outage())?;
            assert_packet_valid(&seeded_m5_service_health_communication_mirror_note())?;
            assert_packet_valid(&seeded_m5_service_health_communication_local_only())?;
            println!(
                "ok: service-health communication valid ({} tiers, {} notes, {} consumers)",
                packet.tiers.len(),
                packet.notes.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<ServiceHealthCommunication, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_service_health_communication()),
        "vendor-outage" => Ok(seeded_m5_service_health_communication_vendor_outage()),
        "mirror-note" => Ok(seeded_m5_service_health_communication_mirror_note()),
        "local-only" => Ok(seeded_m5_service_health_communication_local_only()),
        other => Err(format!(
            "unknown variant: {other} (canonical|vendor-outage|mirror-note|local-only)"
        )
        .into()),
    }
}

fn assert_packet_valid(
    packet: &ServiceHealthCommunication,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
